# Optimize Command

The `optimize` command finds the optimal ESO build to maximize total damage per cast.

## Usage

```bash
eso optimize [options]
```

## Options

| Option | Description | Default |
|--------|-------------|---------|
| `-f, --format <format>` | Output format (`table` or `json`) | `table` |
| `-c, --class <class>` | Require at least 1 skill line from this class | none |
| `-v, --verbose` | Show optimization progress | `false` |

## Build Definition

An optimal build consists of:
- **10 non-ultimate skills** (best morph per base skill)
- **4 modifiers** at max level (selected from 7 available champion points)

## Constraints

- Maximum **3 class skill lines** (can be from different ESO classes)
- Maximum **2 weapon skill lines** (Bow, TwoHanded, DestructionStaff, DualWield)
- At least **1 spammable skill** must be included in the build. A spammable is a skill that deals direct or channeled damage (not DOT)
- When `--class` is specified, at least 1 skill line must be from that class; the remaining 2 class skill line slots can be from any class (or left unused if weapon skills are better alternatives)

## Algorithm

1. **Preprocess skills**: Filter ultimates, deduplicate by baseSkillName (keep best morph)
2. **Generate modifier combinations**: C(7,4) = 35 combinations
3. **For each modifier combination**:
   - Calculate damage for all skills with these modifiers
   - Sort skills by damage descending
   - Greedy select top 10 skills respecting constraints
   - Calculate total damage
4. **Return best build** (highest total damage)

## Examples

### Basic Usage

```bash
eso optimize
```

Output:
```
Optimal Build - Maximum Damage Per Cast
─────────────────────────────────────────────────────────────────────────────────────
Total Damage: 95,189

Modifiers: Biting Aura, Deadly Aim, Master-at-Arms, Thaumaturge

Skills
─────────────────────────────────────────────────────────────────────────────────────
   # Name                      Source       Skill Line             Damage
─────────────────────────────────────────────────────────────────────────────────────
   1 Endless Hail*             Weapon       Bow                     26730
   2 Fatecarver                Arcanist     HeraldOfTheTome         12798
   3 Radiant Glory             Templar      DawnsWrath               8380
  ...
─────────────────────────────────────────────────────────────────────────────────────

Skill Lines: HeraldOfTheTome, DawnsWrath, StormCalling (3/3 class), Bow, DualWield (2/2 weapon)
```

### With Class Filter

Require at least one Nightblade skill line:

```bash
eso optimize -c nightblade
```

### JSON Output

```bash
eso optimize -f json
```

Output:
```json
{
  "skills": [
    {
      "name": "Endless Hail*",
      "skillLine": "Bow",
      "source": "Weapon",
      "damagePerCast": 26730.08
    },
    ...
  ],
  "modifiers": ["Biting Aura", "Deadly Aim", "Master-at-Arms", "Thaumaturge"],
  "totalDamagePerCast": 95189.12,
  "usedClassSkillLines": ["HeraldOfTheTome", "DawnsWrath", "StormCalling"],
  "usedWeaponSkillLines": ["Bow", "DualWield"]
}
```

### Verbose Mode

Show optimization progress:

```bash
eso optimize -v
```

## Valid Classes

- Dragonknight
- Sorcerer
- Nightblade
- Warden
- Necromancer
- Templar
- Arcanist

## Available Modifiers (Champion Points)

| Name | Effect | Max Level | Affects |
|------|--------|-----------|---------|
| Backstabber | +2% per level | 5 | Critical |
| Biting Aura | +3% per level | 2 | AoE |
| Deadly Aim | +3% per level | 2 | Single target |
| Master-at-Arms | +3% per level | 2 | Direct damage |
| Exploiter | +2% per level | 5 | Off-balance |
| Fighting Finesse | +4% per level | 2 | Critical |
| Thaumaturge | +3% per level | 2 | DoT |

## Related Files

- `/src/commands/optimize-build.ts` - Command implementation
- `/src/services/build-optimizer.ts` - Optimization algorithm
- `/src/models/build.ts` - Build interfaces
