//! The 8 Spirit Island elements — Sun/Moon/Fire/Air/Water/Earth/Plant/Animal.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Element {
    Sun,
    Moon,
    Fire,
    Air,
    Water,
    Earth,
    Plant,
    Animal,
}

impl Element {
    pub const ALL: [Element; 8] = [
        Element::Sun,
        Element::Moon,
        Element::Fire,
        Element::Air,
        Element::Water,
        Element::Earth,
        Element::Plant,
        Element::Animal,
    ];

    pub fn short(self) -> &'static str {
        match self {
            Element::Sun => "S",
            Element::Moon => "M",
            Element::Fire => "F",
            Element::Air => "A",
            Element::Water => "W",
            Element::Earth => "E",
            Element::Plant => "P",
            Element::Animal => "An",
        }
    }
}
