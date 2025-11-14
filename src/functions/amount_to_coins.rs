use crate::data::state::Player;

pub fn amount_to_coins(player: &mut Player, mut amount: u32, tft_coin: &[(u32, u32)]) {
  for (price, coins) in tft_coin.iter() {
    while amount >= *price {
        player.tft_coin += coins;
        amount -= *price;
    }
  }
}
