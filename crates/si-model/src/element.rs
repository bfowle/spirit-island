use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

/// The eight elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

    /// Engine and Wiki spellings (`"Sun"`, `"sun"`, `"{Sun}"`).
    pub fn parse(s: &str) -> Option<Element> {
        let t = s.trim().trim_matches(|c| c == '{' || c == '}').to_ascii_lowercase();
        Some(match t.as_str() {
            "sun" => Element::Sun,
            "moon" => Element::Moon,
            "fire" => Element::Fire,
            "air" => Element::Air,
            "water" => Element::Water,
            "earth" => Element::Earth,
            "plant" => Element::Plant,
            "animal" => Element::Animal,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            Element::Sun => "Sun",
            Element::Moon => "Moon",
            Element::Fire => "Fire",
            Element::Air => "Air",
            Element::Water => "Water",
            Element::Earth => "Earth",
            Element::Plant => "Plant",
            Element::Animal => "Animal",
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A bag of elements with counts: a card's elements, a track's permanent
/// elements, an innate threshold, or everything a spirit has this turn.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementSet(pub BTreeMap<Element, u8>);

impl ElementSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_list<I: IntoIterator<Item = Element>>(items: I) -> Self {
        let mut s = Self::new();
        for e in items {
            s.add(e, 1);
        }
        s
    }

    pub fn add(&mut self, e: Element, n: u8) {
        if n == 0 {
            return;
        }
        *self.0.entry(e).or_insert(0) += n;
    }

    pub fn extend(&mut self, other: &ElementSet) {
        for (e, n) in &other.0 {
            self.add(*e, *n);
        }
    }

    pub fn count(&self, e: Element) -> u8 {
        self.0.get(&e).copied().unwrap_or(0)
    }

    pub fn total(&self) -> u32 {
        self.0.values().map(|n| *n as u32).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }

    /// Does this bag reach `threshold` in every element the threshold names?
    pub fn satisfies(&self, threshold: &ElementSet) -> bool {
        threshold.0.iter().all(|(e, n)| self.count(*e) >= *n)
    }

    /// How many elements short of `threshold`, summed over the elements.
    pub fn shortfall(&self, threshold: &ElementSet) -> u32 {
        threshold.0.iter().map(|(e, n)| n.saturating_sub(self.count(*e)) as u32).sum()
    }
}

impl fmt::Display for ElementSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self
            .0
            .iter()
            .filter(|(_, n)| **n > 0)
            .map(|(e, n)| if *n == 1 { e.name().to_string() } else { format!("{n} {e}") })
            .collect();
        f.write_str(&parts.join(" + "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thresholds_compare_per_element() {
        let mut have = ElementSet::new();
        have.add(Element::Sun, 2);
        have.add(Element::Fire, 1);
        let mut need = ElementSet::new();
        need.add(Element::Sun, 2);
        need.add(Element::Fire, 2);
        assert!(!have.satisfies(&need));
        assert_eq!(have.shortfall(&need), 1);
        have.add(Element::Fire, 1);
        assert!(have.satisfies(&need));
        assert_eq!(format!("{have}"), "2 Sun + 2 Fire");
    }

    #[test]
    fn parses_engine_spellings() {
        assert_eq!(Element::parse("{Animal}"), Some(Element::Animal));
        assert_eq!(Element::parse("water"), Some(Element::Water));
        assert_eq!(Element::parse("Any"), None);
    }
}
