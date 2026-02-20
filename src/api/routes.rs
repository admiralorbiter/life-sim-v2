use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use crate::data_loader::GameData;
use crate::engine::game_state::GameState;
use crate::engine::rng;
use crate::engine::turn_runner::{self, PlayerChoices};
use crate::engine::event_deck;
use crate::models::EventCard;
use rand_chacha::ChaCha8Rng;

/// Shared server state: one active game per process (MVP).
pub struct AppState {
    pub game: Mutex<Option<GameState>>,
    pub rng: Mutex<Option<ChaCha8Rng>>,
    /// The event card drawn for the current turn (preview before player picks an option).
    pub pending_event: Mutex<Option<EventCard>>,
}

/// Health check endpoint.
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "game": "Life Roguelite",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// POST /api/new_game — Start a new game (optional seed param).
pub async fn new_game(
    app_state: web::Data<AppState>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let seed = body.get("seed")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| rng::generate_seed());

    let game = GameState::new(seed.clone());
    let game_rng = rng::create_rng(&seed);

    *app_state.game.lock().unwrap() = Some(game.clone());
    *app_state.rng.lock().unwrap() = Some(game_rng);
    *app_state.pending_event.lock().unwrap() = None;

    HttpResponse::Ok().json(serde_json::json!({
        "state": game,
        "message": format!("New game started with seed: {}", seed)
    }))
}

/// GET /api/state — Get current game state.
pub async fn get_state(app_state: web::Data<AppState>) -> impl Responder {
    let game = app_state.game.lock().unwrap();
    match &*game {
        Some(state) => HttpResponse::Ok().json(state),
        None => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No game in progress. Start a new game first."
        })),
    }
}

/// GET /api/phase_data — Get available actions, decisions, and events for the current turn.
pub async fn phase_data(
    app_state: web::Data<AppState>,
    game_data: web::Data<GameData>,
) -> impl Responder {
    let game = app_state.game.lock().unwrap();
    match &*game {
        Some(state) => {
            let stage = &state.current_stage;

            // Available actions for this stage
            let actions: Vec<_> = game_data.actions.iter()
                .filter(|a| a.stages.contains(stage))
                .collect();

            // Decision for this stage (pick one that matches current turn, or first for stage)
            let decision = game_data.decisions.iter()
                .find(|d| d.stage == *stage && d.turn == state.current_turn)
                .or_else(|| game_data.decisions.iter().find(|d| d.stage == *stage));

            // Available event count
            let available_events = event_deck::available_events(
                &game_data.events, stage, &state.used_event_ids
            );

            let is_game_over = turn_runner::is_game_over(state);

            HttpResponse::Ok().json(serde_json::json!({
                "actions": actions,
                "decision": decision,
                "availableEventCount": available_events.len(),
                "isGameOver": is_game_over,
                "currentStage": state.current_stage,
                "currentTurn": state.current_turn,
            }))
        }
        None => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No game in progress."
        })),
    }
}

/// GET /api/draw_event — Draw a random event card for preview (before player picks an option).
/// The drawn card is cached so submit_turn uses the same one.
pub async fn draw_event(
    app_state: web::Data<AppState>,
    game_data: web::Data<GameData>,
) -> impl Responder {
    let game = app_state.game.lock().unwrap();
    let mut game_rng = app_state.rng.lock().unwrap();
    let mut pending = app_state.pending_event.lock().unwrap();

    match (&*game, &mut *game_rng) {
        (Some(state), Some(rng_ref)) => {
            // Draw an event if we haven't already for this turn
            if pending.is_none() {
                let drawn = event_deck::draw_event(
                    &game_data.events, &state.current_stage,
                    &state.used_event_ids, rng_ref,
                );
                *pending = drawn.cloned();
            }

            HttpResponse::Ok().json(serde_json::json!({
                "event": &*pending,
                "playerSupport": state.support,
            }))
        }
        _ => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No game in progress."
        })),
    }
}

/// POST /api/submit_turn — Submit choices and run one turn.
/// If a pending event was drawn via /api/draw_event, that event is used.
pub async fn submit_turn(
    app_state: web::Data<AppState>,
    game_data: web::Data<GameData>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let mut game = app_state.game.lock().unwrap();
    let mut game_rng = app_state.rng.lock().unwrap();
    let mut pending = app_state.pending_event.lock().unwrap();

    let (state, rng_ref) = match (&mut *game, &mut *game_rng) {
        (Some(s), Some(r)) => (s, r),
        _ => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No game in progress."
        })),
    };

    if turn_runner::is_game_over(state) {
        return HttpResponse::Ok().json(serde_json::json!({
            "error": "Game is over!",
            "state": &*state,
            "isGameOver": true,
        }));
    }

    // Parse choices from request body
    let action_ids: Vec<String> = body.get("actionIds")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    let decision_id = body.get("decisionId")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let decision_option_index = body.get("decisionOptionIndex")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

    let event_option_index = body.get("eventOptionIndex")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize);

    let choices = PlayerChoices {
        action_ids,
        decision_id,
        decision_option_index,
        event_option_index,
    };

    // If we have a pending pre-drawn event, pass it to the turn runner
    let result = turn_runner::run_turn_with_event(
        state, &choices, &game_data, rng_ref, pending.take(),
    );

    HttpResponse::Ok().json(serde_json::json!({
        "state": &*state,
        "turnResult": {
            "feedback": result.feedback,
            "eventDrawn": result.event_drawn,
            "stageTransitioned": result.stage_transitioned,
            "newStage": result.new_stage,
            "oldStage": result.old_stage,
            "stressWarning": result.stress_warning,
        },
        "isGameOver": turn_runner::is_game_over(state),
    }))
}

/// GET /api/endings — Get the resolved ending.
pub async fn get_ending(
    app_state: web::Data<AppState>,
    game_data: web::Data<GameData>,
) -> impl Responder {
    let game = app_state.game.lock().unwrap();
    match &*game {
        Some(state) => {
            // Find the best matching ending
            let ending = game_data.endings.iter().find(|e| {
                let money_ok = e.conditions.money.as_ref()
                    .map(|c| {
                        c.min.map_or(true, |min| state.money >= min) &&
                        c.max.map_or(true, |max| state.money <= max)
                    }).unwrap_or(true);

                let stress_ok = e.conditions.stress.as_ref()
                    .map(|c| {
                        c.min.map_or(true, |min| state.stress >= min) &&
                        c.max.map_or(true, |max| state.stress <= max)
                    }).unwrap_or(true);

                let support_ok = e.conditions.support.as_ref()
                    .map(|c| {
                        c.min.map_or(true, |min| state.support >= min) &&
                        c.max.map_or(true, |max| state.support <= max)
                    }).unwrap_or(true);

                let cred_ok = e.conditions.credentials.as_ref()
                    .map(|c| {
                        c.min_count.map_or(true, |min| state.credentials.len() as u32 >= min)
                    }).unwrap_or(true);

                money_ok && stress_ok && support_ok && cred_ok
            });

            HttpResponse::Ok().json(serde_json::json!({
                "ending": ending,
                "state": &*state,
            }))
        }
        None => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No game in progress."
        })),
    }
}

/// GET /api/jobs — List available jobs for the current stage with eligibility.
pub async fn get_jobs(
    app_state: web::Data<AppState>,
    game_data: web::Data<GameData>,
) -> impl Responder {
    let game = app_state.game.lock().unwrap();
    match &*game {
        Some(state) => {
            let current_job_id = state.current_job.as_ref().map(|j| j.id.clone());
            let jobs: Vec<serde_json::Value> = game_data.jobs.iter()
                .filter(|j| j.stages.contains(&state.current_stage))
                .map(|j| {
                    let missing_required: Vec<&String> = j.required_tags.iter()
                        .filter(|t| !state.credentials.contains(t))
                        .collect();
                    let missing_recommended: Vec<&String> = j.recommended_tags.iter()
                        .filter(|t| !state.credentials.contains(t))
                        .collect();
                    let is_current = current_job_id.as_ref() == Some(&j.id);

                    serde_json::json!({
                        "id": j.id,
                        "title": j.title,
                        "description": j.description,
                        "payPerTurn": j.pay_per_turn,
                        "stressPerTurn": j.stress_per_turn,
                        "requiredTags": j.required_tags,
                        "recommendedTags": j.recommended_tags,
                        "growthRate": j.growth_rate,
                        "growthTag": j.growth_tag,
                        "eligible": missing_required.is_empty(),
                        "isCurrent": is_current,
                        "missingRequired": missing_required,
                        "missingRecommended": missing_recommended,
                    })
                })
                .collect();

            let growth_info = state.current_job.as_ref().map(|j| {
                serde_json::json!({
                    "jobTitle": j.title,
                    "jobTurns": state.job_turns,
                    "growthRate": j.growth_rate,
                    "growthTag": j.growth_tag,
                })
            });

            HttpResponse::Ok().json(serde_json::json!({
                "jobs": jobs,
                "currentJob": growth_info,
            }))
        }
        None => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No game in progress."
        })),
    }
}

// ═══════════════════════════════════════════════════════════════
// Debug / Dev Endpoints
// ═══════════════════════════════════════════════════════════════

/// POST /api/debug/skip_stage — Jump to the start of the next stage.
pub async fn debug_skip_stage(app_state: web::Data<AppState>) -> impl Responder {
    let mut game = app_state.game.lock().unwrap();
    match &mut *game {
        Some(state) => {
            let old_stage = state.current_stage.clone();
            let end = turn_runner::stage_end_turn(&state.current_stage);
            state.current_turn = end + 1; // Move past the boundary

            // Trigger the transition
            if let Some(ns) = turn_runner::next_stage(&state.current_stage) {
                state.current_stage = ns;
                state.time_slots = 3;
            }

            HttpResponse::Ok().json(serde_json::json!({
                "state": &*state,
                "message": format!("Skipped from {:?} to {:?}", old_stage, state.current_stage),
            }))
        }
        None => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No game in progress."
        })),
    }
}

/// POST /api/debug/set_stats — Freely set any stat values.
pub async fn debug_set_stats(
    app_state: web::Data<AppState>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let mut game = app_state.game.lock().unwrap();
    match &mut *game {
        Some(state) => {
            if let Some(v) = body.get("money").and_then(|v| v.as_i64()) {
                state.money = v as i32;
            }
            if let Some(v) = body.get("stress").and_then(|v| v.as_i64()) {
                state.stress = v as i32;
            }
            if let Some(v) = body.get("support").and_then(|v| v.as_i64()) {
                state.support = v as i32;
            }
            if let Some(v) = body.get("monthlyBills").and_then(|v| v.as_i64()) {
                state.monthly_bills = v as i32;
            }
            if let Some(v) = body.get("emergencyFund").and_then(|v| v.as_i64()) {
                state.emergency_fund = v as i32;
            }
            if let Some(v) = body.get("turn").and_then(|v| v.as_u64()) {
                state.current_turn = v as u32;
            }

            HttpResponse::Ok().json(serde_json::json!({
                "state": &*state,
                "message": "Stats updated",
            }))
        }
        None => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No game in progress."
        })),
    }
}

/// POST /api/debug/grant_tag — Grant a credential tag to the player.
pub async fn debug_grant_tag(
    app_state: web::Data<AppState>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let mut game = app_state.game.lock().unwrap();
    match &mut *game {
        Some(state) => {
            if let Some(tag) = body.get("tag").and_then(|v| v.as_str()) {
                if !state.credentials.contains(&tag.to_string()) {
                    state.credentials.push(tag.to_string());
                }
                HttpResponse::Ok().json(serde_json::json!({
                    "state": &*state,
                    "message": format!("Granted credential: {}", tag),
                }))
            } else {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Missing 'tag' field."
                }))
            }
        }
        None => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No game in progress."
        })),
    }
}

/// GET /api/timeline — Get the top 8 most impactful decisions.
pub async fn get_timeline(
    app_state: web::Data<AppState>,
) -> impl Responder {
    let game = app_state.game.lock().unwrap();
    match &*game {
        Some(state) => {
            let mut entries = state.decision_log.clone();
            // Sort by total absolute impact magnitude (descending)
            entries.sort_by(|a, b| {
                let mag_a: i32 = a.impact.split(", ")
                    .filter_map(|s| s.split_whitespace().last())
                    .filter_map(|v| v.parse::<i32>().ok())
                    .map(|v| v.abs())
                    .sum();
                let mag_b: i32 = b.impact.split(", ")
                    .filter_map(|s| s.split_whitespace().last())
                    .filter_map(|v| v.parse::<i32>().ok())
                    .map(|v| v.abs())
                    .sum();
                mag_b.cmp(&mag_a)
            });
            entries.truncate(8);
            // Re-sort by turn order for display
            entries.sort_by_key(|e| e.turn);

            HttpResponse::Ok().json(serde_json::json!({
                "timeline": entries,
                "seed": &state.seed,
            }))
        }
        None => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No game in progress."
        })),
    }
}

/// Configure all API routes.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health))
            .route("/new_game", web::post().to(new_game))
            .route("/state", web::get().to(get_state))
            .route("/phase_data", web::get().to(phase_data))
            .route("/draw_event", web::get().to(draw_event))
            .route("/submit_turn", web::post().to(submit_turn))
            .route("/endings", web::get().to(get_ending))
            .route("/timeline", web::get().to(get_timeline))
            .route("/jobs", web::get().to(get_jobs))
            // Debug endpoints
            .route("/debug/skip_stage", web::post().to(debug_skip_stage))
            .route("/debug/set_stats", web::post().to(debug_set_stats))
            .route("/debug/grant_tag", web::post().to(debug_grant_tag))
    );
}
