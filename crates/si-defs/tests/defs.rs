//! The loader against a slice of the digital edition's definitions
//! (`tests/fixtures/engine`, written 2026-10-20 from `si-steam-dump defs`).

use std::collections::BTreeSet;
use std::path::Path;

use si_defs::Definitions;
use si_model::spirit::{Placement, Reclaim, RuleHook, Spike};
use si_model::{Element, PieceKind, Terrain};

fn defs() -> Definitions {
    Definitions::from_dir(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/engine"))).expect("fixtures load")
}

fn reveal(panel: &si_model::SpiritPanel, ids: &[&str]) -> BTreeSet<String> {
    let mut r = panel.starting_revealed();
    for id in ids {
        assert!(panel.tracks.iter().any(|n| n.id == *id), "no slot {id}");
        r.insert(id.to_string());
    }
    r
}

#[test]
fn thunderspeaker_tracks_growth_and_rules() {
    let d = defs();
    let ts = &d.spirits["Thunderspeaker"];
    let start = ts.track_state(&ts.starting_revealed());
    assert_eq!((start.income, start.plays), (1, 1), "Thunderspeaker starts 1 Energy, 1 Card Play");
    // Energy track: 1, Air, 2, Fire, Sun, 3. Plays track: 1, 2, 2, 3, Reclaim 1, 3, 4.
    let s = ts.track_state(&reveal(ts, &["ThunderspeakerEnergy2", "ThunderspeakerEnergy3", "ThunderspeakerCardPlays2", "ThunderspeakerCardPlays3", "ThunderspeakerCardPlays4", "ThunderspeakerCardPlays5"]));
    assert_eq!((s.income, s.plays), (2, 3));
    assert_eq!(s.elements.count(Element::Air), 1);
    assert!(s.reclaim_one, "the fifth plays slot is Reclaim 1");
    assert!(Spike::new(2, 3, "test").met_by(&s));
    // Growth 3 = add a presence + 4 Energy; Growth 2 = two presence into lands with Dahan.
    let g3 = ts.growth.groups.iter().find(|g| g.number == 3).unwrap().yield_();
    assert_eq!((g3.presence, g3.energy), (1, 4));
    let g2 = ts.growth.groups.iter().find(|g| g.number == 2).unwrap().yield_();
    assert_eq!(g2.presence, 2);
    assert!(matches!(g2.placement, Some(Placement::WithDahan { .. })));
    let g1 = ts.growth.groups.iter().find(|g| g.number == 1).unwrap().yield_();
    assert_eq!((g1.reclaim, g1.cards), (Some(Reclaim::All), 2));
    // Special rules carry their typed hooks.
    assert!(ts.hook(|h| matches!(h, RuleHook::PresenceMovesWithDahan)).is_some());
    assert!(ts.hook(|h| matches!(h, RuleHook::PresenceDestroyedPerDahanKilled { range: 1 })).is_some());
    // Innates: two, both Slow; Gather the Warriors range 1.
    assert_eq!(ts.innates.len(), 2);
    let gather = ts.innates.iter().find(|i| i.title == "Gather the Warriors").unwrap();
    assert_eq!(gather.range.as_ref().map(|r| r.distance), Some(1));
    assert!(gather.tiers.iter().any(|t| t.modifier && t.elements.count(Element::Air) == 4), "4 Air makes it Fast");
    assert_eq!(ts.uniques.len(), 4);
}

#[test]
fn volcano_placement_and_range_rules() {
    let d = defs();
    let v = &d.spirits["VolcanoLoomingHigh"];
    assert_eq!(v.hook(|h| matches!(h, RuleHook::PlacementOnly(_))), Some(&RuleHook::PlacementOnly(vec![Terrain::Mountain])));
    assert_eq!(v.hook(|h| matches!(h, RuleHook::RangeBonusFromOrigin { .. })), Some(&RuleHook::RangeBonusFromOrigin { presence: 3, bonus: 1 }));
    let bomb = d.card_by_title("Pyroclastic Bombardment").unwrap();
    let r = bomb.range.as_ref().unwrap();
    assert!(r.from_sacred_site && r.distance == 2);
    assert_eq!(bomb.cost, 3);
    // Growth 3 adds a card, a presence at range 4, a card play this turn and 2 Energy.
    let g3 = v.growth.groups.iter().find(|g| g.number == 3).unwrap().yield_();
    assert_eq!((g3.cards, g3.presence, g3.plays_this_turn, g3.energy), (1, 1, 1, 2));
    assert_eq!(g3.placement, Some(Placement::Any { range: 4 }));
}

#[test]
fn shroud_many_minds_and_vengeance_track_unlocks() {
    let d = defs();
    let sh = &d.spirits["ShroudOfSilentMist"];
    assert!(sh.tracks.iter().any(|n| n.growth_option.as_ref().is_some_and(|g| g.kind == "MovePresence")), "Shroud's plays track grants Move a Presence");
    assert!(sh.tracks.iter().any(|n| n.reclaim_one), "Shroud has a Reclaim 1 slot");
    let start = sh.track_state(&sh.starting_revealed());
    assert_eq!((start.income, start.plays), (0, 1));
    let spike = Spike::new(1, 3, "Rei: Shroud 1/3cp");
    let s = sh.track_state(&reveal(sh, &["ShroudOfSilentMistEnergy2", "ShroudOfSilentMistCardPlays2", "ShroudOfSilentMistCardPlays3", "ShroudOfSilentMistCardPlays4", "ShroudOfSilentMistCardPlays5"]));
    assert!(spike.met_by(&s), "{s:?}");

    let mm = &d.spirits["ManyMindsMoveAsOne"];
    assert_eq!(mm.tracks.iter().find_map(|n| n.pay_to_gain), Some(2), "pay 2 Energy to gain a Power Card");

    let vg = &d.spirits["VengeanceAsABurningPlague"];
    let s = vg.track_state(&reveal(vg, &["VengeanceAsABurningPlagueEnergy2", "VengeanceAsABurningPlagueEnergy3", "VengeanceAsABurningPlagueEnergy4", "VengeanceAsABurningPlagueCardPlays2", "VengeanceAsABurningPlagueCardPlays3", "VengeanceAsABurningPlagueCardPlays4", "VengeanceAsABurningPlagueCardPlays5"]));
    assert!(Spike::new(3, 3, "Rei: Vengeance 3/3cp").met_by(&s), "{s:?}");
    assert_eq!(s.elements.count(Element::Animal) + s.elements.count(Element::Fire), 2);
}

#[test]
fn boards_invader_cards_and_adversaries() {
    let d = defs();
    let a = &d.boards["A"];
    assert_eq!(a.lands.len(), 9);
    let land = |n: u8| a.lands.iter().find(|l| l.id.number == n).unwrap();
    assert_eq!(land(2).initial.cities, 1);
    assert_eq!(land(2).initial.dahan, 1);
    assert_eq!(land(8).initial.towns, 1);
    assert_eq!(land(4).initial.blight, 1);
    assert!(land(2).coastal() && !land(8).coastal());
    let island = si_model::Island::from_boards(&[a.clone()]);
    let a2 = island.get(&si_model::LandId::new("A", 2)).unwrap();
    assert_eq!(a2.count(PieceKind::City), 1);
    // adjacency is symmetric on the board
    for l in &a.lands {
        for n in &l.adjacent {
            assert!(land(*n).adjacent.contains(&l.id.number), "A{} -> A{n} not symmetric", l.id.number);
        }
    }
    assert_eq!(d.invader_cards.len(), 15);
    let by_stage = |s: u8| d.invader_cards.values().filter(|c| c.stage == s).count();
    assert_eq!((by_stage(1), by_stage(2), by_stage(3)), (4, 5, 6));
    assert!(d.invader_cards["Coastal2"].matches(Terrain::Sands, true));

    let hab = &d.adversaries["HabsburgMiningExpedition"];
    assert_eq!(hab.level(2).map(|l| l.difficulty), Some(4));
    assert_eq!(hab.level(2).map(|l| l.fear_modifiers), Some([0, 0, 1]));
    assert_eq!(hab.level(1).map(|l| l.extra.len()), Some(1), "Ceaseless Mining rides on level 1");
    assert_eq!(hab.effects_through(3).len(), 3);
}

#[test]
fn power_cards_carry_thresholds_ranges_and_targets() {
    let d = defs();
    let manifest = d.card_by_title("Manifestation of Power and Glory").unwrap();
    assert_eq!(manifest.cost, 3);
    assert_eq!(manifest.speed, si_model::Speed::Slow);
    assert!(manifest.target.requires_dahan());
    assert_eq!(manifest.range.as_ref().map(|r| r.distance), Some(0));
    assert_eq!(manifest.elements.total(), 3);
    assert!(d.cards.values().any(|c| c.element_thresholds().next().is_some()), "some fixture card has an element threshold");
    assert!(d.cards.values().any(|c| c.target.is_spirit() && c.range.is_none()), "spirit-targeting cards have no land range");
    assert!(d.cards.values().any(|c| c.range.as_ref().is_some_and(|r| !r.origin_terrains.is_empty())), "some card ranges from a terrain");
}

#[test]
fn card_text_parses_into_slot_capabilities() {
    use si_model::power::{Effect, Slot};
    let d = defs();
    let by = |t: &str| d.card_by_title(t).unwrap_or_else(|| panic!("no card {t}"));
    let strike = by("Strike Low with Sudden Fevers");
    assert_eq!(strike.effects, vec![Effect::Fear(1), Effect::SkipRavage]);
    assert!(strike.acts_on(Slot::Ravage) && !strike.acts_on(Slot::Build));
    let fetid = by("Fetid Breath Spreads Infection");
    assert!(fetid.acts_on(Slot::Build) && !fetid.acts_on(Slot::Ravage), "Disease stops a Build, not a Ravage");
    let tide = by("A Dreadful Tide of Scurrying Flesh");
    assert!(tide.fully_parsed(), "{:?}", tide.unparsed());
    assert!(tide.acts_on(Slot::Explore) && tide.acts_on(Slot::Build) && tide.acts_on(Slot::Ravage));
    let plague = by("Plaguebearers");
    assert!(plague.acts_on(Slot::Build) && plague.acts_on(Slot::Ravage));
    let words = by("Words of Warning");
    assert!(words.acts_on(Slot::Ravage) && !words.acts_on(Slot::Build));
    let voice = by("Voice of Thunder");
    assert!(!voice.acts_on(Slot::Ravage), "pushing Dahan saves Dahan, not the land");
    let wash = by("Wash Away");
    assert!(wash.acts_on(Slot::Build) && wash.acts_on(Slot::Ravage));
    let f = by("Dissolving Vapors").features();
    assert_eq!((f.fear, f.damage_each), (1, 2));
    let parsed = d.cards.values().filter(|c| c.fully_parsed()).count();
    assert!(parsed * 10 >= d.cards.len() * 6, "{parsed} of {} fixture cards fully parsed", d.cards.len());
}
