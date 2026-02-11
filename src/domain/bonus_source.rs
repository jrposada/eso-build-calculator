use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BonusSource {
    ChampionPointSlottable,
    ChampionPointPassive,
    Passive,
    Skill,
    Buff,
}

impl fmt::Display for BonusSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BonusSource::ChampionPointSlottable => "Champion Point (Slottable)",
            BonusSource::ChampionPointPassive => "Champion Point (Passive)",
            BonusSource::Passive => "Passive",
            BonusSource::Skill => "Skill",
            BonusSource::Buff => "Buff",
        };
        write!(f, "{}", s)
    }
}
