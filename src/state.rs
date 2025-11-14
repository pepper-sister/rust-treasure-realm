use std::collections::HashMap;

pub struct Player {
  pub tft_coin: u32,
  pub myth_medal: u32,
  pub total_draw_count: u32,
  pub bonus_count: u32,
  pub rewards: HashMap<RewardType, u32>,
  pub prestige_obtained: bool,
}

impl Player {
  pub fn new() -> Self {
    Player {
      tft_coin: 0,
      myth_medal: 0,
      total_draw_count: 0,
      bonus_count: 0,
      rewards: HashMap::new(),
      prestige_obtained: false,
    }
  }
}