use crate::data::bonuses::unique::{
    MAJOR_BREACH, MAJOR_BRUTALITY, MAJOR_SORCERY,
};
use crate::domain::{ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, ExecuteScaling, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static BOW_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        SkillData::new(
            "Rapid Fire",
            "Rapid Fire",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            4.0,
            DamageFlags::physical_single(),
            0.0861217,
            0.904012,
        )]))
        .with_channel_time(4.0),
        SkillData::new(
            "Ballista",
            "Rapid Fire",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            5.0,
            DamageFlags::physical_single(),
            0.0532809,
            0.559899,
        )])),
        SkillData::new(
            "Toxic Barrage",
            "Rapid Fire",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(4.0, DamageFlags::poison_single(), 0.0860872, 0.904026),
            DotDamage::new(8.0, DamageFlags::poison_single(), 0.086083, 0.903875).with_delay(1.0),
        ]))
        .with_channel_time(4.0),
        SkillData::new(
            "Snipe",
            "Snipe",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.1035,
            1.08675,
        )])),
        SkillData::new(
            "Focused Aim",
            "Snipe",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.106915,
            1.12261,
        )]))
        .with_bonuses(vec![MAJOR_BREACH.clone()]),
        SkillData::new(
            "Lethal Arrow",
            "Snipe",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::poison_single(),
            0.106915,
            1.12261,
        )])),
        SkillData::new(
            "Volley",
            "Volley",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            8.0,
            DamageFlags::physical_aoe(),
            0.014815,
            0.155556,
        )
        .with_delay(2.0)
        .with_interval(1.0)])),
        SkillData::new(
            "Arrow Barrage",
            "Volley",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            8.0,
            DamageFlags::physical_aoe(),
            0.019895,
            0.208896,
        )
        .with_delay(2.0)
        .with_interval(1.0)])),
        SkillData::new(
            "Endless Hail",
            "Volley",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            13.0,
            DamageFlags::physical_aoe(),
            0.015304,
            0.160689,
        )
        .with_delay(2.0)
        .with_interval(1.0)])),
        SkillData::new(
            "Thunderous Volley",
            "Volley",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(13.0, DamageFlags::physical_aoe(), 0.015304, 0.160689)
                .with_delay(2.0)
                .with_interval(1.0),
            DotDamage::new(13.0, DamageFlags::physical_aoe(), 0.0, 0.095636)
                .with_delay(2.0)
                .with_interval(1.0)
                .with_flat_increase_per_tick(191.0)
                .ignores_modifier(),
        ])),
        SkillData::new(
            "Scatter Shot",
            "Scatter Shot",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.06,
            0.63,
        )])),
        SkillData::new(
            "Draining Shot",
            "Scatter Shot",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.06198,
            0.65079,
        )])),
        SkillData::new(
            "Magnum Shot",
            "Scatter Shot",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.074376,
            0.780948,
        )])),
        SkillData::new(
            "Arrow Spray",
            "Arrow Spray",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.075,
            0.7875,
        )])),
        SkillData::new(
            "Acid Spray",
            "Arrow Spray",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::poison_aoe(),
                    0.077476,
                    0.81349,
                )])
                .with_dots(vec![DotDamage::new(
                    5.0,
                    DamageFlags::poison_single(),
                    0.014118,
                    0.148235,
                )]),
        ),
        SkillData::new(
            "Bombard",
            "Arrow Spray",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.077476,
            0.81349,
        )])),
        SkillData::new(
            "Poison Arrow",
            "Poison Arrow",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::poison_single(),
                    0.05,
                    0.525,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::poison_single(),
                    0.015,
                    0.1575,
                )]),
        ),
        SkillData::new(
            "Poison Injection",
            "Poison Arrow",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::poison_single(),
                    0.05165,
                    0.542325,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::poison_single(),
                    0.015495,
                    0.162697,
                )]),
        )
        .with_execute(1.2, 0.50, ExecuteScaling::Linear),
        SkillData::new(
            "Venom Arrow",
            "Poison Arrow",
            ClassName::Weapon,
            SkillLineName::Bow,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::poison_single(),
                    0.05165,
                    0.542325,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::poison_single(),
                    0.015495,
                    0.162697,
                )]),
        )
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
    ]
});

