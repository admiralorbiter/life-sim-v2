use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Create a deterministic RNG from a seed string.
///
/// The seed string is converted to bytes and padded/truncated to 32 bytes
/// to create a ChaCha8Rng. Two identical seed strings will always produce
/// the same sequence of random numbers — this is the classroom seed-sharing
/// feature that lets teachers ensure all students face the same events.
pub fn create_rng(seed_str: &str) -> ChaCha8Rng {
    let mut seed_bytes = [0u8; 32];
    let bytes = seed_str.as_bytes();
    for (i, &b) in bytes.iter().enumerate().take(32) {
        seed_bytes[i] = b;
    }
    ChaCha8Rng::from_seed(seed_bytes)
}

/// Generate a random 8-character alphanumeric seed string.
pub fn generate_seed() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = (0..8)
        .map(|_| {
            let idx = rng.gen_range(0..36);
            if idx < 10 {
                (b'0' + idx) as char
            } else {
                (b'A' + idx - 10) as char
            }
        })
        .collect();
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_deterministic_rng() {
        let mut rng1 = create_rng("CLASSROOM2026");
        let mut rng2 = create_rng("CLASSROOM2026");

        let seq1: Vec<f64> = (0..10).map(|_| rng1.gen::<f64>()).collect();
        let seq2: Vec<f64> = (0..10).map(|_| rng2.gen::<f64>()).collect();

        assert_eq!(seq1, seq2, "Same seed must produce same sequence");
    }

    #[test]
    fn test_different_seeds_differ() {
        let mut rng1 = create_rng("SEED_A");
        let mut rng2 = create_rng("SEED_B");

        let val1: f64 = rng1.gen();
        let val2: f64 = rng2.gen();

        assert_ne!(val1, val2, "Different seeds should produce different values");
    }

    #[test]
    fn test_generate_seed_length() {
        let seed = generate_seed();
        assert_eq!(seed.len(), 8);
        assert!(seed.chars().all(|c| c.is_ascii_alphanumeric()));
    }
}
