use crate::data::bonuses::unique::{
    EMPOWER, MAJOR_BERSERK, MAJOR_BRUTALITY, MAJOR_SORCERY,
};
use crate::domain::{ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, ExecuteScaling, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static TWO_HANDED_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        SkillData::new(
            "Berserker Strike",
            "Berserker Strike",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.15,
            1.575,
        )])),
        SkillData::new(
            "Berserker Rage",
            "Berserker Strike",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.15495,
            1.62698,
        )])),
        SkillData::new(
            "Onslaught",
            "Berserker Strike",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.15495,
            1.62698,
        )])),
        SkillData::new(
            "Uppercut",
            "Uppercut",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.115,
            1.2075,
        )]))
        .with_spammable(),
        SkillData::new(
            "Dizzying Swing",
            "Uppercut",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.118795,
            1.24735,
        )]))
        .with_spammable(),
        SkillData::new(
            "Wrecking Blow",
            "Uppercut",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.118795,
            1.24735,
        )]))
        .with_spammable()
        .with_bonuses(vec![
            EMPOWER.clone().with_duration(3.0),
            MAJOR_BERSERK.clone().with_duration(3.0),
        ]),
        SkillData::new(
            "Critical Charge",
            "Critical Charge",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.06,
            0.63,
        )])),
        SkillData::new(
            "Critical Rush",
            "Critical Charge",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.06198,
            0.65079,
        )])),
        SkillData::new(
            "Stampede",
            "Critical Charge",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::physical_aoe(),
                    0.06198,
                    0.65079,
                )])
                .with_dots(vec![DotDamage::new(
                    15.0,
                    DamageFlags::physical_aoe(),
                    0.013773,
                    0.14462,
                )
                .with_interval(1.0)]),
        ),
        SkillData::new(
            "Cleave",
            "Cleave",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.075,
            0.7875,
        )])),
        SkillData::new(
            "Brawler",
            "Cleave",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.077475,
            0.81349,
        )])),
        SkillData::new(
            "Carve",
            "Cleave",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::bleed_aoe(),
                    0.077475,
                    0.81349,
                )])
                .with_dots(vec![DotDamage::new(
                    12.0,
                    DamageFlags::bleed_single(),
                    0.127746,
                    1.3478,
                )]),
        ),
        SkillData::new(
            "Reverse Slash",
            "Reverse Slash",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.05,
            0.525,
        )]))
        .with_spammable()
        .with_execute(3.0, 0.50, ExecuteScaling::Linear),
        SkillData::new(
            "Executioner",
            "Reverse Slash",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::bleed_single(),
            0.05165,
            0.54233,
        )]))
        .with_spammable()
        .with_execute(4.0, 0.50, ExecuteScaling::Linear),
        SkillData::new(
            "Reverse Slice",
            "Reverse Slash",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.05165,
            0.54233,
        )]))
        .with_spammable()
        .with_execute(3.0, 0.50, ExecuteScaling::Linear),
        SkillData::new(
            "Momentum",
            "Momentum",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
        SkillData::new(
            "Forward Momentum",
            "Momentum",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(40.0),
            MAJOR_SORCERY.clone().with_duration(40.0),
        ]),
        SkillData::new(
            "Rally",
            "Momentum",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
    ]
});

