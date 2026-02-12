use crate::data::bonuses::unique::EMPOWER;
use crate::domain::{
    BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue, ClassName, PassiveData,
    SkillLineName,
};
use once_cell::sync::Lazy;

pub static GUILD_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        // === FIGHTERS GUILD ===
        PassiveData::new(
            "Intimidating Presence",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            vec![], // Cost reduction, not tracked
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
            vec![], // Ultimate gen on kill, not tracked
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
        // === MAGES GUILD ===
        PassiveData::new(
            "Mage Adept",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            vec![], // Cost reduction, not tracked
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
        // === UNDAUNTED ===
        PassiveData::new(
            "Undaunted Command",
            ClassName::Guild,
            SkillLineName::Undaunted,
            vec![], // Synergy restore, not tracked
        ),
        PassiveData::new(
            "Undaunted Mettle",
            ClassName::Guild,
            SkillLineName::Undaunted,
            vec![], // Armor type dependent, not tracked
        ),
        // === PSIJIC ORDER ===
        PassiveData::new(
            "Clairvoyance",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            vec![], // Cost reduction, not tracked
        ),
        PassiveData::new(
            "Spell Orb",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            vec![], // Flat damage proc, not tracked
        ),
        PassiveData::new(
            "Concentrated Barrier",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            vec![], // Defensive, not tracked
        ),
        PassiveData::new(
            "Deliberation",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            vec![], // Healing, not tracked
        ),
    ]
});
