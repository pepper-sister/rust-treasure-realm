use crate::data::rewards::RewardType;
use crate::data::rewards::REWARDS_NAMES;

pub fn reward_info(reward: &RewardType) -> (&'static str, bool) {
    let name = REWARDS_NAMES.iter()
        .find(|(r_type, _)| r_type == reward)
        .map(|(_, name)| *name)
        .unwrap_or("");

    match reward {
        RewardType::Prestige => (name, false),
        RewardType::Myth => (name, false),
        RewardType::Legend => (name, false),
        RewardType::Common => (name, false),
        RewardType::KingdomCrystal => (name, false),
        RewardType::MythMedal => (name, true),
    }
}