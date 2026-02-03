# Add Missing Skills

Add missing skills and passives to skill line data files based on raw data from eso-hub.

## Arguments

- `$ARGUMENTS` - ESO-Hub URL (e.g., `https://eso-hub.com/en/skills/nightblade/assassination`) or path format `<class>/<skill-line>` (e.g., `nightblade/shadow`)

## Instructions

1. **Parse the input:**
   - If `$ARGUMENTS` is an eso-hub URL, extract the class and skill-line from the path
   - URL format: `https://eso-hub.com/en/skills/<class>/<skill-line>`
   - Path format: `<class>/<skill-line>`

2. **Validate URL format:**
   - **REJECT** class overview URLs like `https://eso-hub.com/en/classes/<class>` - these don't contain skill data
   - **ACCEPT** only skill line URLs: `https://eso-hub.com/en/skills/<class>/<skill-line>`
   - If invalid format detected, stop and inform user of correct format

3. **Fetch skill data from eso-hub:**
   - **IMPORTANT:** The skill line overview page does NOT contain skill data. You MUST fetch individual skill pages.
   - First, fetch the overview page to get the list of skill names
   - Then fetch each individual skill page: `https://eso-hub.com/en/skills/<class>/<skill-line>/<skill-name>`
   - Example URLs:
     - `https://eso-hub.com/en/skills/nightblade/assassination/death-stroke`
     - `https://eso-hub.com/en/skills/nightblade/assassination/ambush`
     - `https://eso-hub.com/en/skills/nightblade/assassination/master-assassin` (for passives)
   - Each skill page contains: name, description, damage values, buffs/debuffs, durations, and morph info

4. **Map URL segments to Rust enums:**

   ### ClassName Mapping
   | URL segment | Rust Enum |
   |-------------|-----------|
   | `nightblade` | `ClassName::Nightblade` |
   | `dragonknight` | `ClassName::Dragonknight` |
   | `sorcerer` | `ClassName::Sorcerer` |
   | `templar` | `ClassName::Templar` |
   | `warden` | `ClassName::Warden` |
   | `arcanist` | `ClassName::Arcanist` |
   | `weapon` | `ClassName::Weapon` |

   ### SkillLineName Mapping
   | URL segment | Rust Enum |
   |-------------|-----------|
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

5. **Read existing files:**
   - Skills: `src/data/skills/<class>.rs`
   - Passives: `src/data/passives/<class>.rs`
   - Cross-reference to identify missing skills and passives

6. **Parse damage values from descriptions:**

   ### Damage Type Patterns (description text → Rust enum)
   | Text Pattern | Rust Enum |
   |--------------|-----------|
   | "Magic Damage" | `DamageType::Magic` |
   | "Physical Damage" | `DamageType::Physical` |
   | "Disease Damage" | `DamageType::Disease` |
   | "Flame Damage" | `DamageType::Flame` |
   | "Frost Damage" | `DamageType::Frost` |
   | "Poison Damage" | `DamageType::Poison` |
   | "Shock Damage" | `DamageType::Shock` |
   | "Bleed Damage" | `DamageType::Bleed` |

   ### Resource Type Rules
   | Condition | Rust Enum |
   |-----------|-----------|
   | Under "Ultimate abilities" section | `Resource::Ultimate` |
   | Magic/Flame/Frost/Shock damage (default) | `Resource::Magicka` |
   | Physical/Disease/Bleed/Poison damage | `Resource::Stamina` |
   | "Converts to Stamina" in morph desc | `Resource::Stamina` |

   ### Target Type Rules
   | Pattern | Rust Enum |
   |---------|-----------|
   | "area", "around you", "enemies nearby", "cone" | `TargetType::Aoe` |
   | Single target (default) | `TargetType::Single` |

7. **Understand skill structure:**
   - **Base skill** - The original unmodified ability
   - **Morph 1** - First morphed version (indicated by "arrow" before it in raw data)
   - **Morph 2** - Second morphed version (indicated by "arrow" before it in raw data)

---

## Part 1: Active Skills

Generate active skills in `src/data/skills/<class>.rs` using the `SkillData` struct.

### Skill Data Structure

```rust
SkillData::new(
    "Skill Name",           // name
    "Base Skill Name",      // base_skill_name (same as name for base skills)
    ClassName::X,           // class_name
    SkillLineName::Y,       // skill_line
    SkillDamage::new()...,  // damage
    DamageType::X,          // damage_type
    TargetType::X,          // target_type
    Resource::X,            // resource
)
// Optional modifiers:
// .with_channel_time(2.5)
// .with_execute(multiplier, threshold, ExecuteScaling::Flat|Linear)
// .with_bonuses(vec![...])
```

### Damage Patterns

**Instant damage:**
```rust
SkillData::new(
    "Lava Whip",
    "Lava Whip",
    ClassName::Dragonknight,
    SkillLineName::ArdentFlame,
    SkillDamage::new().with_hits(vec![HitDamage::new(2323.0)]),
    DamageType::Flame,
    TargetType::Single,
    Resource::Magicka,
)
```

**Delayed damage (like Scorch):**
```rust
SkillData::new(
    "Scorch",
    "Scorch",
    ClassName::Warden,
    SkillLineName::AnimalCompanions,
    SkillDamage::new().with_hits(vec![
        HitDamage::new(2509.0).with_delay(3.0),
        HitDamage::new(3486.0).with_delay(9.0),
    ]),
    DamageType::Magic,
    TargetType::Aoe,
    Resource::Magicka,
)
```

**DoT with interval:**
```rust
SkillData::new(
    "Dragonknight Standard",
    "Dragonknight Standard",
    ClassName::Dragonknight,
    SkillLineName::ArdentFlame,
    SkillDamage::new().with_dots(vec![
        DotDamage::new(870.0, 16.0).with_interval(1.0)
    ]),
    DamageType::Flame,
    TargetType::Aoe,
    Resource::Ultimate,
)
```

**Direct + DoT:**
```rust
SkillData::new(
    "Searing Strike",
    "Searing Strike",
    ClassName::Dragonknight,
    SkillLineName::ArdentFlame,
    SkillDamage::new()
        .with_hits(vec![HitDamage::new(1161.0)])
        .with_dots(vec![DotDamage::new(3470.0, 20.0)]),
    DamageType::Flame,
    TargetType::Single,
    Resource::Magicka,
)
```

**Channeled skill:**
```rust
SkillData::new(
    "Inhale",
    "Inhale",
    ClassName::Dragonknight,
    SkillLineName::DraconicPower,
    SkillDamage::new().with_hits(vec![
        HitDamage::new(870.0),
        HitDamage::new(1742.0).with_delay(2.5),
    ]),
    DamageType::Flame,
    TargetType::Aoe,
    Resource::Magicka,
).with_channel_time(2.5)
```

**Utility skill (no damage):**
```rust
SkillData::new(
    "Frost Cloak",
    "Frost Cloak",
    ClassName::Warden,
    SkillLineName::WintersEmbrace,
    SkillDamage::new(),
    DamageType::Frost,
    TargetType::Aoe,
    Resource::Magicka,
)
```

### DoT Parsing Rules

- "X Damage over Y seconds" → `DotDamage::new(X, Y)` (total damage, no interval)
- "X Damage every Z seconds for Y seconds" → `DotDamage::new(X, Y).with_interval(Z)`
- "increases by N% per tick" → `.with_increase_per_tick(N/100.0)`

### Damage vs Healing Distinction

**CRITICAL:** When a description mentions both damage AND healing with timing, determine which the timing applies to:

| Description Pattern | Interpretation |
|---------------------|----------------|
| "dealing X Damage over Y seconds" | DoT: `DotDamage::new(X, Y)` |
| "dealing X Damage and healing Y every Z seconds for W seconds" | **Instant damage**: `HitDamage::new(X)` - timing applies to healing only |
| "dealing X Damage every Y seconds" | DoT with interval |

**Example - Strife (CORRECT):**
> "dealing 1548 Magic Damage and healing you... every 2 seconds for 10 seconds"

The "every 2 seconds for 10 seconds" describes the healing, NOT the damage. Use:
```rust
SkillDamage::new().with_hits(vec![HitDamage::new(1548.0)])  // Instant damage
```

**NOT:**
```rust
SkillDamage::new().with_dots(vec![DotDamage::new(1548.0, 10.0).with_interval(2.0)])  // WRONG
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

#### Examples with Bonuses

**Skill with enemy debuff:**
```rust
// Teleport Strike: Minor Vulnerability (10s)
SkillData::new(
    "Teleport Strike",
    "Teleport Strike",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    SkillDamage::new().with_hits(vec![HitDamage::new(1602.0)]),
    DamageType::Magic,
    TargetType::Single,
    Resource::Magicka,
)
.with_bonuses(vec![MINOR_VULNERABILITY.clone()])
```

**Skill with multiple buffs:**
```rust
// Ambush: Minor Vulnerability (10s), Empower (10s), Minor Berserk (10s)
SkillData::new(
    "Ambush",
    "Teleport Strike",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    SkillDamage::new().with_hits(vec![HitDamage::new(1655.0)]),
    DamageType::Physical,
    TargetType::Single,
    Resource::Stamina,
)
.with_bonuses(vec![
    MINOR_VULNERABILITY.clone(),
    EMPOWER.clone(),
    MINOR_BERSERK.clone().with_duration(10.0),
])
```

**Skill with "while slotted" buffs:**
```rust
// Grim Focus: Major Prophecy + Major Savagery while slotted
SkillData::new(
    "Grim Focus",
    "Grim Focus",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    SkillDamage::new().with_hits(vec![HitDamage::new(4182.0)]),
    DamageType::Magic,
    TargetType::Single,
    Resource::Magicka,
)
.with_bonuses(vec![
    MAJOR_PROPHECY.clone().with_trigger(BonusTrigger::AbilitySlotted),
    MAJOR_SAVAGERY.clone().with_trigger(BonusTrigger::AbilitySlotted),
])
```

**Skill with custom unique debuff:**
```rust
// Death Stroke: +20% damage from player attacks for 8s
SkillData::new(
    "Death Stroke",
    "Death Stroke",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    SkillDamage::new().with_hits(vec![HitDamage::new(3716.0)]),
    DamageType::Magic,
    TargetType::Single,
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

- "Deals X% more" = Flat bonus when threshold met
- "Deals up to X% more" = Linear scaling based on missing health

```rust
SkillData::new(
    "Assassin's Blade",
    "Assassin's Blade",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    SkillDamage::new().with_hits(vec![HitDamage::new(1161.0)]),
    DamageType::Magic,
    TargetType::Single,
    Resource::Magicka,
)
.with_execute(3.0, 0.25, ExecuteScaling::Flat)
```

### Important Notes for Active Skills

- **IGNORE healing over time (HoT)** - Only record damage, not healing effects
- Use `SkillDamage::new()` for utility/buff skills with no damage
- Group skills by base skill (base + morphs together with comments)
- Always add descriptive comments above skills explaining their bonuses

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

## Common Pitfalls

### Pitfall 1: Confusing Healing Timing with Damage Timing
Skills like Strife deal **instant damage** but heal over time. Don't mistake the healing interval for damage intervals.

### Pitfall 2: Using Class URLs Instead of Skill Line URLs
- **Wrong:** `https://eso-hub.com/en/classes/nightblade`
- **Correct:** `https://eso-hub.com/en/skills/nightblade/assassination`

---

## Reference Files

- `src/domain/skill.rs` - SkillData struct
- `src/domain/skill_damage.rs` - SkillDamage struct
- `src/domain/hit_damage.rs` - HitDamage struct
- `src/domain/dot_damage.rs` - DotDamage struct
- `src/domain/passive.rs` - PassiveData struct
- `src/domain/bonus.rs` - BonusData struct
- `src/data/types.rs` - All enums (ClassName, SkillLineName, DamageType, BonusTrigger, BonusTarget, etc.)
- `src/data/bonuses/unique.rs` - Predefined bonus constants

---

## Verification

After adding skills and passives, run:

```bash
cargo check
```

Compare output against existing skills in `src/data/skills/<class>.rs` and passives in `src/data/passives/<class>.rs` to ensure consistent formatting and structure.
