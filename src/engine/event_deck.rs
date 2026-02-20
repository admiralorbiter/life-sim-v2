use rand::Rng;
use rand_chacha::ChaCha8Rng;
use crate::models::{EventCard, Stage, Rarity};

/// Draw a stage-appropriate event card from the deck, weighted by rarity,
/// without repeating cards already used in this playthrough.
pub fn draw_event<'a>(
    all_events: &'a [EventCard],
    stage: &Stage,
    used_ids: &[String],
    rng: &mut ChaCha8Rng,
) -> Option<&'a EventCard> {
    // Filter to eligible cards: matching stage, not yet used
    let eligible: Vec<&EventCard> = all_events
        .iter()
        .filter(|e| e.stages.contains(stage) && !used_ids.contains(&e.id))
        .collect();

    if eligible.is_empty() {
        return None;
    }

    // Weighted draw by rarity
    let weights: Vec<f64> = eligible.iter().map(|e| rarity_weight(&e.rarity)).collect();
    let total: f64 = weights.iter().sum();
    let mut roll: f64 = rng.gen::<f64>() * total;

    for (i, weight) in weights.iter().enumerate() {
        roll -= weight;
        if roll <= 0.0 {
            return Some(eligible[i]);
        }
    }

    // Fallback (should not reach here due to float math, but just in case)
    Some(eligible.last().unwrap())
}

/// Rarity weights: Common cards appear more often.
fn rarity_weight(rarity: &Rarity) -> f64 {
    match rarity {
        Rarity::Common => 3.0,
        Rarity::Uncommon => 2.0,
        Rarity::Rare => 1.0,
    }
}

/// Get available events for a given stage (for preview/debugging).
pub fn available_events<'a>(
    all_events: &'a [EventCard],
    stage: &Stage,
    used_ids: &[String],
) -> Vec<&'a EventCard> {
    all_events
        .iter()
        .filter(|e| e.stages.contains(stage) && !used_ids.contains(&e.id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::rng::create_rng;

    fn make_test_events() -> Vec<EventCard> {
        vec![
            EventCard {
                id: "evt_1".to_string(),
                title: "Event 1".to_string(),
                flavor_text: "Test".to_string(),
                stages: vec![Stage::MiddleSchool],
                rarity: Rarity::Common,
                options: vec![],
            },
            EventCard {
                id: "evt_2".to_string(),
                title: "Event 2".to_string(),
                flavor_text: "Test".to_string(),
                stages: vec![Stage::MiddleSchool, Stage::HighSchool],
                rarity: Rarity::Uncommon,
                options: vec![],
            },
            EventCard {
                id: "evt_3".to_string(),
                title: "Event 3".to_string(),
                flavor_text: "Test".to_string(),
                stages: vec![Stage::HighSchool],
                rarity: Rarity::Rare,
                options: vec![],
            },
            EventCard {
                id: "evt_4".to_string(),
                title: "Event 4".to_string(),
                flavor_text: "Test".to_string(),
                stages: vec![Stage::MiddleSchool],
                rarity: Rarity::Common,
                options: vec![],
            },
        ]
    }

    #[test]
    fn test_filter_by_stage() {
        let events = make_test_events();
        let available = available_events(&events, &Stage::MiddleSchool, &[]);
        assert_eq!(available.len(), 3, "Should find 3 middle school events");

        let available = available_events(&events, &Stage::HighSchool, &[]);
        assert_eq!(available.len(), 2, "Should find 2 high school events");
    }

    #[test]
    fn test_no_repeat_draw() {
        let events = make_test_events();
        let mut rng = create_rng("TESTDRAW");
        let used = vec!["evt_1".to_string()];

        // Draw multiple times — evt_1 should never appear
        for _ in 0..20 {
            let card = draw_event(&events, &Stage::MiddleSchool, &used, &mut rng);
            assert!(card.is_some());
            assert_ne!(card.unwrap().id, "evt_1", "Used card should never be drawn");
        }
    }

    #[test]
    fn test_empty_deck_returns_none() {
        let events = make_test_events();
        let mut rng = create_rng("EMPTY");
        // Mark all middle school events as used
        let used = vec!["evt_1".to_string(), "evt_2".to_string(), "evt_4".to_string()];
        let card = draw_event(&events, &Stage::MiddleSchool, &used, &mut rng);
        assert!(card.is_none(), "Should return None when all cards used");
    }

    #[test]
    fn test_deterministic_draw() {
        let events = make_test_events();
        let mut rng1 = create_rng("SAME_SEED");
        let mut rng2 = create_rng("SAME_SEED");

        let card1 = draw_event(&events, &Stage::MiddleSchool, &[], &mut rng1);
        let card2 = draw_event(&events, &Stage::MiddleSchool, &[], &mut rng2);

        assert_eq!(card1.unwrap().id, card2.unwrap().id, "Same seed should draw same card");
    }

    #[test]
    fn test_rarity_weighting() {
        let events = make_test_events();
        let mut rng = create_rng("RARITY");
        let mut common_count = 0;
        let mut uncommon_count = 0;

        // Draw 100 times from a fresh deck each time (no used tracking)
        for i in 0..100 {
            let mut rng_iter = create_rng(&format!("RARITY{}", i));
            if let Some(card) = draw_event(&events, &Stage::MiddleSchool, &[], &mut rng_iter) {
                match card.rarity {
                    Rarity::Common => common_count += 1,
                    Rarity::Uncommon => uncommon_count += 1,
                    Rarity::Rare => {} // evt_3 is high school only
                }
            }
        }

        assert!(
            common_count > uncommon_count,
            "Common cards ({}) should be drawn more often than uncommon ({})",
            common_count,
            uncommon_count
        );
    }
}
