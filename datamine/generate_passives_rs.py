"""Generate Rust passive data files from parsed skill_coefficients.json.

Reads datamine/parsed/skill_coefficients.json and generates:
  - src/data/skill_trees/character_class/{class}/{class}_passives.rs  (7 files)
  - src/data/skill_trees/weapon/{weapon}/{weapon}_passives.rs          (4 files)
  - src/data/skill_trees/guild/{guild}/{guild}_passives.rs              (4 files)

Each file exports `pub static {NAME}_PASSIVES: Lazy<Vec<PassiveData>>`.
Manual overrides (bonuses) are stored in this script.
"""

from __future__ import annotations

import json
import re
from pathlib import Path

# ---------------------------------------------------------------------------
# Filtering
# ---------------------------------------------------------------------------

RELEVANT_SKILL_LINES = {
    # Character classes
    "Aedric Spear", "Dawn's Wrath", "Restoring Light",        # Templar
    "Ardent Flame", "Draconic Power", "Earthen Heart",         # Dragonknight
    "Assassination", "Shadow", "Siphoning",                     # Nightblade
    "Dark Magic", "Daedric Summoning", "Storm Calling",        # Sorcerer
    "Grave Lord", "Bone Tyrant", "Living Death",               # Necromancer
    "Animal Companions", "Green Balance", "Winter's Embrace",  # Warden
    "Herald of the Tome", "Soldier of Apocrypha", "Curative Runeforms",  # Arcanist
    # Weapons
    "Bow", "Destruction Staff", "Dual Wield", "Two Handed",
    # Guilds
    "Fighters Guild", "Mages Guild", "Psijic Order", "Undaunted",
}

# ---------------------------------------------------------------------------
# Mapping tables
# ---------------------------------------------------------------------------

# skill_line (JSON) -> (SkillLineName variant, class group, file directory, file stem)
SKILL_LINE_MAP: dict[str, tuple[str, str, str, str]] = {
    # Templar
    "Aedric Spear":       ("AedricSpear",       "Templar",     "character_class/templar",        "templar"),
    "Dawn's Wrath":       ("DawnsWrath",         "Templar",     "character_class/templar",        "templar"),
    "Restoring Light":    ("RestoringLight",     "Templar",     "character_class/templar",        "templar"),
    # Dragonknight
    "Ardent Flame":       ("ArdentFlame",        "Dragonknight","character_class/dragonknight",   "dragonknight"),
    "Draconic Power":     ("DraconicPower",      "Dragonknight","character_class/dragonknight",   "dragonknight"),
    "Earthen Heart":      ("EarthenHeart",       "Dragonknight","character_class/dragonknight",   "dragonknight"),
    # Nightblade
    "Assassination":      ("Assassination",      "Nightblade",  "character_class/nightblade",     "nightblade"),
    "Shadow":             ("Shadow",             "Nightblade",  "character_class/nightblade",     "nightblade"),
    "Siphoning":          ("Siphoning",          "Nightblade",  "character_class/nightblade",     "nightblade"),
    # Sorcerer
    "Dark Magic":         ("DarkMagic",          "Sorcerer",    "character_class/sorcerer",       "sorcerer"),
    "Daedric Summoning":  ("DaedricSummoning",   "Sorcerer",    "character_class/sorcerer",       "sorcerer"),
    "Storm Calling":      ("StormCalling",       "Sorcerer",    "character_class/sorcerer",       "sorcerer"),
    # Necromancer
    "Grave Lord":         ("GraveLord",          "Necromancer", "character_class/necromancer",    "necromancer"),
    "Bone Tyrant":        ("BoneTyrant",         "Necromancer", "character_class/necromancer",    "necromancer"),
    "Living Death":       ("LivingDeath",        "Necromancer", "character_class/necromancer",    "necromancer"),
    # Warden
    "Animal Companions":  ("AnimalCompanions",   "Warden",      "character_class/warden",         "warden"),
    "Green Balance":      ("GreenBalance",       "Warden",      "character_class/warden",         "warden"),
    "Winter's Embrace":   ("WintersEmbrace",     "Warden",      "character_class/warden",         "warden"),
    # Arcanist
    "Herald of the Tome": ("HeraldOfTheTome",    "Arcanist",    "character_class/arcanist",       "arcanist"),
    "Soldier of Apocrypha":("SoldierOfApocrypha","Arcanist",    "character_class/arcanist",       "arcanist"),
    "Curative Runeforms": ("CurativeRuneforms",  "Arcanist",    "character_class/arcanist",       "arcanist"),
    # Weapons
    "Bow":                ("Bow",                "Weapon",      "weapon/bow",                     "bow"),
    "Destruction Staff":  ("DestructionStaff",   "Weapon",      "weapon/destruction_staff",       "destruction_staff"),
    "Dual Wield":         ("DualWield",          "Weapon",      "weapon/dual_wield",              "dual_wield"),
    "Two Handed":         ("TwoHanded",          "Weapon",      "weapon/two_handed",              "two_handed"),
    # Guilds
    "Fighters Guild":     ("FightersGuild",      "Guild",       "guild/fighters_guild",           "fighters_guild"),
    "Mages Guild":        ("MagesGuild",         "Guild",       "guild/mages_guild",              "mages_guild"),
    "Psijic Order":       ("PsijicOrder",        "Guild",       "guild/psijic_order",             "psijic_order"),
    "Undaunted":          ("Undaunted",          "Guild",       "guild/undaunted",                "undaunted"),
}

# ---------------------------------------------------------------------------
# Name corrections (JSON name -> code name)
# ---------------------------------------------------------------------------

NAME_CORRECTIONS: dict[str, str] = {
    "Bond with Nature": "Bond With Nature",
}

# ---------------------------------------------------------------------------
# Manual passives not in JSON
# ---------------------------------------------------------------------------

MANUAL_PASSIVES: dict[str, dict[str, str]] = {
    "Amplitude":         {"class": "Sorcerer", "skill_line": "Storm Calling"},
    "Long Shots":        {"class": "Weapon",   "skill_line": "Bow"},
    "Everlasting Magic": {"class": "Guild",    "skill_line": "Mages Guild"},
}

# ---------------------------------------------------------------------------
# Passive overrides - keyed by passive name
# Contains raw Rust snippets for bonus data not derivable from JSON.
# ---------------------------------------------------------------------------

PASSIVE_OVERRIDES: dict[str, dict] = {
    # ---- Templar ----
    "Piercing Spear": {
        "bonuses": """\
vec![BonusData::new(
                "Piercing Spear",
                BonusSource::Passive,
                BonusTrigger::AbilitySlotted,
                BonusValue::new("Piercing Spear", BonusTarget::CriticalDamage, 0.12),
            )]""",
    },
    "Spear Wall": {
        "bonuses": """\
vec![
                MINOR_BERSERK
                    .clone()
                    .with_trigger(BonusTrigger::SkillLineSkillCast)
                    .with_duration(6.0),
            ]""",
    },
    "Burning Light": {
        "bonuses": "vec![]",
        "comment": "TODO: To complex, stacks + consume for trigger damage",
    },
    "Balanced Warrior": {
        "bonuses": """\
vec![
                BonusData::new(
                    "Balanced Warrior",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusValue::new(
                        "Balanced Warrior",
                        BonusTarget::WeaponAndSpellDamageMultiplier,
                        0.06,
                    ),
                ),
            ]""",
    },
    "Enduring Rays": {
        "bonuses": "vec![]",
        "comment": "TODO: duration increase to specific skills",
    },
    "Prism": {
        "bonuses": "vec![]",
        "comment": "TODO: generates 3 ultimate",
    },
    "Illuminate": {
        "bonuses": """\
vec![MINOR_SORCERY
                .clone()
                .with_trigger(BonusTrigger::SkillLineSkillCast)]""",
    },

    # ---- Dragonknight ----
    "Combustion": {
        "bonuses": """\
vec![
                BonusData::new(
                    "Combustion (Damage)",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Combustion (Damage)",
                        BonusTarget::BurningAndPoisonDamage,
                        0.33,
                    ),
                ),
                BonusData::new(
                    "Combustion (Restore)",
                    BonusSource::Passive,
                    BonusTrigger::BurningOrPoisonDamageDealt,
                    BonusValue::new(
                        "Combustion (Restore)",
                        BonusTarget::RestoreMagickaOrStamina,
                        423.0,
                    ),
                )
                .with_cooldown(3.0),
            ]""",
    },
    "Warmth": {
        "bonuses": """\
vec![BonusData::new(
                "Warmth",
                BonusSource::Passive,
                BonusTrigger::SkillLineSkillCast,
                BonusValue::new("Warmth", BonusTarget::AoeDamage, 0.06),
            )
            .with_duration(3.0)]""",
    },
    "Searing Heat": {
        "bonuses": "vec![]",
        "comment": "TODO: Passive effect to specific skills.",
    },
    "World in Ruin": {
        "bonuses": """\
vec![BonusData::new(
                "World in Ruin",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("World in Ruin", BonusTarget::BurningAndPoisonDamage, 0.05),
            )]""",
    },
    "Eternal Mountain": {
        "bonuses": """\
vec![BonusData::new(
                "Eternal Mountain",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new(
                    "Eternal Mountain",
                    BonusTarget::DurationSkillLineMultiplier,
                    0.2,
                ),
            )]""",
    },
    "Battle Roar": {
        "bonuses": "vec![]",
        "comment": "TODO: To complex, restore resources on ult cast",
    },
    "Mountain's Blessing": {
        "bonuses": """\
vec![
                MINOR_BRUTALITY
                    .clone()
                    .with_trigger(BonusTrigger::SkillLineSkillCast)
                    .with_cooldown(6.0),
                // TODO: Generates 3 ultimate
            ]""",
    },
    "Helping Hands": {
        "bonuses": "vec![]",
        "comment": "TODO: To complex, restore stamina on skill use with multiple conditions",
    },

    # ---- Nightblade ----
    "Master Assassin": {
        "bonuses": """\
vec![BonusData::new(
                "Master Assassin",
                BonusSource::Passive,
                BonusTrigger::Flanking,
                BonusValue::new("Master Assassin", BonusTarget::CriticalRating, 1448.0),
            )]""",
    },
    "Executioner": {
        "bonuses": "vec![]",
        "comment": "TODO: Restore 1000 Magicka and Stamina when enemy dies within 2s of being damaged",
    },
    "Pressure Points": {
        "bonuses": """\
vec![BonusData::new(
                "Pressure Points",
                BonusSource::Passive,
                BonusTrigger::AbilitySlottedCount,
                BonusValue::new("Pressure Points", BonusTarget::CriticalRating, 548.0),
            )
            .with_skill_line_filter(SkillLineName::Assassination)]""",
    },
    "Hemorrhage": {
        "bonuses": """\
vec![
                BonusData::new(
                    "Hemorrhage",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusValue::new("Hemorrhage", BonusTarget::CriticalDamage, 0.1),
                ),
                MINOR_SAVAGERY // TODO: with condition skill line ability slotted
                    .clone()
                    .with_trigger(BonusTrigger::CriticalDamageDealt),
            ]""",
    },
    "Dark Veil": {
        "bonuses": """\
vec![BonusData::new(
                "Dark Veil",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("Dark Veil", BonusTarget::DurationSkillLineFlat, 2.0),
            )]""",
    },
    "Catalyst": {
        "bonuses": "vec![]",
        "comment": "TODO: Ultimate gain on potion",
    },
    "Magicka Flood": {
        "bonuses": """\
vec![
                BonusData::new(
                    "Magicka Flood (Stamina)",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusValue::new("Magicka Flood (Stamina)", BonusTarget::MaxStamina, 0.06),
                ),
                BonusData::new(
                    "Magicka Flood (Magicka)",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusValue::new("Magicka Flood (Magicka)", BonusTarget::MaxMagicka, 0.06),
                ),
            ]""",
    },
    "Transfer": {
        "bonuses": "vec![]",
        "comment": "TODO: Generate 2 ultimate",
    },

    # ---- Sorcerer ----
    "Blood Magic": {
        "bonuses": "vec![]",
        "comment": "TODO: To complex, increase max resources with conditions",
    },
    "Exploitation": {
        "bonuses": """\
vec![MINOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::SkillLineSkillCast)]""",
    },
    "Rebate": {
        "bonuses": "vec![]",
        "comment": "TODO: to complex, restore resources on non ultimate end",
    },
    "Expert Summoner": {
        "bonuses": """\
vec![
                BonusData::new(
                    "Expert Summoner 1",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new("Expert Summoner (Magicka)", BonusTarget::MaxMagicka, 0.05),
                ),
                BonusData::new(
                    "Expert Summoner 2",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new("Expert Summoner (Stamina)", BonusTarget::MaxStamina, 0.05),
                ),
            ]""",
    },
    "Energized": {
        "bonuses": """\
vec![
                BonusData::new(
                    "Energized (Physical)",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new("Energized (Physical)", BonusTarget::PhysicalDamage, 0.05),
                ),
                BonusData::new(
                    "Energized (Shock)",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new("Energized (Shock)", BonusTarget::ShockDamage, 0.05),
                ),
            ]""",
    },
    "Amplitude": {
        "bonuses": "vec![]",
        "comment": "TODO: To complex, increase damage base on current health",
    },
    "Expert Mage": {
        "bonuses": """\
vec![BonusData::new(
                "Expert Mage",
                BonusSource::Passive,
                BonusTrigger::AbilitySlottedCount,
                BonusValue::new("Expert Mage", BonusTarget::WeaponAndSpellDamageFlat, 108.0),
            )
            .with_skill_line_filter(SkillLineName::StormCalling)]""",
    },

    # ---- Necromancer ----
    "Death Knell": {
        "bonuses": """\
vec![BonusData::new(
                "Death Knell",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("Death Knell", BonusTarget::CriticalRating, 4382.0),
            )
            .with_execute_threshold(0.33)]""",
    },
    "Dismember": {
        "bonuses": """\
vec![BonusData::new(
                "Dismember",
                BonusSource::Passive,
                BonusTrigger::SkillLineSlotted,
                BonusValue::new(
                    "Dismember",
                    BonusTarget::PhysicalAndSpellPenetration,
                    3271.0,
                ),
            )]""",
    },
    "Rapid Rot": {
        "bonuses": """\
vec![BonusData::new(
                "Rapid Rot",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("Rapid Rot", BonusTarget::DotDamage, 0.10),
            )]""",
    },

    # ---- Warden ----
    "Savage Beast": {
        "bonuses": "vec![]",
        "comment": "TODO: 4 ultimate generation",
    },
    "Advanced Species": {
        "bonuses": """\
vec![BonusData::new(
                "Advanced Species",
                BonusSource::Passive,
                BonusTrigger::AbilitySlottedCount,
                BonusValue::new("Advanced Species", BonusTarget::CriticalDamage, 0.05),
            )
            .with_skill_line_filter(SkillLineName::AnimalCompanions)]""",
    },
    "Glacial Presence": {
        "bonuses": """\
vec![
                BonusData::new(
                    "Glacial Presence 1",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Glacial Presence (Chance)",
                        BonusTarget::ChilledStatusEffectChance,
                        2.5,
                    ),
                ),
                BonusData::new(
                    "Glacial Presence 2",
                    BonusSource::Passive,
                    BonusTrigger::Passive,
                    BonusValue::new(
                        "Glacial Presence (Damage)",
                        BonusTarget::ChilledStatusEffectDamage,
                        105.0, // TODO: scales of WeaponOrSpellDamage
                    ),
                ),
            ]""",
    },
    "Piercing Cold": {
        "bonuses": """\
vec![
                BonusData::new(
                    "Piercing Cold",
                    BonusSource::Passive,
                    BonusTrigger::SkillLineSlotted,
                    BonusValue::new("Piercing Cold", BonusTarget::FrostDamage, 0.15),
                ),
            ]""",
    },

    # ---- Arcanist ----
    "Fated Fortune": {
        "bonuses": """\
vec![BonusData::new(
                "Fated Fortune",
                BonusSource::Passive,
                BonusTrigger::ArcanistCrux,
                BonusValue::new("Fated Fortune", BonusTarget::CriticalDamage, 0.12),
            )
            .with_duration(7.0)]""",
    },
    "Harnessed Quintessence": {
        "bonuses": """\
vec![BonusData::new(
                "Harnessed Quintessence W",
                BonusSource::Passive,
                BonusTrigger::MagickaOrStaminaRestored,
                BonusValue::new(
                    "Harnessed Quintessence W",
                    BonusTarget::WeaponAndSpellDamageFlat,
                    284.0,
                ),
            )
            .with_duration(10.0)]""",
    },
    "Psychic Lesion": {
        "bonuses": """\
vec![
                BonusData::new(
                    "Psychic Lesion (Damage)",
                    BonusSource::Passive,
                    BonusTrigger::AbilitySlotted,
                    BonusValue::new(
                        "Psychic Lesion (Damage)",
                        BonusTarget::StatusEffectDamage,
                        0.15,
                    ),
                )
                .with_duration(10.0),
                BonusData::new(
                    "Psychic Lesion (Chance)",
                    BonusSource::Passive,
                    BonusTrigger::AbilitySlotted,
                    BonusValue::new(
                        "Psychic Lesion (Chance)",
                        BonusTarget::StatusEffectChance,
                        0.55,
                    ),
                )
                .with_duration(10.0),
            ]""",
    },
    "Splintered Secrets": {
        "bonuses": """\
vec![BonusData::new(
                "Splintered Secrets",
                BonusSource::Passive,
                BonusTrigger::AbilitySlottedCount,
                BonusValue::new(
                    "Splintered Secrets",
                    BonusTarget::PhysicalAndSpellPenetration,
                    1240.0,
                ),
            )
            .with_skill_line_filter(SkillLineName::HeraldOfTheTome)]""",
    },
    "Hideous Clarity": {
        "bonuses": """\
vec![BonusData::new(
                "Hideous Clarity",
                BonusSource::Passive,
                BonusTrigger::ArcanistCrux, // TODO: should be only on generate
                BonusValue::new(
                    "Hideous Clarity",
                    BonusTarget::RestoreMagickaOrStamina,
                    225.0,
                ),
            )]""",
    },

    # ---- Bow ----
    "Long Shots": {
        "bonuses": """\
vec![BonusData::new(
                "Long Shots",
                BonusSource::Passive,
                BonusTrigger::BowEquipped,
                BonusValue::new("Long Shots (Damage)", BonusTarget::Damage, 0.05),
            )
            .with_alternative(BonusValue::new(
                "Long Shots (Crit Rating)",
                BonusTarget::CriticalRating,
                1314.0,
            ))]""",
    },
    "Accuracy": {
        "bonuses": """\
vec![BonusData::new(
                "Accuracy",
                BonusSource::Passive,
                BonusTrigger::BowEquipped,
                BonusValue::new("Accuracy", BonusTarget::CriticalRating, 1314.0),
            )]""",
    },
    "Hawk Eye": {
        "bonuses": "vec![]",
        "comment": "TODO: To complex, Stacks per basic of increase damage",
    },

    # ---- Destruction Staff ----
    "Tri Focus": {
        "bonuses": "vec![]",
        "comment": "TODO: Requires HA mechanic",
    },
    "Penetrating Magic": {
        "bonuses": """\
vec![BonusData::new(
                "Penetrating Magic",
                BonusSource::Passive,
                BonusTrigger::DestructionStuffEquipped,
                BonusValue::new(
                    "Penetrating Magic",
                    BonusTarget::PhysicalAndSpellPenetration,
                    2974.0,
                ),
            )
            .with_skill_line_filter(SkillLineName::Bow)]""",
    },
    "Elemental Force": {
        "bonuses": """\
vec![BonusData::new(
                "Elemental Force",
                BonusSource::Passive,
                BonusTrigger::DestructionStuffEquipped,
                BonusValue::new("Elemental Force", BonusTarget::StatusEffectChance, 1.0),
            )]""",
    },
    "Ancient Knowledge": {
        "bonuses": """\
vec![BonusData::new(
                "Ancient Knowledge (Inferno)",
                BonusSource::Passive,
                BonusTrigger::DestructionStuffEquipped,
                BonusValue::new("Ancient Knowledge (Inferno)", BonusTarget::DotDamage, 0.12),
            )
            .with_alternative(BonusValue::new(
                "Ancient Knowledge (Lightning)",
                BonusTarget::DirectDamage,
                0.12,
            ))]""",
    },
    "Destruction Expert": {
        "bonuses": "vec![]",
        "comment": "Not tracked - resource recovery on kill",
    },

    # ---- Dual Wield ----
    "Slaughter": {
        "bonuses": """\
vec![BonusData::new(
                "Slaughter",
                BonusSource::Passive,
                BonusTrigger::DualWieldEquipped,
                BonusValue::new("Slaughter", BonusTarget::Damage, 0.20),
            )
            .with_execute_threshold(0.25)
            .with_skill_line_filter(SkillLineName::DualWield)]""",
    },
    "Dual Wield Expert": {
        "bonuses": "vec![]",
        "comment": "TODO: Requires weapon stats tracking",
    },
    "Twin Blade and Blunt": {
        "bonuses": """\
vec![BonusData::new(
                "Twin Blade and Blunt (Axe)",
                BonusSource::Passive,
                BonusTrigger::DualWieldEquipped,
                BonusValue::new(
                    "Twin Blade and Blunt (Axe)",
                    BonusTarget::CriticalDamage,
                    0.06,
                ),
            )
            .with_alternative(BonusValue::new(
                "Twin Blade and Blunt (Mace)",
                BonusTarget::PhysicalAndSpellPenetration,
                1487.0,
            ))
            .with_alternative(BonusValue::new(
                "Twin Blade and Blunt (Sword)",
                BonusTarget::WeaponAndSpellDamageFlat,
                129.0,
            ))
            .with_alternative(BonusValue::new(
                "Twin Blade and Blunt (Dagger)",
                BonusTarget::CriticalRating,
                657.0,
            ))]""",
    },

    # ---- Two Handed ----
    "Heavy Weapons": {
        "bonuses": """\
vec![BonusData::new(
                "Heavy Weapons",
                BonusSource::Passive,
                BonusTrigger::TwoHandedEquipped,
                BonusValue::new(
                    "Heavy Weapons (Sword)",
                    BonusTarget::WeaponAndSpellDamageFlat,
                    258.0,
                ),
            )
            .with_alternative(BonusValue::new(
                "Heavy Weapons (Axe)",
                BonusTarget::CriticalDamage,
                0.12,
            ))
            .with_alternative(BonusValue::new(
                "Heavy Weapons (Mace)",
                BonusTarget::PhysicalAndSpellPenetration,
                2974.0,
            ))]""",
    },
    "Follow Up": {
        "bonuses": "vec![]",
        "comment": "TODO: Requires Heavy Attack tracking to implement",
    },

    # ---- Fighters Guild ----
    "Slayer": {
        "bonuses": """\
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
            .with_skill_line_filter(SkillLineName::FightersGuild)]""",
    },
    "Skilled Tracker": {
        "bonuses": """\
vec![BonusData::new(
                "Skilled Tracker",
                BonusSource::Passive,
                BonusTrigger::Passive,
                BonusValue::new("Skilled Tracker", BonusTarget::Damage, 0.10),
            )
            .with_skill_line_filter(SkillLineName::FightersGuild)]""",
    },

    # ---- Mages Guild ----
    "Everlasting Magic": {
        "bonuses": """\
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
            .with_skill_line_filter(SkillLineName::MagesGuild)]""",
    },
    "Magicka Controller": {
        "bonuses": """\
vec![BonusData::new(
                "Magicka Controller",
                BonusSource::Passive,
                BonusTrigger::AbilitySlottedCount,
                BonusValue::new("Magicka Controller", BonusTarget::MaxMagicka, 0.02),
            )
            .with_skill_line_filter(SkillLineName::MagesGuild)]""",
    },
    "Might of the Guild": {
        "bonuses": """\
vec![EMPOWER
                .clone()
                .with_trigger(BonusTrigger::SkillLineSkillCast)
                .with_skill_line_filter(SkillLineName::MagesGuild)]""",
    },

    # ---- Undaunted ----
    "Undaunted Mettle": {
        "bonuses": "vec![]",
    },
}

# ---------------------------------------------------------------------------
# Undaunted extra code (emitted before the static)
# ---------------------------------------------------------------------------

UNDAUNTED_EXTRA_CODE = """\
/// Returns Undaunted Mettle bonuses based on number of distinct armor weights worn.
/// Each armor type grants +2% MaxMagicka and +2% MaxStamina.
/// Standard 5/1/1 builds use 3 armor types → +6% each.
pub fn undaunted_mettle_bonuses(armor_types: u8) -> Vec<BonusData> {
    if armor_types == 0 {
        return vec![];
    }
    let pct = armor_types as f64 * 0.02;
    vec![
        BonusData::new(
            "Undaunted Mettle",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Max Magicka", BonusTarget::MaxMagicka, pct),
        ),
        BonusData::new(
            "Undaunted Mettle",
            BonusSource::Passive,
            BonusTrigger::Passive,
            BonusValue::new("Max Stamina", BonusTarget::MaxStamina, pct),
        ),
    ]
}

"""


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def escape_rust_string(s: str) -> str:
    """Escape a string for use in Rust string literals."""
    return s.replace("\\", "\\\\").replace('"', '\\"')


def generate_passive(name: str, skill_id: str | None, skill_line_variant: str,
                     class_variant: str) -> str:
    """Generate Rust code for a single PassiveData entry."""
    escaped_name = escape_rust_string(name)
    overrides = PASSIVE_OVERRIDES.get(name, {})

    bonuses = overrides.get("bonuses", "vec![]")
    comment = overrides.get("comment")

    # Format the comment inline with vec![] if present
    if comment and bonuses == "vec![]":
        bonuses_str = f"vec![], // {comment}"
    else:
        bonuses_str = bonuses

    parts = [
        f'        PassiveData::new(\n'
        f'            "{escaped_name}",\n'
        f'            ClassName::{class_variant},\n'
        f'            SkillLineName::{skill_line_variant},\n'
        f'            {bonuses_str},\n'
        f'        )'
    ]

    if skill_id is not None:
        parts.append(f'\n            .with_skill_id({skill_id})')

    return "".join(parts)


# ---------------------------------------------------------------------------
# Import detection
# ---------------------------------------------------------------------------

ALL_BONUS_CONSTANTS = {
    "EMPOWER", "MAJOR_BERSERK", "MAJOR_BREACH", "MAJOR_BRUTALITY",
    "MAJOR_PROPHECY", "MAJOR_SAVAGERY", "MAJOR_SORCERY", "MAJOR_VULNERABILITY",
    "MINOR_BERSERK", "MINOR_BREACH", "MINOR_BRUTALITY", "MINOR_FORCE",
    "MINOR_PROPHECY", "MINOR_SAVAGERY", "MINOR_SORCERY", "MINOR_VULNERABILITY",
}

REEXPORTED_BONUSES = {
    "EMPOWER", "MAJOR_BERSERK", "MAJOR_BREACH", "MAJOR_BRUTALITY",
    "MAJOR_PROPHECY", "MAJOR_SAVAGERY", "MAJOR_SORCERY", "MAJOR_VULNERABILITY",
    "MINOR_BERSERK", "MINOR_BREACH", "MINOR_BRUTALITY",
    "MINOR_PROPHECY", "MINOR_SAVAGERY", "MINOR_SORCERY", "MINOR_VULNERABILITY",
}

UNIQUE_ONLY_BONUSES = {"MINOR_FORCE", "EMPOWER"}


def detect_bonus_imports(code: str, is_class: bool) -> tuple[set[str], set[str]]:
    """Detect which bonus constants are referenced in the generated code.
    Returns (reexported_set, unique_only_set) for class files,
    or (empty_set, all_found_set) for weapon/guild files."""
    reexported = set()
    unique_only = set()
    for const in ALL_BONUS_CONSTANTS:
        if const in code:
            if is_class:
                # Class files import everything from crate::data::bonuses
                reexported.add(const)
            else:
                # Weapon/guild files import from crate::data::bonuses::unique
                unique_only.add(const)
    return reexported, unique_only


def detect_domain_imports(code: str) -> set[str]:
    """Detect which domain types are referenced in the generated code."""
    types = {"ClassName", "PassiveData", "SkillLineName"}
    if "BonusData::" in code or "BonusData::new" in code:
        types.add("BonusData")
    if "BonusSource::" in code:
        types.add("BonusSource")
    if "BonusTarget::" in code:
        types.add("BonusTarget")
    if "BonusTrigger::" in code:
        types.add("BonusTrigger")
    if "BonusValue::" in code or "BonusValue::new" in code:
        types.add("BonusValue")
    return types


def generate_file(passives: list[tuple[str, str | None, str, str]],
                  file_stem: str, class_group: str) -> str:
    """Generate a complete Rust source file.

    passives: list of (name, skill_id, skill_line_variant, class_variant)
    """
    # Generate all passive entries first to detect imports
    passive_entries = []
    for name, skill_id, sl_variant, cls_variant in passives:
        passive_entries.append(generate_passive(name, skill_id, sl_variant, cls_variant))

    passives_code = ",\n".join(passive_entries)

    is_class = class_group in {
        "Templar", "Dragonknight", "Nightblade", "Sorcerer",
        "Necromancer", "Warden", "Arcanist",
    }

    # Detect imports
    reexported_bonuses, unique_bonuses = detect_bonus_imports(passives_code, is_class)
    domain_types = detect_domain_imports(passives_code)

    # Build import lines
    import_lines = []

    if reexported_bonuses:
        names = sorted(reexported_bonuses)
        if len(names) == 1:
            import_lines.append(f"use crate::data::bonuses::{names[0]};")
        else:
            import_lines.append(
                f"use crate::data::bonuses::{{{', '.join(names)}}};"
            )

    if unique_bonuses:
        names = sorted(unique_bonuses)
        if len(names) == 1:
            import_lines.append(f"use crate::data::bonuses::unique::{names[0]};")
        else:
            import_lines.append(
                f"use crate::data::bonuses::unique::{{{', '.join(names)}}};"
            )

    # Domain imports
    domain_sorted = sorted(domain_types)
    import_lines.append(
        f"use crate::domain::{{{', '.join(domain_sorted)}}};"
    )
    import_lines.append("use once_cell::sync::Lazy;")

    imports = "\n".join(import_lines)

    # Static name
    static_name = f"{file_stem.upper()}_PASSIVES"

    # Extra code (for undaunted)
    extra = ""
    if file_stem == "undaunted":
        extra = "\n" + UNDAUNTED_EXTRA_CODE
        # The extra code uses BonusData, BonusSource, etc. - detect those too
        extra_domain = detect_domain_imports(UNDAUNTED_EXTRA_CODE)
        domain_types |= extra_domain
        # Rebuild domain import line
        domain_sorted = sorted(domain_types)
        # Also re-check for bonus imports in extra code
        extra_reexported, extra_unique = detect_bonus_imports(UNDAUNTED_EXTRA_CODE, is_class)
        reexported_bonuses |= extra_reexported
        unique_bonuses |= extra_unique

        # Rebuild all imports
        import_lines = []
        if reexported_bonuses:
            names = sorted(reexported_bonuses)
            if len(names) == 1:
                import_lines.append(f"use crate::data::bonuses::{names[0]};")
            else:
                import_lines.append(
                    f"use crate::data::bonuses::{{{', '.join(names)}}};"
                )
        if unique_bonuses:
            names = sorted(unique_bonuses)
            if len(names) == 1:
                import_lines.append(f"use crate::data::bonuses::unique::{names[0]};")
            else:
                import_lines.append(
                    f"use crate::data::bonuses::unique::{{{', '.join(names)}}};"
                )
        import_lines.append(
            f"use crate::domain::{{{', '.join(domain_sorted)}}};"
        )
        import_lines.append("use once_cell::sync::Lazy;")
        imports = "\n".join(import_lines)

    return (
        f"// Auto-generated by datamine/generate_passives_rs.py - do not edit manually.\n"
        f"// Manual overrides (bonuses) stored in generator script.\n"
        f"{imports}\n"
        f"{extra}\n"
        f"pub static {static_name}: Lazy<Vec<PassiveData>> = Lazy::new(|| {{\n"
        f"    vec![\n"
        f"{passives_code},\n"
        f"    ]\n"
        f"}});\n"
    )


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    base = Path(__file__).parent
    input_path = base / "parsed" / "skill_coefficients.json"
    output_base = base.parent / "src" / "data" / "skill_trees"

    data = json.loads(input_path.read_text(encoding="utf-8"))

    # Group entries by (base_name, skill_line) - base_name strips trailing " N"
    from collections import defaultdict
    groups: dict[tuple[str, str], list[dict]] = defaultdict(list)
    for entry in data:
        sl = entry.get("skill_line", "")
        if sl not in RELEVANT_SKILL_LINES:
            continue
        name = entry["name"]
        base_name = re.sub(r" \d+$", "", name)
        groups[(base_name, sl)].append(entry)

    # Passives have < 4 versions; take the highest version entry
    json_passives: list[tuple[str, str, str]] = []  # (name, skill_id, skill_line)
    for (base_name, sl), entries in groups.items():
        if len(entries) >= 4:
            continue  # This is an active skill
        best = entries[-1]  # Highest version
        corrected_name = NAME_CORRECTIONS.get(base_name, base_name)
        json_passives.append((corrected_name, best["id"], sl))

    # Add manual passives (not in JSON)
    for name, info in MANUAL_PASSIVES.items():
        # Check if already found in JSON
        already = any(n == name and sl == info["skill_line"] for n, _, sl in json_passives)
        if not already:
            json_passives.append((name, None, info["skill_line"]))

    # Group by output file
    files: dict[str, list[tuple[str, str | None, str, str]]] = defaultdict(list)
    for name, skill_id, skill_line in json_passives:
        if skill_line not in SKILL_LINE_MAP:
            continue
        sl_variant, class_group, dir_path, file_stem = SKILL_LINE_MAP[skill_line]
        class_variant = class_group
        key = f"{dir_path}/{file_stem}"
        files[key].append((name, skill_id, sl_variant, class_variant))

    # Sort passives within each file by skill_line order then name
    # Use SKILL_LINE_MAP ordering to preserve skill line grouping
    skill_line_order = list(SKILL_LINE_MAP.keys())
    sl_variant_order = {v[0]: i for i, v in enumerate(SKILL_LINE_MAP.values())}

    for key in files:
        files[key].sort(key=lambda x: (sl_variant_order.get(x[2], 999), x[0]))

    # Generate output files
    for key, passives in sorted(files.items()):
        dir_path, file_stem = key.rsplit("/", 1)
        # class_variant is the same as class_group for our purposes
        class_group = passives[0][3]

        code = generate_file(passives, file_stem, class_group)
        out_path = output_base / dir_path / f"{file_stem}_passives.rs"
        out_path.write_text(code, encoding="utf-8")
        print(f"Generated {len(passives)} passives -> {out_path}")


if __name__ == "__main__":
    main()
