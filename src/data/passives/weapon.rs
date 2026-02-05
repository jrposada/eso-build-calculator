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
        // === DESTRUCTION STAFF ===
        // Tri Focus: Staff-specific Heavy Attack bonuses
        // - Inferno: +4480 Flame Damage over 20s on fully-charged HA
        // - Lightning: Fully-charged HA damages nearby enemies for 100%
        // - Ice: Fully-charged HA grants 5280 damage shield
        // TODO: Requires staff type tracking
        PassiveData::new(
            "Tri Focus",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![],
        ),
        // Penetrating Magic: Destro Staff abilities ignore 2974 Spell Resistance
        PassiveData::new(
            "Penetrating Magic",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![BonusData::new(
                "Penetrating Magic",
                BonusTrigger::DestructionStuffEquipped,
                BonusTarget::PhysicalAndSpellPenetration,
                2974.0,
            )],
        ),
        // Elemental Force: +100% status effect application chance
        // Not tracked - status effect chance
        PassiveData::new(
            "Elemental Force",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![],
        ),
        // Ancient Knowledge: Staff-specific damage bonuses
        // - Inferno: +12% DoT and Status Effect damage
        // - Lightning: +12% Direct and Channeled damage
        // - Ice: Block cost -36%, block damage +20% (defensive, not tracked)
        PassiveData::new(
            "Ancient Knowledge (Inferno)",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![BonusData::new(
                "Ancient Knowledge (Inferno)",
                BonusTrigger::InfernoStaffEquipped,
                BonusTarget::DotDamage,
                0.12,
            )],
        ),
        PassiveData::new(
            "Ancient Knowledge (Lightning)",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![BonusData::new(
                "Ancient Knowledge (Lightning)",
                BonusTrigger::LightningStaffEquipped,
                BonusTarget::DirectDamage,
                0.12,
            )],
        ),
        PassiveData::new(
            "Ancient Knowledge (Ice)",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![], // Block cost -36%, block damage +20% - defensive, not tracked
        ),
        // Destruction Expert: Restore 3600 Magicka on kill, 1800 on shield absorb
        // Not tracked - resource recovery
        PassiveData::new(
            "Destruction Expert",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            vec![],
        ),
        // === DUAL WIELD ===
        // Slaughter: +20% Dual Wield damage against enemies below 25% Health
        PassiveData::new(
            "Slaughter",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![BonusData::new(
                "Slaughter",
                BonusTrigger::DualWieldEquipped,
                BonusTarget::Damage,
                0.20,
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
        // Controlled Fury: Reduces Stamina cost of Dual Wield abilities by 15%
        // Not tracked - cost reduction
        PassiveData::new(
            "Controlled Fury",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![],
        ),
        // Twin Blade and Blunt: Weapon-type dependent bonuses (per weapon):
        // - Axe: +6% Critical Damage
        // - Mace: +1487 Offensive Penetration
        // - Sword: +129 Weapon and Spell Damage
        // - Dagger: +657 Critical Chance
        PassiveData::new(
            "Twin Blade and Blunt (Axe)",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![BonusData::new(
                "Twin Blade and Blunt (Axe)",
                BonusTrigger::DualWieldAxeEquipped,
                BonusTarget::CriticalDamage,
                0.06,
            )],
        ),
        PassiveData::new(
            "Twin Blade and Blunt (Mace)",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![BonusData::new(
                "Twin Blade and Blunt (Mace)",
                BonusTrigger::DualWieldMaceEquipped,
                BonusTarget::PhysicalAndSpellPenetration,
                1487.0,
            )],
        ),
        PassiveData::new(
            "Twin Blade and Blunt (Sword)",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![BonusData::new(
                "Twin Blade and Blunt (Sword)",
                BonusTrigger::DualWieldSwordEquipped,
                BonusTarget::WeaponAndSpellDamage,
                129.0,
            )],
        ),
        PassiveData::new(
            "Twin Blade and Blunt (Dagger)",
            ClassName::Weapon,
            SkillLineName::DualWield,
            vec![BonusData::new(
                "Twin Blade and Blunt (Dagger)",
                BonusTrigger::DualWieldDaggerEquipped,
                BonusTarget::CriticalChance,
                657.0,
            )],
        ),
        // === TWO HANDED ===
        // Forceful: Light/Heavy attacks damage up to 3 nearby enemies for 100% damage
        // Not tracked - cleave mechanic
        PassiveData::new(
            "Forceful",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        ),
        // Heavy Weapons: Bonus depends on weapon type:
        // - Swords: +258 Weapon and Spell Damage
        // - Axes: +12% Critical Damage
        // - Maces: +2974 Offensive Penetration
        PassiveData::new(
            "Heavy Weapons (Sword)",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![BonusData::new(
                "Heavy Weapons (Sword)",
                BonusTrigger::TwoHandedSwordEquipped,
                BonusTarget::WeaponAndSpellDamage,
                258.0,
            )],
        ),
        PassiveData::new(
            "Heavy Weapons (Axe)",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![BonusData::new(
                "Heavy Weapons (Axe)",
                BonusTrigger::TwoHandedAxeEquipped,
                BonusTarget::CriticalDamage,
                0.12,
            )],
        ),
        PassiveData::new(
            "Heavy Weapons (Mace)",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![BonusData::new(
                "Heavy Weapons (Mace)",
                BonusTrigger::TwoHandedMaceEquipped,
                BonusTarget::PhysicalAndSpellPenetration,
                2974.0,
            )],
        ),
        // Balanced Blade: Reduces Stamina cost of Two-Handed abilities by 15%
        // Not tracked - cost reduction
        PassiveData::new(
            "Balanced Blade",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        ),
        // Follow Up: +10% Two Handed damage for 4s after fully-charged Heavy Attack
        // TODO: Requires Heavy Attack tracking to implement
        PassiveData::new(
            "Follow Up",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        ),
        // Battle Rush: +30% Stamina Recovery for 10s after killing target
        // Not tracked - resource recovery
        PassiveData::new(
            "Battle Rush",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            vec![],
        )
    ]
});
