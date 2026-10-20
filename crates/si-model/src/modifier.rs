//! Rule changes from the base game, with their source. Every spirit, every
//! pairing and every adversary level is a different baseline: the base rules
//! plus a list of modifiers, each one traceable to printed text.

use serde::{Deserialize, Serialize};

use crate::spirit::RuleHook;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Source {
    Base,
    Adversary { id: String, level: u8 },
    Scenario { id: String },
    Spirit { id: String, rule: String },
    Aspect { spirit: String, aspect: String, rule: String },
    EventCard { id: String },
    FearCard { id: String },
    BlightCard { id: String },
    Errata { reference: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Modifier {
    pub source: Source,
    pub title: String,
    pub text: String,
    /// Typed when the rules engine knows what to do with it.
    pub hook: Option<RuleHook>,
}

/// The rules in force for one game.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleSet {
    pub modifiers: Vec<Modifier>,
}

impl RuleSet {
    pub fn typed(&self) -> impl Iterator<Item = (&Source, &RuleHook)> {
        self.modifiers.iter().filter_map(|m| m.hook.as_ref().map(|h| (&m.source, h)))
    }

    pub fn untyped(&self) -> impl Iterator<Item = &Modifier> {
        self.modifiers.iter().filter(|m| m.hook.is_none())
    }
}
