# Guild Data

Skills and passives for the four guild skill lines.

## Source

Data extracted from `datamine/parsed/skill_coefficients.json`, originally parsed from [UESP Skill Coefficients](https://esoitem.uesp.net/viewSkillCoef.php).

Skills in the JSON have rank suffixes (e.g., `"Barbed Trap 1"` through `"Barbed Trap 4"`). Values here use max morph rank (rank 4).

## Skill Lines

| Directory          | Skill Line    |
| ------------------ | ------------- |
| `fighters_guild/`  | Fighters Guild |
| `mages_guild/`     | Mages Guild   |
| `undaunted/`       | Undaunted     |
| `psijic_order/`    | Psijic Order  |

Each directory contains:

- `<guild>_skills.rs` — active skills (`SkillData`)
- `<guild>_passives.rs` — passive abilities (`PassiveData`)

### Special: Undaunted Mettle

`undaunted/undaunted_passives.rs` also exports `undaunted_mettle_bonuses(armor_types: u8)`, which dynamically computes +2% MaxMagicka/MaxStamina per distinct armor weight worn.
