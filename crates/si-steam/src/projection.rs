//! **The information firewall.** `EngineSnapshot` (hidden information
//! inside) → `PublicSnapshot` (only what a player at the table could know).
//!
//! This is the only module that reads hidden engine fields. Rules it
//! enforces (spec Section 5):
//! - Face-down decks become counts plus the stage printed on each invader
//!   card back. Deck order and identity never cross.
//! - Earned Fear cards are a count until they resolve into the discard.
//! - The blight card shows its healthy side (identical on 24 of 25 cards)
//!   until it flips.
//! - Draw pools are *allowed pool − cards seen outside the deck*, never the
//!   deck's own list.

use crate::engine_defs::EngineDefs;
use crate::engine_map::{EngineCard, EngineSnapshot, Param, PieceKind};
use crate::log_parse::{RawDecision, TimedDecision};
use serde_json::Value as Json;
use si_common::public::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};

const ELEMENTS: [&str; 8] = ["Sun", "Moon", "Fire", "Air", "Water", "Earth", "Plant", "Animal"];

pub fn project(s: &EngineSnapshot, defs: &EngineDefs) -> PublicSnapshot {
    let mut warnings: Vec<String> = s.unmapped.iter().map(|u| format!("unmapped {u}")).collect();
    if s.multiplayer {
        warnings.push("multiplayer game: stats assume a single shared table".into());
    }

    let ocean: HashSet<&str> = s.lands.iter().filter(|l| l.terrain == "Ocean").map(|l| l.id.as_str()).collect();
    let coastal: HashSet<&str> = s
        .adjacency
        .iter()
        .flat_map(|(a, b)| {
            [(a.as_str(), b.as_str()), (b.as_str(), a.as_str())]
                .into_iter()
                .filter(|(x, y)| ocean.contains(y) && !ocean.contains(x))
                .map(|(x, _)| x)
        })
        .collect();

    let lands: Vec<PublicLand> = s
        .lands
        .iter()
        .map(|l| {
            let mut out = PublicLand {
                id: l.id.clone(),
                board: l.board.clone(),
                number: l.number,
                terrain: l.terrain.clone(),
                is_ocean: l.terrain == "Ocean",
                coastal: coastal.contains(l.id.as_str()),
                damage_taken: l.damage_taken,
                defend: l.defend,
                health_modifier: l.health_modifier,
                ..Default::default()
            };
            for p in &l.pieces {
                let piece = |kind: &str| PublicPiece {
                    kind: kind.into(),
                    max_health: p.max_health.unwrap_or(0),
                    damage: p.damage,
                    strife: p.strife,
                };
                match &p.kind {
                    PieceKind::Explorer => {
                        out.explorers += 1;
                        out.pieces.push(piece("Explorer"));
                    }
                    PieceKind::Town => {
                        out.towns += 1;
                        out.pieces.push(piece("Town"));
                    }
                    PieceKind::City => {
                        out.cities += 1;
                        out.pieces.push(piece("City"));
                    }
                    PieceKind::Dahan => {
                        out.dahan += 1;
                        out.pieces.push(piece("Dahan"));
                    }
                    PieceKind::Thief => out.pieces.push(piece("Thief")),
                    PieceKind::Blight => out.blight += 1,
                    PieceKind::Presence(sp) | PieceKind::Incarna(sp) => *out.presence.entry(sp.clone()).or_default() += 1,
                    PieceKind::Token(t) => *out.tokens.entry(t.clone()).or_default() += 1,
                    PieceKind::Other(o) => warnings.push(format!("unmapped piece {o} in {}", l.id)),
                }
            }
            out
        })
        .collect();

    // ---- spirits: co-op open information
    let card = |id: &str| power_card(id, defs);
    let spirits: Vec<PublicSpirit> = s
        .spirits
        .iter()
        .map(|sp| PublicSpirit {
            id: sp.id.clone(),
            title: defs.title_of(&sp.id).unwrap_or_else(|| sp.id.clone()),
            aspect: sp.aspect.clone(),
            energy: sp.energy,
            card_plays: sp.card_plays,
            phase_status: sp.phase_status.clone(),
            elements: element_map(&sp.elements),
            element_markers: element_map(&sp.element_markers),
            hand: sp.hand.iter().map(|c| card(c)).collect(),
            play_area: sp.play_area.iter().map(|c| card(c)).collect(),
            discard: sp.discard.iter().map(|c| card(c)).collect(),
            forgotten: sp.forgotten.iter().map(|c| card(c)).collect(),
            revealed: sp.reveal.iter().map(|c| card(c)).collect(),
            track: sp.track_nodes.iter().map(|(id, covered)| track_node(&sp.id, id, !covered, defs)).collect(),
            presence_on_island: sp.presence_on_island as u32,
            presence_destroyed: sp.presence_destroyed as u32,
            innates: sp.innates.iter().map(|i| defs.title_of(i).unwrap_or_else(|| i.clone())).collect(),
            innate_powers: innate_powers(&sp.id, defs),
            special_rules: special_rules(&sp.id, defs),
        })
        .collect();

    // ---- invaders: card backs are public, faces only once flipped
    let face_up = |cards: &[EngineCard]| -> Vec<PublicInvaderCard> {
        cards.iter().filter(|c| c.class == "InvaderCard").map(|c| invader_card(&c.id, defs)).collect()
    };
    let mut upcoming_stages = Vec::new();
    for c in s.invader_deck.iter().rev() {
        match (c.class.as_str(), defs.invader_stage(&c.id)) {
            ("InvaderCard", Some(stage)) => upcoming_stages.push(stage),
            ("InvaderCard", None) => warnings.push("invader card with unknown stage in deck".into()),
            (other, _) => warnings.push(format!("{other} in invader deck (not counted as a stage)")),
        }
    }
    let invaders = PublicInvaderDeck {
        upcoming_stages,
        explore: face_up(&s.explore_slot_revealed),
        build: face_up(&s.build_slot),
        ravage: face_up(&s.ravage_slot),
        discard: face_up(&s.invader_discard),
        extra_slots: s.extra_invader_slots.iter().map(|(k, v)| (k.clone(), face_up(v))).collect(),
        highest_stage_reached: s.highest_invader_stage,
        pool: invader_pool(s, defs),
    };

    // ---- fear
    let fear = PublicFear {
        generated: s.fear_generated,
        required_per_card: s.fear_required,
        terror_level: terror_level(s.terror_level_card.as_deref(), &fear_deck_by_level(&s.fear_deck)),
        earned_count: s.fear_earned.iter().filter(|c| c.class == "FearCard").count() as u32,
        deck_by_level: fear_deck_by_level(&s.fear_deck),
        discard: s
            .fear_discard
            .iter()
            .filter(|c| c.class == "FearCard")
            .map(|c| fear_card(c, defs))
            .collect(),
        resolving: s.game_play_area.iter().find(|c| c.class == "FearCard").map(|c| fear_card(c, defs)),
        removed_unseen: s.out_of_game.iter().filter(|c| c.class == "FearCard").count() as u32,
    };

    let named = |id: &str| NamedCard { id: id.to_string(), title: defs.title_of(id).unwrap_or_else(|| id.to_string()) };
    let shown = s
        .game_play_area
        .iter()
        .find(|c| c.class == "EventCard")
        .map(|c| event_card(&c.id, true, defs))
        .or_else(|| s.event_discard.last().map(|c| event_card(&c.id, false, defs)));
    let events = PublicEvents {
        deck_count: s.event_deck.len() as u32,
        shown,
        discard: s.event_discard.iter().map(|c| named(&c.id)).collect(),
    };

    // ---- powers: unseen = allowed − seen outside the deck
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for c in s.minor_discard.iter().chain(&s.major_discard).chain(&s.out_of_game).chain(&s.reveal) {
        seen.insert(&c.id);
    }
    for sp in &s.spirits {
        for id in sp.hand.iter().chain(&sp.play_area).chain(&sp.discard).chain(&sp.forgotten).chain(&sp.reveal).chain(&sp.deck) {
            seen.insert(id);
        }
    }
    let unseen = |allowed: &[String]| -> Vec<PublicPowerCard> {
        allowed.iter().filter(|id| !seen.contains(id.as_str())).map(|id| power_card(id, defs)).collect()
    };
    let powers = PublicPowers {
        minor_deck_count: s.minor_deck.len() as u32,
        major_deck_count: s.major_deck.len() as u32,
        minor_discard: s.minor_discard.iter().map(|c| c.id.clone()).collect(),
        major_discard: s.major_discard.iter().map(|c| c.id.clone()).collect(),
        minor_unseen: unseen(&s.allowed_minor),
        major_unseen: unseen(&s.allowed_major),
    };
    if powers.minor_unseen.len() != s.minor_deck.len() {
        warnings.push(format!(
            "Minor pool mismatch: {} unseen vs {} in deck; Minor draw odds approximate",
            powers.minor_unseen.len(),
            s.minor_deck.len()
        ));
    }
    if powers.major_unseen.len() != s.major_deck.len() {
        warnings.push(format!(
            "Major pool mismatch: {} unseen vs {} in deck; Major draw odds approximate",
            powers.major_unseen.len(),
            s.major_deck.len()
        ));
    }

    // ---- blight card: healthy side until flipped
    let blight = match &s.blight_card {
        Some(b) => {
            let def = defs.blight_cards.get(&b.id);
            let lines = |key: &str| -> Vec<String> {
                def.and_then(|d| d.get(key)).and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default()
            };
            PublicBlight {
                flipped: b.flipped,
                remaining: b.blight_remaining,
                on_island: lands.iter().map(|l| l.blight).sum(),
                healthy_side: lines("healthyIslandBody"),
                blighted_card: b.flipped.then(|| named(&b.id)),
                blighted_text: if b.flipped { lines("flippedBody") } else { Vec::new() },
                unseen_flipped: if b.flipped { Vec::new() } else { unseen_flipped(&b.id, s, defs) },
                unseen_still_healthy: if b.flipped { 0 } else { unseen_still_healthy(&b.id, s, defs) },
                current_still_healthy: b.flipped && is_still_healthy(&b.id, defs),
                pool: blight_pool(b, s, defs),
            }
        }
        None => PublicBlight { on_island: lands.iter().map(|l| l.blight).sum(), ..Default::default() },
    };

    // ---- status effects: a source is named only when it is a public card
    let mut public_ids: HashSet<&str> = HashSet::new();
    for c in s.fear_discard.iter().chain(&s.event_discard).chain(&s.minor_discard).chain(&s.major_discard) {
        public_ids.insert(&c.id);
    }
    for sp in &s.spirits {
        public_ids.insert(&sp.id);
        for id in sp.hand.iter().chain(&sp.play_area).chain(&sp.discard).chain(&sp.forgotten).chain(&sp.innates) {
            public_ids.insert(id);
        }
    }
    for (a, _) in &s.adversaries {
        public_ids.insert(a);
    }
    // What kind of thing made the effect; Fear, Event and Power effects last
    // only this turn (the forecast drops them once the Invader Phase is over).
    let source_kind = |id: &str| -> &'static str {
        if defs.event_cards.contains_key(id) {
            "event"
        } else if defs.fear_cards.contains_key(id) {
            "fear"
        } else if defs.power_cards.contains_key(id) {
            "power"
        } else if defs.blight_cards.contains_key(id) {
            "blight"
        } else if defs.adversaries.contains_key(id) {
            "adversary"
        } else if defs.scenarios.contains_key(id) {
            "scenario"
        } else if defs.spirits.contains_key(id) {
            "spirit"
        } else {
            "other"
        }
    };
    let status_effects = s
        .status_effects
        .iter()
        .map(|e| {
            // Parameters cross only when the source is a public card or rule.
            // An Event's or Fear Card's effect exists only once that card is
            // face up (revealed or resolving), so it is public even before
            // the card reaches its discard (playtest R5).
            let public = e.source.as_deref().is_some_and(|id| {
                public_ids.contains(id) || matches!(source_kind(id), "event" | "fear")
            });
            PublicStatusEffect {
                kind: e.class.trim_end_matches("StatusEffect").to_string(),
                source_kind: e.source.as_deref().map(|id| source_kind(id).to_string()).unwrap_or_default(),
                source: e.source.as_deref().filter(|_| public).map(|id| defs.title_of(id).unwrap_or_else(|| id.to_string())),
                params: if public {
                    e.params
                        .iter()
                        .map(|(k, v)| {
                            let v = match v {
                                Param::Int(i) => StatusParam::Int(*i),
                                Param::Bool(b) => StatusParam::Bool(*b),
                                Param::List(l) => StatusParam::List(l.clone()),
                            };
                            (k.clone(), v)
                        })
                        .collect()
                } else {
                    BTreeMap::new()
                },
            }
        })
        .collect();

    PublicSnapshot {
        game_id: s.game_id.clone(),
        engine_build: s.engine_build.clone(),
        turn: (s.turn_index + 1).max(1) as u32,
        phase: s.phase.clone(),
        step: s.state.clone(),
        difficulty: s.difficulty,
        adversaries: s
            .adversaries
            .iter()
            .map(|(id, level)| AdversaryRef {
                id: id.clone(),
                title: defs.title_of(id).unwrap_or_else(|| id.clone()),
                level: level.unwrap_or(0),
                panel: s.adversary_pieces.iter().find(|(a, _)| a == id).map(|(_, k)| k.clone()).unwrap_or_default(),
            })
            .collect(),
        scenario: s.scenario.clone(),
        multiplayer: s.multiplayer,
        boards: s.boards.iter().map(|b| PublicBoard { id: b.id.clone(), lands: b.lands.clone() }).collect(),
        lands,
        adjacency: s.adjacency.clone(),
        spirits,
        invaders,
        fear,
        events,
        powers,
        blight,
        status_effects,
        ending: s.ending.clone(),
        warnings,
    }
}

fn element_map(counts: &[i64; 8]) -> BTreeMap<String, i64> {
    ELEMENTS.iter().zip(counts).filter(|(_, n)| **n != 0).map(|(e, n)| (e.to_string(), *n)).collect()
}

/// The Terror Level card's level. Otherwise (the England game's end-of-game
/// save, Fear deck empty) the deck says: the first level with cards left, or
/// III when none are.
fn terror_level(card: Option<&str>, by_level: &[u32; 3]) -> u8 {
    match card {
        Some("TerrorLevelIII") => 3,
        Some("TerrorLevelII") => 2,
        Some("TerrorLevelI") => 1,
        _ => by_level.iter().position(|n| *n > 0).map_or(3, |i| i as u8 + 1),
    }
}

/// Fear cards left at each Terror Level. The deck lists bottom → top:
/// Terror Victory, the TL III cards, the TL III divider, the TL II cards, the
/// TL II divider, the TL I cards. Divider positions are public setup facts.
fn fear_deck_by_level(deck: &[EngineCard]) -> [u32; 3] {
    let mut out = [0u32; 3];
    let mut level = 3usize;
    for c in deck {
        match (c.class.as_str(), c.id.as_str()) {
            ("TerrorLevelCard", "TerrorLevelIII") => level = 2,
            ("TerrorLevelCard", "TerrorLevelII") => level = 1,
            ("FearCard", _) => out[level - 1] += 1,
            _ => {}
        }
    }
    out
}

fn invader_card(id: &str, defs: &EngineDefs) -> PublicInvaderCard {
    PublicInvaderCard {
        id: id.to_string(),
        stage: defs.invader_stage(id).unwrap_or(0),
        escalation: defs.invader_cards.get(id).and_then(|c| c.get("escalation")).and_then(Json::as_bool).unwrap_or(false),
        terrains: defs.invader_land_types(id),
    }
}

pub fn power_card(id: &str, defs: &EngineDefs) -> PublicPowerCard {
    let Some(d) = defs.power_cards.get(id) else {
        // Reminder cards (added by rules mid-game) sit in play areas too.
        let r = defs.reminder_cards.get(id);
        return PublicPowerCard {
            id: id.to_string(),
            title: r.and_then(|r| r.get("title")).and_then(Json::as_str).unwrap_or(id).to_string(),
            kind: if r.is_some() { "Reminder" } else { "" }.into(),
            body: r.map(|r| str_list(r, "body")).unwrap_or_default(),
            ..Default::default()
        };
    };
    let s = |k: &str| d.get(k).and_then(Json::as_str).map(String::from);
    PublicPowerCard {
        id: id.to_string(),
        title: s("title").unwrap_or_else(|| id.to_string()),
        cost: d.get("energyCost").and_then(Json::as_i64).unwrap_or(0),
        speed: if d.get("speed").and_then(Json::as_bool).unwrap_or(false) { "Fast" } else { "Slow" }.into(),
        kind: s("powerType").unwrap_or_default(),
        elements: d.get("elementTypes").and_then(Json::as_array).map(|a| a.iter().filter_map(|e| e.as_str().map(String::from)).collect()).unwrap_or_default(),
        range: d.get("rangeDistance").and_then(Json::as_i64),
        from_sacred_site: d.get("rangeSacredSite").and_then(Json::as_bool).unwrap_or(false),
        target: s("targetType"),
        body: str_list(d, "body"),
    }
}

fn track_node(spirit: &str, id: &str, revealed: bool, defs: &EngineDefs) -> TrackNode {
    let def = defs
        .spirits
        .get(spirit)
        .and_then(|sp| sp.get("presenceTrackNodes"))
        .and_then(Json::as_array)
        .and_then(|nodes| nodes.iter().find(|n| n.get("identifier").and_then(Json::as_str) == Some(id)));
    TrackNode {
        id: id.to_string(),
        revealed,
        kind: def.and_then(|d| d.get("trackNodeType")).and_then(Json::as_str).unwrap_or("").to_string(),
        value: def.and_then(|d| d.get("number")).and_then(Json::as_i64),
        elements: def.map(|d| str_list(d, "elementTypes")).unwrap_or_default(),
    }
}

fn str_list(d: &Json, key: &str) -> Vec<String> {
    d.get(key).and_then(Json::as_array).map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default()
}

fn innate_powers(spirit: &str, defs: &EngineDefs) -> Vec<PublicInnate> {
    let Some(list) = defs.spirits.get(spirit).and_then(|s| s.get("innatePowers")).and_then(Json::as_array) else { return Vec::new() };
    list.iter()
        .map(|p| PublicInnate {
            id: p.get("identifier").and_then(Json::as_str).unwrap_or("").to_string(),
            title: p.get("title").and_then(Json::as_str).unwrap_or("").to_string(),
            speed: if p.get("speed").and_then(Json::as_bool).unwrap_or(false) { "Fast" } else { "Slow" }.into(),
            range: p.get("rangeDistance").and_then(Json::as_i64),
            from_sacred_site: p.get("rangeSacredSite").and_then(Json::as_bool).unwrap_or(false),
            target: p.get("targetType").and_then(Json::as_str).map(String::from),
            levels: p
                .get("elementOptions")
                .and_then(Json::as_array)
                .into_iter()
                .flatten()
                .map(|lvl| InnateLevel {
                    elements: lvl
                        .as_object()
                        .into_iter()
                        .flatten()
                        .filter_map(|(k, v)| Some((k.strip_prefix("cost")?.to_string(), v.as_i64()?)))
                        .filter(|(k, _)| ELEMENTS.contains(&k.as_str()))
                        .collect(),
                    body: lvl.get("body").and_then(Json::as_str).unwrap_or("").to_string(),
                })
                .collect(),
        })
        .collect()
}

fn special_rules(spirit: &str, defs: &EngineDefs) -> Vec<NamedText> {
    let Some(list) = defs.spirits.get(spirit).and_then(|s| s.get("specialRules")).and_then(Json::as_array) else { return Vec::new() };
    list.iter()
        .map(|r| NamedText {
            id: r.get("identifier").and_then(Json::as_str).unwrap_or("").to_string(),
            title: r.get("title").and_then(Json::as_str).unwrap_or("").to_string(),
            body: str_list(r, "body"),
        })
        .collect()
}

/// The invader cards this game's rules can draw: the card list, minus Salt
/// Deposits unless an adversary in play uses it (Habsburg Mining).
fn invader_pool(s: &EngineSnapshot, defs: &EngineDefs) -> Vec<PublicInvaderCard> {
    let salt = s.adversaries.iter().any(|(id, _)| id.contains("Mining"));
    let mut ids: Vec<&String> = defs
        .invader_cards
        .keys()
        .filter(|id| salt || !defs.invader_land_types(id).iter().any(|t| t == "SaltDeposits"))
        .collect();
    ids.sort();
    ids.into_iter().map(|id| invader_card(id, defs)).collect()
}

/// The blight cards that share this card's healthy side (the player cannot
/// tell them apart before the flip). Depends only on the public healthy side
/// and the allowed pool, never on which card is face down.
fn flip_pool<'a>(card: &str, s: &'a EngineSnapshot, defs: &'a EngineDefs) -> Vec<&'a String> {
    let key = |id: &str| {
        defs.blight_cards.get(id).map(|d| (d.get("healthyIslandBody").cloned(), d.get("healthyIslandNumberPerStartingBoard").cloned()))
    };
    let mine = key(card);
    let mut pool: Vec<&String> = if s.allowed_blight.is_empty() {
        defs.blight_cards.keys().filter(|id| key(id) == mine).collect()
    } else {
        s.allowed_blight.iter().filter(|id| key(id) == mine).collect()
    };
    pool.sort();
    pool
}

/// How many cards in the flip pool are Still-Healthy: emptying them draws a
/// new Blight Card instead of losing (JE rulebook; open-questions Q11).
fn unseen_still_healthy(card: &str, s: &EngineSnapshot, defs: &EngineDefs) -> u32 {
    flip_pool(card, s, defs)
        .into_iter()
        .filter(|id| defs.blight_cards.get(*id).and_then(|d| d.get("blightCardType")).and_then(Json::as_str) == Some("StillHealthyIsland"))
        .count() as u32
}

fn is_still_healthy(id: &str, defs: &EngineDefs) -> bool {
    defs.blight_cards.get(id).and_then(|d| d.get("blightCardType")).and_then(Json::as_str) == Some("StillHealthyIsland")
}

/// Every Blight Card a later draw could bring: allowed, minus cards already
/// used (face up out of the game), minus the current card once face up.
/// Before the flip the current card stays in (it is hidden among the cards
/// sharing its healthy side), so this never depends on which card it is.
fn blight_pool(current: &crate::engine_map::EngineBlightCard, s: &EngineSnapshot, defs: &EngineDefs) -> Vec<BlightOption> {
    let used: HashSet<&str> =
        s.out_of_game.iter().filter(|c| c.class == "BlightCard" && c.flipped).map(|c| c.id.as_str()).collect();
    let healthy_key = |id: &str| {
        defs.blight_cards.get(id).map(|d| (d.get("healthyIslandBody").cloned(), d.get("healthyIslandNumberPerStartingBoard").cloned()))
    };
    let mine = healthy_key(&current.id);
    let mut ids: Vec<&String> = s
        .allowed_blight
        .iter()
        .filter(|id| !used.contains(id.as_str()) && !(current.flipped && **id == current.id))
        .collect();
    ids.sort();
    ids.into_iter()
        .map(|id| BlightOption {
            flipped_total: defs
                .blight_cards
                .get(id)
                .and_then(|d| d.get("flippedNumberPerStartingBoard"))
                .and_then(Json::as_i64)
                .map(|n| n * s.initial_boards),
            still_healthy: is_still_healthy(id, defs),
            same_healthy_side: !current.flipped && healthy_key(id) == mine,
        })
        .collect()
}

/// Flipped-side blight for each card in the flip pool.
fn unseen_flipped(card: &str, s: &EngineSnapshot, defs: &EngineDefs) -> Vec<Option<i64>> {
    flip_pool(card, s, defs)
        .into_iter()
        .map(|id| {
            defs.blight_cards
                .get(id)
                .and_then(|d| d.get("flippedNumberPerStartingBoard"))
                .and_then(Json::as_i64)
                .map(|n| n * s.initial_boards)
        })
        .collect()
}

/// Card-choice reasons whose options are public (revealed or the player's
/// own cards). Other reasons keep the reason but drop card names.
const PUBLIC_CARD_REASONS: &[&str] =
    &["GainPower", "Forget", "ForgetPower", "Discard", "Reclaim", "PlayPower", "ReturnToHand", "PlayUseAndForgetPower"];

/// "ShadowsFlickerLikeFlameCardPlays2" → "Card Plays track (2)"; a spirit's
/// "…'s Forgotten Powers" setup source → "starting presence".
fn presence_place(s: &str, spirits: &[String]) -> String {
    if s.contains("ForgottenPowers") {
        return "starting presence".into();
    }
    for sp in spirits {
        if let Some(node) = s.strip_prefix(sp.as_str()) {
            let digits: String = node.chars().filter(|c| c.is_ascii_digit()).collect();
            let track: String = node.chars().filter(|c| !c.is_ascii_digit()).collect();
            let track = if track == "CardPlays" { "Card Plays".to_string() } else { track };
            return format!("{track} track ({digits})");
        }
    }
    s.to_string()
}

/// Decisions → public form. Every kind is the player's own visible choice;
/// card names cross only for whitelisted reasons.
pub fn decisions(raw: &[TimedDecision], defs: &EngineDefs) -> Vec<PublicDecision> {
    let title = |id: &str| defs.title_of(id).unwrap_or_else(|| id.to_string());
    let spirits: Vec<String> = defs.spirits.keys().cloned().collect();
    let mut out: Vec<PublicDecision> = Vec::new();
    let all = raw.iter()
        .filter_map(|d| {
            let (kind, text, land, card) = match &d.decision {
                RawDecision::GrowthGroup { choice } => ("growth", choice.replace(" Growth Group ", ": growth option "), None, None),
                RawDecision::GrowthOption { option } => ("growth", format!("Growth: {option}"), None, None),
                RawDecision::CardChoice { reason, selected, choices } if PUBLIC_CARD_REASONS.contains(&reason.as_str()) => {
                    let others: Vec<String> = choices.iter().map(|c| title(c)).filter(|t| t != selected).collect();
                    // The selected card is what the choice does to it: for a
                    // Forget it is the card Forgotten, not one kept (U17).
                    let (kind, text) = match reason.as_str() {
                        "GainPower" if others.is_empty() => ("gain-power", format!("Gained {selected}")),
                        "GainPower" => ("gain-power", format!("Gained {selected} (over {})", others.join(", "))),
                        "Forget" | "ForgetPower" => ("forget", format!("Forgot {selected}")),
                        "Discard" => ("card-choice", format!("Discarded {selected}")),
                        "Reclaim" => ("card-choice", format!("Reclaimed {selected}")),
                        "ReturnToHand" => ("card-choice", format!("Returned {selected} to hand")),
                        _ => ("card-choice", format!("Chose {selected} ({reason})")),
                    };
                    (kind, text, None, Some(selected.clone()))
                }
                RawDecision::EventRevealed { card } => ("event", format!("Event revealed: {}", title(card)), None, Some(title(card))),
                RawDecision::PowersRevealed { deck, titles } => {
                    ("draft", format!("Drew {} ({deck}): {}", titles.len(), titles.join(", ")), None, None)
                }
                RawDecision::CardChoice { reason, .. } => ("card-choice", format!("Chose a card ({reason})"), None, None),
                RawDecision::PlayCard { card } => {
                    let c = power_card(card, defs);
                    ("play", format!("Played {} ({}, {} Energy)", c.title, c.speed, c.cost), None, Some(c.title))
                }
                RawDecision::UsePower { power, target } => {
                    let text = match target {
                        Some(t) => format!("Used {} on {t}", title(power)),
                        None => format!("Used {}", title(power)),
                    };
                    ("power", text, target.clone(), Some(title(power)))
                }
                RawDecision::Effect { option, source, land } => {
                    let at = land.as_ref().map(|l| format!(" in {l}")).unwrap_or_default();
                    ("effect", format!("{source}: chose \"{option}\"{at}"), land.clone(), Some(source.clone()))
                }
                RawDecision::Location { reason, land, .. } if reason == "PowerDeckToDrawFrom" => {
                    ("gain-power", format!("Drew from the {}", land.replace("PowerDeck", " Power deck")), None, None)
                }
                // The engine asks which matching land Ravages first; not a decision about the land.
                RawDecision::Location { reason, .. } if reason == "RavageLand" => return None,
                RawDecision::Location { reason, land, source } => {
                    let from = if source.is_empty() { String::new() } else { format!(" ({source})") };
                    ("land-choice", format!("{reason}: {land}{from}"), Some(land.clone()), None)
                }
                RawDecision::Presence { from, to } => {
                    let text = if to.contains("OutOfGame") {
                        format!("Presence destroyed in {from}")
                    } else {
                        format!("Presence {} → {}", presence_place(from, &spirits), presence_place(to, &spirits))
                    };
                    ("presence", text, Some(to.clone()).filter(|t| !t.contains("OutOfGame")), None)
                }
                RawDecision::Damage { sources, targets, amount } => ("damage", format!("{amount} damage from {sources} assigned among {targets}"), None, None),
                RawDecision::Forget { card } => ("forget", format!("Forgot {}", title(card)), None, Some(title(card))),
            };
            let event = match &d.decision {
                RawDecision::EventRevealed { card } => Some(event_card(card, true, defs)),
                _ => None,
            };
            Some(PublicDecision { turn: d.turn, step: d.step.clone(), kind: kind.into(), text, land, card, ms: d.ms, event })
        });
    // An identical decision repeated in the same step is a replay of one
    // the parser could not see undone; keep one.
    for d in all {
        if out.last().is_some_and(|p| p.turn == d.turn && p.step == d.step && p.text == d.text) {
            continue;
        }
        out.push(d);
    }
    out
}

fn fear_card(c: &EngineCard, defs: &EngineDefs) -> PublicFearCard {
    let body = defs
        .fear_cards
        .get(&c.id)
        .and_then(|d| d.get("body"))
        .and_then(|b| b.as_array())
        .map(|b| b.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default();
    PublicFearCard { id: c.id.clone(), title: defs.title_of(&c.id).unwrap_or_else(|| c.id.clone()), resolved_at_terror_level: c.resolved_at_terror_level, body }
}

fn event_card(id: &str, in_play: bool, defs: &EngineDefs) -> PublicEventCard {
    let strs = |v: Option<&serde_json::Value>| -> Vec<String> {
        v.and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default()
    };
    let sections = defs
        .event_cards
        .get(id)
        .and_then(|d| d.get("eventSections"))
        .and_then(|x| x.as_array())
        .map(|a| {
            a.iter()
                .map(|sec| PublicEventSection {
                    title: sec.get("eventSectionTitle").and_then(|t| t.as_str()).unwrap_or("").to_string(),
                    types: strs(sec.get("eventSectionTypes")),
                    body: strs(sec.get("eventSectionBody")),
                })
                .collect()
        })
        .unwrap_or_default();
    PublicEventCard { id: id.to_string(), title: defs.title_of(id).unwrap_or_else(|| id.to_string()), sections, in_play }
}
