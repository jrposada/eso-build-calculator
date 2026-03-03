# Weapon Data

Skills and passives for the four weapon skill lines.

## Source

Data extracted from `datamine/parsed/skill_coefficients.json`, originally parsed from [UESP Skill Coefficients](https://esoitem.uesp.net/viewSkillCoef.php).

Skills in the JSON have rank suffixes (e.g., `"Poison Injection 1"` through `"Poison Injection 4"`). Values here use max morph rank (rank 4).

## Skill Lines

| Directory            | Skill Line        |
| -------------------- | ----------------- |
| `bow/`               | Bow               |
| `destruction_staff/` | Destruction Staff |
| `dual_wield/`        | Dual Wield        |
| `two_handed/`        | Two Handed        |

Each directory contains:

- `<weapon>_skills.rs` - active skills (`SkillData`)
- `<weapon>_passives.rs` - passive abilities (`PassiveData`)
