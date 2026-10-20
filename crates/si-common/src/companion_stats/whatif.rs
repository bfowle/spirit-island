//! What-if: changes a player tries on the current moment, applied to a copy
//! of the public snapshot so every stat recomputes as if they had happened
//! ("show what changes if you hit land X, never 'play this'": the stat
//! models report, displays that build judgment). Nothing is written back to
//! the game.

use crate::public::{PublicLand, PublicSnapshot};
use crate::rules;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Edit {
    /// Destroy one Invader ("Explorer", "Town", "City") in a land. Towns
    /// and Cities give their Fear.
    Destroy { land: String, kind: String },
    /// Defend in a land this turn.
    Defend { land: String, amount: i64 },
    /// Add one Dahan to a land.
    Dahan { land: String },
    /// Take one presence from a spirit's track ("Energy", "CardPlays"):
    /// its next node is revealed.
    Track { spirit: String, track: String },
}

impl Edit {
    pub fn land(&self) -> Option<&str> {
        match self {
            Edit::Destroy { land, .. } | Edit::Defend { land, .. } | Edit::Dahan { land } => Some(land),
            Edit::Track { .. } => None,
        }
    }

    /// "Destroy a Town in NE7", for the what-if list.
    pub fn describe(&self, short: impl Fn(&str) -> String) -> String {
        match self {
            Edit::Destroy { land, kind } => format!("Destroy {} in {}", kind, short(land)),
            Edit::Defend { land, amount } => format!("Defend {amount} in {}", short(land)),
            Edit::Dahan { land } => format!("+1 Dahan in {}", short(land)),
            Edit::Track { track, .. } => format!("Presence from the {} track", if track == "CardPlays" { "Card Plays" } else { track }),
        }
    }

    /// Parse "destroy:NE7:Town", "defend:NE7:4", "dahan:A3", "track:Spirit:Energy";
    /// a land may be its id or its short name ("NE7").
    pub fn parse(text: &str, s: &PublicSnapshot) -> Option<Edit> {
        let parts: Vec<&str> = text.split(':').collect();
        let land = |name: &str| -> Option<String> {
            s.lands
                .iter()
                .find(|l| l.id == name || short_name(l) == name)
                .map(|l| l.id.clone())
        };
        match parts.as_slice() {
            ["destroy", l, kind] => Some(Edit::Destroy { land: land(l)?, kind: kind.to_string() }),
            ["defend", l, n] => Some(Edit::Defend { land: land(l)?, amount: n.parse().ok()? }),
            ["dahan", l] => Some(Edit::Dahan { land: land(l)? }),
            ["track", spirit, track] => {
                let id = s.spirits.iter().find(|x| x.id == *spirit || x.title == *spirit).map(|x| x.id.clone()).or_else(|| s.spirits.first().map(|x| x.id.clone()))?;
                Some(Edit::Track { spirit: id, track: track.to_string() })
            }
            _ => None,
        }
    }
}

/// "NE7" for a NorthEast board land 7, "D2" for board D.
fn short_name(l: &PublicLand) -> String {
    let b: String = if l.board.len() <= 2 { l.board.clone() } else { l.board.chars().filter(|c| c.is_ascii_uppercase()).collect() };
    format!("{b}{}", l.number)
}

pub fn apply(s: &PublicSnapshot, edits: &[Edit]) -> PublicSnapshot {
    let mut n = s.clone();
    for e in edits {
        match e {
            Edit::Destroy { land, kind } => {
                let Some(l) = n.lands.iter_mut().find(|l| l.id == *land) else { continue };
                let count = match kind.as_str() {
                    "Explorer" => &mut l.explorers,
                    "Town" => &mut l.towns,
                    "City" => &mut l.cities,
                    _ => continue,
                };
                if *count == 0 {
                    continue;
                }
                *count -= 1;
                if let Some(i) = l.pieces.iter().position(|p| p.kind == *kind) {
                    l.pieces.remove(i);
                }
                fear(&mut n, rules::destroy_fear(kind));
            }
            Edit::Defend { land, amount } => {
                if let Some(l) = n.lands.iter_mut().find(|l| l.id == *land) {
                    l.defend += amount;
                }
            }
            Edit::Dahan { land } => {
                if let Some(l) = n.lands.iter_mut().find(|l| l.id == *land) {
                    l.dahan += 1;
                    l.pieces.push(crate::public::PublicPiece { kind: "Dahan".into(), max_health: 2, damage: 0, strife: 0 });
                }
            }
            Edit::Track { spirit, track } => {
                let Some(sp) = n.spirits.iter_mut().find(|x| x.id == *spirit) else { continue };
                let prefix = format!("{}{track}", sp.id);
                let next = sp
                    .track
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| !x.revealed && x.id.strip_prefix(&prefix).is_some_and(|rest| rest.chars().all(|c| c.is_ascii_digit())))
                    .min_by_key(|(_, x)| x.id.strip_prefix(&prefix).and_then(|r| r.parse::<u32>().ok()).unwrap_or(u32::MAX))
                    .map(|(i, _)| i);
                if let Some(i) = next {
                    sp.track[i].revealed = true;
                    sp.presence_on_island += 1;
                }
            }
        }
    }
    n
}

/// Fear generated: cards are earned as the pool fills, off the top of the
/// current Terror Level's section; emptying a section raises the level.
fn fear(n: &mut PublicSnapshot, amount: i64) {
    let f = &mut n.fear;
    f.generated += amount;
    let req = f.required_per_card.max(1);
    while f.generated >= req {
        let Some(section) = f.deck_by_level.iter().position(|c| *c > 0) else { break };
        f.generated -= req;
        f.deck_by_level[section] -= 1;
        f.earned_count += 1;
        if f.deck_by_level[section] == 0 && section < 2 {
            f.terror_level = f.terror_level.max(section as u8 + 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::public::{PublicPiece, PublicSpirit, TrackNode};

    fn snap() -> PublicSnapshot {
        let mut l = PublicLand { id: "MountainNE7".into(), board: "NorthEast".into(), number: 7, terrain: "Mountain".into(), towns: 1, explorers: 2, ..Default::default() };
        l.pieces = vec![
            PublicPiece { kind: "Town".into(), max_health: 2, ..Default::default() },
            PublicPiece { kind: "Explorer".into(), max_health: 1, ..Default::default() },
            PublicPiece { kind: "Explorer".into(), max_health: 1, ..Default::default() },
        ];
        let mut s = PublicSnapshot { lands: vec![l], ..Default::default() };
        s.fear.required_per_card = 4;
        s.fear.generated = 3;
        s.fear.terror_level = 1;
        s.fear.deck_by_level = [1, 3, 3];
        s.spirits = vec![PublicSpirit {
            id: "R".into(),
            track: vec![
                TrackNode { id: "REnergy1".into(), revealed: true, kind: "EnergyPerTurn".into(), value: Some(1), elements: vec![] },
                TrackNode { id: "REnergy2".into(), revealed: false, kind: "EnergyPerTurn".into(), value: Some(2), elements: vec![] },
            ],
            ..Default::default()
        }];
        s
    }

    #[test]
    fn destroying_a_town_gives_fear_and_can_raise_the_level() {
        let s = snap();
        let e = Edit::parse("destroy:NE7:Town", &s).unwrap();
        let n = apply(&s, &[e]);
        let l = &n.lands[0];
        assert_eq!((l.towns, l.pieces.len()), (0, 2));
        // 3 + 1 Fear fills the card: the last Terror I card is earned.
        assert_eq!((n.fear.generated, n.fear.earned_count, n.fear.deck_by_level, n.fear.terror_level), (0, 1, [0, 3, 3], 2));
    }

    #[test]
    fn track_reveals_the_next_node() {
        let s = snap();
        let n = apply(&s, &[Edit::parse("track:R:Energy", &s).unwrap()]);
        assert!(n.spirits[0].track[1].revealed);
        assert_eq!(n.spirits[0].presence_on_island, 1);
    }

    #[test]
    fn defend_and_unknown_land() {
        let s = snap();
        assert!(Edit::parse("defend:XX9:2", &s).is_none());
        let n = apply(&s, &[Edit::parse("defend:MountainNE7:3", &s).unwrap()]);
        assert_eq!(n.lands[0].defend, 3);
    }
}
