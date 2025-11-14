use crate::data::state::Player;
use crate::data::rewards::RewardType;
use crate::functions::draw::draw;
use crate::functions::check_bonus_medal::check_bonus_medal;

pub fn draws(player: &mut Player, count: u32) {
  for _ in 0..count {
    player.tft_coin -= crate::constants::other_constants::COST_PER_DRAW as u32;
    player.total_draw_count += 1;
    player.bonus_count += 1;

    let reward = draw();
    
    match reward {
      RewardType::Prestige => {
        player.prestige_obtained = true;
        *player.rewards.entry(reward).or_insert(0) += 1;
      }
      RewardType::MythMedal => {
        *player.rewards.entry(reward).or_insert(0) += 1;
        player.myth_medal += 5;
      }
      _ => {
        *player.rewards.entry(reward).or_insert(0) += 1;
      }
    }

    check_bonus_medal(player);
  }
}