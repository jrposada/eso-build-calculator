use crate::data::bonuses::{
    MAJOR_BREACH, MAJOR_BRUTALITY, MAJOR_PROPHECY, MAJOR_SAVAGERY, MAJOR_SORCERY, MINOR_BERSERK,
    MINOR_BREACH, MINOR_VULNERABILITY,
};
use crate::domain::{BonusTrigger, ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static WARDEN_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        // === ANIMAL COMPANIONS ===
        // Ultimate - Feral Guardian line
        // Feral Guardian: Guardian's Wrath deals +100% damage below 25% Health
        SkillData::new(
            "Feral Guardian",
            "Feral Guardian",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.025,
            0.2625,
        )])),
        // Eternal Guardian: Guardian's Wrath deals +150% damage below 25% Health
        SkillData::new(
            "Eternal Guardian",
            "Feral Guardian",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.025825,
            0.271163,
        )])),
        // Wild Guardian: Guardian's Savagery deals +100% damage below 25% Health
        SkillData::new(
            "Wild Guardian",
            "Feral Guardian",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::bleed_single(),
            0.028407,
            0.298279,
        )])),
        // Dive line
        // Dive: Sets Off Balance for 7s when cast from >7m (conditional, not tracked)
        SkillData::new(
            "Dive",
            "Dive",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.09,
            0.945,
        )]))
        .with_spammable(),
        // Cutting Dive: Sets Off Balance for 7s when cast from >7m, adds bleed DoT
        SkillData::new(
            "Cutting Dive",
            "Dive",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::bleed_single(),
                    0.09297,
                    0.976185,
                )])
                .with_dots(vec![DotDamage::new(
                    10.0,
                    DamageFlags::bleed_single(),
                    0.009297,
                    0.097618,
                )]),
        ),
        // Screaming Cliff Racer: Sets Off Balance for 7s when cast from >7m,
        // +100 Weapon/Spell Damage for 10s (+400 when hitting Off Balance enemies)
        SkillData::new(
            "Screaming Cliff Racer",
            "Dive",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.09297,
            0.976185,
        )]))
        .with_spammable(),
        // Scorch line
        SkillData::new(
            "Scorch",
            "Scorch",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.108, 1.134).with_delay(3.0),
            HitDamage::new(DamageFlags::magic_aoe(), 0.15, 1.575).with_delay(9.0),
        ])),
        // Deep Fissure: Major Breach + Minor Breach for 10s
        SkillData::new(
            "Deep Fissure",
            "Scorch",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.111564, 1.17142).with_delay(3.0),
            HitDamage::new(DamageFlags::magic_aoe(), 0.15495, 1.62698).with_delay(9.0),
        ]))
        .with_bonuses(vec![
            MAJOR_BREACH.clone().with_duration(10.0),
            MINOR_BREACH.clone().with_duration(10.0),
        ]),
        SkillData::new(
            "Subterranean Assault",
            "Scorch",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::poison_aoe(), 0.111564, 1.17142).with_delay(3.0),
            HitDamage::new(DamageFlags::poison_aoe(), 0.111564, 1.17142).with_delay(6.0),
        ])),
        // Swarm line
        // Swarm: Minor Vulnerability for 20s
        SkillData::new(
            "Swarm",
            "Swarm",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::magic_single(),
            0.018182,
            0.19091,
        )]))
        .with_bonuses(vec![MINOR_VULNERABILITY.clone().with_duration(20.0)]),
        // Fetcher Infection: Minor Vulnerability for 20s, every second cast deals +60% damage
        SkillData::new(
            "Fetcher Infection",
            "Swarm",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::magic_single(),
            0.018782,
            0.19721,
        )]))
        .with_bonuses(vec![MINOR_VULNERABILITY.clone().with_duration(20.0)]),
        // Growing Swarm: Minor Vulnerability for 20s, spreads to nearby enemies
        SkillData::new(
            "Growing Swarm",
            "Swarm",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::bleed_single(),
            0.018782,
            0.19721,
        )]))
        .with_bonuses(vec![MINOR_VULNERABILITY.clone().with_duration(20.0)]),
        // Betty Netch line
        // Betty Netch: Major Brutality + Major Sorcery for 22s
        SkillData::new(
            "Betty Netch",
            "Betty Netch",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(22.0),
            MAJOR_SORCERY.clone().with_duration(22.0),
        ]),
        // Blue Betty: Major Brutality + Major Sorcery for 25s, cleanses 1 debuff every 5s
        // (or +5% damage for 5s if no debuff to cleanse)
        SkillData::new(
            "Blue Betty",
            "Betty Netch",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(25.0),
            MAJOR_SORCERY.clone().with_duration(25.0),
        ]),
        // Bull Netch: Major Brutality + Major Sorcery for 25s, cleanses 1 debuff every 5s
        // (or +5% damage for 5s if no debuff to cleanse)
        SkillData::new(
            "Bull Netch",
            "Betty Netch",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(25.0),
            MAJOR_SORCERY.clone().with_duration(25.0),
        ]),
        // Falcon's Swiftness line
        // Falcon's Swiftness: Major Expedition for 6s, immunity to snares/immobilizations for 4s
        SkillData::new(
            "Falcon's Swiftness",
            "Falcon's Swiftness",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        ),
        // Bird of Prey: Major Expedition for 6s, Minor Berserk while slotted
        SkillData::new(
            "Bird of Prey",
            "Falcon's Swiftness",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        )
        .with_bonuses(vec![MINOR_BERSERK
            .clone()
            .with_trigger(BonusTrigger::AbilitySlotted)]),
        // Deceptive Predator: Major Expedition for 6s, Minor Evasion while slotted (defensive)
        SkillData::new(
            "Deceptive Predator",
            "Falcon's Swiftness",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        ),
        // === GREEN BALANCE === (mostly heals, few damage skills)
        // Ultimate - Secluded Grove line (no damage)
        SkillData::new(
            "Secluded Grove",
            "Secluded Grove",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Enchanted Forest",
            "Secluded Grove",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Healing Thicket",
            "Secluded Grove",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Ultimate,
        ),
        // Fungal Growth line (no damage)
        SkillData::new(
            "Fungal Growth",
            "Fungal Growth",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        ),
        SkillData::new(
            "Enchanted Growth",
            "Fungal Growth",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        ),
        SkillData::new(
            "Soothing Spores",
            "Fungal Growth",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Stamina,
        ),
        // Healing Seed line (no damage)
        SkillData::new(
            "Healing Seed",
            "Healing Seed",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        ),
        SkillData::new(
            "Budding Seeds",
            "Healing Seed",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        ),
        // Corrupting Pollen: Major Defile, Minor Cowardice to enemies (defensive/healing reduction)
        SkillData::new(
            "Corrupting Pollen",
            "Healing Seed",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        ),
        // Living Vines line (no damage)
        SkillData::new(
            "Living Vines",
            "Living Vines",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        ),
        SkillData::new(
            "Leeching Vines",
            "Living Vines",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        ),
        SkillData::new(
            "Living Trellis",
            "Living Vines",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        ),
        // Lotus Flower line
        // Lotus Flower: Major Prophecy + Major Savagery for 20s
        SkillData::new(
            "Lotus Flower",
            "Lotus Flower",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_PROPHECY.clone().with_duration(20.0),
            MAJOR_SAVAGERY.clone().with_duration(20.0),
        ]),
        // Green Lotus: Major Prophecy + Major Savagery for 20s, heals 2 additional targets
        SkillData::new(
            "Green Lotus",
            "Lotus Flower",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_PROPHECY.clone().with_duration(20.0),
            MAJOR_SAVAGERY.clone().with_duration(20.0),
        ]),
        // Lotus Blossom: Major Prophecy + Major Savagery for 60s (1 minute)
        SkillData::new(
            "Lotus Blossom",
            "Lotus Flower",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_PROPHECY.clone().with_duration(60.0),
            MAJOR_SAVAGERY.clone().with_duration(60.0),
        ]),
        // Nature's Grasp line (no damage)
        SkillData::new(
            "Nature's Grasp",
            "Nature's Grasp",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        ),
        SkillData::new(
            "Bursting Vines",
            "Nature's Grasp",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        ),
        SkillData::new(
            "Nature's Embrace",
            "Nature's Grasp",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        ),
        // === WINTER'S EMBRACE ===
        // Ultimate - Sleet Storm line
        // Sleet Storm: Major Protection to allies (defensive)
        SkillData::new(
            "Sleet Storm",
            "Sleet Storm",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(8.0, DamageFlags::frost_aoe(), 0.05, 0.525).with_interval(1.0),
        ])),
        // Northern Storm: Major Protection, +2% damage per second up to 9 stacks (complex)
        SkillData::new(
            "Northern Storm",
            "Sleet Storm",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(8.0, DamageFlags::frost_aoe(), 0.05165, 0.542325).with_interval(1.0),
        ])),
        // Permafrost: Major Protection, Chilled status, 70% snare
        SkillData::new(
            "Permafrost",
            "Sleet Storm",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Ultimate,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(13.0, DamageFlags::frost_aoe(), 0.007059, 0.074118).with_interval(1.0),
        ])),
        // Frost Cloak line (no damage, defensive buffs)
        SkillData::new(
            "Frost Cloak",
            "Frost Cloak",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        ),
        SkillData::new(
            "Expansive Frost Cloak",
            "Frost Cloak",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        ),
        SkillData::new(
            "Ice Fortress",
            "Frost Cloak",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        ),
        // Impaling Shards line
        // Impaling Shards: Chilled status, movement slow
        // Note: Scales off Max Health, not MaxStat+MaxPower
        SkillData::new(
            "Impaling Shards",
            "Impaling Shards",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(12.0, DamageFlags::frost_aoe(), 0.0, 0.0).with_interval(1.0),
        ])),
        // Gripping Shards: Immobilize for 3s, Chilled status
        // Note: Scales off Max Health, not MaxStat+MaxPower
        SkillData::new(
            "Gripping Shards",
            "Impaling Shards",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(12.0, DamageFlags::frost_aoe(), 0.0, 0.0).with_interval(1.0),
        ])),
        // Winter's Revenge: Chilled status, +30% damage with Destruction Staff equipped
        SkillData::new(
            "Winter's Revenge",
            "Impaling Shards",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(12.0, DamageFlags::frost_aoe(), 0.012714, 0.133496).with_interval(1.0),
        ])),
        // Arctic Wind line
        // Arctic Wind: Self heal
        SkillData::new(
            "Arctic Wind",
            "Arctic Wind",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        ),
        // Arctic Blast: Instant damage + DoT, stuns after 2s delay
        SkillData::new(
            "Arctic Blast",
            "Arctic Wind",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::frost_aoe(),
                    0.077475,
                    0.813488,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::frost_aoe(),
                    0.012912,
                    0.135581,
                )
                .with_interval(2.0)]),
        ),
        // Polar Wind: Self heal + ally heal
        SkillData::new(
            "Polar Wind",
            "Arctic Wind",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        ),
        // Crystallized Shield line (no damage, defensive)
        SkillData::new(
            "Crystallized Shield",
            "Crystallized Shield",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        ),
        SkillData::new(
            "Crystallized Slab",
            "Crystallized Shield",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        ),
        // Shimmering Shield: Major Heroism for 6s on projectile absorb
        SkillData::new(
            "Shimmering Shield",
            "Crystallized Shield",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        ),
        // Frozen Gate line
        // Frozen Gate: Teleports and damages enemy after 1.5s delay
        SkillData::new(
            "Frozen Gate",
            "Frozen Gate",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_single(), 0.075, 0.7875).with_delay(1.5),
        ])),
        SkillData::new(
            "Frozen Device",
            "Frozen Gate",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_single(), 0.077475, 0.813488).with_delay(1.5),
        ])),
        SkillData::new(
            "Frozen Retreat",
            "Frozen Gate",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_single(), 0.077475, 0.813488).with_delay(1.5),
        ])),
    ]
});
