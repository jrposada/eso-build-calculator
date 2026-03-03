"""Generate Rust set data files from parsed sets.json.

Reads datamine/parsed/sets.json and generates:
  - src/data/sets/normal.rs
  - src/data/sets/monster.rs
  - src/data/sets/mythic.rs
  - src/data/sets/arena.rs

Each file exports `pub static {CATEGORY}_SETS: Lazy<Vec<SetData>>`.
Proc effects are embedded inline via .with_proc_effects().
"""

from __future__ import annotations

import json
from pathlib import Path

# Map JSON set types to Rust SetType + output category
TYPE_TO_CATEGORY = {
    "Dungeon": ("Normal", "normal"),
    "Overland": ("Normal", "normal"),
    "Trial": ("Normal", "normal"),
    "PVP": ("Normal", "normal"),
    "Crafted": ("Normal", "normal"),
    "Class": ("Normal", "normal"),
    "Other": ("Normal", "normal"),
    None: ("Normal", "normal"),
    "Monster": ("Monster", "monster"),
    "Mythic": ("Mythic", "mythic"),
    "Arena": ("Arena", "arena"),
}

# Map parsed stat names to BonusTarget variants (DPS-relevant only)
STAT_MAP = {
    "Maximum Stamina": ("MaxStaminaFlat", "Max Stamina"),
    "Maximum Magicka": ("MaxMagickaFlat", "Max Magicka"),
    "Weapon and Spell Damage": ("WeaponAndSpellDamageFlat", "Weapon and Spell Damage"),
    "Critical Chance": ("CriticalRating", "Critical Chance"),
    "Offensive Penetration": ("PhysicalAndSpellPenetration", "Offensive Penetration"),
}

# Proc effects keyed by set name -> (piece_count, raw Rust vec![...] code)
PROC_EFFECTS: dict[str, tuple[int, str]] = {
    "Ansuul's Torment": (5, """\
vec![
                    SetProcEffect {
                        name: "Ansuul's Storm Bolt".to_string(),
                        trigger: SetProcTrigger::OnLightAttack,
                        action: SetProcAction::DamageProc {
                            hit_damage: 1471.0,
                            hit_flags: DamageFlags::SHOCK | DamageFlags::SINGLE_TARGET | DamageFlags::DIRECT,
                            dot_total_damage: 0.0,
                            dot_duration: 0.0,
                            dot_flags: DamageFlags::empty(),
                        },
                        cooldown: 0.0,
                    },
                    SetProcEffect {
                        name: "Ansuul's Fury".to_string(),
                        trigger: SetProcTrigger::OnLightAttack,
                        action: SetProcAction::StackingBuff {
                            per_stack_target: Some(BonusTarget::CriticalDamage),
                            per_stack_value: 0.05,
                            max_stacks: 4,
                            stack_duration: 5.0,
                            stack_cooldown: 0.0,
                            at_max_buff_name: String::new(),
                            at_max_buff_target: BonusTarget::CriticalDamage,
                            at_max_buff_value: 0.0,
                            at_max_buff_duration: 0.0,
                        },
                        cooldown: 0.0,
                    },
                ]"""),
    "Arms of Relequen": (5, """\
vec![SetProcEffect {
                    name: "Harmful Winds".to_string(),
                    trigger: SetProcTrigger::OnLightAttack,
                    action: SetProcAction::StackingDot {
                        damage_per_stack_per_tick: 51.0,
                        tick_interval: 2.0,
                        max_stacks: 10,
                        stack_duration: 5.0,
                        stack_cooldown: 1.0,
                        flags: DamageFlags::PHYSICAL | DamageFlags::SINGLE_TARGET | DamageFlags::DOT,
                    },
                    cooldown: 0.0,
                }]"""),
    "Bahsei's Mania": (5, """\
vec![SetProcEffect {
                    name: "Bahsei's Mania".to_string(),
                    trigger: SetProcTrigger::OnDealDamage,
                    action: SetProcAction::ResourceScalingBuff {
                        target: BonusTarget::Damage,
                        max_value: 0.15,
                        threshold_pct: None,
                    },
                    cooldown: 0.0,
                }]"""),
    "Coral Riptide": (5, """\
vec![SetProcEffect {
                    name: "Coral Riptide Minor Force".to_string(),
                    trigger: SetProcTrigger::OnDealDamage,
                    action: SetProcAction::ResourceScalingBuff {
                        target: BonusTarget::CriticalDamage,
                        max_value: 0.10,
                        threshold_pct: Some(50.0),
                    },
                    cooldown: 0.0,
                }]"""),
    "Kinras's Wrath": (5, """\
vec![SetProcEffect {
                    name: "Kinras's Wrath".to_string(),
                    trigger: SetProcTrigger::OnLightAttack,
                    action: SetProcAction::StackingBuff {
                        per_stack_target: None,
                        per_stack_value: 0.0,
                        max_stacks: 5,
                        stack_duration: 5.0,
                        stack_cooldown: 0.0,
                        at_max_buff_name: "Major Berserk".to_string(),
                        at_max_buff_target: BonusTarget::Damage,
                        at_max_buff_value: 0.10,
                        at_max_buff_duration: 5.0,
                    },
                    cooldown: 0.0,
                }]"""),
    "Perfected Ansuul's Torment": (5, """\
vec![
                    SetProcEffect {
                        name: "Perfected Ansuul's Storm Bolt".to_string(),
                        trigger: SetProcTrigger::OnLightAttack,
                        action: SetProcAction::DamageProc {
                            hit_damage: 1471.0,
                            hit_flags: DamageFlags::SHOCK | DamageFlags::SINGLE_TARGET | DamageFlags::DIRECT,
                            dot_total_damage: 0.0,
                            dot_duration: 0.0,
                            dot_flags: DamageFlags::empty(),
                        },
                        cooldown: 0.0,
                    },
                    SetProcEffect {
                        name: "Perfected Ansuul's Fury".to_string(),
                        trigger: SetProcTrigger::OnLightAttack,
                        action: SetProcAction::StackingBuff {
                            per_stack_target: Some(BonusTarget::CriticalDamage),
                            per_stack_value: 0.05,
                            max_stacks: 4,
                            stack_duration: 5.0,
                            stack_cooldown: 0.0,
                            at_max_buff_name: String::new(),
                            at_max_buff_target: BonusTarget::CriticalDamage,
                            at_max_buff_value: 0.0,
                            at_max_buff_duration: 0.0,
                        },
                        cooldown: 0.0,
                    },
                ]"""),
    "Perfected Arms of Relequen": (5, """\
vec![SetProcEffect {
                    name: "Harmful Winds".to_string(),
                    trigger: SetProcTrigger::OnLightAttack,
                    action: SetProcAction::StackingDot {
                        damage_per_stack_per_tick: 51.0,
                        tick_interval: 2.0,
                        max_stacks: 10,
                        stack_duration: 5.0,
                        stack_cooldown: 1.0,
                        flags: DamageFlags::PHYSICAL | DamageFlags::SINGLE_TARGET | DamageFlags::DOT,
                    },
                    cooldown: 0.0,
                }]"""),
    "Perfected Bahsei's Mania": (5, """\
vec![SetProcEffect {
                    name: "Perfected Bahsei's Mania".to_string(),
                    trigger: SetProcTrigger::OnDealDamage,
                    action: SetProcAction::ResourceScalingBuff {
                        target: BonusTarget::Damage,
                        max_value: 0.15,
                        threshold_pct: None,
                    },
                    cooldown: 0.0,
                }]"""),
    "Perfected Coral Riptide": (5, """\
vec![SetProcEffect {
                    name: "Perfected Coral Riptide Minor Force".to_string(),
                    trigger: SetProcTrigger::OnDealDamage,
                    action: SetProcAction::ResourceScalingBuff {
                        target: BonusTarget::CriticalDamage,
                        max_value: 0.10,
                        threshold_pct: Some(50.0),
                    },
                    cooldown: 0.0,
                }]"""),
    "Perfected Whorl of the Depths": (5, """\
vec![SetProcEffect {
                    name: "Perfected Whorl of the Depths".to_string(),
                    trigger: SetProcTrigger::OnDirectDamage,
                    action: SetProcAction::DamageProc {
                        hit_damage: 1025.0,
                        hit_flags: DamageFlags::FROST | DamageFlags::AOE | DamageFlags::DIRECT,
                        dot_total_damage: 7175.0,
                        dot_duration: 6.0,
                        dot_flags: DamageFlags::FROST | DamageFlags::AOE | DamageFlags::DOT,
                    },
                    cooldown: 6.0,
                }]"""),
    "Pillar of Nirn": (5, """\
vec![SetProcEffect {
                    name: "Pillar of Nirn Fissure".to_string(),
                    trigger: SetProcTrigger::OnDealDamage,
                    action: SetProcAction::DamageProc {
                        hit_damage: 803.0,
                        hit_flags: DamageFlags::BLEED | DamageFlags::SINGLE_TARGET | DamageFlags::DIRECT,
                        dot_total_damage: 2405.0,
                        dot_duration: 10.0,
                        dot_flags: DamageFlags::BLEED | DamageFlags::SINGLE_TARGET | DamageFlags::DOT,
                    },
                    cooldown: 10.0,
                }]"""),
    "Tzogvin's Warband": (5, """\
vec![SetProcEffect {
                    name: "Tzogvin's Warband".to_string(),
                    trigger: SetProcTrigger::OnDirectDamage,
                    action: SetProcAction::StackingBuff {
                        per_stack_target: Some(BonusTarget::CriticalRating),
                        per_stack_value: 177.0,
                        max_stacks: 10,
                        stack_duration: 5.0,
                        stack_cooldown: 0.5,
                        at_max_buff_name: "Minor Force".to_string(),
                        at_max_buff_target: BonusTarget::CriticalDamage,
                        at_max_buff_value: 0.10,
                        at_max_buff_duration: 5.0,
                    },
                    cooldown: 0.0,
                }]"""),
    "Whorl of the Depths": (5, """\
vec![SetProcEffect {
                    name: "Whorl of the Depths".to_string(),
                    trigger: SetProcTrigger::OnDirectDamage,
                    action: SetProcAction::DamageProc {
                        hit_damage: 1025.0,
                        hit_flags: DamageFlags::FROST | DamageFlags::AOE | DamageFlags::DIRECT,
                        dot_total_damage: 7175.0,
                        dot_duration: 6.0,
                        dot_flags: DamageFlags::FROST | DamageFlags::AOE | DamageFlags::DOT,
                    },
                    cooldown: 6.0,
                }]"""),
    # Monster sets
    "Kjalnar's Nightmare": (2, """\
vec![SetProcEffect {
                    name: "Bone Colossus".to_string(),
                    trigger: SetProcTrigger::OnLightAttack,
                    action: SetProcAction::DamageProc {
                        hit_damage: 8000.0,
                        hit_flags: DamageFlags::PHYSICAL | DamageFlags::AOE | DamageFlags::DIRECT,
                        dot_total_damage: 0.0,
                        dot_duration: 0.0,
                        dot_flags: DamageFlags::empty(),
                    },
                    cooldown: 10.0,
                }]"""),
    "Stormfist": (2, """\
vec![SetProcEffect {
                    name: "Stormfist Lightning".to_string(),
                    trigger: SetProcTrigger::OnDealDamage,
                    action: SetProcAction::DamageProc {
                        hit_damage: 2025.0,
                        hit_flags: DamageFlags::SHOCK | DamageFlags::AOE | DamageFlags::DIRECT,
                        dot_total_damage: 0.0,
                        dot_duration: 0.0,
                        dot_flags: DamageFlags::empty(),
                    },
                    cooldown: 8.0,
                }]"""),
    "Zaan": (2, """\
vec![SetProcEffect {
                    name: "Zaan Fire Beam".to_string(),
                    trigger: SetProcTrigger::OnDealDamage,
                    action: SetProcAction::DamageProc {
                        hit_damage: 0.0,
                        hit_flags: DamageFlags::empty(),
                        dot_total_damage: 9000.0,
                        dot_duration: 6.0,
                        dot_flags: DamageFlags::FLAME | DamageFlags::SINGLE_TARGET | DamageFlags::DOT,
                    },
                    cooldown: 18.0,
                }]"""),
    # Mythic sets
    "Belharza's Band": (1, """\
vec![
                    SetProcEffect {
                        name: "Belharza's Band LA Bonus".to_string(),
                        trigger: SetProcTrigger::OnLightAttack,
                        action: SetProcAction::FlatLightAttackBonus { value: 900.0 },
                        cooldown: 0.0,
                    },
                    SetProcEffect {
                        name: "Belharza's Band Proc".to_string(),
                        trigger: SetProcTrigger::OnLightAttack,
                        action: SetProcAction::DamageProc {
                            hit_damage: 1471.0,
                            hit_flags: DamageFlags::PHYSICAL | DamageFlags::SINGLE_TARGET | DamageFlags::DIRECT,
                            dot_total_damage: 0.0,
                            dot_duration: 0.0,
                            dot_flags: DamageFlags::empty(),
                        },
                        cooldown: 10.0,
                    },
                ]"""),
}


def escape_rust_string(s: str) -> str:
    """Escape a string for use in Rust string literals."""
    return s.replace("\\", "\\\\").replace('"', '\\"')


def generate_bonus(set_name: str, bonus: dict) -> str | None:
    """Generate Rust code for a single bonus, or None if not DPS-relevant."""
    if bonus.get("type") != "stat":
        return None

    stat = bonus["stat"]
    if stat not in STAT_MAP:
        return None

    target_variant, display_name = STAT_MAP[stat]
    value = float(bonus["value_max"])
    pieces = bonus["pieces"]

    escaped_name = escape_rust_string(set_name)

    return (
        f'BonusData::new(\n'
        f'                    "{escaped_name} {pieces}pc",\n'
        f'                    BonusSource::GearSet,\n'
        f'                    BonusTrigger::Passive,\n'
        f'                    BonusValue::new("{display_name}", BonusTarget::{target_variant}, {value:.1f}),\n'
        f'                )'
    )


def generate_set(entry: dict, rust_type: str) -> str:
    """Generate Rust code for a single SetData."""
    name = entry["name"]
    escaped_name = escape_rust_string(name)
    item_slots = entry.get("item_slots", [])

    # Group bonuses by piece count
    bonuses_by_piece: dict[int, list[str]] = {}
    for bonus in entry.get("bonuses", []):
        code = generate_bonus(name, bonus)
        if code is not None:
            pc = bonus["pieces"]
            bonuses_by_piece.setdefault(pc, []).append(code)

    # Build the chain
    parts = [f'        SetData::new("{escaped_name}", SetType::{rust_type})']

    # Add item_slots
    if item_slots:
        slot_strs = ", ".join(f'"{s}"' for s in item_slots)
        parts.append(f"            .with_item_slots(vec![{slot_strs}])")

    # Add thresholds
    for pc in sorted(bonuses_by_piece.keys()):
        bonus_codes = bonuses_by_piece[pc]
        if len(bonus_codes) == 1:
            parts.append(
                f"            .with_threshold(\n"
                f"                {pc},\n"
                f"                vec![{bonus_codes[0]}],\n"
                f"            )"
            )
        else:
            inner = ",\n                ".join(bonus_codes)
            parts.append(
                f"            .with_threshold(\n"
                f"                {pc},\n"
                f"                vec![\n"
                f"                    {inner},\n"
                f"                ],\n"
                f"            )"
            )

    # Add proc effects if this set has any
    if name in PROC_EFFECTS:
        pc, effects_code = PROC_EFFECTS[name]
        parts.append(
            f"            .with_proc_effects(\n"
            f"                {pc},\n"
            f"                {effects_code},\n"
            f"            )"
        )

    return "\n".join(parts)


def generate_file(sets: list[dict], rust_type: str, category: str) -> str:
    """Generate a complete Rust source file for a category."""
    # Determine which imports we need
    has_bonuses = any(
        bonus.get("type") == "stat" and bonus["stat"] in STAT_MAP
        for entry in sets
        for bonus in entry.get("bonuses", [])
    )

    has_procs = any(entry["name"] in PROC_EFFECTS for entry in sets)

    # Check if BonusTarget is actually referenced in any proc effects for this file
    proc_code_for_file = "".join(
        PROC_EFFECTS[entry["name"]][1]
        for entry in sets
        if entry["name"] in PROC_EFFECTS
    )
    procs_use_bonus_target = "BonusTarget::" in proc_code_for_file

    # Build import list
    domain_types = []
    if has_bonuses:
        domain_types.extend(["BonusData", "BonusSource", "BonusTarget", "BonusTrigger", "BonusValue"])
    if has_procs and not has_bonuses and procs_use_bonus_target:
        # BonusTarget needed for proc effects even without stat bonuses
        domain_types.append("BonusTarget")
    if has_procs:
        domain_types.append("DamageFlags")
    domain_types.extend(["SetData", "SetType"])
    if has_procs:
        domain_types.extend(["SetProcAction", "SetProcEffect", "SetProcTrigger"])
    domain_types.sort()

    imports = (
        "use crate::domain::{\n"
        f"    {', '.join(domain_types)},\n"
        "};\n"
        "use once_cell::sync::Lazy;"
    )

    static_name = f"{category.upper()}_SETS"

    set_entries = []
    for entry in sets:
        set_entries.append(generate_set(entry, rust_type))

    sets_code = ",\n".join(set_entries)

    return (
        f"// Auto-generated by datamine/generate_sets_rs.py — do not edit manually.\n"
        f"// Proc effects are embedded inline from the PROC_EFFECTS dict in the generator.\n"
        f"{imports}\n"
        f"\n"
        f"pub static {static_name}: Lazy<Vec<SetData>> = Lazy::new(|| {{\n"
        f"    vec![\n"
        f"{sets_code},\n"
        f"    ]\n"
        f"}});\n"
    )


def main():
    base = Path(__file__).parent
    input_path = base / "parsed" / "sets.json"
    output_dir = base.parent / "src" / "data" / "sets"

    data = json.loads(input_path.read_text(encoding="utf-8"))

    # Bucket sets by category
    categories: dict[str, list[dict]] = {
        "normal": [],
        "monster": [],
        "mythic": [],
        "arena": [],
    }

    for entry in data:
        set_type = entry.get("type")
        if set_type not in TYPE_TO_CATEGORY:
            # Unknown type, default to normal
            _, cat = TYPE_TO_CATEGORY[None]
        else:
            _, cat = TYPE_TO_CATEGORY[set_type]
        categories[cat].append(entry)

    rust_types = {
        "normal": "Normal",
        "monster": "Monster",
        "mythic": "Mythic",
        "arena": "Arena",
    }

    for cat, sets in categories.items():
        # Sort alphabetically by name
        sets.sort(key=lambda s: s["name"])
        rust_type = rust_types[cat]
        code = generate_file(sets, rust_type, cat)
        out_path = output_dir / f"{cat}.rs"
        out_path.write_text(code, encoding="utf-8")
        print(f"Generated {len(sets)} sets -> {out_path}")


if __name__ == "__main__":
    main()
