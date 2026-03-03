use crate::data::bonuses::unique::{
    MINOR_VULNERABILITY,
};
use crate::domain::{ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static UNDAUNTED_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        SkillData::new(
            "Necrotic Orb",
            "Necrotic Orb",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            10.0,
            DamageFlags::magic_aoe(),
            0.013636,
            0.143182,
        )
        .with_interval(1.0)])),
        SkillData::new(
            "Mystic Orb",
            "Necrotic Orb",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            10.0,
            DamageFlags::magic_aoe(),
            0.014086,
            0.147907,
        )
        .with_interval(1.0)])),
        SkillData::new(
            "Inner Fire",
            "Inner Fire",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.045,
            0.4725,
        )])),
        SkillData::new(
            "Inner Rage",
            "Inner Fire",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.046485,
            0.488093,
        )])),
        SkillData::new(
            "Inner Beast",
            "Inner Fire",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.09297,
            0.976185,
        )]))
        .with_bonuses(vec![MINOR_VULNERABILITY.clone().with_duration(15.0)]),
        SkillData::new(
            "Trapping Webs",
            "Trapping Webs",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_aoe(), 0.075, 0.7875),
            HitDamage::new(DamageFlags::poison_aoe(), 0.1, 1.05).with_delay(10.0),
        ])),
        SkillData::new(
            "Tangling Webs",
            "Trapping Webs",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488),
            HitDamage::new(DamageFlags::poison_aoe(), 0.1033, 1.08465).with_delay(10.0),
        ])),
        SkillData::new(
            "Shadow Silk",
            "Trapping Webs",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488),
            HitDamage::new(DamageFlags::poison_aoe(), 0.1033, 1.08465).with_delay(10.0),
        ])),
    ]
});

