use crate::data::bonuses::unique::{
    MAJOR_PROPHECY, MAJOR_SAVAGERY, MINOR_BERSERK, MINOR_FORCE,
};
use crate::domain::{ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static FIGHTERS_GUILD_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        SkillData::new(
            "Dawnbreaker",
            "Dawnbreaker",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Ultimate,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::physical_aoe(),
                    0.125,
                    1.3125,
                )])
                .with_dots(vec![DotDamage::new(
                    4.0,
                    DamageFlags::physical_single(),
                    0.05,
                    0.525,
                )
                .with_interval(2.0)]),
        ),
        SkillData::new(
            "Flawless Dawnbreaker",
            "Dawnbreaker",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Ultimate,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::physical_aoe(),
                    0.129125,
                    1.35581,
                )])
                .with_dots(vec![DotDamage::new(
                    4.0,
                    DamageFlags::physical_single(),
                    0.05165,
                    0.542325,
                )
                .with_interval(2.0)]),
        )
        .with_bonuses(vec![{
            use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
            BonusData::new(
                "Flawless Dawnbreaker",
                BonusSource::Buff,
                BonusTrigger::Cast,
                BonusValue::new(
                    "Flawless Dawnbreaker",
                    BonusTarget::WeaponAndSpellDamageFlat,
                    300.0,
                ),
            )
            .with_duration(20.0)
        }]),
        SkillData::new(
            "Dawnbreaker of Smiting",
            "Dawnbreaker",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Ultimate,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::physical_aoe(),
                    0.15495,
                    1.62698,
                )])
                .with_dots(vec![DotDamage::new(
                    4.0,
                    DamageFlags::physical_single(),
                    0.06198,
                    0.65079,
                )
                .with_interval(2.0)]),
        ),
        SkillData::new(
            "Silver Bolts",
            "Silver Bolts",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.09,
            0.945,
        )])),
        SkillData::new(
            "Silver Shards",
            "Silver Bolts",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.09297,
            0.976185,
        )])),
        SkillData::new(
            "Silver Leash",
            "Silver Bolts",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.06198,
            0.65079,
        )])),
        SkillData::new(
            "Trap Beast",
            "Trap Beast",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::bleed_single(),
                    0.05,
                    0.525,
                )])
                .with_dots(vec![DotDamage::new(
                    18.0,
                    DamageFlags::bleed_single(),
                    0.015,
                    0.1575,
                )
                .with_interval(2.0)]),
        )
        .with_bonuses(vec![MINOR_FORCE.clone()]),
        SkillData::new(
            "Barbed Trap",
            "Trap Beast",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::bleed_single(),
                    0.06198,
                    0.65079,
                )])
                .with_dots(vec![DotDamage::new(
                    18.0,
                    DamageFlags::bleed_single(),
                    0.015495,
                    0.162697,
                )
                .with_interval(2.0)]),
        )
        .with_bonuses(vec![MINOR_FORCE.clone()]),
        SkillData::new(
            "Lightweight Beast Trap",
            "Trap Beast",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::bleed_single(),
                    0.05165,
                    0.542325,
                )])
                .with_dots(vec![DotDamage::new(
                    18.0,
                    DamageFlags::bleed_single(),
                    0.015495,
                    0.162697,
                )
                .with_interval(2.0)]),
        )
        .with_bonuses(vec![MINOR_FORCE.clone()]),
        SkillData::new(
            "Expert Hunter",
            "Expert Hunter",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
        ]),
        SkillData::new(
            "Evil Hunter",
            "Expert Hunter",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
        ]),
        SkillData::new(
            "Camouflaged Hunter",
            "Expert Hunter",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MINOR_BERSERK.clone().with_duration(5.0),
        ]),
    ]
});

