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
   - If a new damage type is needed (e.g., 'flame', 'frost', 'shock'), add it to `DamageType` in `src/models/skill.ts`

   ### Target Type
   - Single target skills: `targetType: 'single'`
   - AoE skills (mentions "area", "around you", "enemies nearby"): `targetType: 'aoe'`

   ### Damage Values
   - `initial`: The upfront damage number
   - `dot`: Damage over time total (e.g., "2050 Magic Damage over 5 seconds")
   - `dotDuration`: Duration in seconds for the DoT
   - For utility/buff/debuff skills with no damage: use `damage: {}`

7. Organize skills by their skill line (base + morphs grouped together) with comments.

8. After adding skills, run `npx tsc --noEmit` to verify there are no TypeScript errors related to your changes.

## Example Skill Structure

```typescript
{
  name: 'Skill Name',
  esoClass: 'Nightblade',
  skillLine: 'Shadow',
  damage: {
    initial: 1234,    // optional
    dot: 5678,        // optional
    dotDuration: 10,  // optional, in seconds
    dotInterval: 2,  // optional, in seconds
  },
  damageType: 'magic',
  targetType: 'single',
  resource: 'magicka',
},
```
