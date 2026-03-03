"""Generate Rust skill data files from parsed skill_coefficients.json.

Reads datamine/parsed/skill_coefficients.json and generates:
  - src/data/skill_trees/character_class/{class}/{class}_skills.rs  (7 files)
  - src/data/skill_trees/weapon/{weapon}/{weapon}_skills.rs          (4 files)
  - src/data/skill_trees/guild/{guild}/{guild}_skills.rs              (4 files)

Each file exports `pub static {NAME}_SKILLS: Lazy<Vec<SkillData>>`.
Manual overrides (bonuses, execute, spammable, etc.) are stored in this script.
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

# Element-specific destruction staff variants to exclude (we keep generic versions)
DESTRO_ELEMENT_VARIANTS = {
    "Fire Storm", "Ice Storm", "Thunder Storm",
    "Fiery Rage", "Icy Rage", "Thunderous Rage",
    "Eye of Flame", "Eye of Frost", "Eye of Lightning",
    "Fire Impulse", "Frost Impulse", "Shock Impulse",
    "Fire Ring", "Frost Ring", "Shock Ring",
    "Flame Pulsar", "Frost Pulsar", "Storm Pulsar",
    "Wall of Fire", "Wall of Frost", "Wall of Storms",
    "Blockade of Fire", "Blockade of Frost", "Blockade of Storms",
    "Unstable Wall of Fire", "Unstable Wall of Frost", "Unstable Wall of Storms",
    "Flame Touch", "Frost Touch", "Shock Touch",
    "Flame Clench", "Frost Clench", "Shock Clench",
    "Flame Reach", "Frost Reach", "Shock Reach",
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

# mechanic (JSON) -> Resource variant
MECHANIC_MAP = {
    "Magicka":      "Magicka",
    "Stamina":      "Stamina",
    "Werewolf":     "Magicka",
    "Power":        "Stamina",
    "Adrenaline":   "Ultimate",
    "Unknown (32)": "Stamina",
}

# Tag -> DamageFlags constant
TAG_TO_FLAG = {
    "Magic":        "MAGIC",
    "Physical":     "PHYSICAL",
    "Flame":        "FLAME",
    "Frost":        "FROST",
    "Shock":        "SHOCK",
    "Poison":       "POISON",
    "Disease":      "DISEASE",
    "Bleed":        "BLEED",
    "AOE":          "AOE",
    "SingleTarget": "SINGLE_TARGET",
    "Melee":        "MELEE",
    "Ranged":       "RANGED",
    "Channel":      "CHANNEL",
    # DIRECT and DOT are auto-added by constructors, skip them
}

# ---------------------------------------------------------------------------
# Base name mapping (morph name -> base skill name)
# ---------------------------------------------------------------------------

BASE_NAMES = {
    "Absorption Field": "Negate Magic",
    "Acid Spray": "Arrow Spray",
    "Agony Totem": "Bone Totem",
    "Ambush": "Teleport Strike",
    "Arctic Blast": "Arctic Wind",
    "Arrow Barrage": "Volley",
    "Audacious Runemend": "Runemend",
    "Aurora Javelin": "Piercing Javelin",
    "Avid Boneyard": "Boneyard",
    "Ball of Lightning": "Bolt Escape",
    "Ballista": "Rapid Fire",
    "Barbed Trap": "Trap Beast",
    "Beckoning Armor": "Bone Armor",
    "Berserker Rage": "Berserker Strike",
    "Binding Javelin": "Piercing Javelin",
    "Bird of Prey": "Falcon's Swiftness",
    "Biting Jabs": "Puncturing Strikes",
    "Blazing Shield": "Sun Shield",
    "Blazing Spear": "Spear Shards",
    "Blighted Blastbones": "Sacrificial Bones",
    "Blood Craze": "Twin Slashes",
    "Blood Sacrifice": "Render Flesh",
    "Bloodthirst": "Flurry",
    "Blue Betty": "Betty Netch",
    "Bolstering Darkness": "Consuming Darkness",
    "Bombard": "Arrow Spray",
    "Bound Aegis": "Bound Armor",
    "Bound Armaments": "Bound Armor",
    "Boundless Storm": "Lightning Form",
    "Braided Tether": "Restoring Tether",
    "Brawler": "Cleave",
    "Breath of Life": "Rushed Ceremony",
    "Budding Seeds": "Healing Seed",
    "Bull Netch": "Betty Netch",
    "Burning Embers": "Searing Strike",
    "Burning Talons": "Dark Talons",
    "Bursting Vines": "Nature's Grasp",
    "Camouflaged Hunter": "Expert Hunter",
    "Carve": "Cleave",
    "Cascading Fortune": "Remedy Cascade",
    "Cauterize": "Inferno",
    "Cephaliarch's Flail": "Abyssal Impact",
    "Chains of Devastation": "Fiery Grip",
    "Chakram of Destiny": "Chakram Shields",
    "Channeled Acceleration": "Accelerate",
    "Channeled Focus": "Rune Focus",
    "Choking Talons": "Dark Talons",
    "Cinder Storm": "Ash Cloud",
    "Coagulating Blood": "Dragon Blood",
    "Concealed Weapon": "Veiled Strike",
    "Corrosive Armor": "Magma Armor",
    "Corrupting Pollen": "Healing Seed",
    "Crescent Sweep": "Radial Sweep",
    "Crippling Grasp": "Cripple",
    "Critical Rush": "Critical Charge",
    "Critical Surge": "Surge",
    "Crushing Shock": "Force Shock",
    "Crushing Weapon": "Imbue Weapon",
    "Cruxweaver Armor": "Fatewoven Armor",
    "Crystal Fragments": "Crystal Shard",
    "Crystal Weapon": "Crystal Shard",
    "Crystallized Slab": "Crystallized Shield",
    "Curative Surge": "Remedy Cascade",
    "Cutting Dive": "Dive",
    "Daedric Prey": "Daedric Curse",
    "Daedric Refuge": "Daedric Mines",
    "Daedric Tomb": "Daedric Mines",
    "Dark Cloak": "Shadow Cloak",
    "Dark Conversion": "Dark Exchange",
    "Dark Deal": "Dark Exchange",
    "Dark Flare": "Solar Flare",
    "Dark Shade": "Summon Shade",
    "Dawnbreaker of Smiting": "Dawnbreaker",
    "Deaden Pain": "Bitter Harvest",
    "Deadly Cloak": "Blade Cloak",
    "Debilitate": "Cripple",
    "Deceptive Predator": "Falcon's Swiftness",
    "Deep Breath": "Inhale",
    "Deep Fissure": "Scorch",
    "Defensive Rune": "Rune Prison",
    "Degeneration": "Entropy",
    "Destructive Clench": "Destructive Touch",
    "Destructive Reach": "Destructive Touch",
    "Detonating Siphon": "Shocking Siphon",
    "Dizzying Swing": "Uppercut",
    "Dragon Fire Scale": "Protective Scale",
    "Draining Shot": "Scatter Shot",
    "Draw Essence": "Inhale",
    "Elemental Blockade": "Wall of Elements",
    "Elemental Drain": "Weakness to Elements",
    "Elemental Rage": "Elemental Storm",
    "Elemental Ring": "Impulse",
    "Elemental Susceptibility": "Weakness to Elements",
    "Elemental Weapon": "Imbue Weapon",
    "Empowering Grasp": "Grave Grasp",
    "Enchanted Forest": "Secluded Grove",
    "Enchanted Growth": "Fungal Growth",
    "Endless Fury": "Mages' Fury",
    "Endless Hail": "Volley",
    "Enduring Undeath": "Life amid Death",
    "Energy Overload": "Overload",
    "Engulfing Flames": "Fiery Breath",
    "Eruption": "Ash Cloud",
    "Escalating Runeblades": "Runeblades",
    "Eternal Guardian": "Feral Guardian",
    "Everlasting Sweep": "Radial Sweep",
    "Evil Hunter": "Expert Hunter",
    "Evolving Runemend": "Runemend",
    "Executioner": "Reverse Slash",
    "Exhausting Fatecarver": "Fatecarver",
    "Expansive Frost Cloak": "Frost Cloak",
    "Explosive Charge": "Focused Charge",
    "Expunge and Modify": "Expunge",
    "Extended Ritual": "Cleansing Ritual",
    "Eye of the Storm": "Elemental Storm",
    "Ferocious Leap": "Dragon Leap",
    "Fetcher Infection": "Swarm",
    "Flame Lash": "Lava Whip",
    "Flames of Oblivion": "Inferno",
    "Flawless Dawnbreaker": "Dawnbreaker",
    "Fleet-Footed Gate": "Apocryphal Gate",
    "Flying Blade": "Hidden Blade",
    "Focused Aim": "Snipe",
    "Force Pulse": "Force Shock",
    "Forward Momentum": "Momentum",
    "Fossilize": "Petrify",
    "Fragmented Shield": "Obsidian Shield",
    "Frozen Device": "Frozen Gate",
    "Frozen Retreat": "Frozen Gate",
    "Fulminating Rune": "The Imperfect Ring",
    "Funnel Health": "Strife",
    "Ghostly Embrace": "Grave Grasp",
    "Gibbering Shelter": "Gibbering Shield",
    "Glacial Colossus": "Frozen Colossus",
    "Glyphic of the Tides": "Vitalizing Glyphic",
    "Grave Lord's Sacrifice": "Sacrificial Bones",
    "Greater Storm Atronach": "Summon Storm Atronach",
    "Green Dragon Blood": "Dragon Blood",
    "Green Lotus": "Lotus Flower",
    "Gripping Shards": "Impaling Shards",
    "Growing Swarm": "Swarm",
    "Hardened Armor": "Spiked Armor",
    "Hardened Ward": "Conjured Ward",
    "Hasty Prayer": "Healing Ritual",
    "Haunting Curse": "Daedric Curse",
    "Healing Thicket": "Secluded Grove",
    "Healthy Offering": "Malevolent Offering",
    "Hexproof": "Expunge",
    "Honor the Dead": "Rushed Ceremony",
    "Hungry Scythe": "Death Scythe",
    "Hurricane": "Lightning Form",
    "Ice Comet": "Meteor",
    "Ice Fortress": "Frost Cloak",
    "Igneous Shield": "Obsidian Shield",
    "Igneous Weapons": "Molten Weapons",
    "Impale": "Assassin's Blade",
    "Impervious Runeward": "Runespite Ward",
    "Incapacitating Strike": "Death Stroke",
    "Inner Beast": "Inner Fire",
    "Inner Light": "Magelight",
    "Inner Rage": "Inner Fire",
    "Inspired Scholarship": "Tome-Bearer's Inspiration",
    "Intensive Mender": "Spirit Mender",
    "Killer's Blade": "Assassin's Blade",
    "Leeching Strikes": "Siphoning Strikes",
    "Leeching Vines": "Living Vines",
    "Lethal Arrow": "Snipe",
    "Lightning Flood": "Lightning Splash",
    "Lightweight Beast Trap": "Trap Beast",
    "Liquid Lightning": "Lightning Splash",
    "Living Dark": "Eclipse",
    "Living Trellis": "Living Vines",
    "Lotus Blossom": "Lotus Flower",
    "Lotus Fan": "Teleport Strike",
    "Luminous Shards": "Spear Shards",
    "Mages' Wrath": "Mages' Fury",
    "Magma Shell": "Magma Armor",
    "Magnum Shot": "Scatter Shot",
    "Manifestation of Terror": "Aspect of Terror",
    "Mass Hysteria": "Aspect of Terror",
    "Merciless Resolve": "Grim Focus",
    "Mirage": "Blur",
    "Molten Armaments": "Molten Weapons",
    "Molten Whip": "Lava Whip",
    "Mortal Coil": "Restoring Tether",
    "Mystic Orb": "Necrotic Orb",
    "Mystic Siphon": "Shocking Siphon",
    "Nature's Embrace": "Nature's Grasp",
    "Necrotic Potency": "Bitter Harvest",
    "Northern Storm": "Sleet Storm",
    "Noxious Breath": "Fiery Breath",
    "Obsidian Shard": "Stonefist",
    "Onslaught": "Berserker Strike",
    "Passage Between Worlds": "Apocryphal Gate",
    "Permafrost": "Sleet Storm",
    "Pestilent Colossus": "Frozen Colossus",
    "Phantasmal Escape": "Blur",
    "Piercing Mark": "Mark Target",
    "Poison Injection": "Poison Arrow",
    "Polar Wind": "Arctic Wind",
    "Power Extraction": "Drain Power",
    "Power Overload": "Overload",
    "Power Surge": "Surge",
    "Power of the Light": "Backlash",
    "Practiced Incantation": "Rite of Passage",
    "Pragmatic Fatecarver": "Fatecarver",
    "Protective Plate": "Protective Scale",
    "Pulsar": "Impulse",
    "Pummeling Goliath": "Bone Goliath Transformation",
    "Puncturing Sweep": "Puncturing Strikes",
    "Purifying Light": "Backlash",
    "Quick Cloak": "Blade Cloak",
    "Race Against Time": "Accelerate",
    "Radiant Aura": "Restoring Aura",
    "Radiant Glory": "Radiant Destruction",
    "Radiant Magelight": "Magelight",
    "Radiant Oppression": "Radiant Destruction",
    "Radiant Ward": "Sun Shield",
    "Rally": "Momentum",
    "Rapid Strikes": "Flurry",
    "Ravenous Goliath": "Bone Goliath Transformation",
    "Reaper's Mark": "Mark Target",
    "Reconstructive Domain": "Arcanist's Domain",
    "Recuperative Treatise": "Tome-Bearer's Inspiration",
    "Reflective Light": "Sun Fire",
    "Refreshing Path": "Path of Darkness",
    "Regenerative Ward": "Conjured Ward",
    "Relentless Focus": "Grim Focus",
    "Remembrance": "Rite of Passage",
    "Remote Totem": "Bone Totem",
    "Rend": "Lacerate",
    "Rending Slashes": "Twin Slashes",
    "Renewing Animation": "Reanimate",
    "Renewing Undeath": "Life amid Death",
    "Repentance": "Restoring Aura",
    "Resistant Flesh": "Render Flesh",
    "Resonating Glyphic": "Vitalizing Glyphic",
    "Restoring Focus": "Rune Focus",
    "Reverse Slice": "Reverse Slash",
    "Ricochet Skull": "Flame Skull",
    "Ritual of Rebirth": "Healing Ritual",
    "Ritual of Retribution": "Cleansing Ritual",
    "Rune Cage": "Rune Prison",
    "Rune of Displacement": "The Imperfect Ring",
    "Rune of Uncanny Adoration": "Rune of Eldritch Horror",
    "Rune of the Colorless Pool": "Rune of Eldritch Horror",
    "Runeguard of Freedom": "Runic Defense",
    "Runeguard of Still Waters": "Runic Defense",
    "Runic Embrace": "Runic Jolt",
    "Runic Sunder": "Runic Jolt",
    "Sanctum of the Abyssal Sea": "Gibbering Shield",
    "Sap Essence": "Drain Power",
    "Scalding Rune": "Fire Rune",
    "Screaming Cliff Racer": "Dive",
    "Shadow Image": "Summon Shade",
    "Shadow Silk": "Trapping Webs",
    "Shadowy Disguise": "Shadow Cloak",
    "Shattering Rocks": "Petrify",
    "Shattering Spines": "Encase",
    "Shifting Standard": "Dragonknight Standard",
    "Shimmering Shield": "Crystallized Shield",
    "Shooting Star": "Meteor",
    "Shrewd Offering": "Malevolent Offering",
    "Shrouded Daggers": "Hidden Blade",
    "Silver Leash": "Silver Bolts",
    "Silver Shards": "Silver Bolts",
    "Siphoning Attacks": "Siphoning Strikes",
    "Skeletal Arcanist": "Skeletal Mage",
    "Skeletal Archer": "Skeletal Mage",
    "Solar Barrage": "Solar Flare",
    "Solar Disturbance": "Nova",
    "Solar Prison": "Nova",
    "Soothing Spores": "Fungal Growth",
    "Soul Harvest": "Death Stroke",
    "Soul Siphon": "Soul Shred",
    "Soul Tether": "Soul Shred",
    "Spirit Guardian": "Spirit Mender",
    "Spiteward of the Lucid Mind": "Runespite Ward",
    "Stampede": "Critical Charge",
    "Standard of Might": "Dragonknight Standard",
    "Steel Tornado": "Whirlwind",
    "Stone Giant": "Stonefist",
    "Streak": "Bolt Escape",
    "Structured Entropy": "Entropy",
    "Subterranean Assault": "Scorch",
    "Summon Charged Atronach": "Summon Storm Atronach",
    "Summon Twilight Matriarch": "Summon Winged Twilight",
    "Summon Twilight Tormentor": "Summon Winged Twilight",
    "Summon Unstable Clannfear": "Summon Unstable Familiar",
    "Summon Volatile Familiar": "Summon Unstable Familiar",
    "Summoner's Armor": "Bone Armor",
    "Suppression Field": "Negate Magic",
    "Surprise Attack": "Veiled Strike",
    "Swallow Soul": "Strife",
    "Take Flight": "Dragon Leap",
    "Tangling Webs": "Trapping Webs",
    "Tentacular Dread": "Abyssal Impact",
    "The Languid Eye": "The Unblinking Eye",
    "The Tide King's Gaze": "The Unblinking Eye",
    "Thrive in Chaos": "Lacerate",
    "Thunderous Volley": "Volley",
    "Tidal Chakram": "Chakram Shields",
    "Toppling Charge": "Focused Charge",
    "Toxic Barrage": "Rapid Fire",
    "Twisting Path": "Path of Darkness",
    "Unbreakable Fate": "Fatewoven Armor",
    "Unnerving Boneyard": "Boneyard",
    "Unrelenting Grip": "Fiery Grip",
    "Unstable Core": "Eclipse",
    "Unstable Wall of Elements": "Wall of Elements",
    "Vampire's Bane": "Sun Fire",
    "Veil of Blades": "Consuming Darkness",
    "Venom Arrow": "Poison Arrow",
    "Venom Skull": "Flame Skull",
    "Venomous Claw": "Searing Strike",
    "Vibrant Shroud": "Encase",
    "Volatile Armor": "Spiked Armor",
    "Volcanic Rune": "Fire Rune",
    "Whirling Blades": "Whirlwind",
    "Wild Guardian": "Feral Guardian",
    "Winter's Revenge": "Impaling Shards",
    "Wrecking Blow": "Uppercut",
    "Writhing Runeblades": "Runeblades",
    "Zenas' Empowering Disc": "Arcanist's Domain",
}

# ---------------------------------------------------------------------------
# DOT duration overrides for skills where duration can't be parsed
# ---------------------------------------------------------------------------

DOT_DURATION_OVERRIDES: dict[str, float] = {
    "Mystic Orb": 10.0,
    "Summon Unstable Familiar": 20.0,
    "Summon Volatile Familiar": 20.0,
    "Summon Unstable Clannfear": 20.0,
    "Arctic Blast": 20.0,
}

# ---------------------------------------------------------------------------
# Skill overrides — keyed by skill name (after stripping " 4")
# Contains raw Rust snippets for builder calls not derivable from JSON.
# ---------------------------------------------------------------------------

SKILL_OVERRIDES: dict[str, dict] = {
    # ---- Templar ----
    "Puncturing Strikes": {"channel_time": 0.8},
    "Biting Jabs": {
        "channel_time": 0.8,
        "bonuses": 'vec![MAJOR_BRUTALITY.clone().with_duration(10.0), MAJOR_SORCERY.clone().with_duration(10.0)]',
    },
    "Puncturing Sweep": {"channel_time": 0.8},
    "Solar Flare": {"spammable": True, "bonuses": 'vec![EMPOWER.clone()]'},
    "Dark Flare": {"spammable": True, "bonuses": 'vec![EMPOWER.clone()]'},
    "Solar Barrage": {"bonuses": 'vec![EMPOWER.clone().with_duration(20.0)]'},
    "Backlash": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_single(), 0.05, 0.525),
            HitDamage::new(DamageFlags::magic_single(), 0.055333, 0.581).with_delay(6.0),
        ])"""},
    "Power of the Light": {
        "bonuses": 'vec![MAJOR_BREACH.clone()]',
        "damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_single(), 0.05165, 0.542325),
            HitDamage::new(DamageFlags::physical_single(), 0.057159, 0.600173).with_delay(6.0),
        ])""",
    },
    "Purifying Light": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_single(), 0.05165, 0.54233),
            HitDamage::new(DamageFlags::magic_single(), 0.057159, 0.600173).with_delay(6.0),
        ])"""},
    "Unstable Core": {},  # auto-generated damage is fine
    "Radiant Destruction": {"channel_time": 3.8},
    "Radiant Glory": {"channel_time": 3.8},
    "Radiant Oppression": {"channel_time": 3.8},
    "Rite of Passage": {"channel_time": 4.0},
    "Practiced Incantation": {"channel_time": 8.0},
    "Remembrance": {"channel_time": 4.0},
    "Ritual of Retribution": {"damage": """\
SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::magic_aoe(),
            0.018782,
            0.19721,
        )
        .with_interval(2.0)
        .with_increase_per_tick(0.12)])"""},

    # ---- Dragonknight ----
    "Lava Whip": {"spammable": True},
    "Flame Lash": {"spammable": True},
    "Molten Whip": {"spammable": True},
    "Venomous Claw": {"damage": """\
SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::poison_single(),
                    0.05165,
                    0.542325,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::poison_single(),
                    0.015495,
                    0.162697,
                )
                .with_interval(2.0)
                .with_increase_per_tick(0.12)])"""},
    "Noxious Breath": {"bonuses": 'vec![MAJOR_BREACH.clone()]'},
    "Engulfing Flames": {},  # auto-generated damage is fine
    "Inferno": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone().with_duration(20.0),
            MAJOR_SORCERY.clone().with_duration(20.0),
        ]"""},
    "Cauterize": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone().with_duration(20.0),
            MAJOR_SORCERY.clone().with_duration(20.0),
        ]"""},
    "Flames of Oblivion": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone().with_duration(20.0),
            MAJOR_SORCERY.clone().with_duration(20.0),
        ]"""},
    "Inhale": {"channel_time": 2.5, "damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.075, 0.7875).with_delay(2.5),
        ])"""},
    "Deep Breath": {"channel_time": 2.5, "damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.077475, 0.813488).with_delay(2.5),
        ])"""},
    "Draw Essence": {"channel_time": 2.5, "damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.077475, 0.813488).with_delay(2.5),
        ])"""},
    "Stonefist": {"spammable": True},
    "Stone Giant": {"spammable": True},
    "Obsidian Shard": {"spammable": True},
    "Molten Weapons": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]"""},
    "Igneous Weapons": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]"""},
    "Molten Armaments": {"bonuses": """\
vec![
            EMPOWER.clone(),
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]"""},
    "Petrify": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_single(), 0.06, 0.63).with_delay(3.0),
        ])"""},
    "Fossilize": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_single(), 0.06198, 0.65079).with_delay(3.5),
        ])"""},
    "Shattering Rocks": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_single(), 0.06198, 0.65079).with_delay(3.0),
        ])"""},

    # ---- Nightblade ----
    "Death Stroke": {"bonuses": """\
vec![BonusData::new(
            "Death Stroke Debuff",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Death Stroke Debuff", BonusTarget::EnemyDamageTaken, 0.20),
        )
        .with_duration(8.0)]"""},
    "Incapacitating Strike": {"bonuses": """\
vec![BonusData::new(
            "Incapacitating Strike Debuff",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new(
                "Incapacitating Strike Debuff",
                BonusTarget::EnemyDamageTaken,
                0.20,
            ),
        )
        .with_duration(8.0)]"""},
    "Soul Harvest": {"bonuses": """\
vec![BonusData::new(
            "Soul Harvest Debuff",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Soul Harvest Debuff", BonusTarget::EnemyDamageTaken, 0.20),
        )
        .with_duration(8.0)]"""},
    "Veiled Strike": {"spammable": True},
    "Concealed Weapon": {"spammable": True},
    "Surprise Attack": {"spammable": True, "bonuses": 'vec![MAJOR_BREACH.clone()]'},
    "Teleport Strike": {"bonuses": 'vec![MINOR_VULNERABILITY.clone()]'},
    "Ambush": {"bonuses": """\
vec![
            MINOR_VULNERABILITY.clone(),
            EMPOWER.clone(),
            MINOR_BERSERK.clone().with_duration(10.0),
        ]"""},
    "Lotus Fan": {"bonuses": 'vec![MINOR_VULNERABILITY.clone()]'},
    "Assassin's Blade": {"spammable": True, "execute": (3.0, 0.25, "Flat")},
    "Impale": {"spammable": True, "execute": (3.3, 0.25, "Flat")},
    "Killer's Blade": {"spammable": True, "execute": (4.0, 0.50, "Linear")},
    "Mark Target": {"bonuses": 'vec![MAJOR_BREACH.clone()]'},
    "Piercing Mark": {"bonuses": 'vec![MAJOR_BREACH.clone().with_duration(60.0)]'},
    "Reaper's Mark": {"bonuses": """\
vec![
            MAJOR_BREACH.clone(),
            MAJOR_BERSERK.clone().with_duration(10.0),
        ]"""},
    "Grim Focus": {
        "proc_light_attacks": 4,
        "bonuses": """\
vec![
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]""",
    },
    "Merciless Resolve": {
        "proc_light_attacks": 4,
        "bonuses": """\
vec![
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]""",
    },
    "Relentless Focus": {
        "proc_light_attacks": 4,
        "bonuses": """\
vec![
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]""",
    },
    "Strife": {"spammable": True},
    "Funnel Health": {"spammable": True},
    "Swallow Soul": {"spammable": True},
    "Drain Power": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone().with_duration(30.0),
            MAJOR_SORCERY.clone().with_duration(30.0),
        ]"""},
    "Power Extraction": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone().with_duration(30.0),
            MAJOR_SORCERY.clone().with_duration(30.0),
        ]"""},
    "Sap Essence": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone().with_duration(30.0),
            MAJOR_SORCERY.clone().with_duration(30.0),
        ]"""},

    # ---- Sorcerer ----
    "Crystal Shard": {"spammable": True},
    "Crystal Fragments": {"spammable": True},
    "Crystal Weapon": {
        "spammable": True,
        "bonuses": """\
vec![BonusData::new(
            "Crystal Weapon Debuff",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new(
                "Crystal Weapon Debuff",
                BonusTarget::EnemyResistanceReduction,
                1000.0,
            ),
        )
        .with_duration(5.0)]""",
    },
    "Shattering Spines": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488).with_delay(1.5),
        ])"""},
    "Rune Cage": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_single(), 0.077475, 0.813488).with_delay(5.0),
        ])"""},
    "Daedric Mines": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.1, 1.05).with_delay(3.0),
        ])"""},
    "Daedric Tomb": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.1033, 1.08465).with_delay(3.0),
        ])"""},
    "Daedric Curse": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.15, 1.575).with_delay(3.5),
        ])"""},
    "Daedric Prey": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.15495, 1.62698).with_delay(6.0),
        ])"""},
    "Haunting Curse": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.10825, 1.13663).with_delay(3.5),
            HitDamage::new(DamageFlags::magic_aoe(), 0.10825, 1.13663).with_delay(6.5),
        ])"""},
    "Bound Armaments": {
        "bonuses": """\
vec![
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]""",
        "damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_single(), 0.024, 0.252),
            HitDamage::new(DamageFlags::physical_single(), 0.024, 0.252),
            HitDamage::new(DamageFlags::physical_single(), 0.024, 0.252),
            HitDamage::new(DamageFlags::physical_single(), 0.024, 0.252),
            HitDamage::new(DamageFlags::physical_single(), 0.024, 0.252),
            HitDamage::new(DamageFlags::physical_single(), 0.024, 0.252),
            HitDamage::new(DamageFlags::physical_single(), 0.024, 0.252),
            HitDamage::new(DamageFlags::physical_single(), 0.024, 0.252),
        ])""",
    },
    "Mages' Fury": {
        "spammable": True,
        "damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::shock_single(), 0.06, 0.63),
            HitDamage::new(DamageFlags::shock_single(), 0.09, 0.945).with_execute_threshold(0.20),
        ])""",
    },
    "Mages' Wrath": {
        "spammable": True,
        "damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::shock_single(), 0.06198, 0.65079),
            HitDamage::new(DamageFlags::shock_aoe(), 0.09297, 0.976185).with_execute_threshold(0.20),
        ])""",
    },
    "Endless Fury": {
        "spammable": True,
        "damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::shock_single(), 0.06198, 0.65079),
            HitDamage::new(DamageFlags::shock_single(), 0.09297, 0.976185).with_execute_threshold(0.20),
        ])""",
    },
    "Hurricane": {"damage": """\
SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::physical_aoe(),
            0.018782,
            0.19721,
        )
        .with_interval(2.0)
        .with_increase_per_tick(0.12)])"""},
    "Surge": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]"""},
    "Critical Surge": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]"""},
    "Power Surge": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]"""},

    # ---- Necromancer ----
    "Frozen Colossus": {
        "bonuses": 'vec![MAJOR_VULNERABILITY.clone().with_duration(12.0)]',
        "damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_aoe(), 0.15, 1.575),
            HitDamage::new(DamageFlags::frost_aoe(), 0.15, 1.575).with_delay(1.0),
            HitDamage::new(DamageFlags::frost_aoe(), 0.15, 1.575).with_delay(2.0),
        ])""",
    },
    "Glacial Colossus": {
        "bonuses": 'vec![MAJOR_VULNERABILITY.clone().with_duration(12.0)]',
        "damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_aoe(), 0.15495, 1.62698),
            HitDamage::new(DamageFlags::frost_aoe(), 0.15495, 1.62698).with_delay(1.0),
            HitDamage::new(DamageFlags::frost_aoe(), 0.15495, 1.62698).with_delay(2.0),
        ])""",
    },
    "Pestilent Colossus": {
        "bonuses": 'vec![MAJOR_VULNERABILITY.clone().with_duration(12.0)]',
        "damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::disease_aoe(), 0.15495, 1.62698),
            HitDamage::new(DamageFlags::disease_aoe(), 0.15495, 1.62698).with_delay(1.0),
            HitDamage::new(DamageFlags::disease_aoe(), 0.15495, 1.62698).with_delay(2.0),
        ])""",
    },
    "Flame Skull": {"spammable": True},
    "Venom Skull": {"spammable": True},
    "Ricochet Skull": {"spammable": True},
    "Sacrificial Bones": {
        "bonuses": """\
vec![BonusData::new(
            "Sacrificial Bones",
            BonusSource::Buff,
            BonusTrigger::AbilitySlotted,
            BonusValue::new("Sacrificial Bones", BonusTarget::Damage, 0.15),
        )]""",
    },
    "Blighted Blastbones": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::disease_aoe(), 0.12, 1.26).with_delay(2.5),
        ])"""},
    "Grave Lord's Sacrifice": {
        "bonuses": """\
vec![BonusData::new(
            "Grave Lord's Sacrifice",
            BonusSource::Buff,
            BonusTrigger::AbilitySlotted,
            BonusValue::new("Grave Lord's Sacrifice", BonusTarget::Damage, 0.15),
        )]""",
    },
    "Boneyard": {
        "bonuses": """\
vec![BonusData::new(
            "Boneyard",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Boneyard Corpse", BonusTarget::WeaponAndSpellDamageFlat, 300.0),
        )
        .with_duration(10.0)]""",
    },
    "Avid Boneyard": {
        "bonuses": """\
vec![BonusData::new(
            "Avid Boneyard",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Avid Boneyard Corpse", BonusTarget::WeaponAndSpellDamageFlat, 300.0),
        )
        .with_duration(10.0)]""",
    },
    "Unnerving Boneyard": {
        "bonuses": """\
vec![
            BonusData::new(
                "Unnerving Boneyard",
                BonusSource::Skill,
                BonusTrigger::Cast,
                BonusValue::new("Unnerving Boneyard Corpse", BonusTarget::WeaponAndSpellDamageFlat, 300.0),
            )
            .with_duration(10.0),
            MAJOR_BREACH.clone(),
        ]""",
    },
    "Skeletal Mage": {
        "bonuses": """\
vec![BonusData::new(
            "Skeletal Mage",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Mage Corpse", BonusTarget::WeaponAndSpellDamageFlat, 300.0),
        )
        .with_duration(20.0)]""",
    },
    "Skeletal Arcanist": {
        "bonuses": """\
vec![BonusData::new(
            "Skeletal Arcanist",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Arcanist Corpse", BonusTarget::WeaponAndSpellDamageFlat, 300.0),
        )
        .with_duration(20.0)]""",
    },
    "Skeletal Archer": {
        "bonuses": """\
vec![BonusData::new(
            "Skeletal Archer",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Archer Corpse", BonusTarget::WeaponAndSpellDamageFlat, 300.0),
        )
        .with_duration(20.0)]""",
        "damage": """\
SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::physical_single(),
            0.02066,
            0.21693,
        )
        .with_interval(2.0)
        .with_increase_per_tick(0.15)])""",
    },
    "Shocking Siphon": {
        "bonuses": """\
vec![
            BonusData::new(
                "Shocking Siphon",
                BonusSource::Buff,
                BonusTrigger::AbilitySlotted,
                BonusValue::new("Shocking Siphon", BonusTarget::Damage, 0.03),
            ),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]""",
    },
    "Mystic Siphon": {
        "bonuses": """\
vec![
            BonusData::new(
                "Mystic Siphon",
                BonusSource::Buff,
                BonusTrigger::AbilitySlotted,
                BonusValue::new("Mystic Siphon", BonusTarget::Damage, 0.03),
            ),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]""",
    },
    "Detonating Siphon": {
        "bonuses": """\
vec![
            BonusData::new(
                "Detonating Siphon",
                BonusSource::Buff,
                BonusTrigger::AbilitySlotted,
                BonusValue::new("Detonating Siphon", BonusTarget::Damage, 0.03),
            ),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]""",
        "damage": """\
SkillDamage::new()
                .with_dots(vec![DotDamage::new(
                    12.0,
                    DamageFlags::shock_aoe(),
                    0.015495,
                    0.162697,
                )
                .with_interval(2.0)])
                .with_hits(vec![HitDamage::new(
                    DamageFlags::shock_aoe(),
                    0.05165,
                    0.542325,
                )
                .with_delay(12.0)])""",
    },
    "Empowering Grasp": {"bonuses": 'vec![EMPOWER.clone().with_duration(5.0)]'},

    # ---- Warden ----
    "Dive": {"spammable": True},
    "Screaming Cliff Racer": {"spammable": True},
    "Cutting Dive": {"spammable": True},
    "Scorch": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.15, 1.575).with_delay(3.0),
        ])"""},
    "Deep Fissure": {
        "bonuses": """\
vec![
            MAJOR_BREACH.clone(),
            MINOR_BREACH.clone(),
        ]""",
        "damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.15495, 1.62698).with_delay(3.0),
        ])""",
    },
    "Subterranean Assault": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::poison_aoe(), 0.15495, 1.62698).with_delay(3.0),
        ])"""},
    "Swarm": {"bonuses": 'vec![MINOR_VULNERABILITY.clone()]'},
    "Fetcher Infection": {"bonuses": 'vec![MINOR_VULNERABILITY.clone()]'},
    "Growing Swarm": {"bonuses": 'vec![MINOR_VULNERABILITY.clone()]'},
    "Betty Netch": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]"""},
    "Blue Betty": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]"""},
    "Bull Netch": {"bonuses": """\
vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]"""},
    "Bird of Prey": {"bonuses": """\
vec![MINOR_BERSERK.clone().with_trigger(BonusTrigger::AbilitySlotted)]"""},
    "Lotus Flower": {"bonuses": """\
vec![
            MAJOR_PROPHECY.clone(),
            MAJOR_SAVAGERY.clone(),
        ]"""},
    "Green Lotus": {"bonuses": """\
vec![
            MAJOR_PROPHECY.clone(),
            MAJOR_SAVAGERY.clone(),
        ]"""},
    "Lotus Blossom": {"bonuses": """\
vec![
            MAJOR_PROPHECY.clone().with_duration(60.0),
            MAJOR_SAVAGERY.clone().with_duration(60.0),
        ]"""},
    "Frozen Gate": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_aoe(), 0.075, 0.7875).with_delay(3.0),
        ])"""},
    "Frozen Device": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_aoe(), 0.077475, 0.813488).with_delay(3.0),
        ])"""},
    "Frozen Retreat": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_aoe(), 0.077475, 0.813488).with_delay(3.0),
        ])"""},

    # ---- Arcanist ----
    "The Languid Eye": {"damage": """\
SkillDamage::new().with_dots(vec![DotDamage::new(
            10.0,
            DamageFlags::magic_aoe(),
            0.055222,
            0.57983,
        )
        .with_interval(1.0)
        .with_increase_per_tick(0.05)])"""},
    "Runeblades": {"spammable": True},
    "Escalating Runeblades": {"spammable": True},
    "Writhing Runeblades": {"spammable": True},
    "Fatecarver": {"channel_time": 4.0},
    "Exhausting Fatecarver": {"channel_time": 4.0},
    "Pragmatic Fatecarver": {"channel_time": 4.0},
    "Abyssal Impact": {
        "spammable": True,
        "bonuses": """\
vec![BonusData::new(
            "Abyssal Ink",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Abyssal Ink", BonusTarget::EnemyDamageTaken, 0.05),
        )
        .with_duration(15.0)]""",
    },
    "Cephaliarch's Flail": {
        "spammable": True,
        "bonuses": """\
vec![BonusData::new(
            "Abyssal Ink",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Abyssal Ink", BonusTarget::EnemyDamageTaken, 0.05),
        )
        .with_duration(15.0)]""",
    },
    "Tentacular Dread": {
        "spammable": True,
        "bonuses": """\
vec![BonusData::new(
            "Abyssal Ink",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Abyssal Ink", BonusTarget::EnemyDamageTaken, 0.05),
        )
        .with_duration(15.0)]""",
    },
    "Tome-Bearer's Inspiration": {
        "bonuses": """\
vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]""",
    },
    "Inspired Scholarship": {
        "bonuses": """\
vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]""",
    },
    "Recuperative Treatise": {
        "bonuses": """\
vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]""",
    },
    "Fulminating Rune": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.12912, 1.35581).with_delay(2.0),
        ])"""},
    "Runic Jolt": {"spammable": True},
    "Runic Embrace": {"spammable": True},
    "Runic Sunder": {
        "spammable": True,
        "bonuses": """\
vec![BonusData::new(
            "Runic Sunder Debuff",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new(
                "Runic Sunder Debuff",
                BonusTarget::EnemyResistanceReduction,
                2200.0,
            ),
        )
        .with_duration(15.0)]""",
    },
    "Fatewoven Armor": {"bonuses": """\
vec![
            MAJOR_PROPHECY.clone().with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY.clone().with_trigger(BonusTrigger::AbilitySlotted),
        ]"""},
    "Cruxweaver Armor": {"bonuses": """\
vec![
            MAJOR_PROPHECY.clone().with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY.clone().with_trigger(BonusTrigger::AbilitySlotted),
        ]"""},
    "Unbreakable Fate": {"bonuses": """\
vec![
            MAJOR_PROPHECY.clone().with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY.clone().with_trigger(BonusTrigger::AbilitySlotted),
        ]"""},
    "Remedy Cascade": {"channel_time": 4.5},
    "Cascading Fortune": {"channel_time": 4.5},
    "Curative Surge": {"channel_time": 4.5},

    # ---- Bow ----
    "Rapid Fire": {"channel_time": 4.0},
    "Toxic Barrage": {
        "channel_time": 4.0,
        "damage": """\
SkillDamage::new().with_dots(vec![
            DotDamage::new(4.0, DamageFlags::poison_single(), 0.0860872, 0.904026),
            DotDamage::new(8.0, DamageFlags::poison_single(), 0.086083, 0.903875).with_delay(1.0),
        ])""",
    },
    "Focused Aim": {"bonuses": 'vec![MAJOR_BREACH.clone()]'},
    "Volley": {"damage": """\
SkillDamage::new().with_dots(vec![DotDamage::new(
            8.0,
            DamageFlags::physical_aoe(),
            0.014815,
            0.155556,
        )
        .with_delay(2.0)
        .with_interval(1.0)])"""},
    "Arrow Barrage": {"damage": """\
SkillDamage::new().with_dots(vec![DotDamage::new(
            8.0,
            DamageFlags::physical_aoe(),
            0.019895,
            0.208896,
        )
        .with_delay(2.0)
        .with_interval(1.0)])"""},
    "Endless Hail": {"damage": """\
SkillDamage::new().with_dots(vec![DotDamage::new(
            13.0,
            DamageFlags::physical_aoe(),
            0.015304,
            0.160689,
        )
        .with_delay(2.0)
        .with_interval(1.0)])"""},
    "Thunderous Volley": {"damage": """\
SkillDamage::new().with_dots(vec![
            DotDamage::new(13.0, DamageFlags::physical_aoe(), 0.015304, 0.160689)
                .with_delay(2.0)
                .with_interval(1.0),
            DotDamage::new(13.0, DamageFlags::physical_aoe(), 0.0, 0.095636)
                .with_delay(2.0)
                .with_interval(1.0)
                .with_flat_increase_per_tick(191.0)
                .ignores_modifier(),
        ])"""},
    "Poison Injection": {"execute": (1.2, 0.50, "Linear")},
    "Venom Arrow": {"bonuses": 'vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]'},
    "Trapping Webs": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_aoe(), 0.075, 0.7875),
            HitDamage::new(DamageFlags::poison_aoe(), 0.1, 1.05).with_delay(10.0),
        ])"""},
    "Tangling Webs": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488),
            HitDamage::new(DamageFlags::poison_aoe(), 0.1033, 1.08465).with_delay(10.0),
        ])"""},
    "Shadow Silk": {"damage": """\
SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488),
            HitDamage::new(DamageFlags::poison_aoe(), 0.1033, 1.08465).with_delay(10.0),
        ])"""},

    # ---- Destruction Staff ----
    "Force Shock": {"spammable": True},
    "Crushing Shock": {"spammable": True},
    "Force Pulse": {"spammable": True},
    "Unstable Wall of Elements": {"damage": """\
SkillDamage::new()
                .with_dots(vec![DotDamage::new(
                    8.0,
                    DamageFlags::magic_aoe(),
                    0.012521,
                    0.131473,
                )
                .with_interval(1.0)])
                .with_hits(vec![HitDamage::new(
                    DamageFlags::magic_aoe(),
                    0.05165,
                    0.542325,
                )
                .with_delay(8.0)])"""},
    "Weakness to Elements": {"bonuses": 'vec![MAJOR_BREACH.clone().with_duration(30.0)]'},
    "Elemental Drain": {"bonuses": 'vec![MAJOR_BREACH.clone().with_duration(60.0)]'},
    "Elemental Susceptibility": {"bonuses": 'vec![MAJOR_BREACH.clone().with_duration(30.0)]'},
    "Impulse": {"spammable": True},
    "Elemental Ring": {"spammable": True},
    "Pulsar": {"spammable": True},

    # ---- Dual Wield ----
    "Flurry": {"spammable": True},
    "Bloodthirst": {"spammable": True},
    "Rapid Strikes": {"spammable": True},
    "Whirlwind": {"spammable": True, "execute": (0.33, 0.50, "Linear")},
    "Steel Tornado": {"spammable": True, "execute": (0.33, 0.50, "Linear")},
    "Whirling Blades": {"spammable": True, "execute": (1.0, 0.50, "Linear")},
    "Hidden Blade": {"bonuses": 'vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]'},
    "Flying Blade": {
        "bonuses": """\
vec![
            MAJOR_BRUTALITY.clone().with_duration(40.0),
            MAJOR_SORCERY.clone().with_duration(40.0),
        ]""",
    },
    "Shrouded Daggers": {"bonuses": 'vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]'},

    # ---- Two Handed ----
    "Uppercut": {"spammable": True},
    "Dizzying Swing": {"spammable": True},
    "Wrecking Blow": {
        "spammable": True,
        "bonuses": """\
vec![
            EMPOWER.clone().with_duration(3.0),
            MAJOR_BERSERK.clone().with_duration(3.0),
        ]""",
    },
    "Reverse Slash": {"spammable": True, "execute": (3.0, 0.50, "Linear")},
    "Executioner": {"spammable": True, "execute": (4.0, 0.50, "Linear")},
    "Reverse Slice": {"spammable": True, "execute": (3.0, 0.50, "Linear")},
    "Momentum": {"bonuses": 'vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]'},
    "Forward Momentum": {
        "bonuses": """\
vec![
            MAJOR_BRUTALITY.clone().with_duration(40.0),
            MAJOR_SORCERY.clone().with_duration(40.0),
        ]""",
    },
    "Rally": {"bonuses": 'vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]'},

    # ---- Fighters Guild ----
    "Flawless Dawnbreaker": {
        "bonuses": """\
vec![BonusData::new(
                "Flawless Dawnbreaker",
                BonusSource::Buff,
                BonusTrigger::Cast,
                BonusValue::new(
                    "Flawless Dawnbreaker",
                    BonusTarget::WeaponAndSpellDamageFlat,
                    300.0,
                ),
            )
            .with_duration(20.0)]""",
    },
    "Trap Beast": {"bonuses": 'vec![MINOR_FORCE.clone()]'},
    "Barbed Trap": {"bonuses": 'vec![MINOR_FORCE.clone()]'},
    "Lightweight Beast Trap": {"bonuses": 'vec![MINOR_FORCE.clone()]'},
    "Expert Hunter": {
        "bonuses": """\
vec![
            MAJOR_SAVAGERY.clone().with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(BonusTrigger::AbilitySlotted),
        ]""",
    },
    "Evil Hunter": {
        "bonuses": """\
vec![
            MAJOR_SAVAGERY.clone().with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(BonusTrigger::AbilitySlotted),
        ]""",
    },
    "Camouflaged Hunter": {
        "bonuses": """\
vec![
            MAJOR_SAVAGERY.clone().with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(BonusTrigger::AbilitySlotted),
            MINOR_BERSERK.clone().with_duration(5.0),
        ]""",
    },

    # ---- Mages Guild ----
    "Degeneration": {"bonuses": 'vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]'},
    "Magelight": {
        "bonuses": """\
vec![
            MAJOR_SAVAGERY.clone().with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(BonusTrigger::AbilitySlotted),
        ]""",
    },
    "Inner Light": {
        "bonuses": """\
vec![
            MAJOR_SAVAGERY.clone().with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(BonusTrigger::AbilitySlotted),
        ]""",
    },
    "Radiant Magelight": {
        "bonuses": """\
vec![
            MAJOR_SAVAGERY.clone().with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(BonusTrigger::AbilitySlotted),
        ]""",
    },

    # ---- Psijic Order ----
    "Imbue Weapon": {"spammable": True},
    "Elemental Weapon": {"spammable": True},
    "Crushing Weapon": {"spammable": True, "bonuses": 'vec![MAJOR_BREACH.clone().with_duration(5.0)]'},
    "Accelerate": {"bonuses": 'vec![MINOR_FORCE.clone()]'},
    "Channeled Acceleration": {"bonuses": 'vec![MINOR_FORCE.clone().with_duration(60.0)]'},
    "Race Against Time": {"bonuses": 'vec![MINOR_FORCE.clone()]'},

    # ---- Undaunted ----
    "Inner Beast": {"bonuses": 'vec![MINOR_VULNERABILITY.clone().with_duration(15.0)]'},
}


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def escape_rust_string(s: str) -> str:
    """Escape a string for use in Rust string literals."""
    return s.replace("\\", "\\\\").replace('"', '\\"')


def build_flags(tags: list[str]) -> str:
    """Convert metadata tags to DamageFlags expression."""
    flags = []
    for tag in tags:
        if tag in TAG_TO_FLAG:
            flags.append(f"DamageFlags::{TAG_TO_FLAG[tag]}")
    if not flags:
        return "DamageFlags::empty()"
    return " | ".join(flags)


def parse_dot_duration(entry: dict, eq: dict) -> float | None:
    """Try to determine DOT duration from metadata, description, or overrides."""
    name = entry["name"].removesuffix(" 4")

    # 1. Check overrides
    if name in DOT_DURATION_OVERRIDES:
        return DOT_DURATION_OVERRIDES[name]

    # 2. Check metadata
    dur = eq["metadata"].get("duration", -0.001)
    if dur > 0:
        return dur

    # 3. Parse description: find "for <<N>>" or "over <<N>>"
    desc = entry.get("description", "")
    for pattern in [r'for <<(\d+)>>', r'over <<(\d+)>>']:
        for m in re.finditer(pattern, desc):
            idx = int(m.group(1))
            for ceq in entry["equations"]:
                if ceq.get("index") == idx and ceq.get("type") == "constant":
                    val = ceq["value"]
                    sec_match = re.search(r'([\d.]+)\s*second', val)
                    if sec_match:
                        return float(sec_match.group(1))

    return None


def generate_damage_auto(entry: dict) -> str | None:
    """Auto-generate SkillDamage code from JSON equations. Returns None if no damage."""
    hits = []
    dots = []

    for eq in entry.get("equations", []):
        if eq.get("type") != "scaling":
            continue
        meta = eq.get("metadata", {})
        tags = meta.get("tags", [])
        if "Dmg" not in tags:
            continue

        coef_a = eq["terms"][0]["coefficient"] if len(eq["terms"]) > 0 else 0.0
        coef_b = eq["terms"][1]["coefficient"] if len(eq["terms"]) > 1 else 0.0
        flags = build_flags(tags)

        if "DOT" in tags:
            duration = parse_dot_duration(entry, eq)
            if duration is None:
                duration = 10.0  # fallback
            tick = meta.get("tick", -0.001)

            dot_code = f"DotDamage::new(\n"
            dot_code += f"            {duration:.1f},\n"
            dot_code += f"            {flags},\n"
            dot_code += f"            {coef_a},\n"
            dot_code += f"            {coef_b},\n"
            dot_code += f"        )"
            if tick > 0:
                dot_code += f".with_interval({tick:.1f})"
            dots.append(dot_code)
        elif "Direct" in tags:
            hit_code = f"HitDamage::new(\n"
            hit_code += f"            {flags},\n"
            hit_code += f"            {coef_a},\n"
            hit_code += f"            {coef_b},\n"
            hit_code += f"        )"
            hits.append(hit_code)

    if not hits and not dots:
        return None

    parts = ["SkillDamage::new()"]
    if hits:
        if len(hits) == 1:
            parts.append(f".with_hits(vec![{hits[0]}])")
        else:
            inner = ",\n        ".join(hits)
            parts.append(f".with_hits(vec![\n        {inner},\n        ])")
    if dots:
        if len(dots) == 1:
            parts.append(f".with_dots(vec![{dots[0]}])")
        else:
            inner = ",\n        ".join(dots)
            parts.append(f".with_dots(vec![\n        {inner},\n        ])")

    return "\n            ".join(parts)


def generate_skill(entry: dict, skill_line_variant: str, class_variant: str) -> str:
    """Generate Rust code for a single SkillData."""
    name = entry["name"].removesuffix(" 4")
    escaped_name = escape_rust_string(name)
    base_name = BASE_NAMES.get(name, name)
    escaped_base = escape_rust_string(base_name)
    skill_id = entry["id"]
    resource = MECHANIC_MAP.get(entry.get("mechanic", ""), "Magicka")
    overrides = SKILL_OVERRIDES.get(name, {})

    # Build the chain
    parts = [
        f'        SkillData::new(\n'
        f'            "{escaped_name}",\n'
        f'            "{escaped_base}",\n'
        f'            ClassName::{class_variant},\n'
        f'            SkillLineName::{skill_line_variant},\n'
        f'            Resource::{resource},\n'
        f'        )\n'
        f'        .with_skill_id({skill_id})'
    ]

    # Damage
    if "damage" in overrides:
        parts.append(f'        .with_damage(\n            {overrides["damage"]})')
    else:
        damage_code = generate_damage_auto(entry)
        if damage_code:
            parts.append(f"        .with_damage(\n            {damage_code})")

    # Spammable
    if overrides.get("spammable"):
        parts.append("        .with_spammable()")

    # Channel time
    if "channel_time" in overrides:
        parts.append(f"        .with_channel_time({overrides['channel_time']})")

    # Execute
    if "execute" in overrides:
        mult, thresh, scaling = overrides["execute"]
        parts.append(f"        .with_execute({mult}, {thresh}, ExecuteScaling::{scaling})")

    # Proc light attacks
    if "proc_light_attacks" in overrides:
        parts.append(f"        .with_proc_light_attacks({overrides['proc_light_attacks']})")

    # Bonuses
    if "bonuses" in overrides:
        parts.append(f'        .with_bonuses({overrides["bonuses"]})')

    return "\n".join(parts)


# ---------------------------------------------------------------------------
# Import detection
# ---------------------------------------------------------------------------

# All bonus constants that might be referenced
ALL_BONUS_CONSTANTS = {
    "EMPOWER", "MAJOR_BERSERK", "MAJOR_BREACH", "MAJOR_BRUTALITY",
    "MAJOR_PROPHECY", "MAJOR_SAVAGERY", "MAJOR_SORCERY", "MAJOR_VULNERABILITY",
    "MINOR_BERSERK", "MINOR_BREACH", "MINOR_BRUTALITY", "MINOR_FORCE",
    "MINOR_PROPHECY", "MINOR_SAVAGERY", "MINOR_SORCERY", "MINOR_VULNERABILITY",
}

# Constants re-exported at crate::data::bonuses level
REEXPORTED_BONUSES = {
    "EMPOWER", "MAJOR_BERSERK", "MAJOR_BREACH", "MAJOR_BRUTALITY",
    "MAJOR_PROPHECY", "MAJOR_SAVAGERY", "MAJOR_SORCERY", "MAJOR_VULNERABILITY",
    "MINOR_BERSERK", "MINOR_BREACH", "MINOR_BRUTALITY",
    "MINOR_PROPHECY", "MINOR_SAVAGERY", "MINOR_SORCERY", "MINOR_VULNERABILITY",
}

# Constants only available via crate::data::bonuses::unique
UNIQUE_ONLY_BONUSES = {"MINOR_FORCE"}


def detect_bonus_imports(code: str) -> tuple[set[str], set[str]]:
    """Detect which bonus constants are referenced in the generated code.
    Returns (reexported_set, unique_only_set)."""
    reexported = set()
    unique_only = set()
    for const in ALL_BONUS_CONSTANTS:
        if const in code:
            if const in UNIQUE_ONLY_BONUSES:
                unique_only.add(const)
            elif const in REEXPORTED_BONUSES:
                reexported.add(const)
    return reexported, unique_only


def detect_domain_imports(code: str) -> set[str]:
    """Detect which domain types are referenced in the generated code."""
    types = {"ClassName", "Resource", "SkillLineName", "SkillData"}
    if "DamageFlags::" in code:
        types.add("DamageFlags")
    if "HitDamage::" in code or "HitDamage::new" in code:
        types.add("HitDamage")
    if "DotDamage::" in code or "DotDamage::new" in code:
        types.add("DotDamage")
    if "SkillDamage::" in code or "SkillDamage::new" in code:
        types.add("SkillDamage")
    if "ExecuteScaling::" in code:
        types.add("ExecuteScaling")
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


def generate_file(skills: list[tuple[dict, str, str]], file_stem: str, class_group: str) -> str:
    """Generate a complete Rust source file."""
    # Generate all skill entries first to detect imports
    skill_entries = []
    for entry, skill_line_variant, class_variant in skills:
        skill_entries.append(generate_skill(entry, skill_line_variant, class_variant))

    skills_code = ",\n".join(skill_entries)

    # Detect imports
    reexported_bonuses, unique_bonuses = detect_bonus_imports(skills_code)
    domain_types = detect_domain_imports(skills_code)

    # Build import lines
    import_lines = []

    # Bonus imports - determine correct path based on class_group
    is_class = class_group in {
        "Templar", "Dragonknight", "Nightblade", "Sorcerer",
        "Necromancer", "Warden", "Arcanist",
    }

    all_bonus_names = sorted(reexported_bonuses | unique_bonuses)
    if all_bonus_names:
        if is_class:
            # Class skills import from crate::data::bonuses
            import_lines.append(
                f"use crate::data::bonuses::{{\n"
                f"    {', '.join(all_bonus_names)},\n"
                f"}};"
            )
        else:
            # Weapon/guild skills import from crate::data::bonuses::unique
            import_lines.append(
                f"use crate::data::bonuses::unique::{{\n"
                f"    {', '.join(all_bonus_names)},\n"
                f"}};"
            )

    # Domain imports - split into two use statements for readability
    domain_sorted = sorted(domain_types)
    import_lines.append(
        f"use crate::domain::{{\n"
        f"    {', '.join(domain_sorted)},\n"
        f"}};"
    )
    import_lines.append("use once_cell::sync::Lazy;")

    imports = "\n".join(import_lines)

    # Static name
    static_name = f"{file_stem.upper()}_SKILLS"

    return (
        f"// Auto-generated by datamine/generate_skills_rs.py — do not edit manually.\n"
        f"// Manual overrides (bonuses, execute, etc.) stored in generator script.\n"
        f"{imports}\n"
        f"\n"
        f"pub static {static_name}: Lazy<Vec<SkillData>> = Lazy::new(|| {{\n"
        f"    vec![\n"
        f"{skills_code},\n"
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

    # Filter to version 4 skills in relevant skill lines
    v4 = [
        e for e in data
        if e["name"].endswith(" 4")
        and e.get("skill_line") in RELEVANT_SKILL_LINES
    ]

    # Exclude element-specific destruction staff variants
    v4 = [
        e for e in v4
        if not (
            e.get("skill_line") == "Destruction Staff"
            and e["name"].removesuffix(" 4") in DESTRO_ELEMENT_VARIANTS
        )
    ]

    # Group by output file
    files: dict[str, list[tuple[dict, str, str]]] = {}
    for entry in v4:
        skill_line = entry["skill_line"]
        if skill_line not in SKILL_LINE_MAP:
            continue

        sl_variant, class_group, dir_path, file_stem = SKILL_LINE_MAP[skill_line]
        # Determine ClassName variant
        class_variant = class_group

        key = f"{dir_path}/{file_stem}"
        if key not in files:
            files[key] = []
        files[key].append((entry, sl_variant, class_variant))

    # Sort skills within each file by skill_line then name
    for key in files:
        files[key].sort(key=lambda x: (x[0]["skill_line"], x[0]["name"]))

    # Generate output files
    for key, skills in sorted(files.items()):
        dir_path, file_stem = key.rsplit("/", 1)
        # Determine class_group from first skill
        _, class_group, _, _ = SKILL_LINE_MAP[skills[0][0]["skill_line"]]

        code = generate_file(skills, file_stem, class_group)
        out_path = output_base / dir_path / f"{file_stem}_skills.rs"
        out_path.write_text(code, encoding="utf-8")
        print(f"Generated {len(skills)} skills -> {out_path}")


if __name__ == "__main__":
    main()
