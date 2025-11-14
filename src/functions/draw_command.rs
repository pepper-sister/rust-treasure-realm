use crate::functions::read_input::read_input;

pub fn draw_command(player_coin: u32) -> Result<Option<u32>, String> {
  let input = read_input("\n다음 명령 중 하나를 입력해 주세요. ( draw 1 / draw 10 / exit )").map_err(|_| "입력 오류".to_string())?;
  let parts: Vec<&str> = input.split_whitespace().collect();
  
  match parts[0] {
    "exit" => Ok(None),
    "draw" => {
      let count = parts[1].parse::<u32>().map_err(|_| "[ERROR] 뽑기 횟수는 숫자여야 합니다.".to_string())?;

      if count != 1 && count != 10 {
        return Err("[ERROR] 뽑기 횟수는 1 또는 10이어야 합니다.".to_string());
      }

      let required_coin: u32 = count * crate::constants::other_constants::COST_PER_DRAW as u32;
      if player_coin < required_coin {
        let shortage = required_coin - player_coin;
        return Err(format!("[ERROR] TFT 코인이 {}코인 부족합니다.", shortage));
      }

      Ok(Some(count))
    }
    _ => Err("[ERROR] 알 수 없는 명령어입니다. (draw 1, draw 10, exit 중 하나를 입력하세요.)".to_string()),
  }
}