mod constants;
use crate::constants::tft_coin::TFT_COIN;
mod data;
use crate::data::state::Player;
mod functions;
use crate::functions::get_purchase_amount::get_purchase_amount;
use crate::functions::amount_to_coins::amount_to_coins;
mod gacha;
use crate::gacha::gacha;

fn main() {
  let mut player = Player::new();

  let purchase_amount = get_purchase_amount();
  amount_to_coins(&mut player, purchase_amount, TFT_COIN);

  gacha(&mut player);
}
