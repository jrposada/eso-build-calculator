use crate::data::bonuses::{MAJOR_BRUTALITY, MAJOR_SORCERY, MINOR_BREACH};
use crate::domain::{BonusData, DotDamage, HitDamage, SkillDamage, SkillData};
use crate::domain::{
    BonusTarget, BonusTrigger, ClassName, DamageType, Resource, SkillLineName, TargetType,
};
use once_cell::sync::Lazy;

pub static ARCANIST_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        // === HERALD OF THE TOME ===
        // Ultimate - The Unblinking Eye line
        SkillData::new(
            "The Unblinking Eye",
            "The Unblinking Eye",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_dots(vec![DotDamage::new(1115.0, 6.0).with_interval(0.5)]),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        SkillData::new(
            "The Languid Eye",
            "The Unblinking Eye",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_dots(vec![DotDamage::new(1115.0, 6.0)
                .with_interval(0.5)
                .with_increase_per_tick(0.07)]),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        SkillData::new(
            "The Tide King's Gaze",
            "The Unblinking Eye",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_dots(vec![DotDamage::new(1151.0, 8.0).with_interval(0.5)]),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        // Runeblades line
        SkillData::new(
            "Runeblades",
            "Runeblades",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_hits(vec![HitDamage::new(695.0 * 3.0)]),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Escalating Runeblades",
            "Runeblades",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_hits(vec![HitDamage::new(696.0 + 766.0 + 917.0)]),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Writhing Runeblades",
            "Runeblades",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_hits(vec![HitDamage::new(718.0 * 3.0)]),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        // Fatecarver line
        SkillData::new(
            "Fatecarver",
            "Fatecarver",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_dots(vec![DotDamage::new(879.0, 4.0).with_interval(0.3)]),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_channel_time(4.0),
        SkillData::new(
            "Exhausting Fatecarver",
            "Fatecarver",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_dots(vec![DotDamage::new(879.0, 4.0).with_interval(0.3)]),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_channel_time(4.0),
        SkillData::new(
            "Pragmatic Fatecarver",
            "Fatecarver",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_dots(vec![DotDamage::new(879.0, 4.0).with_interval(0.3)]),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_channel_time(4.0),
        // Abyssal Impact line
        // Immobilizes for 3s, applies Abyssal Ink for 20s (5% increased damage to marked enemies)
        SkillData::new(
            "Abyssal Impact",
            "Abyssal Impact",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_hits(vec![HitDamage::new(1939.0)]),
            DamageType::Physical,
            TargetType::Aoe,
            Resource::Stamina,
        )
        .with_bonuses(vec![BonusData::new(
            "Abyssal Ink",
            BonusTrigger::Cast,
            BonusTarget::EnemyDamageTaken,
            0.05,
        )
        .with_duration(20.0)]),
        // Heals for 1000 Health on hit
        SkillData::new(
            "Cephaliarch's Flail",
            "Abyssal Impact",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_hits(vec![HitDamage::new(1939.0)]),
            DamageType::Physical,
            TargetType::Aoe,
            Resource::Stamina,
        )
        .with_bonuses(vec![BonusData::new(
            "Abyssal Ink",
            BonusTrigger::Cast,
            BonusTarget::EnemyDamageTaken,
            0.05,
        )
        .with_duration(20.0)]),
        // Converts to Frost/Magicka, damage to Ink targets increases 2% per Crux
        SkillData::new(
            "Tentacular Dread",
            "Abyssal Impact",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_hits(vec![HitDamage::new(2002.0)]),
            DamageType::Frost,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_bonuses(vec![BonusData::new(
            "Abyssal Ink",
            BonusTrigger::Cast,
            BonusTarget::EnemyDamageTaken,
            0.05,
        )
        .with_duration(20.0)]),
        // Tome-Bearer's Inspiration line
        // While slotted: Major Brutality (+20% Weapon Damage) and Major Sorcery (+20% Spell Damage)
        SkillData::new(
            "Tome-Bearer's Inspiration",
            "Tome-Bearer's Inspiration",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_dots(vec![DotDamage::new(1161.0, 20.0).with_interval(5.0)]),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SORCERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]),
        // While slotted: Major Brutality (+20% Weapon Damage) and Major Sorcery (+20% Spell Damage)
        // Pulses every 3 seconds instead of 5
        SkillData::new(
            "Inspired Scholarship",
            "Tome-Bearer's Inspiration",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_dots(vec![DotDamage::new(935.0, 20.0).with_interval(3.0)]),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SORCERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]),
        // While slotted: Major Brutality (+20% Weapon Damage) and Major Sorcery (+20% Spell Damage)
        // Restores 600 Magicka and Stamina on hit
        SkillData::new(
            "Recuperative Treatise",
            "Tome-Bearer's Inspiration",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_dots(vec![DotDamage::new(1161.0, 20.0).with_interval(5.0)]),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SORCERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]),
        // The Imperfect Ring line
        SkillData::new(
            "The Imperfect Ring",
            "The Imperfect Ring",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_dots(vec![DotDamage::new(4631.0, 20.0)]),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Fulminating Rune",
            "The Imperfect Ring",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1438.0).with_delay(6.0)])
                .with_dots(vec![DotDamage::new(4642.0, 20.0)]),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Rune of Displacement",
            "The Imperfect Ring",
            ClassName::Arcanist,
            SkillLineName::HeraldOfTheTome,
            SkillDamage::new().with_dots(vec![DotDamage::new(4780.0, 18.0)]),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        // === SOLDIER OF APOCRYPHA ===
        // Ultimate - Gibbering Shield line (mostly no damage)
        SkillData::new(
            "Gibbering Shield",
            "Gibbering Shield",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Gibbering Shelter",
            "Gibbering Shield",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Sanctum of the Abyssal Sea",
            "Gibbering Shield",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        // Runic Jolt line
        SkillData::new(
            "Runic Jolt",
            "Runic Jolt",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new().with_hits(vec![HitDamage::new(1161.0)]),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Runic Embrace",
            "Runic Jolt",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new().with_hits(vec![HitDamage::new(1161.0)]),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        // Converts to Physical/Stamina, steals 2200 Armor from enemy
        SkillData::new(
            "Runic Sunder",
            "Runic Jolt",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new().with_hits(vec![HitDamage::new(1161.0)]),
            DamageType::Physical,
            TargetType::Single,
            Resource::Stamina,
        )
        .with_bonuses(vec![BonusData::new(
            "Runic Sunder Armor Steal",
            BonusTrigger::Cast,
            BonusTarget::EnemyResistanceReduction,
            2200.0,
        )
        .with_duration(15.0)]),
        // Runespite Ward line (no damage modeled)
        SkillData::new(
            "Runespite Ward",
            "Runespite Ward",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Impervious Runeward",
            "Runespite Ward",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Spiteward of the Lucid Mind",
            "Runespite Ward",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        // Fatewoven Armor line (no damage)
        // Major Resolve 20s, applies Minor Breach 6s when taking damage
        SkillData::new(
            "Fatewoven Armor",
            "Fatewoven Armor",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        )
        .with_bonuses(vec![MINOR_BREACH.clone().with_duration(6.0)]),
        // Major Resolve 30s, Minor Breach 6s when damaged, generates Crux when hit (5s cooldown)
        SkillData::new(
            "Cruxweaver Armor",
            "Fatewoven Armor",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        )
        .with_bonuses(vec![MINOR_BREACH.clone().with_duration(6.0)]),
        // 5% Block Mitigation + Major Resolve 20s, Minor Breach 6s when damaged
        SkillData::new(
            "Unbreakable Fate",
            "Fatewoven Armor",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        )
        .with_bonuses(vec![MINOR_BREACH.clone().with_duration(6.0)]),
        // Runic Defense line (no damage)
        SkillData::new(
            "Runic Defense",
            "Runic Defense",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Runeguard of Freedom",
            "Runic Defense",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Runeguard of Still Waters",
            "Runic Defense",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        // Rune of Eldritch Horror line (no damage)
        SkillData::new(
            "Rune of Eldritch Horror",
            "Rune of Eldritch Horror",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Rune of Uncanny Adoration",
            "Rune of Eldritch Horror",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Rune of the Colorless Pool",
            "Rune of Eldritch Horror",
            ClassName::Arcanist,
            SkillLineName::SoldierOfApocrypha,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        // === CURATIVE RUNEFORMS === (mostly heals, no damage)
        // Ultimate - Vitalizing Glyphic line (no damage)
        SkillData::new(
            "Vitalizing Glyphic",
            "Vitalizing Glyphic",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Glyphic of the Tides",
            "Vitalizing Glyphic",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Resonating Glyphic",
            "Vitalizing Glyphic",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        // Runemend line (no damage)
        SkillData::new(
            "Runemend",
            "Runemend",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Audacious Runemend",
            "Runemend",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Evolving Runemend",
            "Runemend",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        // Remedy Cascade line (no damage)
        SkillData::new(
            "Remedy Cascade",
            "Remedy Cascade",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_channel_time(4.5),
        SkillData::new(
            "Cascading Fortune",
            "Remedy Cascade",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_channel_time(4.5),
        SkillData::new(
            "Curative Surge",
            "Remedy Cascade",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_channel_time(4.5),
        // Chakram Shields line (no damage)
        SkillData::new(
            "Chakram Shields",
            "Chakram Shields",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Chakram of Destiny",
            "Chakram Shields",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Tidal Chakram",
            "Chakram Shields",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        // Arcanist's Domain line (no damage)
        SkillData::new(
            "Arcanist's Domain",
            "Arcanist's Domain",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Reconstructive Domain",
            "Arcanist's Domain",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Zenas' Empowering Disc",
            "Arcanist's Domain",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        // Apocryphal Gate line (no damage)
        SkillData::new(
            "Apocryphal Gate",
            "Apocryphal Gate",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Fleet-Footed Gate",
            "Apocryphal Gate",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Passage Between Worlds",
            "Apocryphal Gate",
            ClassName::Arcanist,
            SkillLineName::CurativeRuneforms,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
    ]
});
