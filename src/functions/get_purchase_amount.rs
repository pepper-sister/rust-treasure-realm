use crate::functions::read_input::read_input;
use crate::constants::other_constants::MIN_PURCHASE_AMOUNT;

pub fn get_purchase_amount() -> u32 {
  loop {
    if let Ok(input) = read_input("\n뽑기를 진행할 금액을 입력해 주세요.") {
      match input.parse::<u32>() {
        Ok (amount) if amount >= MIN_PURCHASE_AMOUNT as u32 => return amount,
        Ok(_) => println!("[ERROR] 구입 금액은 최소 {}원 이상 입력해야 합니다.", MIN_PURCHASE_AMOUNT),
        Err(_) => println!("[ERROR] 올바른 숫자 형식이 아닙니다."),
      }
    }
  }
}
