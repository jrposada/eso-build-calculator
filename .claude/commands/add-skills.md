# Add Missing Skills

Add missing skills and passives to skill line data files based on coefficient data from UESP.

## Data Source

**UESP Skill Coefficients**: https://esoitem.uesp.net/viewSkillCoef.php

The UESP provides accurate damage coefficients derived from in-game data through regression analysis. This is the authoritative source for skill damage formulas.

**Local Data File**: `datamine/UESP_ESO Skill Coefficients.html`

Download the page and save it locally for reference. The file contains a table with columns:
- Skill Name, ID, Mechanic, Class, Skill Line, #, Description, Equations

## Arguments

- `$ARGUMENTS` - Class and skill line in format `<class>/<skill-line>` (e.g., `nightblade/assassination`, `dragonknight/ardent-flame`)

## Instructions

1. **Parse the input:**
   - Format: `<class>/<skill-line>`
   - Example: `nightblade/assassination`, `dragonknight/ardent-flame`

2. **Read the UESP data file:**
   - Open `datamine/UESP_ESO Skill Coefficients.html`
   - Search for skills matching the specified class and skill line
   - Skills have numbered versions (e.g., "Burning Embers 1", "Burning Embers 2", etc.) - use version 4 (max rank)

3. **Map class/skill-line to Rust enums:**

   ### ClassName Mapping
   | Input | Rust Enum |
   |-------|-----------|
   | `nightblade` | `ClassName::Nightblade` |
   | `dragonknight` | `ClassName::Dragonknight` |
   | `sorcerer` | `ClassName::Sorcerer` |
   | `templar` | `ClassName::Templar` |
   | `warden` | `ClassName::Warden` |
   | `arcanist` | `ClassName::Arcanist` |
   | `weapon` | `ClassName::Weapon` |

   ### SkillLineName Mapping
   | Input | Rust Enum |
   |-------|-----------|
   | `assassination` | `SkillLineName::Assassination` |
   | `shadow` | `SkillLineName::Shadow` |
   | `siphoning` | `SkillLineName::Siphoning` |
   | `ardent-flame` | `SkillLineName::ArdentFlame` |
   | `draconic-power` | `SkillLineName::DraconicPower` |
   | `earthen-heart` | `SkillLineName::EarthenHeart` |
   | `dark-magic` | `SkillLineName::DarkMagic` |
   | `daedric-summoning` | `SkillLineName::DaedricSummoning` |
   | `storm-calling` | `SkillLineName::StormCalling` |
   | `aedric-spear` | `SkillLineName::AedricSpear` |
   | `dawns-wrath` | `SkillLineName::DawnsWrath` |
   | `restoring-light` | `SkillLineName::RestoringLight` |
   | `animal-companions` | `SkillLineName::AnimalCompanions` |
   | `green-balance` | `SkillLineName::GreenBalance` |
   | `winters-embrace` | `SkillLineName::WintersEmbrace` |
   | `curative-runeforms` | `SkillLineName::CurativeRuneforms` |
   | `soldier-of-apocrypha` | `SkillLineName::SoldierOfApocrypha` |
   | `herald-of-the-tome` | `SkillLineName::HeraldOfTheTome` |
   | `bow` | `SkillLineName::Bow` |
   | `two-handed` | `SkillLineName::TwoHanded` |
   | `destruction-staff` | `SkillLineName::DestructionStaff` |
   | `dual-wield` | `SkillLineName::DualWield` |

4. **Read existing files:**
   - Skills: `src/data/skills/<class>.rs`
   - Passives: `src/data/passives/<class>.rs`
   - Cross-reference to identify missing skills and passives

---

## UESP Coefficient Format

The UESP equations column contains damage formulas in this format:

```
<<1>> = 0.05165 MaxStat + 0.542325 MaxPower  (Ultimate, ratio = 10.50, Dmg, Flame, SingleTarget, Direct, Melee, -0.001s duration, -0.001s tick, -0.001s cooldown, R2 = 1)
<<2>> = 0.015495 MaxStat + 0.162697 MaxPower  (Ultimate, ratio = 10.50, Dmg, Flame, SingleTarget, DOT, 18s duration, 2s tick, -0.001s cooldown, R2 = 1)
<<3>> = 20 seconds (Constant)
```

### Parsing Rules

| Pattern | Meaning |
|---------|---------|
| `X MaxStat + Y MaxPower` | Damage coefficients: coef_a = X, coef_b = Y |
| `(Constant)` | Fixed value, no coefficients |
| `Direct` in metadata | Instant hit damage |
| `DOT` in metadata | Damage over time |
| `Xs duration, Ys tick` | DoT duration and tick interval |
| `Dmg, Flame` | Flame damage type |
| `Dmg, Magic` | Magic damage type |
| `SingleTarget` | Single target |
| `AOE` | Area of effect |

### Coefficient Extraction

From `0.05165 MaxStat + 0.542325 MaxPower`:
- `coef_a` = 0.05165 (MaxStat coefficient)
- `coef_b` = 0.542325 (MaxPower coefficient)

---

## Part 1: Active Skills

Generate active skills in `src/data/skills/<class>.rs` using the `SkillData` struct with coefficients.

### Skill Data Structure with Coefficients

```rust
SkillData::new(
    "Skill Name",           // name
    "Base Skill Name",      // base_skill_name (same as name for base skills)
    ClassName::X,           // class_name
    SkillLineName::Y,       // skill_line
    SkillDamage::new()...,  // damage with coefficients and per-component DamageFlags
    Resource::X,            // resource
)
// Optional modifiers:
// .with_channel_time(2.5)
// .with_execute(multiplier, threshold, ExecuteScaling::Flat|Linear)
// .with_bonuses(vec![...])
```

### DamageFlags

Each `HitDamage` and `DotDamage` carries its own `DamageFlags` bitflag describing element, target scope, delivery method, and range. The `DIRECT` flag is auto-added by `HitDamage::new()` and `DOT` is auto-added by `DotDamage::new()`.

**Convenience constructors:**
- `DamageFlags::magic_single()`, `DamageFlags::magic_aoe()`
- `DamageFlags::physical_single()`, `DamageFlags::physical_aoe()`
- `DamageFlags::flame_single()`, `DamageFlags::flame_aoe()`
- `DamageFlags::frost_single()`, `DamageFlags::frost_aoe()`
- `DamageFlags::shock_single()`, `DamageFlags::shock_aoe()`
- `DamageFlags::poison_single()`, `DamageFlags::poison_aoe()`
- `DamageFlags::disease_single()`, `DamageFlags::disease_aoe()`
- `DamageFlags::bleed_single()`, `DamageFlags::bleed_aoe()`

### Damage Patterns with Coefficients

**Instant damage with coefficients:**
```rust
// UESP: <<1>> = 0.09797 MaxStat + 1.02992 MaxPower (Dmg, Flame, SingleTarget, Direct)
SkillData::new(
    "Lava Whip",
    "Lava Whip",
    ClassName::Dragonknight,
    SkillLineName::ArdentFlame,
    SkillDamage::new().with_hits(vec![
        HitDamage::new(2323.0, DamageFlags::flame_single())
            .with_coefficients(0.09797, 1.02992)
    ]),
    Resource::Magicka,
)
```

**DoT with coefficients and interval:**
```rust
// UESP: <<1>> = 0.05165 MaxStat + 0.542325 MaxPower (Dmg, Flame, SingleTarget, Direct)
//       <<2>> = 0.015495 MaxStat + 0.162697 MaxPower (Dmg, Flame, SingleTarget, DOT, 18s duration, 2s tick)
SkillData::new(
    "Burning Embers",
    "Searing Strike",
    ClassName::Dragonknight,
    SkillLineName::ArdentFlame,
    SkillDamage::new()
        .with_hits(vec![
            HitDamage::new(1161.0, DamageFlags::flame_single())
                .with_coefficients(0.05165, 0.542325)
        ])
        .with_dots(vec![
            DotDamage::new(3470.0, 18.0, DamageFlags::flame_single())
                .with_interval(2.0)
                .with_coefficients(0.015495, 0.162697)
        ]),
    Resource::Magicka,
)
```

**Mixed-type skill (different flags per component):**
```rust
// UESP: <<1>> = 0.071294 MaxStat + 0.748588 MaxPower (Dmg, Magic, AOE, Direct)
//       <<2>> = 0.012866 MaxStat + 0.135137 MaxPower (Dmg, Magic, SingleTarget, DOT, 5s duration, 1s tick)
SkillData::new(
    "Lotus Fan",
    "Teleport Strike",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    SkillDamage::new()
        .with_hits(vec![
            HitDamage::new(1603.0, DamageFlags::magic_aoe())
                .with_coefficients(0.071294, 0.748588)
        ])
        .with_dots(vec![
            DotDamage::new(2050.0, 5.0, DamageFlags::magic_single())
                .with_interval(1.0)
                .with_coefficients(0.012866, 0.135137)
        ]),
    Resource::Magicka,
)
```

**Delayed damage with coefficients:**
```rust
// UESP: <<1>> = 0.077475 MaxStat + 0.813488 MaxPower (Dmg, Magic, AOE, Direct)
//       <<2>> = 0.107663 MaxStat + 1.130456 MaxPower (Dmg, Magic, AOE, Direct)
SkillData::new(
    "Scorch",
    "Scorch",
    ClassName::Warden,
    SkillLineName::AnimalCompanions,
    SkillDamage::new().with_hits(vec![
        HitDamage::new(2509.0, DamageFlags::magic_aoe())
            .with_delay(3.0)
            .with_coefficients(0.077475, 0.813488),
        HitDamage::new(3486.0, DamageFlags::magic_aoe())
            .with_delay(9.0)
            .with_coefficients(0.107663, 1.130456),
    ]),
    Resource::Magicka,
)
```

**Channeled skill with coefficients:**
```rust
// UESP: <<1>> = 0.038738 MaxStat + 0.406744 MaxPower (Dmg, Flame, AOE, Direct)
//       <<2>> = 0.077475 MaxStat + 0.813488 MaxPower (Dmg, Flame, AOE, Direct)
SkillData::new(
    "Inhale",
    "Inhale",
    ClassName::Dragonknight,
    SkillLineName::DraconicPower,
    SkillDamage::new().with_hits(vec![
        HitDamage::new(870.0, DamageFlags::flame_aoe())
            .with_coefficients(0.038738, 0.406744),
        HitDamage::new(1742.0, DamageFlags::flame_aoe())
            .with_delay(2.5)
            .with_coefficients(0.077475, 0.813488),
    ]),
    Resource::Magicka,
).with_channel_time(2.5)
```

**Utility skill (no damage, no coefficients):**
```rust
SkillData::new(
    "Frost Cloak",
    "Frost Cloak",
    ClassName::Warden,
    SkillLineName::WintersEmbrace,
    SkillDamage::new(),  // No coefficients needed for utility skills
    Resource::Magicka,
)
```

### DamageFlags Mapping (from UESP metadata)

Each damage component's UESP metadata maps to a `DamageFlags` convenience constructor:

| UESP Element + Target | DamageFlags Constructor |
|------------------------|-------------------------|
| `Dmg, Magic, SingleTarget` | `DamageFlags::magic_single()` |
| `Dmg, Magic, AOE` | `DamageFlags::magic_aoe()` |
| `Dmg, Physical, SingleTarget` | `DamageFlags::physical_single()` |
| `Dmg, Physical, AOE` | `DamageFlags::physical_aoe()` |
| `Dmg, Flame, SingleTarget` | `DamageFlags::flame_single()` |
| `Dmg, Flame, AOE` | `DamageFlags::flame_aoe()` |
| `Dmg, Frost, SingleTarget` | `DamageFlags::frost_single()` |
| `Dmg, Frost, AOE` | `DamageFlags::frost_aoe()` |
| `Dmg, Shock, SingleTarget` | `DamageFlags::shock_single()` |
| `Dmg, Shock, AOE` | `DamageFlags::shock_aoe()` |
| `Dmg, Poison, SingleTarget` | `DamageFlags::poison_single()` |
| `Dmg, Poison, AOE` | `DamageFlags::poison_aoe()` |
| `Dmg, Disease, SingleTarget` | `DamageFlags::disease_single()` |
| `Dmg, Disease, AOE` | `DamageFlags::disease_aoe()` |
| `Dmg, Bleed, SingleTarget` | `DamageFlags::bleed_single()` |
| `Dmg, Bleed, AOE` | `DamageFlags::bleed_aoe()` |

Note: `DIRECT` flag is auto-added by `HitDamage::new()` and `DOT` flag is auto-added by `DotDamage::new()`, so you don't need to include those.

**Important:** Different components of the same skill can have different flags. For example, Lotus Fan has an AOE hit (`magic_aoe()`) and a SingleTarget DoT (`magic_single()`). Always use the per-component metadata from UESP.

### Resource Type Rules

| Condition | Rust Enum |
|-----------|-----------|
| Ultimate abilities (high cost, powerful) | `Resource::Ultimate` |
| Magic/Flame/Frost/Shock damage (default) | `Resource::Magicka` |
| Physical/Disease/Bleed/Poison damage | `Resource::Stamina` |
| Morph converts to Stamina | `Resource::Stamina` |

### DoT Parsing from UESP

From `(DOT, 18s duration, 2s tick)`:
- Duration: 18 seconds → `DotDamage::new(value, 18.0)`
- Tick interval: 2 seconds → `.with_interval(2.0)`

### Tooltip Values

The tooltip `value` parameter in `HitDamage::new(value, flags)` and `DotDamage::new(value, duration, flags)` should be calculated from the coefficients at standard stats (40k max stat, 5.5k max power) or taken from in-game tooltips. This value is used as a fallback when coefficients aren't available.

```
tooltip_value = coef_a * 40000 + coef_b * 5500
```

### Active Skill Bonuses (Buffs/Debuffs on Cast)

Skills can grant buffs to the player or apply debuffs to enemies. Use `.with_bonuses()` to attach these effects.

#### Player Buffs (self-buffs on cast)

| Text Pattern | Predefined Constant | Effect |
|--------------|---------------------|--------|
| "Minor Berserk" | `MINOR_BERSERK` | +5% damage done |
| "Minor Brutality" | `MINOR_BRUTALITY` | +10% Weapon Damage |
| "Minor Sorcery" | `MINOR_SORCERY` | +10% Spell Damage |
| "Minor Savagery" | `MINOR_SAVAGERY` | +1314 Weapon Crit |
| "Minor Prophecy" | `MINOR_PROPHECY` | +1314 Spell Crit |
| "Major Berserk" | `MAJOR_BERSERK` | +10% damage done |
| "Major Brutality" | `MAJOR_BRUTALITY` | +20% Weapon Damage |
| "Major Sorcery" | `MAJOR_SORCERY` | +20% Spell Damage |
| "Major Savagery" | `MAJOR_SAVAGERY` | +2629 Weapon Crit |
| "Major Prophecy" | `MAJOR_PROPHECY` | +2629 Spell Crit |
| "Empower" | `EMPOWER` | +70% Heavy Attack damage |

#### Enemy Debuffs (applied to target)

| Text Pattern | Predefined Constant | Effect |
|--------------|---------------------|--------|
| "Minor Vulnerability" | `MINOR_VULNERABILITY` | +5% damage taken by enemy |
| "Major Breach" / "Sundered" | `MAJOR_BREACH` | -5948 enemy resistance |

#### Custom Debuffs (unique to skill)

For unique debuffs like Death Stroke's +20% damage taken, create inline:
```rust
BonusData::new(
    "Death Stroke Debuff",
    BonusTrigger::Cast,
    BonusTarget::EnemyDamageTaken,
    0.20,  // 20%
)
.with_duration(8.0)
```

#### Changing Bonus Triggers

Use `.with_trigger()` to change when a bonus applies:

```rust
// Major Prophecy active while ability is slotted (not on cast)
MAJOR_PROPHECY.clone().with_trigger(BonusTrigger::AbilitySlotted)
```

#### Examples with Bonuses and Coefficients

**Skill with enemy debuff:**
```rust
// Teleport Strike: Minor Vulnerability (10s)
// UESP: <<1>> = 0.071294 MaxStat + 0.748588 MaxPower (Dmg, Magic, SingleTarget, Direct)
SkillData::new(
    "Teleport Strike",
    "Teleport Strike",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    SkillDamage::new().with_hits(vec![
        HitDamage::new(1602.0, DamageFlags::magic_single())
            .with_coefficients(0.071294, 0.748588)
    ]),
    Resource::Magicka,
)
.with_bonuses(vec![MINOR_VULNERABILITY.clone()])
```

**Skill with custom unique debuff:**
```rust
// Death Stroke: +20% damage from player attacks for 8s
// UESP: <<1>> = 0.165 MaxStat + 1.7325 MaxPower (Dmg, Magic, SingleTarget, Direct)
SkillData::new(
    "Death Stroke",
    "Death Stroke",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    SkillDamage::new().with_hits(vec![
        HitDamage::new(3716.0, DamageFlags::magic_single())
            .with_coefficients(0.165, 1.7325)
    ]),
    Resource::Ultimate,
)
.with_bonuses(vec![BonusData::new(
    "Death Stroke Debuff",
    BonusTrigger::Cast,
    BonusTarget::EnemyDamageTaken,
    0.20,
)
.with_duration(8.0)])
```

### Execute/Finisher Abilities

Track execute mechanics with `.with_execute()`:

| Pattern | Multiplier | Threshold | Scaling |
|---------|------------|-----------|---------|
| "Deals 300% more damage to enemies below 25% Health" | 3.0 | 0.25 | Flat |
| "Deals up to 400% more damage to enemies below 50% Health" | 4.0 | 0.50 | Linear |

```rust
SkillData::new(
    "Assassin's Blade",
    "Assassin's Blade",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    SkillDamage::new().with_hits(vec![
        HitDamage::new(1161.0, DamageFlags::magic_single())
            .with_coefficients(0.05165, 0.542325)
    ]),
    Resource::Magicka,
)
.with_execute(3.0, 0.25, ExecuteScaling::Flat)
```

### Important Notes for Active Skills

- **IGNORE healing** - Only record damage, not healing effects
- Use `SkillDamage::new()` for utility/buff skills with no damage
- Group skills by base skill (base + morphs together with comments)
- Always add descriptive comments above skills explaining their bonuses
- Use skill version 4 (max rank) from UESP data

---

## Part 2: Passive Abilities

Generate passives in `src/data/passives/<class>.rs` using the `PassiveData` struct.

### Passive Data Structure

```rust
PassiveData::new(
    "Passive Name",
    ClassName::X,
    SkillLineName::Y,
    vec![BonusData::new(...), ...],  // One or more bonuses
)
```

### BonusTrigger Mapping (description text → Rust enum)

| Text Pattern | Rust Enum |
|--------------|-----------|
| Always active (no condition) | `BonusTrigger::Passive` |
| "While slotted" / "WITH ... SLOTTED" | `BonusTrigger::SkillLineSlotted` |
| "while this ability is slotted" (specific ability) | `BonusTrigger::AbilitySlotted` |
| "for each ... ability slotted" | `BonusTrigger::AbilitySlottedCount` |
| "When you cast" / on ability use | `BonusTrigger::Cast` |
| "When you deal Critical Damage" | `BonusTrigger::CriticalDamageDealt` |
| "when flanking" / "from flank" | `BonusTrigger::Flanking` |
| "When ... damage is dealt" (burning/poison) | `BonusTrigger::BurningOrPoisonDamageDealt` |
| "When Magicka or Stamina restored" | `BonusTrigger::MagickaOrStaminaRestored` |
| "While Bow equipped" | `BonusTrigger::BowEquipped` |
| "While Two Handed equipped" | `BonusTrigger::TwoHandedEquipped` |
| "While Dual Wield equipped" | `BonusTrigger::DualWieldEquipped` |
| "While Destruction Staff equipped" | `BonusTrigger::DestructionStuffEquipped` |
| "When you have Crux" (Arcanist) | `BonusTrigger::ArcanistCrux` |

### BonusTarget Mapping (description text → Rust enum)

#### Player Stats
| Text Pattern | Rust Enum |
|--------------|-----------|
| "Critical Chance rating" / "Weapon Critical" / "Spell Critical" | `BonusTarget::CriticalChance` |
| "Weapon Critical" (specifically) | `BonusTarget::WeaponCriticalChance` |
| "Spell Critical" (specifically) | `BonusTarget::SpellCriticalChance` |
| "Critical Damage" | `BonusTarget::CriticalDamage` |
| "damage done" / "damage by X%" | `BonusTarget::Damage` |
| "Weapon Damage" | `BonusTarget::WeaponDamage` |
| "Spell Damage" | `BonusTarget::SpellDamage` |
| "Weapon and Spell Damage" | `BonusTarget::WeaponAndSpellDamage` |
| "Physical and Spell Penetration" | `BonusTarget::PhysicalAndSpellPenetration` |
| "Max Magicka" | `BonusTarget::MaxMagicka` |
| "Max Stamina" | `BonusTarget::MaxStamina` |
| "Heavy Attack damage" | `BonusTarget::HeavyAttackDamage` |

#### Damage Type Bonuses
| Text Pattern | Rust Enum |
|--------------|-----------|
| "direct damage" | `BonusTarget::DirectDamage` |
| "damage over time" | `BonusTarget::DotDamage` |
| "AoE damage" / "area damage" | `BonusTarget::AoeDamage` |
| "single target damage" | `BonusTarget::SingleDamage` |
| "Burning" and/or "Poison" damage | `BonusTarget::BurningAndPoisonDamage` |
| "Shock damage" | `BonusTarget::ShockDamage` |
| "Physical damage" | `BonusTarget::PhysicalDamage` |
| "Off Balance" damage | `BonusTarget::OffBalanceDamage` |

#### Enemy Debuffs
| Text Pattern | Rust Enum |
|--------------|-----------|
| "damage taken" (by enemy) | `BonusTarget::EnemyDamageTaken` |
| "reduce ... Resistance" | `BonusTarget::EnemyResistanceReduction` |

#### Utility
| Text Pattern | Rust Enum |
|--------------|-----------|
| "restore ... Magicka" or "restore ... Stamina" | `BonusTarget::RestoreMagickaOrStamina` |
| "duration" (flat seconds) | `BonusTarget::DurationSkillLineFlat` |
| "duration" (percentage) | `BonusTarget::DurationSkillLineMultiplier` |

### Bonus Value Parsing

| Pattern | Value Type |
|---------|------------|
| "by X%" | Decimal (e.g., 10% → `0.10`) |
| "by X rating" | Raw number (e.g., `1448.0`) |
| "restore X" | Raw number |

### Optional Bonus Methods

- `.with_duration(seconds)` - for buffs with duration (e.g., "for 20 seconds")
- `.with_cooldown(seconds)` - for effects with cooldown
- `.with_trigger(BonusTrigger::X)` - to change the trigger type
- `.with_alternative(target, value, breakpoint)` - for conditional bonuses

### Passive Examples

**Conditional passive (flanking):**
```rust
// Master Assassin: +1448 Crit Chance (6.6%) when flanking enemies
PassiveData::new(
    "Master Assassin",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    vec![BonusData::new(
        "Master Assassin",
        BonusTrigger::Flanking,
        BonusTarget::CriticalChance,
        1448.0,
    )],
)
```

**Scaling passive (per ability slotted):**
```rust
// Pressure Points: +548 Crit Chance (2.5%) per Assassination ability slotted
PassiveData::new(
    "Pressure Points",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    vec![BonusData::new(
        "Pressure Points",
        BonusTrigger::AbilitySlottedCount,
        BonusTarget::CriticalChance,
        548.0,  // Per ability
    )],
)
```

**Multi-bonus passive with different triggers:**
```rust
// Hemorrhage: +10% Crit Damage always, Minor Savagery to group on dealing crit damage
PassiveData::new(
    "Hemorrhage",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    vec![
        BonusData::new(
            "Hemorrhage",
            BonusTrigger::Passive,
            BonusTarget::CriticalDamage,
            0.10,
        ),
        MINOR_SAVAGERY.clone().with_trigger(BonusTrigger::CriticalDamageDealt),
    ],
)
```

**Passive with cooldown:**
```rust
PassiveData::new(
    "Combustion",
    ClassName::Dragonknight,
    SkillLineName::ArdentFlame,
    vec![
        BonusData::new(
            "Combustion 1",
            BonusTrigger::Passive,
            BonusTarget::BurningAndPoisonDamage,
            0.33,
        ),
        BonusData::new(
            "Combustion 2",
            BonusTrigger::BurningOrPoisonDamageDealt,
            BonusTarget::RestoreMagickaOrStamina,
            423.0,
        )
        .with_cooldown(3.0),
    ],
)
```

**Passive with no implemented bonuses (placeholder):**
```rust
// Executioner: Restore 1000 Magicka and Stamina when enemy dies within 2s
PassiveData::new(
    "Executioner",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    vec![],  // Resource restore on kill - not tracked in damage calculations
)
```

### Predefined Bonuses

Import from `src/data/bonuses/unique.rs`:

#### Player Buffs
- `MINOR_BERSERK` - 5% damage, Cast trigger, 20s duration
- `MINOR_BRUTALITY` - 10% Weapon Damage, Cast trigger, 20s duration
- `MINOR_SORCERY` - 10% Spell Damage, Cast trigger, 20s duration
- `MINOR_SAVAGERY` - 1314 Weapon Crit, Cast trigger, 20s duration
- `MINOR_PROPHECY` - 1314 Spell Crit, Cast trigger, 20s duration
- `MAJOR_BERSERK` - 10% damage, Cast trigger, 20s duration
- `MAJOR_BRUTALITY` - 20% Weapon Damage, Cast trigger, 20s duration
- `MAJOR_SORCERY` - 20% Spell Damage, Cast trigger, 20s duration
- `MAJOR_SAVAGERY` - 2629 Weapon Crit, Cast trigger, 20s duration
- `MAJOR_PROPHECY` - 2629 Spell Crit, Cast trigger, 20s duration
- `EMPOWER` - 70% Heavy Attack damage, Cast trigger, 10s duration

#### Enemy Debuffs
- `MINOR_VULNERABILITY` - 5% enemy damage taken, Cast trigger, 10s duration
- `MAJOR_BREACH` - 5948 enemy resistance reduction, Cast trigger, 20s duration

---

## Reference Files

- `src/domain/skill.rs` - SkillData struct
- `src/domain/skill_damage.rs` - SkillDamage struct
- `src/domain/hit_damage.rs` - HitDamage struct with `with_coefficients()`
- `src/domain/dot_damage.rs` - DotDamage struct with `with_coefficients()`
- `src/domain/character_stats.rs` - CharacterStats struct
- `src/domain/damage_coefficients.rs` - DamageCoefficients struct
- `src/domain/passive.rs` - PassiveData struct
- `src/domain/bonus.rs` - BonusData struct
- `src/domain/damage_flags.rs` - DamageFlags bitflag type with per-component element/target/delivery flags
- `src/data/types.rs` - All enums (ClassName, SkillLineName, BonusTrigger, BonusTarget, etc.)
- `src/data/bonuses/unique.rs` - Predefined bonus constants
- `datamine/UESP_ESO Skill Coefficients.html` - UESP coefficient data

---

## Verification

After adding skills and passives, run:

```bash
cargo check
cargo test
```

Compare output against existing skills in `src/data/skills/<class>.rs` and passives in `src/data/passives/<class>.rs` to ensure consistent formatting and structure.
