use crate::data::bonuses::{EMPOWER, MAJOR_BREACH, MAJOR_BRUTALITY, MAJOR_SORCERY};
use crate::domain::{ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static TEMPLAR_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        // === AEDRIC SPEAR ===
        // Ultimate - Radial Sweep line
        SkillData::new(
            "Radial Sweep",
            "Radial Sweep",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(2323.0, DamageFlags::magic_aoe())])
                .with_dots(vec![DotDamage::new(1161.0, 6.0, DamageFlags::magic_aoe()).with_interval(2.0)]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Crescent Sweep",
            "Radial Sweep",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(2399.0, DamageFlags::magic_aoe())])
                .with_dots(vec![DotDamage::new(1161.0, 6.0, DamageFlags::magic_aoe()).with_interval(2.0)]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Everlasting Sweep",
            "Radial Sweep",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(2399.0, DamageFlags::physical_aoe())])
                .with_dots(vec![DotDamage::new(1161.0, 10.0, DamageFlags::physical_aoe()).with_interval(2.0)]),
            Resource::Ultimate,
        ),
        // Puncturing Strikes line
        SkillData::new(
            "Puncturing Strikes",
            "Puncturing Strikes",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new().with_hits(vec![HitDamage::new(889.0 * 3.0, DamageFlags::magic_aoe())]),
            Resource::Magicka,
        )
        .with_channel_time(0.8),
        // Biting Jabs: Major Brutality + Major Sorcery (10s)
        SkillData::new(
            "Biting Jabs",
            "Puncturing Strikes",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new().with_hits(vec![HitDamage::new(919.0 * 3.0, DamageFlags::physical_aoe())]),
            Resource::Stamina,
        )
        .with_channel_time(0.8)
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(10.0),
            MAJOR_SORCERY.clone().with_duration(10.0),
        ]),
        SkillData::new(
            "Puncturing Sweep",
            "Puncturing Strikes",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new().with_hits(vec![HitDamage::new(919.0 * 3.0, DamageFlags::magic_aoe())]),
            Resource::Magicka,
        )
        .with_channel_time(0.8),
        // Piercing Javelin line
        SkillData::new(
            "Piercing Javelin",
            "Piercing Javelin",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new().with_hits(vec![HitDamage::new(1392.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Aurora Javelin",
            "Piercing Javelin",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new().with_hits(vec![HitDamage::new(1438.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Binding Javelin",
            "Piercing Javelin",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new().with_hits(vec![HitDamage::new(1393.0, DamageFlags::physical_single())]),
            Resource::Stamina,
        ),
        // Focused Charge line
        SkillData::new(
            "Focused Charge",
            "Focused Charge",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new().with_hits(vec![HitDamage::new(1392.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Explosive Charge",
            "Focused Charge",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new().with_hits(vec![HitDamage::new(1799.0, DamageFlags::magic_aoe())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Toppling Charge",
            "Focused Charge",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new().with_hits(vec![HitDamage::new(1393.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        ),
        // Spear Shards line
        SkillData::new(
            "Spear Shards",
            "Spear Shards",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1742.0, DamageFlags::magic_aoe())])
                .with_dots(vec![DotDamage::new(166.0, 10.0, DamageFlags::magic_aoe()).with_interval(1.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Blazing Spear",
            "Spear Shards",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1742.0, DamageFlags::magic_aoe())])
                .with_dots(vec![DotDamage::new(276.0, 10.0, DamageFlags::magic_aoe()).with_interval(1.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Luminous Shards",
            "Spear Shards",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1742.0, DamageFlags::magic_aoe())])
                .with_dots(vec![DotDamage::new(165.0, 10.0, DamageFlags::magic_aoe()).with_interval(1.0)]),
            Resource::Magicka,
        ),
        // Sun Shield line
        SkillData::new(
            "Sun Shield",
            "Sun Shield",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new().with_hits(vec![HitDamage::new(1742.0, DamageFlags::magic_aoe())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Blazing Shield",
            "Sun Shield",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Radiant Ward",
            "Sun Shield",
            ClassName::Templar,
            SkillLineName::AedricSpear,
            SkillDamage::new().with_hits(vec![HitDamage::new(1742.0, DamageFlags::magic_aoe())]),
            Resource::Magicka,
        ),
        // === DAWN'S WRATH ===
        // Ultimate - Nova line
        SkillData::new(
            "Nova",
            "Nova",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_dots(vec![DotDamage::new(1161.0, 8.0, DamageFlags::magic_aoe()).with_interval(1.0)]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Solar Disturbance",
            "Nova",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_dots(vec![DotDamage::new(1161.0, 8.0, DamageFlags::magic_aoe()).with_interval(1.0)]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Solar Prison",
            "Nova",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_dots(vec![DotDamage::new(1199.0, 8.0, DamageFlags::magic_aoe()).with_interval(1.0)]),
            Resource::Ultimate,
        ),
        // Sun Fire line
        SkillData::new(
            "Sun Fire",
            "Sun Fire",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1161.0, DamageFlags::flame_single())])
                .with_dots(vec![DotDamage::new(3470.0, 20.0, DamageFlags::flame_single())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Reflective Light",
            "Sun Fire",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1199.0, DamageFlags::flame_aoe())])
                .with_dots(vec![DotDamage::new(3470.0, 20.0, DamageFlags::flame_aoe())]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Vampire's Bane",
            "Sun Fire",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1161.0, DamageFlags::flame_single())])
                .with_dots(vec![DotDamage::new(5370.0, 30.0, DamageFlags::flame_single())]),
            Resource::Magicka,
        ),
        // Solar Flare line
        // Solar Flare: Empower (10s) + 5% class ability damage (not tracked)
        SkillData::new(
            "Solar Flare",
            "Solar Flare",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_hits(vec![HitDamage::new(2404.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        )
        .with_bonuses(vec![EMPOWER.clone()]),
        // Dark Flare: Empower (10s) + 5% class ability damage (not tracked) + Major Defile (heal reduction, not tracked)
        SkillData::new(
            "Dark Flare",
            "Solar Flare",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_hits(vec![HitDamage::new(2483.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        )
        .with_bonuses(vec![EMPOWER.clone()]),
        // Solar Barrage: Empower (20s) + 5% class ability damage (not tracked)
        SkillData::new(
            "Solar Barrage",
            "Solar Flare",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_dots(vec![DotDamage::new(435.0, 20.0, DamageFlags::magic_aoe()).with_interval(2.0)]),
            Resource::Magicka,
        )
        .with_bonuses(vec![EMPOWER.clone().with_duration(20.0)]),
        // Backlash line
        SkillData::new(
            "Backlash",
            "Backlash",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(1161.0, DamageFlags::magic_single()),
                HitDamage::new(1284.0, DamageFlags::magic_single()).with_delay(6.0),
            ]),
            Resource::Magicka,
        ),
        // Power of the Light: Major Breach (Sundered status on each hit)
        SkillData::new(
            "Power of the Light",
            "Backlash",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(1161.0, DamageFlags::physical_single()),
                HitDamage::new(1285.0, DamageFlags::physical_single()).with_delay(6.0),
            ]),
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BREACH.clone()]),
        SkillData::new(
            "Purifying Light",
            "Backlash",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(1161.0, DamageFlags::magic_single()),
                HitDamage::new(1285.0, DamageFlags::magic_single()).with_delay(6.0),
            ]),
            Resource::Magicka,
        ),
        // Eclipse line
        SkillData::new(
            "Eclipse",
            "Eclipse",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Living Dark",
            "Eclipse",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Unstable Core",
            "Eclipse",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_hits(vec![HitDamage::new(449.0 + 898.0 + 1799.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        ),
        // Radiant Destruction line
        SkillData::new(
            "Radiant Destruction",
            "Radiant Destruction",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_hits(vec![HitDamage::new(7248.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        )
        .with_channel_time(3.8),
        SkillData::new(
            "Radiant Glory",
            "Radiant Destruction",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_hits(vec![HitDamage::new(7482.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        )
        .with_channel_time(3.8),
        SkillData::new(
            "Radiant Oppression",
            "Radiant Destruction",
            ClassName::Templar,
            SkillLineName::DawnsWrath,
            SkillDamage::new().with_hits(vec![HitDamage::new(7482.0, DamageFlags::magic_single())]),
            Resource::Magicka,
        )
        .with_channel_time(3.8),
        // === RESTORING LIGHT ===
        // Ultimate - Rite of Passage line (no damage)
        SkillData::new(
            "Rite of Passage",
            "Rite of Passage",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Ultimate,
        )
        .with_channel_time(4.0),
        SkillData::new(
            "Practiced Incantation",
            "Rite of Passage",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Ultimate,
        )
        .with_channel_time(8.0),
        SkillData::new(
            "Remembrance",
            "Rite of Passage",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Ultimate,
        )
        .with_channel_time(4.0),
        // Rushed Ceremony line (no damage)
        SkillData::new(
            "Rushed Ceremony",
            "Rushed Ceremony",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Breath of Life",
            "Rushed Ceremony",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Honor the Dead",
            "Rushed Ceremony",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Healing Ritual line (no damage)
        SkillData::new(
            "Healing Ritual",
            "Healing Ritual",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Hasty Prayer",
            "Healing Ritual",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Ritual of Rebirth",
            "Healing Ritual",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Restoring Aura line (no damage)
        SkillData::new(
            "Restoring Aura",
            "Restoring Aura",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Radiant Aura",
            "Restoring Aura",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Repentance",
            "Restoring Aura",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Cleansing Ritual line
        SkillData::new(
            "Cleansing Ritual",
            "Cleansing Ritual",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Extended Ritual",
            "Cleansing Ritual",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Ritual of Retribution",
            "Cleansing Ritual",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new().with_dots(vec![DotDamage::new(435.0, 20.0, DamageFlags::magic_aoe())
                .with_interval(2.0)
                .with_increase_per_tick(0.12)]),
            Resource::Magicka,
        ),
        // Rune Focus line (no damage)
        SkillData::new(
            "Rune Focus",
            "Rune Focus",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Channeled Focus",
            "Rune Focus",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Restoring Focus",
            "Rune Focus",
            ClassName::Templar,
            SkillLineName::RestoringLight,
            SkillDamage::new(),
            Resource::Magicka,
        ),
    ]
});
