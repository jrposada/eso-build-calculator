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
        SkillData::new(
            "Rapid Fire",
            "Rapid Fire",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![DotDamage::new(17415.0, 4.0, DamageFlags::physical_single())]),
            Resource::Ultimate,
        )
        .with_channel_time(4.0),
        SkillData::new(
            "Ballista",
            "Rapid Fire",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![DotDamage::new(15587.0, 5.0, DamageFlags::physical_single())]),
            Resource::Ultimate,
        ),
        // Toxic Barrage: Channel damage + additional poison DoT (9990 over 8s after 1s delay)
        SkillData::new(
            "Toxic Barrage",
            "Rapid Fire",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![
                DotDamage::new(17415.0, 4.0, DamageFlags::poison_single()),
                DotDamage::new(9990.0, 8.0, DamageFlags::poison_single()).with_delay(1.0),
            ]),
            Resource::Ultimate,
        )
        .with_channel_time(4.0),
        // Snipe line
        SkillData::new(
            "Snipe",
            "Snipe",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(2404.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        ),
        // Focused Aim: Applies Sundered status (Major Breach)
        SkillData::new(
            "Focused Aim",
            "Snipe",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(2404.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BREACH.clone()]),
        SkillData::new(
            "Lethal Arrow",
            "Snipe",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(2483.0, DamageFlags::poison_single())]),
            Resource::Stamina,
        ),
        // Volley line
        SkillData::new(
            "Volley",
            "Volley",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![DotDamage::new(342.0, 8.0, DamageFlags::physical_aoe())
                .with_delay(2.0)
                .with_interval(1.0)]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Arrow Barrage",
            "Volley",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![DotDamage::new(460.0, 8.0, DamageFlags::physical_aoe())
                .with_delay(2.0)
                .with_interval(1.0)]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Endless Hail",
            "Volley",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![DotDamage::new(343.0, 13.0, DamageFlags::physical_aoe())
                .with_delay(2.0)
                .with_interval(1.0)]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Thunderous Volley",
            "Volley",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_dots(vec![
                DotDamage::new(343.0, 13.0, DamageFlags::physical_aoe())
                    .with_delay(2.0)
                    .with_interval(1.0),
                DotDamage::new(526.0, 13.0, DamageFlags::physical_aoe())
                    .with_delay(2.0)
                    .with_interval(1.0)
                    .with_flat_increase_per_tick(191.0)
                    .ignores_modifier(),
            ]),
            Resource::Stamina,
        ),
        // Scatter Shot line
        SkillData::new(
            "Scatter Shot",
            "Scatter Shot",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(1392.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Draining Shot",
            "Scatter Shot",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(1393.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Magnum Shot",
            "Scatter Shot",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(1727.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        ),
        // Arrow Spray line
        SkillData::new(
            "Arrow Spray",
            "Arrow Spray",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(1742.0, DamageFlags::physical_aoe())]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Acid Spray",
            "Arrow Spray",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1742.0, DamageFlags::poison_aoe())])
                .with_dots(vec![DotDamage::new(1635.0, 5.0, DamageFlags::poison_aoe())]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Bombard",
            "Arrow Spray",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new().with_hits(vec![HitDamage::new(1742.0, DamageFlags::physical_aoe())]),
            Resource::Stamina,
        ),
        // Poison Arrow line
        SkillData::new(
            "Poison Arrow",
            "Poison Arrow",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1161.0, DamageFlags::poison_single())])
                .with_dots(vec![DotDamage::new(3470.0, 20.0, DamageFlags::poison_single())]),
            Resource::Stamina,
        ),
        // Poison Injection: Deals up to 120% more damage to enemies under 50% Health
        SkillData::new(
            "Poison Injection",
            "Poison Arrow",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1161.0, DamageFlags::poison_single())])
                .with_dots(vec![DotDamage::new(3470.0, 20.0, DamageFlags::poison_single())]),
            Resource::Stamina,
        )
        .with_execute(1.2, 0.50, ExecuteScaling::Linear),
        // Venom Arrow: Grants Major Brutality and Major Sorcery for 20s
        SkillData::new(
            "Venom Arrow",
            "Poison Arrow",
            ClassName::Weapon,
            SkillLineName::Bow,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1161.0, DamageFlags::poison_single())])
                .with_dots(vec![DotDamage::new(3470.0, 20.0, DamageFlags::poison_single())]),
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
        // === TWO HANDED ===
        // Ultimate - Berserker Strike line
        SkillData::new(
            "Berserker Strike",
            "Berserker Strike",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(3486.0, DamageFlags::physical_aoe())]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Berserker Rage",
            "Berserker Strike",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(3600.0, DamageFlags::physical_aoe())]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Onslaught",
            "Berserker Strike",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(3485.0, DamageFlags::physical_aoe())]),
            Resource::Ultimate,
        ),
        // Uppercut line
        SkillData::new(
            "Uppercut",
            "Uppercut",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(2672.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Dizzying Swing",
            "Uppercut",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(2760.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        ),
        // Wrecking Blow: Grants Empower and Major Berserk for 3s
        SkillData::new(
            "Wrecking Blow",
            "Uppercut",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(2760.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        )
        .with_bonuses(vec![
            EMPOWER.clone().with_duration(3.0),
            MAJOR_BERSERK.clone().with_duration(3.0),
        ]),
        // Critical Charge line
        SkillData::new(
            "Critical Charge",
            "Critical Charge",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(1392.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Critical Rush",
            "Critical Charge",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(1393.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Stampede",
            "Critical Charge",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1393.0, DamageFlags::physical_aoe())])
                .with_dots(vec![DotDamage::new(319.0, 15.0, DamageFlags::physical_aoe()).with_interval(1.0)]),
            Resource::Stamina,
        ),
        // Cleave line
        SkillData::new(
            "Cleave",
            "Cleave",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(1742.0, DamageFlags::physical_aoe())]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Brawler",
            "Cleave",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(1742.0, DamageFlags::physical_aoe())]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Carve",
            "Cleave",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1742.0, DamageFlags::bleed_aoe())])
                .with_dots(vec![DotDamage::new(2868.0, 12.0, DamageFlags::bleed_aoe())]),
            Resource::Stamina,
        ),
        // Reverse Slash line - Execute abilities
        // Reverse Slash: Deals up to 300% more damage to enemies below 50% Health
        SkillData::new(
            "Reverse Slash",
            "Reverse Slash",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(1161.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        )
        .with_execute(3.0, 0.50, ExecuteScaling::Linear),
        // Executioner: Deals up to 400% more damage to enemies below 50% Health
        SkillData::new(
            "Executioner",
            "Reverse Slash",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(1161.0, DamageFlags::bleed_single())]),
            Resource::Stamina,
        )
        .with_execute(4.0, 0.50, ExecuteScaling::Linear),
        // Reverse Slice: Deals up to 300% more damage to enemies below 50% Health
        SkillData::new(
            "Reverse Slice",
            "Reverse Slash",
            ClassName::Weapon,
            SkillLineName::TwoHanded,
            SkillDamage::new().with_hits(vec![HitDamage::new(1199.0, DamageFlags::physical_aoe())]),
            Resource::Stamina,
        )
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
        SkillData::new(
            "Elemental Storm",
            "Elemental Storm",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_dots(vec![DotDamage::new(1742.0, 7.0, DamageFlags::magic_aoe()).with_interval(1.0)]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Elemental Rage",
            "Elemental Storm",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_dots(vec![DotDamage::new(2249.0, 7.0, DamageFlags::magic_aoe()).with_interval(1.0)]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Eye of the Storm",
            "Elemental Storm",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_dots(vec![DotDamage::new(1799.0, 7.0, DamageFlags::magic_aoe()).with_interval(1.0)]),
            Resource::Ultimate,
        ),
        // Force Shock line
        SkillData::new(
            "Force Shock",
            "Force Shock",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(2085.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Crushing Shock",
            "Force Shock",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(2088.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Force Pulse",
            "Force Shock",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(2088.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        ),
        // Wall of Elements line
        SkillData::new(
            "Wall of Elements",
            "Wall of Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_dots(vec![DotDamage::new(280.0, 8.0, DamageFlags::magic_aoe()).with_interval(1.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Elemental Blockade",
            "Wall of Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_dots(vec![DotDamage::new(281.0, 10.0, DamageFlags::magic_aoe()).with_interval(1.0)]),
            Resource::Magicka,
        ),
        // Unstable Wall of Elements: Explodes when it expires for additional damage
        SkillData::new(
            "Unstable Wall of Elements",
            "Wall of Elements",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new()
                .with_dots(vec![DotDamage::new(281.0, 8.0, DamageFlags::magic_aoe()).with_interval(1.0)])
                .with_hits(vec![HitDamage::new(1199.0, DamageFlags::magic_aoe()).with_delay(8.0)]),
            Resource::Magicka,
        ),
        // Destructive Touch line
        SkillData::new(
            "Destructive Touch",
            "Destructive Touch",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1161.0, DamageFlags::magic_single())])
                .with_dots(vec![DotDamage::new(3470.0, 20.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Destructive Clench",
            "Destructive Touch",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(1161.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Destructive Reach",
            "Destructive Touch",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1161.0, DamageFlags::magic_single())])
                .with_dots(vec![DotDamage::new(3470.0, 20.0, DamageFlags::magic_single())]),
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
        SkillData::new(
            "Impulse",
            "Impulse",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(1742.0, DamageFlags::magic_aoe())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Elemental Ring",
            "Impulse",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(1799.0, DamageFlags::magic_aoe())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Pulsar",
            "Impulse",
            ClassName::Weapon,
            SkillLineName::DestructionStaff,
            SkillDamage::new().with_hits(vec![HitDamage::new(1742.0, DamageFlags::magic_aoe())]),
            Resource::Magicka,
        ),
        // === DUAL WIELD ===
        // Ultimate - Lacerate line
        SkillData::new(
            "Lacerate",
            "Lacerate",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_dots(vec![DotDamage::new(6960.0, 8.0, DamageFlags::bleed_aoe())]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Rend",
            "Lacerate",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_dots(vec![DotDamage::new(12942.0, 16.0, DamageFlags::bleed_aoe())]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Thrive in Chaos",
            "Lacerate",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_dots(vec![DotDamage::new(6965.0, 8.0, DamageFlags::bleed_aoe())]),
            Resource::Ultimate,
        ),
        // Flurry line
        SkillData::new(
            "Flurry",
            "Flurry",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(667.0, DamageFlags::physical_single()),
                HitDamage::new(667.0, DamageFlags::physical_single()),
                HitDamage::new(667.0, DamageFlags::physical_single()),
                HitDamage::new(667.0, DamageFlags::physical_single()),
            ]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Bloodthirst",
            "Flurry",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(689.0, DamageFlags::bleed_single()),
                HitDamage::new(689.0, DamageFlags::bleed_single()),
                HitDamage::new(689.0, DamageFlags::bleed_single()),
                HitDamage::new(689.0, DamageFlags::bleed_single()),
            ]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Rapid Strikes",
            "Flurry",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(689.0, DamageFlags::physical_single()),
                HitDamage::new(689.0, DamageFlags::physical_single()),
                HitDamage::new(689.0, DamageFlags::physical_single()),
                HitDamage::new(689.0, DamageFlags::physical_single()),
            ]),
            Resource::Stamina,
        ),
        // Twin Slashes line
        SkillData::new(
            "Twin Slashes",
            "Twin Slashes",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new()
                .with_hits(vec![
                    HitDamage::new(580.0, DamageFlags::bleed_single()),
                    HitDamage::new(580.0, DamageFlags::bleed_single()),
                ])
                .with_dots(vec![DotDamage::new(3470.0, 20.0, DamageFlags::bleed_single())]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Blood Craze",
            "Twin Slashes",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new()
                .with_hits(vec![
                    HitDamage::new(580.0, DamageFlags::bleed_single()),
                    HitDamage::new(580.0, DamageFlags::bleed_single()),
                ])
                .with_dots(vec![DotDamage::new(3470.0, 20.0, DamageFlags::bleed_single())]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Rending Slashes",
            "Twin Slashes",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new()
                .with_hits(vec![
                    HitDamage::new(718.0, DamageFlags::bleed_single()),
                    HitDamage::new(718.0, DamageFlags::bleed_single()),
                ])
                .with_dots(vec![DotDamage::new(3470.0, 20.0, DamageFlags::bleed_single())]),
            Resource::Stamina,
        ),
        // Whirlwind line - Execute abilities
        // Whirlwind: Deals up to 33% more damage to enemies below 50% Health
        SkillData::new(
            "Whirlwind",
            "Whirlwind",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![HitDamage::new(1742.0, DamageFlags::physical_aoe())]),
            Resource::Stamina,
        )
        .with_execute(0.33, 0.50, ExecuteScaling::Linear),
        // Steel Tornado: Deals up to 33% more damage to enemies below 50% Health
        SkillData::new(
            "Steel Tornado",
            "Whirlwind",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![HitDamage::new(1742.0, DamageFlags::physical_aoe())]),
            Resource::Stamina,
        )
        .with_execute(0.33, 0.50, ExecuteScaling::Linear),
        // Whirling Blades: Deals up to 100% more damage to enemies below 50% Health
        SkillData::new(
            "Whirling Blades",
            "Whirlwind",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![HitDamage::new(1799.0, DamageFlags::physical_aoe())]),
            Resource::Stamina,
        )
        .with_execute(1.0, 0.50, ExecuteScaling::Linear),
        // Blade Cloak line
        SkillData::new(
            "Blade Cloak",
            "Blade Cloak",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_dots(vec![DotDamage::new(421.0, 20.0, DamageFlags::physical_aoe()).with_interval(2.0)]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Deadly Cloak",
            "Blade Cloak",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_dots(vec![DotDamage::new(567.0, 20.0, DamageFlags::physical_aoe()).with_interval(2.0)]),
            Resource::Stamina,
        ),
        SkillData::new(
            "Quick Cloak",
            "Blade Cloak",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_dots(vec![DotDamage::new(422.0, 30.0, DamageFlags::physical_aoe()).with_interval(2.0)]),
            Resource::Stamina,
        ),
        // Hidden Blade line - Buff skills (Major Brutality + Major Sorcery)
        // Hidden Blade: Major Brutality + Major Sorcery for 20s
        SkillData::new(
            "Hidden Blade",
            "Hidden Blade",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![HitDamage::new(1392.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
        // Flying Blade: Major Brutality + Major Sorcery for 40s
        SkillData::new(
            "Flying Blade",
            "Hidden Blade",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(1438.0, DamageFlags::physical_single()),
                HitDamage::new(2160.0, DamageFlags::physical_single()),
            ]),
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(40.0),
            MAJOR_SORCERY.clone().with_duration(40.0),
        ]),
        // Shrouded Daggers: Major Brutality + Major Sorcery for 20s, bounces to 3 enemies
        SkillData::new(
            "Shrouded Daggers",
            "Hidden Blade",
            ClassName::Weapon,
            SkillLineName::DualWield,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(1799.0, DamageFlags::physical_aoe()),
                HitDamage::new(1799.0, DamageFlags::physical_aoe()),
                HitDamage::new(1799.0, DamageFlags::physical_aoe()),
            ]),
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
    ]
});
