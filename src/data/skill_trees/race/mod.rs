use crate::domain::BonusData;
use serde::{Deserialize, Serialize};

pub mod argonian;
pub mod breton;
pub mod dark_elf;
pub mod high_elf;
pub mod imperial;
pub mod khajit;
pub mod nord;
pub mod orc;
pub mod redguard;
pub mod wood_elf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Race {
    Altmer,
    Argonian,
    Bosmer,
    Breton,
    Dunmer,
    Imperial,
    Khajiit,
    Nord,
    Orc,
    Redguard,
}

pub fn race_bonuses(race: &Race) -> &'static [BonusData] {
    match race {
        Race::Argonian => &argonian::ARGONIAN_BONUSES,
        Race::Breton => &breton::BRETON_BONUSES,
        Race::Dunmer => &dark_elf::DARK_ELF_BONUSES,
        Race::Altmer => &high_elf::HIGH_ELF_BONUSES,
        Race::Imperial => &imperial::IMPERIAL_BONUSES,
        Race::Khajiit => &khajit::KHAJIIT_BONUSES,
        Race::Nord => &nord::NORD_BONUSES,
        Race::Orc => &orc::ORC_BONUSES,
        Race::Redguard => &redguard::REDGUARD_BONUSES,
        Race::Bosmer => &wood_elf::WOOD_ELF_BONUSES,
    }
}
