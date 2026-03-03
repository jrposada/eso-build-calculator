use crate::data::bonuses::{MAJOR_BRUTALITY, MAJOR_SORCERY, MINOR_BREACH};
use crate::domain::{
    BonusData, BonusSource, BonusValue, DamageFlags, DotDamage, HitDamage, SkillDamage, SkillData,
};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, Resource, SkillLineName};
use once_cell::sync::Lazy;

pub static ARCANIST_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        SkillData::new(
            "The Unblinking Eye",
            "The Unblinking Eye",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Ultimate,
        )
            .with_skill_id(40189791)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(6.0, DamageFlags::magic_aoe(), 0.048, 0.504).with_interval(0.5),
        ])),
        SkillData::new(
            "The Languid Eye",
            "The Unblinking Eye",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Ultimate,
        )
            .with_skill_id(40189867)
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            6.0,
            DamageFlags::magic_aoe(),
            0.049584,
            0.520632,
        )
        .with_interval(0.5)
        .with_increase_per_tick(0.07)])),
        SkillData::new(
            "The Tide King's Gaze",
            "The Unblinking Eye",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Ultimate,
        )
            .with_skill_id(40189837)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(8.0, DamageFlags::magic_single(), 0.049584, 0.520632).with_interval(0.5),
        ])),
        SkillData::new(
            "Runeblades",
            "Runeblades",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40185794)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.03,
            0.315,
        )]))
        .with_spammable(),
        SkillData::new(
            "Escalating Runeblades",
            "Runeblades",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40182977)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_aoe(),
            0.03099,
            0.325395,
        )]))
        .with_spammable(),
        SkillData::new(
            "Writhing Runeblades",
            "Runeblades",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40185803)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.03099,
            0.325395,
        )]))
        .with_spammable(),
        SkillData::new(
            "Fatecarver",
            "Fatecarver",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40185805)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(4.0, DamageFlags::magic_aoe(), 0.03785, 0.397425).with_interval(0.3),
        ]))
        .with_channel_time(4.0),
        SkillData::new(
            "Exhausting Fatecarver",
            "Fatecarver",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40183122)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(4.0, DamageFlags::magic_aoe(), 0.039099, 0.41054).with_interval(0.3),
        ]))
        .with_channel_time(4.0),
        SkillData::new(
            "Pragmatic Fatecarver",
            "Fatecarver",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40186366)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(4.0, DamageFlags::magic_aoe(), 0.039099, 0.41054).with_interval(0.3),
        ]))
        .with_channel_time(4.0),
        SkillData::new(
            "Abyssal Impact",
            "Abyssal Impact",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Stamina,
        )
            .with_skill_id(40185817)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.08625,
            0.905625,
        )]))
        .with_spammable()
        .with_bonuses(vec![BonusData::new(
            "Abyssal Ink",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Abyssal Ink", BonusTarget::EnemyDamageTaken, 0.05),
        )
        .with_duration(20.0)]),
        SkillData::new(
            "Cephaliarch's Flail",
            "Abyssal Impact",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Stamina,
        )
            .with_skill_id(40183006)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.08625,
            0.905625,
        )]))
        .with_spammable()
        .with_bonuses(vec![BonusData::new(
            "Abyssal Ink",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Abyssal Ink", BonusTarget::EnemyDamageTaken, 0.05),
        )
        .with_duration(20.0)]),
        SkillData::new(
            "Tentacular Dread",
            "Abyssal Impact",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40185823)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::frost_aoe(),
            0.08625,
            0.905625,
        )]))
        .with_spammable()
        .with_bonuses(vec![BonusData::new(
            "Abyssal Ink",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Abyssal Ink", BonusTarget::EnemyDamageTaken, 0.05),
        )
        .with_duration(20.0)]),
        SkillData::new(
            "Tome-Bearer's Inspiration",
            "Tome-Bearer's Inspiration",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40186452)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(20.0, DamageFlags::magic_single(), 0.05, 0.525).with_interval(5.0),
        ]))
        .with_bonuses(vec![
            MAJOR_BRUTALITY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SORCERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]),
        SkillData::new(
            "Inspired Scholarship",
            "Tome-Bearer's Inspiration",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40185842)
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::magic_single(),
            0.040287,
            0.423013,
        )
        .with_interval(3.0)]))
        .with_bonuses(vec![
            MAJOR_BRUTALITY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SORCERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]),
        SkillData::new(
            "Recuperative Treatise",
            "Tome-Bearer's Inspiration",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40183047)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(20.0, DamageFlags::magic_single(), 0.05165, 0.54235).with_interval(5.0),
        ]))
        .with_bonuses(vec![
            MAJOR_BRUTALITY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SORCERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]),
        SkillData::new(
            "The Imperfect Ring",
            "The Imperfect Ring",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40185836)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(20.0, DamageFlags::magic_single(), 0.018182, 0.19091).with_interval(2.0),
        ])),
        SkillData::new(
            "Fulminating Rune",
            "The Imperfect Ring",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40182988)
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::frost_aoe(),
                    0.06198,
                    0.65079,
                )
                .with_delay(6.0)])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::magic_single(),
                    0.018782,
                    0.19721,
                )
                .with_interval(2.0)]),
        ),
        SkillData::new(
            "Rune of Displacement",
            "The Imperfect Ring",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            Resource::Magicka,
        )
            .with_skill_id(40185839)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(18.0, DamageFlags::magic_single(), 0.02066, 0.21693).with_interval(2.0),
        ])),
        SkillData::new(
            "Gibbering Shield",
            "Gibbering Shield",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Ultimate,
        )
            .with_skill_id(40183676),
        SkillData::new(
            "Gibbering Shelter",
            "Gibbering Shield",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Ultimate,
        )
            .with_skill_id(40192380),
        SkillData::new(
            "Sanctum of the Abyssal Sea",
            "Gibbering Shield",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Ultimate,
        )
            .with_skill_id(40192372),
        SkillData::new(
            "Runic Jolt",
            "Runic Jolt",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40183165)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.05,
            0.525,
        )]))
        .with_spammable(),
        SkillData::new(
            "Runic Embrace",
            "Runic Jolt",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40186531)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.05165,
            0.542325,
        )]))
        .with_spammable(),
        SkillData::new(
            "Runic Sunder",
            "Runic Jolt",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Stamina,
        )
            .with_skill_id(40183430)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.05165,
            0.542325,
        )]))
        .with_spammable()
        .with_bonuses(vec![BonusData::new(
            "Runic Sunder Armor Steal",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new(
                "Runic Sunder Armor Steal",
                BonusTarget::EnemyResistanceReduction,
                2200.0,
            ),
        )
        .with_duration(15.0)]),
        SkillData::new(
            "Runespite Ward",
            "Runespite Ward",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40185894),
        SkillData::new(
            "Impervious Runeward",
            "Runespite Ward",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40183241),
        SkillData::new(
            "Spiteward of the Lucid Mind",
            "Runespite Ward",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40185901),
        SkillData::new(
            "Fatewoven Armor",
            "Fatewoven Armor",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40183648)
        .with_bonuses(vec![MINOR_BREACH.clone().with_duration(6.0)]),
        SkillData::new(
            "Cruxweaver Armor",
            "Fatewoven Armor",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40185908)
        .with_bonuses(vec![MINOR_BREACH.clone().with_duration(6.0)]),
        SkillData::new(
            "Unbreakable Fate",
            "Fatewoven Armor",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40186477)
        .with_bonuses(vec![MINOR_BREACH.clone().with_duration(6.0)]),
        SkillData::new(
            "Runic Defense",
            "Runic Defense",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40185912),
        SkillData::new(
            "Runeguard of Freedom",
            "Runic Defense",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40186489),
        SkillData::new(
            "Runeguard of Still Waters",
            "Runic Defense",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40183401),
        SkillData::new(
            "Rune of Eldritch Horror",
            "Rune of Eldritch Horror",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40185918),
        SkillData::new(
            "Rune of Uncanny Adoration",
            "Rune of Eldritch Horror",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40185921),
        SkillData::new(
            "Rune of the Colorless Pool",
            "Rune of Eldritch Horror",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            Resource::Magicka,
        )
            .with_skill_id(40183267),
        SkillData::new(
            "Vitalizing Glyphic",
            "Vitalizing Glyphic",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Ultimate,
        )
            .with_skill_id(40183709),
        SkillData::new(
            "Glyphic of the Tides",
            "Vitalizing Glyphic",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Ultimate,
        )
            .with_skill_id(40193794),
        SkillData::new(
            "Resonating Glyphic",
            "Vitalizing Glyphic",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Ultimate,
        )
            .with_skill_id(40193558),
        SkillData::new(
            "Runemend",
            "Runemend",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40183261),
        SkillData::new(
            "Audacious Runemend",
            "Runemend",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40186191),
        SkillData::new(
            "Evolving Runemend",
            "Runemend",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40186189),
        SkillData::new(
            "Remedy Cascade",
            "Remedy Cascade",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40183537)
        .with_channel_time(4.5),
        SkillData::new(
            "Cascading Fortune",
            "Remedy Cascade",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40186193)
        .with_channel_time(4.5),
        SkillData::new(
            "Curative Surge",
            "Remedy Cascade",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40186200)
        .with_channel_time(4.5),
        SkillData::new(
            "Chakram Shields",
            "Chakram Shields",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40183447),
        SkillData::new(
            "Chakram of Destiny",
            "Chakram Shields",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40186207),
        SkillData::new(
            "Tidal Chakram",
            "Chakram Shields",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40186209),
        SkillData::new(
            "Arcanist's Domain",
            "Arcanist's Domain",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40183555),
        SkillData::new(
            "Reconstructive Domain",
            "Arcanist's Domain",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40186234),
        SkillData::new(
            "Zenas' Empowering Disc",
            "Arcanist's Domain",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40186229),
        SkillData::new(
            "Apocryphal Gate",
            "Apocryphal Gate",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        ),
        SkillData::new(
            "Fleet-Footed Gate",
            "Apocryphal Gate",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        )
            .with_skill_id(40186211),
        SkillData::new(
            "Passage Between Worlds",
            "Apocryphal Gate",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            Resource::Magicka,
        ),
    ]
});

