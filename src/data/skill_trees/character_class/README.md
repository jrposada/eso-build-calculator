# Character Class Data

Skills and passives for all 7 playable classes.

## Source

Data extracted from `datamine/parsed/skill_coefficients.json`, originally parsed from [UESP Skill Coefficients](https://esoitem.uesp.net/viewSkillCoef.php).

Skills in the JSON have rank suffixes (e.g., `"Flame Lash 1"` through `"Flame Lash 4"`). Values here use max morph rank (rank 4). Each `SkillData` and `PassiveData` entry includes the `skill_id` matching the UESP ability ID.

## Classes

| Directory       | Class        | Skill Lines                                          |
| --------------- | ------------ | ---------------------------------------------------- |
| `arcanist/`     | Arcanist     | Herald of the Tome, Apocryphal Soldier, Curative Runeforms |
| `dragonknight/` | Dragonknight | Ardent Flame, Draconic Power, Earthen Heart          |
| `necromancer/`  | Necromancer  | Grave Lord, Bone Tyrant, Living Death                |
| `nightblade/`   | Nightblade   | Assassination, Shadow, Siphoning                     |
| `sorcerer/`     | Sorcerer     | Dark Magic, Daedric Summoning, Storm Calling         |
| `templar/`      | Templar      | Aedric Spear, Dawn's Wrath, Restoring Light          |
| `warden/`       | Warden       | Animal Companions, Green Balance, Winter's Embrace   |

Each class directory contains:
- `<class>_skills.rs` — active skills (`SkillData`)
- `<class>_passives.rs` — passive abilities (`PassiveData`)
