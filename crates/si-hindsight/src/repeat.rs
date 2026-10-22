//! Repeats. A Power that says "Repeat this Power" is used again once it resolves, a new use
//! and Action of its own (core rulebook p.19, "Repeat"; Jagged Earth rulebook, "Repeats").
//! A line carries each Repeat as a later play of the same Power (`CTM>A8 CTM>A8 CTM>A3`),
//! as the journal records it: it costs no Energy, gives no Elements, is no card played,
//! keeps the Power's speed, and may choose another land. Where the Repeat says where (an
//! adjacent land, a different land within 3), it goes there instead of the usual Range and
//! Target. Repeats do not chain: a Repeat's own "Repeat this Power" is ignored, so a Power
//! is used once plus once for each Repeat its first use reaches.

use std::collections::BTreeMap;

use si_defs::Definitions;
use si_model::island::{Island as Graph, LandId, Terrain};
use si_model::power::{Effect, RepeatAt};
use si_model::ElementSet;
use si_rules::{effects, verify};

use crate::Play;

/// Is the `i`th play a Repeat: a Power an earlier play of the line already used?
pub fn is_repeat(plays: &[Play], i: usize) -> bool {
    plays.get(i).is_some_and(|p| plays[..i].iter().any(|q| q.power == p.power))
}

/// The plays that are not Repeats: the cards played and the innates used, once each.
pub fn first_uses(plays: &[Play]) -> Vec<&Play> {
    (0..plays.len()).filter(|i| !is_repeat(plays, *i)).map(|i| &plays[i]).collect()
}

/// The Repeats a Power's text reaches with these elements, each with where it may go.
pub fn reached(defs: &Definitions, power: &str, elements: &ElementSet) -> Vec<RepeatAt> {
    let Some((base, thresholds)) = verify::power_effects(defs, power) else { return Vec::new() };
    effects::reached_effects(&base, &thresholds, elements).iter().filter_map(|e| if let Effect::Repeat(at) = e { Some(*at) } else { None }).collect()
}

/// Where the Power's printed Repeats go, whatever the elements (no Power prints two places).
pub fn place(defs: &Definitions, power: &str) -> Option<RepeatAt> {
    let (base, thresholds) = verify::power_effects(defs, power)?;
    base.iter().chain(thresholds.iter().flat_map(|(_, e)| e.iter())).find_map(|e| if let Effect::Repeat(at) = e { Some(*at) } else { None })
}

/// The lands a Repeat may take, from the land its Power's first use took: the Power's own
/// targets (`usual`) for a plain Repeat, else where the Repeat says. Never an Ocean.
pub fn lands(at: RepeatAt, first: &LandId, usual: &[LandId], graph: &Graph, presence: &BTreeMap<LandId, u32>) -> Vec<LandId> {
    let land = |id: &LandId| graph.lands.get(id).is_some_and(|l| l.terrain != Terrain::Ocean);
    match at {
        RepeatAt::Usual => usual.to_vec(),
        RepeatAt::Adjacent => graph.lands.get(first).map(|l| l.adjacent.iter().filter(|a| land(a)).cloned().collect()).unwrap_or_default(),
        RepeatAt::Within(n) => graph.lands.keys().filter(|id| *id != first && land(id) && graph.distance(first, id, true).is_some_and(|d| d <= n as u32)).cloned().collect(),
        RepeatAt::TargetOrPresence => {
            let mut v = vec![first.clone()];
            v.extend(presence.iter().filter(|(id, n)| **n > 0 && *id != first).map(|(id, _)| id.clone()));
            v
        }
    }
}

/// The lines the plays make with each Power's Repeats added after its first use: as many as
/// its tiers reach beyond the uses the line already has. A Slow Repeat goes with no land,
/// to take one after the Invader steps (`resolve_slow` keeps it where the Repeat may go);
/// a Fast Repeat at each land it may take, one line for each, at most `cap` lines.
/// `usual` gives a play's own targets.
pub fn expand(defs: &Definitions, plays: &[Play], elements: &ElementSet, usual: &dyn Fn(&Play) -> Vec<LandId>, graph: &Graph, presence: &BTreeMap<LandId, u32>, cap: usize) -> Vec<Vec<Play>> {
    let mut lines: Vec<Vec<Play>> = vec![plays.to_vec()];
    for (i, p) in plays.iter().enumerate() {
        if p.inert || is_repeat(plays, i) {
            continue;
        }
        let have = plays.iter().filter(|q| q.power == p.power).count() - 1;
        for at in reached(defs, &p.power, elements).into_iter().skip(have) {
            let repeat = Play { target: None, cost: 0, moves: Vec::new(), exact: false, kills: None, recorded: false, ..p.clone() };
            if !p.fast {
                for l in lines.iter_mut() {
                    l.push(repeat.clone());
                }
                continue;
            }
            let Some(first) = &p.target else { continue };
            let options = lands(at, first, &usual(p), graph, presence);
            let mut next: Vec<Vec<Play>> = Vec::new();
            for l in &lines {
                for tg in &options {
                    if next.len() >= cap.max(1) {
                        break;
                    }
                    let mut l2 = l.clone();
                    l2.push(Play { target: Some(tg.clone()), ..repeat.clone() });
                    next.push(l2);
                }
            }
            if !next.is_empty() {
                lines = next;
            }
        }
    }
    lines
}

/// The law of a line's Repeats: a Repeat costs nothing and keeps the Power's speed; a Power
/// that prints a Repeat is used no more than once plus the Repeats its tiers reach, and a
/// Repeat that names its place goes there from the land the first use took. A Power used
/// again that prints none is another's grant (Gift of Twinned Days' "Target Spirit may
/// Repeat") or its own rule (Slip the Flow of Time "any number of times"), not counted here;
/// plays as the journal recorded them are not held to it.
pub fn holds(defs: &Definitions, plays: &[Play], elements: &ElementSet, graph: &Graph) -> Vec<si_model::oracle::Violation> {
    use si_model::oracle::check;
    let mut out = Vec::new();
    for (i, p) in plays.iter().enumerate() {
        if p.recorded || !is_repeat(plays, i) {
            continue;
        }
        let first = plays.iter().find(|q| q.power == p.power).unwrap();
        let k = plays[..i].iter().filter(|q| q.power == p.power).count();
        let allowed = reached(defs, &p.power, elements);
        check(&mut out, p.cost == 0, "a Repeat costs nothing", || format!("{} repeated for {} Energy", p.power, p.cost));
        if place(defs, &p.power).is_some() {
            check(&mut out, k <= allowed.len(), "no more Repeats than reached", || format!("{} used {} times, its tiers reach {} Repeat(s)", p.power, k + 1, allowed.len()));
        }
        check(&mut out, p.fast == first.fast, "a Repeat keeps the Power's speed", || format!("{} first {} then {}", p.power, first.fast, p.fast));
        if let (Some(at), Some(from), Some(to)) = (allowed.get(k - 1), &first.target, &p.target) {
            let ok = match at {
                RepeatAt::Usual | RepeatAt::TargetOrPresence => true,
                RepeatAt::Adjacent => graph.lands.get(from).is_some_and(|l| l.adjacent.contains(to)),
                RepeatAt::Within(n) => to != from && graph.distance(from, to, true).is_some_and(|d| d <= *n as u32),
            };
            check(&mut out, ok, "a Repeat goes where it says", || format!("{} repeated {:?} from {from} at {to}", p.power, at));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use si_model::oracle::{property, Rng};

    fn defs() -> Option<Definitions> {
        let d = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/engine-defs");
        d.is_dir().then(|| Definitions::from_dir(&d).expect("the engine definitions load"))
    }

    fn play(power: &str, target: Option<LandId>, fast: bool, innate: bool) -> Play {
        Play { power: power.into(), target, fast, innate, cost: 0, places: false, forced: false, moves: Vec::new(), exact: false, inert: false, recorded: false, kills: None }
    }

    fn board_a(defs: &Definitions) -> Graph {
        Graph::from_boards(&[defs.board_def("A").expect("board A").clone()])
    }

    #[test]
    fn creepers_at_three_tiers_is_used_three_times() {
        let Some(defs) = defs() else { return };
        let graph = board_a(&defs);
        let creepers = play("CreepersTearIntoMortar", None, false, true);
        let mut els = ElementSet::new();
        for (e, n) in [(si_model::element::Element::Moon, 3), (si_model::element::Element::Plant, 4)] {
            els.add(e, n);
        }
        let lines = expand(&defs, std::slice::from_ref(&creepers), &els, &|_| vec![], &graph, &BTreeMap::new(), 8);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].iter().map(|p| (p.power.as_str(), p.target.is_none())).collect::<Vec<_>>(), vec![("CreepersTearIntoMortar", true); 3], "the first use and its two Repeats, each to take its land after the Invader steps");
        assert!(holds(&defs, &lines[0], &els, &graph).is_empty());
        // two tiers: one Repeat; a third use breaks the law
        let mut two = ElementSet::new();
        two.add(si_model::element::Element::Moon, 2);
        two.add(si_model::element::Element::Plant, 3);
        assert_eq!(expand(&defs, std::slice::from_ref(&creepers), &two, &|_| vec![], &graph, &BTreeMap::new(), 8)[0].len(), 2);
        assert!(!holds(&defs, &lines[0], &two, &graph).is_empty(), "three uses on two tiers");
    }

    #[test]
    fn a_fast_repeat_on_an_adjacent_land_takes_each_neighbor() {
        let Some(defs) = defs() else { return };
        let graph = board_a(&defs);
        // Insatiable Hunger of the Swarm: 2 Air + 4 Animal repeats it on an adjacent land
        let first = play("InsatiableHungerOfTheSwarm", Some(LandId::new("A", 5)), true, false);
        let card = defs.cards.get("InsatiableHungerOfTheSwarm").unwrap();
        let els = card.thresholds.iter().find(|t| t.effects.iter().any(|e| matches!(e, Effect::Repeat(_)))).unwrap().elements.clone();
        let lines = expand(&defs, std::slice::from_ref(&first), &els, &|_| vec![], &graph, &BTreeMap::new(), 16);
        let neighbors: Vec<LandId> = graph.lands[&LandId::new("A", 5)].adjacent.iter().filter(|a| graph.lands[*a].terrain != Terrain::Ocean).cloned().collect();
        assert_eq!(lines.len(), neighbors.len());
        for l in &lines {
            assert!(neighbors.contains(l[1].target.as_ref().unwrap()));
            assert!(holds(&defs, l, &els, &graph).is_empty(), "{l:?}");
        }
        // the same land again is not adjacent to itself
        let wrong = vec![first.clone(), Play { cost: 0, ..first.clone() }];
        assert!(!holds(&defs, &wrong, &els, &graph).is_empty());
        // below the threshold no Repeat is added
        assert_eq!(expand(&defs, &[first], &card.elements, &|_| vec![], &graph, &BTreeMap::new(), 16), vec![vec![play("InsatiableHungerOfTheSwarm", Some(LandId::new("A", 5)), true, false)]]);
    }

    #[test]
    fn a_repeat_is_no_card_played_and_no_elements_again() {
        let plays = vec![play("SeaMonsters", Some(LandId::new("A", 2)), false, false), play("SeaMonsters", Some(LandId::new("A", 1)), false, false)];
        assert!(!is_repeat(&plays, 0) && is_repeat(&plays, 1));
        assert_eq!(first_uses(&plays).len(), 1);
    }

    /// Random Powers with random elements, expanded: every line keeps the law.
    #[test]
    fn every_expanded_line_keeps_the_repeat_law() {
        let Some(defs) = defs() else { return };
        let graph = board_a(&defs);
        let repeating: Vec<String> = defs.cards.keys().chain(defs.spirits.values().flat_map(|s| s.innates.iter().map(|i| &i.id))).filter(|id| verify::power_effects(&defs, id).is_some_and(|(b, th)| b.iter().chain(th.iter().flat_map(|(_, e)| e.iter())).any(|e| matches!(e, Effect::Repeat(_))))).cloned().collect();
        assert!(repeating.len() >= 12, "{repeating:?}");
        let lands: Vec<LandId> = graph.lands.keys().filter(|id| graph.lands[*id].terrain != Terrain::Ocean).cloned().collect();
        property(
            "repeats",
            5,
            2000,
            |rng: &mut Rng| {
                let power = repeating[rng.below(repeating.len())].clone();
                let mut els = ElementSet::new();
                for e in si_model::element::Element::ALL {
                    els.add(e, rng.between(0, 5) as u8);
                }
                let fast = rng.coin();
                let target = if fast || rng.coin() { Some(lands[rng.below(lands.len())].clone()) } else { None };
                let innate = !defs.cards.contains_key(&power);
                (play(&power, target, fast, innate), els)
            },
            |(p, els)| {
                let usual = |_: &Play| lands.clone();
                let presence: BTreeMap<LandId, u32> = [(LandId::new("A", 3), 2)].into_iter().collect();
                expand(&defs, std::slice::from_ref(p), els, &usual, &graph, &presence, 8).iter().flat_map(|l| holds(&defs, l, els, &graph)).collect()
            },
        );
    }
}
