use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum RandomKind {
    Os,
    Seeded,
}

pub enum Random {
    Os,
    Seeded(u32),
}

impl Random {
    pub fn below(&mut self, upper: u32) -> Result<u32, getrandom::Error> {
        // Reject the incomplete block so that every residue has the same number
        // of source values when the 32-bit input is uniform.
        let threshold = upper.wrapping_neg() % upper;
        loop {
            let number = match self {
                Self::Os => getrandom::u32()?,
                Self::Seeded(state) => {
                    // A deliberately predictable linear congruential generator.
                    *state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    *state
                }
            };
            if number >= threshold {
                return Ok(number % upper);
            }
        }
    }
}

pub fn ticket(
    mut weights: Vec<u32>,
    picks: usize,
    rng: &mut Random,
) -> Result<Vec<u32>, getrandom::Error> {
    let mut result = Vec::with_capacity(picks);
    for _ in 0..picks {
        let mut cursor = rng.below(weights.iter().sum())?;
        for (index, weight) in weights.iter_mut().enumerate() {
            if cursor < *weight {
                result.push(index as u32 + 1);
                *weight = 0;
                break;
            }
            cursor -= *weight;
        }
    }
    result.sort_unstable();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_tickets_replay_for_both_games() {
        for (max, count) in [(43, 6), (37, 7)] {
            let mut first = Random::Seeded(42);
            let mut replay = Random::Seeded(42);
            for _ in 0..20 {
                let numbers = ticket(vec![1; max], count, &mut first).unwrap();
                assert_eq!(numbers, ticket(vec![1; max], count, &mut replay).unwrap());
                assert_eq!(numbers.len(), count);
                assert!(numbers.windows(2).all(|pair| pair[0] < pair[1]));
                assert!(numbers.iter().all(|&n| (1..=max as u32).contains(&n)));
            }
        }
    }
}
