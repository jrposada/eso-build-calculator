use crate::data::{
    BonusTarget, ClassName, DamageType, Resource, SkillLineName, SkillMechanic, TargetType,
};
use crate::domain::{BonusData, ExecuteData, ExecuteScaling, SkillDamage};
use serde::{Deserialize, Serialize};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute: Option<ExecuteData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonuses: Option<Vec<BonusData>>,
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
            execute: None,
            bonuses: None,
        }
    }

    pub fn with_channel_time(mut self, channel_time: f64) -> Self {
        self.channel_time = Some(channel_time);
        self
    }

    pub fn with_execute(mut self, multiplier: f64, threshold: f64, scaling: ExecuteScaling) -> Self {
        self.execute = Some(ExecuteData::new(multiplier, threshold, scaling));
        self
    }

    pub fn with_bonuses(mut self, bonuses: Vec<BonusData>) -> Self {
        self.bonuses = Some(bonuses);
        self
    }

    /// Get the skill mechanic
    pub fn mechanic(&self) -> SkillMechanic {
        if self.channel_time.is_some() {
            return SkillMechanic::Channeled;
        }

        if self.damage.dots.as_ref().is_some_and(|d| !d.is_empty()) {
            return SkillMechanic::Dot;
        }

        if let Some(hits) = &self.damage.hits {
            if !hits.is_empty() && hits.iter().any(|h| h.value > 0.0) {
                return SkillMechanic::Instant;
            }
        }

        SkillMechanic::Unknown
    }

    /// Get the skill duration (max DoT duration or channel time)
    pub fn duration(&self) -> f64 {
        if let Some(dots) = &self.damage.dots {
            if !dots.is_empty() {
                return dots
                    .iter()
                    .map(|dot| dot.duration + dot.delay.unwrap_or(0.0))
                    .fold(0.0, f64::max);
            }
        }
        self.channel_time.unwrap_or(0.0)
    }

    /// Calculate the total damage per cast with optional bonuses
    pub fn calculate_damage_per_cast(&self, bonuses: &[BonusData]) -> f64 {
        let mut total_damage = 0.0;

        // Map target type to bonus type
        let target_bonus_type = match self.target_type {
            TargetType::Aoe => BonusTarget::AoeDamage,
            TargetType::Single => BonusTarget::SingleDamage,
        };

        // Sum all direct hits
        if let Some(hits) = &self.damage.hits {
            let hit_affected_by = [BonusTarget::DirectDamage, target_bonus_type];

            let hit_modifiers: Vec<_> = bonuses
                .iter()
                .filter(|b| hit_affected_by.contains(&b.target))
                .collect();

            for hit in hits {
                total_damage += Self::apply_damage_modifier(&hit_modifiers, hit.value);
            }
        }

        // Add DoT damage over full duration
        if let Some(dots) = &self.damage.dots {
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
                        total_damage += Self::apply_damage_modifier(&dot_modifiers, tick_damage);
                    }
                }
            }
        }

        total_damage
    }

    fn apply_damage_modifier(modifiers: &[&BonusData], value: f64) -> f64 {
        let total_modifier: f64 = modifiers.iter().map(|m| m.value).sum();
        value * (1.0 + total_modifier)
    }

    /// Calculate damage at a specific enemy health percentage, including execute bonuses
    pub fn calculate_damage_at_health(&self, bonuses: &[BonusData], enemy_health_percent: f64) -> f64 {
        let base = self.calculate_damage_per_cast(bonuses);
        if let Some(execute) = &self.execute {
            base * execute.calculate_multiplier(enemy_health_percent)
        } else {
            base
        }
    }

    /// Format skill details for display
    pub fn format_details(&self) -> String {
        let mut lines = Vec::new();

        lines.push("=".repeat(60));
        lines.push(format!("  {}", self.name));
        lines.push("=".repeat(60));
        lines.push(String::new());
        lines.push("  Basic Info".to_string());
        lines.push(format!("  {}", "-".repeat(56)));
        lines.push(format!("  Base Skill:      {}", self.base_skill_name));
        lines.push(format!("  Source:          {}", self.class_name));
        lines.push(format!("  Skill Line:      {}", self.skill_line));
        lines.push(format!("  Resource:        {}", self.resource));
        lines.push(format!("  Damage Type:     {}", self.damage_type));
        lines.push(format!("  Target Type:     {}", self.target_type));
        lines.push(format!("  Mechanic:        {}", self.mechanic()));

        if let Some(channel_time) = self.channel_time {
            lines.push(format!("  Channel Time:    {}s", channel_time));
        }

        lines.push(String::new());
        lines.push("  Damage".to_string());
        lines.push(format!("  {}", "-".repeat(56)));

        if let Some(hits) = &self.damage.hits {
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

        if let Some(dots) = &self.damage.dots {
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

        if let Some(execute) = &self.execute {
            lines.push(String::new());
            lines.push("  Execute".to_string());
            lines.push(format!("  {}", "-".repeat(56)));
            let scaling_str = match execute.scaling {
                ExecuteScaling::Flat => "flat",
                ExecuteScaling::Linear => "linear",
            };
            lines.push(format!(
                "  Threshold:       {:.0}% HP",
                execute.threshold * 100.0
            ));
            lines.push(format!(
                "  Bonus:           +{:.0}% ({})",
                execute.multiplier * 100.0,
                scaling_str
            ));
            lines.push(format!(
                "  Damage @0% HP:   {:.0}",
                self.calculate_damage_at_health(&[], 0.0)
            ));
        }

        if let Some(bonuses) = &self.bonuses {
            if !bonuses.is_empty() {
                lines.push(String::new());
                lines.push("  Granted Bonuses".to_string());
                lines.push(format!("  {}", "-".repeat(56)));
                for bonus in bonuses {
                    let duration_str = bonus
                        .duration
                        .map(|d| format!(" ({}s)", d))
                        .unwrap_or_default();
                    lines.push(format!("    - {}{}", bonus.name, duration_str));
                }
            }
        }

        lines.join("\n")
    }
}
