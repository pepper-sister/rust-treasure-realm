use crate::data::rewards::RewardType;
use crate::constants::other_constants::MEDAL_PRESTIGE;
use crate::constants::other_constants::MEDAL_MYTH;

pub fn is_duplication(final_rewards: &mut std::collections::HashMap<RewardType, u32>, reward_type: RewardType) -> u32 {
  if let Some(count) = final_rewards.get(&reward_type).cloned() {
    if count > 1 {
      let duplication = count - 1;
      let bonus = match reward_type {
        RewardType::Prestige => duplication * MEDAL_PRESTIGE as u32,
        RewardType::Myth => duplication * MEDAL_MYTH as u32,
        _ => 0,
      };
      final_rewards.insert(reward_type, 1);
      return bonus;
    }
  }
  0
}