use crate::domain::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue, ClassName, PassiveData,
    SkillLineName,
};
use once_cell::sync::Lazy;

pub static DUAL_WIELD_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        PassiveData::new(
            "Slaughter",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![BonusData::new(
                "Slaughter",
                BonusSource::Passive,
                BonusTrigger::DualWieldEquipped,
                BonusValue::new("Slaughter", BonusTarget::Damage, 0.20),
            )
            .with_execute_threshold(0.25)
            .with_skill_line_filter(SkillLineName::DualWield)],
        ),
        // TODO: Requires weapon stats tracking
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
                "Twin Blade and Blunt (Axe)",
                BonusSource::Passive,
                BonusTrigger::DualWieldEquipped,
                BonusValue::new(
                    "Twin Blade and Blunt (Axe)",
                    BonusTarget::CriticalDamage,
                    0.06,
                ),
            )
            .with_alternative(BonusValue::new(
                "Twin Blade and Blunt (Mace)",
                BonusTarget::PhysicalAndSpellPenetration,
                1487.0,
            ))
            .with_alternative(BonusValue::new(
                "Twin Blade and Blunt (Sword)",
                BonusTarget::WeaponAndSpellDamageFlat,
                129.0,
            ))
            .with_alternative(BonusValue::new(
                "Twin Blade and Blunt (Dagger)",
                BonusTarget::CriticalRating,
                657.0,
            ))],
        ),
    ]
});

