# Add Missing Skills

Add missing skills and passives to skill line data files based on raw data from eso-hub.

## Arguments

- `$ARGUMENTS` - ESO-Hub URL (e.g., `https://eso-hub.com/en/skills/nightblade/assassination`) or path format `<class>/<skill-line>` (e.g., `nightblade/shadow`)

## Instructions

1. **Parse the input:**
   - If `$ARGUMENTS` is an eso-hub URL, extract the class and skill-line from the path
   - URL format: `https://eso-hub.com/en/skills/<class>/<skill-line>`
   - Path format: `<class>/<skill-line>`

2. **Fetch skill data from eso-hub:**
   - Navigate to the URL and extract raw skill data from the page
   - The page contains sections: "Ultimate abilities", "Active abilities", "Passive abilities"
   - Each skill shows: name, description (with damage values), and morphs

3. **Map URL segments to Rust enums:**

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

4. **Read existing files:**
   - Skills: `src/data/skills/<class>.rs`
   - Passives: `src/data/passives/<class>.rs`
   - Cross-reference to identify missing skills and passives

5. **Parse damage values from descriptions:**

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

6. **Understand skill structure:**
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
// Optional: .with_channel_time(2.5)
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

### Active Skill Bonuses (Buffs on Cast)

Note buffs granted by skills in comments. Common patterns:

| Text Pattern | Predefined Bonus |
|--------------|------------------|
| "grants you Minor Berserk" | `MINOR_BERSERK` |
| "grants you Minor Brutality" | `MINOR_BRUTALITY` |
| "grants you Minor Sorcery" | `MINOR_SORCERY` |
| "grants you Minor Savagery" | `MINOR_SAVAGERY` |
| "grants you Minor Prophecy" | `MINOR_PROPHECY` |
| "grants you Empower" | Empower (70% Heavy Attack damage) |
| "Major Berserk" | 10% damage done |
| "Major Brutality" | 20% Weapon Damage |
| "Major Sorcery" | 20% Spell Damage |
| "Major Savagery" | 2629 Weapon Crit |
| "Major Prophecy" | 2629 Spell Crit |

Example:
```rust
// Ambush grants: Empower (10s), Minor Berserk (10s)
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
```

### Execute/Finisher Abilities

Track execute mechanics in comments:

| Pattern | Multiplier | Threshold | Scaling |
|---------|------------|-----------|---------|
| "Deals 300% more damage to enemies below 25% Health" | 300% | 25% | Flat |
| "Deals up to 400% more damage to enemies below 50% Health" | 400% | 50% | Linear |

- "Deals X% more" = Flat bonus when threshold met
- "Deals up to X% more" = Linear scaling based on missing health

Example:
```rust
// Execute: 300% bonus damage to enemies below 25% Health (flat)
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
```

### Important Notes for Active Skills

- **IGNORE healing over time (HoT)** - Only record damage, not healing effects
- Use `SkillDamage::new()` for utility/buff skills with no damage
- Group skills by base skill (base + morphs together with comments)

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
| "for each ... ability slotted" | `BonusTrigger::AbilitySlottedCount` |
| "When you cast" / on ability use | `BonusTrigger::Cast` |
| "When ... damage is dealt" (burning/poison) | `BonusTrigger::BurningOrPoisonDamageDealt` |
| "When Magicka or Stamina restored" | `BonusTrigger::MagickaOrStaminaRestored` |
| "While Bow equipped" | `BonusTrigger::BowEquipped` |
| "While Two Handed equipped" | `BonusTrigger::TwoHandedEquipped` |
| "While Dual Wield equipped" | `BonusTrigger::DualWieldEquipped` |
| "While Destruction Staff equipped" | `BonusTrigger::DestructionStuffEquipped` |
| "When you have Crux" (Arcanist) | `BonusTrigger::ArcanistCrux` |

### BonusTarget Mapping (description text → Rust enum)

| Text Pattern | Rust Enum |
|--------------|-----------|
| "Critical Chance rating" / "Weapon Critical" / "Spell Critical" | `BonusTarget::CriticalChance` |
| "Critical Damage" | `BonusTarget::CriticalDamage` |
| "damage done" / "damage by X%" | `BonusTarget::Damage` |
| "Weapon Damage" | `BonusTarget::WeaponDamage` |
| "Spell Damage" | `BonusTarget::SpellDamage` |
| "Weapon and Spell Damage" | `BonusTarget::WeaponAndSpellDamage` |
| "Physical and Spell Penetration" / "Resistance" | `BonusTarget::PhysicalAndSpellPenetration` |
| "restore ... Magicka" or "restore ... Stamina" | `BonusTarget::RestoreMagickaOrStamina` |
| "direct damage" | `BonusTarget::DirectDamage` |
| "damage over time" | `BonusTarget::DotDamage` |
| "AoE damage" / "area damage" | `BonusTarget::AoeDamage` |
| "single target damage" | `BonusTarget::SingleDamage` |
| "Burning" and/or "Poison" damage | `BonusTarget::BurningAndPoisonDamage` |
| "Shock damage" | `BonusTarget::ShockDamage` |
| "Physical damage" | `BonusTarget::PhysicalDamage` |
| "Off Balance" damage | `BonusTarget::OffBalanceDamage` |
| "Max Magicka" | `BonusTarget::MaxMagicka` |
| "Max Stamina" | `BonusTarget::MaxStamina` |
| "duration" (skill line abilities) | `BonusTarget::DurationSkillLineMultiplier` |

### Bonus Value Parsing

| Pattern | Value Type |
|---------|------------|
| "by X%" | Decimal (e.g., 10% → `0.10`) |
| "by X rating" | Raw number (e.g., `1448.0`) |
| "restore X" | Raw number |

### Optional Bonus Methods

- `.with_duration(seconds)` - for buffs with duration (e.g., "for 20 seconds")
- `.with_cooldown(seconds)` - for effects with cooldown
- `.with_alternative(target, value, breakpoint)` - for conditional bonuses

### Passive Examples

**Simple passive (always active):**
```rust
PassiveData::new(
    "Master Assassin",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    vec![BonusData::new(
        "Master Assassin",
        BonusTrigger::Passive,
        BonusTarget::CriticalChance,
        1448.0,
    )],
)
```

**Scaling passive (per ability slotted):**
```rust
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

**Multi-bonus passive:**
```rust
PassiveData::new(
    "Hemorrhage",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    vec![
        BonusData::new(
            "Hemorrhage 1",
            BonusTrigger::SkillLineSlotted,
            BonusTarget::CriticalDamage,
            0.10,  // 10%
        ),
        BonusData::new(
            "Hemorrhage 2",
            BonusTrigger::Cast,  // On dealing crit damage
            BonusTarget::WeaponCriticalChance,
            1314.0,
        )
        .with_duration(20.0),
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
PassiveData::new(
    "Executioner",
    ClassName::Nightblade,
    SkillLineName::Assassination,
    vec![],  // TODO: Restore 1000 Magicka and Stamina when enemy dies within 2s
)
```

### Predefined Bonuses

Import from `src/data/bonuses/unique.rs`:
- `MINOR_SAVAGERY` - 1314 Weapon Crit, Cast trigger, 20s duration
- `MINOR_BERSERK` - 5% damage, Cast trigger, 20s duration
- `MINOR_PROPHECY` - 1314 Spell Crit, Cast trigger, 20s duration
- `MINOR_BRUTALITY` - 10% Weapon Damage, Cast trigger, 20s duration
- `MINOR_SORCERY` - 10% Spell Damage, Cast trigger, 20s duration

---

## Reference Files

- `src/domain/skill.rs` - SkillData struct
- `src/domain/skill_damage.rs` - SkillDamage struct
- `src/domain/hit_damage.rs` - HitDamage struct
- `src/domain/dot_damage.rs` - DotDamage struct
- `src/domain/passive.rs` - PassiveData struct
- `src/domain/bonus.rs` - BonusData struct
- `src/data/types.rs` - All enums (ClassName, SkillLineName, DamageType, etc.)
- `src/data/bonuses/unique.rs` - Predefined bonus constants

---

## Verification

After adding skills and passives, run:

```bash
cargo check
```

Compare output against existing skills in `src/data/skills/<class>.rs` and passives in `src/data/passives/<class>.rs` to ensure consistent formatting and structure.
