//! The engine's own journal, read from a save. Every decision save carries the whole
//! game's typed journal (`Game.Journal._journalEntries`): the material the app writes
//! `log.txt` from. Read here, each entry is typed and the model objects it names are
//! resolved to the identifiers the log uses (lands `SandsA4`, pieces `Town2`, spirits
//! `Thunderspeaker`, cards `Sands1`, track slots `ThunderspeakerCardPlays5`), so a
//! save alone rebuilds the record: no `log.txt`, no undo guessing, no journal copies.
//! `parsed` turns the entries into the record's events and markers for the replay walk.
//!
//! Through the firewall: only what the entries name. A face-down card is never looked
//! into; the journal names a card once the engine has flipped or moved it in the open.

use std::collections::BTreeSet;

use si_model::event::{Event, InvaderStep, PieceMove};
use si_model::game::Phase;
use si_model::island::{PieceKind, TokenKind};
use si_record::log::{card_pile_move, land, Marker, Parsed};
use si_record::record::Choice;

use crate::engine_map::{enum_name, PHASE_TYPES, STATE_TYPES};
use crate::nrbf::{Graph, Node, Value};

const ELEMENTS: &[&str] = &["Sun", "Moon", "Fire", "Air", "Water", "Earth", "Plant", "Animal"];
const MOVE_REASONS: &[&str] = &["None", "Destroy", "Replace", "Remove", "Push", "Gather"];
const RULE_ACTIONS: &[&str] = &["SpiritAction", "InvaderAction", "FearAction", "BlightAction", "ScenarioAction", "EventAction", "AdversaryAction", "TokenAction"];

/// One journal entry with its place in the game.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub index: usize,
    pub turn_index: i64,
    pub state_index: i64,
    /// The engine's phase and state names at the entry (`Invader`, `Ravage`).
    pub phase: String,
    pub state: String,
    pub undo_related: bool,
    pub kind: Kind,
}

/// What an entry records, with every model object as its identifier.
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    ChangeState { from: Option<String>, to: String },
    /// The player's answer to an activation: what was picked, from the activation's own fields.
    UseActivation { activation: String, instance: String, selection: Option<i64>, target: Option<String>, power: Option<String>, location: Option<String>, pieces: Vec<String>, board: Option<String>, choices: Vec<String>, completed: bool, spend_energy: bool },
    AddPieces { adds: Vec<(String, String)>, source: Option<String> },
    MovePieces { moves: Vec<(String, String, String)>, reason: String, source: Option<String> },
    DestroyPieces { pieces: Vec<(String, Option<String>)>, source: Option<String> },
    MovePresence { spirit: String, moves: Vec<(String, String, String)>, reason: String },
    GainEnergy { spirit: String, amount: i64 },
    PayEnergy { spirit: String, amount: i64 },
    GainElements { spirit: String, elements: Vec<String>, source: Option<String> },
    LoseElements { spirit: String, elements: Vec<String> },
    PlayPowerCard { spirit: String, card: String, unplayed: bool },
    StartPower { spirit: String, power: String, target: Option<String> },
    UsePower { spirit: String, power: String, target: Option<String> },
    ModifyPowers { powers: Vec<String>, reset: bool },
    UseGrowthOption { spirit: String, group: Option<i64>, option_index: Option<i64>, always: bool, undone: bool, option: Option<String> },
    FinishGrowthOption { spirit: String, group: Option<i64>, option_index: Option<i64>, option: Option<String> },
    AdjustFear { amount: i64, land: Option<String>, before: i64 },
    FlipCard { class: String, card: String, flipped: bool },
    FlipCardStack { location: String },
    InvaderState { card: String, lands: Vec<String>, matching: Vec<String> },
    Explore { land: String, card: Option<String>, success: bool },
    BuildLand { land: String, card: Option<String>, cancelled: bool },
    RavageLand { land: String, card: Option<String>, cancelled: bool },
    RavageComplete { land: String, card: Option<String>, blighted: bool, lands_blighted: Vec<String> },
    DealDamage { sources: Vec<String>, targets: Vec<String>, lands: Vec<String>, distribution: Vec<(String, i64)>, set_instead: bool, pretend: bool },
    /// One entry per Blight piece of an action: `added` counts the action so far, `previously` what the earlier entries of it already added.
    AddBlight { destination: Option<String>, amount: i64, added: i64, previously: i64, cascade: bool, from_ravage: bool, from_box: bool, from_card: i64, destroy_presence: bool },
    /// Blight moving between the box, the Blight card and lands (a refill of the pool, a Power moving Blight).
    MoveBlight { moves: Vec<(String, String, String)>, source: Option<String> },
    RemoveBlight { land: Option<String>, amount: i64, source: Option<String> },
    Defend { lands: Vec<String>, amount: i64, source: Option<String> },
    AddTokens { adds: Vec<(String, String)>, source: Option<String> },
    MoveTokens { moves: Vec<(String, String, String)>, reason: String },
    MoveCards { moves: Vec<(String, String, String)>, spirit: Option<String> },
    RevealCards { cards: Vec<String>, from: Option<String>, spirit: Option<String>, from_gain: bool },
    GainPowerCard { spirit: Option<String>, card: Option<String>, from: Option<String>, not_gained: Vec<String>, taken: bool },
    ResolveEventCard { card: String, start: bool },
    ResolveFearCard { card: String, terror: i64, start: bool },
    RuleAction { rule: String, initial: bool, triggering: Option<i64> },
    FinishRuleAction { rule: String, initial: bool, start: i64 },
    GameSetup,
    /// The engine's own message to the player ("Game Over: Victory!").
    Text(String),
    /// An entry class this reader does not type yet (counted, never dropped silently).
    Other(String),
}

/// The journal's entries in order, from a decoded save.
pub fn entries(g: &Graph) -> Vec<Entry> {
    let Some(items) = g.root().child("Journal").and_then(|j| j.child("journalEntries")) else { return Vec::new() };
    items.item_nodes().into_iter().enumerate().map(|(i, n)| entry(i, n)).collect()
}

fn entry(index: usize, n: Node) -> Entry {
    let state = n.child("State");
    let phase = enum_name(PHASE_TYPES, state.and_then(|s| s.child("PhaseType")).and_then(|p| p.boxed_int()));
    let state_name = enum_name(STATE_TYPES, state.and_then(|s| s.child("StateType")).and_then(|p| p.boxed_int()));
    Entry {
        index,
        turn_index: n.int("TurnIndex").unwrap_or(0),
        state_index: n.int("StateIndex").unwrap_or(0),
        phase,
        state: state_name,
        undo_related: n.bool("IsUndoRelated").unwrap_or(false),
        kind: kind(n),
    }
}

// ------------------------------------------------------------------ resolving names

/// A model object's identifier, else its title.
fn name(n: Node) -> Option<String> {
    n.str("Identifier").or_else(|| n.str("title")).or_else(|| n.str("Title")).map(String::from)
}

fn name_of(n: Option<Node>) -> Option<String> {
    n.and_then(name)
}

/// A location as the log prints it: its title ("Out of Game", "Thunderspeaker's Hand"),
/// else its identifier (a land: "SandsA4").
fn location(n: Node) -> Option<String> {
    n.str("title").filter(|t| !t.is_empty()).or_else(|| n.str("Identifier")).map(String::from)
}

fn location_of(n: Option<Node>) -> Option<String> {
    n.and_then(location)
}

fn state_name(n: Option<Node>) -> Option<String> {
    n.map(|s| enum_name(STATE_TYPES, s.child("StateType").and_then(|p| p.boxed_int())))
}

fn names(n: Option<Node>) -> Vec<String> {
    n.map(|l| l.item_nodes().into_iter().filter_map(name).collect()).unwrap_or_default()
}

fn locations(n: Option<Node>) -> Vec<String> {
    n.map(|l| l.item_nodes().into_iter().filter_map(location).collect()).unwrap_or_default()
}

fn enums(n: Option<Node>, table: &[&str]) -> Vec<String> {
    n.map(|l| l.item_nodes().into_iter().map(|e| enum_name(table, e.boxed_int())).collect()).unwrap_or_default()
}

/// The items of a `List<Tuple<..>>`, each tuple's members resolved as names (the first)
/// and locations (the rest).
fn tuples(n: Option<Node>) -> Vec<Vec<Option<String>>> {
    n.map(|l| {
        l.item_nodes()
            .into_iter()
            .map(|t| (1..=4).map(|i| t.child(&format!("m_Item{i}")).and_then(|m| if i == 1 { name(m) } else { location(m).or_else(|| name(m)) })).collect())
            .collect()
    })
    .unwrap_or_default()
}

fn pair(t: &[Option<String>]) -> Option<(String, String)> {
    Some((t.first()?.clone()?, t.get(1)?.clone()?))
}

fn triple(t: &[Option<String>]) -> Option<(String, String, String)> {
    Some((t.first()?.clone()?, t.get(1).cloned().flatten().unwrap_or_default(), t.get(2).cloned().flatten().unwrap_or_default()))
}

fn strings(n: Option<Node>) -> Vec<String> {
    n.map(|l| {
        l.items()
            .into_iter()
            .filter_map(|v| match v {
                Value::Ref(id) => l.graph.node(id).as_str().map(String::from),
                _ => None,
            })
            .collect()
    })
    .unwrap_or_default()
}

fn kind(n: Node) -> Kind {
    let class = n.class_name().unwrap_or("?");
    let source = name_of(n.child("Source"));
    match class {
        "ChangeStateJournalEntry" => Kind::ChangeState { from: state_name(n.child("FromState")), to: state_name(n.child("ToState")).unwrap_or_default() },
        "UseActivationJournalEntry" => Kind::UseActivation {
            activation: n.str("ActivationType").unwrap_or("").to_string(),
            instance: n.str("ActivationInstanceIdentifier").unwrap_or("").to_string(),
            selection: n.int("SelectionIndex"),
            target: n.child("PowerTarget").and_then(|t| location(t).or_else(|| name(t))),
            power: name_of(n.child("Power")),
            location: location_of(n.child("Location")),
            pieces: names(n.child("Pieces")),
            board: name_of(n.child("Board")),
            choices: strings(n.child("ChoiceIdentifiers")),
            completed: n.bool("HasCompleted").unwrap_or(false),
            spend_energy: n.bool("ShouldSpendEnergy").unwrap_or(false),
        },
        "AddIslandPiecesJournalEntry" => Kind::AddPieces { adds: tuples(n.child("IslandPieceAdditions")).iter().filter_map(|t| pair(t)).collect(), source },
        "MoveIslandPiecesJournalEntry" => Kind::MovePieces {
            moves: tuples(n.child("IslandPieceMovements")).iter().filter_map(|t| triple(t)).collect(),
            reason: enum_name(MOVE_REASONS, n.child("Reason").and_then(|r| r.boxed_int())),
            source,
        },
        "DestroyPiecesJournalEntry" => {
            let destroys = tuples(n.child("destroys"));
            let pieces = if destroys.is_empty() {
                names(n.child("destroys")).into_iter().map(|p| (p, None)).collect()
            } else {
                destroys.iter().filter_map(|t| Some((t.first()?.clone()?, t.get(1).cloned().flatten()))).collect()
            };
            Kind::DestroyPieces { pieces, source: name_of(n.child("SourceModel")).or(source) }
        }
        "MovePresenceJournalEntry" => Kind::MovePresence {
            spirit: names(n.child("SpiritsMovingPresence")).into_iter().next().unwrap_or_default(),
            moves: tuples(n.child("PresenceMovements")).iter().filter_map(|t| triple(t)).collect(),
            reason: enum_name(MOVE_REASONS, n.child("Reason").and_then(|r| r.boxed_int())),
        },
        "GainEnergyJournalEntry" => Kind::GainEnergy { spirit: name_of(n.child("Spirit")).unwrap_or_default(), amount: n.int("Amount").unwrap_or(0) },
        "PayEnergyJournalEntry" => Kind::PayEnergy { spirit: name_of(n.child("Spirit")).unwrap_or_default(), amount: n.int("Amount").unwrap_or(0) },
        "GainElementsJournalEntry" => Kind::GainElements { spirit: name_of(n.child("Spirit")).unwrap_or_default(), elements: enums(n.child("Elements"), ELEMENTS), source },
        "LoseElementsJournalEntry" => Kind::LoseElements { spirit: name_of(n.child("Spirit")).unwrap_or_default(), elements: enums(n.child("Elements"), ELEMENTS) },
        "PlayPowerCardJournalEntry" => Kind::PlayPowerCard {
            spirit: name_of(n.child("SpiritPlayingCard")).unwrap_or_default(),
            card: name_of(n.child("PowerCardPlayed")).unwrap_or_default(),
            unplayed: n.bool("WasCardUnplayed").unwrap_or(false),
        },
        "StartPowerJournalEntry" | "UsePowerJournalEntry" => {
            let spirit = name_of(n.child("SpiritThatUsedPower")).unwrap_or_default();
            let power = name_of(n.child("PowerUsed")).unwrap_or_default();
            let target = n.child("PowerTarget").and_then(|t| location(t).or_else(|| name(t)));
            if class == "StartPowerJournalEntry" { Kind::StartPower { spirit, power, target } } else { Kind::UsePower { spirit, power, target } }
        }
        "ModifyPowersJournalEntry" => Kind::ModifyPowers { powers: names(n.child("powersModified")), reset: n.bool("ResetPower").unwrap_or(false) },
        "UseGrowthOptionJournalEntry" => Kind::UseGrowthOption {
            spirit: name_of(n.child("SpirithWithGrowth")).unwrap_or_default(),
            group: n.int("GrowthGroupNumber"),
            option_index: n.int("GrowthOptionIndex"),
            always: n.bool("IsAlwaysGrowthOption").unwrap_or(false),
            undone: n.bool("WasGrowthUndone").unwrap_or(false),
            option: name_of(n.child("GrowthOption")),
        },
        "FinishGrowthOptionJournalEntry" => Kind::FinishGrowthOption {
            spirit: name_of(n.child("SpirithWithGrowth")).unwrap_or_default(),
            group: n.int("GrowthGroupNumber"),
            option_index: n.int("GrowthOptionIndex"),
            option: name_of(n.child("GrowthOption")),
        },
        "AdjustFearGeneratedJournalEntry" => Kind::AdjustFear {
            amount: n.int("AmountOfFearGenerated").unwrap_or(0),
            land: n.child("targetIslandLocation").or_else(|| n.child("TargetLand")).and_then(location),
            before: n.int("AmountOfFearGeneratedBeforeAction").unwrap_or(0),
        },
        "FlipCardJournalEntry" => {
            let c = n.child("FlippedCard");
            Kind::FlipCard { class: c.and_then(|c| c.class_name()).unwrap_or("?").to_string(), card: name_of(c).unwrap_or_default(), flipped: n.bool("FlippedValueAfter").unwrap_or(false) }
        }
        "FlipCardStackJournalEntry" => Kind::FlipCardStack { location: location_of(n.child("FlippedLocation")).unwrap_or_default() },
        "InvaderStateJournalEntry" => Kind::InvaderState {
            card: name_of(n.child("SourceInvaderCard")).unwrap_or_default(),
            lands: locations(n.child("LandsToActUpon")),
            matching: locations(n.child("LandsMatchingInvaderCard")),
        },
        "ExploreJournalEntry" => Kind::Explore { land: location_of(n.child("LandExplored")).unwrap_or_default(), card: name_of(n.child("ExploreSlotInvaderCard")), success: n.bool("WasSuccessfullyExplorer").unwrap_or(false) },
        "BuildLandJournalEntry" => Kind::BuildLand { land: location_of(n.child("LandBuilt")).unwrap_or_default(), card: name_of(n.child("BuildSlotInvaderCard")), cancelled: n.bool("WasCancelled").unwrap_or(false) },
        "RavageLandJournalEntry" => Kind::RavageLand { land: location_of(n.child("LandRavaged")).unwrap_or_default(), card: name_of(n.child("RavageSlotInvaderCard")), cancelled: n.bool("WasCancelled").unwrap_or(false) },
        "RavageCompleteJournalEntry" => Kind::RavageComplete {
            land: location_of(n.child("LandRavaged")).unwrap_or_default(),
            card: name_of(n.child("RavageSlotInvaderCard")),
            blighted: n.bool("DidAddBlight").unwrap_or(false),
            lands_blighted: locations(n.child("LandsBlighted")),
        },
        "DealDamageJournalEntry" => {
            let distribution = n
                .child("DamageDistribution")
                .map(|d| {
                    d.dict_entries()
                        .into_iter()
                        .filter_map(|(k, v)| {
                            let key = match k {
                                Value::Ref(id) => d.graph.node(id).as_str().map(String::from)?,
                                _ => return None,
                            };
                            Some((key, v.as_i64().unwrap_or(0)))
                        })
                        .collect()
                })
                .unwrap_or_default();
            Kind::DealDamage {
                sources: names(n.child("DamageSources")),
                targets: names(n.child("DamageTargets")),
                lands: locations(n.child("islandLocationsWhereDamageWasDealt")),
                distribution,
                set_instead: n.bool("SetDamageInstead").unwrap_or(false),
                pretend: n.bool("IsDamagePretend").unwrap_or(false),
            }
        }
        "AddBlightJournalEntry" => Kind::AddBlight {
            destination: location_of(n.child("Destination")).or_else(|| location_of(n.child("LandDestination"))),
            amount: n.int("AmountOfBlight").unwrap_or(0),
            added: n.int("AmountOfBlightActuallyAdded").unwrap_or(0),
            previously: n.int("AmountOfBlightPreviouslyAdded").unwrap_or(0),
            cascade: n.bool("IsFromCascade").unwrap_or(false),
            from_ravage: n.bool("IsFromRavage").unwrap_or(false),
            from_box: n.bool("IsFromBox").unwrap_or(false),
            from_card: n.int("AmountFromBlightCard").unwrap_or(0),
            destroy_presence: n.bool("DestroyPresence").unwrap_or(false),
        },
        "DefendJournalEntry" => Kind::Defend { lands: locations(n.child("IslandLocationsToDefend").or_else(|| n.child("LandsToDefend"))), amount: n.int("Amount").unwrap_or(0), source },
        "MoveBlightJournalEntry" => Kind::MoveBlight { moves: tuples(n.child("BlightMovements")).iter().filter_map(|t| triple(t)).collect(), source },
        "RemoveBlightJournalEntry" => Kind::RemoveBlight {
            land: n.child("IslandLocationRemovingBlight").or_else(|| n.child("LandRemovingBlight")).and_then(location),
            amount: n.int("AmountOfBlight").unwrap_or(0),
            source,
        },
        "AddTokenJournalEntry" => Kind::AddTokens { adds: tuples(n.child("TokenAdditions")).iter().filter_map(|t| pair(t)).collect(), source },
        "MoveTokensJournalEntry" => Kind::MoveTokens { moves: tuples(n.child("TokenMovements")).iter().filter_map(|t| triple(t)).collect(), reason: enum_name(MOVE_REASONS, n.child("Reason").and_then(|r| r.boxed_int())) },
        "MoveCardsJournalEntry" => Kind::MoveCards { moves: tuples(n.child("CardMovements")).iter().filter_map(|t| triple(t)).collect(), spirit: name_of(n.child("SpiritMovingCards")) },
        "RevealCardsJournalEntry" => Kind::RevealCards {
            cards: names(n.child("revealedCards")),
            from: location_of(n.child("LocationRevealedFrom")),
            spirit: name_of(n.child("SpiritRevealingCards")),
            from_gain: n.bool("FromGainCard").unwrap_or(false),
        },
        "GainPowerCardJournalEntry" => Kind::GainPowerCard {
            spirit: name_of(n.child("SpiritGainingCard")),
            card: name_of(n.child("PowerCardGained")),
            from: location_of(n.child("LocationGainedFrom")),
            not_gained: names(n.child("PowerCardsNotGained")),
            taken: n.bool("WasTaken").unwrap_or(false),
        },
        "ResolveEventCardJournalEntry" => Kind::ResolveEventCard { card: name_of(n.child("EventCardResolved")).unwrap_or_default(), start: n.bool("StartOfResolution").unwrap_or(false) },
        "ResolveFearCardJournalEntry" => Kind::ResolveFearCard {
            card: name_of(n.child("FearCardResolved")).unwrap_or_default(),
            terror: n.int("TerrorLevel").unwrap_or(0),
            start: n.bool("StartOfResolution").unwrap_or(false),
        },
        "RuleActionJournalEntry" => Kind::RuleAction {
            rule: enum_name(RULE_ACTIONS, n.child("RuleActionType").and_then(|r| r.boxed_int())),
            initial: n.bool("IsInitialAction").unwrap_or(false),
            triggering: n.int("TriggeringRuleActionIndex"),
        },
        "FinishRuleActionJournalEntry" => Kind::FinishRuleAction {
            rule: enum_name(RULE_ACTIONS, n.child("RuleActionType").and_then(|r| r.boxed_int())),
            initial: n.bool("IsInitialAction").unwrap_or(false),
            start: n.int("StartRuleActionIndex").unwrap_or(0),
        },
        "GameSetupJournalEntry" => Kind::GameSetup,
        "TextJournalEntry" => Kind::Text(n.str("Text").unwrap_or("").to_string()),
        other => Kind::Other(other.to_string()),
    }
}

impl Entry {
    /// One line, for `si-steam-dump journal` and the eye.
    pub fn words(&self) -> String {
        format!("[{:>3}] T{} {:<22} {}{}", self.index, self.turn_index + 1, format!("{}/{}", self.phase, self.state), kind_words(&self.kind), if self.undo_related { "  [undo]" } else { "" })
    }
}

fn kind_words(k: &Kind) -> String {
    let moves = |m: &[(String, String, String)]| m.iter().map(|(p, f, t)| format!("{p} ({f} => {t})")).collect::<Vec<_>>().join(", ");
    let adds = |m: &[(String, String)]| m.iter().map(|(p, t)| format!("{p} => {t}")).collect::<Vec<_>>().join(", ");
    match k {
        Kind::ChangeState { from, to } => format!("ChangeState {} -> {to}", from.as_deref().unwrap_or("-")),
        Kind::UseActivation { activation, instance, selection, target, power, location, pieces, board, choices, completed, spend_energy } => {
            let mut parts = vec![format!("UseActivation {activation} [{instance}]")];
            if let Some(s) = selection { parts.push(format!("selection {s}")); }
            if let Some(t) = target { parts.push(format!("target {t}")); }
            if let Some(p) = power { parts.push(format!("power {p}")); }
            if let Some(l) = location { parts.push(format!("location {l}")); }
            if !pieces.is_empty() { parts.push(format!("pieces {}", pieces.join("/"))); }
            if let Some(b) = board { parts.push(format!("board {b}")); }
            if !choices.is_empty() { parts.push(format!("choices {}", choices.join("/"))); }
            if *spend_energy { parts.push("spend energy".into()); }
            if !completed { parts.push("not completed".into()); }
            parts.join(" ")
        }
        Kind::AddPieces { adds: a, source } => format!("AddPieces {} (source {})", adds(a), source.as_deref().unwrap_or("-")),
        Kind::MovePieces { moves: m, reason, source } => format!("MovePieces {} reason {reason} (source {})", moves(m), source.as_deref().unwrap_or("-")),
        Kind::DestroyPieces { pieces, source } => format!("DestroyPieces {} (source {})", pieces.iter().map(|(p, l)| format!("{p}@{}", l.as_deref().unwrap_or("?"))).collect::<Vec<_>>().join(", "), source.as_deref().unwrap_or("-")),
        Kind::MovePresence { spirit, moves: m, reason } => format!("MovePresence {spirit} {} reason {reason}", moves(m)),
        Kind::GainEnergy { spirit, amount } => format!("GainEnergy {spirit} +{amount}"),
        Kind::PayEnergy { spirit, amount } => format!("PayEnergy {spirit} -{amount}"),
        Kind::GainElements { spirit, elements, source } => format!("GainElements {spirit} {} (source {})", elements.join("+"), source.as_deref().unwrap_or("-")),
        Kind::LoseElements { spirit, elements } => format!("LoseElements {spirit} {}", elements.join("+")),
        Kind::PlayPowerCard { spirit, card, unplayed } => format!("{} {spirit} {card}", if *unplayed { "UnplayPowerCard" } else { "PlayPowerCard" }),
        Kind::StartPower { spirit, power, target } => format!("StartPower {spirit} {power} -> {}", target.as_deref().unwrap_or("-")),
        Kind::UsePower { spirit, power, target } => format!("UsePower {spirit} {power} -> {}", target.as_deref().unwrap_or("-")),
        Kind::ModifyPowers { powers, reset } => format!("ModifyPowers {} reset {reset}", powers.join("/")),
        Kind::UseGrowthOption { spirit, group, option_index, always, undone, option } => format!("UseGrowthOption {spirit} G{} #{} {}{}{}", group.map(|g| g.to_string()).unwrap_or("-".into()), option_index.map(|g| g.to_string()).unwrap_or("-".into()), option.as_deref().unwrap_or("?"), if *always { " (always)" } else { "" }, if *undone { " UNDONE" } else { "" }),
        Kind::FinishGrowthOption { spirit, group, option_index, option } => format!("FinishGrowthOption {spirit} G{} #{} {}", group.map(|g| g.to_string()).unwrap_or("-".into()), option_index.map(|g| g.to_string()).unwrap_or("-".into()), option.as_deref().unwrap_or("?")),
        Kind::AdjustFear { amount, land, before } => format!("AdjustFear {amount:+} at {} (before {before})", land.as_deref().unwrap_or("-")),
        Kind::FlipCard { class, card, flipped } => format!("FlipCard {class} {card} -> {}", if *flipped { "face up" } else { "face down" }),
        Kind::FlipCardStack { location } => format!("FlipCardStack {location}"),
        Kind::InvaderState { card, lands, matching } => format!("InvaderState {card} acts on {} (matching {})", lands.join("/"), matching.join("/")),
        Kind::Explore { land, card, success } => format!("Explore {land} by {}{}", card.as_deref().unwrap_or("?"), if *success { "" } else { " (no explorer)" }),
        Kind::BuildLand { land, card, cancelled } => format!("Build {land} by {}{}", card.as_deref().unwrap_or("?"), if *cancelled { " CANCELLED" } else { "" }),
        Kind::RavageLand { land, card, cancelled } => format!("Ravage {land} by {}{}", card.as_deref().unwrap_or("?"), if *cancelled { " CANCELLED" } else { "" }),
        Kind::RavageComplete { land, blighted, lands_blighted, .. } => format!("RavageComplete {land}{}", if *blighted { format!(" blight on {}", lands_blighted.join("/")) } else { String::new() }),
        Kind::DealDamage { sources, targets, lands, distribution, set_instead, pretend } => format!("DealDamage from {} to {} in {}: {}{}{}", sources.join("+"), targets.join("/"), lands.join("/"), distribution.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(" "), if *set_instead { " (set)" } else { "" }, if *pretend { " (pretend)" } else { "" }),
        Kind::AddBlight { destination, amount, added, previously, cascade, from_ravage, from_box, from_card, destroy_presence } => format!("AddBlight {} x{amount} (added {added} after {previously}, card {from_card}{}{}{}{})", destination.as_deref().unwrap_or("-"), if *cascade { ", cascade" } else { "" }, if *from_ravage { ", ravage" } else { "" }, if *from_box { ", box" } else { "" }, if *destroy_presence { ", destroys presence" } else { "" }),
        Kind::Defend { lands, amount, source } => format!("Defend {} by {amount} (source {})", lands.join("/"), source.as_deref().unwrap_or("-")),
        Kind::MoveBlight { moves: m, source } => format!("MoveBlight {} (source {})", moves(m), source.as_deref().unwrap_or("-")),
        Kind::RemoveBlight { land, amount, source } => format!("RemoveBlight {} x{amount} (source {})", land.as_deref().unwrap_or("?"), source.as_deref().unwrap_or("-")),
        Kind::AddTokens { adds: a, source } => format!("AddTokens {} (source {})", adds(a), source.as_deref().unwrap_or("-")),
        Kind::MoveTokens { moves: m, reason } => format!("MoveTokens {} reason {reason}", moves(m)),
        Kind::MoveCards { moves: m, spirit } => format!("MoveCards {} ({})", moves(m), spirit.as_deref().unwrap_or("-")),
        Kind::RevealCards { cards, from, spirit, from_gain } => format!("RevealCards {} from {} ({}{})", cards.join("/"), from.as_deref().unwrap_or("?"), spirit.as_deref().unwrap_or("-"), if *from_gain { ", gain" } else { "" }),
        Kind::GainPowerCard { spirit, card, from, not_gained, taken } => format!("GainPowerCard {} {} from {} (not gained {}{})", spirit.as_deref().unwrap_or("-"), card.as_deref().unwrap_or("?"), from.as_deref().unwrap_or("?"), not_gained.join("/"), if *taken { ", taken" } else { "" }),
        Kind::ResolveEventCard { card, start } => format!("ResolveEventCard {card} {}", if *start { "start" } else { "end" }),
        Kind::ResolveFearCard { card, terror, start } => format!("ResolveFearCard {card} at TL{terror} {}", if *start { "start" } else { "end" }),
        Kind::RuleAction { rule, initial, triggering } => format!("RuleAction {rule}{}{}", if *initial { " initial" } else { "" }, triggering.map(|t| format!(" triggered by #{t}")).unwrap_or_default()),
        Kind::FinishRuleAction { rule, start, .. } => format!("FinishRuleAction {rule} (from #{start})"),
        Kind::GameSetup => "GameSetup".into(),
        Kind::Text(t) => format!("Text {t}"),
        Kind::Other(c) => format!("? {c}"),
    }
}

// ------------------------------------------------------------------ to the record

fn phase_of(state: &str) -> Phase {
    Phase::parse(state)
}

fn step_of(state: &str) -> Option<InvaderStep> {
    match state {
        "Explore" => Some(InvaderStep::Explore),
        "Build" | "HighImmigration" => Some(InvaderStep::Build),
        "Ravage" => Some(InvaderStep::Ravage),
        _ => None,
    }
}

fn piece_move(piece: &str, from: &str, to: &str) -> PieceMove {
    PieceMove { kind: PieceKind::parse(piece), from: land(from), to: land(to), piece: piece.to_string() }
}

/// A track slot's identifier when a presence came off one (anything but a land, out of
/// game or a forgotten pile).
fn track_slot(loc: &str) -> Option<String> {
    let l = loc.trim();
    if l.is_empty() || land(l).is_some() || l.contains("Out of Game") || l.ends_with("OutOfGame") || l.contains("Forgotten") {
        None
    } else {
        Some(l.to_string())
    }
}

/// The record's events and markers from the entries, in order. `boards` are the island's
/// board ids (the log's `Building Adjacency Matrix` marker); `ending` the save's
/// EndingResult, which the journal itself does not record.
pub fn parsed(entries: &[Entry], boards: &[String], ending: Option<&str>, spirits: &[String], setup_tokens: &[(String, String)]) -> Vec<(Option<u64>, Parsed)> {
    let mut out: Vec<(Option<u64>, Parsed)> = Vec::new();
    // a pile's title names its spirit by title ("Shroud of Silent Mist's Hand"); the
    // record names spirits by identifier
    let spirit_id = |title: &str| -> String {
        let key: String = title.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_ascii_lowercase();
        spirits.iter().find(|id| id.to_ascii_lowercase() == key).cloned().unwrap_or_else(|| title.to_string())
    };
    let mut push = |p: Parsed| out.push((None, p));
    push(Parsed::Marker(Marker::Boards(boards.to_vec())));
    for (land_id, token) in setup_tokens {
        push(Parsed::Event(Event::TokensAdded { land: land(land_id), kind: TokenKind::parse(token), count: 1 }));
    }
    // the Invader card whose step is under way, resolved when the step's state ends
    let mut step_card: Option<(InvaderStep, String)> = None;
    let mut groups_marked: BTreeSet<(i64, String, i64)> = BTreeSet::new();
    let mut last_revealed: Vec<String> = Vec::new();
    // the last AddBlight entry: (index, land, cascade, amount, added), to read a re-journaled action as its continuation
    let mut last_blight: Option<(usize, String, bool, i64, i64)> = None;
    // a DestroyPieces entry is held until the move off the island that follows it has gone
    // through: the walk takes the move as the removal and skips the destroy of a piece gone
    let mut held_destroy: Option<Event> = None;
    // the Event or Fear card being resolved: the source of a choice whose identifier does not name it
    let mut resolving: Option<String> = None;
    // the source of the last destroy in the Invader step under way: what paid for a cancel
    // (Rampant Green's presence for a Ravage it chokes)
    let mut step_source: Option<String> = None;
    let mut state = String::new();
    for e in entries {
        match &e.kind {
            Kind::ResolveEventCard { card, start } | Kind::ResolveFearCard { card, start, .. } => resolving = start.then(|| card.clone()),
            _ => {}
        }
        if e.state != state {
            state = e.state.clone();
            step_source = None;
        }
        let destroy_move = matches!(&e.kind, Kind::MovePieces { reason, .. } | Kind::MoveTokens { reason, .. } | Kind::MovePresence { reason, .. } if reason == "Destroy");
        if !destroy_move {
            if let Some(d) = held_destroy.take() {
                push(Parsed::Event(d));
            }
        }
        match &e.kind {
            Kind::ChangeState { from, to } => {
                if let Some((step, card)) = step_card.take() {
                    push(Parsed::Event(Event::InvaderCardResolved { step, card }));
                }
                push(Parsed::Event(Event::PhaseChanged { from: from.as_deref().map(phase_of), to: phase_of(to) }));
                if to == "TimePasses" {
                    push(Parsed::Event(Event::TimePassed));
                }
                state = to.clone();
            }
            Kind::InvaderState { card, .. } => {
                if let Some(step) = step_of(&state) {
                    if let Some((prev_step, prev)) = step_card.take() {
                        if prev != *card {
                            push(Parsed::Event(Event::InvaderCardResolved { step: prev_step, card: prev }));
                        }
                    }
                    step_card = Some((step, card.clone()));
                }
            }
            Kind::RavageLand { land: l, card, cancelled: false } => {
                step_source = None;
                if let Some(id) = land(l) {
                    push(Parsed::Event(Event::Ravaged { land: id, source: card.clone() }));
                }
            }
            Kind::RavageLand { land: l, cancelled: true, .. } | Kind::BuildLand { land: l, cancelled: true, .. } => {
                let step = if matches!(e.kind, Kind::RavageLand { .. }) { InvaderStep::Ravage } else { InvaderStep::Build };
                if let Some(id) = land(l) {
                    push(Parsed::Event(Event::Canceled { step, land: Some(id), source: step_source.take().unwrap_or_else(|| "the engine".into()) }));
                }
            }
            Kind::BuildLand { .. } => step_source = None,
            Kind::AddPieces { adds, source } => {
                let (tokens, pieces): (Vec<_>, Vec<_>) = adds.iter().partition(|(p, _)| TokenKind::parse(p).is_some() && PieceKind::parse(p).is_none());
                if !pieces.is_empty() {
                    push(Parsed::Event(Event::PiecesAdded { moves: pieces.iter().map(|(p, t)| piece_move(p, "", t)).collect() }));
                }
                for (p, t) in tokens {
                    push(Parsed::Event(Event::TokensAdded { land: land(t), kind: TokenKind::parse(p), count: 1 }));
                }
                let _ = source;
            }
            Kind::MovePieces { moves, .. } => push(Parsed::Event(Event::PiecesMoved { moves: moves.iter().map(|(p, f, t)| piece_move(p, f, t)).collect() })),
            Kind::DestroyPieces { pieces, source } => {
                if step_of(&state).is_some() {
                    step_source = source.clone();
                }
                if let Some(d) = held_destroy.take() {
                    push(Parsed::Event(d));
                }
                // the land goes into the source as the log wrote it, for a printed piece the record has not named yet
                let at = pieces.iter().find_map(|(_, l)| l.as_deref().and_then(land)).map(|l| format!(" LandTarget: {l},")).unwrap_or_default();
                held_destroy = Some(Event::PiecesDestroyed {
                    pieces: pieces.iter().map(|(p, l)| (p.clone(), l.as_deref().and_then(land))).collect(),
                    source: format!("{}{at}", source.clone().unwrap_or_default()),
                });
            }
            Kind::MovePresence { spirit, moves, reason } => {
                for (presence, from, to) in moves {
                    let from_land = land(from);
                    let to_land = land(to);
                    let destroyed = to_land.is_none() && (reason == "Destroy" || to.contains("Out of Game") || to.ends_with("OutOfGame") || to.contains("Forgotten Powers"));
                    let node = if from_land.is_none() { track_slot(from) } else if to_land.is_none() && !destroyed { track_slot(to) } else { None };
                    let _ = presence;
                    push(Parsed::Event(Event::PresenceMoved { spirit: spirit.clone(), node, from: from_land, to: to_land, destroyed }));
                }
            }
            Kind::GainEnergy { spirit, amount } => push(Parsed::Event(Event::EnergyGained { spirit: spirit.clone(), amount: *amount })),
            Kind::PayEnergy { spirit, amount } => push(Parsed::Event(Event::EnergyPaid { spirit: spirit.clone(), amount: *amount })),
            Kind::GainElements { spirit, elements, source } => push(Parsed::Event(Event::ElementsGained { spirit: Some(spirit.clone()), source: source.clone().unwrap_or_default(), elements: elements.clone() })),
            Kind::PlayPowerCard { spirit, card, unplayed } => push(Parsed::Event(if *unplayed {
                Event::CardUnplayed { spirit: Some(spirit.clone()), card: card.clone() }
            } else {
                Event::CardPlayed { spirit: Some(spirit.clone()), card: card.clone() }
            })),
            Kind::StartPower { power, .. } => push(Parsed::Marker(Marker::PowerPhase { power: power.clone(), phase: phase_of(&e.state) })),
            Kind::UsePower { spirit, power, target } => push(Parsed::Event(Event::PowerUsed { spirit: Some(spirit.clone()), power: power.clone(), target: target.as_deref().and_then(land), speed: None })),
            Kind::UseGrowthOption { spirit, group, option, undone, always, .. } => {
                if let Some(g) = group {
                    if !*always && groups_marked.insert((e.turn_index, spirit.clone(), *g)) {
                        push(Parsed::Marker(Marker::GrowthGroup { spirit: spirit.clone(), group: *g as u8 }));
                    }
                }
                if let Some(o) = option {
                    push(Parsed::Marker(Marker::GrowthOption { spirit: spirit.clone(), option: o.clone(), undo: *undone }));
                }
            }
            Kind::AdjustFear { amount, land: l, .. } => push(Parsed::Event(Event::Fear { amount: *amount as i32, land: l.as_deref().and_then(land), source: String::new() })),
            Kind::FlipCard { class, card, flipped: true } if class == "InvaderCard" => push(Parsed::Event(Event::InvaderCardRevealed { card: card.clone() })),
            Kind::DealDamage { sources, lands, distribution, pretend: false, .. } => {
                let amount: i64 = distribution.iter().map(|(_, v)| *v).sum();
                push(Parsed::Event(Event::Damage {
                    source: sources.first().cloned().unwrap_or_default(),
                    land: lands.first().and_then(|l| land(l)),
                    amount: amount.max(0) as u32,
                    distribution: distribution.iter().map(|(k, v)| (k.clone(), (*v).max(0) as u32)).collect(),
                }));
            }
            Kind::AddBlight { destination, amount, added, previously, cascade, .. } => {
                let dest = destination.clone().unwrap_or_default();
                let continued = last_blight.as_ref().is_some_and(|(i, l, c, a, _)| *i + 1 == e.index && *l == dest && *c == *cascade && *a == *amount);
                let before = if continued { last_blight.as_ref().map(|x| x.4).unwrap_or(0).max(*previously) } else { *previously };
                if *added > before {
                    push(Parsed::Event(Event::BlightAdded { land: destination.as_deref().and_then(land), amount: (*added - before) as u32, cascade: *cascade }));
                }
                last_blight = Some((e.index, dest, *cascade, *amount, *added));
            }
            Kind::MoveTokens { moves, .. } => {
                push(Parsed::Event(Event::PiecesMoved { moves: moves.iter().map(|(p, f, t)| piece_move(p, f, t)).collect() }));
                // a token the adversary keeps on its own panel (Russia's Hunters and their Beasts)
                for (p, _, t) in moves.iter().filter(|(_, _, t)| t.contains("Adversary Location")) {
                    let _ = t;
                    push(Parsed::Event(Event::AdversaryTook { piece: p.clone() }));
                }
            }
            Kind::Defend { lands, amount, source } => push(Parsed::Event(Event::Defend { lands: lands.iter().filter_map(|l| land(l)).collect(), amount: (*amount).max(0) as u32, source: source.clone().unwrap_or_default() })),
            Kind::MoveBlight { moves, .. } => {
                // land to land is a move; a pool refill or a removal to the box is not a land change
                // (a removal has its own RemoveBlight entry)
                for (_, from, to) in moves {
                    if let (Some(f), Some(t)) = (land(from), land(to)) {
                        push(Parsed::Event(Event::BlightRemoved { land: Some(f), amount: 1 }));
                        push(Parsed::Event(Event::BlightAdded { land: Some(t), amount: 1, cascade: false }));
                    }
                }
            }
            Kind::RemoveBlight { land: l, amount, .. } if *amount > 0 => push(Parsed::Event(Event::BlightRemoved { land: l.as_deref().and_then(land), amount: *amount as u32 })),
            Kind::AddTokens { adds, .. } => {
                for (tok, loc) in adds {
                    push(Parsed::Event(Event::TokensAdded { land: land(loc), kind: TokenKind::parse(tok), count: 1 }));
                }
            }
            Kind::MoveCards { moves, .. } => {
                let mut events = Vec::new();
                for (card, from, to) in moves {
                    if to == "Fear Earned" {
                        events.push(Event::FearCardEarned);
                    } else if card.starts_with("TerrorLevel") && to == "Out of Game" {
                        events.push(Event::TerrorLevel { level: if card.ends_with("III") { 3 } else { 2 } });
                    } else if (to == "Minor Power Discard" || to == "Major Power Discard") && (from.ends_with("'s Hand") || from.ends_with("'s Discard") || from.ends_with("'s Play Area")) {
                        // forgotten: from the spirit's own pile back to the deck's discard (a card
                        // revealed and not gained goes back from the reveal pile: not a forget)
                        events.push(Event::CardForgotten { spirit: from.split("'s ").next().map(&spirit_id), card: card.clone() });
                    } else if let Some(ev) = card_pile_move(card, from, to) {
                        events.push(match ev {
                            Event::CardGained { spirit, card, deck } => Event::CardGained { spirit: spirit.map(|s| spirit_id(&s)), card, deck },
                            Event::CardsDiscarded { spirit, cards } => Event::CardsDiscarded { spirit: spirit.map(|s| spirit_id(&s)), cards },
                            Event::CardForgotten { spirit, card } => Event::CardForgotten { spirit: spirit.map(|s| spirit_id(&s)), card },
                            other => other,
                        });
                    }
                }
                match events.len() {
                    0 => push(Parsed::Skipped("MoveCards".into())),
                    1 => push(Parsed::Event(events.pop().unwrap())),
                    _ => push(Parsed::Many(events)),
                }
            }
            Kind::RevealCards { cards, from_gain: true, .. } => last_revealed = cards.clone(),
            Kind::GainPowerCard { card: Some(card), not_gained, .. } => {
                // the draft in the order it was shown, as the log's CardChoices had it
                let choices = if last_revealed.contains(card) {
                    std::mem::take(&mut last_revealed)
                } else {
                    let mut c = not_gained.clone();
                    c.push(card.clone());
                    c
                };
                push(Parsed::Marker(Marker::CardChoice { reason: "GainPower".into(), selected: card.clone(), choices }));
            }
            Kind::ResolveEventCard { card, start: false } => push(Parsed::Event(Event::EventResolved { card: card.clone() })),
            Kind::ResolveEventCard { card, start: true } => push(Parsed::Event(Event::EventRevealed { card: card.clone() })),
            Kind::ResolveFearCard { card, start: false, .. } => push(Parsed::Event(Event::FearCardResolved { card: card.clone() })),
            Kind::UseActivation { activation, instance, selection, target, location, pieces, board, choices, .. } => {
                if let Some(m) = choice(activation, instance, *selection, target.as_deref(), location.as_deref(), pieces, board.as_deref(), choices, resolving.as_deref()) {
                    push(Parsed::Marker(m));
                }
            }
            Kind::GameSetup => push(Parsed::Event(Event::Setup { text: "setup complete".into() })),
            Kind::Text(_) => push(Parsed::Noise),
            Kind::RuleAction { .. } | Kind::FinishRuleAction { .. } => push(Parsed::Skipped("RuleAction".into())),
            Kind::Other(c) => push(Parsed::Skipped(c.clone())),
            _ => push(Parsed::Noise),
        }
    }
    if let Some(d) = held_destroy.take() {
        out.push((None, Parsed::Event(d)));
    }
    if let Some(e) = ending {
        if let Some((step, card)) = step_card.take() {
            out.push((None, Parsed::Event(Event::InvaderCardResolved { step, card })));
        }
        out.push((None, Parsed::Event(Event::GameOver { ending: si_model::game::Ending::parse(e) })));
    }
    if let Some((step, card)) = step_card.take() {
        out.push((None, Parsed::Event(Event::InvaderCardResolved { step, card })));
    }
    out
}

/// The player's decisions the log also marks: a location, piece, board or effect chosen,
/// with the options the activation's identifier lists (`...-JungleA3/JungleA8-...`).
#[allow(clippy::too_many_arguments)]
fn choice(activation: &str, instance: &str, selection: Option<i64>, target: Option<&str>, location: Option<&str>, pieces: &[String], board: Option<&str>, listed: &[String], resolving: Option<&str>) -> Option<Marker> {
    let kind = activation.strip_suffix("Activation").unwrap_or(activation);
    if !matches!(kind, "SelectLocation" | "SelectEffect" | "SelectPiece" | "SelectSpirit" | "SelectBoard" | "ChooseNumber") {
        return None;
    }
    let segs: Vec<&str> = instance.split('-').collect();
    let choices: Vec<String> = segs.iter().find(|s| s.contains('/') && s.chars().next().is_some_and(|c| c.is_ascii_uppercase())).map(|s| s.split('/').map(String::from).collect()).unwrap_or_default();
    // an effect chosen is the branch at the selection, in the card's printed order (the
    // engine lists them as `ChoiceIdentifiers`), not the Power's land
    let choices = if kind == "SelectEffect" && !listed.is_empty() { listed.to_vec() } else { choices };
    let branch = if kind == "SelectEffect" { selection.and_then(|i| choices.get(i as usize).cloned()) } else { None };
    let selected = branch
        .or_else(|| location.map(String::from))
        .or_else(|| board.map(String::from))
        .or_else(|| (!pieces.is_empty()).then(|| pieces.join("/")))
        .or_else(|| target.map(String::from))
        .or_else(|| selection.and_then(|i| choices.get(i as usize).cloned()))
        .or_else(|| selection.map(|i| i.to_string()))?;
    // `SelectLocation-0-Thunderspeaker-AddingBeast-JungleA3/JungleA8-ResolveEventCardAction-`:
    // the reason follows the spirit's name; the spirit is the first segment after the kind
    // and its number that reads as a name (no digits, no slash)
    let is_name = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric()) && s.chars().any(|c| c.is_ascii_alphabetic()) && s.chars().next().is_some_and(|c| c.is_ascii_uppercase());
    let names: Vec<&str> = segs.iter().skip(1).copied().filter(|s| is_name(s) && !s.ends_with("Action") && *s != kind && !s.ends_with("Activation")).collect();
    let reason = names.get(1).map(|s| s.to_string()).unwrap_or_default();
    let source_kind = if instance.contains("ResolveEventCardAction") || instance.contains("EventAction") {
        "Event"
    } else if instance.contains("ResolveFearCardAction") {
        "Fear"
    } else if instance.contains("UsePowerAction") {
        "Power"
    } else if instance.contains("ResolveInvaderCardAction") {
        "Invader"
    } else if instance.contains("AddBlightAction") {
        "Blight"
    } else {
        "Rule"
    };
    let kind_name = kind.trim_start_matches("Select").trim_start_matches("Choose").to_string();
    let source = choice_source(&segs, source_kind, resolving);
    Some(Marker::Choice(Choice { phase: Phase::SpiritPhase, kind: kind_name, reason, selected, choices, source, source_kind: source_kind.to_string(), optional: false, spirit: None, instance: 0 }))
}

/// What a choice serves, as the log names it: the Power (`UsePowerAction-<spirit>-<power>`,
/// or a land chosen for `/UsePower-<power>`), the Event or Fear card (`ResolveEventCardAction-<card>`,
/// else the card being resolved). Empty when the identifier names nothing.
fn choice_source(segs: &[&str], source_kind: &str, resolving: Option<&str>) -> String {
    let after = |key: &str, skip: usize| segs.iter().position(|s| s.ends_with(key)).and_then(|i| segs.get(i + 1 + skip)).filter(|s| !s.is_empty()).map(|s| s.to_string());
    if let Some(p) = after("UsePowerAction", 1) {
        return p;
    }
    if let Some(p) = segs.iter().position(|s| s.ends_with("/UsePower")).and_then(|i| segs.get(i + 1)).filter(|s| !s.is_empty()) {
        return p.to_string();
    }
    for key in ["ResolveEventCardAction", "ResolveFearCardAction"] {
        if let Some(c) = after(key, 0) {
            return c;
        }
    }
    if matches!(source_kind, "Event" | "Fear") {
        return resolving.unwrap_or_default().to_string();
    }
    String::new()
}

/// The tokens the engine placed at setup without a journal entry (Jagged Earth adds
/// Beasts and Disease to the base boards when tokens are in play; the board definitions
/// carry only the printed ones): each token the journal never added, at the land its
/// first mention moves or destroys it from, else at the land the save holds it in.
pub fn setup_tokens(snap: &crate::engine_map::EngineSnapshot, entries: &[Entry]) -> Vec<(String, String)> {
    use std::collections::BTreeMap;
    let mut first: BTreeMap<String, Option<String>> = BTreeMap::new(); // token -> the land it was first moved or destroyed from (None: first added)
    let mut note = |token: &str, from: Option<String>| {
        first.entry(token.to_string()).or_insert(from);
    };
    for e in entries {
        match &e.kind {
            Kind::AddTokens { adds, .. } => {
                for (t, _) in adds {
                    note(t, None);
                }
            }
            Kind::AddPieces { adds, .. } => {
                for (t, _) in adds {
                    if TokenKind::parse(t).is_some() {
                        note(t, None);
                    }
                }
            }
            Kind::MoveTokens { moves, .. } => {
                for (t, from, _) in moves {
                    note(t, Some(from.clone()));
                }
            }
            Kind::MovePieces { moves, .. } => {
                for (t, from, _) in moves {
                    if TokenKind::parse(t).is_some() {
                        note(t, Some(from.clone()));
                    }
                }
            }
            Kind::DestroyPieces { pieces, .. } => {
                for (t, at) in pieces {
                    if TokenKind::parse(t).is_some() {
                        note(t, at.clone());
                    }
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    for (token, from) in &first {
        if let Some(land_id) = from {
            if land(land_id).is_some() {
                out.push((land_id.clone(), token.clone()));
            }
        }
    }
    for l in &snap.lands {
        for p in &l.pieces {
            if matches!(p.kind, crate::engine_map::PieceKind::Token(_)) && !first.contains_key(&p.id) && TokenKind::parse(&p.id).is_some() {
                out.push((l.id.clone(), p.id.clone()));
            }
        }
    }
    out.sort();
    out
}

/// The record from a decoded save's journal alone.
pub fn record(g: &Graph, defs: Option<&si_defs::Definitions>, source: &str) -> si_record::record::GameRecord {
    let snap = crate::engine_map::map(g);
    let boards: Vec<String> = snap.boards.iter().map(|b| b.id.clone()).collect();
    let adversary = snap.adversaries.first().map(|(id, _)| id.clone());
    let spirits: Vec<String> = snap.spirits.iter().map(|s| s.id.clone()).collect();
    let all = entries(g);
    let mut setup = setup_tokens(&snap, &all);
    // the board definitions print some tokens (the thematic boards, Jagged Earth's own);
    // the walk seeds those itself, so only the rest are seeded here
    if let Some(d) = defs {
        let mut printed: std::collections::BTreeMap<(String, TokenKind), u32> = std::collections::BTreeMap::new();
        for b in &snap.boards {
            if let Some(def) = d.board_def(&b.id) {
                for l in &def.lands {
                    for (count, kind) in [(l.initial.beasts, TokenKind::Beast), (l.initial.wilds, TokenKind::Wilds), (l.initial.disease, TokenKind::Disease), (l.initial.strife, TokenKind::Strife), (l.initial.badlands, TokenKind::Badlands)] {
                        if count > 0 {
                            *printed.entry((l.id.to_string(), kind)).or_default() += count as u32;
                        }
                    }
                }
            }
        }
        setup.retain(|(land_id, token)| {
            let Some(kind) = TokenKind::parse(token) else { return true };
            let Some(id) = land(land_id) else { return true };
            match printed.get_mut(&(id.to_string(), kind)) {
                Some(n) if *n > 0 => {
                    *n -= 1;
                    false
                }
                _ => true,
            }
        });
    }
    let items = parsed(&all, &boards, snap.ending.as_deref(), &spirits, &setup);
    let mut rec = si_record::replay::Replay::new(defs).run_parsed(items, adversary, source);
    // the save names what the journal never says: the adversary's level, the difficulty, the scenario
    if let Some((_, level)) = snap.adversaries.first() {
        rec.meta.adversary_level = level.map(|l| l as u8);
    }
    rec.meta.difficulty = snap.difficulty;
    rec.meta.scenario = snap.scenario.clone();
    rec.meta.links = crate::engine_map::cross_board_links(&snap);
    rec
}

/// The record's end against the save's own board: every land's counts (pieces, Blight,
/// presence, tokens) as the record replayed them versus as the save holds them. Empty
/// when they agree; the journal reader's own acceptance test.
pub fn check(rec: &si_record::record::GameRecord, seed: &si_record::replay::Seed) -> Vec<String> {
    let mut out = Vec::new();
    let board = seed.board();
    for (land, want) in &board {
        let got = rec.lands.get(land).map(|l| l.end).unwrap_or_default();
        if got != *want {
            out.push(format!("{land}: record {got:?} | save {want:?}"));
        }
    }
    for land in rec.lands.keys() {
        if !board.contains_key(land) && rec.lands[land].end != si_record::record::Counts::default() {
            out.push(format!("{land}: record {:?} | save has no such land", rec.lands[land].end));
        }
    }
    out
}
