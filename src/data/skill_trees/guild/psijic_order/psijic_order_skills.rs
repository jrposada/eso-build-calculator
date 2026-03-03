use crate::data::bonuses::unique::{
    MAJOR_BREACH, MINOR_FORCE,
};
use crate::domain::{ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static PSIJIC_ORDER_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        SkillData::new(
            "Imbue Weapon",
            "Imbue Weapon",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.09,
            0.945,
        )]))
        .with_spammable(),
        SkillData::new(
            "Elemental Weapon",
            "Imbue Weapon",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.09297,
            0.976185,
        )]))
        .with_spammable(),
        SkillData::new(
            "Crushing Weapon",
            "Imbue Weapon",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.09297,
            0.976185,
        )]))
        .with_spammable()
        .with_bonuses(vec![MAJOR_BREACH.clone().with_duration(5.0)]),
        SkillData::new(
            "Accelerate",
            "Accelerate",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            Resource::Magicka,
        )
        .with_bonuses(vec![MINOR_FORCE.clone()]),
        SkillData::new(
            "Channeled Acceleration",
            "Accelerate",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            Resource::Magicka,
        )
        .with_bonuses(vec![MINOR_FORCE.clone().with_duration(60.0)]),
        SkillData::new(
            "Race Against Time",
            "Accelerate",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            Resource::Magicka,
        )
        .with_bonuses(vec![MINOR_FORCE.clone()]),
    ]
});

