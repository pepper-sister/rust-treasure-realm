use crate::functions::read_input::read_input;

pub fn get_purchase_amount() -> u32 {
  loop {
    if let Ok(input) = read_input("뽑기를 진행할 금액을 입력해 주세요.") {
    if let Ok(amount) = input.parse::<u32>() {
      return amount;
      }
    }
  }
}
