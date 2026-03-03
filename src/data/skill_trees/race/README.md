# Race Data

Racial passive bonuses for all 10 playable races.

## Source

Data extracted from `datamine/parsed/skill_coefficients.json`, originally parsed from [UESP Skill Coefficients](https://esoitem.uesp.net/viewSkillCoef.php).

Racial passives in the JSON use the `"Werewolf"` mechanic and have rank suffixes in their names (e.g., `"Life Mender 1"`, `"Life Mender 2"`, `"Life Mender 3"`). Values here use max rank (rank 3). Each `BonusData` entry includes the `skill_id` matching the UESP ability ID.

## Races

| File          | Race     |
| ------------- | -------- |
| `argonian.rs` | Argonian |
| `breton.rs`   | Breton   |
| `dark_elf.rs` | Dark Elf |
| `high_elf.rs` | High Elf |
| `imperial.rs` | Imperial |
| `khajit.rs`   | Khajiit  |
| `nord.rs`     | Nord     |
| `orc.rs`      | Orc      |
| `redguard.rs` | Redguard |
| `wood_elf.rs` | Wood Elf |
