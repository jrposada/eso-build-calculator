# Add Missing Skills

Add missing skills to a skill line data file based on RAW.md.

## Arguments

- `$ARGUMENTS` - Path to the skill line file (e.g., `src/data/skills/nightblade/shadow.ts`)

## Instructions

1. Read the RAW.md file in the repository root to get the raw skill data.

2. Read the target skill file at `$ARGUMENTS` to understand:
   - The current skills already defined
   - The ESO class and skill line name being used
   - The existing code structure and formatting

3. Read `src/models/skill.ts` to understand the Skill interface and available types.

4. Cross-reference RAW.md with the existing skills to identify missing skills.

5. For each skill line in RAW.md, identify:
   - **Base skill** - The original unmodified ability
   - **Morph 1** - First morphed version (indicated by "arrow" before it)
   - **Morph 2** - Second morphed version (indicated by "arrow" before it)

6. When adding skills, follow these conventions:

   ### Resource Type
   - Ultimate abilities: `resource: 'ultimate'`
   - Magicka skills (Magic Damage, no "Converts to Stamina"): `resource: 'magicka'`
   - Stamina skills (Physical/Disease Damage, "Converts to Stamina"): `resource: 'stamina'`

   ### Damage Type
   - Magic Damage: `damageType: 'magic'`
   - Physical Damage: `damageType: 'physical'`
   - Disease Damage: `damageType: 'disease'`
   - Bleed Damage: `damageType: 'bleed'`
   - Flame Damage: `damageType: 'flame'`
   - Frost Damage: `damageType: 'frost'`
   - Poison Damage: `damageType: 'poison'`
   - If a new damage type is needed, add it to `DamageType` in `src/models/skill.ts`

   ### Target Type
   - Single target skills: `targetType: 'single'`
   - AoE skills (mentions "area", "around you", "enemies nearby"): `targetType: 'aoe'`

   ### Damage Values
   - `hits`: Array of direct damage instances. Each hit has:
     - `value`: The damage amount (required)
     - `delay`: Time in seconds before the damage occurs (optional, omit for instant damage)
   - `dot`: Damage over time per tick (e.g., "435 Magic Damage every 2 seconds")
   - `dotDuration`: Duration in seconds for the DoT
   - `dotInterval`: Time between DoT ticks in seconds
   - `dotIncreasePerTick`: Percentage increase per tick as decimal (e.g., 0.12 for "increases by 12% per tick")
   - For utility/buff/debuff skills with no damage: use `damage: {}`

7. Organize skills by their skill line (base + morphs grouped together) with comments.

8. After adding skills, run `npx tsc --noEmit` to verify there are no TypeScript errors related to your changes.

## Example Skill Structures

### Instant damage skill
```typescript
{
  name: 'Lava Whip',
  esoClass: 'Dragonknight',
  skillLine: 'ArdentFlame',
  damage: {
    hits: [{ value: 2323 }],
  },
  damageType: 'flame',
  targetType: 'single',
  resource: 'magicka',
},
```

### Delayed damage skill (like Scorch)
```typescript
{
  name: 'Scorch',
  esoClass: 'Warden',
  skillLine: 'AnimalCompanions',
  damage: {
    hits: [
      { value: 2509, delay: 3 },   // First hit after 3 seconds
      { value: 3486, delay: 9 },   // Second hit after 9 seconds
    ],
  },
  damageType: 'magic',
  targetType: 'aoe',
  resource: 'magicka',
},
```

### Channeled skill with multiple hits
```typescript
{
  name: 'Inhale',
  esoClass: 'Dragonknight',
  skillLine: 'DraconicPower',
  damage: {
    hits: [
      { value: 870 },              // Instant damage
      { value: 1742, delay: 2.5 }, // Exhale damage after channel
    ],
  },
  damageType: 'flame',
  targetType: 'aoe',
  resource: 'magicka',
  channelTime: 2.5,
},
```

### Direct damage + DoT skill
```typescript
{
  name: 'Searing Strike',
  esoClass: 'Dragonknight',
  skillLine: 'ArdentFlame',
  damage: {
    hits: [{ value: 1161 }],
    dot: 3470,
    dotDuration: 20,
  },
  damageType: 'flame',
  targetType: 'single',
  resource: 'magicka',
},
```

### Pure DoT skill (no direct damage)
```typescript
{
  name: 'Swarm',
  esoClass: 'Warden',
  skillLine: 'AnimalCompanions',
  damage: {
    dot: 4631,
    dotDuration: 20,
  },
  damageType: 'magic',
  targetType: 'single',
  resource: 'magicka',
},
```

### DoT with increasing damage per tick
```typescript
{
  name: 'Ritual of Retribution',
  esoClass: 'Templar',
  skillLine: 'RestoringLight',
  damage: {
    dot: 435,
    dotDuration: 20,
    dotInterval: 2,
    dotIncreasePerTick: 0.12, // increases by 12% per tick
  },
  damageType: 'magic',
  targetType: 'aoe',
  resource: 'magicka',
},
```

### Utility/buff skill (no damage)
```typescript
{
  name: 'Frost Cloak',
  esoClass: 'Warden',
  skillLine: 'WintersEmbrace',
  damage: {},
  damageType: 'frost',
  targetType: 'aoe',
  resource: 'magicka',
},
```
