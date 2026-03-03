use crate::data::bonuses::unique::{
    MAJOR_BRUTALITY, MAJOR_PROPHECY, MAJOR_SAVAGERY, MAJOR_SORCERY,
};
use crate::domain::{ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static MAGES_GUILD_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        SkillData::new(
            "Meteor",
            "Meteor",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Ultimate,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::flame_aoe(),
                    0.175,
                    1.8375,
                )])
                .with_dots(vec![DotDamage::new(
                    11.0,
                    DamageFlags::flame_aoe(),
                    0.05,
                    0.525,
                )
                .with_interval(1.0)]),
        ),
        SkillData::new(
            "Shooting Star",
            "Meteor",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Ultimate,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::flame_aoe(),
                    0.18078,
                    1.89814,
                )])
                .with_dots(vec![DotDamage::new(
                    11.0,
                    DamageFlags::flame_aoe(),
                    0.05165,
                    0.542325,
                )
                .with_interval(1.0)]),
        ),
        SkillData::new(
            "Ice Comet",
            "Meteor",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Ultimate,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::frost_aoe(),
                    0.198853,
                    2.08795,
                )])
                .with_dots(vec![DotDamage::new(
                    11.0,
                    DamageFlags::frost_aoe(),
                    0.056815,
                    0.596557,
                )
                .with_interval(1.0)]),
        ),
        SkillData::new(
            "Entropy",
            "Entropy",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::magic_single(),
            0.018182,
            0.19091,
        )
        .with_interval(2.0)])),
        SkillData::new(
            "Degeneration",
            "Entropy",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::magic_single(),
            0.018782,
            0.19721,
        )
        .with_interval(2.0)]))
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
        SkillData::new(
            "Structured Entropy",
            "Entropy",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::magic_single(),
            0.018782,
            0.19721,
        )
        .with_interval(2.0)])),
        SkillData::new(
            "Fire Rune",
            "Fire Rune",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_aoe(),
            0.1,
            1.05,
        )])),
        SkillData::new(
            "Volcanic Rune",
            "Fire Rune",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_aoe(),
            0.1033,
            1.08465,
        )])),
        SkillData::new(
            "Scalding Rune",
            "Fire Rune",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::flame_aoe(),
                    0.1033,
                    1.08465,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::flame_single(),
                    0.01127,
                    0.11833,
                )
                .with_interval(2.0)]),
        ),
        SkillData::new(
            "Magelight",
            "Magelight",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
        ]),
        SkillData::new(
            "Inner Light",
            "Magelight",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
        ]),
        SkillData::new(
            "Radiant Magelight",
            "Magelight",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
        ]),
    ]
});

