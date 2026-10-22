//! What the app is asking the player right now, read from Unity's `Player.log` beside
//! the saves (the view's own debug log): the engine puts an intent to the player
//! (`IntendActivation: [...]`) and `IntentManager calling Proceed: [...]` records the
//! answer. Only those two line kinds are read, and of them the activation's name and
//! its public attributes: the power, the growth option, the reason, the selection.

use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// The latest intent and whether the player has answered it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Activity {
    pub asks: String,
    pub answered: bool,
}

impl Activity {
    pub fn words(&self) -> String {
        if self.answered {
            format!("The app's last step: {}.", self.asks)
        } else {
            format!("The app asks: {}.", self.asks)
        }
    }
}

/// The last `bytes` of a file, from its first whole line.
pub(super) fn tail_text(path: &Path, bytes: u64) -> Option<String> {
    let mut f = std::fs::File::open(path).ok()?;
    let len = f.metadata().ok()?.len();
    let start = len.saturating_sub(bytes);
    f.seek(SeekFrom::Start(start)).ok()?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).ok()?;
    let mut text = String::from_utf8_lossy(&buf).to_string();
    if start > 0 {
        if let Some(i) = text.find('\n') {
            text = text[i + 1..].to_string();
        }
    }
    Some(text)
}

/// From the lines in file order, stamps already removed.
pub(super) fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Option<Activity> {
    let mut intent: Option<String> = None;
    let mut answered = false;
    for l in lines {
        if let Some(rest) = l.strip_prefix("IntendActivation: [") {
            intent = Some(rest.to_string());
            answered = false;
        } else if let Some(rest) = l.strip_prefix("IntentManager calling Proceed: [") {
            // the answer carries the selection (Selected=...), so it replaces the intent's text
            intent = Some(rest.to_string());
            answered = true;
        }
    }
    intent.map(|a| Activity { asks: describe(&a), answered })
}

fn name(a: &str) -> &str {
    a.split(':').next().unwrap_or(a).trim()
}

/// `SelectGrowthGroup` -> `Select Growth Group`.
fn spaced(camel: &str) -> String {
    let mut out = String::new();
    let mut prev: Option<char> = None;
    for c in camel.chars() {
        if c.is_ascii_uppercase() && prev.is_some_and(|p| p.is_ascii_lowercase() || p.is_ascii_digit()) {
            out.push(' ');
        }
        out.push(c);
        prev = Some(c);
    }
    out
}

/// The attribute's value: up to the comma, the closing bracket or the next ` Key=`.
fn attr<'a>(a: &'a str, key: &str) -> Option<&'a str> {
    let (_, rest) = a.split_once(key)?;
    let end = rest.find(|c| c == ',' || c == ']').unwrap_or(rest.len());
    let v = rest[..end].trim();
    let v = match v.find(" Sources=") {
        Some(i) => &v[..i],
        None => v,
    };
    let v = match v.find(" InstanceNumber") {
        Some(i) => &v[..i],
        None => v,
    };
    if v.is_empty() { None } else { Some(v.trim()) }
}

/// `UsePowerActivation: UsePower=SuddenAmbush Sources=[FastPowerStateController...` ->
/// `Use Power: Sudden Ambush (Fast)`.
fn describe(a: &str) -> String {
    let n = spaced(name(a).trim_end_matches("Activation"));
    let speed = if a.contains("FastPowerStateController") {
        " (Fast)"
    } else if a.contains("SlowPowerStateController") {
        " (Slow)"
    } else {
        ""
    };
    if let Some(p) = attr(a, "UsePower=") {
        return format!("{n}: {}{speed}", spaced(p));
    }
    if let Some(g) = attr(a, "GrowthOptionController=") {
        let who = attr(a, "SpiritWithGrowth=").map(|s| format!(" ({s})")).unwrap_or_default();
        return format!("{n}: {g}{who}");
    }
    for key in ["Selected=", "Reason=", "Title="] {
        if let Some(v) = attr(a, key) {
            let v = if key == "Reason=" { spaced(v) } else { v.to_string() };
            return format!("{n}: {v}");
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_intent_waits_until_its_proceed() {
        let lines = [
            "IntendActivation: [SelectGrowthGroupActivation: InstanceNumber: 1 InstanceIdentifier: GrowthGroup-Spirit:Thunderspeaker-1-2-3-0 Source: [GrowStateController: State=x]] allowProceed: True",
        ];
        let a = parse(lines.iter().copied()).unwrap();
        assert_eq!(a, Activity { asks: "Select Growth Group".into(), answered: false });
        assert_eq!(a.words(), "The app asks: Select Growth Group.");
        let lines = [
            lines[0],
            "IntentManager calling Proceed: [SelectGrowthGroupActivation: Selected=Thunderspeaker Growth Group 2, InstanceNumber: 1 InstanceIdentifier: GrowthGroup-Spirit:Thunderspeaker-1-2-3-0 Source: [GrowStateController: State=x]]",
        ];
        let a = parse(lines.iter().copied()).unwrap();
        assert_eq!(a, Activity { asks: "Select Growth Group: Thunderspeaker Growth Group 2".into(), answered: true });
        assert_eq!(a.words(), "The app's last step: Select Growth Group: Thunderspeaker Growth Group 2.");
    }

    #[test]
    fn powers_growth_and_reasons_read_as_words() {
        let d = |s: &str| describe(s);
        assert_eq!(d("UsePowerActivation: UsePower=SuddenAmbush Sources=[FastPowerStateController: State=[State: 1]] InstanceNumber: 40"), "Use Power: Sudden Ambush (Fast)");
        assert_eq!(d("SpiritPhaseActivation: SpiritWithGrowth=Thunderspeaker, GrowthOptionController=Add a Presence to Jungle or Wetland, IsUndoActivation=False"), "Spirit Phase: Add a Presence to Jungle or Wetland (Thunderspeaker)");
        assert_eq!(d("SelectLocationActivation: Reason=RavageLand, InstanceNumber: 7"), "Select Location: Ravage Land");
        assert_eq!(d("SelectPieceActivation: Reason=DestroyPiece, InstanceNumber: 9"), "Select Piece: Destroy Piece");
        assert_eq!(d("PlayPowerCardActivation: InstanceNumber: 3 InstanceIdentifier: x"), "Play Power Card");
        assert_eq!(d("ContinueActivation: ActivationString=Continue-Finished-51-- Title=A decision, InstanceNumber: 2"), "Continue: A decision");
    }

    #[test]
    fn a_proceed_of_another_activation_is_a_step_of_its_own() {
        let lines = [
            "IntendActivation: [PlayPowerCardActivation: InstanceNumber: 3 InstanceIdentifier: PPC-1 Source: x]",
            "IntentManager calling Proceed: [StartOfGameActivation: InstanceNumber: 0 InstanceIdentifier: StartOfGame- Source: GameController]",
        ];
        let a = parse(lines.iter().copied()).unwrap();
        assert_eq!(a.asks, "Start Of Game");
        assert!(a.answered);
        assert!(parse(["nothing here"].into_iter()).is_none());
    }
}
