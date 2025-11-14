use crate::data::state::Player;

pub fn check_bonus_medal(player: &mut Player) {
  let mut bonus_medal = 0;
  let count_for_print = player.bonus_count;
  
  if player.bonus_count == 20 || player.bonus_count == 40 {
    bonus_medal = 1;
  } else if player.bonus_count == 60 {
    bonus_medal = 2;
    player.bonus_count = 0;
  }

  if bonus_medal > 0 {
    player.myth_medal += bonus_medal;
    println!("\n🍀 {}회 뽑기 보상: 신화 메달 {}개 획득 🍀", count_for_print, bonus_medal);
  }
}