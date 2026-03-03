# Armor Data

Passives for the three armor skill lines (Light, Medium, Heavy).

## Source

Data extracted from `datamine/parsed/skill_coefficients.json`, originally parsed from [UESP Skill Coefficients](https://esoitem.uesp.net/viewSkillCoef.php).

Armor passives in the JSON have rank suffixes (e.g., `"Agility 1"`, `"Agility 2"`). Values here use max rank. Each `BonusData` entry includes the `skill_id` matching the UESP ability ID.

Passive values are per-piece. Stacking by number of equipped pieces is handled at the service layer.

## Skill Lines

| Directory | Skill Line   | DPS Passives           | Non-DPS Passives (omitted)                                   |
| --------- | ------------ | ---------------------- | ------------------------------------------------------------ |
| `light/`  | Light Armor  | Prodigy, Concentration | Evocation, Grace, Spell Warding                              |
| `medium/` | Medium Armor | Agility, Dexterity     | Wind Walker, Athletics, Improved Sneak                       |
| `heavy/`  | Heavy Armor  | _(none)_               | Juggernaut, Resolve, Constitution, Revitalize, Rapid Mending |

Each directory contains:

- `<weight>_passives.rs` — DPS-relevant passives as `BonusData`
- `<weight>_skills.rs` — Empty; armor active skills are defensive buffs not modeled in the calculator
