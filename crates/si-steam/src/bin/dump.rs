//! Dev tool: decode NRBF saves and print structure.
//!
//! `si-steam-dump check <files…>`        decode each file, report errors
//! `si-steam-dump hist <file>`           class histogram
//! `si-steam-dump tree <file> [path] [depth]`  print the tree under a dotted path

use si_steam::nrbf::{self, Node, Object, Value};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("check") => {
            let (mut ok, mut bad) = (0, 0);
            for f in &args[1..] {
                let buf = std::fs::read(f)?;
                match nrbf::decode(&buf) {
                    Ok(g) => {
                        ok += 1;
                        if args.len() == 2 {
                            println!("{f}: {} objects", g.objects.len());
                        }
                    }
                    Err(e) => {
                        bad += 1;
                        println!("FAIL {f}: {e}");
                    }
                }
            }
            println!("decoded {ok}, failed {bad}");
        }
        Some("hist") => {
            let g = nrbf::decode(&si_steam::source::read_save(std::path::Path::new(&args[1]))?)?;
            for (name, n) in g.class_histogram().iter().take(80) {
                println!("{n:6}  {name}");
            }
        }
        Some("tree") => {
            let g = nrbf::decode(&si_steam::source::read_save(std::path::Path::new(&args[1]))?)?;
            let mut node = g.root();
            if let Some(path) = args.get(2).filter(|p| !p.is_empty() && *p != ".") {
                for part in path.split('.') {
                    node = if let Ok(i) = part.parse::<usize>() {
                        node.item_nodes()[i]
                    } else {
                        node.child(part).ok_or_else(|| anyhow::anyhow!("no child {part}"))?
                    };
                }
            }
            let depth = args.get(3).and_then(|d| d.parse().ok()).unwrap_or(1);
            print_node(node, 0, depth);
        }
        Some("snap") => {
            let g = nrbf::decode(&si_steam::source::read_save(std::path::Path::new(&args[1]))?)?;
            println!("{:#?}", si_steam::engine_map::map(&g));
        }
        Some("sum") => {
            let g = nrbf::decode(&si_steam::source::read_save(std::path::Path::new(&args[1]))?)?;
            summary(&si_steam::engine_map::map(&g));
        }
        Some("adjprobe") => {
            let g = nrbf::decode(&si_steam::source::read_save(std::path::Path::new(&args[1]))?)?;
            let defs = si_steam::engine_defs::EngineDefs::load(std::path::Path::new(si_steam::engine_defs::DEFAULT_DLL))?;
            let island = g.root().child("Island").unwrap();
            let n = island.int("LandMatrixDimensionSize").unwrap() as usize;
            let cells: Vec<bool> = island.child("LandAdjacencyMatrix").unwrap().items().iter().map(|v| matches!(v, Value::Bool(true))).collect();
            let trues: Vec<(usize, usize)> = (0..n).flat_map(|i| (0..n).map(move |j| (i, j))).filter(|&(i, j)| cells[i * n + j]).collect();
            println!("n={n} true cells={}", trues.len());
            for i in 0..n { let row: Vec<usize> = (0..n).filter(|&j| cells[i*n+j]).collect(); if !row.is_empty() { println!("  {i}: {row:?}"); } }
            for b in island.child("islandBoards").unwrap().item_nodes() {
                let id = b.str("Identifier").unwrap();
                let def = &defs.boards[id];
                let adj: Vec<(usize, usize)> = def["lands"].as_array().unwrap().iter().flat_map(|l| { let a = l["number"].as_u64().unwrap() as usize; l["adjacentLands"].as_array().unwrap().iter().map(move |x| (a, x.as_u64().unwrap() as usize)).collect::<Vec<_>>() }).collect();
                println!("board {id} defs adjacency {:?}", adj);
            }
        }
        Some("dims") => {
            for f in &args[1..] {
                let g = nrbf::decode(&std::fs::read(f)?)?;
                let island = g.root().child("Island").unwrap();
                let boards: Vec<String> = island.child("islandBoards").unwrap().item_nodes().iter().map(|b| format!("{}#{}", b.str("Identifier").unwrap_or("?"), b.int("BoardNumber").unwrap_or(-1))).collect();
                println!("n={:?} boards={boards:?}", island.int("LandMatrixDimensionSize"));
            }
        }
        Some("defs") => {
            let d = si_steam::engine_defs::EngineDefs::load(std::path::Path::new(si_steam::engine_defs::DEFAULT_DLL))?;
            let out = std::path::Path::new(&args[1]);
            std::fs::create_dir_all(out)?;
            for (name, table) in [("power_cards", &d.power_cards), ("invader_cards", &d.invader_cards), ("boards", &d.boards), ("adversaries", &d.adversaries), ("spirits", &d.spirits), ("fear_cards", &d.fear_cards), ("event_cards", &d.event_cards), ("blight_cards", &d.blight_cards), ("scenarios", &d.scenarios)] {
                let sorted: std::collections::BTreeMap<_, _> = table.iter().collect();
                std::fs::write(out.join(format!("{name}.json")), serde_json::to_string_pretty(&sorted)?)?;
            }
            println!("engine {} → {}", d.engine_sha256, out.display());
        }
        Some("pub") => {
            let g = nrbf::decode(&si_steam::source::read_save(std::path::Path::new(&args[1]))?)?;
            let defs = si_steam::engine_defs::EngineDefs::load(std::path::Path::new(si_steam::engine_defs::DEFAULT_DLL))?;
            let p = si_steam::projection::project(&si_steam::engine_map::map(&g), &defs);
            println!("{}", serde_json::to_string_pretty(&p)?);
        }
        Some("scan") => {
            for f in &args[1..] {
                let g = nrbf::decode(&std::fs::read(f)?)?;
                let s = si_steam::engine_map::map(&g);
                let b = s.blight_card.as_ref();
                let name = std::path::Path::new(f).file_name().unwrap().to_string_lossy().to_string();
                println!("{name:58} T{} {:22} fear {:>2}/{} earned {} disc {} | blight {} flip {} | inv {} | ev {} | E{:?}", s.turn_index + 1, s.state, s.fear_generated, s.fear_required, s.fear_earned.len(), s.fear_discard.len(), b.map(|b| b.blight_remaining).unwrap_or(-1), b.map(|b| b.flipped).unwrap_or(false), s.invader_deck.len(), s.event_deck.len(), s.spirits.iter().map(|x| x.energy).collect::<Vec<_>>());
            }
        }
        Some("decisions") => {
            for d in si_steam::log_parse::capture_decisions(std::path::Path::new(&args[1])) {
                println!("T{} {:20} {:?}", d.turn, d.step, d.decision);
            }
        }
        Some("review") | Some("stats") | Some("backtest") | Some("timeline") | Some("outlook") | Some("race") | Some("chain") | Some("backfill") => {
            // Replay a capture's saves in capture order through the companion.
            let defs = si_steam::engine_defs::EngineDefs::load(std::path::Path::new(si_steam::engine_defs::DEFAULT_DLL))?;
            let dir = std::path::Path::new(&args[1]).join("saves");
            let mut files: Vec<_> = std::fs::read_dir(&dir)?.filter_map(|e| e.ok()).map(|e| e.path()).collect();
            files.sort();
            // The third argument: a save index, or a moment "T3:FastPower"
            // (the first save of that turn and step).
            let upto: usize = args.get(2).and_then(|a| a.parse().ok()).unwrap_or(usize::MAX);
            let moment: Option<(u32, String)> = args.get(2).and_then(|a| {
                let (t, step) = a.strip_prefix('T')?.split_once(':')?;
                Some((t.parse().ok()?, step.to_string()))
            });
            let mut c = si_steam::companion::Companion::default();
            for f in files.iter().take(upto.saturating_add(1)) {
                let g = nrbf::decode(&si_steam::source::read_save(f)?)?;
                c.push(si_steam::projection::project(&si_steam::engine_map::map(&g), &defs));
            }
            let at = |tl: &[si_common::public::PublicSnapshot]| -> usize {
                match &moment {
                    Some((t, step)) => tl.iter().position(|s| s.turn == *t && s.step == *step).unwrap_or(tl.len() - 1),
                    None => tl.len() - 1,
                }
            };
            if args[0] == "timeline" {
                // One public snapshot per line, for ad hoc analysis.
                for s in c.timeline() {
                    println!("{}", serde_json::to_string(s)?);
                }
            } else if args[0] == "race" {
                // With an index: the race at that save. Without: the race at
                // the final turn's first save, against the recorded ending.
                let print_rows = |turns: &[u32], rows: &[(String, String, Vec<f64>)]| {
                    print!("{:<34}", "");
                    for t in turns {
                        print!(" T{t:<5}");
                    }
                    println!();
                    for (_, title, cif) in rows {
                        print!("{:<34}", title.chars().take(34).collect::<String>());
                        for v in cif {
                            print!(" {:>6.2}", v);
                        }
                        println!();
                    }
                };
                if args.len() > 2 {
                    let tl = c.timeline();
                    let st = si_common::companion_stats::backtest::race_at(tl, tl.len() - 1);
                    let r = &st.race;
                    let s = tl.last().unwrap();
                    println!("T{} {} · {} orders · binding: {:?}", s.turn, s.step, r.orders, r.binding.map(|i| (&r.endings[i].title, r.endings[i].median_turn, r.endings[i].first_turn)));
                    let rows: Vec<(String, String, Vec<f64>)> = r.endings.iter().map(|e| (e.key.clone(), e.title.clone(), e.cif.clone())).collect();
                    print_rows(&r.turns, &rows);
                    for e in r.endings.iter().filter(|e| !e.scenario.is_empty()) {
                        println!("  {}: {}", e.title, e.scenario.join(" · "));
                    }
                } else if let Some(k) = si_common::companion_stats::backtest::race_check(c.timeline()) {
                    println!(
                        "ending {:?} at T{} · race computed at T{} {} · realized {:?} cif {:?} · binding {:?} · others max {:.2}",
                        k.ending, k.end_turn, k.at.0, k.at.1, k.realized_key, k.realized_cif.map(|v| (v * 100.0).round() / 100.0), k.binding, k.others_max
                    );
                    print_rows(&k.turns, &k.rows);
                }
            } else if args[0] == "backfill" {
                // Pretend the companion joined at turn N: rebuild the turns
                // before it from the log and compare with the real saves.
                // `backfill <capture> T<N>`.
                let n: u32 = args.get(2).and_then(|a| a.trim_start_matches('T').parse().ok()).unwrap_or(3);
                let tl = c.timeline();
                let Some(idx) = tl.iter().position(|s| s.turn >= n) else { anyhow::bail!("no save at turn {n}") };
                let anchor = &tl[idx];
                let real = si_common::companion_stats::round_summaries(tl);
                // The anchor save's time: the capture file whose snapshot is
                // the anchor (an undo can repeat a turn's first save).
                let mut anchor_ms = None;
                for f in &files {
                    let g = nrbf::decode(&si_steam::source::read_save(f)?)?;
                    if si_steam::projection::project(&si_steam::engine_map::map(&g), &defs) == *anchor {
                        let name = f.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                        anchor_ms = name.split_once("__").and_then(|(stamp, _)| {
                            let (hms, ms) = stamp.split_once('_')?;
                            let v = |r: std::ops::Range<usize>| hms.get(r)?.parse::<u64>().ok();
                            Some(((v(0..2)? * 60 + v(2..4)?) * 60 + v(4..6)?) * 1000 + ms.parse::<u64>().ok()?)
                        });
                        break;
                    }
                }
                let Some(anchor_ms) = anchor_ms else { anyhow::bail!("no capture save for the anchor") };
                let log = std::fs::read_to_string(std::path::Path::new(&args[1]).join("log-stream.txt"))?;
                let undo = si_steam::log_parse::undo_write_times(std::path::Path::new(&args[1]));
                let rows = si_steam::backfill::rows(&log, anchor, anchor_ms, &undo);
                println!("anchor T{} {} · {} rows from the log", anchor.turn, anchor.step, rows.len());
                println!("  {:<4} {:<8} {:>6} {:>5} {:>5} {:>5} {:>6} {:>5} {:>5} {:>5} {:>4} {:>6} {:>5}  actual (X/B/R)", "turn", "source", "expl", "town", "city", "dahan", "blight", "card", "fear", "cards", "TL", "pres", "E/P");
                for r in rows.iter().chain(real.iter().filter(|r| r.turn < n)) {
                    println!(
                        "  {:<4} {:<8} {:>6} {:>5} {:>5} {:>5} {:>6} {:>5} {:>5} {:>5} {:>4} {:>6} {:>5}  {:?}",
                        r.turn, if r.from_log { "log" } else { "saves" }, r.explorers, r.towns, r.cities, r.dahan, r.blight_on_island, r.blight_remaining, r.fear_total, r.fear_cards_earned, r.terror_level,
                        format!("{}/{}", r.presence_on_island, r.presence_destroyed), format!("{}/{}", r.energy_income, r.plays), r.actual
                    );
                }
            } else if args[0] == "chain" {
                // The chain at a save: this turn's actions with what each
                // plants, the derived weights, and the pace turn by turn.
                let tl = c.timeline();
                let idx = at(tl);
                let st = si_common::companion_stats::backtest::race_at(tl, idx);
                let ch = &st.chain;
                let s = &tl[idx];
                println!("T{} {} · {} orders · weights Explore {:.2} Build {:.2} Ravage {:.2} · sources {} of {} lands, {} empty, R {:.2} · crossing {:?}",
                    s.turn, s.step, ch.orders, ch.weights[0], ch.weights[1], ch.weights[2], ch.sources.now, ch.sources.lands, ch.sources.empty, ch.sources.r, ch.crossing);
                for a in &ch.actions {
                    println!("  {:<7} {:<4} {:<8} {}/{} value {:>5.2} = actions {:.2} + blight {:.2} + dahan {:.2} + tail {:.2} · {}",
                        a.step, a.land, a.terrain, a.chance.0, a.chance.1, a.value, a.actions, a.blight, a.dahan, a.tail, a.plants.join(" · "));
                }
                for cnow in &ch.capacity_now {
                    println!("  {}: plays {} energy {} · best {:.2} ({:.2} a play) · {}", cnow.spirit, cnow.plays, cnow.energy, cnow.best, cnow.per_play,
                        cnow.cuts.iter().map(|(c, t, v)| format!("{c} → {t} {v:.2}")).collect::<Vec<_>>().join(" · "));
                }
                for h in st.chain.turns.iter().filter(|t| t.turn < s.turn) {
                    if let Some(r) = si_common::companion_stats::round_summaries(&tl[..=idx]).into_iter().find(|r| r.turn == h.turn) {
                        println!("  T{} set to act {:?} · did {:?} · next as advanced {:?}", r.turn, r.forecast_start, r.actual, r.forecast_next);
                    }
                }
                println!("  {:<4} {:>6} {:>6} {:>6} {:>8} {:>8} {:>5} {:>8} {:>6}  basis", "turn", "X", "B", "R", "pressure", "capacity", "plays", "backlog", "ratio");
                for t in &ch.turns {
                    println!("  {:<4} {:>6.1} {:>6.1} {:>6.1} {:>8.1} {:>8.1} {:>5} {:>8.1} {:>6}  {}{}",
                        t.turn, t.lands[0], t.lands[1], t.lands[2], t.pressure, t.capacity, t.plays, t.backlog,
                        t.ratio.map(|r| format!("{r:.2}")).unwrap_or_else(|| "-".into()), t.basis, if t.reclaim { " · Reclaim" } else { "" });
                }
            } else if args[0] == "outlook" {
                for t in si_common::companion_stats::backtest::outlook_check(c.timeline()) {
                    let ok = t.in_rows && t.explores_row == t.explores_actual;
                    println!(
                        "T{} {:<14} {} rows={} explores row={:?} actual={:?} | next turn unanswered {:?} actual {:?}",
                        t.turn,
                        t.card.clone().unwrap_or_default(),
                        if ok { "OK  " } else { "MISS" },
                        t.in_rows,
                        t.explores_row,
                        t.explores_actual,
                        t.next_unanswered,
                        t.next_actual
                    );
                }
            } else if args[0] == "backtest" {
                for t in si_common::companion_stats::backtest::backtest(c.timeline()) {
                    println!("Turn {}: card {:?}", t.turn, t.explore_card);
                    if !t.high_immigration_predicted.is_empty() || !t.high_immigration_actual.is_empty() {
                        println!("  high immig.    predicted {:?}\n                 actual    {:?}", t.high_immigration_predicted, t.high_immigration_actual);
                    }
                    println!("  ravage blight  predicted {:?}  actual {:?}", t.ravage_blight_predicted, t.ravage_blight_actual);
                    println!("  build          predicted {:?}\n                 actual    {:?}", t.builds_predicted, t.builds_actual);
                    if !t.escalation_predicted.is_empty() {
                        println!("  escalation     predicted {:?}\n                 actual    {:?}", t.escalation_predicted, t.escalation_actual);
                    }
                    println!("  explore        predicted {:?}\n                 actual    {:?}", t.explores_predicted, t.explores_actual);
                    for a in &t.rule_actions_actual {
                        println!("  rule action: {a}");
                    }
                    for s in &t.skipped_by_choice {
                        println!("  skipped by choice: {s}");
                    }
                    for m in &t.mismatches {
                        println!("  MISMATCH {m}");
                    }
                }
            } else if args[0] == "review" {
                let raw = si_steam::log_parse::capture_decisions(std::path::Path::new(&args[1]));
                let r = si_common::companion_stats::review::review(c.timeline(), &si_steam::projection::decisions(&raw, &defs));
                println!("{}", serde_json::to_string_pretty(&r)?);
            } else if moment.is_some() {
                // The stats at a moment, as JSON.
                let tl = c.timeline();
                let st = si_common::companion_stats::backtest::race_at(tl, at(tl));
                println!("{}", serde_json::to_string_pretty(&st)?);
            } else {
                let st = c.state(Default::default());
                println!("{}", serde_json::to_string_pretty(&st.stats)?);
            }
        }
        _ => eprintln!("usage: si-steam-dump check|hist|tree|snap|sum|adjprobe|dims|defs|pub|scan|review|stats|backtest|timeline …"),
    }
    Ok(())
}

fn print_node(n: Node, indent: usize, depth: usize) {
    let pad = "  ".repeat(indent);
    match n.object() {
        Some(Object::String(s)) => println!("{pad}\"{s}\""),
        Some(Object::Array { values, .. }) => {
            println!("{pad}[array len {}]", values.len());
            if depth > 0 {
                for (i, v) in values.iter().enumerate().take(40) {
                    print_value(n, &format!("[{i}]"), v, indent + 1, depth - 1);
                }
            }
        }
        Some(Object::Class { .. }) => {
            println!("{pad}<{}> #{}", n.full_class_name().unwrap_or("?"), n.id);
            if depth == 0 {
                return;
            }
            let Some(Object::Class { meta, values }) = n.object() else { return };
            for (name, v) in meta.members.iter().zip(values) {
                print_value(n, name, v, indent + 1, depth - 1);
            }
        }
        None => println!("{pad}(missing #{})", n.id),
    }
}

fn print_value(parent: Node, name: &str, v: &Value, indent: usize, depth: usize) {
    let pad = "  ".repeat(indent);
    match v {
        Value::Ref(id) => {
            let child = parent.graph.node(*id);
            match child.object() {
                Some(Object::String(s)) => println!("{pad}{name} = \"{s}\""),
                Some(Object::Class { meta, .. }) if depth == 0 => {
                    let size = child.raw("_size").and_then(|v| v.as_i64());
                    let extra = size.map(|s| format!(" size={s}")).unwrap_or_default();
                    let ident = child.str("Identifier").map(|s| format!(" id={s}")).unwrap_or_default();
                    println!("{pad}{name} = <{}> #{id}{extra}{ident}", nrbf::short_name(&meta.name));
                }
                Some(Object::Array { values, .. }) if depth == 0 => {
                    println!("{pad}{name} = [array len {}] #{id}", values.len())
                }
                _ => {
                    println!("{pad}{name} =");
                    print_node(child, indent + 1, depth);
                }
            }
        }
        other => println!("{pad}{name} = {other:?}"),
    }
}

fn summary(s: &si_steam::engine_map::EngineSnapshot) {
    use si_steam::engine_map::PieceKind::*;
    println!("game {} build {} turn {} state {}/{} diff {:?} adv {:?}", s.game_id, s.engine_build, s.turn_index, s.phase, s.state, s.difficulty, s.adversaries);
    println!("fear {}/{} terror {:?} blight {:?} ending {:?}", s.fear_generated, s.fear_required, s.terror_level_card, s.blight_card, s.ending);
    for l in &s.lands {
        let mut c = std::collections::BTreeMap::<String, i64>::new();
        for p in &l.pieces {
            let k = match &p.kind { Explorer => "E".into(), Town => "T".into(), City => "C".into(), Dahan => "D".into(), Blight => "B".into(), Presence(_) => "P".into(), Token(t) => t.clone(), other => format!("{other:?}") };
            *c.entry(k).or_default() += 1;
        }
        let adj: Vec<_> = s.adjacency.iter().filter_map(|(a, b)| if a == &l.id { Some(b.as_str()) } else if b == &l.id { Some(a.as_str()) } else { None }).collect();
        println!("  {:12} dmg{} def{} {:?}  adj {:?}", l.id, l.damage_taken, l.defend, c, adj);
    }
    let ids = |v: &[si_steam::engine_map::EngineCard]| v.iter().map(|c| format!("{}{}{}", c.id, if c.flipped {"^"} else {""}, c.override_stage.map(|s| format!("(s{s})")).unwrap_or_default())).collect::<Vec<_>>().join(" ");
    println!("invader deck [{}]: {}", s.invader_deck.len(), ids(&s.invader_deck));
    println!("explore revealed: {}  build: {}  ravage: {}  discard: {}", ids(&s.explore_slot_revealed), ids(&s.build_slot), ids(&s.ravage_slot), ids(&s.invader_discard));
    println!("extra slots: {:?}", s.extra_invader_slots.iter().map(|(k, v)| format!("{k}: {}", ids(v))).collect::<Vec<_>>());
    println!("fear deck [{}]: {}", s.fear_deck.len(), s.fear_deck.iter().map(|c| format!("{}:{}", c.class, c.id)).collect::<Vec<_>>().join(" "));
    println!("fear earned: {}  discard: {}", ids(&s.fear_earned), ids(&s.fear_discard));
    println!("event deck {} discard: {}", s.event_deck.len(), ids(&s.event_discard));
    println!("minor deck {} discard {}  major deck {} discard {}", s.minor_deck.len(), s.minor_discard.len(), s.major_deck.len(), s.major_discard.len());
    println!("out of game: {}", ids(&s.out_of_game));
    for sp in &s.spirits {
        println!("spirit {} E{} plays{} status {:?} elements {:?} markers {:?}", sp.id, sp.energy, sp.card_plays, sp.phase_status, sp.elements, sp.element_markers);
        println!("  hand {:?}\n  play {:?}\n  discard {:?}\n  forgotten {:?}", sp.hand, sp.play_area, sp.discard, sp.forgotten);
        println!("  track {:?}", sp.track_nodes);
        println!("  presence island {} destroyed {} innates {:?}", sp.presence_on_island, sp.presence_destroyed, sp.innates);
    }
    println!("status effects: {:?}", s.status_effects);
    let d = |a: &str, b: &str| s.distances.iter().find(|(x, y, _)| (x == a && y == b) || (x == b && y == a)).map(|t| t.2);
    println!("distances: C1-C2 {:?} C1-C8 {:?} C4-E8 {:?} C1-E1 {:?} OceanC0-C3 {:?} C2-C3 {:?} (pairs {})", d("JungleC1", "SandsC2"), d("JungleC1", "WetlandC8"), d("JungleC4", "WetlandE8"), d("JungleC1", "SandsE1"), d("OceanC0", "MountainC3"), d("SandsC2", "MountainC3"), s.distances.len());
    println!("unmapped: {:?}", s.unmapped);
}
