use crate::data::{BonusTarget, BonusTrigger, ClassName, SkillLineName};
use crate::domain::{BonusData, PassiveData};
use crate::services::BonusService;
use once_cell::sync::Lazy;

pub static WEAPON_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    // Long Shots: 5% damage when close (≤15m), 1314 crit rating when far (>15m)
    // Calculate breakpoint once during initialization
    let long_shots_breakpoint = BonusService::calculate_breakpoint(
        BonusTarget::Damage,
        0.05,
        BonusTarget::CriticalChance,
        1314.0,
    )
    .expect("Long Shots breakpoint calculation should succeed");

    vec![
        // === BOW ===
        PassiveData::new(
            "Long Shots",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![BonusData::new(
                "Long Shots",
                BonusTrigger::SkillLineSlotted,
                BonusTarget::Damage,
                0.05,
            )
            .with_alternative(
                BonusTarget::CriticalChance,
                1314.0,
                long_shots_breakpoint,
            )],
        ),
        PassiveData::new(
            "Accuracy",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![BonusData::new(
                "Accuracy",
                BonusTrigger::SkillLineSlotted,
                BonusTarget::CriticalChance,
                0.08,
            )],
        ),
        PassiveData::new("Ranger", ClassName::Weapon, SkillLineName::Bow, vec![]),
        PassiveData::new(
            "Hawk Eye",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![BonusData::new(
                "Hawk Eye",
                BonusTrigger::AbilitySlottedCount,
                BonusTarget::CriticalDamage,
                0.02,
            )],
        ),
        // === DUAL WIELD ===
        PassiveData::new(
            "Slaughter",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![],
        ),
        PassiveData::new(
            "Dual Wield Expert",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![],
        ),
        PassiveData::new(
            "Controlled Fury",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![],
        ),
        PassiveData::new(
            "Twin Blade and Blunt",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![BonusData::new(
                "Twin Blade and Blunt",
                BonusTrigger::SkillLineSlotted,
                BonusTarget::CriticalDamage,
                0.05,
            )],
        ),
        // === TWO HANDED ===
        PassiveData::new(
            "Forceful",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        ),
        PassiveData::new(
            "Heavy Weapons",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![BonusData::new(
                "Heavy Weapons",
                BonusTrigger::SkillLineSlotted,
                BonusTarget::CriticalDamage,
                0.12,
            )],
        ),
        PassiveData::new(
            "Balanced Blade",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        ),
        PassiveData::new(
            "Follow Up",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        ),
        // === DESTRUCTION STAFF ===
        PassiveData::new(
            "Tri Focus",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![],
        ),
        PassiveData::new(
            "Penetrating Magic",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![],
        ),
        PassiveData::new(
            "Elemental Force",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![],
        ),
        PassiveData::new(
            "Ancient Knowledge",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![BonusData::new(
                "Ancient Knowledge",
                BonusTrigger::AbilitySlottedCount,
                BonusTarget::CriticalChance,
                0.04,
            )],
        ),
    ]
});
