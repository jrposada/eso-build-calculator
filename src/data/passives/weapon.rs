use crate::domain::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue, ClassName, PassiveData,
    SkillLineName,
};
use once_cell::sync::Lazy;

pub static WEAPON_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === BOW ===
        PassiveData::new(
            "Long Shots",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![BonusData::new(
                "Long Shots",
                BonusSource::Passive,
                BonusTrigger::BowEquipped,
                BonusValue::new("Long Shots (Damage)", BonusTarget::Damage, 0.05),
            )
            .with_alternative(BonusValue::new(
                "Long Shots (Crit Rating)",
                BonusTarget::CriticalRating,
                1314.0,
            ))],
        ),
        PassiveData::new(
            "Accuracy",
            ClassName::Weapon,
            SkillLineName::Bow,
            vec![BonusData::new(
                "Accuracy",
                BonusSource::Passive,
                BonusTrigger::BowEquipped,
                BonusValue::new("Accuracy", BonusTarget::CriticalRating, 1314.0),
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
        // === DESTRUCTION STAFF ===
        // Tri Focus: Staff-specific Heavy Attack bonuses
        // - Inferno: +4480 Flame Damage over 20s on fully-charged HA
        // - Lightning: Fully-charged HA damages nearby enemies for 100%
        // - Ice: Fully-charged HA grants 5280 damage shield
        // TODO: Requires HA mechanic
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
            vec![BonusData::new(
                "Penetrating Magic",
                BonusSource::Passive,
                BonusTrigger::DestructionStuffEquipped,
                BonusValue::new(
                    "Penetrating Magic",
                    BonusTarget::PhysicalAndSpellPenetration,
                    2974.0,
                ),
            )
            .with_skill_line_filter(SkillLineName::Bow)],
        ),
        PassiveData::new(
            "Elemental Force",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![BonusData::new(
                "Elemental Force",
                BonusSource::Passive,
                BonusTrigger::DestructionStuffEquipped,
                BonusValue::new("Elemental Force", BonusTarget::StatusEffectChance, 1.0),
            )],
        ),
        PassiveData::new(
            "Ancient Knowledge",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![BonusData::new(
                "Ancient Knowledge (Inferno)",
                BonusSource::Passive,
                BonusTrigger::DestructionStuffEquipped,
                BonusValue::new("Ancient Knowledge (Inferno)", BonusTarget::DotDamage, 0.12),
            )
            .with_alternative(BonusValue::new(
                "Ancient Knowledge (Lightning)",
                BonusTarget::DirectDamage,
                0.12,
            ))],
            // Ignore Ice alternative -> Block cost -36%, block damage +20% - defensive, not tracked
        ),
        PassiveData::new(
            "Destruction Expert",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![], //Not tracked - resource recovery on kill
        ),
        // === DUAL WIELD ===
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
        // Dual Wield Expert: +6% of off-hand weapon damage to Weapon and Spell Damage
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
            vec![], // Not tracked - cost reduction
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
        // === TWO HANDED ===
        PassiveData::new(
            "Forceful",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![], // Not tracked - cleave mechanic
        ),
        PassiveData::new(
            "Heavy Weapons",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![BonusData::new(
                "Heavy Weapons",
                BonusSource::Passive,
                BonusTrigger::TwoHandedEquipped,
                BonusValue::new(
                    "Heavy Weapons (Sword)",
                    BonusTarget::WeaponAndSpellDamageFlat,
                    258.0,
                ),
            )
            .with_alternative(BonusValue::new(
                "Heavy Weapons (Axe)",
                BonusTarget::CriticalDamage,
                0.12,
            ))
            .with_alternative(BonusValue::new(
                "Heavy Weapons (Mace)",
                BonusTarget::PhysicalAndSpellPenetration,
                2974.0,
            ))],
        ),
        PassiveData::new(
            "Balanced Blade",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![], // Not tracked - cost reduction
        ),
        // Follow Up: +10% Two Handed damage for 4s after fully-charged Heavy Attack
        // TODO: Requires Heavy Attack tracking to implement
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
            vec![], // Not tracked - resource recovery
        ),
    ]
});
