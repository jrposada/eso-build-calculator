pub const BUILD_CONSTRAINTS: BuildConstraints = BuildConstraints {
    skill_count: 10,
    champion_point_count: 4,
    class_skill_line_count: 3,
    weapon_skill_line_count: 2,
};

#[derive(Debug, Clone, Copy)]
pub struct BuildConstraints {
    pub skill_count: usize,
    pub champion_point_count: usize,
    pub class_skill_line_count: usize,
    pub weapon_skill_line_count: usize,
}
