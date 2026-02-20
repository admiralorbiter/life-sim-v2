use rand_chacha::ChaCha8Rng;
use crate::engine::game_state::GameState;
use crate::engine::stat_calculator;
use crate::engine::event_deck;
use crate::data_loader::GameData;
use crate::models::{EventCard, Stage};

/// Player choices submitted for a single turn.
#[derive(Debug, Clone)]
pub struct PlayerChoices {
    /// IDs of actions selected in Phase 1 (Plan).
    pub action_ids: Vec<String>,
    /// ID of the decision selected in Phase 2 (Commit).
    pub decision_id: String,
    /// Index of the chosen option within the decision.
    pub decision_option_index: usize,
    /// Index of the chosen option for the Phase 3 event (if any).
    pub event_option_index: Option<usize>,
}

/// Result of running a single turn.
#[derive(Debug, Clone)]
pub struct TurnResult {
    /// The event card drawn in Phase 3 (if any).
    pub event_drawn: Option<EventCard>,
    /// Human-readable feedback messages from all phases.
    pub feedback: Vec<String>,
    /// Whether a stage transition occurred.
    pub stage_transitioned: bool,
    /// Stress threshold warning (if applicable).
    pub stress_warning: Option<String>,
}

/// Run one complete turn through all 4 phases.
pub fn run_turn(
    state: &mut GameState,
    choices: &PlayerChoices,
    data: &GameData,
    rng: &mut ChaCha8Rng,
) -> TurnResult {
    run_turn_with_event(state, choices, data, rng, None)
}

/// Run one complete turn, optionally with a pre-drawn event card.
/// If `pre_drawn_event` is Some, that event is used instead of drawing a new one.
pub fn run_turn_with_event(
    state: &mut GameState,
    choices: &PlayerChoices,
    data: &GameData,
    rng: &mut ChaCha8Rng,
    pre_drawn_event: Option<EventCard>,
) -> TurnResult {
    let mut feedback = Vec::new();

    // === Phase 1: Plan (Allocate Time) ===
    for action_id in &choices.action_ids {
        if let Some(action) = data.actions.iter().find(|a| a.id == *action_id) {
            let msgs = stat_calculator::apply_effects(state, &action.effects);
            feedback.extend(msgs);
        }
    }

    // === Phase 2: Commit (Make a Decision) ===
    if let Some(decision) = data.decisions.iter().find(|d| d.id == choices.decision_id) {
        if let Some(option) = decision.options.get(choices.decision_option_index) {
            let msgs = stat_calculator::apply_effects(state, &option.effects);
            feedback.extend(msgs);

            // Grant tag if this option provides one
            if let Some(ref tag) = option.grants_tag {
                if !state.credentials.contains(tag) {
                    state.credentials.push(tag.clone());
                    feedback.push(format!("📚 Earned: {}", tag));
                }
            }
        }
    }

    // === Phase 3: Event (Draw a Life Card) ===
    // Use pre-drawn event if available, otherwise draw a new one
    let event_drawn = pre_drawn_event.or_else(|| {
        event_deck::draw_event(&data.events, &state.current_stage, &state.used_event_ids, rng)
            .cloned()
    });

    if let Some(ref event) = event_drawn {
        // Mark as used (avoid duplication if already in the list)
        if !state.used_event_ids.contains(&event.id) {
            state.used_event_ids.push(event.id.clone());
        }

        // Apply event response if player chose one
        if let Some(opt_idx) = choices.event_option_index {
            if let Some(option) = event.options.get(opt_idx) {
                let msgs = stat_calculator::apply_effects(state, &option.effects);
                feedback.extend(msgs);
            }
        }
    }

    // === Phase 4: Feedback ===
    // Apply job income
    let job_msgs = stat_calculator::apply_job_income(state);
    feedback.extend(job_msgs);

    // Apply monthly bills (Stage D only)
    if state.current_stage == Stage::EarlyAdult {
        let bill_msgs = stat_calculator::apply_monthly_bills(state);
        feedback.extend(bill_msgs);
    }

    // Check stress threshold
    let stress_warning = stat_calculator::check_stress_threshold(state);
    if let Some(ref warning) = stress_warning {
        feedback.push(warning.clone());
    }

    // Advance turn
    state.current_turn += 1;

    // Check for stage transition
    let stage_transitioned = check_and_transition_stage(state);
    if stage_transitioned {
        feedback.push(format!("🎓 Advancing to {}!", state.current_stage));
    }

    TurnResult {
        event_drawn,
        feedback,
        stage_transitioned,
        stress_warning,
    }
}

/// Stage turn boundaries (inclusive end turn for each stage).
fn stage_end_turn(stage: &Stage) -> u32 {
    match stage {
        Stage::MiddleSchool => 4,   // Turns 1-4
        Stage::HighSchool => 10,    // Turns 5-10
        Stage::PostHigh => 13,      // Turns 11-13
        Stage::EarlyAdult => 19,    // Turns 14-19
    }
}

/// Check if the current turn has passed the stage boundary, and if so, transition.
fn check_and_transition_stage(state: &mut GameState) -> bool {
    let end = stage_end_turn(&state.current_stage);
    if state.current_turn > end {
        let next = next_stage(&state.current_stage);
        if let Some(next_stage) = next {
            state.current_stage = next_stage;
            // Reset time slots for new stage
            state.time_slots = 3;
            return true;
        }
    }
    false
}

/// Get the next stage in sequence, or None if at the final stage.
fn next_stage(stage: &Stage) -> Option<Stage> {
    match stage {
        Stage::MiddleSchool => Some(Stage::HighSchool),
        Stage::HighSchool => Some(Stage::PostHigh),
        Stage::PostHigh => Some(Stage::EarlyAdult),
        Stage::EarlyAdult => None,
    }
}

/// Check if the game is over (past the final turn).
pub fn is_game_over(state: &GameState) -> bool {
    state.current_turn > stage_end_turn(&Stage::EarlyAdult)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::rng::create_rng;
    use std::path::PathBuf;

    fn load_test_data() -> GameData {
        let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
        GameData::load_from_dir(&data_dir).expect("Should load test data")
    }

    #[test]
    fn test_run_single_turn() {
        let data = load_test_data();
        let mut state = GameState::new("TURN_TEST".to_string());
        let mut rng = create_rng("TURN_TEST");

        let initial_money = state.money;
        let initial_stress = state.stress;

        let choices = PlayerChoices {
            action_ids: vec!["act_study".to_string(), "act_rest".to_string()],
            decision_id: "dec_club_choice_a".to_string(),
            decision_option_index: 0, // Tech Club
            event_option_index: Some(0), // First option on whatever card is drawn
        };

        let result = run_turn(&mut state, &choices, &data, &mut rng);

        // Turn should have advanced
        assert_eq!(state.current_turn, 2);
        // Should have some feedback
        assert!(!result.feedback.is_empty(), "Should have feedback messages");
        // An event should have been drawn (we have events for middle school)
        assert!(result.event_drawn.is_some(), "Should have drawn an event card");
        // The drawn event should be marked as used
        assert!(!state.used_event_ids.is_empty(), "Should track used event");
        // Tech Club should have granted IT Fundamentals tag
        assert!(
            state.credentials.contains(&"IT Fundamentals".to_string()),
            "Tech Club should grant IT Fundamentals"
        );
    }

    #[test]
    fn test_run_three_turns() {
        let data = load_test_data();
        let mut state = GameState::new("THREE_TURNS".to_string());
        let mut rng = create_rng("THREE_TURNS");

        for turn in 0..3 {
            let choices = PlayerChoices {
                action_ids: vec!["act_study".to_string()],
                decision_id: if turn == 0 { "dec_club_choice_a" } else { "dec_effort_a" }.to_string(),
                decision_option_index: 1, // Balanced options
                event_option_index: Some(0),
            };

            let result = run_turn(&mut state, &choices, &data, &mut rng);
            assert!(!result.feedback.is_empty(), "Turn {} should produce feedback", turn + 1);
        }

        assert_eq!(state.current_turn, 4, "Should be on turn 4 after 3 turns");
        assert_eq!(
            state.used_event_ids.len(),
            3,
            "Should have used 3 event cards"
        );
        // All used events should be unique
        let mut unique = state.used_event_ids.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), state.used_event_ids.len(), "No event should repeat");
    }

    #[test]
    fn test_stage_transition() {
        let mut state = GameState::new("STAGE".to_string());
        state.current_turn = 5; // Past middle school (turns 1-4)
        let transitioned = check_and_transition_stage(&mut state);
        assert!(transitioned, "Should transition from Middle School");
        assert_eq!(state.current_stage, Stage::HighSchool);
    }

    #[test]
    fn test_game_over() {
        let mut state = GameState::new("OVER".to_string());
        state.current_turn = 20;
        assert!(is_game_over(&state));

        state.current_turn = 19;
        assert!(!is_game_over(&state));
    }

    #[test]
    fn test_stats_clamp_during_turn() {
        let data = load_test_data();
        let mut state = GameState::new("CLAMP".to_string());
        let mut rng = create_rng("CLAMP");

        // Set stress near max to test clamping
        state.stress = 95;

        let choices = PlayerChoices {
            action_ids: vec!["act_clubs".to_string()], // +2 stress
            decision_id: "dec_effort_a".to_string(),
            decision_option_index: 0, // "All in" = +10 stress
            event_option_index: Some(0),
        };

        let result = run_turn(&mut state, &choices, &data, &mut rng);

        assert!(state.stress <= 100, "Stress should clamp at 100, got {}", state.stress);
        assert!(
            result.stress_warning.is_some(),
            "Should have stress warning at > 75"
        );
    }
}
