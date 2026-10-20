//! `si replay <log|capture> [--defs <dir>] [--json]`
//!
//! Replays a digital-edition log into a record and prints it: turns, spirits,
//! lands, coverage. Definitions come from `--defs` or `SI_DEFS_DIR` (the
//! directory `si-steam-dump defs` writes).

use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use si_defs::Definitions;
use si_record::{GameRecord, Replay, TouchKind};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first() else { return usage() };
    match cmd.as_str() {
        "replay" => replay(&args[1..]),
        "report" => report(&args[1..]),
        "corpus" => corpus(&args[1..]),
        "cards" => cards(&args[1..]),
        _ => usage(),
    }
}

fn usage() -> Result<()> {
    eprintln!("usage: si replay <EndGame.txt | capture dir> [--defs <dir>] [--json] [--lands]");
    eprintln!("       si report <EndGame.txt | capture dir> [--defs <dir>]        land tempo, track tempo, build profile");
    eprintln!("       si corpus <path>... [--defs <dir>] [--md]                  one row per game over captures and EndGame logs, then groupings");
    eprintln!("       si cards [--defs <dir>] [--unparsed [n]] [--card <title>] [--features] [--fit]   effect-parser coverage; one card's parse; the feature table; the pricing fit");
    Ok(())
}

fn defs_from(args: &[String]) -> Result<Option<Definitions>> {
    let dir = args.iter().position(|a| a == "--defs").and_then(|i| args.get(i + 1)).cloned().or_else(|| std::env::var("SI_DEFS_DIR").ok());
    match dir {
        Some(d) => Ok(Some(Definitions::from_dir(Path::new(&d))?)),
        None => Ok(None),
    }
}

fn replay(args: &[String]) -> Result<()> {
    let Some(path) = args.first() else { bail!("replay needs a log file or capture directory") };
    let defs = defs_from(args)?;
    let (lines, undo, source) = si_record::replay::load(&PathBuf::from(path))?;
    let rec = Replay::new(defs.as_ref()).run(&lines, undo.as_deref(), &source);
    if args.iter().any(|a| a == "--json") {
        println!("{}", serde_json::to_string_pretty(&rec)?);
        return Ok(());
    }
    print_record(&rec, defs.as_ref(), args.iter().any(|a| a == "--lands" || a == "--all"));
    Ok(())
}

fn cards(args: &[String]) -> Result<()> {
    let Some(defs) = defs_from(args)? else { bail!("cards needs --defs <dir> or SI_DEFS_DIR") };
    let pos = |flag: &str| args.iter().position(|a| a == flag);
    if let Some(i) = pos("--card") {
        let name = args.get(i + 1).cloned().unwrap_or_default();
        let Some(c) = defs.cards.get(&name).or_else(|| defs.card_by_title(&name)) else { bail!("no card {name}") };
        println!("{} [{}] {:?} cost {} {:?} range {:?} target {}", c.title, c.id, c.kind, c.cost, c.speed, c.range.as_ref().map(|r| (r.distance, r.from_sacred_site)), c.target.0);
        for b in &c.body {
            println!("  | {}", si_defs::text::plain(b));
        }
        for e in &c.effects {
            println!("  = {e:?}");
        }
        for t in &c.thresholds {
            println!("  threshold {} {}", t.elements, t.title);
            for b in &t.body {
                println!("    | {}", si_defs::text::plain(b));
            }
            for e in &t.effects {
                println!("    = {e:?}");
            }
        }
        let f = c.features();
        println!("  features {f:?}");
        println!("  acts on: explore {} build {} ravage {}", c.acts_on(si_model::power::Slot::Explore), c.acts_on(si_model::power::Slot::Build), c.acts_on(si_model::power::Slot::Ravage));
        return Ok(());
    }
    if let Some(i) = pos("--fit") {
        let kind = args.get(i + 1).filter(|k| !k.starts_with("--")).cloned().unwrap_or("all".into());
        return pricing_fit(&defs, &kind);
    }
    if args.iter().any(|a| a == "--features") {
        println!("id\ttitle\tkind\tcost\tspeed\trange\tsacred\ttarget\telements\t{}\tother", si_model::power::Features::NAMES.join("\t"));
        for c in defs.cards.values() {
            let f = c.features();
            let v: Vec<String> = f.vector().iter().map(|x| format!("{x}")).collect();
            println!("{}\t{}\t{:?}\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{}", c.id, c.title, c.kind, c.cost, c.speed, c.range.as_ref().map(|r| r.distance.to_string()).unwrap_or("-".into()), c.range.as_ref().is_some_and(|r| r.from_sacred_site), c.target.0, c.elements.total(), v.join("\t"), f.other);
        }
        return Ok(());
    }
    let total = defs.cards.len();
    let full = defs.cards.values().filter(|c| c.fully_parsed()).count();
    let mut by_kind: std::collections::BTreeMap<String, (usize, usize)> = Default::default();
    let mut slots = [0usize; 3];
    for c in defs.cards.values() {
        let e = by_kind.entry(format!("{:?}", c.kind)).or_default();
        e.0 += 1;
        if c.fully_parsed() {
            e.1 += 1;
        }
        for (i, s) in [si_model::power::Slot::Explore, si_model::power::Slot::Build, si_model::power::Slot::Ravage].into_iter().enumerate() {
            if c.acts_on(s) {
                slots[i] += 1;
            }
        }
    }
    println!("{full} of {total} Power Cards fully parsed ({:.0}%)", 100.0 * full as f64 / total.max(1) as f64);
    for (k, (n, f)) in &by_kind {
        println!("  {k}: {f}/{n}");
    }
    println!("cards that can act on a slot: explore {} · build {} · ravage {}", slots[0], slots[1], slots[2]);
    let tiers: Vec<&si_model::spirit::Tier> = defs.spirits.values().flat_map(|s| s.innates.iter().flat_map(|i| i.tiers.iter())).collect();
    let tiers_full = tiers.iter().filter(|t| !t.effects.is_empty() && !t.effects.iter().any(|e| e.is_other())).count();
    println!("{tiers_full} of {} innate tiers fully parsed", tiers.len());
    if let Some(i) = pos("--unparsed") {
        let n: usize = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(40);
        let mut counts: std::collections::BTreeMap<String, (u32, String)> = Default::default();
        for c in defs.cards.values() {
            for s in c.unparsed() {
                let key: String = s.chars().map(|ch| if ch.is_ascii_digit() { 'N' } else { ch }).collect();
                let e = counts.entry(key).or_insert((0, c.title.clone()));
                e.0 += 1;
            }
        }
        let mut v: Vec<_> = counts.into_iter().collect();
        v.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(&b.0)));
        println!("unparsed sentences ({} distinct):", v.len());
        for (s, (n_, ex)) in v.into_iter().take(n) {
            println!("  {n_:>3}  {s}   [{ex}]");
        }
    }
    Ok(())
}

/// Section 4 of the tempo spec: cost regressed on what the card does, over
/// the fully parsed cards. Coefficients are the implied Energy price of one
/// unit of each feature; residuals name the cards priced under or over
/// their effect.
fn pricing_fit(defs: &Definitions, kind: &str) -> Result<()> {
    use si_model::power::{Features, PowerKind, Speed};
    let want = |c: &si_model::PowerCard| match kind {
        "minor" => c.kind == PowerKind::Minor,
        "major" => c.kind == PowerKind::Major,
        "unique" => c.kind == PowerKind::Unique,
        _ => true,
    };
    let cards: Vec<&si_model::PowerCard> = defs.cards.values().filter(|c| c.fully_parsed() && want(c)).collect();
    let names: Vec<String> = ["intercept"].iter().map(|s| s.to_string()).chain(Features::NAMES.iter().map(|s| s.to_string())).chain(["slow", "range", "from sacred site", "major", "unique", "elements"].iter().map(|s| s.to_string())).collect();
    let row = |c: &si_model::PowerCard| -> Vec<f64> {
        let mut v = vec![1.0];
        v.extend(c.features().vector());
        v.push(if c.speed == Speed::Slow { 1.0 } else { 0.0 });
        v.push(c.range.as_ref().map(|r| r.distance as f64).unwrap_or(0.0));
        v.push(if c.range.as_ref().is_some_and(|r| r.from_sacred_site) { 1.0 } else { 0.0 });
        v.push(if c.kind == PowerKind::Major { 1.0 } else { 0.0 });
        v.push(if c.kind == PowerKind::Unique { 1.0 } else { 0.0 });
        v.push(c.elements.total() as f64);
        v
    };
    let x: Vec<Vec<f64>> = cards.iter().map(|c| row(c)).collect();
    let y: Vec<f64> = cards.iter().map(|c| c.cost as f64).collect();
    let Some(beta) = si_metrics::fit::least_squares(&x, &y) else { bail!("no cards to fit") };
    let res = si_metrics::fit::residuals(&x, &y, &beta);
    println!("PRICING FIT ({kind}) — cost ≈ Σ coefficient × feature, over {} fully parsed cards (R² {:.2})", cards.len(), si_metrics::fit::r_squared(&y, &res));
    println!("Energy per unit of:");
    for (i, (n, b)) in names.iter().zip(&beta).enumerate() {
        let used = x.iter().any(|r| r[i] != 0.0) && (i == 0 || x.iter().any(|r| r[i] != x[0][i]));
        if used {
            println!("  {:<18} {:>6.2}", n, b);
        }
    }
    let mut order: Vec<usize> = (0..cards.len()).collect();
    order.sort_by(|a, b| res[*b].partial_cmp(&res[*a]).unwrap());
    println!("priced above their effect (cost − fit, largest first):");
    for i in order.iter().take(10) {
        println!("  {:>+5.1}  {} (cost {}, {:?})", res[*i], cards[*i].title, cards[*i].cost, cards[*i].kind);
    }
    println!("priced below their effect:");
    for i in order.iter().rev().take(10) {
        println!("  {:>+5.1}  {} (cost {}, {:?})", res[*i], cards[*i].title, cards[*i].cost, cards[*i].kind);
    }
    Ok(())
}

fn report(args: &[String]) -> Result<()> {
    let Some(path) = args.first() else { bail!("report needs a log file or capture directory") };
    let defs = defs_from(args)?;
    let (lines, undo, source) = si_record::replay::load(&PathBuf::from(path))?;
    let rec = Replay::new(defs.as_ref()).run(&lines, undo.as_deref(), &source);
    let chains = si_metrics::tempo::chains_with(&rec, defs.as_ref());
    println!("{}", rec.meta.source);
    println!("spirits {} | boards {} | adversary {} | turns {} | {}", rec.meta.spirits.join("+"), rec.meta.boards.join(""), rec.meta.adversary.clone().unwrap_or("none".into()), rec.meta.turns, rec.meta.ending.map(|e| e.words()).unwrap_or("no ending"));
    if rec.coverage.undo_unresolved > 0 {
        println!("caution: {} of {} undo reloads had no clock; placed one save back, {} repaired from repeated facts", rec.coverage.undo_unresolved, rec.coverage.undo_reloads, rec.coverage.undo_repairs);
    }
    if let Some(est) = defs.as_ref().and_then(|d| si_metrics::adversary::estimate(&rec, d)) {
        println!(
            "adversary level {} (difficulty {}) — {}",
            est.label(),
            est.difficulty.map(|(a, b)| if a == b { a.to_string() } else { format!("{a}-{b}") }).unwrap_or("?".into()),
            est.evidence.join("; ")
        );
    }
    println!();
    println!("LAND TEMPO — each Invader card's pass over a land: Explore t → Build t+1 → Ravage t+2");
    println!("land terrain   card       t0   E  B  R   outcome    blight  class       price    reach                      touches by clock position");
    for c in &chains {
        // one entry per clock position: the powers used, then what else happened
        let mut touches: Vec<String> = Vec::new();
        for clock in [si_metrics::Clock::BeforeExplore, si_metrics::Clock::AfterExplore, si_metrics::Clock::AfterBuild, si_metrics::Clock::AfterRavage] {
            let here: Vec<&si_metrics::tempo::ChainTouch> = c.touches.iter().filter(|t| t.clock == clock).collect();
            if here.is_empty() {
                continue;
            }
            let mut powers: Vec<String> = Vec::new();
            let (mut defend, mut dmg, mut out, mut into, mut presence, mut fate) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
            for t in &here {
                if !t.touch.by_spirit() {
                    fate += 1;
                    continue;
                }
                match &t.touch.kind {
                    TouchKind::Power => {
                        let p = short(&t.touch.by);
                        if !powers.contains(&p) {
                            powers.push(p);
                        }
                    }
                    TouchKind::Defend(n) => defend += n,
                    TouchKind::Damage(n) => dmg += n,
                    TouchKind::Presence => presence += 1,
                    TouchKind::Moved { out: o, into: i } => {
                        out += o;
                        into += i;
                    }
                    TouchKind::Recover(_) => {}
                }
            }
            let mut parts = powers;
            if defend > 0 {
                parts.push(format!("def{defend}"));
            }
            if dmg > 0 {
                parts.push(format!("dmg{dmg}"));
            }
            if out + into > 0 {
                parts.push(format!("mv-{out}+{into}"));
            }
            if presence > 0 {
                parts.push(format!("+{presence}pres"));
            }
            if fate > 0 {
                parts.push(format!("event/fear×{fate}"));
            }
            let name = match clock {
                si_metrics::Clock::BeforeExplore => "pre-X",
                si_metrics::Clock::AfterExplore => "X→B",
                si_metrics::Clock::AfterBuild => "B→R",
                si_metrics::Clock::AfterRavage => "post-R",
            };
            touches.push(format!("{name}[{}]", parts.join(" ")));
        }
        let opt = |t: Option<u32>| t.map(|t| t.to_string()).unwrap_or("-".into());
        let price = if c.price_cards > 0 { format!("{}c {}E", c.price_cards, c.price_energy) } else { String::new() };
        let reach = match &c.reach {
            Some(r) if c.left_to_blight() => match r.first_ravage() {
                Some(card) if card.sure() => format!("could: {} T{}", short(&card.card), card.turn),
                Some(card) => format!("maybe: {} (range?)", short(&card.card)),
                None => "no reach".to_string(),
            },
            Some(r) if c.class == si_metrics::Class::On && r.could_build() => "above was open".to_string(),
            _ => String::new(),
        };
        println!(
            "{:<4} {:<9} {:<10} {:>2}  {:>2} {:>2} {:>2}   {:<10} {:>3}{}  {:<11} {:<8} {:<26} {}",
            c.land,
            c.terrain.map(|t| t.name()).unwrap_or(""),
            c.card.clone().unwrap_or_default(),
            c.t0,
            opt(c.explore),
            opt(c.build),
            opt(c.ravage),
            format!("{:?}", c.outcome).to_lowercase(),
            c.blight,
            if c.cascade > 0 { format!("+{}", c.cascade) } else { "  ".into() },
            c.class.name(),
            price,
            reach,
            touches.join("  ")
        );
    }
    println!();
    println!("LEDGER — per turn: lands due, how the Ravage-due ones ended, and what the spirits paid (all spirits)");
    println!("turn  due B/R   above  on  gambled  failed  untouched  sacrificed  blight   cards  energy   hand→discard  runway");
    let mut all_rows: Vec<si_metrics::TrackRow> = Vec::new();
    for sp in &rec.meta.spirits {
        all_rows.extend(si_metrics::track::rows(&rec, defs.as_ref(), sp));
    }
    let paid = |turn: u32| rec.spirits.iter().filter(|s| s.turn == turn).map(|s| s.energy_paid).sum::<i64>();
    let ledger = si_metrics::ledger::ledger(&chains, &all_rows, &paid);
    for l in &ledger {
        println!(
            "T{:<4} {:>2}/{:<3}   {:>5} {:>3} {:>8} {:>7} {:>10} {:>11} {:>7}   {:>5} {:>7}   {:>4}→{:<9} {}",
            l.turn,
            l.due_build,
            l.due_ravage,
            l.above,
            l.on,
            l.gambled,
            l.failed,
            l.untouched,
            l.sacrificed,
            l.blight,
            l.cards_spent,
            l.energy_spent,
            l.hand_after,
            l.discard_after,
            match (l.reclaimed, l.runway) {
                (true, _) => "reclaim".to_string(),
                (false, Some(r)) => format!("{r} turn{}", if r == 1 { "" } else { "s" }),
                (false, None) => "-".to_string(),
            }
        );
    }
    let handled: u32 = ledger.iter().map(si_metrics::LedgerTurn::handled).sum();
    let spent: u32 = ledger.iter().map(|l| l.cards_spent).sum();
    let (price_cards, price_energy): (u32, i64) = chains.iter().map(|c| (c.price_cards, c.price_energy)).fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
    if spent > 0 {
        println!("handled per card over the game: {handled} lands / {spent} cards = {:.2} · cards aimed at a chain's land: {price_cards} ({price_energy} Energy)", handled as f64 / spent as f64);
    }
    println!();
    println!("by Ravage turn:");
    for (turn, classes) in si_metrics::tempo::classes_by_turn(&chains) {
        println!("  T{:<3} {}", turn, classes.iter().map(|(c, n)| format!("{} {}", c.name(), n)).collect::<Vec<_>>().join(" · "));
    }
    let total = chains.iter().filter(|c| c.class != si_metrics::Class::Open).count().max(1);
    let count = |cl: si_metrics::Class| chains.iter().filter(|c| c.class == cl).count();
    println!(
        "  game  above {} · on {} · gambled {} · failed {} · sacrificed {} · untouched {} · late {}  (of {} chains)",
        count(si_metrics::Class::Above),
        count(si_metrics::Class::On),
        count(si_metrics::Class::Gambled),
        count(si_metrics::Class::Failed),
        count(si_metrics::Class::Sacrificed),
        count(si_metrics::Class::Untouched),
        count(si_metrics::Class::Late),
        total
    );
    println!();
    for sp in &rec.meta.spirits {
        let title = defs.as_ref().and_then(|d| d.spirits.get(sp)).map(|p| p.title.clone()).unwrap_or(sp.clone());
        let rows = si_metrics::track::rows(&rec, defs.as_ref(), sp);
        println!("TRACK TEMPO — {title}");
        println!("turn  growth   x/ycp   presence  cards  reclaim  played/plays  energy  hand  discard");
        for r in &rows {
            println!(
                "T{:<4} {:<8} {:<7} +{:<2}{:<5} +{:<4} {:<8} {}/{:<10} {:>4}  {:>4}  {:>7}",
                r.turn,
                r.growth,
                match (r.income, r.plays) {
                    (Some(i), Some(p)) => format!("{i}/{p}cp"),
                    _ => "?".into(),
                },
                r.presence_placed,
                format!("{}{}", if r.presence_other > 0 { format!("+{}b", r.presence_other) } else { String::new() }, if r.presence_destroyed > 0 { format!("(-{})", r.presence_destroyed) } else { String::new() }),
                r.cards_gained,
                if r.reclaimed { "reclaim" } else { "" },
                r.cards_played,
                r.plays.map(|p| p.to_string()).unwrap_or("?".into()),
                r.energy_after,
                r.hand,
                r.discard
            );
        }
        for spike in si_metrics::spikes::spikes_for(sp) {
            match si_metrics::track::spike_turn(&rows, &spike) {
                Some(t) => println!("spike {} reached T{}{}  — {}", spike.label, t, spike.by_turn.map(|b| format!(" (target T{b})")).unwrap_or_default(), spike.source),
                None => println!("spike {} not reached{}  — {}", spike.label, spike.by_turn.map(|b| format!(" (target T{b})")).unwrap_or_default(), spike.source),
            }
        }
        let weak = si_metrics::track::weak_turns(&rows);
        println!("weak turns {:?} · reclaim turns {:?} · reclaim loops {}", weak, si_metrics::track::reclaim_turns(&rows), si_metrics::track::reclaim_loops(&rows));
        let blight_by_t4: u32 = rec.turns.iter().filter(|t| t.turn >= 1 && t.turn <= 4).map(|t| t.blight_added + t.blight_cascaded).sum();
        let p = si_metrics::profile::profile(defs.as_ref(), sp, &rows, &chains, blight_by_t4);
        println!(
            "PROFILE  scaling {:?} (presence by T3 {}/{}, cards gained {}, first reclaim {}) · stance {:?} (above share {}) · blight by T4 {}",
            p.scaling,
            p.presence_by_t3.0,
            p.presence_by_t3.1,
            p.cards_gained_by_t3,
            p.first_reclaim.map(|t| format!("T{t}")).unwrap_or("none".into()),
            p.stance,
            p.above_share.map(|s| format!("{:.0}%", s * 100.0)).unwrap_or("-".into()),
            p.blight_by_t4
        );
        println!();
    }
    println!("GROWTH SHAPES — each series over the turns, binned by the simplest order that explains it (Lumia)");
    for (name, series) in growth_series(&rec) {
        match si_metrics::growth::fit(&series) {
            Some(f) => println!("  {:<28} {:<12} {}", name, f.order.name(), series.iter().map(|v| format!("{v}")).collect::<Vec<_>>().join(" ")),
            None => println!("  {:<28} {:<12} {}", name, "too short", series.iter().map(|v| format!("{v}")).collect::<Vec<_>>().join(" ")),
        }
    }
    Ok(())
}

/// The per-turn series a game record gives, for growth fitting.
fn growth_series(rec: &GameRecord) -> Vec<(String, Vec<f64>)> {
    let mut out = Vec::new();
    let turns: Vec<&si_record::TurnRecord> = rec.turns.iter().filter(|t| t.turn >= 1).collect();
    let mut blight = 0.0;
    out.push(("Blight on the island".into(), turns.iter().map(|t| { blight += (t.blight_added + t.blight_cascaded) as f64; blight }).collect()));
    out.push(("Invaders on the board".into(), turns.iter().map(|t| t.board_after.invaders() as f64).collect()));
    out.push(("lands with Invaders".into(), turns.iter().map(|t| t.lands_with_invaders_after as f64).collect()));
    let mut fear = 0.0;
    out.push(("Fear cards earned".into(), turns.iter().map(|t| { fear += t.fear_cards_earned as f64; fear }).collect()));
    for sp in &rec.meta.spirits {
        let rows: Vec<&si_record::SpiritTurn> = rec.spirits.iter().filter(|s| s.spirit == *sp && s.turn >= 1).collect();
        out.push((format!("{} Energy banked", short(sp)), rows.iter().map(|s| s.energy_after as f64).collect()));
        out.push((format!("{} presence on island", short(sp)), rows.iter().map(|s| s.presence_after.values().sum::<u32>() as f64).collect()));
        out.push((format!("{} income", short(sp)), rows.iter().map(|s| s.income.unwrap_or(0) as f64).collect()));
        out.push((format!("{} cards in hand", short(sp)), rows.iter().map(|s| s.hand_after.len() as f64).collect()));
    }
    out
}

/// One game, measured.
struct Row {
    source: String,
    exact: bool,
    adversary: Option<String>,
    level: String,
    difficulty: Option<(u8, u8)>,
    blight_shape: String,
    spirits: Vec<String>,
    boards: String,
    turns: u32,
    ending: Option<si_model::Ending>,
    scaling: si_metrics::Scaling,
    stance: si_metrics::Stance,
    presence_t3: (u32, u32),
    cards_t3: u32,
    first_reclaim: Option<u32>,
    spike: Option<(String, Option<u32>, Option<u32>)>,
    classes: std::collections::BTreeMap<si_metrics::Class, u32>,
    blight_t4: u32,
    blight_end: u32,
    fear_cards_t4: u32,
    weak: u32,
    signature: String,
}

fn games_under(path: &Path) -> Vec<PathBuf> {
    if path.is_file() {
        return vec![path.to_path_buf()];
    }
    if path.join("log-stream.txt").exists() {
        return vec![path.to_path_buf()];
    }
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(path) {
        let mut entries: Vec<PathBuf> = rd.filter_map(|e| e.ok()).map(|e| e.path()).collect();
        entries.sort();
        for e in entries {
            if e.is_dir() && e.join("log-stream.txt").exists() {
                out.push(e);
            } else if e.is_file() && e.file_name().is_some_and(|n| n.to_string_lossy().starts_with("EndGame--") && n.to_string_lossy().ends_with(".txt")) {
                out.push(e);
            }
        }
    }
    out
}

fn measure(rec: &GameRecord, defs: Option<&Definitions>, source: &str) -> Row {
    let chains = si_metrics::tempo::chains_with(rec, defs);
    let spirit = rec.meta.spirits.first().cloned().unwrap_or_default();
    let rows = si_metrics::track::rows(rec, defs, &spirit);
    let blight_t4: u32 = rec.turns.iter().filter(|t| (1..=4).contains(&t.turn)).map(|t| t.blight_added + t.blight_cascaded).sum();
    let blight_end: u32 = rec.turns.iter().filter(|t| t.turn >= 1).map(|t| t.blight_added + t.blight_cascaded).sum();
    let fear_cards_t4: u32 = rec.turns.iter().filter(|t| (1..=4).contains(&t.turn)).map(|t| t.fear_cards_earned).sum();
    let p = si_metrics::profile::profile(defs, &spirit, &rows, &chains, blight_t4);
    let spike = si_metrics::spikes::spikes_for(&spirit).first().map(|sp| (sp.label.clone(), sp.by_turn, si_metrics::track::spike_turn(&rows, sp)));
    let mut classes = std::collections::BTreeMap::new();
    for c in &chains {
        if c.class != si_metrics::Class::Open {
            *classes.entry(c.class).or_insert(0u32) += 1;
        }
    }
    // the same game recorded twice (a capture and its EndGame log) shares its
    // first revealed Invader cards and its ending; distinct cards, so an undo
    // re-reveal in a bare log does not split the pair
    let mut explores: Vec<String> = Vec::new();
    for c in rec.turns.iter().flat_map(|t| t.explore.iter()) {
        if !explores.contains(c) {
            explores.push(c.clone());
        }
        if explores.len() == 4 {
            break;
        }
    }
    let signature = format!("{}|{}|{:?}|{}", rec.meta.spirits.join("+"), rec.meta.boards.join(""), rec.meta.ending, explores.join(","));
    let est = defs.and_then(|d| si_metrics::adversary::estimate(rec, d));
    let blight_series = growth_series(rec).into_iter().find(|(n, _)| n == "Blight on the island").map(|(_, s)| s).unwrap_or_default();
    Row {
        source: source.to_string(),
        exact: rec.coverage.undo_unresolved == 0,
        adversary: rec.meta.adversary.clone(),
        level: est.as_ref().map(|e| e.label()).unwrap_or_else(|| if rec.meta.adversary.is_some() { "?".into() } else { "-".into() }),
        difficulty: est.as_ref().and_then(|e| e.difficulty),
        blight_shape: si_metrics::growth::fit(&blight_series).map(|f| f.order.name()).unwrap_or("-".into()),
        spirits: rec.meta.spirits.clone(),
        boards: rec.meta.boards.join(""),
        turns: rec.meta.turns,
        ending: rec.meta.ending,
        scaling: p.scaling,
        stance: p.stance,
        presence_t3: p.presence_by_t3,
        cards_t3: p.cards_gained_by_t3,
        first_reclaim: p.first_reclaim,
        spike,
        classes,
        blight_t4,
        blight_end,
        fear_cards_t4,
        weak: si_metrics::track::weak_turns(&rows).len() as u32,
        signature,
    }
}

fn corpus(args: &[String]) -> Result<()> {
    let defs = defs_from(args)?;
    let md = args.iter().any(|a| a == "--md");
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut skip = false;
    for (i, a) in args.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        if a == "--defs" {
            skip = true;
            continue;
        }
        if a.starts_with("--") {
            continue;
        }
        let _ = i;
        paths.extend(games_under(Path::new(a)));
    }
    // A bare EndGame log of a game that a capture also holds is the same game
    // twice: the capture (exact undo clock) wins. Captures first.
    paths.sort_by_key(|p| !p.is_dir());
    let mut capture_lines: Vec<std::collections::HashSet<String>> = Vec::new();
    let mut rows: Vec<Row> = Vec::new();
    for p in &paths {
        let (lines, undo, source) = match si_record::replay::load(p) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("skip {}: {e}", p.display());
                continue;
            }
        };
        let actions: Vec<&str> = lines.iter().map(|l| l.text.trim()).filter(|t| t.starts_with("DidPerformAction")).collect();
        if p.is_dir() {
            capture_lines.push(actions.iter().map(|t| t.to_string()).collect());
        } else {
            let tail: Vec<&str> = actions.iter().rev().take(40).copied().collect();
            if !tail.is_empty() && capture_lines.iter().any(|set| tail.iter().filter(|t| set.contains(**t)).count() * 10 >= tail.len() * 9) {
                eprintln!("skip {} (same game as a capture)", p.display());
                continue;
            }
        }
        let rec = Replay::new(defs.as_ref()).run(&lines, undo.as_deref(), &source);
        if !rec.meta.from_setup || rec.meta.turns < 2 {
            eprintln!("skip {} (joined in progress or too short: {} turns)", p.display(), rec.meta.turns);
            continue;
        }
        rows.push(measure(&rec, defs.as_ref(), &source));
    }
    // dedupe: keep the exact (capture) record of a game over its bare log
    rows.sort_by(|a, b| a.signature.cmp(&b.signature).then(b.exact.cmp(&a.exact)));
    rows.dedup_by(|a, b| a.signature == b.signature);
    rows.sort_by(|a, b| a.spirits.cmp(&b.spirits).then(a.source.cmp(&b.source)));
    let sep = if md { " | " } else { "  " };
    let name = |s: &str| defs.as_ref().and_then(|d| d.spirits.get(s)).map(|p| p.short_title.clone()).filter(|t| !t.is_empty()).unwrap_or(s.to_string());
    let header = ["spirit", "board", "adversary", "level", "diff", "result", "turns", "undo", "scaling", "stance", "pres T3", "cards T3", "reclaim", "spike", "above", "on", "gambled", "failed", "sacrificed", "untouched", "late", "blight T4", "blight end", "blight shape", "fear cards T4", "weak turns"];
    if md {
        println!("| {} |", header.join(" | "));
        println!("|{}|", header.iter().map(|_| "---").collect::<Vec<_>>().join("|"));
    } else {
        println!("{}", header.join(sep));
    }
    let count = |r: &Row, c: si_metrics::Class| r.classes.get(&c).copied().unwrap_or(0);
    for r in &rows {
        let cells = vec![
            r.spirits.iter().map(|s| name(s)).collect::<Vec<_>>().join("+"),
            r.boards.clone(),
            r.adversary.clone().map(|a| a.trim_start_matches("The").replace("KingdomOf", "").replace("TsardomOf", "").replace("Expedition", "").replace("Monarchy", "")).unwrap_or("-".into()),
            r.level.clone(),
            r.difficulty.map(|(a, b)| if a == b { a.to_string() } else { format!("{a}-{b}") }).unwrap_or("-".into()),
            r.ending.map(|e| if e.won() { "won" } else { "lost" }).unwrap_or("open").to_string(),
            r.turns.to_string(),
            if r.exact { "exact" } else { "heuristic" }.to_string(),
            format!("{:?}", r.scaling),
            format!("{:?}", r.stance),
            format!("{}/{}", r.presence_t3.0, r.presence_t3.1),
            r.cards_t3.to_string(),
            r.first_reclaim.map(|t| format!("T{t}")).unwrap_or("-".into()),
            r.spike.as_ref().map(|(l, by, got)| format!("{l} {}{}", got.map(|t| format!("T{t}")).unwrap_or("never".into()), by.map(|b| format!(" (target T{b})")).unwrap_or_default())).unwrap_or("-".into()),
            count(r, si_metrics::Class::Above).to_string(),
            count(r, si_metrics::Class::On).to_string(),
            count(r, si_metrics::Class::Gambled).to_string(),
            count(r, si_metrics::Class::Failed).to_string(),
            count(r, si_metrics::Class::Sacrificed).to_string(),
            count(r, si_metrics::Class::Untouched).to_string(),
            count(r, si_metrics::Class::Late).to_string(),
            r.blight_t4.to_string(),
            r.blight_end.to_string(),
            r.blight_shape.clone(),
            r.fear_cards_t4.to_string(),
            r.weak.to_string(),
        ];
        if md {
            println!("| {} |", cells.join(" | "));
        } else {
            println!("{}", cells.join(sep));
        }
    }
    println!();
    // groupings: wins by scaling, by stance, by blight at T4, by spike reached
    let finished: Vec<&Row> = rows.iter().filter(|r| r.ending.is_some()).collect();
    let won = |rs: &[&Row]| rs.iter().filter(|r| r.ending.is_some_and(|e| e.won())).count();
    println!("GROUPINGS ({} finished games; counts, not significance)", finished.len());
    for sc in [si_metrics::Scaling::Fast, si_metrics::Scaling::Minus, si_metrics::Scaling::Problematic, si_metrics::Scaling::Unknown] {
        let g: Vec<&Row> = finished.iter().copied().filter(|r| r.scaling == sc).collect();
        if !g.is_empty() {
            println!("  scaling {:?}: {} games, {} won, blight by T4 avg {:.1}", sc, g.len(), won(&g), g.iter().map(|r| r.blight_t4 as f64).sum::<f64>() / g.len() as f64);
        }
    }
    for st in [si_metrics::Stance::Above, si_metrics::Stance::On, si_metrics::Stance::Unknown] {
        let g: Vec<&Row> = finished.iter().copied().filter(|r| r.stance == st).collect();
        if !g.is_empty() {
            println!("  stance {:?}: {} games, {} won, blight by T4 avg {:.1}", st, g.len(), won(&g), g.iter().map(|r| r.blight_t4 as f64).sum::<f64>() / g.len() as f64);
        }
    }
    for (label, pred) in [("blight by T4 ≤ 2", Box::new(|r: &Row| r.blight_t4 <= 2) as Box<dyn Fn(&Row) -> bool>), ("blight by T4 3–4", Box::new(|r: &Row| (3..=4).contains(&r.blight_t4))), ("blight by T4 ≥ 5", Box::new(|r: &Row| r.blight_t4 >= 5))] {
        let g: Vec<&Row> = finished.iter().copied().filter(|r| pred(r)).collect();
        if !g.is_empty() {
            println!("  {label}: {} games, {} won", g.len(), won(&g));
        }
    }
    for (label, pred) in [("adversary difficulty ≤ 4 (lowest candidate)", Box::new(|r: &Row| r.difficulty.is_some_and(|(a, _)| a <= 4)) as Box<dyn Fn(&Row) -> bool>), ("adversary difficulty 5–7", Box::new(|r: &Row| r.difficulty.is_some_and(|(a, _)| (5..=7).contains(&a)))), ("adversary difficulty ≥ 8", Box::new(|r: &Row| r.difficulty.is_some_and(|(a, _)| a >= 8))), ("no adversary", Box::new(|r: &Row| r.adversary.is_none()))] {
        let g: Vec<&Row> = finished.iter().copied().filter(|r| pred(r)).collect();
        if !g.is_empty() {
            println!("  {label}: {} games, {} won", g.len(), won(&g));
        }
    }
    let mut shapes: std::collections::BTreeMap<String, Vec<&Row>> = Default::default();
    for r in &finished {
        shapes.entry(r.blight_shape.clone()).or_default().push(r);
    }
    for (shape, g) in &shapes {
        println!("  blight grows {shape}: {} games, {} won", g.len(), won(g));
    }
    let sacrificed = |r: &Row| count(r, si_metrics::Class::Sacrificed) + count(r, si_metrics::Class::Untouched);
    for (label, pred) in [("no sacrificed/untouched chains", Box::new(|r: &Row| sacrificed(r) == 0) as Box<dyn Fn(&Row) -> bool>), ("1–2 sacrificed/untouched", Box::new(|r: &Row| (1..=2).contains(&sacrificed(r)))), ("3+ sacrificed/untouched", Box::new(|r: &Row| sacrificed(r) >= 3))] {
        let g: Vec<&Row> = finished.iter().copied().filter(|r| pred(r)).collect();
        if !g.is_empty() {
            println!("  {label}: {} games, {} won", g.len(), won(&g));
        }
    }
    Ok(())
}

fn print_record(r: &GameRecord, defs: Option<&Definitions>, lands: bool) {
    let m = &r.meta;
    println!("{}", m.source);
    println!(
        "spirits {} | boards {} | turns {} | {} | {}",
        m.spirits.join("+"),
        m.boards.join(""),
        m.turns,
        m.ending.map(|e| e.words()).unwrap_or("no ending"),
        if m.from_setup { "from setup" } else { "joined in progress" }
    );
    let c = &r.coverage;
    println!(
        "coverage: {} typed, {} skipped ({}), {} noise | undo reloads {} ({} events dropped, {} unresolved, {} repaired) | log copies {}",
        c.typed_total(),
        c.skipped_total(),
        c.skipped.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "),
        c.noise,
        c.undo_reloads,
        c.undo_dropped,
        c.undo_unresolved,
        c.undo_repairs,
        c.copies
    );
    println!();
    println!("turn  ravage      build       explore     fear  cards  terror  blight(+casc)  E/T/C/D after  invaded");
    for t in &r.turns {
        println!(
            "{:<5} {:<10}  {:<10}  {:<10}  {:>4}  {:>5}  {:>6}  {:>5}(+{})      {:>2}/{:>2}/{:>2}/{:>2}       {}",
            if t.turn == 0 { "setup".to_string() } else { format!("T{}", t.turn) },
            t.ravage.join("+"),
            t.build.join("+"),
            t.explore.join("+"),
            t.fear_generated,
            t.fear_cards_earned,
            t.terror_after,
            t.blight_added,
            t.blight_cascaded,
            t.board_after.explorers,
            t.board_after.towns,
            t.board_after.cities,
            t.board_after.dahan,
            t.lands_with_invaders_after
        );
    }
    println!();
    for sp in &m.spirits {
        let title = defs.and_then(|d| d.spirits.get(sp)).map(|p| p.title.clone()).unwrap_or(sp.clone());
        println!("{title}");
        println!("turn  growth           x/ycp   presence placed            E +gain -paid =after  played                               gained/reclaimed/forgotten          hand  discard  powers (phase target)");
        for st in r.spirits.iter().filter(|s| s.spirit == *sp) {
            let growth = if st.growth_groups.is_empty() { "-".to_string() } else { st.growth_groups.iter().map(|g| format!("G{g}")).collect::<Vec<_>>().join("+") };
            let xy = match (st.income, st.plays) {
                (Some(i), Some(p)) => format!("{i}/{p}cp"),
                _ => "?".into(),
            };
            let changes = format!(
                "{}{}{}",
                if st.cards_gained.is_empty() { String::new() } else { format!("+{} ", st.cards_gained.join(",")) },
                if st.reclaimed { format!("reclaim({}) ", st.reclaimed_cards.len()) } else { String::new() },
                if st.cards_forgotten.is_empty() { String::new() } else { format!("forget {} ", st.cards_forgotten.join(",")) }
            );
            let powers: Vec<String> = st.powers_used.iter().map(|(ph, p, l)| format!("{}:{}{}", &ph.name()[..1], short(p), l.as_ref().map(|l| format!("@{l}")).unwrap_or_default())).collect();
            let mut placed: Vec<String> = st.presence_placed.iter().map(|(n, l)| format!("{}→{}", n.as_deref().map(slot_short).unwrap_or("board"), l.as_ref().map(|l| l.to_string()).unwrap_or("?".into()))).collect();
            placed.extend(st.presence_moved.iter().map(|(f, t)| format!("{f}⇢{t}")));
            println!(
                "{:<5} {:<16} {:<7} {:<26} {:>2} +{:<3} -{:<3} ={:<4}  {:<36} {:<35} {:>4}  {:>7}  {}",
                if st.turn == 0 { "setup".to_string() } else { format!("T{}", st.turn) },
                growth,
                xy,
                placed.join(" "),
                st.energy_after - st.energy_gained + st.energy_paid,
                st.energy_gained,
                st.energy_paid,
                st.energy_after,
                st.cards_played.iter().map(|c| short(c)).collect::<Vec<_>>().join(","),
                changes,
                st.hand_after.len(),
                st.discard_after.len(),
                powers.join(" ")
            );
        }
        println!();
    }
    if lands {
        println!("lands: visits (turn step card: +added blight/cascade dmg def dahan-lost inv-destroyed → E/T/C/D after) and touches (turn phase by)");
        for (id, l) in &r.lands {
            if l.visits.is_empty() && l.touches.is_empty() && l.blight.is_empty() {
                continue;
            }
            println!("{}{}  end E/T/C/D/B {}/{}/{}/{}/{}", id, l.terrain.map(|t| format!(" {t}")).unwrap_or_default(), l.end.explorers, l.end.towns, l.end.cities, l.end.dahan, l.end.blight);
            for v in &l.visits {
                println!(
                    "   T{} {:<7} {:<10} +{:<8} blight {}/{} dmg {} def {} dahan-lost {} destroyed {} → {}/{}/{}/{}",
                    v.turn,
                    v.step.map(|s| format!("{s:?}")).unwrap_or_default(),
                    v.card.clone().unwrap_or_default(),
                    v.added.iter().map(|k| format!("{k:?}")).collect::<Vec<_>>().join("+"),
                    v.blight,
                    v.cascade,
                    v.damage,
                    v.defend,
                    v.dahan_lost,
                    v.invaders_destroyed,
                    v.after.explorers,
                    v.after.towns,
                    v.after.cities,
                    v.after.dahan
                );
            }
            for t in &l.touches {
                let kind = match &t.kind {
                    TouchKind::Power => "power".to_string(),
                    TouchKind::Defend(n) => format!("defend {n}"),
                    TouchKind::Damage(n) => format!("damage {n}"),
                    TouchKind::Presence => "presence".to_string(),
                    TouchKind::Moved { out, into } => format!("moved out {out} in {into}"),
                    TouchKind::Recover(n) => format!("blight removed {n}"),
                };
                println!("   T{} {:<7} {:<22} {}", t.turn, t.phase.name(), kind, short(&t.by));
            }
        }
    }
}

/// `ShroudOfSilentMistCardPlays3` → `P3`, `…Energy2` → `E2`.
fn slot_short(id: &str) -> &str {
    if let Some(i) = id.find("CardPlays") {
        return &id[i + 4..];
    }
    if let Some(i) = id.find("Energy") {
        return &id[i..];
    }
    id
}

fn short(s: &str) -> String {
    let caps: String = s.chars().filter(|c| c.is_ascii_uppercase() || c.is_ascii_digit()).collect();
    if caps.len() >= 2 && s.chars().all(|c| c.is_ascii_alphanumeric()) { caps } else { s.chars().take(18).collect() }
}
