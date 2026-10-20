//! One log line → one typed event. Formats were read from the digital
//! edition's logs on 2026-10-20 (EndGame--*.txt and the recorder's
//! log-stream.txt, which prefixes a clock).

use si_model::event::{Event, InvaderStep, PieceMove};
use si_model::game::{Ending, Phase};
use si_model::island::{LandId, PieceKind, TokenKind};

/// A raw line with its clock, when the file has one (`12:49:57.950  text`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub ms: Option<u64>,
    pub text: String,
}

pub fn split_stamp(raw: &str) -> Line {
    if let Some((t, rest)) = raw.split_once("  ") {
        if let Some(ms) = parse_clock(t) {
            return Line { ms: Some(ms), text: rest.trim_start().to_string() };
        }
    }
    Line { ms: None, text: raw.to_string() }
}

fn parse_clock(t: &str) -> Option<u64> {
    let (hms, ms) = t.split_once('.')?;
    let mut parts = hms.split(':').map(|p| p.parse::<u64>().ok());
    let (h, m, s) = (parts.next()??, parts.next()??, parts.next()??);
    Some(((h * 60 + m) * 60 + s) * 1000 + ms.parse::<u64>().ok()?)
}

/// Lines read in one bulk read (the recorder's start, or a re-read after the
/// game rewrote its log) all carry that moment as their stamp: a run of many
/// lines with one stamp. Such lines have no time of their own.
pub fn bulk_flags(lines: &[Line]) -> Vec<bool> {
    let mut out = vec![false; lines.len()];
    let mut i = 0;
    while i < lines.len() {
        let mut j = i;
        while j < lines.len() && lines[j].ms.is_some() && lines[j].ms == lines[i].ms {
            j += 1;
        }
        if j - i >= 30 {
            for b in &mut out[i..j] {
                *b = true;
            }
        }
        i = (j).max(i + 1);
    }
    out
}

/// The recorder re-reads the whole file whenever the game rewrites its log,
/// so a capture holds the same game several times. Keep the last copy (the
/// complete one) and lend it real clocks from the earlier copies, line by
/// line (a rewrite drops some bookkeeping lines, so the alignment skips).
/// Lines that never got a clock of their own come back with `ms: None`.
/// Returns the copy and how many copies the file held.
pub fn timed_last_copy(lines: &[Line]) -> (Vec<Line>, u32) {
    let starts: Vec<usize> = lines.iter().enumerate().filter(|(_, l)| l.text.trim() == "GameView Start").map(|(i, _)| i).collect();
    let copies = starts.len() as u32;
    let Some(&last) = starts.last() else {
        let mut out = lines.to_vec();
        for (l, b) in out.iter_mut().zip(bulk_flags(lines)) {
            if b {
                l.ms = None;
            }
        }
        return (out, copies);
    };
    let segment = |a: usize| -> &[Line] {
        let end = starts.iter().copied().find(|b| *b > a).unwrap_or(lines.len());
        &lines[a + 1..end]
    };
    let mut out: Vec<Line> = segment(last).to_vec();
    let out_bulk = bulk_flags(&out);
    for (l, b) in out.iter_mut().zip(&out_bulk) {
        if *b {
            l.ms = None;
        }
    }
    let same_game = |a: usize| {
        let (x, y) = (segment(a), segment(last));
        let n = x.len().min(y.len()).min(12);
        n > 0 && x[..n].iter().zip(&y[..n]).all(|(p, q)| p.text == q.text)
    };
    for &earlier in starts.iter().filter(|a| **a < last && same_game(**a)) {
        let seg = segment(earlier);
        let bulk = bulk_flags(seg);
        let mut j = 0;
        for o in out.iter_mut() {
            let window_end = seg.len().min(j + 80);
            if let Some(off) = seg[j..window_end].iter().position(|l| l.text == o.text) {
                let cand = &seg[j + off];
                if !bulk[j + off] {
                    if let Some(t) = cand.ms {
                        o.ms = Some(o.ms.map_or(t, |x| x.min(t)));
                    }
                }
                j += off + 1;
            }
        }
    }
    (out, copies)
}

/// Markers the log carries between actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Marker {
    GameViewStart,
    Saving,
    /// `***** LOADED FILE: save-Undo-003.dat *****`
    LoadedUndo(u32),
    LoadedContinue,
    /// `IntentManager calling Proceed: [SpiritPhaseActivation: SpiritWithGrowth=Volcano, GrowthOptionController=Gain 2 Energy, …]`
    GrowthOption { spirit: String, option: String, undo: bool },
    /// `UsePowerActivation … PhaseType=FastPower`: the phase the power is used in.
    PowerPhase { power: String, phase: Phase },
    /// A decision the player made among cards (`SelectCardActivation: Reason=GainPower, Selected=…`).
    CardChoice { reason: String, selected: String },
    /// `SelectGrowthGroupActivation: Selected=<SpiritId> Growth Group 2`
    GrowthGroup { spirit: String, group: u8 },
    /// `Building Adjacency Matrix: [C, E] / []`: the boards in play, as the
    /// short codes land ids use.
    Boards(Vec<String>),
}

/// What one line is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Parsed {
    Event(Event),
    /// One line that records several facts (several cards moving piles).
    Many(Vec<Event>),
    Marker(Marker),
    /// A `DidPerformAction` with a known shape we do not type (counted).
    Skipped(String),
    Noise,
}

pub fn parse(text: &str) -> Parsed {
    let t = text.trim();
    if t == "GameView Start" {
        return Parsed::Marker(Marker::GameViewStart);
    }
    if t.starts_with("***** SAVING") {
        return Parsed::Marker(Marker::Saving);
    }
    if let Some(rest) = t.strip_prefix("Building Adjacency Matrix: [") {
        let names = rest.split(']').next().unwrap_or("");
        let boards: Vec<String> = names.split(',').map(|n| LandId::board_code(n.trim())).filter(|n| !n.is_empty()).collect();
        return Parsed::Marker(Marker::Boards(boards));
    }
    if let Some(rest) = t.strip_prefix("***** LOADED FILE: save-Undo-") {
        let n: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        return Parsed::Marker(Marker::LoadedUndo(n.parse().unwrap_or(0)));
    }
    if t.starts_with("***** LOADED CONTINUE") {
        return Parsed::Marker(Marker::LoadedContinue);
    }
    if let Some(body) = t.strip_prefix("DidPerformAction: [") {
        if let Some(many) = card_pile_moves(body) {
            return many;
        }
        return match action(body) {
            Some(e) => Parsed::Event(e),
            None => Parsed::Skipped(body.split(':').next().unwrap_or("").to_string()),
        };
    }
    if t.contains("calling Proceed:") || t.contains("calling exclusive Proceed:") {
        if let Some(m) = activation(t) {
            return Parsed::Marker(m);
        }
    }
    Parsed::Noise
}

fn between<'a>(s: &'a str, start: &str, ends: &[&str]) -> Option<&'a str> {
    let i = s.find(start)? + start.len();
    let rest = &s[i..];
    let j = ends.iter().filter_map(|e| rest.find(e)).min().unwrap_or(rest.len());
    Some(rest[..j].trim())
}

fn land(s: &str) -> Option<LandId> {
    let t = s.trim();
    if t.is_empty() || t == "Out of Game" || t == "OutOfGame" || t == "Blight" || t.ends_with("OutOfGame") || t.contains("Forgotten Powers") {
        return None;
    }
    t.parse().ok()
}

fn number(s: &str) -> i64 {
    s.trim().trim_end_matches(|c: char| !c.is_ascii_digit() && c != '-').parse().unwrap_or(0)
}

/// `[Town2 ( Out of Game => JungleD2), Explorer5 ( Jungle B4 => OutOfGame)]`
fn piece_moves(s: &str) -> Vec<PieceMove> {
    let inner = s.trim().trim_start_matches('[').trim_end_matches(']');
    inner
        .split("), ")
        .filter_map(|part| {
            let (piece, rest) = part.split_once(" (")?;
            let (from, to) = rest.split_once("=>")?;
            let piece = piece.trim().to_string();
            Some(PieceMove { kind: PieceKind::parse(&piece), from: land(from), to: land(to.trim_end_matches(')')), piece })
        })
        .collect()
}

/// `(A, From, To), (B, From, To)` card movement triples.
fn card_moves(s: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    let mut rest = s.trim();
    while let Some(i) = rest.find('(') {
        let Some(j) = rest[i..].find(')') else { break };
        let parts: Vec<&str> = rest[i + 1..i + j].rsplitn(3, ", ").collect();
        if parts.len() == 3 {
            out.push((parts[2].trim().to_string(), parts[1].trim().to_string(), parts[0].trim().to_string()));
        }
        rest = &rest[i + j + 1..];
    }
    out
}

fn spirit_of_energy(s: &str, key: &str) -> Option<(String, i64)> {
    let sp = between(s, key, &[" Amount:", ","])?.to_string();
    let amount = number(between(s, "Amount:", &[",", " "])?);
    Some((sp, amount))
}

fn action(body: &str) -> Option<Event> {
    let name = body.split(':').next()?.trim();
    Some(match name {
        "ChangeStateAction" => Event::PhaseChanged {
            from: between(body, "From=", &[","]).filter(|f| *f != "null").map(Phase::parse),
            to: Phase::parse(between(body, "To=", &[","])?),
        },
        "ResolveInvaderCardAction" => Event::InvaderCardResolved {
            step: InvaderStep::parse(between(body, "InvaderActionType: ", &[","])?)?,
            card: between(body, "InvaderCard: ", &["]", ","])?.to_string(),
        },
        "FlipCardAction" => {
            let card = between(body, "CardBeingFlipped=", &["]", ","])?.to_string();
            if body.contains("Source=ResolveInvaderCardAction") {
                Event::InvaderCardRevealed { card }
            } else {
                return None;
            }
        }
        "AdvanceInvaderCardsAction" => Event::SlotsAdvanced { moves: card_moves(between(body, "CardMovements=", &["]"])?) },
        "RavageAction" => Event::Ravaged { land: land(between(body, "LandTarget: ", &[","])?)? },
        "AddIslandPiecesAction" => Event::PiecesAdded { moves: piece_moves(between(body, "PieceMovements=", &[])?) },
        "MovePiecesAction" => Event::PiecesMoved { moves: piece_moves(between(body, "PieceMovements=", &[])?) },
        "AddTokensAction" => {
            let moves = piece_moves(between(body, "PieceMovements=", &[])?);
            let first = moves.first()?;
            Event::TokensAdded { land: first.to.clone(), kind: TokenKind::parse(&first.piece), count: moves.len() as u32 }
        }
        "DestroyPiecesAction" => {
            let list = between(body, "PiecesBeingDestroyed=", &["]"])?;
            let source = between(body, "SourceController=[", &[":"]).unwrap_or("").to_string();
            Event::PiecesDestroyed { pieces: list.split(',').map(|p| (p.trim().to_string(), None)).filter(|(p, _)| !p.is_empty()).collect(), source }
        }
        "DealDamageAction" => {
            let amount = number(between(body, "DamageAmount=", &["]", ","])?) as u32;
            let source = between(body, "DamageSources=[", &["]"]).unwrap_or("").to_string();
            let targets = between(body, "DamageTargets=[", &["]"]).unwrap_or("");
            let land = targets.split('/').find_map(|t| t.rsplit_once('-').and_then(|(_, l)| (!l.is_empty()).then(|| l.parse::<LandId>().ok()).flatten()));
            let distribution = between(body, "DamageDistribution=", &["]"])
                .map(|d| d.split(", ").filter_map(|p| p.split_once(" => ").map(|(k, v)| (k.trim().to_string(), number(v) as u32))).collect())
                .unwrap_or_default();
            Event::Damage { source, land, amount, distribution }
        }
        "DefendAction" => Event::Defend {
            lands: between(body, "IslandLocationsToDefend=[", &["]"])?.split(',').filter_map(land).collect(),
            amount: number(between(body, "Amount=", &["]", ","])?) as u32,
            source: between(body, "SourceModel=", &[", IslandLocationsToDefend"]).unwrap_or("").to_string(),
        },
        "AddBlightAction" => Event::BlightAdded {
            land: between(body, "Destination: ", &[","]).and_then(land),
            amount: number(between(body, "AmountOfBlight: ", &["]", ","])?) as u32,
            cascade: between(body, "IsFromCascade: ", &[","]).is_some_and(|v| v == "True"),
        },
        "RemoveBlightAction" => {
            let moves = piece_moves(between(body, "PieceMovements=", &[])?);
            Event::BlightRemoved { land: moves.first().and_then(|m| m.from.clone()), amount: moves.len() as u32 }
        }
        "AdjustFearGeneratedAction" => Event::Fear {
            amount: number(between(body, "AmountOfFearBeingGenerated=", &[","])?) as i32,
            land: between(body, "IslandLocationGeneratingFear=", &["]", ","]).and_then(land),
            source: between(body, "Source=[", &[":"]).unwrap_or("").to_string(),
        },
        "MoveCardAction" => {
            let card = between(body, "CardBeingMoved=", &[","])?.to_string();
            let to = between(body, "To=", &[","]).unwrap_or("");
            let from = between(body, "From=", &[","]).unwrap_or("");
            if to == "Fear Earned" {
                Event::FearCardEarned
            } else if card.starts_with("TerrorLevel") && to == "Out of Game" {
                Event::TerrorLevel { level: if card.ends_with("III") { 3 } else { 2 } }
            } else {
                return card_pile_move(&card, from, to);
            }
        }
        "MoveCardsAction" | "DiscardCardsAction" | "RevealCardsAction" => return None,
        "ResolveFearCardAction" => Event::FearCardResolved { card: between(body, "FearCard: ", &["]", ","])?.to_string() },
        "ResolveEventCardAction" => Event::EventResolved { card: between(body, "EventCard: ", &["]", ","])?.to_string() },
        "UsePowerAction" => {
            let power = between(body, "PowerToUse:", &[","])?.to_string();
            let target = between(body, "PowerTarget: ", &["]", ","]).and_then(land);
            Event::PowerUsed { spirit: None, power, target, speed: None }
        }
        "PlayPowerCardAction" => Event::CardPlayed { spirit: None, card: between(body, "PlayPowerCard-", &["-"])?.to_string() },
        "UnplayPowerCardAction" => Event::CardUnplayed { spirit: None, card: between(body, "UnplayPowerCard-", &["-"])?.to_string() },
        "PayEnergyAction" => {
            let (spirit, amount) = spirit_of_energy(body, "SpiritPayingEnergy:")?;
            Event::EnergyPaid { spirit, amount }
        }
        "GainEnergyAction" => {
            let (spirit, amount) = spirit_of_energy(body, "SpiritGainingEnergy:")?;
            Event::EnergyGained { spirit, amount }
        }
        "GainElementsAction" => Event::ElementsGained { spirit: None, source: between(body, "Source: ", &["]"]).unwrap_or("").to_string() },
        "AdjustCardPlaysAction" => Event::PlaysAdjusted { source: between(body, "Source: [", &[":"]).unwrap_or("").to_string() },
        "MovePresenceAction" => {
            let moves = piece_moves(between(body, "PieceMovements=", &[])?);
            let m = moves.first()?;
            let raw_from = between(body, "( ", &[" =>"]).unwrap_or("");
            let spirit = m.piece.split("Presence").next().unwrap_or("").split("Incarna").next().unwrap_or("").to_string();
            let node = (m.from.is_none() && !raw_from.contains("Forgotten") && !raw_from.ends_with("OutOfGame") && !raw_from.is_empty()).then(|| raw_from.to_string());
            let destroyed = m.to.is_none() && between(body, "=> ", &[")"]).is_some_and(|t| t.ends_with("OutOfGame"));
            Event::PresenceMoved { spirit, node, from: m.from.clone(), to: m.to.clone(), destroyed }
        }
        "ForgetPowerCardAction" => Event::CardForgotten { spirit: None, card: between(body, "CardBeingMoved=", &[","])?.to_string() },
        "AddNewCardAction" => return None,
        "GameOverAction" => Event::GameOver { ending: Ending::parse(between(body, "EndingResult=", &["]", ","])?) },
        "TimePassesAction" => Event::TimePassed,
        "AddStatusEffectAction" => Event::StatusAdded { text: between(body, "StatusEffect=", &[", NumberOfUses", "]"]).unwrap_or("").chars().take(120).collect() },
        "CompleteSetupAction" => Event::Setup { text: "setup complete".into() },
        _ => return None,
    })
}

/// `MoveCardsAction` / `DiscardCardsAction` lines carry several card moves;
/// each one that touches a spirit's piles becomes its own event.
fn card_pile_moves(body: &str) -> Option<Parsed> {
    let name = body.split(':').next()?.trim();
    if !matches!(name, "MoveCardsAction" | "DiscardCardsAction" | "RevealCardsAction") {
        return None;
    }
    let moves = card_moves(between(body, "CardMovements=", &[])?);
    let events: Vec<Event> = moves.iter().filter_map(|(c, f, t)| card_pile_move(c, f, t)).collect();
    Some(match events.len() {
        0 => Parsed::Skipped(name.to_string()),
        1 => Parsed::Event(events.into_iter().next().unwrap()),
        _ => Parsed::Many(events),
    })
}

/// A card moving between a spirit's piles, decks and discards.
fn card_pile_move(card: &str, from: &str, to: &str) -> Option<Event> {
    let owner = |loc: &str| -> Option<String> { loc.contains("'s ").then(|| loc.split("'s ").next().unwrap_or("").to_string()) };
    if to.ends_with("'s Hand") {
        let deck = if from.contains("Deck") || from.contains("Reveal") { from.to_string() } else { from.to_string() };
        return Some(Event::CardGained { spirit: owner(to), card: card.to_string(), deck });
    }
    if to.ends_with("'s Discard") && from.ends_with("'s Play Area") {
        return Some(Event::CardsDiscarded { spirit: owner(from), cards: vec![card.to_string()] });
    }
    if to.ends_with("'s Forgotten Powers") {
        return Some(Event::CardForgotten { spirit: owner(to), card: card.to_string() });
    }
    None
}

fn activation(t: &str) -> Option<Marker> {
    if let Some(sel) = between(t, "SelectGrowthGroupActivation: Selected=", &[", InstanceNumber", ","]) {
        let (spirit, group) = sel.split_once(" Growth Group ")?;
        return Some(Marker::GrowthGroup { spirit: spirit.trim().to_string(), group: group.trim().parse().ok()? });
    }
    if let Some(sp) = between(t, "SpiritPhaseActivation: SpiritWithGrowth=", &[","]) {
        let option = between(t, "GrowthOptionController=", &[","])?.to_string();
        let undo = between(t, "IsUndoActivation=", &[","]).is_some_and(|v| v == "True") || t.contains("UndoSpiritPhaseActivation");
        return Some(Marker::GrowthOption { spirit: sp.to_string(), option, undo });
    }
    if let Some(p) = between(t, "UsePowerActivation: UsePower=", &[" "]) {
        let phase = between(t, "PhaseType=", &[",", "]"]).map(Phase::parse)?;
        return Some(Marker::PowerPhase { power: p.to_string(), phase });
    }
    if let Some(reason) = between(t, "SelectCardActivation: Reason=", &[","]) {
        let selected = between(t, "Selected=", &[", InstanceNumber", ","])?.to_string();
        return Some(Marker::CardChoice { reason: reason.to_string(), selected });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(s: &str) -> Event {
        match parse(s) {
            Parsed::Event(e) => e,
            other => panic!("not an event: {other:?} for {s}"),
        }
    }

    #[test]
    fn invader_steps_pieces_and_blight() {
        assert_eq!(ev("DidPerformAction: [ResolveInvaderCardAction: Source: Stage I Mountain, InvaderActionType: Build, InvaderCard: Mountain1]"), Event::InvaderCardResolved { step: InvaderStep::Build, card: "Mountain1".into() });
        match ev("DidPerformAction: [AddIslandPiecesAction: PieceMovements=[Explorer1 ( Out of Game => MountainA1)]") {
            Event::PiecesAdded { moves } => {
                assert_eq!(moves.len(), 1);
                assert_eq!(moves[0].kind, Some(PieceKind::Explorer));
                assert_eq!(moves[0].to, Some(LandId::new("A", 1)));
                assert_eq!(moves[0].from, None);
            }
            e => panic!("{e:?}"),
        }
        match ev("DidPerformAction: [MovePiecesAction: PieceMovements=[City5 ( Jungle C1 => OutOfGame), Town7 ( Jungle C1 => OutOfGame)]") {
            Event::PiecesMoved { moves } => {
                assert_eq!(moves.len(), 2);
                assert_eq!(moves[1].from, Some(LandId::new("C", 1)));
                assert_eq!(moves[1].to, None);
            }
            e => panic!("{e:?}"),
        }
        assert_eq!(
            ev("DidPerformAction: [AddBlightAction: Source: Widespread Clearcutting, Destination: Sands C2, WillCascade: False, IsFromCascade: False, AmountOfBlight: 1]"),
            Event::BlightAdded { land: Some(LandId::new("C", 2)), amount: 1, cascade: false }
        );
        assert_eq!(ev("DidPerformAction: [RavageAction: LandTarget: Sands A4, Source: Stage I Sands]"), Event::Ravaged { land: LandId::new("A", 4) });
    }

    #[test]
    fn powers_cards_energy_and_presence() {
        assert_eq!(ev("DidPerformAction: [UsePowerAction: Source: Tigers Hunting, PowerToUse:TigersHunting, PowerTarget: SandsC2]"), Event::PowerUsed { spirit: None, power: "TigersHunting".into(), target: Some(LandId::new("C", 2)), speed: None });
        assert_eq!(ev("DidPerformAction: [PlayPowerCardAction: Source: [PlayPowerCardActivation: InstanceNumber: 428 InstanceIdentifier: PlayPowerCard-TigersHunting-0-0-0- Source: [x]]"), Event::CardPlayed { spirit: None, card: "TigersHunting".into() });
        assert_eq!(ev("DidPerformAction: [PayEnergyAction: SpiritPayingEnergy:StarlightSeeksItsForm Amount:2, Source: [x]]"), Event::EnergyPaid { spirit: "StarlightSeeksItsForm".into(), amount: 2 });
        assert_eq!(
            ev("DidPerformAction: [MovePresenceAction: PieceMovements=[StarlightPresence4 ( StarlightSeeksItsFormCardPlays2 => JungleC4)]"),
            Event::PresenceMoved { spirit: "Starlight".into(), node: Some("StarlightSeeksItsFormCardPlays2".into()), from: None, to: Some(LandId::new("C", 4)), destroyed: false }
        );
        assert_eq!(
            ev("DidPerformAction: [MovePresenceAction: PieceMovements=[VolcanoPresence13 ( Volcano Looming High's Forgotten Powers => MountainA1)]"),
            Event::PresenceMoved { spirit: "Volcano".into(), node: None, from: None, to: Some(LandId::new("A", 1)), destroyed: false }
        );
        assert_eq!(ev("DidPerformAction: [DefendAction: SourceModel=Air Moves, Earth Endures, IslandLocationsToDefend=[Sands C2], Amount=5]"), Event::Defend { lands: vec![LandId::new("C", 2)], amount: 5, source: "Air Moves, Earth Endures".into() });
    }

    #[test]
    fn fear_terror_and_card_piles() {
        assert_eq!(ev("DidPerformAction: [MoveCardAction: CardBeingMoved=ImmigrationSlows, To=Fear Earned, From=Fear Deck, Offset=]"), Event::FearCardEarned);
        assert_eq!(ev("DidPerformAction: [MoveCardAction: CardBeingMoved=TerrorLevelII, To=Out of Game, From=Fear Deck, Offset=]"), Event::TerrorLevel { level: 2 });
        assert_eq!(
            ev("DidPerformAction: [AdjustFearGeneratedAction: AmountOfFearBeingGenerated=3,Source=[UsePowerAction: Source: Sea Monsters],AmountOfGeneratedFearBeforeAction=1,AmountOfGeneratedFearAfterAction=4,IslandLocationGeneratingFear=Jungle C1]"),
            Event::Fear { amount: 3, land: Some(LandId::new("C", 1)), source: "UsePowerAction".into() }
        );
        match parse("DidPerformAction: [DiscardCardsAction: CardMovements=(Sea Monsters, Starlight Seeks Its Form's Play Area, Starlight Seeks Its Form's Discard), (Sap the Strength of Multitudes, Starlight Seeks Its Form's Play Area, Starlight Seeks Its Form's Discard)]") {
            Parsed::Many(events) => {
                assert_eq!(events.len(), 2);
                assert!(matches!(&events[0], Event::CardsDiscarded { spirit: Some(s), cards } if s == "Starlight Seeks Its Form" && cards == &vec!["Sea Monsters".to_string()]));
            }
            e => panic!("{e:?}"),
        }
        match parse("DidPerformAction: [MoveCardsAction: CardMovements=(Unnerving Pall, Shroud of Silent Mist's Discard, Shroud of Silent Mist's Hand), (The Fog Closes In, Shroud of Silent Mist's Discard, Shroud of Silent Mist's Hand)]") {
            Parsed::Many(events) => assert!(events.iter().all(|e| matches!(e, Event::CardGained { deck, .. } if deck.contains("Discard"))), "a reclaim is several gains from the discard"),
            e => panic!("{e:?}"),
        }
        assert_eq!(ev("DidPerformAction: [GameOverAction: EndingResult=WinTerrorVictory]"), Event::GameOver { ending: Ending::WinTerrorVictory });
    }

    #[test]
    fn markers_and_stamps() {
        assert_eq!(parse("***** LOADED FILE: save-Undo-003.dat *****"), Parsed::Marker(Marker::LoadedUndo(3)));
        assert_eq!(parse("***** SAVING *****"), Parsed::Marker(Marker::Saving));
        let l = split_stamp("12:49:57.950  GameView Start");
        assert_eq!(l.ms, Some(((12 * 60 + 49) * 60 + 57) * 1000 + 950));
        assert_eq!(parse(&l.text), Parsed::Marker(Marker::GameViewStart));
        assert_eq!(
            parse("IntentManager calling Proceed: [SpiritPhaseActivation: SpiritWithGrowth=Volcano, GrowthOptionController=Gain 2 Energy, IsUndoActivation=False, ShouldUseActivationAutomatically=True]"),
            Parsed::Marker(Marker::GrowthOption { spirit: "Volcano".into(), option: "Gain 2 Energy".into(), undo: false })
        );
        assert_eq!(
            parse("IntentManager calling Proceed: [UsePowerActivation: UsePower=SteamVents Sources=[FastPowerStateController: State=[State: PhaseType=FastPower, StateType=FastPower], Steam Vents, Enabled=True]"),
            Parsed::Marker(Marker::PowerPhase { power: "SteamVents".into(), phase: Phase::FastPower })
        );
        assert!(matches!(parse("DidPerformAction: [ModifyPowersAction: PowersToModify=[A] ResetPower=True PowerSpeed=]"), Parsed::Skipped(s) if s == "ModifyPowersAction"));
    }
}
