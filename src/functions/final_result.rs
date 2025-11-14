use crate::data::state::Player;
use crate::data::rewards::RewardType;
use crate::functions::reward_info::reward_info;
use crate::functions::is_duplication::is_duplication;

pub fn final_result(player: &mut Player) {
  println!("\n뽑기가 종료되었습니다.");
  println!("===================================");
  println!("뽑기 결과 (총 {}회)", player.total_draw_count);
  
  let mut final_rewards = player.rewards.clone();

  let mut bonus_medal_duplication = 0;
  bonus_medal_duplication += is_duplication(&mut final_rewards, RewardType::Prestige);
  bonus_medal_duplication += is_duplication(&mut final_rewards, RewardType::Myth);
  
  player.myth_medal += bonus_medal_duplication;

  let order = [
    RewardType::Prestige,
    RewardType::Myth,
    RewardType::Legend,
    RewardType::Common,
    RewardType::KingdomCrystal,
    RewardType::MythMedal,
  ];

  for reward in order.iter() {
    if let Some(count) = final_rewards.get(reward) {
      if *count > 0 {
        let (name, _) = reward_info(reward);
        println!("{} - {}개", name, count);
      }
    }
  }

  println!("\n신화 메달: {}개", player.myth_medal);
  println!("TFT 코인: {}코인", player.tft_coin);
  println!("===================================");
}