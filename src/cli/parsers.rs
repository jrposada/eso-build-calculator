use crate::data::bonuses::CHAMPION_POINTS;
use crate::domain::{BonusData, ClassName, SkillLineName};

pub fn parse_class_name(s: &str) -> Result<ClassName, String> {
    let s = s.trim();
    match s.to_lowercase().as_str() {
        "arcanist" => Ok(ClassName::Arcanist),
        "dragonknight" => Ok(ClassName::Dragonknight),
        "nightblade" => Ok(ClassName::Nightblade),
        "sorcerer" => Ok(ClassName::Sorcerer),
        "templar" => Ok(ClassName::Templar),
        "warden" => Ok(ClassName::Warden),
        _ => Err(format!(
            "Invalid class '{}'. Valid options: arcanist, dragonknight, nightblade, sorcerer, templar, warden",
            s
        )),
    }
}

pub fn parse_class_skill_line(s: &str) -> Result<SkillLineName, String> {
    let s = s.trim();
    match s.to_lowercase().as_str() {
        // Arcanist
        "curative-runeforms" | "curativeruneforms" => Ok(SkillLineName::CurativeRuneforms),
        "herald-of-the-tome" | "heraldofthetome" => Ok(SkillLineName::HeraldOfTheTome),
        "soldier-of-apocrypha" | "soldierofapocrypha" => Ok(SkillLineName::SoldierOfApocrypha),
        // Dragonknight
        "ardent-flame" | "ardentflame" => Ok(SkillLineName::ArdentFlame),
        "draconic-power" | "draconicpower" => Ok(SkillLineName::DraconicPower),
        "earthen-heart" | "earthenheart" => Ok(SkillLineName::EarthenHeart),
        // Nightblade
        "assassination" => Ok(SkillLineName::Assassination),
        "shadow" => Ok(SkillLineName::Shadow),
        "siphoning" => Ok(SkillLineName::Siphoning),
        // Sorcerer
        "daedric-summoning" | "daedricsummoning" => Ok(SkillLineName::DaedricSummoning),
        "dark-magic" | "darkmagic" => Ok(SkillLineName::DarkMagic),
        "storm-calling" | "stormcalling" => Ok(SkillLineName::StormCalling),
        // Templar
        "aedric-spear" | "aedricspear" => Ok(SkillLineName::AedricSpear),
        "dawns-wrath" | "dawnswrath" => Ok(SkillLineName::DawnsWrath),
        "restoring-light" | "restoringlight" => Ok(SkillLineName::RestoringLight),
        // Warden
        "animal-companions" | "animalcompanions" => Ok(SkillLineName::AnimalCompanions),
        "green-balance" | "greenbalance" => Ok(SkillLineName::GreenBalance),
        "winters-embrace" | "wintersembrace" => Ok(SkillLineName::WintersEmbrace),
        _ => Err(format!(
            "Invalid class skill line '{}'. Valid options: curative-runeforms, herald-of-the-tome, soldier-of-apocrypha, \
            ardent-flame, draconic-power, earthen-heart, assassination, shadow, siphoning, \
            daedric-summoning, dark-magic, storm-calling, aedric-spear, dawns-wrath, restoring-light, \
            animal-companions, green-balance, winters-embrace",
            s
        )),
    }
}

pub fn parse_weapon_skill_line(s: &str) -> Result<SkillLineName, String> {
    let s = s.trim();
    match s.to_lowercase().as_str() {
        "bow" => Ok(SkillLineName::Bow),
        "destruction-staff" | "destructionstaff" => Ok(SkillLineName::DestructionStaff),
        "dual-wield" | "dualwield" => Ok(SkillLineName::DualWield),
        "two-handed" | "twohanded" => Ok(SkillLineName::TwoHanded),
        _ => Err(format!(
            "Invalid weapon skill line '{}'. Valid options: bow, destruction-staff, dual-wield, two-handed",
            s
        )),
    }
}

pub fn parse_champion_point(s: &str) -> Result<BonusData, String> {
    let s = s.trim();
    // Normalize input: replace hyphens with spaces for matching
    let normalized = s.to_lowercase().replace('-', " ");

    CHAMPION_POINTS
        .iter()
        .find(|cp| cp.name.to_lowercase() == normalized)
        .cloned()
        .ok_or_else(|| {
            format!(
                "Invalid champion point '{}'. Valid options: backstabber, biting-aura, deadly-aim, \
                exploiter, fighting-finesse, master-at-arms, thaumaturge",
                s
            )
        })
}
