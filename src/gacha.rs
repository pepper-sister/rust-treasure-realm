use crate::data::state::Player;
use crate::functions::draw_command::draw_command;
use crate::functions::draws::draws;
use crate::functions::print_draw_result::print_draw_result;
use crate::functions::final_result::final_result;

pub fn gacha(player: &mut Player) {
  loop {
    if player.tft_coin < crate::constants::other_constants::COST_PER_DRAW as u32 {
      println!("[ERROR] TFT 코인이 부족하여 뽑기를 중단합니다.");
      break;
    }

    if player.prestige_obtained {
        break;
    }

    match draw_command(player.tft_coin) {
      Ok(Some(count)) => {      
        let rewards_before_draw = player.rewards.clone(); 

        draws(player, count);
        print_draw_result(&player, count, &rewards_before_draw);

        if player.prestige_obtained {
            break;
        }
      }
      Ok(None) => {
        break;
      }
      Err(e) => {
        println!("{}", e);
      }
    }
  }

  final_result(player)
}