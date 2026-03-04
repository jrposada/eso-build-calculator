pub mod arena;
pub mod monster;
pub mod mythic;
pub mod normal;

use crate::domain::SetData;
use once_cell::sync::Lazy;

pub use arena::ARENA_SETS;
pub use monster::MONSTER_SETS;
pub use mythic::MYTHIC_SETS;
pub use normal::NORMAL_SETS;

pub static ALL_SETS: Lazy<Vec<&'static SetData>> = Lazy::new(|| {
    let mut sets: Vec<&'static SetData> = Vec::new();
    sets.extend(NORMAL_SETS.iter());
    sets.extend(MONSTER_SETS.iter());
    sets.extend(MYTHIC_SETS.iter());
    sets.extend(ARENA_SETS.iter());
    sets
});

impl SetData {
    pub fn parse(s: &str) -> Result<&'static SetData, String> {
        let s = s.trim();
        let normalized = s.to_lowercase().replace('-', " ");

        ALL_SETS
            .iter()
            .find(|set| set.name.to_lowercase().replace('-', " ") == normalized)
            .copied()
            .ok_or_else(|| {
                let names: Vec<_> = ALL_SETS.iter().map(|s| s.name.as_str()).collect();
                format!("Invalid set '{}'. Valid options: {}", s, names.join(", "))
            })
    }
}
