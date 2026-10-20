//! `log.txt` → the player's decisions (growth, cards kept, powers and their
//! targets, effect and land choices, presence placed, damage assigned).
//!
//! The raw log names hidden cards (each Fear card when earned), so this
//! module only extracts whitelisted decision lines, and `projection`
//! decides what of each crosses the firewall.
//!
//! Input is a capture log: lines prefixed `HH:MM:SS.mmm  ` (the capture
//! tool's or the live recorder's wall clock). Undo: an undo checkpoint
//! `save-Undo-k.dat` is written the moment a decision is made (Test 1: Undo-000
//! at 11:39:01.121, 3 ms after "select Bats"); `LOADED FILE: save-Undo-k.dat`
//! reverts to just before it. Decisions from the matching checkpoint's write
//! time up to the load are dropped.

use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum RawDecision {
    GrowthGroup { choice: String },
    GrowthOption { option: String },
    /// `reason` = GainPower, Forget, …; `choices` are engine card ids.
    CardChoice { reason: String, selected: String, choices: Vec<String> },
    PlayCard { card: String },
    UsePower { power: String, target: Option<String> },
    Effect { option: String, source: String, land: Option<String> },
    Location { reason: String, land: String, source: String },
    Presence { from: String, to: String },
    Damage { sources: String, targets: String, amount: i64 },
    Forget { card: String },
    /// The turn's Event, face up (public). The save at the Event step is
    /// written before the draw, so the log is the first place it shows (U7).
    EventRevealed { card: String },
    /// Power Cards drawn for a gain, shown to the player (U12). Titles, since
    /// the log names them by title; some titles contain commas.
    PowersRevealed { deck: String, titles: Vec<String> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimedDecision {
    pub ms: u64,
    pub turn: u32,
    pub step: String,
    pub decision: RawDecision,
}

fn stamp(line: &str) -> Option<(u64, &str)> {
    let (t, rest) = line.split_once("  ")?;
    let (hms, ms) = t.split_once('.')?;
    let mut parts = hms.split(':').map(|p| p.parse::<u64>().ok());
    let (h, m, s) = (parts.next()??, parts.next()??, parts.next()??);
    Some((((h * 60 + m) * 60 + s) * 1000 + ms.parse::<u64>().ok()?, rest.trim_start()))
}

fn between<'a>(s: &'a str, start: &str, end: &[&str]) -> Option<&'a str> {
    let i = s.find(start)? + start.len();
    let rest = &s[i..];
    let j = end.iter().filter_map(|e| rest.find(e)).min().unwrap_or(rest.len());
    Some(rest[..j].trim())
}

/// "Jungle C4" or "JungleC4" → "JungleC4".
fn land_id(s: &str) -> String {
    s.replace(' ', "")
}

fn is_land(s: &str) -> bool {
    let t = s.replace(' ', "");
    ["Jungle", "Mountain", "Sands", "Wetland", "Ocean"].iter().any(|p| t.starts_with(p))
        && t.chars().last().is_some_and(|c| c.is_ascii_digit())
}

fn parse_decision(text: &str) -> Option<RawDecision> {
    if let Some(choice) = between(text, "SelectGrowthGroupActivation: Selected=", &[", InstanceNumber"]) {
        return Some(RawDecision::GrowthGroup { choice: choice.to_string() });
    }
    if text.contains("Proceed: [SpiritPhaseActivation:") {
        let option = between(text, "GrowthOptionController=", &[","])?;
        return Some(RawDecision::GrowthOption { option: option.split_whitespace().collect::<Vec<_>>().join(" ") });
    }
    if text.contains("SelectCardActivation:") {
        let choices = between(text, "CardChoices=", &[", SelectedCard="])?.split(", ").map(String::from).collect();
        let selected = between(text, "SelectedCard=", &[", SelectCardReason="])?.to_string();
        let reason = between(text, "SelectCardReason=", &[","])?.to_string();
        return Some(RawDecision::CardChoice { reason, selected, choices });
    }
    if text.starts_with("DidPerformAction: [PlayPowerCardAction:") {
        let card = between(text, "PlayPowerCard-", &["-"])?;
        return Some(RawDecision::PlayCard { card: card.to_string() });
    }
    if text.starts_with("DidPerformAction: [UsePowerAction:") {
        let power = between(text, "PowerToUse:", &[","])?.to_string();
        let target = between(text, "PowerTarget:", &["]"]).filter(|t| !t.is_empty()).map(land_id);
        return Some(RawDecision::UsePower { power, target });
    }
    if text.contains("SelectEffectActivation: Selected=") {
        let option = between(text, "Selected=", &[", InstanceNumber"])?.to_string();
        let source = between(text, " Source: ", &[" IsOptional"]).unwrap_or("").to_string();
        let land = between(text, "InstanceIdentifier: ", &[" "])
            .and_then(|id| id.split('-').find(|p| is_land(p)).map(String::from));
        return Some(RawDecision::Effect { option, source, land });
    }
    if text.contains("SelectLocationActivation: Reason=") {
        let reason = between(text, "Reason=", &[","])?.to_string();
        let land = between(text, "Selected=", &[", InstanceNumber"])?.to_string();
        let source = between(text, " Source: ", &[" IsOptional"]).unwrap_or("").to_string();
        let source = if source.starts_with('[') { String::new() } else { source };
        return Some(RawDecision::Location { reason, land: land_id(&land), source });
    }
    if text.starts_with("DidPerformAction: [MovePresenceAction:") {
        let mv = between(text, "( ", &[")"])?;
        let (from, to) = mv.split_once(" => ")?;
        return Some(RawDecision::Presence { from: land_id(from), to: land_id(to) });
    }
    if text.contains("Proceed: DealDamageActivation:") {
        return Some(RawDecision::Damage {
            sources: between(text, "DamageSources=[", &["]"])?.to_string(),
            targets: between(text, "DamageTargets=[", &["]"])?.to_string(),
            amount: between(text, "DamageAmount=", &["]"])?.parse().ok()?,
        });
    }
    if text.starts_with("DidPerformAction: [ForgetPowerCardAction:") && text.contains("From=") && text.contains("Hand") {
        return Some(RawDecision::Forget { card: between(text, "CardBeingMoved=", &[","])?.to_string() });
    }
    if text.starts_with("DidPerformAction: [MoveCardAction:") && text.contains("To=Play Area, From=Event Deck") {
        return Some(RawDecision::EventRevealed { card: between(text, "CardBeingMoved=", &[","])?.to_string() });
    }
    if text.starts_with("DidPerformAction: [RevealCardsAction:") && text.contains("Power Deck,") {
        return powers_revealed(text);
    }
    None
}

/// `RevealCardsAction: CardMovements=(Title, Minor Power Deck, X's Reveal
/// Location), (…)`. Titles can contain ", " ("Vanish Softly Away, Forgotten by
/// All"), so each group is split from the right: destination, then source.
fn powers_revealed(text: &str) -> Option<RawDecision> {
    let body = text.split_once("CardMovements=(")?.1;
    let body = body.trim_end().trim_end_matches(']').trim_end_matches(')');
    let mut deck = String::new();
    let mut titles = Vec::new();
    for group in body.split("), (") {
        let mut parts = group.rsplitn(3, ", ");
        let (_to, from, title) = (parts.next()?, parts.next()?, parts.next()?);
        if let Some(d) = from.strip_suffix(" Power Deck") {
            deck = d.to_string();
            titles.push(title.to_string());
        }
    }
    (!titles.is_empty()).then_some(RawDecision::PowersRevealed { deck, titles })
}

/// Decisions from a capture folder (`log-stream.txt` + `events.txt` or `saves/`).
pub fn capture_decisions(capture: &Path) -> Vec<TimedDecision> {
    let Ok(log) = std::fs::read_to_string(capture.join("log-stream.txt")) else { return Vec::new() };
    parse_log(&log, &undo_write_times(capture))
}

/// When each undo checkpoint was written. `events.txt` has the exact write
/// (`NEW  save-Undo-000.dat`); capture file names are the copy time, a few
/// hundred ms later, so they are moved back by 400 ms.
pub fn undo_write_times(capture: &Path) -> BTreeMap<String, Vec<u64>> {
    let mut out: BTreeMap<String, Vec<u64>> = BTreeMap::new();
    if let Ok(events) = std::fs::read_to_string(capture.join("events.txt")) {
        for line in events.lines() {
            let Some((ms, rest)) = stamp(line) else { continue };
            if let Some(name) = rest.strip_prefix("NEW").map(str::trim).filter(|n| n.starts_with("save-Undo-")) {
                out.entry(name.to_string()).or_default().push(ms);
            }
        }
        if !out.is_empty() {
            return out;
        }
    }
    for e in std::fs::read_dir(capture.join("saves")).into_iter().flatten().flatten() {
        let f = e.file_name().to_string_lossy().to_string();
        let Some((t, name)) = f.split_once("__") else { continue };
        let Some((hms, ms)) = t.split_once('_') else { continue };
        let n = |r: std::ops::Range<usize>| hms.get(r).and_then(|x| x.parse::<u64>().ok());
        let (Some(h), Some(m), Some(sec), Ok(ms)) = (n(0..2), n(2..4), n(4..6), ms.parse::<u64>()) else { continue };
        let name = name.trim_end_matches(".gz").to_string();
        if name.starts_with("save-Undo-") {
            out.entry(name).or_default().push((((h * 60 + m) * 60 + sec) * 1000 + ms).saturating_sub(400));
        }
    }
    out
}

/// How long after a choice its undo checkpoint can be written. Test 1: 3 ms;
/// the 2026-10-19 Prussia game: 130 ms (playtest U19).
const CHECKPOINT_LAG_MS: u64 = 2000;

pub fn parse_log(log: &str, undo_writes: &BTreeMap<String, Vec<u64>>) -> Vec<TimedDecision> {
    let mut out: Vec<TimedDecision> = Vec::new();
    let mut turn = 1u32;
    let mut invaded = false;
    let mut step = "PlayerSetup".to_string();
    // (time, turn, step) at every step change, to restore them on an undo.
    let mut steps: Vec<(u64, u32, String)> = Vec::new();
    for line in log.lines() {
        let Some((ms, text)) = stamp(line) else { continue };
        if text.starts_with("GameView Start") {
            out.clear();
            steps.clear();
            turn = 1;
            invaded = false;
            step = "PlayerSetup".into();
            continue;
        }
        if let Some(file) = between(text, "LOADED FILE: ", &[" *****"]) {
            if let Some(&t) = undo_writes.get(file).and_then(|ts| ts.iter().filter(|t| **t <= ms).max()) {
                // The checkpoint is written just after the choice it undoes:
                // cut at the last choice made shortly before the write.
                let cut = out
                    .iter()
                    .rev()
                    .find(|d| d.ms <= t && t - d.ms <= CHECKPOINT_LAG_MS)
                    .map(|d| d.ms)
                    .unwrap_or_else(|| t.saturating_sub(50));
                out.retain(|d| d.ms < cut);
                steps.retain(|(at, _, _)| *at < cut);
                if let Some((_, tn, st)) = steps.last() {
                    turn = *tn;
                    step = st.clone();
                }
            }
            continue;
        }
        // Turns count as the saves do: Time Passes into the Spirit Phase
        // starts the next one, once per Invader Phase played (an undo
        // checkpoint reload logs that transition again without one between).
        if text.starts_with("DidPerformAction: [ChangeStateAction: From=TimePasses/TimePasses, To=SpiritPhase/SpiritPhase") && invaded {
            turn += 1;
            invaded = false;
        }
        if let Some(to) = text.strip_prefix("DidPerformAction: [ChangeStateAction: ").and_then(|t| between(t, "To=", &["/"])) {
            step = to.to_string();
            if matches!(step.as_str(), "Ravage" | "Build" | "Explore") {
                invaded = true;
            }
            steps.push((ms, turn, step.clone()));
            continue;
        }
        // Take-backs inside the Spirit phase net out against what they undo.
        if text.starts_with("DidPerformAction: [UnplayPowerCardAction:") {
            if let Some(card) = between(text, "UnplayPowerCard-", &["-"]) {
                if let Some(i) = out.iter().rposition(|d| matches!(&d.decision, RawDecision::PlayCard { card: c } if c == card)) {
                    out.remove(i);
                }
            }
            continue;
        }
        if text.contains("Proceed: [UndoSpiritPhaseActivation:") {
            if let Some(opt) = between(text, "GrowthOptionController=", &[","]) {
                let opt = opt.split_whitespace().collect::<Vec<_>>().join(" ");
                if let Some(i) = out.iter().rposition(|d| matches!(&d.decision, RawDecision::GrowthOption { option } if *option == opt)) {
                    out.remove(i);
                }
            }
            continue;
        }
        if let Some(decision) = parse_decision(text) {
            if let RawDecision::Presence { from, to } = &decision {
                if is_land(from) && !is_land(to) {
                    if let Some(i) = out.iter().rposition(|d| matches!(&d.decision, RawDecision::Presence { from: f, to: t } if t == from && f == to)) {
                        out.remove(i);
                        continue;
                    }
                }
            }
            out.push(TimedDecision { ms, turn, step: step.clone(), decision });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_power_use_with_target() {
        let d = parse_decision("DidPerformAction: [UsePowerAction: Source: Bats Scout For Raids By Darkness, PowerToUse:BatsScoutForRaidsByDarkness, PowerTarget: MountainC3]");
        assert_eq!(d, Some(RawDecision::UsePower { power: "BatsScoutForRaidsByDarkness".into(), target: Some("MountainC3".into()) }));
    }

    #[test]
    fn undo_drops_the_undone_choice() {
        let log = "11:39:01.118  IntentManager calling Proceed: [SelectCardActivation: CardChoices=A, B, SelectedCard=B Card, SelectCardReason=GainPower, IsEnabled=True]\n\
                   11:39:13.889  ***** LOADED FILE: save-Undo-000.dat *****\n\
                   11:40:01.010  IntentManager calling Proceed: [SelectCardActivation: CardChoices=A, B, SelectedCard=A Card, SelectCardReason=GainPower, IsEnabled=True]\n";
        let undo = BTreeMap::from([("save-Undo-000.dat".to_string(), vec![(11 * 3600 + 39 * 60 + 1) * 1000 + 121])]);
        let d = parse_log(log, &undo);
        assert_eq!(d.len(), 1);
        assert!(matches!(&d[0].decision, RawDecision::CardChoice { selected, .. } if selected == "A Card"));
    }
}
