use crate::data::bonuses::{
    MAJOR_BREACH, MAJOR_BRUTALITY, MAJOR_PROPHECY, MAJOR_SAVAGERY, MAJOR_SORCERY, MINOR_BERSERK,
    MINOR_BREACH, MINOR_VULNERABILITY,
};
use crate::domain::{BonusTrigger, ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static WARDEN_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        SkillData::new(
            "Feral Guardian",
            "Feral Guardian",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Ultimate,
        )
            .with_skill_id(85985)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.025,
            0.2625,
        )])),
        SkillData::new(
            "Eternal Guardian",
            "Feral Guardian",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Ultimate,
        )
            .with_skill_id(85989)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.025825,
            0.271163,
        )])),
        SkillData::new(
            "Wild Guardian",
            "Feral Guardian",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Ultimate,
        )
            .with_skill_id(85993)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::bleed_single(),
            0.028407,
            0.298279,
        )])),
        SkillData::new(
            "Dive",
            "Dive",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
            .with_skill_id(85998)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.09,
            0.945,
        )]))
        .with_spammable(),
        SkillData::new(
            "Cutting Dive",
            "Dive",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        )
            .with_skill_id(86002)
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
        SkillData::new(
            "Screaming Cliff Racer",
            "Dive",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
            .with_skill_id(86006)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.09297,
            0.976185,
        )]))
        .with_spammable(),
        SkillData::new(
            "Scorch",
            "Scorch",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
            .with_skill_id(93593)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::magic_aoe(), 0.108, 1.134).with_delay(3.0),
            HitDamage::new(DamageFlags::magic_aoe(), 0.15, 1.575).with_delay(9.0),
        ])),
        SkillData::new(
            "Deep Fissure",
            "Scorch",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
            .with_skill_id(93778)
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
            .with_skill_id(93791)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::poison_aoe(), 0.111564, 1.17142).with_delay(3.0),
            HitDamage::new(DamageFlags::poison_aoe(), 0.111564, 1.17142).with_delay(6.0),
        ])),
        SkillData::new(
            "Swarm",
            "Swarm",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
            .with_skill_id(86026)
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::magic_single(),
            0.018182,
            0.19091,
        )]))
        .with_bonuses(vec![MINOR_VULNERABILITY.clone().with_duration(20.0)]),
        SkillData::new(
            "Fetcher Infection",
            "Swarm",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
            .with_skill_id(86030)
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::magic_single(),
            0.018782,
            0.19721,
        )]))
        .with_bonuses(vec![MINOR_VULNERABILITY.clone().with_duration(20.0)]),
        SkillData::new(
            "Growing Swarm",
            "Swarm",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        )
            .with_skill_id(86034)
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::bleed_single(),
            0.018782,
            0.19721,
        )]))
        .with_bonuses(vec![MINOR_VULNERABILITY.clone().with_duration(20.0)]),
        SkillData::new(
            "Betty Netch",
            "Betty Netch",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
            .with_skill_id(86053)
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(22.0),
            MAJOR_SORCERY.clone().with_duration(22.0),
        ]),
        SkillData::new(
            "Blue Betty",
            "Betty Netch",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Magicka,
        )
            .with_skill_id(86057)
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(25.0),
            MAJOR_SORCERY.clone().with_duration(25.0),
        ]),
        SkillData::new(
            "Bull Netch",
            "Betty Netch",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        )
            .with_skill_id(86061)
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(25.0),
            MAJOR_SORCERY.clone().with_duration(25.0),
        ]),
        SkillData::new(
            "Falcon's Swiftness",
            "Falcon's Swiftness",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        )
            .with_skill_id(86040),
        SkillData::new(
            "Bird of Prey",
            "Falcon's Swiftness",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        )
            .with_skill_id(86048)
        .with_bonuses(vec![MINOR_BERSERK
            .clone()
            .with_trigger(BonusTrigger::AbilitySlotted)]),
        SkillData::new(
            "Deceptive Predator",
            "Falcon's Swiftness",
            ClassName::Warden,
            SkillLineName::AnimalCompanions,
            Resource::Stamina,
        )
            .with_skill_id(86044),
        SkillData::new(
            "Secluded Grove",
            "Secluded Grove",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Ultimate,
        )
            .with_skill_id(93968),
        SkillData::new(
            "Enchanted Forest",
            "Secluded Grove",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Ultimate,
        )
            .with_skill_id(93971),
        SkillData::new(
            "Healing Thicket",
            "Secluded Grove",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Ultimate,
        )
            .with_skill_id(93974),
        SkillData::new(
            "Fungal Growth",
            "Fungal Growth",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93771),
        SkillData::new(
            "Enchanted Growth",
            "Fungal Growth",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93774),
        SkillData::new(
            "Soothing Spores",
            "Fungal Growth",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Stamina,
        )
            .with_skill_id(93777),
        SkillData::new(
            "Healing Seed",
            "Healing Seed",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93804),
        SkillData::new(
            "Budding Seeds",
            "Healing Seed",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93807),
        SkillData::new(
            "Corrupting Pollen",
            "Healing Seed",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93810),
        SkillData::new(
            "Living Vines",
            "Living Vines",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93877),
        SkillData::new(
            "Leeching Vines",
            "Living Vines",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93880),
        SkillData::new(
            "Living Trellis",
            "Living Vines",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93883),
        SkillData::new(
            "Lotus Flower",
            "Lotus Flower",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93908)
        .with_bonuses(vec![
            MAJOR_PROPHECY.clone().with_duration(20.0),
            MAJOR_SAVAGERY.clone().with_duration(20.0),
        ]),
        SkillData::new(
            "Green Lotus",
            "Lotus Flower",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93911)
        .with_bonuses(vec![
            MAJOR_PROPHECY.clone().with_duration(20.0),
            MAJOR_SAVAGERY.clone().with_duration(20.0),
        ]),
        SkillData::new(
            "Lotus Blossom",
            "Lotus Flower",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93914)
        .with_bonuses(vec![
            MAJOR_PROPHECY.clone().with_duration(60.0),
            MAJOR_SAVAGERY.clone().with_duration(60.0),
        ]),
        SkillData::new(
            "Nature's Grasp",
            "Nature's Grasp",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93934),
        SkillData::new(
            "Bursting Vines",
            "Nature's Grasp",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93937),
        SkillData::new(
            "Nature's Embrace",
            "Nature's Grasp",
            ClassName::Warden,
            SkillLineName::GreenBalance,
            Resource::Magicka,
        )
            .with_skill_id(93940),
        SkillData::new(
            "Sleet Storm",
            "Sleet Storm",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Ultimate,
        )
            .with_skill_id(86112)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(8.0, DamageFlags::frost_aoe(), 0.05, 0.525).with_interval(1.0),
        ])),
        SkillData::new(
            "Northern Storm",
            "Sleet Storm",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Ultimate,
        )
            .with_skill_id(86116)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(8.0, DamageFlags::frost_aoe(), 0.05165, 0.542325).with_interval(1.0),
        ])),
        SkillData::new(
            "Permafrost",
            "Sleet Storm",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Ultimate,
        )
            .with_skill_id(86120)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(13.0, DamageFlags::frost_aoe(), 0.007059, 0.074118).with_interval(1.0),
        ])),
        SkillData::new(
            "Frost Cloak",
            "Frost Cloak",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86125),
        SkillData::new(
            "Expansive Frost Cloak",
            "Frost Cloak",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86129),
        SkillData::new(
            "Ice Fortress",
            "Frost Cloak",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86133),
        SkillData::new(
            "Impaling Shards",
            "Impaling Shards",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86164)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(12.0, DamageFlags::frost_aoe(), 0.0, 0.0).with_interval(1.0),
        ])),
        SkillData::new(
            "Gripping Shards",
            "Impaling Shards",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86168)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(12.0, DamageFlags::frost_aoe(), 0.0, 0.0).with_interval(1.0),
        ])),
        SkillData::new(
            "Winter's Revenge",
            "Impaling Shards",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86172)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(12.0, DamageFlags::frost_aoe(), 0.012714, 0.133496).with_interval(1.0),
        ])),
        SkillData::new(
            "Arctic Wind",
            "Arctic Wind",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86151),
        SkillData::new(
            "Arctic Blast",
            "Arctic Wind",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86159)
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
        SkillData::new(
            "Polar Wind",
            "Arctic Wind",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86155),
        SkillData::new(
            "Crystallized Shield",
            "Crystallized Shield",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86138),
        SkillData::new(
            "Crystallized Slab",
            "Crystallized Shield",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86142),
        SkillData::new(
            "Shimmering Shield",
            "Crystallized Shield",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86146),
        SkillData::new(
            "Frozen Gate",
            "Frozen Gate",
            ClassName::Warden,
            SkillLineName::WintersEmbrace,
            Resource::Magicka,
        )
            .with_skill_id(86178)
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
            .with_skill_id(86182)
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
            .with_skill_id(86186)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::frost_single(), 0.077475, 0.813488).with_delay(1.5),
        ])),
    ]
});

