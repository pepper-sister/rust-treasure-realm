use crate::data::state::Player;
use crate::data::rewards::RewardType;
use std::collections::HashMap;
use crate::functions::reward_info::reward_info;

pub fn print_draw_result(player: &Player, draw_count: u32, rewards_before: &HashMap<RewardType, u32>,) {
  println!("\n===================================");
  println!("{}회 뽑기 결과 (총 {}회)", draw_count, player.total_draw_count);

  let order = [
    RewardType::Prestige,
    RewardType::Myth,
    RewardType::Legend,
    RewardType::Common,
    RewardType::KingdomCrystal,
    RewardType::MythMedal,
  ];

  for reward in order.iter() {
    if let Some(&current_count) = player.rewards.get(reward) {
      let previous_count = *rewards_before.get(reward).unwrap_or(&0);
      let new_gained = current_count.saturating_sub(previous_count);

      if new_gained > 0 {
        let (name, _) = reward_info(reward);
        println!("{} - {}개", name, new_gained);
      }
    }
  }

  println!("\n남은 TFT 코인: {}코인", player.tft_coin);
  println!("===================================");
}