//! Spirit Island colors for the terminal. Lands by terrain (Mountain grey,
//! Sands yellow, Jungle green, Wetland blue, Ocean cyan), Fast orange and
//! Slow blue-violet, Blight purple, Fear magenta, Dahan orange-brown, Rei's
//! classes from green (above) to red (sacrificed, untouched), scores by sign,
//! won green and lost red, headings bold. Off with `--no-color`, `NO_COLOR`,
//! or when stdout is not a terminal; forced on with `--color` or `SI_COLOR=1`.

use std::collections::BTreeMap;
use std::io::IsTerminal;
use std::sync::{Mutex, OnceLock};

use regex::Regex;
use si_defs::Definitions;
use si_model::island::Terrain;
use si_model::power::Speed;

struct Palette {
    terrain: BTreeMap<String, Terrain>,
    speed: BTreeMap<String, Speed>,
    /// Innate codes and ids: painted underlined, so an innate never reads like a card.
    innate: std::collections::BTreeSet<String>,
}

static ENABLED: OnceLock<bool> = OnceLock::new();
static PALETTE: Mutex<Option<Palette>> = Mutex::new(None);

pub fn init(args: &[String]) {
    let forced = args.iter().any(|a| a == "--color") || std::env::var("SI_COLOR").is_ok_and(|v| v == "1");
    let on = forced || (!args.iter().any(|a| a == "--no-color") && std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal());
    let _ = ENABLED.set(on);
}

pub fn enabled() -> bool {
    *ENABLED.get().unwrap_or(&false)
}

/// Teach the painter the lands and the cards: land ids to terrain, card and
/// innate short codes and ids to speed.
pub fn register(defs: &Definitions) {
    let mut terrain = BTreeMap::new();
    for b in defs.boards.values() {
        for l in &b.lands {
            terrain.insert(l.id.to_string(), l.terrain);
        }
    }
    let mut speed = BTreeMap::new();
    let mut codes = si_record::notation::Codes::new(Some(defs), false);
    for c in defs.cards.values() {
        speed.insert(codes.code(&c.id), c.speed);
        speed.insert(c.id.clone(), c.speed);
    }
    let mut innate = std::collections::BTreeSet::new();
    for i in defs.spirits.values().flat_map(|s| s.innates.iter()) {
        speed.insert(codes.code(&i.id), i.speed);
        speed.insert(i.id.clone(), i.speed);
        innate.insert(codes.code(&i.id));
        innate.insert(i.id.clone());
    }
    *PALETTE.lock().unwrap() = Some(Palette { terrain, speed, innate });
}

fn c256(n: u8, s: &str) -> String {
    format!("\x1b[38;5;{n}m{s}\x1b[0m")
}

fn bold(s: &str) -> String {
    format!("\x1b[1m{s}\x1b[0m")
}

fn c256u(n: u8, s: &str) -> String {
    format!("\x1b[4;38;5;{n}m{s}\x1b[0m")
}

/// Element abbreviations as the hindsight prints them: `Su1 Mo1 Fi2 Ai1 Wa1 Ea1 Pl1 An2`.
fn element_color(abbr: &str) -> Option<u8> {
    Some(match abbr {
        "Su" => 220,
        "Mo" => 250,
        "Fi" => 196,
        "Ai" => 159,
        "Wa" => 33,
        "Ea" => 130,
        "Pl" => 34,
        "An" => 131,
        _ => return None,
    })
}

const MOUNTAIN: u8 = 245;
const SANDS: u8 = 220;
const JUNGLE: u8 = 34;
const WETLAND: u8 = 39;
const OCEAN: u8 = 45;
const FAST: u8 = 208;
const SLOW: u8 = 99;
const BLIGHT: u8 = 129;
const FEAR: u8 = 201;
const DAHAN: u8 = 172;
const GOOD: u8 = 70;
const BAD: u8 = 160;
const WARN: u8 = 178;
const DIM: u8 = 244;

fn terrain_color(t: Terrain) -> u8 {
    match t {
        Terrain::Mountain => MOUNTAIN,
        Terrain::Sands => SANDS,
        Terrain::Jungle => JUNGLE,
        Terrain::Wetland => WETLAND,
        Terrain::Ocean => OCEAN,
    }
}

/// Paint one line of output.
pub fn paint(line: &str) -> String {
    if !enabled() {
        return line.to_string();
    }
    static LAND: OnceLock<Regex> = OnceLock::new();
    static WORD: OnceLock<Regex> = OnceLock::new();
    static SCORE: OnceLock<Regex> = OnceLock::new();
    let land = LAND.get_or_init(|| Regex::new(r"\b(?:NW|NE|SW|SE|[A-HW])\d{1,2}\b").unwrap());
    let word = WORD.get_or_init(|| Regex::new(r"\b[A-Za-z][A-Za-z0-9']*\b|Bl\+?\d*~?|Fr\+\d+|†").unwrap());
    static ELEMENT: OnceLock<Regex> = OnceLock::new();
    let element = ELEMENT.get_or_init(|| Regex::new(r"\b(Su|Mo|Fi|Ai|Wa|Ea|Pl|An)(\d)\b").unwrap());
    let score = SCORE.get_or_init(|| Regex::new(r"[+-]\d+\.\d\d").unwrap());
    let palette = PALETTE.lock().unwrap();
    let trimmed = line.trim_start();
    // headings: all-caps words at the line start
    if trimmed.len() > 4 && trimmed.split(|c: char| c == ' ' || c == '—' || c == '-').next().is_some_and(|w| w.len() >= 4 && w.chars().all(|c| c.is_ascii_uppercase() || c == '_')) && !trimmed.starts_with("TP") {
        return bold(line);
    }
    let mut out = String::new();
    let mut last = 0;
    // pass 1: lands
    let mut pieces: Vec<(usize, usize, String)> = Vec::new();
    for m in land.find_iter(line) {
        let color = palette.as_ref().and_then(|p| p.terrain.get(m.as_str())).map(|t| terrain_color(*t));
        if let Some(c) = color {
            pieces.push((m.start(), m.end(), c256(c, m.as_str())));
        }
    }
    for m in element.captures_iter(line) {
        let whole = m.get(0).unwrap();
        if let Some(c) = element_color(&m[1]) {
            pieces.push((whole.start(), whole.end(), c256(c, whole.as_str())));
        }
    }
    for m in word.find_iter(line) {
        if pieces.iter().any(|(s, e, _)| m.start() >= *s && m.start() < *e) {
            continue;
        }
        let w = m.as_str();
        let painted = match w {
            "above" | "won" | "prevented" | "cleared" | "Sound" | "matched" => Some(c256(GOOD, w)),
            "on" | "defended" => Some(c256(37, w)),
            "gambled" | "Gambled" | "open" => Some(c256(WARN, w)),
            "failed" | "late" => Some(c256(FAST, w)),
            "sacrificed" | "untouched" | "lost" | "blighted" | "Mistake" | "mismatches" => Some(c256(BAD, w)),
            "Fast" | "F" => Some(c256(FAST, w)),
            "Slow" | "S" => Some(c256(SLOW, w)),
            "Blight" | "blight" => Some(c256(BLIGHT, w)),
            "Fear" | "fear" => Some(c256(FEAR, w)),
            "Dahan" | "dahan" => Some(c256(DAHAN, w)),
            "Reclaim" | "reclaim" | "Rc" => Some(c256(WARN, w)),
            "†" => Some(c256(BAD, w)),
            _ if w.starts_with("Bl") && (w.len() == 2 || w[2..].starts_with(['+', '~', '=']) || w[2..].chars().all(|c| c.is_ascii_digit())) => Some(c256(BLIGHT, w)),
            _ if w.starts_with("Fr+") => Some(c256(FEAR, w)),
            _ => palette.as_ref().and_then(|p| p.speed.get(w).map(|s| (s, p.innate.contains(w)))).map(|(s, inn)| match (s, inn) {
                (Speed::Fast, false) => c256(FAST, w),
                (Speed::Slow, false) => c256(SLOW, w),
                (Speed::Fast, true) => c256u(FAST, w),
                (Speed::Slow, true) => c256u(SLOW, w),
            }),
        };
        if let Some(p) = painted {
            pieces.push((m.start(), m.end(), p));
        }
    }
    for m in score.find_iter(line) {
        if pieces.iter().any(|(s, e, _)| m.start() >= *s && m.start() < *e) {
            continue;
        }
        let v: f64 = m.as_str().parse().unwrap_or(0.0);
        pieces.push((m.start(), m.end(), c256(if v > 0.0 { GOOD } else if v < 0.0 { BAD } else { DIM }, m.as_str())));
    }
    pieces.sort_by_key(|(s, _, _)| *s);
    for (s, e, p) in pieces {
        if s < last {
            continue;
        }
        out.push_str(&line[last..s]);
        out.push_str(&p);
        last = e;
    }
    out.push_str(&line[last..]);
    out
}

/// Print a painted line.
pub fn emit(line: &str) {
    println!("{}", paint(line));
}
