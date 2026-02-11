use crate::data::bonuses::{MAJOR_BRUTALITY, MAJOR_PROPHECY, MAJOR_SAVAGERY, MAJOR_SORCERY};
use crate::domain::{BonusData, BonusSource, DotDamage, HitDamage, SkillDamage, SkillData};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, DamageFlags, Resource, SkillLineName};
use once_cell::sync::Lazy;

pub static SORCERER_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        // === DARK MAGIC ===
        // Ultimate - Negate Magic line
        SkillData::new(
            "Negate Magic",
            "Negate Magic",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new(),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Absorption Field",
            "Negate Magic",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new(),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Suppression Field",
            "Negate Magic",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                12.0,
                DamageFlags::magic_aoe(),
                0.044697,
                0.46932,
            )
            .with_interval(1.0)]),
            Resource::Ultimate,
        ),
        // Crystal Shard line
        SkillData::new(
            "Crystal Shard",
            "Crystal Shard",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.1035,
                1.08675,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        SkillData::new(
            "Crystal Fragments",
            "Crystal Shard",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.106915,
                1.12261,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        // Crystal Weapon: Reduces enemy armor by 1000 for 5 seconds
        SkillData::new(
            "Crystal Weapon",
            "Crystal Shard",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(DamageFlags::physical_single(), 0.09297, 0.976185),
                HitDamage::new(DamageFlags::physical_single(), 0.037188, 0.390474),
            ]),
            Resource::Stamina,
        )
        .with_spammable()
        .with_bonuses(vec![BonusData::new(
            "Crystal Weapon Armor Reduction",
            BonusSource::Skill,
            BonusTrigger::Cast,
            BonusTarget::EnemyResistanceReduction,
            1000.0,
        )
        .with_duration(5.0)]),
        // Encase line
        SkillData::new(
            "Encase",
            "Encase",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Shattering Spines",
            "Encase",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_aoe(),
                0.085222,
                0.894836,
            )
            .with_delay(4.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Vibrant Shroud",
            "Encase",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Rune Prison line
        SkillData::new(
            "Rune Prison",
            "Rune Prison",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Defensive Rune",
            "Rune Prison",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Rune Cage",
            "Rune Prison",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.077475,
                0.813488,
            )
            .with_delay(3.0)]),
            Resource::Magicka,
        ),
        // Dark Exchange line (no damage)
        SkillData::new(
            "Dark Exchange",
            "Dark Exchange",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Dark Conversion",
            "Dark Exchange",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Dark Deal",
            "Dark Exchange",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Daedric Mines line
        SkillData::new(
            "Daedric Mines",
            "Daedric Mines",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.1125,
                1.18125,
            )
            .with_delay(3.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Daedric Refuge",
            "Daedric Mines",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Daedric Tomb",
            "Daedric Mines",
            ClassName::Sorcerer,
            SkillLineName::DarkMagic,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.116213,
                1.22023,
            )]),
            Resource::Magicka,
        ),
        // === DAEDRIC SUMMONING ===
        // Ultimate - Summon Storm Atronach line
        SkillData::new(
            "Summon Storm Atronach",
            "Summon Storm Atronach",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(DamageFlags::shock_aoe(), 0.1, 1.05)])
                .with_dots(vec![DotDamage::new(
                    18.0,
                    DamageFlags::shock_single(),
                    0.05,
                    0.525,
                )
                .with_interval(1.0)]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Greater Storm Atronach",
            "Summon Storm Atronach",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(DamageFlags::shock_aoe(), 0.1, 1.05)])
                .with_dots(vec![DotDamage::new(
                    18.0,
                    DamageFlags::shock_single(),
                    0.065,
                    0.6825,
                )
                .with_interval(1.0)]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Summon Charged Atronach",
            "Summon Storm Atronach",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(DamageFlags::shock_aoe(), 0.1, 1.05)])
                .with_dots(vec![DotDamage::new(
                    18.0,
                    DamageFlags::shock_aoe(),
                    0.1,
                    1.05,
                )
                .with_interval(2.0)]),
            Resource::Ultimate,
        ),
        // Summon Unstable Familiar line
        SkillData::new(
            "Summon Unstable Familiar",
            "Summon Unstable Familiar",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                20.0,
                DamageFlags::shock_aoe(),
                0.018182,
                0.19091,
            )
            .with_interval(2.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Summon Unstable Clannfear",
            "Summon Unstable Familiar",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Summon Volatile Familiar",
            "Summon Unstable Familiar",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                20.0,
                DamageFlags::shock_aoe(),
                0.018782,
                0.19721,
            )
            .with_interval(2.0)]),
            Resource::Magicka,
        ),
        // Daedric Curse line
        SkillData::new(
            "Daedric Curse",
            "Daedric Curse",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_aoe(),
                0.125,
                1.3125,
            )
            .with_delay(6.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Daedric Prey",
            "Daedric Curse",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_aoe(),
                0.12913,
                1.35581,
            )
            .with_delay(6.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Haunting Curse",
            "Daedric Curse",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(DamageFlags::magic_aoe(), 0.12913, 1.35581).with_delay(3.5),
                HitDamage::new(DamageFlags::magic_aoe(), 0.12913, 1.35581).with_delay(12.0),
            ]),
            Resource::Magicka,
        ),
        // Summon Winged Twilight line (no damage)
        SkillData::new(
            "Summon Winged Twilight",
            "Summon Winged Twilight",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Summon Twilight Matriarch",
            "Summon Winged Twilight",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Summon Twilight Tormentor",
            "Summon Winged Twilight",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Conjured Ward line (no damage)
        SkillData::new(
            "Conjured Ward",
            "Conjured Ward",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Hardened Ward",
            "Conjured Ward",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Regenerative Ward",
            "Conjured Ward",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Bound Armor line
        SkillData::new(
            "Bound Armor",
            "Bound Armor",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Bound Aegis",
            "Bound Armor",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Bound Armaments: While slotted grants Major Prophecy + Major Savagery
        SkillData::new(
            "Bound Armaments",
            "Bound Armor",
            ClassName::Sorcerer,
            SkillLineName::DaedricSummoning,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(DamageFlags::physical_single(), 0.037188, 0.390474),
                HitDamage::new(DamageFlags::physical_single(), 0.037188, 0.390474).with_delay(0.3),
                HitDamage::new(DamageFlags::physical_single(), 0.037188, 0.390474).with_delay(0.6),
                HitDamage::new(DamageFlags::physical_single(), 0.037188, 0.390474).with_delay(0.9),
                HitDamage::new(DamageFlags::physical_single(), 0.037188, 0.390474).with_delay(1.2),
                HitDamage::new(DamageFlags::physical_single(), 0.037188, 0.390474).with_delay(1.5),
                HitDamage::new(DamageFlags::physical_single(), 0.037188, 0.390474).with_delay(1.8),
                HitDamage::new(DamageFlags::physical_single(), 0.037188, 0.390474).with_delay(2.1),
            ]),
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]),
        // === STORM CALLING ===
        // Ultimate - Overload line
        SkillData::new(
            "Overload",
            "Overload",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::shock_single(),
                0.1,
                1.05,
            )]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Energy Overload",
            "Overload",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::shock_single(),
                0.1033,
                1.08465,
            )]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Power Overload",
            "Overload",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::shock_single(),
                0.11363,
                1.19311,
            )]),
            Resource::Ultimate,
        ),
        // Mages' Fury line
        // Base 870 damage + additional 3195 when enemy drops to ≤20% HP
        SkillData::new(
            "Mages' Fury",
            "Mages' Fury",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(DamageFlags::shock_single(), 0.0375, 0.39375),
                HitDamage::new(DamageFlags::shock_single(), 0.1375, 1.44375)
                    .with_execute_threshold(0.20),
            ]),
            Resource::Magicka,
        )
        .with_spammable(),
        SkillData::new(
            "Endless Fury",
            "Mages' Fury",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(DamageFlags::shock_single(), 0.03875, 0.40675),
                HitDamage::new(DamageFlags::shock_single(), 0.142038, 1.49139)
                    .with_execute_threshold(0.20),
            ]),
            Resource::Magicka,
        )
        .with_spammable(),
        SkillData::new(
            "Mages' Wrath",
            "Mages' Fury",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(DamageFlags::shock_single(), 0.03875, 0.40675),
                HitDamage::new(DamageFlags::shock_aoe(), 0.142038, 1.49139)
                    .with_execute_threshold(0.20),
            ]),
            Resource::Magicka,
        )
        .with_spammable(),
        // Lightning Form line
        SkillData::new(
            "Lightning Form",
            "Lightning Form",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                20.0,
                DamageFlags::shock_aoe(),
                0.02,
                0.21,
            )
            .with_interval(2.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Boundless Storm",
            "Lightning Form",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                30.0,
                DamageFlags::shock_aoe(),
                0.02066,
                0.21693,
            )
            .with_interval(2.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Hurricane",
            "Lightning Form",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                20.0,
                DamageFlags::physical_aoe(),
                0.02066,
                0.21693,
            )
            .with_interval(2.0)
            .with_increase_per_tick(0.12)]),
            Resource::Stamina,
        ),
        // Lightning Splash line
        SkillData::new(
            "Lightning Splash",
            "Lightning Splash",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                10.0,
                DamageFlags::shock_aoe(),
                0.013333,
                0.14,
            )
            .with_interval(1.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Lightning Flood",
            "Lightning Splash",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                10.0,
                DamageFlags::shock_aoe(),
                0.017905,
                0.188006,
            )
            .with_interval(1.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Liquid Lightning",
            "Lightning Splash",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                15.0,
                DamageFlags::shock_aoe(),
                0.013773,
                0.14462,
            )
            .with_interval(1.0)]),
            Resource::Magicka,
        ),
        // Surge line (no damage, grants Major Brutality + Major Sorcery for 33s)
        SkillData::new(
            "Surge",
            "Surge",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new(),
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(33.0),
            MAJOR_SORCERY.clone().with_duration(33.0),
        ]),
        SkillData::new(
            "Critical Surge",
            "Surge",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new(),
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(33.0),
            MAJOR_SORCERY.clone().with_duration(33.0),
        ]),
        SkillData::new(
            "Power Surge",
            "Surge",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new(),
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(33.0),
            MAJOR_SORCERY.clone().with_duration(33.0),
        ]),
        // Bolt Escape line
        SkillData::new(
            "Bolt Escape",
            "Bolt Escape",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Ball of Lightning",
            "Bolt Escape",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Streak",
            "Bolt Escape",
            ClassName::Sorcerer,
            SkillLineName::StormCalling,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::shock_aoe(),
                0.06198,
                0.65079,
            )]),
            Resource::Magicka,
        ),
    ]
});
