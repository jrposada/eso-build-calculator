use crate::data::{
    BonusTarget, ClassName, DamageType, Resource, SkillLineName, SkillMechanic, SkillType,
    TargetType,
};
use crate::domain::BonusData;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Hit damage data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HitDamage {
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<f64>,
}

impl HitDamage {
    pub fn new(value: f64) -> Self {
        Self { value, delay: None }
    }

    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = Some(delay);
        self
    }
}

/// DoT (Damage over Time) damage data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DotDamage {
    pub value: f64,
    pub duration: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<f64>,
    /// Defaults to duration if not specified (total damage over duration)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,
    /// Percentage increase per tick (e.g., 0.12 for 12%)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub increase_per_tick: Option<f64>,
    /// Flat increase per tick
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_increase_per_tick: Option<f64>,
    /// If true, this damage ignores modifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignores_modifier: Option<bool>,
}

impl DotDamage {
    pub fn new(value: f64, duration: f64) -> Self {
        Self {
            value,
            duration,
            delay: None,
            interval: None,
            increase_per_tick: None,
            flat_increase_per_tick: None,
            ignores_modifier: None,
        }
    }

    pub fn with_interval(mut self, interval: f64) -> Self {
        self.interval = Some(interval);
        self
    }

    pub fn with_increase_per_tick(mut self, increase: f64) -> Self {
        self.increase_per_tick = Some(increase);
        self
    }

    pub fn with_flat_increase_per_tick(mut self, increase: f64) -> Self {
        self.flat_increase_per_tick = Some(increase);
        self
    }

    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn ignores_modifier(mut self) -> Self {
        self.ignores_modifier = Some(true);
        self
    }
}

/// Skill damage containing hits and dots
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SkillDamage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hits: Option<Vec<HitDamage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dots: Option<Vec<DotDamage>>,
}

impl SkillDamage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_hits(mut self, hits: Vec<HitDamage>) -> Self {
        self.hits = Some(hits);
        self
    }

    pub fn with_dots(mut self, dots: Vec<DotDamage>) -> Self {
        self.dots = Some(dots);
        self
    }

    pub fn has_damage(&self) -> bool {
        let has_hits = self.hits.as_ref().is_some_and(|h| !h.is_empty());
        let has_dots = self.dots.as_ref().is_some_and(|d| !d.is_empty());
        has_hits || has_dots
    }
}

/// Raw skill data used to construct skills
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillData {
    pub name: String,
    pub base_skill_name: String,
    pub class_name: ClassName,
    pub skill_line: SkillLineName,
    pub damage: SkillDamage,
    pub damage_type: DamageType,
    pub target_type: TargetType,
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_time: Option<f64>,
}

impl SkillData {
    pub fn new(
        name: impl Into<String>,
        base_skill_name: impl Into<String>,
        class_name: ClassName,
        skill_line: SkillLineName,
        damage: SkillDamage,
        damage_type: DamageType,
        target_type: TargetType,
        resource: Resource,
    ) -> Self {
        Self {
            name: name.into(),
            base_skill_name: base_skill_name.into(),
            class_name,
            skill_line,
            damage,
            damage_type,
            target_type,
            resource,
            channel_time: None,
        }
    }

    pub fn with_channel_time(mut self, channel_time: f64) -> Self {
        self.channel_time = Some(channel_time);
        self
    }
}

/// A skill with calculated properties
#[derive(Debug, Clone)]
pub struct Skill {
    data: Arc<SkillData>,
}

impl Skill {
    pub fn new(data: SkillData) -> Self {
        Self {
            data: Arc::new(data),
        }
    }

    pub fn from_data(data: Arc<SkillData>) -> Self {
        Self { data }
    }

    pub fn from_arc(data: Arc<SkillData>) -> Self {
        Self { data }
    }

    // Getters
    pub fn name(&self) -> &str {
        &self.data.name
    }

    pub fn base_skill_name(&self) -> &str {
        &self.data.base_skill_name
    }

    pub fn class_name(&self) -> ClassName {
        self.data.class_name
    }

    pub fn skill_line(&self) -> SkillLineName {
        self.data.skill_line
    }

    pub fn damage(&self) -> &SkillDamage {
        &self.data.damage
    }

    pub fn damage_type(&self) -> DamageType {
        self.data.damage_type
    }

    pub fn target_type(&self) -> TargetType {
        self.data.target_type
    }

    pub fn resource(&self) -> Resource {
        self.data.resource
    }

    pub fn channel_time(&self) -> Option<f64> {
        self.data.channel_time
    }

    pub fn data(&self) -> &SkillData {
        &self.data
    }

    pub fn data_arc(&self) -> Arc<SkillData> {
        Arc::clone(&self.data)
    }

    /// Get the skill type (class or weapon)
    pub fn skill_type(&self) -> SkillType {
        if self.data.class_name == ClassName::Weapon {
            SkillType::Weapon
        } else {
            SkillType::Class
        }
    }

    /// Get the skill mechanic
    pub fn mechanic(&self) -> SkillMechanic {
        if self.data.channel_time.is_some() {
            return SkillMechanic::Channeled;
        }

        if self
            .data
            .damage
            .dots
            .as_ref()
            .is_some_and(|d| !d.is_empty())
        {
            return SkillMechanic::Dot;
        }

        if let Some(hits) = &self.data.damage.hits {
            if !hits.is_empty() && hits.iter().any(|h| h.value > 0.0) {
                return SkillMechanic::Instant;
            }
        }

        SkillMechanic::Unknown
    }

    /// Get the skill duration (max DoT duration or channel time)
    pub fn duration(&self) -> f64 {
        if let Some(dots) = &self.data.damage.dots {
            if !dots.is_empty() {
                return dots
                    .iter()
                    .map(|dot| dot.duration + dot.delay.unwrap_or(0.0))
                    .fold(0.0, f64::max);
            }
        }
        self.data.channel_time.unwrap_or(0.0)
    }

    /// Check if this is an ultimate skill
    pub fn is_ultimate(&self) -> bool {
        self.data.resource == Resource::Ultimate
    }

    /// Calculate the total damage per cast with optional bonuses
    pub fn calculate_damage_per_cast(&self, bonuses: &[BonusData]) -> f64 {
        let mut total_damage = 0.0;

        // Map target type to bonus type
        let target_bonus_type = match self.data.target_type {
            TargetType::Aoe => BonusTarget::AoeDamage,
            TargetType::Single => BonusTarget::SingleDamage,
        };

        // Sum all direct hits
        if let Some(hits) = &self.data.damage.hits {
            let hit_affected_by = [BonusTarget::DirectDamage, target_bonus_type];

            let hit_modifiers: Vec<_> = bonuses
                .iter()
                .filter(|b| hit_affected_by.contains(&b.target))
                .collect();

            for hit in hits {
                total_damage += self.apply_damage_modifier(&hit_modifiers, hit.value);
            }
        }

        // Add DoT damage over full duration
        if let Some(dots) = &self.data.damage.dots {
            let dot_affected_by = [BonusTarget::DotDamage, target_bonus_type];

            let dot_modifiers: Vec<_> = bonuses
                .iter()
                .filter(|b| dot_affected_by.contains(&b.target))
                .collect();

            for dot in dots {
                // If interval is not defined then we only know the total damage done over
                // the duration which is equivalent to interval = duration
                let interval = dot.interval.unwrap_or(dot.duration);
                let ticks = (dot.duration / interval).floor() as i32;
                let increase_per_tick = dot.increase_per_tick.unwrap_or(0.0);
                let flat_increase_per_tick = dot.flat_increase_per_tick.unwrap_or(0.0);

                for i in 0..ticks {
                    let percentage_multiplier = 1.0 + (i as f64) * increase_per_tick;
                    let flat_increase = (i as f64) * flat_increase_per_tick;
                    let tick_damage = dot.value * percentage_multiplier + flat_increase;

                    if dot.ignores_modifier.unwrap_or(false) {
                        total_damage += tick_damage;
                    } else {
                        total_damage += self.apply_damage_modifier(&dot_modifiers, tick_damage);
                    }
                }
            }
        }

        total_damage
    }

    fn apply_damage_modifier(&self, modifiers: &[&BonusData], value: f64) -> f64 {
        let total_modifier: f64 = modifiers.iter().map(|m| m.value).sum();
        value * (1.0 + total_modifier)
    }

    /// Format skill details for display
    pub fn format_details(&self) -> String {
        let mut lines = Vec::new();

        lines.push("=".repeat(60));
        lines.push(format!("  {}", self.name()));
        lines.push("=".repeat(60));
        lines.push(String::new());
        lines.push("  Basic Info".to_string());
        lines.push(format!("  {}", "-".repeat(56)));
        lines.push(format!("  Base Skill:      {}", self.base_skill_name()));
        lines.push(format!("  Source:          {}", self.class_name()));
        lines.push(format!("  Skill Line:      {}", self.skill_line()));
        lines.push(format!("  Resource:        {}", self.resource()));
        lines.push(format!("  Damage Type:     {}", self.damage_type()));
        lines.push(format!("  Target Type:     {}", self.target_type()));
        lines.push(format!("  Mechanic:        {}", self.mechanic()));

        if let Some(channel_time) = self.channel_time() {
            lines.push(format!("  Channel Time:    {}s", channel_time));
        }

        lines.push(String::new());
        lines.push("  Damage".to_string());
        lines.push(format!("  {}", "-".repeat(56)));

        if let Some(hits) = &self.data.damage.hits {
            if !hits.is_empty() {
                lines.push("  Hits:".to_string());
                for (j, hit) in hits.iter().enumerate() {
                    let delay = hit
                        .delay
                        .map(|d| format!(" (delay: {}s)", d))
                        .unwrap_or_default();
                    lines.push(format!("    {}. {}{}", j + 1, hit.value, delay));
                }
            }
        }

        if let Some(dots) = &self.data.damage.dots {
            if !dots.is_empty() {
                lines.push("  DoTs:".to_string());
                for (j, dot) in dots.iter().enumerate() {
                    let interval = dot
                        .interval
                        .map(|i| format!(" every {}s", i))
                        .unwrap_or_default();
                    let increase = dot
                        .increase_per_tick
                        .map(|i| format!(" (+{:.0}%/tick)", i * 100.0))
                        .unwrap_or_default();
                    let flat_increase = dot
                        .flat_increase_per_tick
                        .map(|f| format!(" (+{}/tick)", f))
                        .unwrap_or_default();
                    lines.push(format!(
                        "    {}. {}{} for {}s{}{}",
                        j + 1,
                        dot.value,
                        interval,
                        dot.duration,
                        increase,
                        flat_increase
                    ));
                }
            }
        }

        lines.push(String::new());
        lines.push("  Calculated".to_string());
        lines.push(format!("  {}", "-".repeat(56)));

        let duration = self.duration();
        let duration_str = if duration > 0.0 {
            format!("{}s", duration)
        } else {
            "instant".to_string()
        };
        lines.push(format!("  Duration:        {}", duration_str));
        lines.push(format!(
            "  Damage/Cast:     {:.0}",
            self.calculate_damage_per_cast(&[])
        ));

        lines.join("\n")
    }
}
