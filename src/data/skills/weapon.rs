use crate::data::bonuses::unique::{
    EMPOWER, MAJOR_BERSERK, MAJOR_BREACH, MAJOR_BRUTALITY, MAJOR_SORCERY,
};
use crate::domain::{ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, ExecuteScaling, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static WEAPON_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        // === BOW ===
        // Ultimate - Rapid Fire line
        // Rapid Fire 4: <<1>> = 0.0861217 MaxStat + 0.904012 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Rapid Fire",
            "Rapid Fire",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                4.0,
                DamageFlags::physical_single(),
                0.0861217,
                0.904012,
            )]),
            Resource::Ultimate,
        )
        .with_channel_time(4.0),
        // Ballista 4: <<1>> = 0.0532809 MaxStat + 0.559899 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Ballista",
            "Rapid Fire",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                5.0,
                DamageFlags::physical_single(),
                0.0532809,
                0.559899,
            )]),
            Resource::Ultimate,
        ),
        // Toxic Barrage 4: <<1>> = 0.0860872 MaxStat + 0.904026 MaxPower (Dmg, Poison, SingleTarget, Direct)
        //                  <<3>> = 0.086083 MaxStat + 0.903875 MaxPower (Dmg, Poison, SingleTarget, DOT)
        SkillData::new(
            "Toxic Barrage",
            "Rapid Fire",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![
                DotDamage::new(4.0, DamageFlags::poison_single(), 0.0860872, 0.904026),
                DotDamage::new(8.0, DamageFlags::poison_single(), 0.086083, 0.903875)
                    .with_delay(1.0),
            ]),
            Resource::Ultimate,
        )
        .with_channel_time(4.0),
        // Snipe line
        // Snipe 4: <<1>> = 0.1035 MaxStat + 1.08675 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Snipe",
            "Snipe",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.1035,
                1.08675,
            )]),
            Resource::Stamina,
        ),
        // Focused Aim 4: <<1>> = 0.106915 MaxStat + 1.12261 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Focused Aim",
            "Snipe",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.106915,
                1.12261,
            )]),
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BREACH.clone()]),
        // Lethal Arrow 4: <<1>> = 0.106915 MaxStat + 1.12261 MaxPower (Dmg, Poison, SingleTarget, Direct)
        SkillData::new(
            "Lethal Arrow",
            "Snipe",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::poison_single(),
                0.106915,
                1.12261,
            )]),
            Resource::Stamina,
        ),
        // Volley line
        // Volley 4: <<1>> = 0.014815 MaxStat + 0.155556 MaxPower (Dmg, Physical, AOE, DOT)
        SkillData::new(
            "Volley",
            "Volley",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                8.0,
                DamageFlags::physical_aoe(),
                0.014815,
                0.155556,
            )
            .with_delay(2.0)
            .with_interval(1.0)]),
            Resource::Stamina,
        ),
        // Arrow Barrage 4: <<1>> = 0.019895 MaxStat + 0.208896 MaxPower (Dmg, Physical, AOE, DOT)
        SkillData::new(
            "Arrow Barrage",
            "Volley",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                8.0,
                DamageFlags::physical_aoe(),
                0.019895,
                0.208896,
            )
            .with_delay(2.0)
            .with_interval(1.0)]),
            Resource::Stamina,
        ),
        // Endless Hail 4: <<1>> = 0.015304 MaxStat + 0.160689 MaxPower (Dmg, Physical, AOE, DOT)
        SkillData::new(
            "Endless Hail",
            "Volley",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                13.0,
                DamageFlags::physical_aoe(),
                0.015304,
                0.160689,
            )
            .with_delay(2.0)
            .with_interval(1.0)]),
            Resource::Stamina,
        ),
        // Thunderous Volley: Uses Endless Hail coefficients for base DOT + set bonus DOT
        SkillData::new(
            "Thunderous Volley",
            "Volley",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![
                DotDamage::new(13.0, DamageFlags::physical_aoe(), 0.015304, 0.160689)
                    .with_delay(2.0)
                    .with_interval(1.0),
                DotDamage::new(13.0, DamageFlags::physical_aoe(), 0.0, 0.095636)
                    .with_delay(2.0)
                    .with_interval(1.0)
                    .with_flat_increase_per_tick(191.0)
                    .ignores_modifier(),
            ]),
            Resource::Stamina,
        ),
        // Scatter Shot line
        // Scatter Shot 4: <<1>> = 0.06 MaxStat + 0.63 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Scatter Shot",
            "Scatter Shot",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.06,
                0.63,
            )]),
            Resource::Stamina,
        ),
        // Draining Shot 4: <<1>> = 0.06198 MaxStat + 0.65079 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Draining Shot",
            "Scatter Shot",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.06198,
                0.65079,
            )]),
            Resource::Stamina,
        ),
        // Magnum Shot 4: <<1>> = 0.074376 MaxStat + 0.780948 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Magnum Shot",
            "Scatter Shot",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.074376,
                0.780948,
            )]),
            Resource::Stamina,
        ),
        // Arrow Spray line
        // Arrow Spray 4: <<1>> = 0.075 MaxStat + 0.7875 MaxPower (Dmg, Physical, AOE, Direct)
        SkillData::new(
            "Arrow Spray",
            "Arrow Spray",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_aoe(),
                0.075,
                0.7875,
            )]),
            Resource::Stamina,
        ),
        // Acid Spray 4: <<1>> = 0.077476 MaxStat + 0.81349 MaxPower (Dmg, Poison, AOE, Direct)
        //               <<2>> = 0.014118 MaxStat + 0.148235 MaxPower (Dmg, Poison, SingleTarget, DOT)
        SkillData::new(
            "Acid Spray",
            "Arrow Spray",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::poison_aoe(),
                    0.077476,
                    0.81349,
                )])
                .with_dots(vec![DotDamage::new(
                    5.0,
                    DamageFlags::poison_single(),
                    0.014118,
                    0.148235,
                )]),
            Resource::Stamina,
        ),
        // Bombard 4: <<1>> = 0.077476 MaxStat + 0.81349 MaxPower (Dmg, Physical, AOE, Direct)
        SkillData::new(
            "Bombard",
            "Arrow Spray",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_aoe(),
                0.077476,
                0.81349,
            )]),
            Resource::Stamina,
        ),
        // Poison Arrow line
        // Poison Arrow 4: <<1>> = 0.05 MaxStat + 0.525 MaxPower (Dmg, Poison, SingleTarget, Direct)
        //                 <<2>> = 0.015 MaxStat + 0.1575 MaxPower (Dmg, Poison, SingleTarget, DOT)
        SkillData::new(
            "Poison Arrow",
            "Poison Arrow",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::poison_single(),
                    0.05,
                    0.525,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::poison_single(),
                    0.015,
                    0.1575,
                )]),
            Resource::Stamina,
        ),
        // Poison Injection 4: <<1>> = 0.05165 MaxStat + 0.542325 MaxPower (Dmg, Poison, SingleTarget, Direct)
        //                     <<2>> = 0.015495 MaxStat + 0.162697 MaxPower (Dmg, Poison, SingleTarget, DOT)
        // Deals up to 120% more damage to enemies under 50% Health
        SkillData::new(
            "Poison Injection",
            "Poison Arrow",
            ClassName::Weapon,
            SkillLineName::Bow,
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
                )]),
            Resource::Stamina,
        )
        .with_execute(1.2, 0.50, ExecuteScaling::Linear),
        // Venom Arrow 4: <<1>> = 0.05165 MaxStat + 0.542325 MaxPower (Dmg, Poison, SingleTarget, Direct)
        //                <<2>> = 0.015495 MaxStat + 0.162697 MaxPower (Dmg, Poison, SingleTarget, DOT)
        // Grants Major Brutality and Major Sorcery for 20s
        SkillData::new(
            "Venom Arrow",
            "Poison Arrow",
            ClassName::Weapon,
            SkillLineName::Bow,
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
                )]),
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
        // === TWO HANDED ===
        // Ultimate - Berserker Strike line
        // Berserker Strike 4: <<1>> = 0.15 MaxStat + 1.575 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Berserker Strike",
            "Berserker Strike",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_aoe(),
                0.15,
                1.575,
            )]),
            Resource::Ultimate,
        ),
        // Berserker Rage 4: <<1>> = 0.15495 MaxStat + 1.62698 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Berserker Rage",
            "Berserker Strike",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_aoe(),
                0.15495,
                1.62698,
            )]),
            Resource::Ultimate,
        ),
        // Onslaught 4: <<1>> = 0.15495 MaxStat + 1.62698 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Onslaught",
            "Berserker Strike",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_aoe(),
                0.15495,
                1.62698,
            )]),
            Resource::Ultimate,
        ),
        // Uppercut line
        // Uppercut 4: <<1>> = 0.115 MaxStat + 1.2075 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Uppercut",
            "Uppercut",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.115,
                1.2075,
            )]),
            Resource::Stamina,
        )
        .with_spammable(),
        // Dizzying Swing 4: <<1>> = 0.118795 MaxStat + 1.24735 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Dizzying Swing",
            "Uppercut",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.118795,
                1.24735,
            )]),
            Resource::Stamina,
        )
        .with_spammable(),
        // Wrecking Blow 4: <<1>> = 0.118795 MaxStat + 1.24735 MaxPower (Dmg, Physical, SingleTarget, Direct)
        // Grants Empower and Major Berserk for 3s
        SkillData::new(
            "Wrecking Blow",
            "Uppercut",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.118795,
                1.24735,
            )]),
            Resource::Stamina,
        )
        .with_spammable()
        .with_bonuses(vec![
            EMPOWER.clone().with_duration(3.0),
            MAJOR_BERSERK.clone().with_duration(3.0),
        ]),
        // Critical Charge line
        // Critical Charge 4: <<1>> = 0.06 MaxStat + 0.63 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Critical Charge",
            "Critical Charge",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.06,
                0.63,
            )]),
            Resource::Stamina,
        ),
        // Critical Rush 4: <<1>> = 0.06198 MaxStat + 0.65079 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Critical Rush",
            "Critical Charge",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.06198,
                0.65079,
            )]),
            Resource::Stamina,
        ),
        // Stampede 4: <<1>> = 0.06198 MaxStat + 0.65079 MaxPower (Dmg, Physical, AOE, Direct)
        //             <<2>> = 0.013773 MaxStat + 0.14462 MaxPower (Dmg, Physical, AOE, DOT)
        SkillData::new(
            "Stampede",
            "Critical Charge",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::physical_aoe(),
                    0.06198,
                    0.65079,
                )])
                .with_dots(vec![DotDamage::new(
                    15.0,
                    DamageFlags::physical_aoe(),
                    0.013773,
                    0.14462,
                )
                .with_interval(1.0)]),
            Resource::Stamina,
        ),
        // Cleave line
        // Cleave 4: <<1>> = 0.075 MaxStat + 0.7875 MaxPower (Dmg, Physical, AOE, Direct)
        SkillData::new(
            "Cleave",
            "Cleave",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_aoe(),
                0.075,
                0.7875,
            )]),
            Resource::Stamina,
        ),
        // Brawler 4: <<1>> = 0.077475 MaxStat + 0.81349 MaxPower (Dmg, Physical, AOE, Direct)
        SkillData::new(
            "Brawler",
            "Cleave",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_aoe(),
                0.077475,
                0.81349,
            )]),
            Resource::Stamina,
        ),
        // Carve 4: <<1>> = 0.077475 MaxStat + 0.81349 MaxPower (Dmg, Bleed, AOE, Direct)
        //          <<2>> = 0.127746 MaxStat + 1.3478 MaxPower (Dmg, Bleed, SingleTarget, DOT)
        SkillData::new(
            "Carve",
            "Cleave",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::bleed_aoe(),
                    0.077475,
                    0.81349,
                )])
                .with_dots(vec![DotDamage::new(
                    12.0,
                    DamageFlags::bleed_single(),
                    0.127746,
                    1.3478,
                )]),
            Resource::Stamina,
        ),
        // Reverse Slash line - Execute abilities
        // Reverse Slash 4: <<1>> = 0.05 MaxStat + 0.525 MaxPower (Dmg, Physical, SingleTarget, Direct)
        // Deals up to 300% more damage to enemies below 50% Health
        SkillData::new(
            "Reverse Slash",
            "Reverse Slash",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.05,
                0.525,
            )]),
            Resource::Stamina,
        )
        .with_spammable()
        .with_execute(3.0, 0.50, ExecuteScaling::Linear),
        // Executioner 4: <<1>> = 0.05165 MaxStat + 0.54233 MaxPower (Dmg, Bleed, SingleTarget, Direct)
        // Deals up to 400% more damage to enemies below 50% Health
        SkillData::new(
            "Executioner",
            "Reverse Slash",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::bleed_single(),
                0.05165,
                0.54233,
            )]),
            Resource::Stamina,
        )
        .with_spammable()
        .with_execute(4.0, 0.50, ExecuteScaling::Linear),
        // Reverse Slice 4: <<1>> = 0.05165 MaxStat + 0.54233 MaxPower (Dmg, Physical, AOE, Direct)
        // Deals up to 300% more damage to enemies below 50% Health
        SkillData::new(
            "Reverse Slice",
            "Reverse Slash",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_aoe(),
                0.05165,
                0.54233,
            )]),
            Resource::Stamina,
        )
        .with_spammable()
        .with_execute(3.0, 0.50, ExecuteScaling::Linear),
        // Momentum line - Buff skills (Major Brutality + Major Sorcery)
        // Momentum: Major Brutality + Major Sorcery for 20s
        SkillData::new(
            "Momentum",
            "Momentum",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new(),
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
        // Forward Momentum: Major Brutality + Major Sorcery for 40s
        SkillData::new(
            "Forward Momentum",
            "Momentum",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new(),
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(40.0),
            MAJOR_SORCERY.clone().with_duration(40.0),
        ]),
        // Rally: Major Brutality + Major Sorcery for 20s
        SkillData::new(
            "Rally",
            "Momentum",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new(),
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
        // === DESTRUCTION STAFF ===
        // Ultimate - Elemental Storm line
        // Elemental Storm 4: <<1>> = 0.075 MaxStat + 0.7875 MaxPower (Dmg, Magic, AOE, DOT)
        SkillData::new(
            "Elemental Storm",
            "Elemental Storm",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                7.0,
                DamageFlags::magic_aoe(),
                0.075,
                0.7875,
            )
            .with_interval(1.0)]),
            Resource::Ultimate,
        ),
        // Elemental Rage 4: <<1>> = 0.09685 MaxStat + 1.0169 MaxPower (Dmg, Magic, AOE, DOT)
        SkillData::new(
            "Elemental Rage",
            "Elemental Storm",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                7.0,
                DamageFlags::magic_aoe(),
                0.09685,
                1.0169,
            )
            .with_interval(1.0)]),
            Resource::Ultimate,
        ),
        // Eye of the Storm 4: <<1>> = 0.07748 MaxStat + 0.81349 MaxPower (Dmg, Magic, AOE, DOT)
        SkillData::new(
            "Eye of the Storm",
            "Elemental Storm",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                7.0,
                DamageFlags::magic_aoe(),
                0.07748,
                0.81349,
            )
            .with_interval(1.0)]),
            Resource::Ultimate,
        ),
        // Force Shock line
        // Force Shock 4: 3 hits (Flame + Frost + Shock), each = 0.03 MaxStat + 0.315 MaxPower
        // Total: 0.09 MaxStat + 0.945 MaxPower (Dmg, Magic, SingleTarget, Direct)
        SkillData::new(
            "Force Shock",
            "Force Shock",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.09,
                0.945,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        // Crushing Shock 4: 3 hits (Flame + Frost + Shock), each = 0.03099 MaxStat + 0.325395 MaxPower
        // Total: 0.09297 MaxStat + 0.976185 MaxPower (Dmg, Magic, SingleTarget, Direct)
        SkillData::new(
            "Crushing Shock",
            "Force Shock",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.09297,
                0.976185,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        // Force Pulse 4: 3 hits (Flame + Frost + Shock), each = 0.03099 MaxStat + 0.325395 MaxPower
        // Total: 0.09297 MaxStat + 0.976185 MaxPower (Dmg, Magic, SingleTarget, Direct)
        SkillData::new(
            "Force Pulse",
            "Force Shock",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.09297,
                0.976185,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        // Wall of Elements line
        // Wall of Elements 4: <<1>> = 0.012121 MaxStat + 0.127273 MaxPower (Dmg, Magic, AOE, DOT)
        SkillData::new(
            "Wall of Elements",
            "Wall of Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                8.0,
                DamageFlags::magic_aoe(),
                0.012121,
                0.127273,
            )
            .with_interval(1.0)]),
            Resource::Magicka,
        ),
        // Elemental Blockade 4: <<1>> = 0.012521 MaxStat + 0.131473 MaxPower (Dmg, Magic, AOE, DOT)
        SkillData::new(
            "Elemental Blockade",
            "Wall of Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                10.0,
                DamageFlags::magic_aoe(),
                0.012521,
                0.131473,
            )
            .with_interval(1.0)]),
            Resource::Magicka,
        ),
        // Unstable Wall of Elements 4: <<1>> = 0.012521 MaxStat + 0.131473 MaxPower (Dmg, Magic, AOE, DOT)
        //                               <<3>> = 0.05165 MaxStat + 0.542325 MaxPower (Dmg, Magic, AOE, Direct)
        SkillData::new(
            "Unstable Wall of Elements",
            "Wall of Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new()
                .with_dots(vec![DotDamage::new(
                    8.0,
                    DamageFlags::magic_aoe(),
                    0.012521,
                    0.131473,
                )
                .with_interval(1.0)])
                .with_hits(vec![HitDamage::new(
                    DamageFlags::magic_aoe(),
                    0.05165,
                    0.542325,
                )
                .with_delay(8.0)]),
            Resource::Magicka,
        ),
        // Destructive Touch line
        // Destructive Touch 4: <<1>> = 0.05 MaxStat + 0.525 MaxPower (Dmg, Magic, SingleTarget, Direct)
        //                      <<2>> = 0.015 MaxStat + 0.1575 MaxPower (Dmg, Magic, SingleTarget, DOT)
        SkillData::new(
            "Destructive Touch",
            "Destructive Touch",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::magic_single(),
                    0.05,
                    0.525,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::magic_single(),
                    0.015,
                    0.1575,
                )]),
            Resource::Magicka,
        ),
        // Destructive Clench 4: <<1>> = 0.05165 MaxStat + 0.542325 MaxPower (Dmg, Magic, SingleTarget, Direct)
        SkillData::new(
            "Destructive Clench",
            "Destructive Touch",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.05165,
                0.542325,
            )]),
            Resource::Magicka,
        ),
        // Destructive Reach 4: <<1>> = 0.05165 MaxStat + 0.542325 MaxPower (Dmg, Magic, SingleTarget, Direct)
        //                      <<2>> = 0.015495 MaxStat + 0.162697 MaxPower (Dmg, Flame, SingleTarget, DOT)
        SkillData::new(
            "Destructive Reach",
            "Destructive Touch",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::magic_single(),
                    0.05165,
                    0.542325,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::magic_single(),
                    0.015495,
                    0.162697,
                )]),
            Resource::Magicka,
        ),
        // Weakness to Elements line - Debuff skills (Major Breach)
        // Weakness to Elements: Major Breach for 30s
        SkillData::new(
            "Weakness to Elements",
            "Weakness to Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new(),
            Resource::Magicka,
        )
        .with_bonuses(vec![MAJOR_BREACH.clone().with_duration(30.0)]),
        // Elemental Drain: Major Breach for 60s + Minor Magickasteal
        SkillData::new(
            "Elemental Drain",
            "Weakness to Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new(),
            Resource::Magicka,
        )
        .with_bonuses(vec![MAJOR_BREACH.clone().with_duration(60.0)]),
        // Elemental Susceptibility: Major Breach for 30s + periodic status effects
        SkillData::new(
            "Elemental Susceptibility",
            "Weakness to Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new(),
            Resource::Magicka,
        )
        .with_bonuses(vec![MAJOR_BREACH.clone().with_duration(30.0)]),
        // Impulse line
        // Impulse 4: <<1>> = 0.075 MaxStat + 0.7875 MaxPower (Dmg, Magic, AOE, Direct)
        SkillData::new(
            "Impulse",
            "Impulse",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_aoe(),
                0.075,
                0.7875,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        // Elemental Ring 4: <<1>> = 0.077475 MaxStat + 0.813488 MaxPower (Dmg, Magic, AOE, Direct)
        SkillData::new(
            "Elemental Ring",
            "Impulse",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_aoe(),
                0.077475,
                0.813488,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        // Pulsar 4: <<1>> = 0.077475 MaxStat + 0.813488 MaxPower (Dmg, Magic, AOE, Direct)
        SkillData::new(
            "Pulsar",
            "Impulse",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_aoe(),
                0.077475,
                0.813488,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        // === DUAL WIELD ===
        // Ultimate - Lacerate line
        // Lacerate 4: <<1>> = 0.06 MaxStat + 0.63 MaxPower (Dmg, Bleed, SingleTarget, DOT)
        SkillData::new(
            "Lacerate",
            "Lacerate",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                8.0,
                DamageFlags::bleed_aoe(),
                0.06,
                0.63,
            )]),
            Resource::Ultimate,
        ),
        // Rend 4: <<1>> = 0.06198 MaxStat + 0.65079 MaxPower (Dmg, Bleed, SingleTarget, DOT)
        SkillData::new(
            "Rend",
            "Lacerate",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                16.0,
                DamageFlags::bleed_aoe(),
                0.06198,
                0.65079,
            )]),
            Resource::Ultimate,
        ),
        // Thrive in Chaos 4: <<1>> = 0.06198 MaxStat + 0.65079 MaxPower (Dmg, Bleed, SingleTarget, DOT)
        SkillData::new(
            "Thrive in Chaos",
            "Lacerate",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                8.0,
                DamageFlags::bleed_aoe(),
                0.06198,
                0.65079,
            )]),
            Resource::Ultimate,
        ),
        // Flurry line
        // Flurry 4: <<1>> = 0.02875 MaxStat + 0.301875 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Flurry",
            "Flurry",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(DamageFlags::physical_single(), 0.02875, 0.301875),
                HitDamage::new(DamageFlags::physical_single(), 0.02875, 0.301875),
                HitDamage::new(DamageFlags::physical_single(), 0.02875, 0.301875),
                HitDamage::new(DamageFlags::physical_single(), 0.02875, 0.301875),
            ]),
            Resource::Stamina,
        )
        .with_spammable(),
        // Bloodthirst 4: <<1>> = 0.0297 MaxStat + 0.31184 MaxPower (Dmg, Bleed, SingleTarget, Direct)
        SkillData::new(
            "Bloodthirst",
            "Flurry",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(DamageFlags::bleed_single(), 0.0297, 0.31184),
                HitDamage::new(DamageFlags::bleed_single(), 0.0297, 0.31184),
                HitDamage::new(DamageFlags::bleed_single(), 0.0297, 0.31184),
                HitDamage::new(DamageFlags::bleed_single(), 0.0297, 0.31184),
            ]),
            Resource::Stamina,
        )
        .with_spammable(),
        // Rapid Strikes 4: <<1>> = 0.0297 MaxStat + 0.31184 MaxPower (Dmg, Physical, SingleTarget, Direct)
        SkillData::new(
            "Rapid Strikes",
            "Flurry",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(DamageFlags::physical_single(), 0.0297, 0.31184),
                HitDamage::new(DamageFlags::physical_single(), 0.0297, 0.31184),
                HitDamage::new(DamageFlags::physical_single(), 0.0297, 0.31184),
                HitDamage::new(DamageFlags::physical_single(), 0.0297, 0.31184),
            ]),
            Resource::Stamina,
        )
        .with_spammable(),
        // Twin Slashes line
        // Twin Slashes 4: <<1>> = 0.025 MaxStat + 0.2625 MaxPower (Dmg, Bleed, SingleTarget, Direct)
        //                 <<2>> = 0.015 MaxStat + 0.1575 MaxPower (Dmg, Bleed, SingleTarget, DOT)
        SkillData::new(
            "Twin Slashes",
            "Twin Slashes",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new()
                .with_hits(vec![
                    HitDamage::new(DamageFlags::bleed_single(), 0.025, 0.2625),
                    HitDamage::new(DamageFlags::bleed_single(), 0.025, 0.2625),
                ])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::bleed_single(),
                    0.015,
                    0.1575,
                )]),
            Resource::Stamina,
        ),
        // Blood Craze 4: <<1>> = 0.025825 MaxStat + 0.271163 MaxPower (Dmg, Bleed, SingleTarget, Direct)
        //                <<2>> = 0.015495 MaxStat + 0.162697 MaxPower (Dmg, Bleed, SingleTarget, DOT)
        SkillData::new(
            "Blood Craze",
            "Twin Slashes",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new()
                .with_hits(vec![
                    HitDamage::new(DamageFlags::bleed_single(), 0.025825, 0.271163),
                    HitDamage::new(DamageFlags::bleed_single(), 0.025825, 0.271163),
                ])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::bleed_single(),
                    0.015495,
                    0.162697,
                )]),
            Resource::Stamina,
        ),
        // Rending Slashes 4: <<1>> = 0.03099 MaxStat + 0.325395 MaxPower (Dmg, Bleed, SingleTarget, Direct)
        //                    <<2>> = 0.015495 MaxStat + 0.162697 MaxPower (Dmg, Bleed, SingleTarget, DOT)
        SkillData::new(
            "Rending Slashes",
            "Twin Slashes",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new()
                .with_hits(vec![
                    HitDamage::new(DamageFlags::bleed_single(), 0.03099, 0.325395),
                    HitDamage::new(DamageFlags::bleed_single(), 0.03099, 0.325395),
                ])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::bleed_single(),
                    0.015495,
                    0.162697,
                )]),
            Resource::Stamina,
        ),
        // Whirlwind line - Execute abilities
        // Whirlwind 4: <<1>> = 0.075 MaxStat + 0.7875 MaxPower (Dmg, Physical, AOE, Direct)
        // Deals up to 33% more damage to enemies below 50% Health
        SkillData::new(
            "Whirlwind",
            "Whirlwind",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_aoe(),
                0.075,
                0.7875,
            )]),
            Resource::Stamina,
        )
        .with_spammable()
        .with_execute(0.33, 0.50, ExecuteScaling::Linear),
        // Steel Tornado 4: <<1>> = 0.077476 MaxStat + 0.81349 MaxPower (Dmg, Physical, AOE, Direct)
        // Deals up to 33% more damage to enemies below 50% Health
        SkillData::new(
            "Steel Tornado",
            "Whirlwind",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_aoe(),
                0.077476,
                0.81349,
            )]),
            Resource::Stamina,
        )
        .with_spammable()
        .with_execute(0.33, 0.50, ExecuteScaling::Linear),
        // Whirling Blades 4: <<1>> = 0.077476 MaxStat + 0.81349 MaxPower (Dmg, Physical, AOE, Direct)
        // Deals up to 100% more damage to enemies below 50% Health
        SkillData::new(
            "Whirling Blades",
            "Whirlwind",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_aoe(),
                0.077476,
                0.81349,
            )]),
            Resource::Stamina,
        )
        .with_spammable()
        .with_execute(1.0, 0.50, ExecuteScaling::Linear),
        // Blade Cloak line
        // Blade Cloak 4: <<4>> = 0.018182 MaxStat + 0.19091 MaxPower (Dmg, Physical, AOE, DOT)
        SkillData::new(
            "Blade Cloak",
            "Blade Cloak",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                20.0,
                DamageFlags::physical_aoe(),
                0.018182,
                0.19091,
            )
            .with_interval(2.0)]),
            Resource::Stamina,
        ),
        // Deadly Cloak 4: <<4>> = 0.024417 MaxStat + 0.256373 MaxPower (Dmg, Physical, AOE, DOT)
        SkillData::new(
            "Deadly Cloak",
            "Blade Cloak",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                20.0,
                DamageFlags::physical_aoe(),
                0.024417,
                0.256373,
            )
            .with_interval(2.0)]),
            Resource::Stamina,
        ),
        // Quick Cloak 4: <<4>> = 0.018782 MaxStat + 0.19721 MaxPower (Dmg, Physical, AOE, DOT)
        SkillData::new(
            "Quick Cloak",
            "Blade Cloak",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                30.0,
                DamageFlags::physical_aoe(),
                0.018782,
                0.19721,
            )
            .with_interval(2.0)]),
            Resource::Stamina,
        ),
        // Hidden Blade line - Buff skills (Major Brutality + Major Sorcery)
        // Hidden Blade 4: <<1>> = 0.06 MaxStat + 0.63 MaxPower (Dmg, Physical, SingleTarget, Direct)
        // Major Brutality + Major Sorcery for 20s
        SkillData::new(
            "Hidden Blade",
            "Hidden Blade",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.06,
                0.63,
            )]),
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
        // Flying Blade 4: <<1>> = 0.06198 MaxStat + 0.65079 MaxPower (Dmg, Physical, SingleTarget, Direct)
        //                 <<4>> = 0.09297 MaxStat + 0.976185 MaxPower (Dmg, Physical, SingleTarget, Direct)
        // Major Brutality + Major Sorcery for 40s
        SkillData::new(
            "Flying Blade",
            "Hidden Blade",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(DamageFlags::physical_single(), 0.06198, 0.65079),
                HitDamage::new(DamageFlags::physical_single(), 0.09297, 0.976185),
            ]),
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(40.0),
            MAJOR_SORCERY.clone().with_duration(40.0),
        ]),
        // Shrouded Daggers 4: <<1>> = 0.077475 MaxStat + 0.813488 MaxPower (Dmg, Physical, SingleTarget, Direct)
        // Major Brutality + Major Sorcery for 20s, bounces to 3 enemies
        SkillData::new(
            "Shrouded Daggers",
            "Hidden Blade",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488),
                HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488),
                HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488),
            ]),
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
    ]
});
