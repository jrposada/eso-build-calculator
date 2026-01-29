import { describe, expect, it } from 'vitest';

import { SkillData } from '../data/skills/types';
import { ClassName } from '../data/types';
import { SkillsService } from './skills-service';

function createMockSkill(
  overrides: Partial<SkillData> = {},
): SkillData<ClassName> {
  return {
    name: 'Test Skill',
    baseSkillName: 'Test Skill Base',
    className: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: { hits: [{ value: 100 }] },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
    ...overrides,
  };
}

describe('SkillsService', () => {
  describe('getSkillsByClassName', () => {
    describe('basic filtering by class', () => {
      it('returns skills matching the specified className', () => {
        const skills: SkillData[] = [
          createMockSkill({ name: 'DK Skill 1', className: 'Dragonknight' }),
          createMockSkill({ name: 'DK Skill 2', className: 'Dragonknight' }),
          createMockSkill({ name: 'Sorc Skill', className: 'Sorcerer' }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight');

        expect(result).toHaveLength(2);
        expect(result.map((s) => s.name)).toEqual(['DK Skill 1', 'DK Skill 2']);
      });

      it('returns empty array when no skills match the className', () => {
        const skills: SkillData[] = [
          createMockSkill({ name: 'DK Skill', className: 'Dragonknight' }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Sorcerer');

        expect(result).toEqual([]);
      });

      it('returns empty array when service has no skills', () => {
        const service = new SkillsService([]);

        const result = service.getSkillsByClassName('Dragonknight');

        expect(result).toEqual([]);
      });

      it('works with all valid class names', () => {
        const classNames: ClassName[] = [
          'Dragonknight',
          'Sorcerer',
          'Nightblade',
          'Warden',
          'Necromancer',
          'Templar',
          'Arcanist',
          'Weapon',
        ];
        const skills: SkillData[] = classNames.map((className) =>
          createMockSkill({ name: `${className} Skill`, className }),
        );
        const service = new SkillsService(skills);

        for (const className of classNames) {
          const result = service.getSkillsByClassName(className);
          expect(result).toHaveLength(1);
          expect(result[0]!.className).toBe(className);
        }
      });
    });

    describe('excludeBaseSkills option', () => {
      it('excludes skills where name equals baseSkillName when enabled', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'Fireball',
            baseSkillName: 'Fireball',
            className: 'Dragonknight',
          }),
          createMockSkill({
            name: 'Empowered Fireball',
            baseSkillName: 'Fireball',
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight', {
          excludeBaseSkills: true,
        });

        expect(result).toHaveLength(1);
        expect(result[0]!.name).toBe('Empowered Fireball');
      });

      it('includes base skills when option is false', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'Fireball',
            baseSkillName: 'Fireball',
            className: 'Dragonknight',
          }),
          createMockSkill({
            name: 'Empowered Fireball',
            baseSkillName: 'Fireball',
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight', {
          excludeBaseSkills: false,
        });

        expect(result).toHaveLength(2);
      });

      it('includes base skills when option is not provided', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'Fireball',
            baseSkillName: 'Fireball',
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight');

        expect(result).toHaveLength(1);
      });
    });

    describe('excludeUltimates option', () => {
      it('excludes skills with ultimate resource when enabled', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'Regular Skill',
            resource: 'magicka',
            className: 'Dragonknight',
          }),
          createMockSkill({
            name: 'Ultimate Skill',
            resource: 'ultimate',
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight', {
          excludeUltimates: true,
        });

        expect(result).toHaveLength(1);
        expect(result[0]!.name).toBe('Regular Skill');
      });

      it('includes ultimates when option is false', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'Ultimate Skill',
            resource: 'ultimate',
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight', {
          excludeUltimates: false,
        });

        expect(result).toHaveLength(1);
      });

      it('includes ultimates when option is not provided', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'Ultimate Skill',
            resource: 'ultimate',
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight');

        expect(result).toHaveLength(1);
      });
    });

    describe('excludeNonDamaging option', () => {
      it('excludes skills with no hits and no dots when enabled', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'Damage Skill',
            damage: { hits: [{ value: 100 }] },
            className: 'Dragonknight',
          }),
          createMockSkill({
            name: 'Buff Skill',
            damage: {},
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight', {
          excludeNonDamaging: true,
        });

        expect(result).toHaveLength(1);
        expect(result[0]!.name).toBe('Damage Skill');
      });

      it('includes skills with hits', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'Hit Skill',
            damage: { hits: [{ value: 100 }] },
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight', {
          excludeNonDamaging: true,
        });

        expect(result).toHaveLength(1);
      });

      it('includes skills with dots', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'DoT Skill',
            damage: { dots: [{ value: 50, duration: 10 }] },
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight', {
          excludeNonDamaging: true,
        });

        expect(result).toHaveLength(1);
      });

      it('includes skills with both hits and dots', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'Full Damage Skill',
            damage: {
              hits: [{ value: 100 }],
              dots: [{ value: 50, duration: 10 }],
            },
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight', {
          excludeNonDamaging: true,
        });

        expect(result).toHaveLength(1);
      });

      it('excludes skills with empty hits and dots arrays', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'Empty Damage Skill',
            damage: { hits: [], dots: [] },
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight', {
          excludeNonDamaging: true,
        });

        expect(result).toHaveLength(0);
      });

      it('includes non-damaging skills when option is false', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'Buff Skill',
            damage: {},
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight', {
          excludeNonDamaging: false,
        });

        expect(result).toHaveLength(1);
      });

      it('includes non-damaging skills when option is not provided', () => {
        const skills: SkillData[] = [
          createMockSkill({
            name: 'Buff Skill',
            damage: {},
            className: 'Dragonknight',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight');

        expect(result).toHaveLength(1);
      });
    });

    describe('combined options', () => {
      it('applies all filters together', () => {
        const skills: SkillData[] = [
          // Should be included: morphed, non-ultimate, has damage
          createMockSkill({
            name: 'Empowered Fireball',
            baseSkillName: 'Fireball',
            resource: 'magicka',
            damage: { hits: [{ value: 100 }] },
            className: 'Dragonknight',
          }),
          // Excluded: base skill
          createMockSkill({
            name: 'Fireball',
            baseSkillName: 'Fireball',
            resource: 'magicka',
            damage: { hits: [{ value: 80 }] },
            className: 'Dragonknight',
          }),
          // Excluded: ultimate
          createMockSkill({
            name: 'Morphed Ultimate',
            baseSkillName: 'Ultimate',
            resource: 'ultimate',
            damage: { hits: [{ value: 500 }] },
            className: 'Dragonknight',
          }),
          // Excluded: no damage
          createMockSkill({
            name: 'Morphed Buff',
            baseSkillName: 'Buff',
            resource: 'magicka',
            damage: {},
            className: 'Dragonknight',
          }),
          // Excluded: wrong class
          createMockSkill({
            name: 'Sorcerer Skill',
            baseSkillName: 'Sorc Base',
            resource: 'magicka',
            damage: { hits: [{ value: 100 }] },
            className: 'Sorcerer',
          }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight', {
          excludeBaseSkills: true,
          excludeUltimates: true,
          excludeNonDamaging: true,
        });

        expect(result).toHaveLength(1);
        expect(result[0]!.name).toBe('Empowered Fireball');
      });

      it('handles empty options object', () => {
        const skills: SkillData[] = [
          createMockSkill({ name: 'Skill 1', className: 'Dragonknight' }),
          createMockSkill({ name: 'Skill 2', className: 'Dragonknight' }),
        ];
        const service = new SkillsService(skills);

        const result = service.getSkillsByClassName('Dragonknight', {});

        expect(result).toHaveLength(2);
      });
    });
  });
});
