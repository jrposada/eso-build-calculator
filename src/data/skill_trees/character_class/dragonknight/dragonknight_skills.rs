use crate::data::bonuses::{
    EMPOWER, MAJOR_BREACH, MAJOR_BRUTALITY, MAJOR_PROPHECY, MAJOR_SAVAGERY, MAJOR_SORCERY,
};
use crate::domain::{BonusTrigger, ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static DRAGONKNIGHT_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        SkillData::new(
            "Dragonknight Standard",
            "Dragonknight Standard",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Ultimate,
        )
            .with_skill_id(33963)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(16.0, DamageFlags::flame_aoe(), 0.0375, 0.39375).with_interval(1.0),
        ])),
        SkillData::new(
            "Shifting Standard",
            "Dragonknight Standard",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Ultimate,
        )
            .with_skill_id(33987)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(25.0, DamageFlags::flame_aoe(), 0.038737, 0.406744).with_interval(1.0),
        ])),
        SkillData::new(
            "Standard of Might",
            "Dragonknight Standard",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Ultimate,
        )
            .with_skill_id(34021)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(16.0, DamageFlags::flame_aoe(), 0.038737, 0.406744).with_interval(1.0),
        ])),
        SkillData::new(
            "Lava Whip",
            "Lava Whip",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(23811)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.1,
            1.05,
        )]))
        .with_spammable(),
        SkillData::new(
            "Flame Lash",
            "Lava Whip",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(23924)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.1033,
            1.08465,
        )]))
        .with_spammable(),
        SkillData::new(
            "Molten Whip",
            "Lava Whip",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(23819)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.1033,
            1.08465,
        )]))
        .with_spammable(),
        SkillData::new(
            "Searing Strike",
            "Searing Strike",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(23886)
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::flame_single(),
                    0.05,
                    0.525,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::flame_single(),
                    0.015,
                    0.1575,
                )]),
        ),
        SkillData::new(
            "Burning Embers",
            "Searing Strike",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(23898)
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::flame_single(),
                    0.05165,
                    0.542325,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::flame_single(),
                    0.015495,
                    0.162697,
                )]),
        ),
        SkillData::new(
            "Venomous Claw",
            "Searing Strike",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Stamina,
        )
            .with_skill_id(23915)
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::poison_single(),
                    0.05165,
                    0.542325,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::poison_single(),
                    0.015495,
                    0.162697,
                )
                .with_interval(2.0)
                .with_increase_per_tick(0.12)]),
        ),
        SkillData::new(
            "Fiery Breath",
            "Fiery Breath",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(34031)
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::flame_aoe(),
                    0.075,
                    0.7875,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::flame_single(),
                    0.0125,
                    0.13125,
                )]),
        ),
        SkillData::new(
            "Engulfing Flames",
            "Fiery Breath",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(34048)
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::flame_aoe(),
                    0.077475,
                    0.813488,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::flame_single(),
                    0.012913,
                    0.135581,
                )]),
        ),
        SkillData::new(
            "Noxious Breath",
            "Fiery Breath",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Stamina,
        )
            .with_skill_id(34039)
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::poison_aoe(),
                    0.077475,
                    0.813488,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::poison_single(),
                    0.012913,
                    0.135581,
                )]),
        )
        .with_bonuses(vec![MAJOR_BREACH.clone()]),
        SkillData::new(
            "Fiery Grip",
            "Fiery Grip",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(23744)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.06,
            0.63,
        )])),
        SkillData::new(
            "Chains of Devastation",
            "Fiery Grip",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(32229)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.06198,
            0.65079,
        )])),
        SkillData::new(
            "Unrelenting Grip",
            "Fiery Grip",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(23778)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.06198,
            0.65079,
        )])),
        SkillData::new(
            "Inferno",
            "Inferno",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(34061)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(15.0, DamageFlags::flame_single(), 0.075, 0.7875).with_interval(5.0),
        ]))
        .with_bonuses(vec![
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]),
        SkillData::new(
            "Cauterize",
            "Inferno",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(34100)
        .with_bonuses(vec![
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]),
        SkillData::new(
            "Flames of Oblivion",
            "Inferno",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            Resource::Magicka,
        )
            .with_skill_id(34080)
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            15.0,
            DamageFlags::flame_single(),
            0.077475,
            0.813488,
        )
        .with_interval(5.0)]))
        .with_bonuses(vec![
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]),
        SkillData::new(
            "Dragon Leap",
            "Dragon Leap",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Ultimate,
        )
            .with_skill_id(33658)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.1825,
            1.91625,
        )])),
        SkillData::new(
            "Ferocious Leap",
            "Dragon Leap",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Ultimate,
        )
            .with_skill_id(33679)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_aoe(),
            0.188523,
            1.97949,
        )])),
        SkillData::new(
            "Take Flight",
            "Dragon Leap",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Ultimate,
        )
            .with_skill_id(33668)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.216801,
            2.27641,
        )])),
        SkillData::new(
            "Spiked Armor",
            "Spiked Armor",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(23828),
        SkillData::new(
            "Hardened Armor",
            "Spiked Armor",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(23856),
        SkillData::new(
            "Volatile Armor",
            "Spiked Armor",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(23842)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(20.0, DamageFlags::flame_aoe(), 0.0, 0.0),
        ])),
        SkillData::new(
            "Dark Talons",
            "Dark Talons",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(32111)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_aoe(),
            0.075,
            0.7875,
        )])),
        SkillData::new(
            "Burning Talons",
            "Dark Talons",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(32123)
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::flame_aoe(),
                    0.077475,
                    0.813488,
                )])
                .with_dots(vec![DotDamage::new(
                    5.0,
                    DamageFlags::flame_single(),
                    0.014118,
                    0.148235,
                )]),
        ),
        SkillData::new(
            "Choking Talons",
            "Dark Talons",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(32135)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_aoe(),
            0.077475,
            0.813488,
        )])),
        SkillData::new(
            "Dragon Blood",
            "Dragon Blood",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(33619),
        SkillData::new(
            "Coagulating Blood",
            "Dragon Blood",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(33646),
        SkillData::new(
            "Green Dragon Blood",
            "Dragon Blood",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(33632),
        SkillData::new(
            "Protective Scale",
            "Protective Scale",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(33743),
        SkillData::new(
            "Dragon Fire Scale",
            "Protective Scale",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(33759),
        SkillData::new(
            "Protective Plate",
            "Protective Scale",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(33749),
        SkillData::new(
            "Inhale",
            "Inhale",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(33697)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::flame_aoe(), 0.0375, 0.39375),
            HitDamage::new(DamageFlags::flame_aoe(), 0.075, 0.7875).with_delay(2.5),
        ]))
        .with_channel_time(2.5),
        SkillData::new(
            "Deep Breath",
            "Inhale",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(33715)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::flame_aoe(), 0.038737, 0.406744),
            HitDamage::new(DamageFlags::flame_aoe(), 0.096844, 1.01686).with_delay(2.5),
        ]))
        .with_channel_time(2.5),
        SkillData::new(
            "Draw Essence",
            "Inhale",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            Resource::Magicka,
        )
            .with_skill_id(33732)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::flame_aoe(), 0.038737, 0.406744),
            HitDamage::new(DamageFlags::flame_aoe(), 0.077475, 0.813488).with_delay(2.5),
        ]))
        .with_channel_time(2.5),
        SkillData::new(
            "Magma Armor",
            "Magma Armor",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Ultimate,
        )
            .with_skill_id(19982)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(10.0, DamageFlags::flame_aoe(), 0.015, 0.1575).with_interval(1.0),
        ])),
        SkillData::new(
            "Corrosive Armor",
            "Magma Armor",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Ultimate,
        )
            .with_skill_id(33852)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(10.0, DamageFlags::poison_aoe(), 0.015, 0.1575).with_interval(1.0),
        ])),
        SkillData::new(
            "Magma Shell",
            "Magma Armor",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Ultimate,
        )
            .with_skill_id(33841)
        .with_damage(SkillDamage::new().with_dots(vec![
            DotDamage::new(10.0, DamageFlags::flame_aoe(), 0.015, 0.1575).with_interval(1.0),
        ])),
        SkillData::new(
            "Stonefist",
            "Stonefist",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Stamina,
        )
            .with_skill_id(32194)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.1,
            1.05,
        )]))
        .with_spammable(),
        SkillData::new(
            "Obsidian Shard",
            "Stonefist",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(32199)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.02,
            0.21,
        )])),
        SkillData::new(
            "Stone Giant",
            "Stonefist",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Stamina,
        )
            .with_skill_id(32205)
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.1033,
            1.08465,
        )]))
        .with_spammable(),
        SkillData::new(
            "Molten Weapons",
            "Molten Weapons",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(32156)
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(30.0),
            MAJOR_SORCERY.clone().with_duration(30.0),
        ]),
        SkillData::new(
            "Igneous Weapons",
            "Molten Weapons",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(32166)
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(60.0),
            MAJOR_SORCERY.clone().with_duration(60.0),
        ]),
        SkillData::new(
            "Molten Armaments",
            "Molten Weapons",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(32173)
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(30.0),
            MAJOR_SORCERY.clone().with_duration(30.0),
            EMPOWER.clone().with_duration(30.0),
        ]),
        SkillData::new(
            "Obsidian Shield",
            "Obsidian Shield",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(33866),
        SkillData::new(
            "Fragmented Shield",
            "Obsidian Shield",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(33881),
        SkillData::new(
            "Igneous Shield",
            "Obsidian Shield",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(33872),
        SkillData::new(
            "Petrify",
            "Petrify",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(33898)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::flame_single(), 0.05, 0.525).with_delay(2.5),
        ])),
        SkillData::new(
            "Fossilize",
            "Petrify",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(33921)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::flame_single(), 0.05165, 0.542325).with_delay(2.5),
        ])),
        SkillData::new(
            "Shattering Rocks",
            "Petrify",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(33944)
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::flame_single(), 0.05165, 0.542325).with_delay(2.5),
        ])),
        SkillData::new(
            "Ash Cloud",
            "Ash Cloud",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(33778),
        SkillData::new(
            "Cinder Storm",
            "Ash Cloud",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(33797),
        SkillData::new(
            "Eruption",
            "Ash Cloud",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            Resource::Magicka,
        )
            .with_skill_id(33816)
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::flame_aoe(),
                    0.077475,
                    0.813488,
                )])
                .with_dots(vec![DotDamage::new(
                    15.0,
                    DamageFlags::flame_aoe(),
                    0.013773,
                    0.14462,
                )
                .with_interval(1.0)]),
        ),
    ]
});

