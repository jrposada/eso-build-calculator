use crate::data::bonuses::{
    EMPOWER, MAJOR_BERSERK, MAJOR_BREACH, MAJOR_BRUTALITY, MAJOR_PROPHECY, MAJOR_SAVAGERY,
    MAJOR_SORCERY, MINOR_BERSERK, MINOR_VULNERABILITY,
};
use crate::domain::{BonusData, DotDamage, ExecuteScaling, HitDamage, SkillDamage, SkillData};
use crate::domain::{BonusTarget, BonusTrigger, ClassName, DamageFlags, Resource, SkillLineName};
use once_cell::sync::Lazy;

pub static NIGHTBLADE_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        // === ASSASSINATION ===
        // Ultimate - Death Stroke line
        // Death Stroke: +20% damage from player attacks for 8s
        SkillData::new(
            "Death Stroke",
            "Death Stroke",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.16,
                1.68,
            )]),
            Resource::Ultimate,
        )
        .with_bonuses(vec![BonusData::new(
            "Death Stroke Debuff",
            BonusTrigger::Cast,
            BonusTarget::EnemyDamageTaken,
            0.20,
        )
        .with_duration(8.0)]),
        // Incapacitating Strike: +20% damage for 8s (12s with 120+ ult), stun 3s with 120+ ult
        SkillData::new(
            "Incapacitating Strike",
            "Death Stroke",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::disease_single(),
                0.16528,
                1.73544,
            )]),
            Resource::Ultimate,
        )
        .with_bonuses(vec![BonusData::new(
            "Incapacitating Strike Debuff",
            BonusTrigger::Cast,
            BonusTarget::EnemyDamageTaken,
            0.20,
        )
        .with_duration(8.0)]),
        // Soul Harvest: +20% damage for 8s, Major Defile, +10 Ultimate on kill (while slotted)
        SkillData::new(
            "Soul Harvest",
            "Death Stroke",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.16528,
                1.73544,
            )]),
            Resource::Ultimate,
        )
        .with_bonuses(vec![BonusData::new(
            "Soul Harvest Debuff",
            BonusTrigger::Cast,
            BonusTarget::EnemyDamageTaken,
            0.20,
        )
        .with_duration(8.0)]),
        // Veiled Strike line
        // Veiled Strike: Off Balance on flank (conditional, not tracked)
        SkillData::new(
            "Veiled Strike",
            "Veiled Strike",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.1,
                1.05,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        // Concealed Weapon: Off Balance on flank, Minor Expedition while slotted,
        // +10% damage for 15s after leaving stealth (conditional bonuses not tracked)
        SkillData::new(
            "Concealed Weapon",
            "Veiled Strike",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.11363,
                1.19311,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        // Surprise Attack: Sundered status (Major Breach equivalent)
        SkillData::new(
            "Surprise Attack",
            "Veiled Strike",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.1033,
                1.08465,
            )]),
            Resource::Stamina,
        )
        .with_spammable()
        .with_bonuses(vec![MAJOR_BREACH.clone()]),
        // Teleport Strike line
        // Teleport Strike: Minor Vulnerability (10s)
        SkillData::new(
            "Teleport Strike",
            "Teleport Strike",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.069,
                0.7245,
            )]),
            Resource::Magicka,
        )
        .with_bonuses(vec![MINOR_VULNERABILITY.clone()]),
        // Ambush: Minor Vulnerability (10s), Empower (10s), Minor Berserk (10s)
        SkillData::new(
            "Ambush",
            "Teleport Strike",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::physical_single(),
                0.071277,
                0.748408,
            )]),
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MINOR_VULNERABILITY.clone(),
            EMPOWER.clone(),
            MINOR_BERSERK.clone().with_duration(10.0),
        ]),
        // Lotus Fan: Minor Vulnerability (10s) to all enemies hit
        SkillData::new(
            "Lotus Fan",
            "Teleport Strike",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::magic_aoe(),
                    0.071277,
                    0.748408,
                )])
                .with_dots(vec![DotDamage::new(
                    5.0,
                    DamageFlags::magic_single(),
                    0.017683,
                    0.185666,
                )
                .with_interval(1.0)]),
            Resource::Magicka,
        )
        .with_bonuses(vec![MINOR_VULNERABILITY.clone()]),
        // Assassin's Blade line
        SkillData::new(
            "Assassin's Blade",
            "Assassin's Blade",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.05,
                0.525,
            )]),
            Resource::Magicka,
        )
        .with_spammable()
        .with_execute(3.0, 0.25, ExecuteScaling::Flat),
        SkillData::new(
            "Impale",
            "Assassin's Blade",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.05165,
                0.542325,
            )]),
            Resource::Magicka,
        )
        .with_spammable()
        .with_execute(3.3, 0.25, ExecuteScaling::Flat),
        SkillData::new(
            "Killer's Blade",
            "Assassin's Blade",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::disease_single(),
                0.05165,
                0.54233,
            )]),
            Resource::Stamina,
        )
        .with_spammable()
        .with_execute(4.0, 0.50, ExecuteScaling::Linear),
        // Mark Target line (no damage)
        // Mark Target: Major Breach (20s), heal to full on target death
        SkillData::new(
            "Mark Target",
            "Mark Target",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new(),
            Resource::Magicka,
        )
        .with_bonuses(vec![MAJOR_BREACH.clone()]),
        // Piercing Mark: Major Breach (60s), reveals stealthed enemies
        SkillData::new(
            "Piercing Mark",
            "Mark Target",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new(),
            Resource::Magicka,
        )
        .with_bonuses(vec![MAJOR_BREACH.clone().with_duration(60.0)]),
        // Reaper's Mark: Major Breach (20s), Major Berserk (10s) on target death
        SkillData::new(
            "Reaper's Mark",
            "Mark Target",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new(),
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BREACH.clone(),
            // Major Berserk granted on target death (conditional)
            MAJOR_BERSERK.clone().with_duration(10.0),
        ]),
        // Grim Focus line
        // Grim Focus: Major Prophecy + Major Savagery while slotted (+2629 crit rating)
        SkillData::new(
            "Grim Focus",
            "Grim Focus",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.18,
                1.89,
            )]),
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]),
        // Merciless Resolve: Major Prophecy + Major Savagery while slotted, 50% heal on proc
        SkillData::new(
            "Merciless Resolve",
            "Grim Focus",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.204534,
                2.14761,
            )]),
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_PROPHECY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
            MAJOR_SAVAGERY
                .clone()
                .with_trigger(BonusTrigger::AbilitySlotted),
        ]),
        // Relentless Focus: Major Prophecy + Major Savagery while slotted, 4 stacks to proc
        SkillData::new(
            "Relentless Focus",
            "Grim Focus",
            ClassName::Nightblade,
            SkillLineName::Assassination,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::disease_single(),
                0.18594,
                1.95237,
            )]),
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
        // === SHADOW ===
        // Ultimate - Consuming Darkness line
        SkillData::new(
            "Consuming Darkness",
            "Consuming Darkness",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Bolstering Darkness",
            "Consuming Darkness",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Veil of Blades",
            "Consuming Darkness",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                10.0,
                DamageFlags::magic_aoe(),
                0.06198,
                0.65079,
            )
            .with_interval(1.0)]),
            Resource::Ultimate,
        ),
        // Shadow Cloak line (no damage)
        SkillData::new(
            "Shadow Cloak",
            "Shadow Cloak",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Dark Cloak",
            "Shadow Cloak",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Shadowy Disguise",
            "Shadow Cloak",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Blur line (no damage)
        SkillData::new(
            "Blur",
            "Blur",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Mirage",
            "Blur",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Phantasmal Escape",
            "Blur",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Path of Darkness line
        SkillData::new(
            "Path of Darkness",
            "Path of Darkness",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Refreshing Path",
            "Path of Darkness",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Twisting Path",
            "Path of Darkness",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                10.0,
                DamageFlags::magic_aoe(),
                0.016278,
                0.170915,
            )
            .with_interval(1.0)]),
            Resource::Magicka,
        ),
        // Aspect of Terror line (no damage)
        SkillData::new(
            "Aspect of Terror",
            "Aspect of Terror",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Manifestation of Terror",
            "Aspect of Terror",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Mass Hysteria",
            "Aspect of Terror",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Summon Shade line
        SkillData::new(
            "Summon Shade",
            "Summon Shade",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                20.0,
                DamageFlags::magic_single(),
                0.02,
                0.21,
            )
            .with_interval(2.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Dark Shade",
            "Summon Shade",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                20.0,
                DamageFlags::magic_aoe(),
                0.026858,
                0.28201,
            )
            .with_interval(2.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Shadow Image",
            "Summon Shade",
            ClassName::Nightblade,
            SkillLineName::Shadow,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                20.0,
                DamageFlags::magic_single(),
                0.02066,
                0.21693,
            )
            .with_interval(2.0)]),
            Resource::Magicka,
        ),
        // === SIPHONING ===
        // Ultimate - Soul Shred line
        SkillData::new(
            "Soul Shred",
            "Soul Shred",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_aoe(),
                0.15,
                1.575,
            )]),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Soul Siphon",
            "Soul Shred",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new(),
            Resource::Ultimate,
        ),
        SkillData::new(
            "Soul Tether",
            "Soul Shred",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::magic_aoe(),
                    0.15495,
                    1.62698,
                )])
                .with_dots(vec![DotDamage::new(
                    8.0,
                    DamageFlags::magic_aoe(),
                    0.027,
                    0.2835,
                )
                .with_interval(1.0)]),
            Resource::Ultimate,
        ),
        // Strife line (instant damage, healing over time is out of scope)
        SkillData::new(
            "Strife",
            "Strife",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.066667,
                0.7,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        SkillData::new(
            "Funnel Health",
            "Strife",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.068867,
                0.7231,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        SkillData::new(
            "Swallow Soul",
            "Strife",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_single(),
                0.09297,
                0.976185,
            )]),
            Resource::Magicka,
        )
        .with_spammable(),
        // Malevolent Offering line (no damage)
        SkillData::new(
            "Malevolent Offering",
            "Malevolent Offering",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Healthy Offering",
            "Malevolent Offering",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Shrewd Offering",
            "Malevolent Offering",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Cripple line
        SkillData::new(
            "Cripple",
            "Cripple",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                20.0,
                DamageFlags::magic_single(),
                0.018182,
                0.19091,
            )
            .with_interval(2.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Crippling Grasp",
            "Cripple",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::magic_single(),
                    0.05165,
                    0.542325,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::magic_single(),
                    0.018782,
                    0.19721,
                )
                .with_interval(2.0)]),
            Resource::Magicka,
        ),
        SkillData::new(
            "Debilitate",
            "Cripple",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new().with_dots(vec![DotDamage::new(
                20.0,
                DamageFlags::magic_single(),
                0.018782,
                0.19721,
            )
            .with_interval(2.0)]),
            Resource::Magicka,
        ),
        // Siphoning Strikes line (no damage)
        SkillData::new(
            "Siphoning Strikes",
            "Siphoning Strikes",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Leeching Strikes",
            "Siphoning Strikes",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        SkillData::new(
            "Siphoning Attacks",
            "Siphoning Strikes",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new(),
            Resource::Magicka,
        ),
        // Drain Power line (grants Major Brutality + Major Sorcery for 30s on hit)
        SkillData::new(
            "Drain Power",
            "Drain Power",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_aoe(),
                0.075,
                0.7875,
            )]),
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(30.0),
            MAJOR_SORCERY.clone().with_duration(30.0),
        ]),
        // Power Extraction: Major Brutality + Major Sorcery (30s), Minor Courage (30s), applies Minor Cowardice (10s) to enemies
        SkillData::new(
            "Power Extraction",
            "Drain Power",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::disease_aoe(),
                0.077475,
                0.813488,
            )]),
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(30.0),
            MAJOR_SORCERY.clone().with_duration(30.0),
        ]),
        // Sap Essence: Major Brutality + Major Sorcery (30s), heals allies
        SkillData::new(
            "Sap Essence",
            "Drain Power",
            ClassName::Nightblade,
            SkillLineName::Siphoning,
            SkillDamage::new().with_hits(vec![HitDamage::new(
                DamageFlags::magic_aoe(),
                0.077475,
                0.813488,
            )]),
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(30.0),
            MAJOR_SORCERY.clone().with_duration(30.0),
        ]),
    ]
});
