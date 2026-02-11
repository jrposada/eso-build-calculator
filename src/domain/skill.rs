use super::{
    formulas, BonusData, CharacterStats, ClassName, DamageFlags, ExecuteData, ExecuteScaling,
    Resource, SkillDamage, SkillLineName, SkillMechanic,
};
use serde::{Deserialize, Serialize};

/// Raw skill data used to construct skills
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillData {
    pub name: String,
    pub base_skill_name: String,
    pub class_name: ClassName,
    pub skill_line: SkillLineName,
    pub damage: SkillDamage,
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute: Option<ExecuteData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonuses: Option<Vec<BonusData>>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub spammable: bool,
}

impl SkillData {
    pub fn new(
        name: impl Into<String>,
        base_skill_name: impl Into<String>,
        class_name: ClassName,
        skill_line: SkillLineName,
        damage: SkillDamage,
        resource: Resource,
    ) -> Self {
        Self {
            name: name.into(),
            base_skill_name: base_skill_name.into(),
            class_name,
            skill_line,
            damage,
            resource,
            channel_time: None,
            execute: None,
            bonuses: None,
            spammable: false,
        }
    }

    pub fn with_channel_time(mut self, channel_time: f64) -> Self {
        self.channel_time = Some(channel_time);
        self
    }

    pub fn with_execute(
        mut self,
        multiplier: f64,
        threshold: f64,
        scaling: ExecuteScaling,
    ) -> Self {
        self.execute = Some(ExecuteData::new(multiplier, threshold, scaling));
        self
    }

    pub fn with_bonuses(mut self, bonuses: Vec<BonusData>) -> Self {
        self.bonuses = Some(bonuses);
        self
    }

    pub fn with_spammable(mut self) -> Self {
        self.spammable = true;
        self
    }

    /// Get primary damage flags from the first hit or dot (for display purposes)
    pub fn primary_flags(&self) -> Option<DamageFlags> {
        if let Some(hits) = &self.damage.hits {
            if let Some(hit) = hits.first() {
                return Some(hit.flags);
            }
        }
        if let Some(dots) = &self.damage.dots {
            if let Some(dot) = dots.first() {
                return Some(dot.flags);
            }
        }
        None
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
            if !hits.is_empty() {
                return SkillMechanic::Instant;
            }
        }

        // Default to Instant for skills with no damage
        SkillMechanic::Instant
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

    /// Default stats used when no character stats are provided (40k max stat, 5.5k max power)
    const DEFAULT_MAX_STAT: f64 = 40000.0;
    const DEFAULT_MAX_POWER: f64 = 5500.0;

    /// Calculate the total damage per cast with optional bonuses using default stats.
    /// This excludes conditional execute hits (hits with execute_threshold).
    pub fn calculate_damage_per_cast(&self, bonuses: &[BonusData]) -> f64 {
        self.calculate_damage_at_health_internal(
            bonuses,
            Self::DEFAULT_MAX_STAT,
            Self::DEFAULT_MAX_POWER,
            None,
        )
    }

    /// Internal method to calculate damage with enemy health filtering
    ///
    /// # Arguments
    /// * `bonuses` - Active damage bonuses
    /// * `max_stat` - The higher of max_magicka and max_stamina
    /// * `max_power` - The higher of weapon_damage and spell_damage
    /// * `enemy_health` - Optional enemy health percentage for execute calculations
    fn calculate_damage_at_health_internal(
        &self,
        bonuses: &[BonusData],
        max_stat: f64,
        max_power: f64,
        enemy_health: Option<f64>,
    ) -> f64 {
        let mut total_damage = 0.0;

        // Filter bonuses by enemy health (for execute bonuses) and skill line
        let applicable_bonuses: Vec<_> = bonuses
            .iter()
            .filter(|b| {
                b.applies_to_skill_line(self.skill_line)
                    && enemy_health.map_or(!b.is_execute_bonus(), |h| b.applies_at_health(h))
            })
            .collect();

        // Sum all direct hits
        if let Some(hits) = &self.damage.hits {
            for hit in hits {
                // Filter bonuses for this specific hit's flags
                let hit_modifiers: Vec<_> = applicable_bonuses
                    .iter()
                    .filter(|b| hit.flags.matches_bonus_target(b.target))
                    .copied()
                    .collect();

                let hit_value = hit.effective_value(max_stat, max_power);

                if let Some(threshold) = hit.execute_threshold {
                    if enemy_health.map_or(false, |h| h < threshold) {
                        total_damage += Self::apply_damage_modifier(&hit_modifiers, hit_value);
                    }
                } else {
                    total_damage += Self::apply_damage_modifier(&hit_modifiers, hit_value);
                }
            }
        }

        // Add DoT damage over full duration
        if let Some(dots) = &self.damage.dots {
            for dot in dots {
                // Filter bonuses for this specific dot's flags
                let dot_modifiers: Vec<_> = applicable_bonuses
                    .iter()
                    .filter(|b| dot.flags.matches_bonus_target(b.target))
                    .copied()
                    .collect();

                let dot_value = dot.effective_value(max_stat, max_power);

                let interval = dot.interval.unwrap_or(dot.duration);
                let ticks = (dot.duration / interval).floor() as i32;
                let increase_per_tick = dot.increase_per_tick.unwrap_or(0.0);
                let flat_increase_per_tick = dot.flat_increase_per_tick.unwrap_or(0.0);

                for i in 0..ticks {
                    let percentage_multiplier = 1.0 + (i as f64) * increase_per_tick;
                    let flat_increase = (i as f64) * flat_increase_per_tick;
                    let tick_damage = dot_value * percentage_multiplier + flat_increase;

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

    /// Calculate damage at a specific enemy health percentage using default stats,
    /// including execute bonuses.
    pub fn calculate_damage_at_health(
        &self,
        bonuses: &[BonusData],
        enemy_health_percent: f64,
    ) -> f64 {
        let base = self.calculate_damage_at_health_internal(
            bonuses,
            Self::DEFAULT_MAX_STAT,
            Self::DEFAULT_MAX_POWER,
            Some(enemy_health_percent),
        );
        if let Some(execute) = &self.execute {
            base * execute.calculate_multiplier(enemy_health_percent)
        } else {
            base
        }
    }

    /// Calculate damage using character stats for coefficient-based calculation.
    /// Calculate pre-mitigation damage (tooltip) using character stats but without
    /// armor penetration or critical multipliers.
    pub fn calculate_tooltip_damage_with_stats(
        &self,
        bonuses: &[BonusData],
        stats: &CharacterStats,
    ) -> f64 {
        let max_stat = stats.max_stat();
        let max_power = stats.max_power();
        self.calculate_damage_at_health_internal(bonuses, max_stat, max_power, None)
    }

    pub fn calculate_damage_with_stats(
        &self,
        bonuses: &[BonusData],
        stats: &CharacterStats,
    ) -> f64 {
        self.calculate_damage_with_stats_internal(bonuses, stats, None)
    }

    /// Calculate damage at specific health with character stats
    pub fn calculate_damage_with_stats_at_health(
        &self,
        bonuses: &[BonusData],
        stats: &CharacterStats,
        enemy_health_percent: f64,
    ) -> f64 {
        let base =
            self.calculate_damage_with_stats_internal(bonuses, stats, Some(enemy_health_percent));
        if let Some(execute) = &self.execute {
            base * execute.calculate_multiplier(enemy_health_percent)
        } else {
            base
        }
    }

    /// Internal method to calculate damage with character stats
    fn calculate_damage_with_stats_internal(
        &self,
        bonuses: &[BonusData],
        stats: &CharacterStats,
        enemy_health: Option<f64>,
    ) -> f64 {
        let max_stat = stats.max_stat();
        let max_power = stats.max_power();

        let pre_mitigation =
            self.calculate_damage_at_health_internal(bonuses, max_stat, max_power, enemy_health);

        let armor_factor = formulas::armor_damage_factor(stats.target_armor, stats.penetration);
        let crit_mult = formulas::critical_multiplier(stats.critical_chance, stats.critical_damage);

        pre_mitigation * armor_factor * crit_mult
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

        if let Some(flags) = self.primary_flags() {
            lines.push(format!("  Damage Type:     {}", flags.element_display()));
            lines.push(format!("  Target Type:     {}", flags.target_display()));
        }

        lines.push(format!("  Mechanic:        {}", self.mechanic()));

        if self.spammable {
            lines.push("  Spammable:       Yes".to_string());
        }

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
                    let execute = hit
                        .execute_threshold
                        .map(|t| format!(" (execute: <{:.0}% HP)", t * 100.0))
                        .unwrap_or_default();
                    let flags_str = format!(" [{}]", hit.flags);
                    let value =
                        hit.effective_value(Self::DEFAULT_MAX_STAT, Self::DEFAULT_MAX_POWER);
                    lines.push(format!(
                        "    {}. {:.0}{}{}{}",
                        j + 1,
                        value,
                        delay,
                        execute,
                        flags_str
                    ));
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
                    let flags_str = format!(" [{}]", dot.flags);
                    let value =
                        dot.effective_value(Self::DEFAULT_MAX_STAT, Self::DEFAULT_MAX_POWER);
                    lines.push(format!(
                        "    {}. {:.0}{} for {}s{}{}{}",
                        j + 1,
                        value,
                        interval,
                        dot.duration,
                        increase,
                        flat_increase,
                        flags_str
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

        // Check if skill has conditional execute hits
        let has_execute_hits = self.damage.hits.as_ref().map_or(false, |hits| {
            hits.iter().any(|h| h.execute_threshold.is_some())
        });

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
        } else if has_execute_hits {
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
