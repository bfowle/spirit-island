//! Invader forecast: the coming Invader Phase, resolved in its real order
//! (Ravage → Build → Explore), the next turn's pipeline, and card counting.
//!
//! Each invader card passes Explore → Build → Ravage on three successive
//! turns. Inside one Invader Phase the order is reversed: Ravage first, then
//! Build, then Explore. So a land's Explore odds are computed on the board
//! *after* this phase's Builds (a new Town is a new Explore source), and the
//! card in the Build slot names next turn's Ravage lands.

use crate::public::{PublicInvaderCard, PublicLand, PublicSnapshot};
use crate::rules::{self, RavageLedger};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InvaderPhaseForecast {
    /// The turn whose Invader Phase this describes.
    pub turn: u32,
    /// Where the game is relative to it: "before", "ravage", "build", "explore".
    pub position: String,
    pub ravage_cards: Vec<PublicInvaderCard>,
    pub build_cards: Vec<PublicInvaderCard>,
    pub ravages: Vec<RavageForecast>,
    pub builds: Vec<BuildForecast>,
    pub explore: ExploreForecast,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RavageForecast {
    pub land: String,
    pub ledger: RavageLedger,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BuildForecast {
    pub land: String,
    pub adds: String,
    pub done: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExploreForecast {
    /// Set when the Explore card is already face up.
    pub revealed: Option<PublicInvaderCard>,
    /// Stage on the back of the next card; `None` if the deck is empty.
    pub next_stage: Option<u8>,
    /// Empty deck: the spirits lose at this Explore.
    pub deck_empty: bool,
    pub candidates: Vec<ExploreCandidate>,
    /// Per land: (cards that explore it, candidate cards).
    pub per_land: BTreeMap<String, (u32, u32)>,
    pub pocket_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExploreCandidate {
    pub card: PublicInvaderCard,
    pub lands: Vec<String>,
    /// Lands that match the card but have no Explore source in range.
    pub blocked: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StageCount {
    pub stage: u8,
    pub seen: Vec<String>,
    pub unseen: Vec<PublicInvaderCard>,
    pub left_in_deck: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LandOutlook {
    pub id: String,
    pub terrain: String,
    pub coastal: bool,
    pub explorers: u32,
    pub towns: u32,
    pub cities: u32,
    pub dahan: u32,
    pub blight: u32,
    pub presence: u32,
    /// "coastal" (always explorable), "fed" (an inland source in range),
    /// "dead" (no source in range after this phase's Builds), "ocean".
    pub explore_access: String,
    /// Explore sources in range after this phase's Builds (the land itself included).
    pub fed_by: Vec<String>,
    /// Inland lands whose only Explore source is this land.
    pub sole_source_for: Vec<String>,
    pub ravage_now: Option<RavageLedger>,
    pub build_now: Option<String>,
    /// (cards that explore it, candidates).
    pub explore_chance: (u32, u32),
    pub ravage_next: Ravage2,
    /// (cards that would build here next turn, candidates).
    pub build_next_chance: (u32, u32),
    pub badges: Vec<String>,
    /// What an action here changes if it resolves in the Fast phase / Slow phase.
    pub fast_window: Vec<String>,
    pub slow_window: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum Ravage2 {
    #[default]
    No,
    /// Matches next turn's Ravage card and already has Invaders.
    Certain,
    /// Matches it and has Invaders only if explored this phase.
    IfExplored,
}

/// Steps after which this turn's Invader Phase is complete.
fn phase_position(step: &str) -> &'static str {
    match step {
        "Ravage" => "ravage",
        "Build" => "build",
        "Explore" => "explore",
        "AdvanceInvaderCards" | "SlowPower" | "TimePasses" => "after",
        _ => "before",
    }
}

pub struct InvaderView {
    pub phase: InvaderPhaseForecast,
    pub lands: Vec<LandOutlook>,
    pub stages: Vec<StageCount>,
}

pub fn forecast(s: &PublicSnapshot) -> InvaderView {
    let nbrs = rules::neighbors(s);
    let position = phase_position(&s.step);
    // After this turn's Invader Phase the slots already hold next turn's cards.
    let (turn, position) = if position == "after" { (s.turn + 1, "before") } else { (s.turn, position) };
    let ravage_done = matches!(position, "build" | "explore");
    let build_done = position == "explore";
    let inv = &s.invaders;
    let mut notes = Vec::new();
    for (slot, cards) in &inv.extra_slots {
        if !cards.is_empty() {
            notes.push(format!("{slot} holds {}: an extra invader action this turn (see rules register)", titles(cards)));
        }
    }

    // ---- Ravage (first)
    let ravages: Vec<RavageForecast> = s
        .lands
        .iter()
        .filter(|l| inv.ravage.iter().any(|c| rules::ravages_in(c, l)))
        .map(|l| RavageForecast { land: l.id.clone(), ledger: rules::ravage_ledger(l, &s.status_effects), done: ravage_done })
        .collect();

    // A counterattack that destroys every Invader whatever the assignment
    // clears the land before Build and Explore: it stops this turn's Build
    // there and can remove an Explore source (Test 1 Turn 3: Jungle E3's
    // Dahan destroyed the Town feeding Mountain E5 and Wetland E4).
    let mut after_build: Vec<PublicLand> = s.lands.clone();
    if !ravage_done {
        for r in ravages.iter().filter(|r| r.ledger.counterattack_clears && r.ledger.skip_available.is_none()) {
            if let Some(l) = after_build.iter_mut().find(|l| l.id == r.land) {
                l.explorers = 0;
                l.towns = 0;
                l.cities = 0;
                l.pieces.retain(|p| !rules::is_invader(&p.kind));
                notes.push(format!("{}: the Dahan counterattack destroys every Invader before Build and Explore", l.id));
            }
        }
    }

    // ---- Build (second), then the board Explore sees.
    let mut builds = Vec::new();
    for l in after_build.iter_mut() {
        if inv.build.iter().any(|c| rules::builds_in(c, l)) {
            let (piece, bnotes) = rules::build_outcome(l, &s.status_effects);
            if !build_done {
                match piece {
                    Some("City") => l.cities += 1,
                    Some(_) => l.towns += 1,
                    None => {}
                }
            }
            if let Some(p) = piece {
                builds.push(BuildForecast { land: l.id.clone(), adds: p.to_string(), done: build_done, notes: bnotes });
            } else {
                notes.push(format!("{}: no Build ({})", l.id, bnotes.join("; ")));
            }
        }
    }

    // ---- Explore (third)
    let seen: BTreeSet<&str> = inv
        .explore
        .iter()
        .chain(&inv.build)
        .chain(&inv.ravage)
        .chain(&inv.discard)
        .chain(inv.extra_slots.iter().flat_map(|(_, v)| v))
        .map(|c| c.id.as_str())
        .collect();
    let stages: Vec<StageCount> = (1..=3u8)
        .map(|st| StageCount {
            stage: st,
            seen: inv.pool.iter().filter(|c| c.stage == st && seen.contains(c.id.as_str())).map(|c| c.id.clone()).collect(),
            unseen: inv.pool.iter().filter(|c| c.stage == st && !seen.contains(c.id.as_str())).cloned().collect(),
            left_in_deck: inv.upcoming_stages.iter().filter(|x| **x == st).count() as u32,
        })
        .collect();

    let isolated = rules::isolated_lands(&s.status_effects);
    let fed = |land: &PublicLand| -> Vec<&str> {
        rules::explore_sources_for(land, &after_build, &nbrs).into_iter().filter(|src| !isolated.contains(*src)).collect()
    };
    let revealed = inv.explore.first().cloned();
    let next_stage = inv.upcoming_stages.first().copied();
    let candidates_cards: Vec<PublicInvaderCard> = match (&revealed, next_stage) {
        (Some(c), _) => vec![c.clone()],
        (None, Some(st)) => stages[(st - 1) as usize].unseen.clone(),
        (None, None) => Vec::new(),
    };
    let candidates: Vec<ExploreCandidate> = candidates_cards
        .iter()
        .map(|c| {
            let matching: Vec<&PublicLand> = after_build.iter().filter(|l| rules::card_matches(c, l)).collect();
            let open = |l: &&&PublicLand| {
                !isolated.contains(&l.id) && !fed(l).is_empty() && !rules::wilds_stop_explore(l, &s.status_effects)
            };
            ExploreCandidate {
                card: c.clone(),
                lands: matching.iter().filter(|l| open(l)).map(|l| l.id.clone()).collect(),
                blocked: matching
                    .iter()
                    .filter(|l| !open(l))
                    .map(|l| {
                        if isolated.contains(&l.id) {
                            format!("{} (Isolated)", l.id)
                        } else if fed(l).is_empty() {
                            l.id.clone()
                        } else {
                            format!("{} (Wilds)", l.id)
                        }
                    })
                    .collect(),
            }
        })
        .collect();
    let den = candidates.len() as u32;
    let mut per_land: BTreeMap<String, (u32, u32)> = BTreeMap::new();
    for l in s.lands.iter().filter(|l| !l.is_ocean) {
        let num = candidates.iter().filter(|c| c.lands.contains(&l.id)).count() as u32;
        per_land.insert(l.id.clone(), (num, den));
    }
    let pocket_rule = match (revealed.is_some(), next_stage) {
        (true, _) => "The Explore card is face up: these lands explore.".into(),
        (false, Some(st)) => format!(
            "Next card is Stage {st}: each of the {den} unseen Stage {st} cards is equally likely. Chance a land explores = cards that hit it ÷ {den}."
        ),
        (false, None) => "The Invader deck is empty: the spirits lose at the next Explore.".into(),
    };
    let explore = ExploreForecast { revealed, next_stage, deck_empty: next_stage.is_none() && inv.explore.is_empty(), candidates, per_land, pocket_rule };
    // Escalation: on a flagged card, the adversary's Escalation resolves
    // (for most adversaries before this Explore; see the rules register).
    let escalating = match &explore.revealed {
        Some(c) => c.escalation as u32,
        None => explore.candidates.iter().filter(|c| c.card.escalation).count() as u32,
    };
    if escalating > 0 && !build_done {
        for a in &s.adversaries {
            if let Some((title, text, _)) = crate::rules_register::escalation(&a.id) {
                let text = text.replace("<i>", "").replace("</i>", "");
                notes.push(format!(
                    "Escalation ({title}) if the card is flagged ({escalating} of {}): {text} Unless it says otherwise it resolves before the Explore, so Towns it adds are Explore sources this turn; these odds don't include them yet (FAQ; open-questions NQ-G).",
                    explore.candidates.len()
                ));
            }
        }
    }

    // ---- next turn: the Build card becomes the Ravage card, this Explore card the Build card.
    let explored_now = |id: &str| explore.per_land.get(id).copied().unwrap_or((0, 0));
    let mut lands = Vec::new();
    for (l, lb) in s.lands.iter().zip(&after_build) {
        let ravage_now = ravages.iter().find(|r| r.land == l.id).map(|r| r.ledger.clone());
        let build_now = builds.iter().find(|b| b.land == l.id).map(|b| b.adds.clone());
        let fed_by: Vec<String> = fed(lb).into_iter().map(String::from).collect();
        let explore_access = if l.is_ocean {
            "ocean"
        } else if fed_by.iter().any(|f| s.lands.iter().any(|x| x.id == *f && x.is_ocean)) || l.coastal {
            "coastal"
        } else if fed_by.is_empty() {
            "dead"
        } else {
            "fed"
        };
        let (en, ed) = explored_now(&l.id);
        let matches_build = inv.build.iter().any(|c| rules::card_matches(c, l));
        let ravage_next = if !matches_build || l.is_ocean {
            Ravage2::No
        } else if rules::has_invaders(lb) {
            Ravage2::Certain
        } else if en > 0 {
            Ravage2::IfExplored
        } else {
            Ravage2::No
        };
        let will_have_invaders = rules::has_invaders(lb) || !fed_by.is_empty();
        let build_next_num = if l.is_ocean {
            0
        } else {
            explore.candidates.iter().filter(|c| rules::card_matches(&c.card, l) && will_have_invaders).count() as u32
        };
        let sole_source_for: Vec<String> = if rules::is_explore_source(lb) && !lb.is_ocean {
            after_build
                .iter()
                .filter(|x| !x.is_ocean && !x.coastal && x.id != l.id)
                .filter(|x| {
                    let f = fed(x);
                    f.len() == 1 && f[0] == lb.id
                })
                .map(|x| x.id.clone())
                .collect()
        } else {
            Vec::new()
        };

        let mut badges = Vec::new();
        if ravage_now.is_some() {
            badges.push("ravage-now".to_string());
        }
        if build_now.is_some() {
            badges.push("build-now".into());
        }
        if ravage_next == Ravage2::Certain {
            badges.push("ravage-next".into());
        }
        if ravage_now.is_some() && ravage_next == Ravage2::Certain {
            badges.push("back-to-back".into());
        }
        if explore_access == "dead" && !rules::has_invaders(l) {
            badges.push("explore-dead".into());
        }
        if !sole_source_for.is_empty() {
            badges.push("sole-source".into());
        }

        let (fast_window, slow_window) =
            timing_windows(l, ravage_now.as_ref(), build_now.as_deref(), (en, ed), &ravage_next, &fed_by, &sole_source_for, position);

        lands.push(LandOutlook {
            id: l.id.clone(),
            terrain: l.terrain.clone(),
            coastal: l.coastal,
            explorers: l.explorers,
            towns: l.towns,
            cities: l.cities,
            dahan: l.dahan,
            blight: l.blight,
            presence: l.presence.values().sum(),
            explore_access: explore_access.into(),
            fed_by,
            sole_source_for,
            ravage_now,
            build_now,
            explore_chance: (en, ed),
            ravage_next,
            build_next_chance: (build_next_num, ed),
            badges,
            fast_window,
            slow_window,
        });
    }

    InvaderView {
        phase: InvaderPhaseForecast {
            turn,
            position: position.into(),
            ravage_cards: inv.ravage.clone(),
            build_cards: inv.build.clone(),
            ravages,
            builds,
            explore,
            notes,
        },
        lands,
        stages,
    }
}

/// What a Fast action vs a Slow action in this land can still change.
/// Mechanical statements only (no recommendation): Fast resolves before the
/// Invader Phase, Slow after it, so Slow shapes next turn's cards.
#[allow(clippy::too_many_arguments)]
fn timing_windows(
    l: &PublicLand,
    ravage_now: Option<&RavageLedger>,
    build_now: Option<&str>,
    explore: (u32, u32),
    ravage_next: &Ravage2,
    fed_by: &[String],
    sole_source_for: &[String],
    position: &str,
) -> (Vec<String>, Vec<String>) {
    let mut fast = Vec::new();
    let mut slow = Vec::new();
    if l.is_ocean {
        return (fast, slow);
    }
    let before = position == "before";
    if let Some(r) = ravage_now {
        let result = if r.adds_blight {
            format!("{} damage vs health {}: +1 blight{}", r.damage_after_defend, r.land_health, if r.cascades { ", cascades" } else { "" })
        } else {
            format!("{} damage vs health {}: no blight", r.damage_after_defend, r.land_health)
        };
        if before {
            fast.push(format!("Before this turn's Ravage ({result})."));
            slow.push("After this turn's Ravage: too late for it.".into());
        }
    }
    if let Some(piece) = build_now
        && before {
            fast.push(format!("Before this turn's Build: removing every Invader here stops the +{piece}."));
            slow.push(format!("After the Build: the new {piece} stays and counts next turn."));
        }
    if explore.0 > 0 && !l.coastal && before {
        fast.push(format!(
            "Explore {} in {}: removing its source{} ({}) before the Explore stops it.",
            explore.0,
            explore.1,
            if fed_by.len() > 1 { "s" } else { "" },
            fed_by.join(", ")
        ));
    }
    if explore.0 > 0 && !rules::has_invaders(l) {
        slow.push(format!(
            "If explored ({} in {}), clearing that lone Explorer after the Explore stops next turn's Build here.",
            explore.0, explore.1
        ));
    }
    if *ravage_next == Ravage2::Certain {
        let both = "Fast or Slow this turn both act before next turn's Ravage here.";
        fast.push(both.into());
        slow.push(both.into());
    }
    if !sole_source_for.is_empty() {
        slow.push(format!(
            "Only Explore source for {}: clearing its Towns/Cities leaves {} unexplorable.",
            sole_source_for.join(", "),
            if sole_source_for.len() > 1 { "them" } else { "it" }
        ));
    }
    (fast, slow)
}

fn titles(cards: &[PublicInvaderCard]) -> String {
    cards.iter().map(|c| c.id.as_str()).collect::<Vec<_>>().join(", ")
}
