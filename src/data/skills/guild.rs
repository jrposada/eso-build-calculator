use crate::data::bonuses::unique::{
    MAJOR_BREACH, MAJOR_BRUTALITY, MAJOR_PROPHECY, MAJOR_SAVAGERY, MAJOR_SORCERY, MINOR_BERSERK,
    MINOR_FORCE, MINOR_VULNERABILITY,
};
use crate::domain::{ClassName, DamageFlags, Resource, SkillLineName};
use crate::domain::{DotDamage, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static GUILD_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        // === FIGHTERS GUILD ===
        // Ultimate - Dawnbreaker line
        // Dawnbreaker: hit 0.125/1.3125 phys_aoe + dot 0.05/0.525 phys_single 4s/2s
        SkillData::new(
            "Dawnbreaker",
            "Dawnbreaker",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Ultimate,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::physical_aoe(),
                    0.125,
                    1.3125,
                )])
                .with_dots(vec![DotDamage::new(
                    4.0,
                    DamageFlags::physical_single(),
                    0.05,
                    0.525,
                )
                .with_interval(2.0)]),
        ),
        // Flawless Dawnbreaker: hit 0.129125/1.35581 + dot 0.05165/0.542325 4s/2s + 300 W&S Damage 20s
        SkillData::new(
            "Flawless Dawnbreaker",
            "Dawnbreaker",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Ultimate,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::physical_aoe(),
                    0.129125,
                    1.35581,
                )])
                .with_dots(vec![DotDamage::new(
                    4.0,
                    DamageFlags::physical_single(),
                    0.05165,
                    0.542325,
                )
                .with_interval(2.0)]),
        )
        .with_bonuses(vec![{
            use crate::domain::{BonusData, BonusSource, BonusTarget, BonusTrigger, BonusValue};
            BonusData::new(
                "Flawless Dawnbreaker",
                BonusSource::Buff,
                BonusTrigger::Cast,
                BonusValue::new(
                    "Flawless Dawnbreaker",
                    BonusTarget::WeaponAndSpellDamageFlat,
                    300.0,
                ),
            )
            .with_duration(20.0)
        }]),
        // Dawnbreaker of Smiting: hit 0.15495/1.62698 + dot 0.06198/0.65079 4s/2s
        SkillData::new(
            "Dawnbreaker of Smiting",
            "Dawnbreaker",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Ultimate,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::physical_aoe(),
                    0.15495,
                    1.62698,
                )])
                .with_dots(vec![DotDamage::new(
                    4.0,
                    DamageFlags::physical_single(),
                    0.06198,
                    0.65079,
                )
                .with_interval(2.0)]),
        ),
        // Silver Bolts line
        // Silver Bolts: hit 0.09/0.945 phys_single
        SkillData::new(
            "Silver Bolts",
            "Silver Bolts",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.09,
            0.945,
        )])),
        // Silver Shards: hit 0.09297/0.976185 phys_aoe
        SkillData::new(
            "Silver Shards",
            "Silver Bolts",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_aoe(),
            0.09297,
            0.976185,
        )])),
        // Silver Leash: hit 0.06198/0.65079 phys_single
        SkillData::new(
            "Silver Leash",
            "Silver Bolts",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.06198,
            0.65079,
        )])),
        // Trap Beast line
        // Trap Beast: hit 0.05/0.525 bleed_single + dot 0.015/0.1575 bleed_single 18s/2s + Minor Force
        SkillData::new(
            "Trap Beast",
            "Trap Beast",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::bleed_single(),
                    0.05,
                    0.525,
                )])
                .with_dots(vec![DotDamage::new(
                    18.0,
                    DamageFlags::bleed_single(),
                    0.015,
                    0.1575,
                )
                .with_interval(2.0)]),
        )
        .with_bonuses(vec![MINOR_FORCE.clone()]),
        // Barbed Trap: hit 0.06198/0.65079 bleed_single + dot 0.015495/0.162697 bleed_single 18s/2s + Minor Force
        SkillData::new(
            "Barbed Trap",
            "Trap Beast",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::bleed_single(),
                    0.06198,
                    0.65079,
                )])
                .with_dots(vec![DotDamage::new(
                    18.0,
                    DamageFlags::bleed_single(),
                    0.015495,
                    0.162697,
                )
                .with_interval(2.0)]),
        )
        .with_bonuses(vec![MINOR_FORCE.clone()]),
        // Lightweight Beast Trap: hit 0.05165/0.542325 bleed_single + dot 0.015495/0.162697 bleed_single 18s/2s + Minor Force
        SkillData::new(
            "Lightweight Beast Trap",
            "Trap Beast",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::bleed_single(),
                    0.05165,
                    0.542325,
                )])
                .with_dots(vec![DotDamage::new(
                    18.0,
                    DamageFlags::bleed_single(),
                    0.015495,
                    0.162697,
                )
                .with_interval(2.0)]),
        )
        .with_bonuses(vec![MINOR_FORCE.clone()]),
        // Expert Hunter line - No damage, Major Savagery+Prophecy (slotted)
        SkillData::new(
            "Expert Hunter",
            "Expert Hunter",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
        ]),
        // Evil Hunter: No damage, Major Savagery+Prophecy (slotted)
        SkillData::new(
            "Evil Hunter",
            "Expert Hunter",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
        ]),
        // Camouflaged Hunter: No damage, Major Savagery+Prophecy (slotted) + Minor Berserk 5s on flank crit
        SkillData::new(
            "Camouflaged Hunter",
            "Expert Hunter",
            ClassName::Guild,
            SkillLineName::FightersGuild,
            Resource::Stamina,
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MINOR_BERSERK.clone().with_duration(5.0),
        ]),
        // === MAGES GUILD ===
        // Ultimate - Meteor line
        // Meteor: hit 0.175/1.8375 flame_aoe + dot 0.05/0.525 flame_aoe 11s/1s
        SkillData::new(
            "Meteor",
            "Meteor",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Ultimate,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::flame_aoe(),
                    0.175,
                    1.8375,
                )])
                .with_dots(vec![DotDamage::new(
                    11.0,
                    DamageFlags::flame_aoe(),
                    0.05,
                    0.525,
                )
                .with_interval(1.0)]),
        ),
        // Shooting Star: hit 0.18078/1.89814 flame_aoe + dot 0.05165/0.542325 flame_aoe 11s/1s
        SkillData::new(
            "Shooting Star",
            "Meteor",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Ultimate,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::flame_aoe(),
                    0.18078,
                    1.89814,
                )])
                .with_dots(vec![DotDamage::new(
                    11.0,
                    DamageFlags::flame_aoe(),
                    0.05165,
                    0.542325,
                )
                .with_interval(1.0)]),
        ),
        // Ice Comet: hit 0.198853/2.08795 frost_aoe + dot 0.056815/0.596557 frost_aoe 11s/1s
        SkillData::new(
            "Ice Comet",
            "Meteor",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Ultimate,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::frost_aoe(),
                    0.198853,
                    2.08795,
                )])
                .with_dots(vec![DotDamage::new(
                    11.0,
                    DamageFlags::frost_aoe(),
                    0.056815,
                    0.596557,
                )
                .with_interval(1.0)]),
        ),
        // Entropy line
        // Entropy: dot 0.018182/0.19091 magic_single 20s/2s
        SkillData::new(
            "Entropy",
            "Entropy",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::magic_single(),
            0.018182,
            0.19091,
        )
        .with_interval(2.0)])),
        // Degeneration: dot 0.018782/0.19721 magic_single 20s/2s + Major Brutality+Sorcery
        SkillData::new(
            "Degeneration",
            "Entropy",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::magic_single(),
            0.018782,
            0.19721,
        )
        .with_interval(2.0)]))
        .with_bonuses(vec![MAJOR_BRUTALITY.clone(), MAJOR_SORCERY.clone()]),
        // Structured Entropy: dot 0.018782/0.19721 magic_single 20s/2s
        SkillData::new(
            "Structured Entropy",
            "Entropy",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            20.0,
            DamageFlags::magic_single(),
            0.018782,
            0.19721,
        )
        .with_interval(2.0)])),
        // Fire Rune line
        // Fire Rune: hit 0.1/1.05 flame_aoe
        SkillData::new(
            "Fire Rune",
            "Fire Rune",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_aoe(),
            0.1,
            1.05,
        )])),
        // Volcanic Rune: hit 0.1033/1.08465 flame_aoe
        SkillData::new(
            "Volcanic Rune",
            "Fire Rune",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_aoe(),
            0.1033,
            1.08465,
        )])),
        // Scalding Rune: hit 0.1033/1.08465 flame_aoe + dot 0.01127/0.11833 flame_single 20s/2s
        SkillData::new(
            "Scalding Rune",
            "Fire Rune",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_damage(
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(
                    DamageFlags::flame_aoe(),
                    0.1033,
                    1.08465,
                )])
                .with_dots(vec![DotDamage::new(
                    20.0,
                    DamageFlags::flame_single(),
                    0.01127,
                    0.11833,
                )
                .with_interval(2.0)]),
        ),
        // Magelight line - No damage, Major Savagery+Prophecy (slotted)
        SkillData::new(
            "Magelight",
            "Magelight",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
        ]),
        SkillData::new(
            "Inner Light",
            "Magelight",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
        ]),
        SkillData::new(
            "Radiant Magelight",
            "Magelight",
            ClassName::Guild,
            SkillLineName::MagesGuild,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_SAVAGERY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
            MAJOR_PROPHECY.clone().with_trigger(crate::domain::BonusTrigger::AbilitySlotted),
        ]),
        // === UNDAUNTED ===
        // Necrotic Orb line
        // Necrotic Orb: dot 0.013636/0.143182 magic_aoe 10s/1s
        SkillData::new(
            "Necrotic Orb",
            "Necrotic Orb",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            10.0,
            DamageFlags::magic_aoe(),
            0.013636,
            0.143182,
        )
        .with_interval(1.0)])),
        // Mystic Orb: dot 0.014086/0.147907 magic_aoe 10s/1s
        SkillData::new(
            "Mystic Orb",
            "Necrotic Orb",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_dots(vec![DotDamage::new(
            10.0,
            DamageFlags::magic_aoe(),
            0.014086,
            0.147907,
        )
        .with_interval(1.0)])),
        // Energy Orb: heal only, skipped
        // Inner Fire line
        // Inner Fire: hit 0.045/0.4725 flame_single
        SkillData::new(
            "Inner Fire",
            "Inner Fire",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.045,
            0.4725,
        )])),
        // Inner Rage: hit 0.046485/0.488093 flame_single
        SkillData::new(
            "Inner Rage",
            "Inner Fire",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::flame_single(),
            0.046485,
            0.488093,
        )])),
        // Inner Beast: hit 0.09297/0.976185 phys_single + Minor Vulnerability 15s
        SkillData::new(
            "Inner Beast",
            "Inner Fire",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.09297,
            0.976185,
        )]))
        .with_bonuses(vec![MINOR_VULNERABILITY.clone().with_duration(15.0)]),
        // Trapping Webs line
        // Trapping Webs: hit 0.075/0.7875 phys_aoe + hit(delay 10s) 0.1/1.05 poison_aoe
        SkillData::new(
            "Trapping Webs",
            "Trapping Webs",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_aoe(), 0.075, 0.7875),
            HitDamage::new(DamageFlags::poison_aoe(), 0.1, 1.05).with_delay(10.0),
        ])),
        // Tangling Webs: hit 0.077475/0.813488 phys_aoe + hit(delay 10s) 0.1033/1.08465 poison_aoe
        SkillData::new(
            "Tangling Webs",
            "Trapping Webs",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488),
            HitDamage::new(DamageFlags::poison_aoe(), 0.1033, 1.08465).with_delay(10.0),
        ])),
        // Shadow Silk: hit 0.077475/0.813488 phys_aoe + hit(delay 10s) 0.1033/1.08465 poison_aoe
        SkillData::new(
            "Shadow Silk",
            "Trapping Webs",
            ClassName::Guild,
            SkillLineName::Undaunted,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![
            HitDamage::new(DamageFlags::physical_aoe(), 0.077475, 0.813488),
            HitDamage::new(DamageFlags::poison_aoe(), 0.1033, 1.08465).with_delay(10.0),
        ])),
        // === PSIJIC ORDER ===
        // Imbue Weapon line
        // Imbue Weapon: hit 0.09/0.945 phys_single (spammable)
        SkillData::new(
            "Imbue Weapon",
            "Imbue Weapon",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.09,
            0.945,
        )]))
        .with_spammable(),
        // Elemental Weapon: hit 0.09297/0.976185 magic_single (spammable)
        SkillData::new(
            "Elemental Weapon",
            "Imbue Weapon",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            Resource::Magicka,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::magic_single(),
            0.09297,
            0.976185,
        )]))
        .with_spammable(),
        // Crushing Weapon: hit 0.09297/0.976185 phys_single (spammable) + Major Breach 5s
        SkillData::new(
            "Crushing Weapon",
            "Imbue Weapon",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            Resource::Stamina,
        )
        .with_damage(SkillDamage::new().with_hits(vec![HitDamage::new(
            DamageFlags::physical_single(),
            0.09297,
            0.976185,
        )]))
        .with_spammable()
        .with_bonuses(vec![MAJOR_BREACH.clone().with_duration(5.0)]),
        // Accelerate line - No damage, Minor Force
        // Accelerate: Minor Force 20s
        SkillData::new(
            "Accelerate",
            "Accelerate",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            Resource::Magicka,
        )
        .with_bonuses(vec![MINOR_FORCE.clone()]),
        // Channeled Acceleration: Minor Force 60s
        SkillData::new(
            "Channeled Acceleration",
            "Accelerate",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            Resource::Magicka,
        )
        .with_bonuses(vec![MINOR_FORCE.clone().with_duration(60.0)]),
        // Race Against Time: Minor Force 20s
        SkillData::new(
            "Race Against Time",
            "Accelerate",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            Resource::Magicka,
        )
        .with_bonuses(vec![MINOR_FORCE.clone()]),
    ]
});
