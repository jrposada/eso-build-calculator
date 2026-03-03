use crate::data::bonuses::unique::EMPOWER;
use crate::domain::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue, ClassName, PassiveData,
    SkillLineName,
};
use once_cell::sync::Lazy;

pub static MAGES_GUILD_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        PassiveData::new(
            "Mage Adept",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            vec![],
        ),
        PassiveData::new(
            "Everlasting Magic",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            vec![BonusData::new(
                "Everlasting Magic",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new(
                    "Everlasting Magic",
                    BonusTarget::DurationSkillLineFlat,
                    2.0,
                ),
            )
            .with_skill_line_filter(SkillLineName::MagesGuild)],
        ),
        PassiveData::new(
            "Magicka Controller",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            vec![BonusData::new(
                "Magicka Controller",
                BonusSource::Passive,
                BonusTrigger::AbilitySlottedCount,
                BonusValue::new("Magicka Controller", BonusTarget::MaxMagicka, 0.02),
            )
            .with_skill_line_filter(SkillLineName::MagesGuild)],
        ),
        PassiveData::new(
            "Might of the Guild",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            vec![EMPOWER
                .clone()
                .with_trigger(BonusTrigger::SkillLineSkillCast)
                .with_skill_line_filter(SkillLineName::MagesGuild)],
        ),
    ]
});
