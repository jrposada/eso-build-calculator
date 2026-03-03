use crate::data::bonuses::unique::{
    MAJOR_BRUTALITY, MAJOR_SORCERY,
};
use crate::domain::{ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, ExecuteScaling, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static DUAL_WIELD_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        SkillData::new(
            "Lacerate",
            "Lacerate",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            8.0,
            DamageFlags::bleed_aoe(),
            0.06,
            0.63,
        )])),
        SkillData::new(
            "Rend",
            "Lacerate",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            16.0,
            DamageFlags::bleed_aoe(),
            0.06198,
            0.65079,
        )])),
        SkillData::new(
            "Thrive in Chaos",
            "Lacerate",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            8.0,
            DamageFlags::bleed_aoe(),
            0.06198,
            0.65079,
        )])),
        SkillData::new(
            "Flurry",
            "Flurry",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_single(), 0.02875, 0.301875),
            HitDamage::new(DamageFlags::physical_single(), 0.02875, 0.301875),
            HitDamage::new(DamageFlags::physical_single(), 0.02875, 0.301875),
            HitDamage::new(DamageFlags::physical_single(), 0.02875, 0.301875),
        ]))
        .with_spammable(),
        SkillData::new(
            "Bloodthirst",
            "Flurry",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::bleed_single(), 0.0297, 0.31184),
            HitDamage::new(DamageFlags::bleed_single(), 0.0297, 0.31184),
            HitDamage::new(DamageFlags::bleed_single(), 0.0297, 0.31184),
            HitDamage::new(DamageFlags::bleed_single(), 0.0297, 0.31184),
        ]))
        .with_spammable(),
        SkillData::new(
            "Rapid Strikes",
            "Flurry",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_single(), 0.0297, 0.31184),
            HitDamage::new(DamageFlags::physical_single(), 0.0297, 0.31184),
            HitDamage::new(DamageFlags::physical_single(), 0.0297, 0.31184),
            HitDamage::new(DamageFlags::physical_single(), 0.0297, 0.31184),
        ]))
        .with_spammable(),
        SkillData::new(
            "Twin Slashes",
            "Twin Slashes",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![
                    HitDamage::new(DamageFlags::bleed_single(), 0.025, 0.2625),
                    HitDamage::new(DamageFlags::bleed_single(), 0.025, 0.2625),
                ])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::bleed_single(),
                    0.015,
                    0.1575,
                )]),
        ),
        SkillData::new(
            "Blood Craze",
            "Twin Slashes",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![
                    HitDamage::new(DamageFlags::bleed_single(), 0.025825, 0.271163),
                    HitDamage::new(DamageFlags::bleed_single(), 0.025825, 0.271163),
                ])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::bleed_single(),
                    0.015495,
                    0.162697,
                )]),
        ),
        SkillData::new(
            "Rending Slashes",
            "Twin Slashes",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![
                    HitDamage::new(DamageFlags::bleed_single(), 0.03099, 0.325395),
                    HitDamage::new(DamageFlags::bleed_single(), 0.03099, 0.325395),
                ])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::bleed_single(),
                    0.015495,
                    0.162697,
                )]),
        ),
        SkillData::new(
            "Whirlwind",
            "Whirlwind",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.075,
            0.7875,
        )]))
        .with_spammable()
        .with_execute(0.33, 0.50, ExecuteScaling::Linear),
        SkillData::new(
            "Steel Tornado",
            "Whirlwind",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.077476,
            0.81349,
        )]))
        .with_spammable()
        .with_execute(0.33, 0.50, ExecuteScaling::Linear),
        SkillData::new(
            "Whirling Blades",
            "Whirlwind",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.077476,
            0.81349,
        )]))
        .with_spammable()
        .with_execute(1.0, 0.50, ExecuteScaling::Linear),
        SkillData::new(
            "Blade Cloak",
            "Blade Cloak",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(20.0, DamageFlags::physical_aoe(), 0.018182, 0.19091).with_interval(2.0),
        ])),
        SkillData::new(
            "Deadly Cloak",
            "Blade Cloak",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::physical_aoe(),
            0.024417,
            0.256373,
        )
        .with_interval(2.0)])),
        SkillData::new(
            "Quick Cloak",
            "Blade Cloak",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(30.0, DamageFlags::physical_aoe(), 0.018782, 0.19721).with_interval(2.0),
        ])),
        SkillData::new(
            "Hidden Blade",
            "Hidden Blade",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.06,
            0.63,
        )]))
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
        SkillData::new(
            "Flying Blade",
            "Hidden Blade",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_single(), 0.06198, 0.65079),
            HitDamage::new(DamageFlags::physical_single(), 0.09297, 0.976185),
        ]))
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(40.0),
            MAJOR_SORCERY.clone().with_duration(40.0),
        ]),
        SkillData::new(
            "Shrouded Daggers",
            "Hidden Blade",
            ClassName::Weapon,
            SkillLineName::DualWield,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488),
            HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488),
            HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488),
        ]))
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
    ]
});

