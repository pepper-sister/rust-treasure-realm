pub enum RewardType {
  Prestige,
  Myth,
  Legend,
  Common,
  KingdomCrystal,
  MythMedal,
}

pub const REWARDS_NAMES: &[(RewardType, &'static str)] = &[
  (RewardType::Prestige, "[프레스티지]미니 영혼의 꽃 아리 🌟"),
  (RewardType::Myth, "[신화]미니 우주 그루브 블리츠크랭크 🥊"),
  (RewardType::Legend, "[전설]형님 샥샥이 🦈"),
  (RewardType::Common, "[일반]울끈불끈 집게발 🦞"),
  (RewardType::KingdomCrystal, "왕국 수정 🔮"),
  (RewardType::MythMedal, "신화 메달 🎖️"),
];