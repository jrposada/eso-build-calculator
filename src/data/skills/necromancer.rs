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
        // === GRAVE LORD ===
        // Ultimate - Frozen Colossus line
        // Frozen Colossus: 3 smashes over 3s, Major Vulnerability (12s)
        SkillData::new(
            "Frozen Colossus",
            "Frozen Colossus",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_aoe(), 0.133333, 1.4),
            HitDamage::new(DamageFlags::frost_aoe(), 0.133333, 1.4).with_delay(1.0),
            HitDamage::new(DamageFlags::frost_aoe(), 0.133333, 1.4).with_delay(2.0),
        ]))
        .with_bonuses(vec![MAJOR_VULNERABILITY.clone()]),
        // Glacial Colossus: 3 smashes, stun on final, Major Vulnerability (17s)
        SkillData::new(
            "Glacial Colossus",
            "Frozen Colossus",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_aoe(), 0.137733, 1.4462),
            HitDamage::new(DamageFlags::frost_aoe(), 0.137733, 1.4462).with_delay(1.0),
            HitDamage::new(DamageFlags::frost_aoe(), 0.137733, 1.4462).with_delay(2.0),
        ]))
        .with_bonuses(vec![MAJOR_VULNERABILITY.clone().with_duration(17.0)]),
        // Pestilent Colossus: 3 smashes with increasing damage, Disease, Major Vulnerability (12s)
        SkillData::new(
            "Pestilent Colossus",
            "Frozen Colossus",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::disease_aoe(), 0.137733, 1.4462),
            HitDamage::new(DamageFlags::disease_aoe(), 0.14462, 1.51851).with_delay(1.0),
            HitDamage::new(DamageFlags::disease_aoe(), 0.151851, 1.59444).with_delay(2.0),
        ]))
        .with_bonuses(vec![MAJOR_VULNERABILITY.clone()]),
        // Flame Skull line (spammable)
        SkillData::new(
            "Flame Skull",
            "Flame Skull",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.09,
            0.945,
        )]))
        .with_spammable(),
        // Venom Skull: Poison/Stamina morph, slotted bonus counts Necro casts toward 3rd cast
        SkillData::new(
            "Venom Skull",
            "Flame Skull",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::poison_single(),
            0.09297,
            0.97619,
        )]))
        .with_spammable(),
        // Ricochet Skull: 3rd cast bounces to 2 additional enemies
        SkillData::new(
            "Ricochet Skull",
            "Flame Skull",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.09297,
            0.97619,
        )]))
        .with_spammable(),
        // Sacrificial Bones line
        // Sacrificial Bones: +15% Necromancer ability and DoT damage for 10s
        SkillData::new(
            "Sacrificial Bones",
            "Sacrificial Bones",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
        .with_bonuses(vec![BonusData::new(
            "Sacrificial Bones",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Sacrificial Bones", BonusTarget::Damage, 0.15),
        )
        .with_duration(10.0)]),
        // Blighted Blastbones: Disease AOE explosion after 2.5s delay, Major Defile (not tracked)
        SkillData::new(
            "Blighted Blastbones",
            "Sacrificial Bones",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::disease_aoe(), 0.15495, 1.62698).with_delay(2.5),
        ])),
        // Grave Lord's Sacrifice: +15% Necromancer ability and DoT damage for 20s
        SkillData::new(
            "Grave Lord's Sacrifice",
            "Sacrificial Bones",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
        .with_bonuses(vec![BonusData::new(
            "Grave Lord's Sacrifice",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusValue::new("Grave Lord's Sacrifice", BonusTarget::Damage, 0.15),
        )
        .with_duration(20.0)]),
        // Boneyard line
        // Boneyard: Frost AOE DoT 10s, Minor Vulnerability
        SkillData::new(
            "Boneyard",
            "Boneyard",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(10.0, DamageFlags::frost_aoe(), 0.012121, 0.127273).with_interval(1.0),
        ]))
        .with_bonuses(vec![MINOR_VULNERABILITY.clone()]),
        // Avid Boneyard: Self-activatable synergy
        SkillData::new(
            "Avid Boneyard",
            "Boneyard",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(10.0, DamageFlags::frost_aoe(), 0.012521, 0.131473).with_interval(1.0),
        ]))
        .with_bonuses(vec![MINOR_VULNERABILITY.clone()]),
        // Unnerving Boneyard: Major Breach + Minor Vulnerability
        SkillData::new(
            "Unnerving Boneyard",
            "Boneyard",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(10.0, DamageFlags::frost_aoe(), 0.012521, 0.131473).with_interval(1.0),
        ]))
        .with_bonuses(vec![
            MAJOR_BREACH.clone(),
            MINOR_VULNERABILITY.clone(),
        ]),
        // Skeletal Mage line (pet, attacks every 2s for 20s)
        // Skeletal Mage: Major Brutality + Major Sorcery
        SkillData::new(
            "Skeletal Mage",
            "Skeletal Mage",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(20.0, DamageFlags::shock_single(), 0.02, 0.21).with_interval(2.0),
        ]))
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]),
        // Skeletal Arcanist: Hits nearby enemies too
        SkillData::new(
            "Skeletal Arcanist",
            "Skeletal Mage",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(20.0, DamageFlags::shock_single(), 0.02066, 0.21693)
                .with_interval(2.0),
        ]))
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]),
        // Skeletal Archer: Physical/Stamina, +15% damage per hit
        SkillData::new(
            "Skeletal Archer",
            "Skeletal Mage",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(20.0, DamageFlags::physical_single(), 0.02066, 0.21693)
                .with_interval(2.0)
                .with_increase_per_tick(0.15),
        ]))
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone(),
            MAJOR_SORCERY.clone(),
        ]),
        // Shocking Siphon line (corpse siphon, AoE DoT)
        // Shocking Siphon: Major Savagery + Major Prophecy, +3% damage while slotted
        SkillData::new(
            "Shocking Siphon",
            "Shocking Siphon",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
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
        // Mystic Siphon: Adds recovery while siphoning
        SkillData::new(
            "Mystic Siphon",
            "Shocking Siphon",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Magicka,
        )
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
        // Detonating Siphon: Disease/Stamina, corpse explodes at end
        SkillData::new(
            "Detonating Siphon",
            "Shocking Siphon",
            ClassName::Necromancer,
            SkillLineName::GraveLord,
            Resource::Stamina,
        )
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
        // === BONE TYRANT ===
        // Ultimate - Bone Goliath Transformation line
        // Bone Goliath Transformation: +30000 Max Health, health-scaled attacks
        SkillData::new(
            "Bone Goliath Transformation",
            "Bone Goliath Transformation",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Ultimate,
        ),
        // Pummeling Goliath: Bash attacks hit multiple targets
        SkillData::new(
            "Pummeling Goliath",
            "Bone Goliath Transformation",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.077475,
            0.813488,
        )])),
        // Ravenous Goliath: AoE damage + heal, scales off Health
        SkillData::new(
            "Ravenous Goliath",
            "Bone Goliath Transformation",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            // Non-standard scaling: damage scales with Health, not MaxStat/MaxPower
            DotDamage::new(20.0, DamageFlags::magic_aoe(), 0.0, 0.0).with_interval(1.0),
        ])),
        // Death Scythe line
        SkillData::new(
            "Death Scythe",
            "Death Scythe",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_aoe(),
            0.075,
            0.7875,
        )])),
        // Ruinous Scythe: Bleed/Stamina, sets Off Balance
        SkillData::new(
            "Ruinous Scythe",
            "Death Scythe",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::bleed_aoe(),
            0.077475,
            0.813488,
        )])),
        // Hungry Scythe: Heals over time after hitting
        SkillData::new(
            "Hungry Scythe",
            "Death Scythe",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_aoe(),
            0.077475,
            0.813488,
        )])),
        // Bone Armor line (no damage, Major + Minor Resolve)
        SkillData::new(
            "Bone Armor",
            "Bone Armor",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        ),
        // Beckoning Armor: Pulls ranged attackers
        SkillData::new(
            "Beckoning Armor",
            "Bone Armor",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        ),
        // Summoner's Armor: Reduces pet skill costs by 15%
        SkillData::new(
            "Summoner's Armor",
            "Bone Armor",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        ),
        // Grave Grasp line
        // Grave Grasp: CC only (snare, immobilize, stun), Minor Maim
        SkillData::new(
            "Grave Grasp",
            "Grave Grasp",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        ),
        // Empowering Grasp: Major Maim + Empower to allies
        SkillData::new(
            "Empowering Grasp",
            "Grave Grasp",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
        .with_bonuses(vec![EMPOWER.clone()]),
        // Ghostly Embrace: Deals Frost damage + DoT, creates corpse
        SkillData::new(
            "Ghostly Embrace",
            "Grave Grasp",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        )
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
        // Bone Totem line (no damage, Minor Protection + Major Cowardice + fear)
        SkillData::new(
            "Bone Totem",
            "Bone Totem",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        ),
        // Remote Totem: Can be placed at range
        SkillData::new(
            "Remote Totem",
            "Bone Totem",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        ),
        // Agony Totem: Longer duration, synergy for allies
        SkillData::new(
            "Agony Totem",
            "Bone Totem",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        ),
        // Bitter Harvest line (corpse consume, no damage)
        // Bitter Harvest: Grants Ultimate, while slotted reduces damage taken by 3%
        SkillData::new(
            "Bitter Harvest",
            "Bitter Harvest",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        ),
        // Deaden Pain: Major Protection while healing
        SkillData::new(
            "Deaden Pain",
            "Bitter Harvest",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        ),
        // Necrotic Potency: +6 Ultimate per corpse consumed
        SkillData::new(
            "Necrotic Potency",
            "Bitter Harvest",
            ClassName::Necromancer,
            SkillLineName::BoneTyrant,
            Resource::Magicka,
        ),
        // === LIVING DEATH ===
        // Ultimate - Reanimate line (resurrection, no damage)
        SkillData::new(
            "Reanimate",
            "Reanimate",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Ultimate,
        ),
        // Renewing Animation: Restores resources per ally resurrected
        SkillData::new(
            "Renewing Animation",
            "Reanimate",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Ultimate,
        ),
        // Render Flesh line (heal, no damage)
        SkillData::new(
            "Render Flesh",
            "Render Flesh",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Blood Sacrifice: Consumes corpse to heal a second target
        SkillData::new(
            "Blood Sacrifice",
            "Render Flesh",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Resistant Flesh: Grants resistance to target
        SkillData::new(
            "Resistant Flesh",
            "Render Flesh",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Life amid Death line (heal, no damage)
        SkillData::new(
            "Life amid Death",
            "Life amid Death",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Renewing Undeath: Removes negative effects
        SkillData::new(
            "Renewing Undeath",
            "Life amid Death",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Enduring Undeath: Can consume additional corpses to extend duration
        SkillData::new(
            "Enduring Undeath",
            "Life amid Death",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Restoring Tether line (heal, no damage)
        SkillData::new(
            "Restoring Tether",
            "Restoring Tether",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Braided Tether: Also heals allies around you
        SkillData::new(
            "Braided Tether",
            "Restoring Tether",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Mortal Coil: Restores Magicka and Stamina while siphoning
        SkillData::new(
            "Mortal Coil",
            "Restoring Tether",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Expunge line (cleanse utility, no damage)
        SkillData::new(
            "Expunge",
            "Expunge",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Expunge and Modify: Restores resources per negative effect removed
        SkillData::new(
            "Expunge and Modify",
            "Expunge",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Hexproof: Removes up to 4 negative effects
        SkillData::new(
            "Hexproof",
            "Expunge",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Spirit Mender line (healing pet, no damage)
        SkillData::new(
            "Spirit Mender",
            "Spirit Mender",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Spirit Guardian: 10% damage transferred to spirit
        SkillData::new(
            "Spirit Guardian",
            "Spirit Mender",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
        // Intensive Mender: Shorter duration, heals 3 targets per pulse
        SkillData::new(
            "Intensive Mender",
            "Spirit Mender",
            ClassName::Necromancer,
            SkillLineName::LivingDeath,
            Resource::Magicka,
        ),
    ]
});
