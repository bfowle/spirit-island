//! Object graph → typed `EngineSnapshot`.
//!
//! `EngineSnapshot` mirrors the engine model and **contains hidden
//! information** (deck orders, earned Fear card names, the blight card's
//! other side). It never leaves this crate except through `projection`.
//!
//! Field names are the engine's backing-field names, read through
//! `nrbf::Node`'s name-tolerant accessors so the three engine builds in
//! Brett's save folder (1.0.9116, 1.0.9602, 1.0.9776) all map.

use std::collections::BTreeMap;
use crate::nrbf::{Graph, Node, Value};
use sha2::{Digest, Sha256};

// Engine enum tables (SpiritIslandEngine.dll 1.0.9776 constants).
pub(crate) const PHASE_TYPES: &[&str] = &["StartOfGame", "Spirit", "FastPower", "Invader", "SlowPower", "TimePasses"];
pub(crate) const STATE_TYPES: &[&str] = &[
    "PlayerSetup", "InvadersStartingAction", "Grow", "GainEnergy", "PlayPayPower", "FastPower",
    "BlightedIsland", "Fear", "Ravage", "Build", "Explore", "AdvanceInvaderCards", "SlowPower",
    "TimePasses", "HighImmigration", "SpiritPhase", "Event",
];
const SPIRIT_PHASE_STATUS: &[&str] = &["Grow", "GainEnergy", "PlayPayPower", "Complete"];
pub(crate) const ISLAND_PIECE_TYPES: &[&str] = &["Dahan", "Explorer", "Town", "City", "Thief"];
pub(crate) const TOKEN_TYPES: &[&str] = &["Beast", "Wilds", "Disease", "Strife", "Scenario", "Badlands", "Special", "Vitality"];
const ENDING_RESULTS: &[&str] = &[
    "WinInvadersDestroyed", "WinTerrorVictory", "WinSacrificeVictory", "WinScenario",
    "LoseSpiritDestroyed", "LoseTimeRunsOut", "LoseTooMuchBlight", "LoseAdversary", "LoseScenario",
    "LoseAbandoned", "LoseSpiritUnique",
];
const TERRAINS: &[&str] = &["Jungle", "Mountain", "Sands", "Wetland", "Ocean"];

pub(crate) fn enum_name(table: &[&str], v: Option<i64>) -> String {
    v.and_then(|i| table.get(i as usize)).map(|s| s.to_string()).unwrap_or_else(|| format!("{v:?}"))
}

#[derive(Debug, Clone)]
pub struct EngineSnapshot {
    /// First 12 hex chars of SHA-256 of the engine seed. The seed itself is
    /// never stored.
    pub game_id: String,
    pub engine_build: String,
    pub turn_index: i64,
    pub state_index: i64,
    pub phase: String,
    pub state: String,
    pub difficulty: Option<i64>,
    pub adversaries: Vec<(String, Option<i64>)>,
    /// Pieces on each adversary's panel location, by kind: what adversary
    /// rules track there (Russia's destroyed Beasts; Russia game save, end:
    /// `TheTsardomOfRussiaAdversary` held Beast1, Beast2).
    pub adversary_pieces: Vec<(String, BTreeMap<String, u32>)>,
    pub scenario: Option<String>,
    pub multiplayer: bool,
    pub boards: Vec<EngineBoard>,
    pub lands: Vec<EngineLand>,
    /// Undirected land adjacency by land id (Ocean lands included).
    pub adjacency: Vec<(String, String)>,
    /// Engine `LandDistanceCache`: distance between every pair of lands
    /// (a < b in land order), as the engine counts Range.
    pub distances: Vec<(String, String, i64)>,
    pub spirits: Vec<EngineSpirit>,
    pub fear_generated: i64,
    pub fear_required: i64,
    pub terror_level_card: Option<String>,
    pub highest_invader_stage: i64,
    pub blight_card: Option<EngineBlightCard>,
    // ---- card zones (hidden order inside) ----
    pub invader_deck: Vec<EngineCard>,
    pub explore_slot_revealed: Vec<EngineCard>,
    pub build_slot: Vec<EngineCard>,
    pub ravage_slot: Vec<EngineCard>,
    pub invader_discard: Vec<EngineCard>,
    /// Extra invader slots (England's High Immigration, …) beyond the base four.
    pub extra_invader_slots: Vec<(String, Vec<EngineCard>)>,
    pub fear_deck: Vec<EngineCard>,
    pub fear_earned: Vec<EngineCard>,
    pub fear_discard: Vec<EngineCard>,
    pub event_deck: Vec<EngineCard>,
    pub event_discard: Vec<EngineCard>,
    /// Cards being resolved now: a revealed Event or Fear Card.
    pub game_play_area: Vec<EngineCard>,
    pub minor_deck: Vec<EngineCard>,
    pub minor_discard: Vec<EngineCard>,
    pub major_deck: Vec<EngineCard>,
    pub major_discard: Vec<EngineCard>,
    pub out_of_game: Vec<EngineCard>,
    pub reveal: Vec<EngineCard>,
    pub allowed_minor: Vec<String>,
    pub allowed_major: Vec<String>,
    pub allowed_fear: Vec<String>,
    pub allowed_event: Vec<String>,
    pub allowed_blight: Vec<String>,
    pub initial_boards: i64,
    pub status_effects: Vec<EngineStatusEffect>,
    pub ending: Option<String>,
    /// Classes or values the map did not recognize (type names only).
    pub unmapped: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EngineBoard {
    pub id: String,
    pub number: i64,
    pub lands: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EngineLand {
    pub id: String,
    pub board: String,
    pub number: i64,
    pub terrain: String,
    pub pieces: Vec<EnginePiece>,
    pub damage_taken: i64,
    pub defend: i64,
    pub health_modifier: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PieceKind {
    Explorer,
    Town,
    City,
    Dahan,
    Thief,
    Blight,
    Presence(String),
    Incarna(String),
    Token(String),
    Other(String),
}

#[derive(Debug, Clone)]
pub struct EnginePiece {
    pub id: String,
    pub kind: PieceKind,
    pub damage: i64,
    pub max_health: Option<i64>,
    pub strife: i64,
}

#[derive(Debug, Clone)]
pub struct EngineCard {
    pub id: String,
    /// Short engine class: InvaderCard, FearCard, TerrorLevelCard, EventCard, PowerCard, …
    pub class: String,
    pub flipped: bool,
    pub known: bool,
    pub override_stage: Option<i64>,
    pub resolved_at_terror_level: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct EngineBlightCard {
    pub id: String,
    pub flipped: bool,
    pub blight_remaining: i64,
}

#[derive(Debug, Clone)]
pub struct EngineSpirit {
    pub id: String,
    pub energy: i64,
    pub card_plays: i64,
    pub phase_status: Option<String>,
    pub elements: [i64; 8],
    pub element_markers: [i64; 8],
    pub hand: Vec<String>,
    pub play_area: Vec<String>,
    pub discard: Vec<String>,
    pub deck: Vec<String>,
    pub forgotten: Vec<String>,
    pub reveal: Vec<String>,
    /// Presence-track node ids, in engine order, with whether a presence still covers each.
    pub track_nodes: Vec<(String, bool)>,
    pub presence_on_island: usize,
    pub presence_destroyed: usize,
    pub innates: Vec<String>,
    pub aspect: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EngineStatusEffect {
    pub class: String,
    pub source: Option<String>,
    /// Non-default members by name: ints, bools, and lists of enum names or
    /// land ids. Interpreted (or flagged) by `si_common::rules`.
    pub params: Vec<(String, Param)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Param {
    Int(i64),
    Bool(bool),
    List(Vec<String>),
}

pub fn map(g: &Graph) -> EngineSnapshot {
    let root = g.root();
    let mut unmapped = Vec::new();

    let game_id = root
        .int("RandomSeed")
        .map(|seed| {
            let h = Sha256::digest(seed.to_le_bytes());
            h.iter().take(6).map(|b| format!("{b:02x}")).collect::<String>()
        })
        .unwrap_or_else(|| "unknown".into());
    let engine_build = g
        .libraries
        .values()
        .find(|l| l.starts_with("SpiritIslandEngine,"))
        .and_then(|l| l.split("Version=").nth(1))
        .map(|v| v.split(',').next().unwrap_or(v).to_string())
        .unwrap_or_default();

    let state = root.child("State");
    let phase = enum_name(PHASE_TYPES, state.and_then(|s| s.child("PhaseType")).and_then(|n| n.boxed_int()));
    let state_name = enum_name(STATE_TYPES, state.and_then(|s| s.child("StateType")).and_then(|n| n.boxed_int()));

    let adversaries = root
        .child("adversaries")
        .map(|l| {
            l.item_nodes()
                .into_iter()
                .map(|a| (a.str("Identifier").unwrap_or("?").to_string(), a.int("Level")))
                .collect()
        })
        .unwrap_or_default();
    let adversary_pieces = root
        .child("adversaries")
        .map(|l| {
            l.item_nodes()
                .into_iter()
                .map(|a| {
                    let mut kinds: BTreeMap<String, u32> = BTreeMap::new();
                    for p in a.child("AdversaryLocation").and_then(|loc| loc.child("pieces")).map(|l| l.item_nodes()).unwrap_or_default() {
                        if let Some(id) = p.str("Identifier") {
                            *kinds.entry(id.trim_end_matches(|c: char| c.is_ascii_digit()).to_string()).or_default() += 1;
                        }
                    }
                    (a.str("Identifier").unwrap_or("?").to_string(), kinds)
                })
                .collect()
        })
        .unwrap_or_default();
    let scenario = root.child("Scenario").and_then(|s| s.str("Identifier")).map(String::from);

    // ---- island
    let mut boards = Vec::new();
    let mut lands = Vec::new();
    let island = root.child("Island");
    let board_nodes = island.and_then(|i| i.child("islandBoards")).map(|l| l.item_nodes()).unwrap_or_default();
    for b in &board_nodes {
        let board_id = b.str("Identifier").unwrap_or("?").to_string();
        let land_nodes = b.child("lands").map(|l| l.item_nodes()).unwrap_or_default();
        let mut ids = Vec::new();
        for l in &land_nodes {
            let land = map_land(*l, &board_id, &mut unmapped);
            ids.push(land.id.clone());
            lands.push(land);
        }
        boards.push(EngineBoard { id: board_id, number: b.int("BoardNumber").unwrap_or(0), lands: ids });
    }
    let adjacency = island.map(|i| map_adjacency(i, &boards, &lands)).unwrap_or_default();
    let distances = island.map(|i| map_distances(i, &boards, &lands)).unwrap_or_default();

    // Presence pieces are listed both on the spirit and in each land's pieces;
    // the land list is the one read.
    let spirits: Vec<EngineSpirit> = root
        .child("spirits")
        .map(|l| l.item_nodes().into_iter().map(map_spirit).collect())
        .unwrap_or_default();

    let cards_of = |name: &str| -> Vec<EngineCard> {
        root.child(name).and_then(|loc| loc.child("cards")).map(|l| l.item_nodes().into_iter().map(map_card).collect()).unwrap_or_default()
    };
    let ids_of = |name: &str| -> Vec<String> {
        root.child(name)
            .map(|a| a.items().iter().filter_map(|v| v.as_ref_id()).filter_map(|id| g.node(id).as_str().map(String::from)).collect())
            .unwrap_or_default()
    };

    let explore_all = cards_of("ExploreSlot");
    let (explore_revealed, invader_deck): (Vec<_>, Vec<_>) = explore_all.into_iter().partition(|c| c.flipped);

    let base_slots = ["ExploreSlot", "BuildSlot", "RavageSlot", "DiscardSlot"];
    let extra_invader_slots = root
        .child("invaderCardSlots")
        .map(|l| {
            l.item_nodes()
                .into_iter()
                .filter_map(|slot| {
                    let id = slot.str("Identifier")?.to_string();
                    if base_slots.contains(&id.as_str()) {
                        return None;
                    }
                    let cards = slot.child("cards").map(|c| c.item_nodes().into_iter().map(map_card).collect()).unwrap_or_default();
                    Some((id, cards))
                })
                .collect()
        })
        .unwrap_or_default();

    let blight_card = root.child("BlightCard").map(|b| EngineBlightCard {
        id: b.str("Identifier").unwrap_or("?").to_string(),
        flipped: b.bool("IsFlipped").unwrap_or(false),
        blight_remaining: b.child("BlightLocation").and_then(|l| l.child("pieces")).map(|p| p.items().len() as i64).unwrap_or(0),
    });

    let status_effects = root
        .child("statusEffects")
        .map(|l| {
            l.item_nodes()
                .into_iter()
                .map(|s| EngineStatusEffect {
                    class: s.class_name().unwrap_or("?").to_string(),
                    source: s.child("Source").and_then(|src| src.str("Identifier")).map(String::from),
                    params: status_params(s),
                })
                .collect()
        })
        .unwrap_or_default();

    EngineSnapshot {
        game_id,
        engine_build,
        turn_index: root.int("TurnIndex").unwrap_or(0),
        state_index: root.int("StateIndex").unwrap_or(0),
        phase,
        state: state_name,
        difficulty: root.int("DifficultyRating"),
        adversaries,
        adversary_pieces,
        scenario,
        multiplayer: root.bool("IsGameMultiplayer").unwrap_or(false),
        boards,
        lands,
        adjacency,
        distances,
        spirits,
        fear_generated: root.int("FearGenerated").unwrap_or(0),
        fear_required: root.int("FearRequiredForFearCard").unwrap_or(0),
        terror_level_card: root.child("TerrorLevelCard").and_then(|t| t.str("Identifier")).map(String::from),
        highest_invader_stage: root.int("HighestInvaderStageReached").unwrap_or(0),
        blight_card,
        invader_deck,
        explore_slot_revealed: explore_revealed,
        build_slot: cards_of("BuildSlot"),
        ravage_slot: cards_of("RavageSlot"),
        invader_discard: cards_of("DiscardSlot"),
        extra_invader_slots,
        fear_deck: cards_of("FearDeck"),
        fear_earned: cards_of("FearEarned"),
        fear_discard: cards_of("FearDiscard"),
        event_deck: cards_of("EventDeck"),
        event_discard: cards_of("EventDiscard"),
        game_play_area: cards_of("PlayArea"),
        minor_deck: cards_of("MinorPowerDeck"),
        minor_discard: cards_of("MinorPowerDiscard"),
        major_deck: cards_of("MajorPowerDeck"),
        major_discard: cards_of("MajorPowerDiscard"),
        out_of_game: cards_of("OutOfGame"),
        reveal: cards_of("RevealLocation"),
        allowed_minor: ids_of("AllowedMinorPowerCardIdentifiers"),
        allowed_major: ids_of("AllowedMajorPowerCardIdentifiers"),
        allowed_fear: ids_of("AllowedFearCardIdentifiers"),
        allowed_event: ids_of("AllowedEventCardIdentifiers"),
        allowed_blight: ids_of("AllowedBlightCardIdentifiers"),
        initial_boards: root.int("InitialNumberOfBoards").unwrap_or(1),
        status_effects,
        ending: root.int("EndingResult").map(|v| enum_name(ENDING_RESULTS, Some(v))),
        unmapped,
    }
}

fn map_land(l: Node, board_id: &str, unmapped: &mut Vec<String>) -> EngineLand {
    let id = l.str("Identifier").unwrap_or("?").to_string();
    let terrain = TERRAINS.iter().find(|t| id.starts_with(*t)).map(|t| t.to_string()).unwrap_or_else(|| {
        unmapped.push(format!("terrain:{id}"));
        "Unknown".into()
    });
    let number = id.trim_start_matches(|c: char| c.is_alphabetic()).parse().unwrap_or(-1);
    let pieces = l.child("pieces").map(|p| p.item_nodes().into_iter().map(|n| map_piece(n, unmapped)).collect()).unwrap_or_default();
    let defend = l.child("defense").map(|d| d.item_nodes().iter().filter_map(|n| n.int("Amount")).sum()).unwrap_or(0);
    let health_modifier =
        l.child("HealthModifiers").map(|d| d.item_nodes().iter().filter_map(|n| n.int("Amount")).sum()).unwrap_or(0);
    EngineLand {
        id,
        board: board_id.to_string(),
        number,
        terrain,
        pieces,
        damage_taken: l.int("DamageTaken").unwrap_or(0),
        defend,
        health_modifier,
    }
}

fn map_piece(p: Node, unmapped: &mut Vec<String>) -> EnginePiece {
    let id = p.str("Identifier").unwrap_or("?").to_string();
    let class = p.class_name().unwrap_or("?");
    let kind = match class {
        "IslandPiece" => match enum_name(ISLAND_PIECE_TYPES, p.child("IslandPieceType").and_then(|n| n.boxed_int()).or_else(|| p.int("IslandPieceType"))).as_str() {
            "Explorer" => PieceKind::Explorer,
            "Town" => PieceKind::Town,
            "City" => PieceKind::City,
            "Dahan" => PieceKind::Dahan,
            "Thief" => PieceKind::Thief,
            other => PieceKind::Other(other.to_string()),
        },
        "BlightPiece" => PieceKind::Blight,
        "Token" => PieceKind::Token(enum_name(TOKEN_TYPES, p.child("TokenType").and_then(|n| n.boxed_int()).or_else(|| p.int("TokenType")))),
        "Presence" => PieceKind::Presence(p.child("OwnerSpirit").and_then(|s| s.str("Identifier")).unwrap_or("?").to_string()),
        "Incarna" => PieceKind::Incarna(p.child("OwnerSpirit").and_then(|s| s.str("Identifier")).unwrap_or("?").to_string()),
        other => {
            unmapped.push(format!("piece:{other}"));
            PieceKind::Other(other.to_string())
        }
    };
    let strife = p
        .child("TokenLocation")
        .and_then(|t| t.child("pieces"))
        .map(|l| l.item_nodes().iter().filter(|t| t.str("Identifier").is_some_and(|s| s.starts_with("Strife"))).count() as i64)
        .unwrap_or(0);
    // Engine `IslandPiece::GetCurrentMaxHealth`: MaxHealth + Σ HealthModifiers, at least 1.
    let modifiers: i64 = p.child("HealthModifiers").map(|l| l.item_nodes().iter().filter_map(|m| m.int("Amount")).sum()).unwrap_or(0);
    let max_health = p.int("MaxHealth").map(|h| (h + modifiers).max(1));
    EnginePiece { id, kind, damage: p.int("DamageTaken").unwrap_or(0), max_health, strife }
}

fn map_card(c: Node) -> EngineCard {
    EngineCard {
        id: c.str("Identifier").unwrap_or("?").to_string(),
        class: c.class_name().unwrap_or("?").to_string(),
        flipped: c.bool("IsFlipped").unwrap_or(false),
        known: c.bool("IsKnown").unwrap_or(false),
        override_stage: c.int("OverrideStage"),
        resolved_at_terror_level: c.int("ResolvedAtTerrorLevel"),
    }
}

fn card_ids(loc: Option<Node>) -> Vec<String> {
    loc.and_then(|l| l.child("cards"))
        .map(|l| l.item_nodes().iter().filter_map(|c| c.str("Identifier").map(String::from)).collect())
        .unwrap_or_default()
}

fn map_spirit(s: Node) -> EngineSpirit {
    let el = |prefix: &str| -> [i64; 8] {
        let names = ["Sun", "Moon", "Fire", "Air", "Water", "Earth", "Plant", "Animal"];
        let mut out = [0; 8];
        for (i, n) in names.iter().enumerate() {
            out[i] = s.int(&format!("{prefix}{n}")).unwrap_or(0);
        }
        out
    };
    let track_nodes = s
        .child("presenceTrack")
        .map(|l| {
            l.item_nodes()
                .into_iter()
                .map(|n| {
                    let covered = n.child("pieces").map(|p| !p.items().is_empty()).unwrap_or(false);
                    (n.str("Identifier").unwrap_or("?").to_string(), covered)
                })
                .collect()
        })
        .unwrap_or_default();
    let presence = s.child("presence").map(|l| l.item_nodes()).unwrap_or_default();
    let mut on_island = 0;
    let mut destroyed = 0;
    for p in &presence {
        let loc = p.child("OwnerLocation");
        match loc.and_then(|l| l.class_name()) {
            Some("Land") => on_island += 1,
            Some("PresenceTrackNode") => {}
            _ => {
                if loc.and_then(|l| l.str("Identifier")).is_some_and(|i| i.ends_with("OutOfGame")) {
                    destroyed += 1;
                }
            }
        }
    }
    EngineSpirit {
        id: s.str("Identifier").unwrap_or("?").to_string(),
        energy: s.int("Energy").unwrap_or(0),
        card_plays: s.int("RemainingCardPlays").unwrap_or(0),
        phase_status: s.int("SpiritPhaseStatus").map(|v| enum_name(SPIRIT_PHASE_STATUS, Some(v))),
        elements: el("Elements"),
        element_markers: el("ElementMarkers"),
        hand: card_ids(s.child("Hand")),
        play_area: card_ids(s.child("PlayArea")),
        discard: card_ids(s.child("Discard")),
        deck: card_ids(s.child("Deck")),
        forgotten: card_ids(s.child("OutOfGame")),
        reveal: card_ids(s.child("RevealLocation")),
        track_nodes,
        presence_on_island: on_island,
        presence_destroyed: destroyed,
        innates: s
            .child("innatePowers")
            .map(|l| l.item_nodes().iter().filter_map(|p| p.str("Identifier").map(String::from)).collect())
            .unwrap_or_default(),
        aspect: s.child("Aspect").and_then(|a| a.str("Identifier")).map(String::from),
    }
}

fn matrix_index(boards: &[EngineBoard], l: &EngineLand, n: i64) -> Option<i64> {
    let b = boards.iter().find(|b| b.id == l.board)?;
    (l.number >= 0).then(|| b.number * MATRIX_STRIDE + l.number).filter(|&i| i < n)
}

/// `LandDistanceCache` uses the adjacency matrix's index space.
fn map_distances(island: Node, boards: &[EngineBoard], lands: &[EngineLand]) -> Vec<(String, String, i64)> {
    let n = island.int("LandMatrixDimensionSize").unwrap_or(0);
    let Some(cache) = island.child("LandDistanceCache") else { return Vec::new() };
    let cells: Vec<i64> = cache.items().iter().map(|v| v.as_i64().unwrap_or(-1)).collect();
    let mut out = Vec::new();
    for (a, la) in lands.iter().enumerate() {
        let Some(i) = matrix_index(boards, la, n) else { continue };
        for lb in &lands[a + 1..] {
            let Some(j) = matrix_index(boards, lb, n) else { continue };
            if let Some(d) = cells.get((i * n + j) as usize).copied().filter(|d| *d >= 0) {
                out.push((la.id.clone(), lb.id.clone(), d));
            }
        }
    }
    out
}

/// Lands per board in the engine's `LandAdjacencyMatrix` index space.
const MATRIX_STRIDE: i64 = 16;

/// The edges between lands of different boards, from the save's adjacency matrix.
pub fn cross_board_links(snap: &EngineSnapshot) -> Vec<(si_model::island::LandId, si_model::island::LandId)> {
    snap.adjacency
        .iter()
        .filter_map(|(a, b)| Some((si_record::log::land(a)?, si_record::log::land(b)?)))
        .filter(|(a, b)| a.board != b.board)
        .collect()
}

/// The engine's `LandAdjacencyMatrix` is a flattened `bool[n, n]` with
/// n = 16 × boards. A land's index is `BoardNumber × 16 + land number`
/// (verified against the board definitions and against cross-board cascade
/// choices in the Test 1 log; n = 16 × boards in all 46 EndGame files).
fn map_adjacency(island: Node, boards: &[EngineBoard], lands: &[EngineLand]) -> Vec<(String, String)> {
    let n = island.int("LandMatrixDimensionSize").unwrap_or(0);
    let Some(matrix) = island.child("LandAdjacencyMatrix") else { return Vec::new() };
    let cells: Vec<bool> = matrix.items().iter().map(|v| matches!(v, Value::Bool(true))).collect();
    let index = |l: &EngineLand| -> Option<i64> {
        let b = boards.iter().find(|b| b.id == l.board)?;
        (l.number >= 0).then(|| b.number * MATRIX_STRIDE + l.number).filter(|&i| i < n)
    };
    let mut out = Vec::new();
    for (a, la) in lands.iter().enumerate() {
        let Some(i) = index(la) else { continue };
        for lb in &lands[a + 1..] {
            let Some(j) = index(lb) else { continue };
            let cell = |x: i64, y: i64| cells.get((x * n + y) as usize).copied().unwrap_or(false);
            if cell(i, j) || cell(j, i) {
                out.push((la.id.clone(), lb.id.clone()));
            }
        }
    }
    out
}

/// Engine bookkeeping members of `StatusEffect` that carry no rule meaning.
const STATUS_BOOKKEEPING: &[&str] = &[
    "Source", "StatusEffectType", "HandlesOwnExpiration", "StartTurnIndex", "DecisionMakers", "RuleActionIndex",
    "PersistAcrossGames", "SuppressDisplayText", "InstanceIdentifier", "AllHealthModifiers",
    "ModifiersCurrentlyBeingRemoved", "PiecesAddingModifiersTo", "TargetsAddingModifiersTo", "Identifier",
    "propertiesInt", "propertiesBool", "propertiesString",
];

fn status_params(s: Node) -> Vec<(String, Param)> {
    let mut out = Vec::new();
    for name in s.member_names() {
        let short = name.rsplit('+').next().unwrap_or(name);
        let short = short.strip_prefix('<').and_then(|x| x.split('>').next()).unwrap_or(short.trim_start_matches('_'));
        if STATUS_BOOKKEEPING.contains(&short) {
            continue;
        }
        let Some(v) = s.raw(short) else { continue };
        let param = match v {
            Value::Bool(true) => Some(Param::Bool(true)),
            Value::Ref(id) => {
                let n = s.graph.node(*id);
                if let Some(i) = n.boxed_int() {
                    Some(Param::Int(i))
                } else if n.class_name() == Some("Land") {
                    n.str("Identifier").map(|l| Param::List(vec![l.to_string()]))
                } else if n.has("_items") {
                    let table = enum_table(n.full_class_name().unwrap_or(""));
                    let items: Vec<String> = n
                        .items()
                        .iter()
                        .filter_map(|v| {
                            let item = s.graph.node(v.as_ref_id()?);
                            match (table, item.boxed_int()) {
                                (Some(t), Some(i)) => t.get(i as usize).map(|x| x.to_string()),
                                _ => item.str("Identifier").map(String::from),
                            }
                        })
                        .collect();
                    (!items.is_empty()).then_some(Param::List(items))
                } else {
                    None
                }
            }
            other => other.as_i64().filter(|i| *i != 0).map(Param::Int),
        };
        if let Some(p) = param {
            out.push((short.to_string(), p));
        }
    }
    out
}

fn enum_table(list_class: &str) -> Option<&'static [&'static str]> {
    const PIECE_TYPES: &[&str] = &["Presence", "Blight", "IslandPiece", "Token", "Incarna"];
    if list_class.contains("Engine.IslandPieceType") {
        Some(ISLAND_PIECE_TYPES)
    } else if list_class.contains("Engine.TokenType") {
        Some(TOKEN_TYPES)
    } else if list_class.contains("Engine.PieceType") {
        Some(PIECE_TYPES)
    } else if list_class.contains("Engine.StateType") {
        Some(STATE_TYPES)
    } else if list_class.contains("Engine.TargetType") {
        Some(si_common::rules::TARGET_TYPES)
    } else {
        None
    }
}
