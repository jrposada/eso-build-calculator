use crate::data::bonuses::{
    EMPOWER, MAJOR_BREACH, MAJOR_BRUTALITY, MAJOR_PROPHECY, MAJOR_SAVAGERY, MAJOR_SORCERY,
};
use crate::data::{BonusTrigger, ClassName, DamageType, Resource, SkillLineName, TargetType};
use crate::domain::{DotDamage, HitDamage, SkillDamage, SkillData};
use once_cell::sync::Lazy;

pub static DRAGONKNIGHT_SKILLS: Lazy<Vec<SkillData>> = Lazy::new(|| {
    vec![
        // === ARDENT FLAME ===
        // Ultimate - Dragonknight Standard line
        SkillData::new(
            "Dragonknight Standard",
            "Dragonknight Standard",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new().with_dots(vec![DotDamage::new(870.0, 16.0).with_interval(1.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Shifting Standard",
            "Dragonknight Standard",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new().with_dots(vec![DotDamage::new(898.0, 25.0).with_interval(1.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Standard of Might",
            "Dragonknight Standard",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new().with_dots(vec![DotDamage::new(870.0, 16.0).with_interval(1.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        // Lava Whip line
        SkillData::new(
            "Lava Whip",
            "Lava Whip",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new().with_hits(vec![HitDamage::new(2323.0)]),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Flame Lash",
            "Lava Whip",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new().with_hits(vec![HitDamage::new(2323.0)]),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        // Molten Whip: Too complex - Seething Fury stacks (+20% damage per stack, +100 Weapon/Spell Damage per stack)
        SkillData::new(
            "Molten Whip",
            "Lava Whip",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new().with_hits(vec![HitDamage::new(2323.0)]),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        // Searing Strike line
        SkillData::new(
            "Searing Strike",
            "Searing Strike",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1161.0)])
                .with_dots(vec![DotDamage::new(3470.0, 20.0)]),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Burning Embers",
            "Searing Strike",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1161.0)])
                .with_dots(vec![DotDamage::new(3470.0, 20.0)]),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Venomous Claw",
            "Searing Strike",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1161.0)])
                .with_dots(vec![DotDamage::new(347.0, 20.0)
                    .with_interval(2.0)
                    .with_increase_per_tick(0.12)]),
            DamageType::Poison,
            TargetType::Single,
            Resource::Stamina,
        ),
        // Fiery Breath line
        SkillData::new(
            "Fiery Breath",
            "Fiery Breath",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1742.0)])
                .with_dots(vec![DotDamage::new(2900.0, 20.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        // Engulfing Flames: Too complex - enemies take up to 6% more Flame Damage
        SkillData::new(
            "Engulfing Flames",
            "Fiery Breath",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1799.0)])
                .with_dots(vec![DotDamage::new(2980.0, 20.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        // Noxious Breath: Major Breach (20s), Poisoned status
        SkillData::new(
            "Noxious Breath",
            "Fiery Breath",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1799.0)])
                .with_dots(vec![DotDamage::new(2980.0, 20.0)]),
            DamageType::Poison,
            TargetType::Aoe,
            Resource::Stamina,
        )
        .with_bonuses(vec![MAJOR_BREACH.clone()]),
        // Fiery Grip line
        SkillData::new(
            "Fiery Grip",
            "Fiery Grip",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new().with_hits(vec![HitDamage::new(1392.0)]),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Chains of Devastation",
            "Fiery Grip",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new().with_hits(vec![HitDamage::new(1438.0)]),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Unrelenting Grip",
            "Fiery Grip",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new().with_hits(vec![HitDamage::new(1438.0)]),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        // Inferno line
        // Inferno: Major Prophecy + Major Savagery while slotted, fireball every 5s
        SkillData::new(
            "Inferno",
            "Inferno",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new().with_dots(vec![DotDamage::new(1742.0, 15.0).with_interval(5.0)]),
            DamageType::Flame,
            TargetType::Single,
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
        // Cauterize: Major Prophecy + Major Savagery while slotted, heals instead of damages
        SkillData::new(
            "Cauterize",
            "Inferno",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new(),
            DamageType::Flame,
            TargetType::Aoe,
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
        // Flames of Oblivion: Major Prophecy + Major Savagery while slotted, launches 3 fireballs
        SkillData::new(
            "Flames of Oblivion",
            "Inferno",
            ClassName::Dragonknight,
            SkillLineName::ArdentFlame,
            SkillDamage::new().with_dots(vec![DotDamage::new(1799.0, 15.0).with_interval(5.0)]),
            DamageType::Flame,
            TargetType::Aoe,
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
        // === DRACONIC POWER ===
        // Ultimate - Dragon Leap line
        SkillData::new(
            "Dragon Leap",
            "Dragon Leap",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new().with_hits(vec![HitDamage::new(4241.0)]),
            DamageType::Physical,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Ferocious Leap",
            "Dragon Leap",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new().with_hits(vec![HitDamage::new(4241.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Take Flight",
            "Dragon Leap",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new().with_hits(vec![HitDamage::new(5037.0)]),
            DamageType::Physical,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        // Spiked Armor line
        SkillData::new(
            "Spiked Armor",
            "Spiked Armor",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new(),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Hardened Armor",
            "Spiked Armor",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new(),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Volatile Armor",
            "Spiked Armor",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new().with_dots(vec![DotDamage::new(11.0, 20.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        // Dark Talons line
        SkillData::new(
            "Dark Talons",
            "Dark Talons",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new().with_hits(vec![HitDamage::new(1742.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Burning Talons",
            "Dark Talons",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1799.0)])
                .with_dots(vec![DotDamage::new(1635.0, 5.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Choking Talons",
            "Dark Talons",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new().with_hits(vec![HitDamage::new(1742.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        // Dragon Blood line (no damage)
        SkillData::new(
            "Dragon Blood",
            "Dragon Blood",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Coagulating Blood",
            "Dragon Blood",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Green Dragon Blood",
            "Dragon Blood",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        // Protective Scale line (no damage)
        SkillData::new(
            "Protective Scale",
            "Protective Scale",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Dragon Fire Scale",
            "Protective Scale",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new(),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Protective Plate",
            "Protective Scale",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Single,
            Resource::Magicka,
        ),
        // Inhale line
        SkillData::new(
            "Inhale",
            "Inhale",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(870.0),
                HitDamage::new(1742.0).with_delay(2.5),
            ]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_channel_time(2.5),
        SkillData::new(
            "Deep Breath",
            "Inhale",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(870.0),
                HitDamage::new(2249.0).with_delay(2.5),
            ]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_channel_time(2.5),
        SkillData::new(
            "Draw Essence",
            "Inhale",
            ClassName::Dragonknight,
            SkillLineName::DraconicPower,
            SkillDamage::new().with_hits(vec![
                HitDamage::new(870.0),
                HitDamage::new(1742.0).with_delay(2.5),
            ]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_channel_time(2.5),
        // === EARTHEN HEART ===
        // Ultimate - Magma Armor line
        SkillData::new(
            "Magma Armor",
            "Magma Armor",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new().with_dots(vec![DotDamage::new(336.0, 10.0).with_interval(1.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Corrosive Armor",
            "Magma Armor",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new().with_dots(vec![DotDamage::new(347.0, 10.0).with_interval(1.0)]),
            DamageType::Poison,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        SkillData::new(
            "Magma Shell",
            "Magma Armor",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new().with_dots(vec![DotDamage::new(347.0, 10.0).with_interval(1.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Ultimate,
        ),
        // Stonefist line
        SkillData::new(
            "Stonefist",
            "Stonefist",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new().with_hits(vec![HitDamage::new(2323.0)]),
            DamageType::Physical,
            TargetType::Aoe,
            Resource::Stamina,
        ),
        SkillData::new(
            "Obsidian Shard",
            "Stonefist",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new().with_hits(vec![HitDamage::new(448.0)]),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        // Stone Giant: Too complex - Stagger stacks (+65 damage taken per stack for 5s)
        SkillData::new(
            "Stone Giant",
            "Stonefist",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new().with_hits(vec![HitDamage::new(2323.0)]),
            DamageType::Physical,
            TargetType::Aoe,
            Resource::Stamina,
        ),
        // Molten Weapons line (no damage)
        // Molten Weapons: Major Brutality + Major Sorcery (30s)
        SkillData::new(
            "Molten Weapons",
            "Molten Weapons",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new(),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(30.0),
            MAJOR_SORCERY.clone().with_duration(30.0),
        ]),
        // Igneous Weapons: Major Brutality + Major Sorcery (60s)
        SkillData::new(
            "Igneous Weapons",
            "Molten Weapons",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new(),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(60.0),
            MAJOR_SORCERY.clone().with_duration(60.0),
        ]),
        // Molten Armaments: Major Brutality + Major Sorcery (30s), Empower (30s)
        SkillData::new(
            "Molten Armaments",
            "Molten Weapons",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new(),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        )
        .with_bonuses(vec![
            MAJOR_BRUTALITY.clone().with_duration(30.0),
            MAJOR_SORCERY.clone().with_duration(30.0),
            EMPOWER.clone().with_duration(30.0),
        ]),
        // Obsidian Shield line (no damage)
        SkillData::new(
            "Obsidian Shield",
            "Obsidian Shield",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Fragmented Shield",
            "Obsidian Shield",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Igneous Shield",
            "Obsidian Shield",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        // Petrify line
        SkillData::new(
            "Petrify",
            "Petrify",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new().with_hits(vec![HitDamage::new(1161.0).with_delay(2.5)]),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Fossilize",
            "Petrify",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new().with_hits(vec![HitDamage::new(1199.0).with_delay(2.5)]),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        SkillData::new(
            "Shattering Rocks",
            "Petrify",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new().with_hits(vec![HitDamage::new(1199.0).with_delay(2.5)]),
            DamageType::Flame,
            TargetType::Single,
            Resource::Magicka,
        ),
        // Ash Cloud line
        SkillData::new(
            "Ash Cloud",
            "Ash Cloud",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Cinder Storm",
            "Ash Cloud",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new(),
            DamageType::Magic,
            TargetType::Aoe,
            Resource::Magicka,
        ),
        SkillData::new(
            "Eruption",
            "Ash Cloud",
            ClassName::Dragonknight,
            SkillLineName::EarthenHeart,
            SkillDamage::new()
                .with_hits(vec![HitDamage::new(1799.0)])
                .with_dots(vec![DotDamage::new(319.0, 15.0).with_interval(1.0)]),
            DamageType::Flame,
            TargetType::Aoe,
            Resource::Magicka,
        ),
    ]
});
