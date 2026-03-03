use crate::data::bonuses::unique::{
    MAJOR_BREACH,
};
use crate::domain::{ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static DESTRUCTION_STAFF_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        SkillData::new(
            "Elemental Storm",
            "Elemental Storm",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(7.0, DamageFlags::magic_aoe(), 0.075, 0.7875).with_interval(1.0),
        ])),
        SkillData::new(
            "Elemental Rage",
            "Elemental Storm",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(7.0, DamageFlags::magic_aoe(), 0.09685, 1.0169).with_interval(1.0),
        ])),
        SkillData::new(
            "Eye of the Storm",
            "Elemental Storm",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(7.0, DamageFlags::magic_aoe(), 0.07748, 0.81349).with_interval(1.0),
        ])),
        SkillData::new(
            "Force Shock",
            "Force Shock",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.09,
            0.945,
        )]))
        .with_spammable(),
        SkillData::new(
            "Crushing Shock",
            "Force Shock",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.09297,
            0.976185,
        )]))
        .with_spammable(),
        SkillData::new(
            "Force Pulse",
            "Force Shock",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.09297,
            0.976185,
        )]))
        .with_spammable(),
        SkillData::new(
            "Wall of Elements",
            "Wall of Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(8.0, DamageFlags::magic_aoe(), 0.012121, 0.127273).with_interval(1.0),
        ])),
        SkillData::new(
            "Elemental Blockade",
            "Wall of Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(10.0, DamageFlags::magic_aoe(), 0.012521, 0.131473).with_interval(1.0),
        ])),
        SkillData::new(
            "Unstable Wall of Elements",
            "Wall of Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_damage(
            SkillDamage::new()
                .with_dots(vec![DotDamage::new(
                    8.0,
                    DamageFlags::magic_aoe(),
                    0.012521,
                    0.131473,
                )
                .with_interval(1.0)])
                .with_hits(vec![HitDamage::new(
                    DamageFlags::magic_aoe(),
                    0.05165,
                    0.542325,
                )
                .with_delay(8.0)]),
        ),
        SkillData::new(
            "Destructive Touch",
            "Destructive Touch",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::magic_single(),
                    0.05,
                    0.525,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::magic_single(),
                    0.015,
                    0.1575,
                )]),
        ),
        SkillData::new(
            "Destructive Clench",
            "Destructive Touch",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.05165,
            0.542325,
        )])),
        SkillData::new(
            "Destructive Reach",
            "Destructive Touch",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::magic_single(),
                    0.05165,
                    0.542325,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::magic_single(),
                    0.015495,
                    0.162697,
                )]),
        ),
        SkillData::new(
            "Weakness to Elements",
            "Weakness to Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_bonuses(vec![MAJOR_BREACH.clone().with_duration(30.0)]),
        SkillData::new(
            "Elemental Drain",
            "Weakness to Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_bonuses(vec![MAJOR_BREACH.clone().with_duration(60.0)]),
        SkillData::new(
            "Elemental Susceptibility",
            "Weakness to Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_bonuses(vec![MAJOR_BREACH.clone().with_duration(30.0)]),
        SkillData::new(
            "Impulse",
            "Impulse",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_aoe(),
            0.075,
            0.7875,
        )]))
        .with_spammable(),
        SkillData::new(
            "Elemental Ring",
            "Impulse",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_aoe(),
            0.077475,
            0.813488,
        )]))
        .with_spammable(),
        SkillData::new(
            "Pulsar",
            "Impulse",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_aoe(),
            0.077475,
            0.813488,
        )]))
        .with_spammable(),
    ]
});

