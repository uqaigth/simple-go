use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub enum Stone {
    Black,
    White,
    Chi,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RuleSet {
    // 中国规则，数子法，贴7.5目
    China,
    // 日本规则，数目法，贴6.5目
    Japan,
}


#[derive(PartialEq, Copy, Clone)]
pub struct Step {
    pub stone: Stone,
    pub step: u16,
    pub pos: (i8, i8),
}


#[derive(PartialEq, Clone)]
pub struct GameRecord {
    pub rule_set: RuleSet,
    pub grid_size: i8,
    pub step: u16,
    pub steps: Vec<Step>,
    pub next: Stone,
    pub capture: HashMap<Stone, u16>,
}
