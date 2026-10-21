//! A save as a `Seed`: the one reader of an engine snapshot on the live path. Only
//! fields a player at the table can see are read (the firewall test
//! `seed_is_invariant_to_hidden_permutations` holds this): the board, each spirit's own
//! piles and Energy, the face-up slots and discard, the Blight card, the Fear counts,
//! the Terror Level. Never the Invader deck, the face-down Explore slot, the Fear, Event,
//! Minor or Major decks, or the names of Fear cards earned face down.

use std::collections::BTreeMap;

use si_model::game::Phase;
use si_model::island::LandId;
use si_record::replay::{Seed, SeedSpirit};
use si_record::Counts;

use crate::engine_map::{EngineCard, EngineSnapshot, PieceKind as K};

/// `MountainA4` / `A4` as the record names a land.
pub fn land_id_of(engine: &str) -> Option<LandId> {
    let short = ["Mountain", "Sands", "Jungle", "Wetland", "Ocean"].iter().find_map(|t| engine.strip_prefix(t)).unwrap_or(engine);
    short.parse().ok()
}

fn ids(v: &[EngineCard]) -> Vec<String> {
    v.iter().map(|c| c.id.clone()).collect()
}

pub fn from_snapshot(snap: &EngineSnapshot, save_name: &str) -> Seed {
    let spirit_of = |s: &str| -> String { snap.spirits.iter().find(|sp| sp.id == s).or_else(|| snap.spirits.iter().find(|sp| sp.id.contains(s) || s.contains(&sp.id))).map(|sp| sp.id.clone()).unwrap_or_else(|| s.to_string()) };
    let mut lands = Vec::new();
    for l in &snap.lands {
        let Some(id) = land_id_of(&l.id) else { continue };
        let mut c = Counts::default();
        let mut presence: BTreeMap<String, u32> = BTreeMap::new();
        for p in &l.pieces {
            match &p.kind {
                K::Explorer => c.explorers += 1,
                K::Town => c.towns += 1,
                K::City => c.cities += 1,
                K::Dahan => c.dahan += 1,
                K::Blight => c.blight += 1,
                K::Presence(s) => *presence.entry(spirit_of(s)).or_insert(0) += 1,
                K::Token(t) => match t.as_str() {
                    "Beast" | "Beasts" => c.beasts += 1,
                    "Disease" => c.disease += 1,
                    "Strife" => c.strife += 1,
                    "Wilds" => c.wilds += 1,
                    "Badlands" => c.badlands += 1,
                    _ => {}
                },
                _ => {}
            }
        }
        c.presence = presence.values().sum();
        lands.push((id, c, presence.into_iter().collect()));
    }
    let spirits = snap
        .spirits
        .iter()
        .map(|sp| SeedSpirit {
            id: sp.id.clone(),
            energy: sp.energy,
            hand: sp.hand.clone(),
            play: sp.play_area.clone(),
            discard: sp.discard.clone(),
            forgotten: sp.forgotten.clone(),
            revealed: sp.track_nodes.iter().filter(|(_, covered)| !covered).map(|(id, _)| id.clone()).collect(),
        })
        .collect();
    let mut seen: Vec<String> = ids(&snap.invader_discard);
    seen.extend(ids(&snap.ravage_slot));
    seen.extend(ids(&snap.build_slot));
    seen.extend(ids(&snap.explore_slot_revealed));
    seen.extend(snap.extra_invader_slots.iter().flat_map(|(_, v)| ids(v)));
    seen.sort();
    seen.dedup();
    let boards: Vec<String> = lands.iter().map(|(id, _, _)| id.board.clone()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
    // Terror Level cards leave the game as each level is reached
    let terror = 1 + snap.out_of_game.iter().filter(|c| c.id.starts_with("TerrorLevel")).count() as u8;
    let (adversary, adversary_level) = match snap.adversaries.first() {
        Some((id, level)) => (Some(id.clone()), level.map(|l| l as u8)),
        None => (None, None),
    };
    Seed {
        turn_index: snap.turn_index.max(0) as u32,
        phase: Phase::parse(&snap.state),
        boards,
        lands,
        spirits,
        terror,
        invader_cards_seen: seen,
        game_id: snap.game_id.clone(),
        save_name: save_name.to_string(),
        adversary,
        adversary_level,
        difficulty: snap.difficulty,
        scenario: snap.scenario.clone(),
        ravage_slot: ids(&snap.ravage_slot),
        build_slot: ids(&snap.build_slot),
        extra_slots: snap.extra_invader_slots.iter().map(|(name, v)| (name.clone(), ids(v))).collect(),
        // the card's name is hidden until it flips (its healthy side is the same for all)
        blight_card: snap.blight_card.as_ref().filter(|b| b.flipped).map(|b| b.id.clone()),
        blight_pool: snap.blight_card.as_ref().map(|b| b.blight_remaining),
        blight_flipped: snap.blight_card.as_ref().is_some_and(|b| b.flipped),
        fear_generated: snap.fear_generated,
        fear_required: snap.fear_required,
        fear_cards_banked: snap.fear_earned.len() as u32,
        ending: snap.ending.clone(),
    }
}
