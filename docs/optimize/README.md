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
2. **Generate skill line combinations**:
   - Class skill lines: up to 3 from any ESO class (C(21, 0..3) combinations)
   - Weapon skill lines: up to 2 (C(4, 0..2) combinations)
   - Only combinations with enough skills to fill 10 slots are valid
3. **Generate modifier combinations**: C(7,4) = 35 combinations
4. **For each skill line × modifier combination**:
   - Collect all passives from selected skill lines
   - Calculate damage for all skills from those skill lines, applying passive bonuses globally
   - Select best 10 skills by damage (constraints satisfied by skill line selection)
   - Calculate total damage
5. **Return best build** (highest total damage)

### Why Skill Line Combinations Matter

A greedy approach that selects skills by individual damage can miss optimal builds. Skill line passives affect ALL skills in the build, not just skills from that line.

**Example**: Skill Line A has a skill dealing 100 damage. Skill Line B has a skill dealing 99 damage but includes a passive granting +200% damage to all skills. Including Skill Line B results in higher total damage because the passive amplifies every skill in the build.

This is why the algorithm must explore different skill line combinations rather than greedily selecting top-damage skills.

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
