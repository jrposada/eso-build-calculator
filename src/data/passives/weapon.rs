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
                BonusTrigger::BowEquipped,
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
                BonusTrigger::BowEquipped,
                BonusTarget::CriticalChance,
                1314.0,
            )],
        ),
        PassiveData::new(
            "Ranger",
             ClassName::Weapon,
              SkillLineName::Bow,
               vec![], // Reduce stamina cost of bow abilities 15%
            ),
        PassiveData::new(
            "Hawk Eye",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![], // TODO: To complex, Stacks per basic of increase damage
        ),
        PassiveData::new(
            "Hasty Retreat",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![], // Mayor expedition
        ),
        // === DESTRUCTION STAFF === // TODO
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
            vec![],
        ),
        PassiveData::new(
            "Destruction Expert",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![],
        ),
        // === DUAL WIELD === // TODO
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
        // === TWO HANDED === // TODO
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
            vec![],
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
        PassiveData::new(
            "Battle Rush",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        )
    ]
});
