use rand::Rng;
use crate::data::rewards::RewardType;
use crate::constants::probabilities::PROBABILITIES_MAX;
use crate::constants::probabilities::PROBABILITIES;

pub fn draw() -> RewardType {
  let mut rng = rand::thread_rng();
  let draw = rng.gen_range(0..PROBABILITIES_MAX);
  let mut probabilities_sum = 0;

  for (reward_type, prob) in PROBABILITIES.iter() {
    probabilities_sum += prob;
    if draw < probabilities_sum {
      return reward_type.clone();
    }
  }

  RewardType::KingdomCrystal
}