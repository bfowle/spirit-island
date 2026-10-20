//! Growth-option resolution + presence-track slot reveal math.
//!
//! The invariant this enforces: a single growth-placement reveals *one*
//! track slot. Failing to respect this is the class of bug caught on
//! Shadows Opening A (prior draft assumed both tracks revealed simultaneously).

use serde::{Deserialize, Serialize};

/// Wiki-shorthand growth-effect tokens parsed from the `first`/`second`/`third`
/// fields of a Spirit's growth options. Unknown tokens land in `Other(String)`
/// — these are spirit-specific (e.g. `Thunder1`, `Night`, `Spread`) and must
/// be interpreted by consulting the spirit's panel text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GrowthEffect {
    Reclaim,
    Reclaim1,
    Gain1Power,
    Gain2Power,
    /// Place 1 Presence from a track, at the given Range.
    AddPresence {
        range: u8,
    },
    /// +N Energy this turn (growth-effect only, not a track reveal).
    Energy(u8),
    /// +N card plays this turn.
    CardPlay(u8),
    /// Movement of an existing presence (range N).
    MovePresence {
        range: u8,
    },
    /// Any spirit-specific token the parser didn't recognise.
    Other(String),
}

impl GrowthEffect {
    /// Parse a Wiki-shorthand token into a structured `GrowthEffect`.
    pub fn parse(raw: &str) -> Self {
        match raw {
            "reclaim" => Self::Reclaim,
            "reclaim1" => Self::Reclaim1,
            "gain1p" => Self::Gain1Power,
            "gain2p" => Self::Gain2Power,
            s if s.starts_with("addpresence") => {
                let range = s.trim_start_matches("addpresence").parse().unwrap_or(0);
                Self::AddPresence { range }
            }
            s if s.starts_with("energy") => {
                let n = s.trim_start_matches("energy").parse().unwrap_or(0);
                Self::Energy(n)
            }
            s if s.starts_with("card") => {
                let n = s.trim_start_matches("card").parse().unwrap_or(0);
                Self::CardPlay(n)
            }
            s if s.starts_with("movepresence") => {
                let range = s.trim_start_matches("movepresence").parse().unwrap_or(0);
                Self::MovePresence { range }
            }
            other => Self::Other(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrowthOption {
    pub index: u8,
    pub effects: Vec<GrowthEffect>,
}

/// Represents which physical track a presence placement draws from.
/// Only one track gets revealed per placement — enforcing this at the type
/// level prevents the Shadows T1 authoring bug at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackChoice {
    Energy,
    CardPlay,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_tokens() {
        assert_eq!(GrowthEffect::parse("reclaim"), GrowthEffect::Reclaim);
        assert_eq!(GrowthEffect::parse("gain1p"), GrowthEffect::Gain1Power);
        assert_eq!(
            GrowthEffect::parse("addpresence3"),
            GrowthEffect::AddPresence { range: 3 }
        );
        assert_eq!(GrowthEffect::parse("energy3"), GrowthEffect::Energy(3));
    }

    #[test]
    fn unknown_tokens_are_preserved() {
        match GrowthEffect::parse("Thunder2") {
            GrowthEffect::Other(s) => assert_eq!(s, "Thunder2"),
            _ => panic!("expected Other"),
        }
    }
}
