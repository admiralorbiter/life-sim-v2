use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use crate::data_loader::GameData;
use crate::engine::game_state::GameState;
use crate::engine::rng;
use crate::engine::turn_runner::{self, PlayerChoices};
use crate::engine::event_deck;
use rand_chacha::ChaCha8Rng;

/// Shared server state: one active game per process (MVP).
pub struct AppState {
    pub game: Mutex<Option<GameState>>,
    pub rng: Mutex<Option<ChaCha8Rng>>,
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

/// POST /api/submit_turn — Submit choices and run one turn.
pub async fn submit_turn(
    app_state: web::Data<AppState>,
    game_data: web::Data<GameData>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let mut game = app_state.game.lock().unwrap();
    let mut game_rng = app_state.rng.lock().unwrap();

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

    let result = turn_runner::run_turn(state, &choices, &game_data, rng_ref);

    HttpResponse::Ok().json(serde_json::json!({
        "state": &*state,
        "turnResult": {
            "feedback": result.feedback,
            "eventDrawn": result.event_drawn,
            "stageTransitioned": result.stage_transitioned,
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

/// Configure all API routes.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health))
            .route("/new_game", web::post().to(new_game))
            .route("/state", web::get().to(get_state))
            .route("/phase_data", web::get().to(phase_data))
            .route("/submit_turn", web::post().to(submit_turn))
            .route("/endings", web::get().to(get_ending))
    );
}
