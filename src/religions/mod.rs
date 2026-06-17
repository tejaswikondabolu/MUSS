pub mod cpp;
pub mod javascript;
pub mod python;
pub mod rust;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Doctrine {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct Relic {
    pub name: &'static str,
    pub description: &'static str,
    pub power: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct Miracle {
    pub title: &'static str,
    pub code: &'static str,
    pub explanation: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct Commandment {
    pub text: &'static str,
    pub meaning: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct Religion {
    pub id: &'static str,
    pub name: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub deity: &'static str,
    pub core_belief: &'static str,
    pub sacred_text: &'static str,
    pub founded: &'static str,
    pub forbidden_practices: Vec<&'static str>,
    pub doctrines: Vec<Doctrine>,
    pub relics: Vec<Relic>,
    pub miracles: Vec<Miracle>,
    pub commandments: Vec<Commandment>,
    pub history: &'static str,
    pub danger_level: &'static str,
    pub color: &'static str,
}

pub fn all_religions() -> Vec<Religion> {
    vec![
        rust::church_of_rust(),
        javascript::cult_of_javascript(),
        python::order_of_python(),
        cpp::ancient_empire(),
    ]
}

pub fn get_religion(id: &str) -> Option<Religion> {
    all_religions().into_iter().find(|r| r.id == id)
}
