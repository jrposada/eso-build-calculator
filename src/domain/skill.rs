use super::{
    formulas, BonusData, CharacterStats, ClassName, DamageFlags, ExecuteData, ExecuteScaling,
    ResolveContext, Resource, SkillDamage, SkillLineName, SkillMechanic,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillData {
    pub name: String,
    pub base_skill_name: String,
    pub class_name: ClassName,
    pub skill_line: SkillLineName,
    pub resource: Resource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage: Option<SkillDamage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute: Option<ExecuteData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonuses: Option<Vec<BonusData>>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub spammable: bool,
}

// Builder
impl SkillData {
    pub fn new(
        name: impl Into<String>,
        base_skill_name: impl Into<String>,
        class_name: ClassName,
        skill_line: SkillLineName,
        resource: Resource,
    ) -> Self {
        Self {
            name: name.into(),
            base_skill_name: base_skill_name.into(),
            class_name,
            skill_line,
            resource,
            damage: None,
            channel_time: None,
            execute: None,
            bonuses: None,
            spammable: false,
        }
    }

    pub fn with_damage(mut self, damage: SkillDamage) -> Self {
        self.damage = Some(damage);
        self
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
}

impl SkillData {
    pub fn calculate_damage_per_cast(
        &self,
        bonuses: &[BonusData],
        stats: &CharacterStats,
        enemy_health: Option<f64>,
    ) -> f64 {
        let max_stat = stats.max_stat();
        let max_power = stats.max_power();

        let mut total_damage_per_cast = 0.0;

        let ctx = ResolveContext::default();
        let applicable: Vec<_> = bonuses
            .iter()
            .filter(|b| {
                b.skill_line_filter.map_or(true, |sl| sl == self.skill_line)
                    && match b.execute_threshold {
                        Some(threshold) => enemy_health.map_or(false, |h| h <= threshold),
                        None => true,
                    }
            })
            .map(|b| b.resolve(&ctx))
            .collect();

        if let Some(damage) = &self.damage {
            if let Some(hits) = &damage.hits {
                for hit in hits {
                    let modifier: f64 = applicable
                        .iter()
                        .filter(|bv| hit.flags.matches_bonus_target(bv.target))
                        .map(|bv| bv.value)
                        .sum();

                    let hit_value = hit.effective_value(max_stat, max_power);

                    if let Some(threshold) = hit.execute_threshold {
                        if enemy_health.map_or(false, |h| h < threshold) {
                            total_damage_per_cast += hit_value * (1.0 + modifier);
                        }
                    } else {
                        total_damage_per_cast += hit_value * (1.0 + modifier);
                    }
                }
            }

            if let Some(dots) = &damage.dots {
                for dot in dots {
                    let modifier: f64 = applicable
                        .iter()
                        .filter(|bv| dot.flags.matches_bonus_target(bv.target))
                        .map(|bv| bv.value)
                        .sum();

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
                            total_damage_per_cast += tick_damage;
                        } else {
                            total_damage_per_cast += tick_damage * (1.0 + modifier);
                        }
                    }
                }
            }
        }

        if let Some(execute) = &self.execute {
            if let Some(health) = enemy_health {
                total_damage_per_cast *= execute.calculate_multiplier(health);
            }
        }

        let armor_factor = formulas::armor_damage_factor(stats.target_armor, stats.penetration);
        let crit_mult = formulas::critical_multiplier(stats.critical_chance, stats.critical_damage);

        total_damage_per_cast * armor_factor * crit_mult
    }

    pub fn mechanic(&self) -> SkillMechanic {
        if self.channel_time.is_some() {
            return SkillMechanic::Channeled;
        }

        if let Some(damage) = &self.damage {
            if damage.dots.as_ref().is_some_and(|d| !d.is_empty()) {
                return SkillMechanic::Dot;
            }

            if let Some(hits) = &damage.hits {
                if !hits.is_empty() {
                    return SkillMechanic::Instant;
                }
            }
        }

        // Default to Instant for skills with no damage
        SkillMechanic::Instant
    }

    pub fn duration(&self) -> f64 {
        if let Some(damage) = &self.damage {
            if let Some(dots) = &damage.dots {
                if !dots.is_empty() {
                    return dots
                        .iter()
                        .map(|dot| dot.duration + dot.delay.unwrap_or(0.0))
                        .fold(0.0, f64::max);
                }
            }
        }
        self.channel_time.unwrap_or(0.0)
    }
}

// Format
impl SkillData {
    fn primary_flags(&self) -> Option<DamageFlags> {
        let damage = self.damage.as_ref()?;
        if let Some(hits) = &damage.hits {
            if let Some(hit) = hits.first() {
                return Some(hit.flags);
            }
        }
        if let Some(dots) = &damage.dots {
            if let Some(dot) = dots.first() {
                return Some(dot.flags);
            }
        }
        None
    }

    fn fmt_header(&self) -> Vec<String> {
        vec!["=".repeat(60), format!("  {}", self.name), "=".repeat(60)]
    }

    fn fmt_basic_info(&self) -> Vec<String> {
        let mut lines = vec![
            "  Basic Info".to_string(),
            format!("  {}", "-".repeat(56)),
            format!("  Base Skill:      {}", self.base_skill_name),
            format!("  Source:          {}", self.class_name),
            format!("  Skill Line:      {}", self.skill_line),
            format!("  Resource:        {}", self.resource),
        ];

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

        lines
    }

    fn fmt_damage(&self) -> Vec<String> {
        let mut lines = vec!["  Damage".to_string(), format!("  {}", "-".repeat(56))];

        let stats = CharacterStats::default();
        let max_stat = stats.max_stat();
        let max_power = stats.max_power();

        if let Some(damage) = &self.damage {
            if let Some(hits) = &damage.hits {
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
                        let value = hit.effective_value(max_stat, max_power);
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

            if let Some(dots) = &damage.dots {
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
                        let value = dot.effective_value(max_stat, max_power);
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
        }

        lines
    }

    fn fmt_calculated(&self) -> Vec<String> {
        let mut lines = vec!["  Calculated".to_string(), format!("  {}", "-".repeat(56))];

        let duration = self.duration();
        let duration_str = if duration > 0.0 {
            format!("{}s", duration)
        } else {
            "instant".to_string()
        };
        lines.push(format!("  Duration:        {}", duration_str));
        let default_stats = CharacterStats::default();
        lines.push(format!(
            "  Damage/Cast:     {:.0}",
            self.calculate_damage_per_cast(&[], &default_stats, None)
        ));

        let has_execute_hits = self
            .damage
            .as_ref()
            .and_then(|d| d.hits.as_ref())
            .map_or(false, |hits| {
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
                self.calculate_damage_per_cast(&[], &default_stats, Some(0.0))
            ));
        } else if has_execute_hits {
            lines.push(format!(
                "  Damage @0% HP:   {:.0}",
                self.calculate_damage_per_cast(&[], &default_stats, Some(0.0))
            ));
        }

        lines
    }

    fn fmt_bonuses(&self) -> Vec<String> {
        let mut lines = Vec::new();

        if let Some(bonuses) = &self.bonuses {
            if !bonuses.is_empty() {
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

        lines
    }
}

impl std::fmt::Display for SkillData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();
        lines.extend(self.fmt_header());
        lines.push(String::new());
        lines.extend(self.fmt_basic_info());
        lines.push(String::new());
        lines.extend(self.fmt_damage());
        lines.push(String::new());
        lines.extend(self.fmt_calculated());

        let bonus_lines = self.fmt_bonuses();
        if !bonus_lines.is_empty() {
            lines.push(String::new());
            lines.extend(bonus_lines);
        }

        write!(f, "{}", lines.join("\n"))
    }
}
