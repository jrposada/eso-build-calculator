use crate::domain::{ClassName, PassiveData, SkillLineName};
use once_cell::sync::Lazy;

pub static PSIJIC_ORDER_PASSIVES: Lazy<Vec<PassiveData>> = Lazy::new(|| {
    vec![
        PassiveData::new(
            "Clairvoyance",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            vec![],
        ),
        PassiveData::new(
            "Spell Orb",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            vec![],
        ),
        PassiveData::new(
            "Concentrated Barrier",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            vec![],
        ),
        PassiveData::new(
            "Deliberation",
            ClassName::Guild,
            SkillLineName::PsijicOrder,
            vec![],
        ),
    ]
});
