use crate::engine::game_state::GameState;
use crate::models::event::{StatEffect, StatType};

/// Clamp ranges for each stat.
#[allow(dead_code)]
const MONEY_MIN: i32 = 0;
const STRESS_MIN: i32 = 0;
const STRESS_MAX: i32 = 100;
const SUPPORT_MIN: i32 = 0;
const SUPPORT_MAX: i32 = 10;
const TIME_SLOTS_MIN: u32 = 0;
const TIME_SLOTS_MAX: u32 = 4;

/// Stress threshold: above this, outcomes degrade.
pub const STRESS_DANGER: i32 = 75;
/// Support threshold: above this, free mitigation available.
#[allow(dead_code)]
pub const SUPPORT_BONUS: i32 = 7;
/// Money threshold: at or below 0, triggers debt card.
#[allow(dead_code)]
pub const MONEY_DANGER: i32 = 0;

/// Apply a list of stat effects to the game state, with clamping.
/// Returns a list of human-readable feedback strings describing what changed.
pub fn apply_effects(state: &mut GameState, effects: &[StatEffect]) -> Vec<String> {
    let mut feedback = Vec::new();

    for effect in effects {
        match effect.stat {
            StatType::Money => {
                let before = state.money;
                state.money += effect.delta;
                // No floor clamp — money CAN go negative (debt).
                let actual = state.money - before;
                if actual != 0 {
                    feedback.push(format!("💰 Money {:+}", actual));
                }
            }
            StatType::Stress => {
                let before = state.stress;
                state.stress += effect.delta;
                state.stress = state.stress.clamp(STRESS_MIN, STRESS_MAX);
                let actual = state.stress - before;
                if actual != 0 {
                    feedback.push(format!("😰 Stress {:+}", actual));
                }
            }
            StatType::Support => {
                let before = state.support;
                state.support += effect.delta;
                state.support = state.support.clamp(SUPPORT_MIN, SUPPORT_MAX);
                let actual = state.support - before;
                if actual != 0 {
                    feedback.push(format!("🤝 Support {:+}", actual));
                }
            }
            StatType::TimeSlots => {
                let before = state.time_slots;
                if effect.delta >= 0 {
                    state.time_slots = state.time_slots.saturating_add(effect.delta as u32);
                } else {
                    state.time_slots = state.time_slots.saturating_sub(effect.delta.unsigned_abs());
                }
                state.time_slots = state.time_slots.clamp(TIME_SLOTS_MIN, TIME_SLOTS_MAX);
                let actual = state.time_slots as i32 - before as i32;
                if actual != 0 {
                    feedback.push(format!("⏰ Time {:+}", actual));
                }
            }
            StatType::Credentials => {
                if let Some(ref tag) = effect.tag {
                    if !state.credentials.contains(tag) {
                        state.credentials.push(tag.clone());
                        feedback.push(format!("📚 Earned: {}", tag));
                    }
                }
            }
        }
    }

    feedback
}

/// Misalignment stress penalty (missing recommended tags).
const MISALIGN_STRESS: i32 = 3;
/// Misalignment pay multiplier (75% of normal pay).
const MISALIGN_PAY_MULT: f64 = 0.75;

/// Apply job income to the game state (Phase 4).
/// If the player is missing recommendedTags, they get reduced pay and extra stress.
pub fn apply_job_income(state: &mut GameState) -> Vec<String> {
    let mut feedback = Vec::new();
    if let Some(ref job) = state.current_job {
        // Check misalignment: missing any recommended tags?
        let missing_recommended: Vec<&String> = job.recommended_tags.iter()
            .filter(|tag| !state.credentials.contains(tag))
            .collect();
        let misaligned = !missing_recommended.is_empty();

        // Calculate pay (reduced if misaligned)
        let pay = if misaligned {
            (job.pay_per_turn as f64 * MISALIGN_PAY_MULT) as i32
        } else {
            job.pay_per_turn
        };

        // Calculate stress (extra if misaligned)
        let stress = job.stress_per_turn + if misaligned { MISALIGN_STRESS } else { 0 };

        state.money += pay;
        state.stress += stress;
        state.stress = state.stress.clamp(STRESS_MIN, STRESS_MAX);

        feedback.push(format!("💼 {} pay: +${}", job.title, pay));
        if stress > 0 {
            feedback.push(format!("😰 Work stress: +{}", stress));
        }
        if misaligned {
            let tags: Vec<&str> = missing_recommended.iter().map(|s| s.as_str()).collect();
            feedback.push(format!("⚠️ Misaligned — missing: {}", tags.join(", ")));
        }
    }
    feedback
}

/// Apply monthly bills (Stage D only, Phase 4).
pub fn apply_monthly_bills(state: &mut GameState) -> Vec<String> {
    let mut feedback = Vec::new();
    if state.monthly_bills > 0 {
        state.money -= state.monthly_bills;
        feedback.push(format!("🏠 Bills: -${}", state.monthly_bills));
        if state.money < 0 {
            feedback.push("⚠️ You're in debt! Bills exceeded your cash.".to_string());
        }
    }
    feedback
}

/// Apply emergency fund to cover debt (Stage D, after bills).
/// If money is negative and we have an emergency fund, draw from it.
pub fn apply_emergency_fund(state: &mut GameState) -> Vec<String> {
    let mut feedback = Vec::new();
    if state.money < 0 && state.emergency_fund > 0 {
        let shortfall = (-state.money) as i32;
        let covered = shortfall.min(state.emergency_fund);
        state.money += covered;
        state.emergency_fund -= covered;
        feedback.push(format!("🏦 Emergency fund covered ${} (remaining: ${})", covered, state.emergency_fund));
        if state.money >= 0 {
            feedback.push("✅ Debt cleared by emergency fund!".to_string());
        }
    }
    feedback
}

/// Check stress threshold and return warning if applicable.
pub fn check_stress_threshold(state: &GameState) -> Option<String> {
    if state.stress > STRESS_DANGER {
        Some("⚠️ Stress is dangerously high! Risk of missed day and degraded outcomes.".to_string())
    } else {
        None
    }
}

/// Check if support is high enough for bonus mitigation.
#[allow(dead_code)]
pub fn has_support_bonus(state: &GameState) -> bool {
    state.support > SUPPORT_BONUS
}

/// Check if player is in debt.
#[allow(dead_code)]
pub fn is_in_debt(state: &GameState) -> bool {
    state.money <= MONEY_DANGER
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state() -> GameState {
        GameState::new("TEST".to_string())
    }

    fn money_effect(delta: i32) -> StatEffect {
        StatEffect { stat: StatType::Money, delta, tag: None }
    }

    fn stress_effect(delta: i32) -> StatEffect {
        StatEffect { stat: StatType::Stress, delta, tag: None }
    }

    fn support_effect(delta: i32) -> StatEffect {
        StatEffect { stat: StatType::Support, delta, tag: None }
    }

    fn time_effect(delta: i32) -> StatEffect {
        StatEffect { stat: StatType::TimeSlots, delta, tag: None }
    }

    fn credential_effect(tag: &str) -> StatEffect {
        StatEffect { stat: StatType::Credentials, delta: 0, tag: Some(tag.to_string()) }
    }

    #[test]
    fn test_apply_money_positive() {
        let mut state = make_state();
        let fb = apply_effects(&mut state, &[money_effect(50)]);
        assert_eq!(state.money, 150);
        assert!(fb[0].contains("+50"));
    }

    #[test]
    fn test_apply_money_negative() {
        let mut state = make_state();
        let fb = apply_effects(&mut state, &[money_effect(-80)]);
        assert_eq!(state.money, 20);
        assert!(fb[0].contains("-80"));
    }

    #[test]
    fn test_money_can_go_negative() {
        let mut state = make_state(); // money = 100
        apply_effects(&mut state, &[money_effect(-200)]);
        assert_eq!(state.money, -100, "Money should be able to go negative (debt)");
    }

    #[test]
    fn test_stress_clamps_at_100() {
        let mut state = make_state(); // stress = 20
        apply_effects(&mut state, &[stress_effect(90)]);
        assert_eq!(state.stress, 100, "Stress should clamp at 100");
    }

    #[test]
    fn test_stress_clamps_at_zero() {
        let mut state = make_state(); // stress = 20
        apply_effects(&mut state, &[stress_effect(-50)]);
        assert_eq!(state.stress, 0, "Stress should clamp at 0");
    }

    #[test]
    fn test_support_clamps_at_10() {
        let mut state = make_state(); // support = 5
        apply_effects(&mut state, &[support_effect(8)]);
        assert_eq!(state.support, 10, "Support should clamp at 10");
    }

    #[test]
    fn test_support_clamps_at_zero() {
        let mut state = make_state(); // support = 5
        apply_effects(&mut state, &[support_effect(-10)]);
        assert_eq!(state.support, 0, "Support should clamp at 0");
    }

    #[test]
    fn test_time_slots_clamp_at_4() {
        let mut state = make_state(); // time_slots = 3
        apply_effects(&mut state, &[time_effect(5)]);
        assert_eq!(state.time_slots, 4, "Time should clamp at 4");
    }

    #[test]
    fn test_time_slots_clamp_at_zero() {
        let mut state = make_state(); // time_slots = 3
        apply_effects(&mut state, &[time_effect(-10)]);
        assert_eq!(state.time_slots, 0, "Time should clamp at 0");
    }

    #[test]
    fn test_credential_added() {
        let mut state = make_state();
        let fb = apply_effects(&mut state, &[credential_effect("IT Fundamentals")]);
        assert_eq!(state.credentials, vec!["IT Fundamentals"]);
        assert!(fb[0].contains("IT Fundamentals"));
    }

    #[test]
    fn test_credential_no_duplicate() {
        let mut state = make_state();
        apply_effects(&mut state, &[credential_effect("CPR")]);
        let fb = apply_effects(&mut state, &[credential_effect("CPR")]);
        assert_eq!(state.credentials.len(), 1, "Should not add duplicate credential");
        assert!(fb.is_empty(), "No feedback for duplicate credential");
    }

    #[test]
    fn test_multiple_effects() {
        let mut state = make_state();
        let effects = vec![money_effect(-25), stress_effect(5), support_effect(-1)];
        let fb = apply_effects(&mut state, &effects);
        assert_eq!(state.money, 75);
        assert_eq!(state.stress, 25);
        assert_eq!(state.support, 4);
        assert_eq!(fb.len(), 3);
    }

    #[test]
    fn test_stress_threshold_warning() {
        let mut state = make_state();
        state.stress = 76;
        assert!(check_stress_threshold(&state).is_some());
    }

    #[test]
    fn test_stress_threshold_ok() {
        let mut state = make_state();
        state.stress = 50;
        assert!(check_stress_threshold(&state).is_none());
    }

    #[test]
    fn test_support_bonus() {
        let mut state = make_state();
        state.support = 8;
        assert!(has_support_bonus(&state));
        state.support = 7;
        assert!(!has_support_bonus(&state));
    }

    #[test]
    fn test_debt_detection() {
        let mut state = make_state();
        assert!(!is_in_debt(&state));
        state.money = 0;
        assert!(is_in_debt(&state));
    }

    #[test]
    fn test_job_income() {
        let mut state = make_state();
        state.current_job = Some(crate::models::Job {
            id: "test".to_string(),
            title: "Test Job".to_string(),
            required_tags: vec![],
            recommended_tags: vec![],
            pay_per_turn: 50,
            stress_per_turn: 3,
            growth_rate: 0,
            growth_tag: None,
            stages: vec![],
            description: "Test".to_string(),
        });
        let fb = apply_job_income(&mut state);
        assert_eq!(state.money, 150);
        assert_eq!(state.stress, 23);
        assert!(!fb.is_empty());
    }

    #[test]
    fn test_monthly_bills() {
        let mut state = make_state();
        state.monthly_bills = 75;
        let fb = apply_monthly_bills(&mut state);
        assert_eq!(state.money, 25);
        assert!(fb[0].contains("75"));
    }

    #[test]
    fn test_monthly_bills_triggers_debt() {
        let mut state = make_state();
        state.monthly_bills = 200;
        let fb = apply_monthly_bills(&mut state);
        assert!(state.money < 0 || state.money == 0);
        assert!(fb.iter().any(|f| f.contains("debt")));
    }

    #[test]
    fn test_misalignment_penalty() {
        let mut state = make_state();
        state.current_job = Some(crate::models::Job {
            id: "test".to_string(),
            title: "Test Job".to_string(),
            required_tags: vec![],
            recommended_tags: vec!["Customer Service".to_string()],
            pay_per_turn: 40,
            stress_per_turn: 4,
            growth_rate: 0,
            growth_tag: None,
            stages: vec![],
            description: "Test".to_string(),
        });
        // Player does NOT have "Customer Service" → misaligned
        let fb = apply_job_income(&mut state);
        // Pay should be 75% of 40 = 30
        assert_eq!(state.money, 130); // 100 + 30
        // Stress should be 4 + 3 = 7
        assert_eq!(state.stress, 27); // 20 + 7
        assert!(fb.iter().any(|f| f.contains("Misaligned")));
    }

    #[test]
    fn test_aligned_job_no_penalty() {
        let mut state = make_state();
        state.credentials.push("Customer Service".to_string());
        state.current_job = Some(crate::models::Job {
            id: "test".to_string(),
            title: "Test Job".to_string(),
            required_tags: vec![],
            recommended_tags: vec!["Customer Service".to_string()],
            pay_per_turn: 40,
            stress_per_turn: 4,
            growth_rate: 0,
            growth_tag: None,
            stages: vec![],
            description: "Test".to_string(),
        });
        let fb = apply_job_income(&mut state);
        assert_eq!(state.money, 140); // 100 + 40 (full pay)
        assert_eq!(state.stress, 24); // 20 + 4 (no extra)
        assert!(!fb.iter().any(|f| f.contains("Misaligned")));
    }
}
