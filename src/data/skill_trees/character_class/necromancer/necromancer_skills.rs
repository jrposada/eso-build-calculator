use crate::data::bonuses::{
    EMPOWER, MAJOR_BREACH, MAJOR_BRUTALITY, MAJOR_PROPHECY, MAJOR_SAVAGERY, MAJOR_SORCERY,
    MAJOR_VULNERABILITY, MINOR_VULNERABILITY,
};
use crate::domain::{BonusData, BonusSource, BonusValue, DamageFlags, DotDamage, HitDamage};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, Resource, SkillDamage, SkillData};
use crate::domain::SkillLineName;
use once_cell::sync::Lazy;

pub static NECROMANCER_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        SkillData::new(
            "Frozen Colossus",
            "Frozen Colossus",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Ultimate,
        )
            .with_skill_id(40122174)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_aoe(), 0.133333, 1.4),
            HitDamage::new(DamageFlags::frost_aoe(), 0.133333, 1.4).with_delay(1.0),
            HitDamage::new(DamageFlags::frost_aoe(), 0.133333, 1.4).with_delay(2.0),
        ]))
        .with_bonuses(vec![MAJOR_VULNERABILITY.clone()]),
        SkillData::new(
            "Glacial Colossus",
            "Frozen Colossus",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Ultimate,
        )
            .with_skill_id(40122388)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_aoe(), 0.137733, 1.4462),
            HitDamage::new(DamageFlags::frost_aoe(), 0.137733, 1.4462).with_delay(1.0),
            HitDamage::new(DamageFlags::frost_aoe(), 0.137733, 1.4462).with_delay(2.0),
        ]))
        .with_bonuses(vec![MAJOR_VULNERABILITY.clone().with_duration(17.0)]),
        SkillData::new(
            "Pestilent Colossus",
            "Frozen Colossus",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Ultimate,
        )
            .with_skill_id(40122395)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::disease_aoe(), 0.137733, 1.4462),
            HitDamage::new(DamageFlags::disease_aoe(), 0.14462, 1.51851).with_delay(1.0),
            HitDamage::new(DamageFlags::disease_aoe(), 0.151851, 1.59444).with_delay(2.0),
        ]))
        .with_bonuses(vec![MAJOR_VULNERABILITY.clone()]),
        SkillData::new(
            "Flame Skull",
            "Flame Skull",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
            .with_skill_id(40114108)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.09,
            0.945,
        )]))
        .with_spammable(),
        SkillData::new(
            "Venom Skull",
            "Flame Skull",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Stamina,
        )
            .with_skill_id(40117624)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::poison_single(),
            0.09297,
            0.97619,
        )]))
        .with_spammable(),
        SkillData::new(
            "Ricochet Skull",
            "Flame Skull",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
            .with_skill_id(40117637)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.09297,
            0.97619,
        )]))
        .with_spammable(),
        SkillData::new(
            "Sacrificial Bones",
            "Sacrificial Bones",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
            .with_skill_id(40114860)
        .with_bonuses(vec![BonusData::new(
            "Sacrificial Bones",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Sacrificial Bones", BonusTarget::Damage, 0.15),
        )
        .with_duration(10.0)]),
        SkillData::new(
            "Blighted Blastbones",
            "Sacrificial Bones",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Stamina,
        )
            .with_skill_id(40117690)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::disease_aoe(), 0.15495, 1.62698).with_delay(2.5),
        ])),
        SkillData::new(
            "Grave Lord's Sacrifice",
            "Sacrificial Bones",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
            .with_skill_id(40117749)
        .with_bonuses(vec![BonusData::new(
            "Grave Lord's Sacrifice",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Grave Lord's Sacrifice", BonusTarget::Damage, 0.15),
        )
        .with_duration(20.0)]),
        SkillData::new(
            "Boneyard",
            "Boneyard",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
            .with_skill_id(40115252)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(10.0, DamageFlags::frost_aoe(), 0.012121, 0.127273).with_interval(1.0),
        ]))
        .with_bonuses(vec![MINOR_VULNERABILITY.clone()]),
        SkillData::new(
            "Avid Boneyard",
            "Boneyard",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
            .with_skill_id(40117850)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(10.0, DamageFlags::frost_aoe(), 0.012521, 0.131473).with_interval(1.0),
        ]))
        .with_bonuses(vec![MINOR_VULNERABILITY.clone()]),
        SkillData::new(
            "Unnerving Boneyard",
            "Boneyard",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
            .with_skill_id(40117805)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(10.0, DamageFlags::frost_aoe(), 0.012521, 0.131473).with_interval(1.0),
        ]))
        .with_bonuses(vec![
            MAJOR_BREACH.clone(),
            MINOR_VULNERABILITY.clone(),
        ]),
        SkillData::new(
            "Skeletal Mage",
            "Skeletal Mage",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
            .with_skill_id(40114317)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(20.0, DamageFlags::shock_single(), 0.02, 0.21).with_interval(2.0),
        ]))
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]),
        SkillData::new(
            "Skeletal Arcanist",
            "Skeletal Mage",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
            .with_skill_id(40118726)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(20.0, DamageFlags::shock_single(), 0.02066, 0.21693)
                .with_interval(2.0),
        ]))
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]),
        SkillData::new(
            "Skeletal Archer",
            "Skeletal Mage",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Stamina,
        )
            .with_skill_id(40118680)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(20.0, DamageFlags::physical_single(), 0.02066, 0.21693)
                .with_interval(2.0)
                .with_increase_per_tick(0.15),
        ]))
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]),
        SkillData::new(
            "Shocking Siphon",
            "Shocking Siphon",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
            .with_skill_id(40115924)
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            19.333,
            DamageFlags::shock_aoe(),
            0.008888,
            0.093333,
        )
        .with_interval(0.666)]))
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone(),
            MAJOR_PROPHECY.clone(),
            BonusData::new(
                "Shocking Siphon",
                BonusSource::Skill,
                BonusTrigger::AbilitySlotted,
                BonusValue::new("Shocking Siphon", BonusTarget::Damage, 0.03),
            ),
        ]),
        SkillData::new(
            "Mystic Siphon",
            "Shocking Siphon",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
            .with_skill_id(40118008)
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            19.333,
            DamageFlags::shock_aoe(),
            0.009182,
            0.096413,
        )
        .with_interval(0.666)]))
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone(),
            MAJOR_PROPHECY.clone(),
            BonusData::new(
                "Mystic Siphon",
                BonusSource::Skill,
                BonusTrigger::AbilitySlotted,
                BonusValue::new("Mystic Siphon", BonusTarget::Damage, 0.03),
            ),
        ]),
        SkillData::new(
            "Detonating Siphon",
            "Shocking Siphon",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Stamina,
        )
            .with_skill_id(40118763)
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![
                    HitDamage::new(DamageFlags::disease_aoe(), 0.077475, 0.813488)
                        .with_delay(20.0),
                ])
                .with_dots(vec![DotDamage::new(
                    19.333,
                    DamageFlags::disease_aoe(),
                    0.009182,
                    0.096413,
                )
                .with_interval(0.666)]),
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone(),
            MAJOR_PROPHECY.clone(),
            BonusData::new(
                "Detonating Siphon",
                BonusSource::Skill,
                BonusTrigger::AbilitySlotted,
                BonusValue::new("Detonating Siphon", BonusTarget::Damage, 0.03),
            ),
        ]),
        SkillData::new(
            "Bone Goliath Transformation",
            "Bone Goliath Transformation",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Ultimate,
        )
            .with_skill_id(40115001),
        SkillData::new(
            "Pummeling Goliath",
            "Bone Goliath Transformation",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Ultimate,
        )
            .with_skill_id(40118664)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.077475,
            0.813488,
        )])),
        SkillData::new(
            "Ravenous Goliath",
            "Bone Goliath Transformation",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Ultimate,
        )
            .with_skill_id(40118279)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(20.0, DamageFlags::magic_aoe(), 0.0, 0.0).with_interval(1.0),
        ])),
        SkillData::new(
            "Death Scythe",
            "Death Scythe",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40115115)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_aoe(),
            0.075,
            0.7875,
        )])),
        SkillData::new(
            "Ruinous Scythe",
            "Death Scythe",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Stamina,
        )
            .with_skill_id(40118226)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::bleed_aoe(),
            0.077475,
            0.813488,
        )])),
        SkillData::new(
            "Hungry Scythe",
            "Death Scythe",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40118223)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_aoe(),
            0.077475,
            0.813488,
        )])),
        SkillData::new(
            "Bone Armor",
            "Bone Armor",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40115206),
        SkillData::new(
            "Beckoning Armor",
            "Bone Armor",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40118237),
        SkillData::new(
            "Summoner's Armor",
            "Bone Armor",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40118244),
        SkillData::new(
            "Grave Grasp",
            "Grave Grasp",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40115177),
        SkillData::new(
            "Empowering Grasp",
            "Grave Grasp",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40118352)
        .with_bonuses(vec![EMPOWER.clone()]),
        SkillData::new(
            "Ghostly Embrace",
            "Grave Grasp",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40118308)
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::frost_aoe(),
                    0.038737,
                    0.406744,
                )])
                .with_dots(vec![DotDamage::new(
                    4.0,
                    DamageFlags::frost_single(),
                    0.014118,
                    0.148235,
                )
                .with_interval(1.0)]),
        ),
        SkillData::new(
            "Bone Totem",
            "Bone Totem",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40115093),
        SkillData::new(
            "Remote Totem",
            "Bone Totem",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40118380),
        SkillData::new(
            "Agony Totem",
            "Bone Totem",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40118404),
        SkillData::new(
            "Bitter Harvest",
            "Bitter Harvest",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40115238),
        SkillData::new(
            "Deaden Pain",
            "Bitter Harvest",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40118623),
        SkillData::new(
            "Necrotic Potency",
            "Bitter Harvest",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
            .with_skill_id(40118639),
        SkillData::new(
            "Reanimate",
            "Reanimate",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Renewing Animation",
            "Reanimate",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Ultimate,
        )
            .with_skill_id(40118367),
        SkillData::new(
            "Render Flesh",
            "Render Flesh",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40114196),
        SkillData::new(
            "Blood Sacrifice",
            "Render Flesh",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40117888),
        SkillData::new(
            "Resistant Flesh",
            "Render Flesh",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40117883),
        SkillData::new(
            "Life amid Death",
            "Life amid Death",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40115315),
        SkillData::new(
            "Renewing Undeath",
            "Life amid Death",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40118017),
        SkillData::new(
            "Enduring Undeath",
            "Life amid Death",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40118809),
        SkillData::new(
            "Restoring Tether",
            "Restoring Tether",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40115926),
        SkillData::new(
            "Braided Tether",
            "Restoring Tether",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40118070),
        SkillData::new(
            "Mortal Coil",
            "Restoring Tether",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40118122),
        SkillData::new(
            "Expunge",
            "Expunge",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40115307),
        SkillData::new(
            "Expunge and Modify",
            "Expunge",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40117940),
        SkillData::new(
            "Hexproof",
            "Expunge",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40117919),
        SkillData::new(
            "Spirit Mender",
            "Spirit Mender",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40115710),
        SkillData::new(
            "Spirit Guardian",
            "Spirit Mender",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40118912),
        SkillData::new(
            "Intensive Mender",
            "Spirit Mender",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        )
            .with_skill_id(40118840),
    ]
});

