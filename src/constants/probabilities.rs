use crate::data::rewards::RewardType;

pub const PROBABILITIES_MAX: u32 = 10000000;

pub const PROBABILITIES: &[(RewardType, u32)] = &[
  (RewardType::Prestige, 1000),
  (RewardType::Myth, 30000),
  (RewardType::Legend, 500000),
  (RewardType::Common, 4000000),
  (RewardType::KingdomCrystal, 5438995),
  (RewardType::MythMedal, 30005),
];