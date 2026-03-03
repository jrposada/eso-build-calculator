use crate::domain::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue, ClassName, PassiveData,
    SkillLineName,
};
use once_cell::sync::Lazy;

pub static FIGHTERS_GUILD_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        PassiveData::new(
            "Intimidating Presence",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            vec![],
        ),
        PassiveData::new(
            "Slayer",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            vec![BonusData::new(
                "Slayer",
                BonusSource::Passive,
                BonusTrigger::AbilitySlottedCount,
                BonusValue::new(
                    "Slayer",
                    BonusTarget::WeaponAndSpellDamageMultiplier,
                    0.03,
                ),
            )
            .with_skill_line_filter(SkillLineName::FightersGuild)],
        ),
        PassiveData::new(
            "Banish the Wicked",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            vec![],
        ),
        PassiveData::new(
            "Skilled Tracker",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            vec![BonusData::new(
                "Skilled Tracker",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("Skilled Tracker", BonusTarget::Damage, 0.10),
            )
            .with_skill_line_filter(SkillLineName::FightersGuild)],
        ),
    ]
});
