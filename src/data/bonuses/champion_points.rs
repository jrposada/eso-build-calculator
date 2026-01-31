use crate::data::BonusType;
use crate::domain::ChampionPointBonus;
use once_cell::sync::Lazy;

pub static CHAMPION_POINTS: Lazy<Vec<ChampionPointBonus>> = Lazy::new(|| {
    vec![
        ChampionPointBonus::new("Backstabber", BonusType::CriticalDamage, 0.02 * 5.0),
        ChampionPointBonus::new("Biting Aura", BonusType::AoeDamage, 0.03 * 2.0),
        ChampionPointBonus::new("Deadly Aim", BonusType::SingleDamage, 0.03 * 2.0),
        ChampionPointBonus::new("Master-at-Arms", BonusType::DirectDamage, 0.03 * 2.0),
        ChampionPointBonus::new("Exploiter", BonusType::OffBalanceDamage, 0.02 * 5.0),
        ChampionPointBonus::new("Fighting Finesse", BonusType::CriticalDamage, 0.04 * 2.0),
        ChampionPointBonus::new("Thaumaturge", BonusType::DotDamage, 0.03 * 2.0),
    ]
});
