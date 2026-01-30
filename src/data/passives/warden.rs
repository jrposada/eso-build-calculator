use crate::data::{BonusClassName, BonusType, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
use once_cell::sync::Lazy;

pub static WARDEN_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === ANIMAL COMPANIONS ===
        PassiveData::new("Bond With Nature", ClassName::Warden, SkillLineName::AnimalCompanions, vec![]),
        PassiveData::new("Savage Beast", ClassName::Warden, SkillLineName::AnimalCompanions, vec![]),
        PassiveData::new("Flourish", ClassName::Warden, SkillLineName::AnimalCompanions, vec![]),
        PassiveData::new("Advanced Species", ClassName::Warden, SkillLineName::AnimalCompanions, vec![
            BonusData::new(BonusClassName::AbilitySlottedCount, BonusType::CriticalDamage, 0.03),
        ]),

        // === GREEN BALANCE ===
        PassiveData::new("Accelerated Growth", ClassName::Warden, SkillLineName::GreenBalance, vec![]),
        PassiveData::new("Nature's Gift", ClassName::Warden, SkillLineName::GreenBalance, vec![]),
        PassiveData::new("Emerald Moss", ClassName::Warden, SkillLineName::GreenBalance, vec![]),
        PassiveData::new("Maturation", ClassName::Warden, SkillLineName::GreenBalance, vec![]),

        // === WINTER'S EMBRACE ===
        PassiveData::new("Glacial Presence", ClassName::Warden, SkillLineName::WintersEmbrace, vec![]),
        PassiveData::new("Frozen Armor", ClassName::Warden, SkillLineName::WintersEmbrace, vec![]),
        PassiveData::new("Icy Aura", ClassName::Warden, SkillLineName::WintersEmbrace, vec![]),
        PassiveData::new("Piercing Cold", ClassName::Warden, SkillLineName::WintersEmbrace, vec![]),
    ]
});
