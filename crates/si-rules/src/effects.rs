//! Spirit actions: the parsed `Effect` language executed on a land. Where the
//! rules leave a choice to the player (which Invaders take the Damage, which
//! pieces are Pushed, which branch of an "or"), the executor takes a stated
//! default and says so, so the verifier can tell a choice from a rule gap.

use serde::{Deserialize, Serialize};
use si_model::island::{LandId, PieceKind, Terrain, TokenKind};
use si_model::power::{DamageTarget, Effect, PieceFilter};
use si_model::ElementSet;

use crate::invaders::{Island, Land};

/// One piece a Power moved: gathered into its land (`to` is the target) or pushed out of
/// it (`from` is the target). In a line's request `piece` is None: the effect's own kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Move {
    pub from: LandId,
    pub to: LandId,
    #[serde(default)]
    pub piece: Option<PieceKind>,
    #[serde(default = "one")]
    pub n: u32,
}

fn one() -> u32 {
    1
}

/// What a Power did to its target land.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Applied {
    pub damage: u32,
    pub destroyed: Vec<PieceKind>,
    /// Pieces Removed (not Destroyed: no Fear, no "destroyed" triggers).
    #[serde(default)]
    pub removed: Vec<PieceKind>,
    pub defend: u32,
    pub pushed: u32,
    /// What was pushed, by kind (Invaders out of a land matter; Dahan pushes do not).
    pub pushed_pieces: Vec<PieceKind>,
    pub gathered: u32,
    pub blight_removed: u32,
    pub tokens_added: u32,
    /// Presence the Power added to the island.
    #[serde(default)]
    pub presence_added: u32,
    pub fear: u32,
    /// Ravages the Power forced (Manifest Incarnation), with what each did.
    #[serde(default)]
    pub ravaged: Vec<crate::invaders::Outcome>,
    /// Triggers the Power armed, to fire on later plays this turn.
    #[serde(default)]
    pub triggers: Vec<TriggerArmed>,
    /// Token types a `TokenChoice` offered, for the `AddChosenToken` that follows.
    #[serde(default)]
    pub token_options: Vec<TokenKind>,
    /// Every piece gathered into or pushed out of the land, with where it came from or went.
    #[serde(default)]
    pub moves: Vec<Move>,
    /// Presence that went along with the Dahan this Power moved (Ally of the Dahan).
    #[serde(default)]
    pub presence_moved: u32,
    pub notes: Vec<String>,
}

/// A trigger a Power armed in a land ("the next time an Invader is destroyed here, 2 Fear").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerArmed {
    pub when: String,
    pub then: Vec<Effect>,
    pub once: bool,
    pub land: LandId,
}

/// Per-piece health in the land (England 5 and other health rules are the
/// caller's business through `health_bonus`).
pub struct Ctx {
    pub health_bonus: u32,
    /// The spirit's elements this turn (track plus cards), for thresholds.
    pub elements: ElementSet,
    /// Blight counts as Badlands for this spirit's actions (Vengeance).
    pub blight_as_badlands: bool,
    /// Adversary rules, for a Ravage the Power forces (None: not simulated).
    pub rules: Option<crate::invaders::Rules>,
    /// Where the player put this Power's spread pieces this turn, as the log
    /// recorded the choice: (the engine's reason such as `AddingDisease`, land).
    pub placements: Vec<(String, LandId)>,
    /// The piece and branch choices the log recorded for this Power: (reason such as `AddTokenOfType`, selected).
    pub picks: Vec<(String, String)>,
    /// The gathers and pushes the line asks for (`GW>A1[<A3 >A4]`): a source or a
    /// destination the Power may use is taken before any guess.
    pub moves: Vec<Move>,
    /// `moves` is all the Power moves (the record's, or a bracket the player wrote): no
    /// guessed gather or push beyond it.
    pub moves_exact: bool,
    /// The "or" branches the log recorded this Power taking, by printed order.
    pub branches: Vec<usize>,
    /// The spirit's presence goes along with each Dahan moved out of its land, one per
    /// Dahan (Thunderspeaker's Ally of the Dahan; the player may decline, and never has
    /// in the recorded games).
    pub presence_with_dahan: bool,
}

impl Ctx {
    /// A context with nothing recorded and no spirit rules: the elements and Health only.
    pub fn plain(health_bonus: u32, elements: ElementSet) -> Ctx {
        Ctx { health_bonus, elements, blight_as_badlands: false, rules: None, placements: vec![], picks: vec![], moves: vec![], moves_exact: false, branches: vec![], presence_with_dahan: false }
    }
}

/// Shroud's Mists Shift and Flow: 1 presence gathered into `into` before a Power acts,
/// from the adjacent land holding the most (the player's choice). False when none is next door.
pub fn gather_presence(island: &mut Island, into: &LandId) -> bool {
    let Some(adj) = island.lands.get(into).map(|l| l.adjacent.clone()) else { return false };
    let Some(from) = adj.iter().filter(|a| island.lands.get(*a).is_some_and(|l| l.c.presence > 0)).max_by_key(|a| island.lands.get(*a).map(|l| l.c.presence).unwrap_or(0)).cloned() else { return false };
    if let Some(l) = island.lands.get_mut(&from) {
        l.c.presence -= 1;
    }
    if let Some(l) = island.lands.get_mut(into) {
        l.c.presence += 1;
    }
    true
}

/// A Dahan moved from `from` to `to`: the spirit's presence goes along when it may.
fn presence_along(island: &mut Island, from: &LandId, to: &LandId, ctx: &Ctx, a: &mut Applied) {
    if !ctx.presence_with_dahan {
        return;
    }
    let Some(src) = island.lands.get_mut(from) else { return };
    if src.c.presence == 0 {
        return;
    }
    src.c.presence -= 1;
    if let Some(dst) = island.lands.get_mut(to) {
        dst.c.presence += 1;
    }
    if a.presence_moved == 0 {
        a.notes.push("presence moved with the Dahan (Ally of the Dahan; the player may keep it)".into());
    }
    a.presence_moved += 1;
}

/// How many a "per" counts in the land.
fn per_n(per: &PieceFilter, island: &Island, target: &LandId, ctx: &Ctx) -> u32 {
    match per {
        PieceFilter::Other(desc) => per_count_other(desc, island, target, ctx),
        _ => island.lands.get(target).map(|l| count_of(l, per)).unwrap_or(0),
    }
}

/// The one Damage amount a "per" holds, as the engine deals it: `N Damage per X`, and
/// a "per" inside a "per" (`each Dahan deals Damage equal to your presence`) multiplied out.
fn per_damage(then: &[Effect], island: &Island, target: &LandId, ctx: &Ctx) -> Option<(u32, DamageTarget)> {
    match then {
        [Effect::Damage { amount, to }] => Some((*amount as u32, *to)),
        [Effect::Per { per, then }] => per_damage(then, island, target, ctx).map(|(n, to)| (n * per_n(per, island, target, ctx), to)),
        _ => None,
    }
}

/// Does the effect list move Dahan only by the player's say (a Dahan push or gather)?
fn moves_dahan(es: &[Effect]) -> bool {
    es.iter().any(|e| match e {
        Effect::Push { what, .. } | Effect::Gather { what, .. } => what.iter().any(|f| matches!(f, PieceFilter::Piece(PieceKind::Dahan))),
        Effect::Per { then, .. } | Effect::Conditional { then, .. } => moves_dahan(then),
        _ => false,
    })
}

/// Which branch of an "or" the line takes: the one the log recorded (by its printed order,
/// else by its words), else the one whose
/// gathers or pushes the line asks for, else the first that does not wait on a Dahan move
/// the line does not ask for (Voice of Thunder without `[>A4]` takes its Fear).
fn choose_branch(branches: &[Vec<Effect>], target: &LandId, ctx: &Ctx) -> usize {
    if let Some(i) = ctx.branches.iter().copied().find(|i| *i < branches.len()) {
        return i;
    }
    let kind_of = |b: &[Effect]| -> &'static str {
        match b.first() {
            Some(Effect::Push { .. }) | Some(Effect::Per { .. }) if moves_dahan(b) => "push",
            Some(Effect::Gather { .. }) => "gather",
            Some(Effect::Fear(_)) | Some(Effect::FearIfInvaders(_)) => "fear",
            Some(Effect::Defend(_)) => "defend",
            Some(Effect::Damage { .. }) => "damage",
            _ => "",
        }
    };
    for (_, picked) in &ctx.picks {
        let w = picked.to_ascii_lowercase();
        if let Some(i) = branches.iter().position(|b| {
            let k = kind_of(b);
            !k.is_empty() && w.starts_with(k) || (k == "fear" && w.contains("{fear}") && !w.contains("push") && !w.contains("gather"))
        }) {
            return i;
        }
    }
    let asked = ctx.moves.iter().any(|m| m.from == *target || m.to == *target);
    if asked {
        if let Some(i) = branches.iter().position(|b| moves_dahan(b)) {
            return i;
        }
    }
    branches.iter().position(|b| !moves_dahan(b)).unwrap_or(0)
}

fn condition_holds(cond: &str, l: &Land) -> Option<bool> {
    let c = cond.to_ascii_lowercase();
    if c.contains("invaders are present") {
        return Some(l.c.invaders() > 0);
    }
    if c.contains("no invaders are present") || c.contains("no invaders") {
        return Some(l.c.invaders() == 0);
    }
    if c.contains("target land has {dahan}") || c.contains("{dahan} are present") {
        return Some(l.c.dahan > 0);
    }
    if c.contains("target land has {town}/{city}") {
        return Some(l.c.buildings() > 0);
    }
    if c.contains("target land has {beast}") || c.contains("a land with {beast}") {
        return Some(l.c.beasts > 0);
    }
    if c.contains("target land has {badlands}") {
        return Some(l.c.badlands > 0);
    }
    if c.contains("has no {blight}") || c.contains("doesn't have any") {
        return Some(l.c.blight == 0);
    }
    if c.contains("a {beast} can be gathered") {
        return Some(true);
    }
    if c.contains("target land is coastal") {
        return Some(l.coastal);
    }
    for (name, t) in [("{jungle", Terrain::Jungle), ("{mountain", Terrain::Mountain), ("{sands", Terrain::Sands), ("{wetland", Terrain::Wetland)] {
        if c.contains("target land is ") && c.contains(name) {
            // "{JungleWetland}" lists several terrains
            let terrains: Vec<Terrain> = ["jungle", "mountain", "sands", "wetland"].iter().filter(|w| c.contains(*w)).filter_map(|w| Terrain::parse(&format!("{}{}", w[..1].to_uppercase(), &w[1..]))).collect();
            let _ = t;
            return Some(l.terrain.is_some_and(|lt| terrains.contains(&lt)));
        }
    }
    None
}

/// How many of a described thing a "for each" counts when the description is
/// not a plain piece: element sets the spirit has, Blight in or adjacent to the
/// land, adjacent lands with presence, a token past the first, a list of tokens.
fn per_count_other(desc: &str, island: &Island, target: &LandId, ctx: &Ctx) -> u32 {
    let lower = desc.to_ascii_lowercase();
    let Some(l) = island.lands.get(target) else { return 0 };
    if let Some(spec) = lower.strip_prefix("elements:") {
        // "2 {moon} 2 {earth}": how many whole sets the spirit's elements hold
        let mut sets = u32::MAX;
        let mut pending = 1u32;
        for w in spec.split_whitespace() {
            if let Ok(n) = w.parse::<u32>() {
                pending = n;
            } else {
                let name = w.trim_matches(|c| c == '{' || c == '}');
                let cap = name.chars().next().map(|c| c.to_ascii_uppercase().to_string() + &name[1..]).unwrap_or_default();
                if let Some(e) = si_model::Element::parse(&cap) {
                    sets = sets.min(ctx.elements.count(e) as u32 / pending.max(1));
                    pending = 1;
                }
            }
        }
        return if sets == u32::MAX { 0 } else { sets };
    }
    // "per {Air} you have": one per element of that kind the spirit has this turn
    let cap = desc.trim().chars().next().map(|c| c.to_ascii_uppercase().to_string() + &desc.trim()[1..]).unwrap_or_default();
    if let Some(e) = si_model::Element::parse(&cap) {
        return ctx.elements.count(e) as u32;
    }
    // "every 2 {Dahan}": one per whole group of that many (Lead the Furious Assault)
    if let Some(rest) = lower.strip_prefix("every ") {
        let mut w = rest.split_whitespace();
        if let (Some(n), Some(kind)) = (w.next().and_then(|n| n.parse::<u32>().ok()), w.next()) {
            return count_of(l, &PieceFilter::parse(&desc.trim()[desc.trim().len() - kind.len()..])) / n.max(1);
        }
    }
    if lower.contains("past the first") {
        let first = desc.split(' ').next().unwrap_or("");
        return count_of(l, &PieceFilter::parse(first)).saturating_sub(1);
    }
    if lower.contains("adjacent") {
        if lower.contains("{blight}") {
            let here = if lower.contains("in or adjacent") { l.c.blight } else { 0 };
            return here + l.adjacent.iter().filter_map(|id| island.lands.get(id)).map(|x| x.c.blight).sum::<u32>();
        }
        if lower.contains("presence") {
            return adjacent_with_presence(island, target);
        }
        return 0;
    }
    if lower.contains("pushed by this power") {
        return 0;
    }
    // "{Badlands}/{Beast}/{Disease}/{Wilds}": every token of those kinds
    desc.split(|c| c == '/' || c == ' ').filter(|w| w.starts_with('{')).map(|w| count_of(l, &PieceFilter::parse(w))).sum()
}

fn count_of(l: &Land, f: &PieceFilter) -> u32 {
    match f {
        PieceFilter::Piece(PieceKind::Explorer) => l.c.explorers,
        PieceFilter::Piece(PieceKind::Town) => l.c.towns,
        PieceFilter::Piece(PieceKind::City) => l.c.cities,
        PieceFilter::Piece(PieceKind::Dahan) => l.c.dahan,
        PieceFilter::Token(TokenKind::Beast) => l.c.beasts,
        PieceFilter::Token(TokenKind::Disease) => l.c.disease,
        PieceFilter::Token(TokenKind::Strife) => l.c.strife,
        PieceFilter::Token(TokenKind::Wilds) => l.c.wilds,
        PieceFilter::Token(TokenKind::Badlands) => l.c.badlands,
        PieceFilter::Token(_) => 0,
        PieceFilter::Presence => l.c.presence,
        PieceFilter::Blight => l.c.blight,
        PieceFilter::AnyInvader => l.c.invaders(),
        PieceFilter::Other(_) => 0,
    }
}

/// "For each adjacent land with your Presence": lands next door holding presence.
fn adjacent_with_presence(island: &Island, target: &si_model::island::LandId) -> u32 {
    island.lands.get(target).map(|l| l.adjacent.iter().filter(|a| island.lands.get(a).is_some_and(|n| n.c.presence > 0)).count() as u32).unwrap_or(0)
}

fn add_piece(l: &mut Land, k: PieceKind) {
    match k {
        PieceKind::Explorer => l.c.explorers += 1,
        PieceKind::Town => l.c.towns += 1,
        PieceKind::City => l.c.cities += 1,
        PieceKind::Dahan => l.c.dahan += 1,
    }
}

fn add_filtered(l: &mut Land, f: &PieceFilter) {
    match f {
        PieceFilter::Piece(k) => add_piece(l, *k),
        PieceFilter::Token(TokenKind::Beast) => l.c.beasts += 1,
        _ => {}
    }
}

fn piece_of(f: &PieceFilter) -> Option<PieceKind> {
    match f {
        PieceFilter::Piece(k) => Some(*k),
        _ => None,
    }
}

/// The moves a line asked for that this application has not made yet: a "per Air"
/// gather runs the effect once per element, and each run takes its share of the ask.
fn remaining(ctx: &Ctx, a: &Applied, which: impl Fn(&Move) -> bool) -> Vec<Move> {
    // asks along the same path count once, summed (the record lists each piece moved)
    let mut asks: Vec<Move> = Vec::new();
    for m in ctx.moves.iter().filter(|m| which(m)) {
        match asks.iter_mut().find(|x| x.from == m.from && x.to == m.to) {
            Some(x) => x.n += m.n,
            None => asks.push(m.clone()),
        }
    }
    asks.iter()
        .filter_map(|m| {
            let done: u32 = a.moves.iter().filter(|d| d.from == m.from && d.to == m.to).map(|d| d.n).sum();
            (m.n > done).then(|| Move { n: m.n - done, ..m.clone() })
        })
        .collect()
}

fn record_move(a: &mut Applied, from: LandId, to: LandId, piece: Option<PieceKind>, n: u32) {
    if let Some(m) = a.moves.iter_mut().find(|m| m.from == from && m.to == to && m.piece == piece) {
        m.n += n;
    } else {
        a.moves.push(Move { from, to, piece, n });
    }
}

/// Where a pushed piece goes when the line does not say: the player's likeliest choice.
fn push_destination(island: &Island, adjacent: &[LandId], k: PieceKind) -> Option<LandId> {
    let mut best: Option<(i64, LandId)> = None;
    for id in adjacent {
        let Some(n) = island.lands.get(id) else { continue };
        if n.terrain == Some(Terrain::Ocean) {
            continue;
        }
        let score: i64 = if k == PieceKind::Dahan { n.c.invaders() as i64 * 10 - n.c.dahan as i64 } else { n.c.dahan as i64 * 10 + n.c.presence as i64 * 5 + n.c.invaders() as i64 };
        if best.as_ref().is_none_or(|(b, _)| score < *b) {
            best = Some((score, id.clone()));
        }
    }
    best.map(|(_, id)| id)
}

fn remove_pieces(l: &mut Land, filters: &[PieceFilter], count: u32, cheapest_first: bool) -> Vec<PieceKind> {
    let mut order: Vec<PieceKind> = Vec::new();
    let kinds: Vec<PieceKind> = filters
        .iter()
        .flat_map(|f| match f {
            PieceFilter::Piece(k) => vec![*k],
            PieceFilter::AnyInvader => vec![PieceKind::Explorer, PieceKind::Town, PieceKind::City],
            _ => vec![],
        })
        .collect();
    let mut ordered = kinds.clone();
    ordered.sort_by_key(|k| k.health());
    if !cheapest_first {
        ordered.reverse();
    }
    let mut left = count;
    for k in ordered {
        while left > 0 {
            let n = match k {
                PieceKind::Explorer => &mut l.c.explorers,
                PieceKind::Town => &mut l.c.towns,
                PieceKind::City => &mut l.c.cities,
                PieceKind::Dahan => &mut l.c.dahan,
            };
            if *n == 0 {
                break;
            }
            *n -= 1;
            left -= 1;
            order.push(k);
        }
    }
    order
}

/// Deal `amount` Damage in the land to the given target class; Invaders die
/// cheapest first (the kill-count default), Badlands add to each instance.
fn damage(l: &mut Land, amount: u32, to: DamageTarget, ctx: &Ctx, a: &mut Applied) {
    let badlands = l.c.badlands + if ctx.blight_as_badlands { l.c.blight } else { 0 };
    let total = amount + if amount > 0 { badlands } else { 0 };
    match to {
        DamageTarget::Dahan | DamageTarget::EachDahan => {
            let per = if to == DamageTarget::EachDahan { total * l.c.dahan.max(1) } else { total };
            let killed = l.c.dahan.min(per / 2);
            l.c.dahan -= killed;
            a.destroyed.extend(std::iter::repeat(PieceKind::Dahan).take(killed as usize));
            a.damage += per;
        }
        DamageTarget::EachInvader => {
            // the engine logs one Damage amount for an "each" effect; so do we
            let each = total;
            for (k, n) in [(PieceKind::Explorer, l.c.explorers), (PieceKind::Town, l.c.towns), (PieceKind::City, l.c.cities)] {
                let h = k.health() as u32 + ctx.health_bonus;
                if each >= h {
                    for _ in 0..n {
                        a.destroyed.push(k);
                    }
                    match k {
                        PieceKind::Explorer => l.c.explorers = 0,
                        PieceKind::Town => l.c.towns = 0,
                        PieceKind::City => l.c.cities = 0,
                        PieceKind::Dahan => {}
                    }
                }
            }
            a.damage += each;
        }
        DamageTarget::EachTownCity => {
            for (k, n) in [(PieceKind::Town, l.c.towns), (PieceKind::City, l.c.cities)] {
                let h = k.health() as u32 + ctx.health_bonus;
                if total >= h {
                    for _ in 0..n {
                        a.destroyed.push(k);
                    }
                    match k {
                        PieceKind::Town => l.c.towns = 0,
                        _ => l.c.cities = 0,
                    }
                }
            }
            a.damage += total;
        }
        DamageTarget::Invaders | DamageTarget::TownCityOnly | DamageTarget::ExplorerTownOnly | DamageTarget::Town => {
            a.damage += total;
            let allowed: Vec<PieceKind> = match to {
                DamageTarget::TownCityOnly => vec![PieceKind::Town, PieceKind::City],
                DamageTarget::ExplorerTownOnly => vec![PieceKind::Explorer, PieceKind::Town],
                DamageTarget::Town => vec![PieceKind::Town],
                _ => vec![PieceKind::Explorer, PieceKind::Town, PieceKind::City],
            };
            // the player spends the damage on the most valuable set it destroys (two Towns
            // over a City for 4 Damage; three Cities, a Town and an Explorer for 12)
            let has = |k: PieceKind, n: u32| if allowed.contains(&k) { n } else { 0 };
            let hb = ctx.health_bonus;
            let (ke, kt, kc) = crate::invaders::best_kills(total, (has(PieceKind::Explorer, l.c.explorers), has(PieceKind::Town, l.c.towns), has(PieceKind::City, l.c.cities)), (1 + hb, 2 + hb, 3 + hb));
            l.c.cities -= kc;
            l.c.towns -= kt;
            l.c.explorers -= ke;
            a.destroyed.extend(std::iter::repeat_n(PieceKind::City, kc as usize).chain(std::iter::repeat_n(PieceKind::Town, kt as usize)).chain(std::iter::repeat_n(PieceKind::Explorer, ke as usize)));
            if total > 0 {
                a.notes.push("damage distribution: the most valuable Invaders it kills (player's choice)".into());
            }
        }
    }
}

fn apply_one(e: &Effect, island: &mut Island, target: &si_model::island::LandId, ctx: &Ctx, a: &mut Applied) {
    let Some(l) = island.lands.get_mut(target) else { return };
    match e {
        Effect::Damage { amount, to } => damage(l, *amount as u32, *to, ctx, a),
        Effect::DamageBonus(n) => {
            a.damage += *n as u32;
            a.notes.push("+damage applied as extra total".into());
        }
        Effect::Defend(n) => {
            l.defend += *n as u32;
            a.defend += *n as u32;
        }
        Effect::Push { count, what } => {
            let adjacent = l.adjacent.clone();
            let mut asked: Vec<Move> = remaining(ctx, a, |m| m.from == *target && adjacent.contains(&m.to));
            // a Dahan push is a positional choice the line must make (`>A4`): unasked, the
            // Dahan stay, and the note says so. Invaders pushed leave either way.
            let dahan_only = what.iter().all(|f| matches!(f, PieceFilter::Piece(PieceKind::Dahan)));
            if ctx.moves_exact && asked.is_empty() {
                // the record (or the player's bracket) pushed nothing here
            } else if dahan_only && asked.is_empty() && count_of(l, &PieceFilter::Piece(PieceKind::Dahan)) > 0 {
                a.notes.push(format!("push up to {} Dahan: not taken, where they go is the player's choice (write >A4 to push one to A4)", if *count == 255 { "all".to_string() } else { count.to_string() }));
            } else {
            let avail: u32 = what.iter().map(|f| count_of(l, f)).sum();
            let n = if *count == 255 { avail } else if *count == 128 { avail / 2 } else { (*count as u32).min(avail) };
            let n = if asked.is_empty() { n } else { n.min(asked.iter().map(|m| m.n).sum::<u32>()) };
            let removed = remove_pieces(l, what, n, false);
            a.pushed += removed.len() as u32;
            a.pushed_pieces.extend(removed.iter().copied());
            if n < avail {
                a.notes.push("push: which pieces is the player's choice".into());
            }
            // each piece goes to an adjacent land: the one the line asks for, else a guess at
            // the player's choice (an Invader to the land with the fewest Dahan and no presence)
            let guessed = asked.is_empty() && !removed.is_empty();
            for k in removed {
                let dest = match asked.iter_mut().find(|m| m.n > 0 && m.piece.is_none_or(|p| p == k)) {
                    Some(m) => {
                        m.n -= 1;
                        Some(m.to.clone())
                    }
                    None => push_destination(island, &adjacent, k),
                };
                let Some(dest) = dest else {
                    a.notes.push(format!("{k:?} pushed with no adjacent land to go to"));
                    continue;
                };
                if let Some(d) = island.lands.get_mut(&dest) {
                    add_piece(d, k);
                }
                if k == PieceKind::Dahan {
                    presence_along(island, target, &dest, ctx, a);
                }
                record_move(a, target.clone(), dest, Some(k), 1);
            }
            if guessed {
                a.notes.push("push: where the pieces go is the player's choice".into());
            }
            }
        }
        Effect::Gather { count, what } => {
            // sources are the adjacent lands: the ones the line asks for first, then the
            // fullest (the player's choice)
            let adjacent = l.adjacent.clone();
            let want = if *count >= 128 { 0 } else { *count as u32 };
            let mut left = want;
            let what = what.clone();
            let target_id = target.clone();
            let asked: Vec<Move> = remaining(ctx, a, |m| m.to == target_id && adjacent.contains(&m.from));
            for f in &what {
                let mut plan: Vec<(u32, LandId)> = asked.iter().map(|m| (m.n, m.from.clone())).collect();
                let mut rest: Vec<(u32, LandId)> = adjacent.iter().filter(|a| !asked.iter().any(|m| m.from == **a)).filter_map(|a| island.lands.get(a).filter(|n| n.terrain != Some(Terrain::Ocean)).map(|n| (count_of(n, f), a.clone()))).filter(|(n, _)| *n > 0).collect();
                rest.sort_by(|x, y| y.0.cmp(&x.0));
                if !ctx.moves_exact {
                    plan.extend(rest);
                }
                for (cap, src) in plan {
                    if left == 0 {
                        break;
                    }
                    let avail = island.lands.get(&src).map(|n| count_of(n, f)).unwrap_or(0);
                    let take = avail.min(cap).min(left);
                    if take == 0 {
                        continue;
                    }
                    if let Some(src_land) = island.lands.get_mut(&src) {
                        remove_pieces(src_land, std::slice::from_ref(f), take, true);
                    }
                    if let Some(t) = island.lands.get_mut(&target_id) {
                        for _ in 0..take {
                            add_filtered(t, f);
                        }
                    }
                    if piece_of(f) == Some(PieceKind::Dahan) {
                        for _ in 0..take {
                            presence_along(island, &src, &target_id, ctx, a);
                        }
                    }
                    record_move(a, src.clone(), target_id.clone(), piece_of(f), take);
                    left -= take;
                }
            }
            a.gathered += want - left;
            if want > 0 && asked.is_empty() && !ctx.moves_exact {
                a.notes.push("gather: from the fullest adjacent land first (player's choice)".into());
            }
            return;
        }
        Effect::Fear(n) => a.fear += *n as u32,
        Effect::FearIfInvaders(n) => {
            if l.c.invaders() > 0 {
                a.fear += *n as u32;
            }
        }
        Effect::RemoveBlight(n) => {
            let r = l.c.blight.min(*n as u32);
            l.c.blight -= r;
            a.blight_removed += r;
        }
        Effect::AddBlight(n) => l.c.blight += *n as u32,
        Effect::AddDahan(n) => l.c.dahan += *n as u32,
        Effect::AddPresence { count, .. } => {
            l.c.presence += *count as u32;
            a.presence_added += *count as u32;
        }
        Effect::AddToken { kind, count } => {
            let n = *count as u32;
            match kind {
                TokenKind::Beast => l.c.beasts += n,
                TokenKind::Disease => l.c.disease += n,
                TokenKind::Strife => l.c.strife += n,
                TokenKind::Wilds => l.c.wilds += n,
                TokenKind::Badlands => l.c.badlands += n,
                _ => {}
            }
            a.tokens_added += n;
        }
        Effect::RemoveToken { kind, count } => {
            let n = *count as u32;
            let slot = match kind {
                TokenKind::Beast => Some(&mut l.c.beasts),
                TokenKind::Disease => Some(&mut l.c.disease),
                TokenKind::Strife => Some(&mut l.c.strife),
                TokenKind::Wilds => Some(&mut l.c.wilds),
                TokenKind::Badlands => Some(&mut l.c.badlands),
                _ => None,
            };
            if let Some(s) = slot {
                *s = s.saturating_sub(if n >= 128 { *s / 2 } else { n });
            }
        }
        Effect::Destroy { count, what } => {
            let n = if *count == 255 { what.iter().map(|f| count_of(l, f)).sum() } else { *count as u32 };
            let removed = remove_pieces(l, what, n, false);
            a.destroyed.extend(removed);
        }
        Effect::Remove { count, what } => {
            let n = if *count == 255 { what.iter().map(|f| count_of(l, f)).sum() } else if *count == 128 { what.iter().map(|f| count_of(l, f)).sum::<u32>() / 2 } else { *count as u32 };
            let removed = remove_pieces(l, what, n, false);
            a.removed.extend(removed);
        }
        Effect::Replace { count, from, to } => {
            let n = (*count as u32).min(count_of(l, from));
            let replaced = if matches!(from, PieceFilter::Presence) {
                l.c.presence -= n;
                n
            } else {
                remove_pieces(l, std::slice::from_ref(from), n, true).len() as u32
            };
            for _ in 0..replaced {
                match to {
                    PieceFilter::Piece(PieceKind::Explorer) => l.c.explorers += 1,
                    PieceFilter::Piece(PieceKind::Town) => l.c.towns += 1,
                    PieceFilter::Piece(PieceKind::City) => l.c.cities += 1,
                    PieceFilter::Piece(PieceKind::Dahan) => l.c.dahan += 1,
                    PieceFilter::Token(TokenKind::Beast) => l.c.beasts += 1,
                    PieceFilter::Token(TokenKind::Disease) => l.c.disease += 1,
                    PieceFilter::Token(TokenKind::Strife) => l.c.strife += 1,
                    PieceFilter::Token(TokenKind::Wilds) => l.c.wilds += 1,
                    PieceFilter::Token(TokenKind::Badlands) => l.c.badlands += 1,
                    _ => {}
                }
            }
        }
        Effect::Downgrade(n) => {
            let mut left = *n as u32;
            while left > 0 && l.c.cities > 0 {
                l.c.cities -= 1;
                l.c.towns += 1;
                left -= 1;
            }
            while left > 0 && l.c.towns > 0 {
                l.c.towns -= 1;
                l.c.explorers += 1;
                left -= 1;
            }
            while left > 0 && l.c.explorers > 0 {
                l.c.explorers -= 1;
                a.destroyed.push(PieceKind::Explorer);
                left -= 1;
            }
        }
        Effect::Per { per, then } => {
            let n = per_n(per, island, target, ctx);
            // "N Damage per X" is one Damage amount (Badlands add once), not N instances;
            // so is "each Dahan deals Damage equal to your presence" (12 for 4 Dahan and 3 presence)
            match per_damage(then, island, target, ctx) {
                Some((amount, to)) => {
                    let l = island.lands.get_mut(target).unwrap();
                    damage(l, amount * n, to, ctx, a);
                }
                None => {
                    for _ in 0..n {
                        for e in then {
                            apply_one(e, island, target, ctx, a);
                        }
                    }
                }
            }
        }
        Effect::Conditional { condition, then } => {
            let holds = condition_holds(condition, l);
            match holds {
                Some(true) => then.iter().for_each(|e| apply_one(e, island, target, ctx, a)),
                Some(false) => {}
                None => a.notes.push(format!("condition not evaluated: {condition}")),
            }
        }
        Effect::Choice(branches) => {
            let i = choose_branch(branches, target, ctx);
            if let Some(b) = branches.get(i) {
                b.iter().for_each(|e| apply_one(e, island, target, ctx, a));
                a.notes.push(format!("'or': {} (player's choice)", b.iter().map(Effect::words).collect::<Vec<_>>().join("; ")));
            }
        }
        Effect::Isolate | Effect::SkipRavage | Effect::SkipBuild | Effect::SkipExplore | Effect::SkipAll | Effect::SkipAction(_) | Effect::NoParticipation(_) | Effect::DahanImmune | Effect::InvaderDamageMinus(_) => {
            // Invader-phase effects: read by the Invader simulator from the engine's status effects
        }
        Effect::GainEnergy(_) | Effect::GainPowerCard { .. } | Effect::GainElement(_) | Effect::RangeBonus(_) | Effect::MayBeFast { .. } | Effect::Repeat | Effect::Both | Effect::Move { .. } | Effect::PlayAnother | Effect::MakeFast(_) | Effect::Reclaim(_) | Effect::RegainForgotten | Effect::EnergyPer { .. } | Effect::InvaderRule(_) | Effect::Unmodeled(_) | Effect::SpiritEffect(_) | Effect::SpecialToken { .. } | Effect::Timing { .. } | Effect::Movement(_) | Effect::DeckEffect(_) | Effect::Variable { .. } | Effect::Choose(_) => {
            a.notes.push(format!("not simulated: {}", e.words()));
        }
        Effect::Clarification(_) | Effect::MaySlow => {}
        Effect::Distribution(t) => a.notes.push(format!("distribution kept on the target (player's choice): {t}")),
        Effect::DestroyPresence(n) => {
            let n = (*n as u32).min(l.c.presence);
            l.c.presence -= n;
            a.notes.push(format!("{n} of your presence destroyed"));
        }
        Effect::RemoveHealth(h) => {
            // cheapest Invaders first, as many as the Health allows
            let mut left = *h as u32;
            let bonus = ctx.health_bonus;
            while left >= 1 && l.c.explorers > 0 {
                l.c.explorers -= 1;
                left -= 1;
                a.removed.push(PieceKind::Explorer);
            }
            while left >= 2 + bonus && l.c.towns > 0 {
                l.c.towns -= 1;
                left -= 2 + bonus;
                a.removed.push(PieceKind::Town);
            }
            while left >= 3 + bonus && l.c.cities > 0 {
                l.c.cities -= 1;
                left -= 3 + bonus;
                a.removed.push(PieceKind::City);
            }
        }
        Effect::Infighting(n) => {
            // the Strife-bearing Invaders deal their Damage to the others: the strongest ones assumed to carry it
            let strifed = if *n == 255 { l.c.strife.min(l.c.invaders()) } else { (*n as u32).min(l.c.invaders()) };
            let mut dmg = 0u32;
            let (mut c, mut t, mut e) = (l.c.cities, l.c.towns, l.c.explorers);
            for _ in 0..strifed {
                if c > 0 {
                    c -= 1;
                    dmg += 3;
                } else if t > 0 {
                    t -= 1;
                    dmg += 2;
                } else if e > 0 {
                    e -= 1;
                    dmg += 1;
                }
            }
            if dmg > 0 {
                a.notes.push(format!("{strifed} Invader(s) with Strife deal {dmg} Damage to the others (which carry Strife is the player's choice)"));
                damage(l, dmg, DamageTarget::Invaders, ctx, a);
            }
        }
        Effect::Duel { invaders, dahan } => {
            let d = (*dahan as u32).min(l.c.dahan);
            let i = (*invaders as u32).min(l.c.invaders());
            if d > 0 && i > 0 {
                damage(l, d * 2, DamageTarget::Invaders, ctx, a);
                // a Town or City kills a Dahan; an Explorer does not
                let strong = (l.c.cities + l.c.towns).min(i);
                let lost = strong.min(l.c.dahan);
                l.c.dahan -= lost;
                a.notes.push(format!("{d} Dahan and {i} Invader(s) fight: {lost} Dahan lost (which Invaders is the player's choice)"));
            }
        }
        Effect::NoBlight => {
            if let Some(l) = island.lands.get_mut(target) {
                l.no_blight = true;
            }
        }
        Effect::AddInvaders { kind, count } => {
            let n = *count as u32;
            match kind {
                PieceKind::Explorer => l.c.explorers += n,
                PieceKind::Town => l.c.towns += n,
                PieceKind::City => l.c.cities += n,
                PieceKind::Dahan => l.c.dahan += n,
            }
        }
        Effect::Build => match &ctx.rules {
            Some(rules) => {
                let o = crate::invaders::build_land(island, target, rules);
                a.notes.push(format!("forced Build: {:?} added", o.added));
            }
            None => a.notes.push("forced Build not simulated: no adversary rules in context".into()),
        },
        Effect::Area { scope, then } => {
            let sc = scope.to_ascii_lowercase();
            let adjacent: Vec<LandId> = island.lands.get(target).map(|t| t.adjacent.clone()).unwrap_or_default();
            let mut by_invaders: Vec<LandId> = adjacent.iter().filter(|id| island.lands.get(*id).is_some_and(|l| l.terrain != Some(Terrain::Ocean))).cloned().collect();
            by_invaders.sort_by_key(|id| std::cmp::Reverse(island.lands.get(id).map(|l| l.c.invaders()).unwrap_or(0)));
            let lands: Vec<LandId> = if sc.starts_with("each adjacent land") || sc.starts_with("each other coastal land") {
                by_invaders
            } else if sc.starts_with("that land and each adjacent land") {
                std::iter::once(target.clone()).chain(by_invaders).collect()
            } else if sc.starts_with("both target and one adjacent land") || sc.starts_with("both origin land and target land") {
                a.notes.push(format!("area: {scope} — the fullest adjacent land taken (player's choice)"));
                std::iter::once(target.clone()).chain(by_invaders.into_iter().take(1)).collect()
            } else if let Some(n) = sc.strip_prefix("up to ").and_then(|r| r.split(' ').next()).and_then(|n| n.parse::<usize>().ok()) {
                a.notes.push(format!("area: {scope} — the {n} fullest adjacent lands taken (player's choice)"));
                by_invaders.into_iter().take(n).collect()
            } else if sc.starts_with("each land where target spirit has presence") || sc == "your lands" || sc.starts_with("each of its lands") {
                // the Spirit's lands: in a solo game every presence on the island is theirs
                island.lands.values().filter(|x| x.c.presence > 0 && x.terrain != Some(Terrain::Ocean)).map(|x| x.id.clone()).collect()
            } else {
                vec![target.clone()]
            };
            for land in lands {
                for e in then {
                    apply_one(e, island, &land, ctx, a);
                }
            }
        }
        Effect::Trigger { when, then, once } => {
            a.triggers.push(TriggerArmed { when: when.clone(), then: then.clone(), once: *once, land: target.clone() });
            a.notes.push(format!("trigger armed: when {when}"));
        }
        Effect::NoCascade => {
            if let Some(l) = island.lands.get_mut(target) {
                l.no_cascade = true;
            }
        }
        Effect::DahanHealth(n) => {
            if let Some(l) = island.lands.get_mut(target) {
                l.dahan_health += *n as u32;
            }
        }
        Effect::DahanSimultaneous => {
            if let Some(l) = island.lands.get_mut(target) {
                l.dahan_simultaneous = true;
            }
        }
        Effect::ChooseLand(_) => {}
        Effect::TokenChoice { options, .. } => a.token_options = options.clone(),
        Effect::AddChosenToken(n) => {
            let n = *n as u32;
            let picked: Option<TokenKind> = ctx.picks.iter().find(|(r, _)| r == "AddTokenOfType").and_then(|(_, sel)| {
                let name: String = sel.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
                if name == "Blight" { if ctx.blight_as_badlands { Some(TokenKind::Badlands) } else { None } } else { TokenKind::parse(&name) }
            });
            let count_in = |l: &Land, k: TokenKind| -> u32 {
                match k {
                    TokenKind::Beast => l.c.beasts,
                    TokenKind::Disease => l.c.disease,
                    TokenKind::Strife => l.c.strife,
                    TokenKind::Wilds => l.c.wilds,
                    TokenKind::Badlands => l.c.badlands + if ctx.blight_as_badlands { l.c.blight } else { 0 },
                    _ => 0,
                }
            };
            let kind = picked.or_else(|| {
                let adjacent: Vec<LandId> = island.lands.get(target).map(|t| t.adjacent.clone()).unwrap_or_default();
                let options = if a.token_options.is_empty() { vec![TokenKind::Badlands, TokenKind::Beast, TokenKind::Disease, TokenKind::Strife, TokenKind::Wilds] } else { a.token_options.clone() };
                options.into_iter().map(|k| (k, adjacent.iter().filter_map(|id| island.lands.get(id)).map(|l| count_in(l, k)).sum::<u32>())).filter(|(_, c)| *c > 0).max_by_key(|(_, c)| *c).map(|(k, _)| k)
            });
            match kind {
                Some(k) => {
                    if let Some(l) = island.lands.get_mut(target) {
                        match k {
                            TokenKind::Beast => l.c.beasts += n,
                            TokenKind::Disease => l.c.disease += n,
                            TokenKind::Strife => l.c.strife += n,
                            TokenKind::Wilds => l.c.wilds += n,
                            TokenKind::Badlands => l.c.badlands += n,
                            _ => {}
                        }
                        a.tokens_added += n;
                    }
                    a.notes.push(format!("{n} {k:?} added ({})", if picked.is_some() { "the type the log recorded" } else { "the type most present nearby; player's choice" }));
                }
                None => a.notes.push("no token type available in an adjacent land".into()),
            }
        }
        Effect::Spread { kind, count, scope } => {
            let word = match kind {
                TokenKind::Wilds => "Wild".to_string(),
                k => format!("{k:?}"),
            };
            let reason = format!("Adding{word}");
            let recorded: Vec<LandId> = ctx.placements.iter().filter(|(r, _)| *r == reason).map(|(_, l)| l.clone()).collect();
            let lands: Vec<LandId> = if !recorded.is_empty() {
                a.notes.push(format!("distribution: {count} {word} placed as the log recorded"));
                recorded
            } else {
                let coastal_only = scope.to_ascii_lowercase().contains("coastal");
                let mut cands: Vec<(&LandId, u32)> = island.lands.iter().filter(|(id, l)| *id != target && (!coastal_only || l.coastal) && l.terrain != Some(Terrain::Ocean)).map(|(id, l)| (id, l.c.invaders())).collect();
                cands.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(y.0)));
                a.notes.push(format!("distribution: {count} {word} among {scope} is the player's choice; spread over the fullest lands"));
                cands.into_iter().map(|(id, _)| id.clone()).collect()
            };
            if lands.is_empty() {
                a.notes.push(format!("no land for {count} {word} among {scope}"));
            } else {
                for i in 0..*count as usize {
                    let target_land = lands[i % lands.len()].clone();
                    if let Some(land) = island.lands.get_mut(&target_land) {
                        match kind {
                            TokenKind::Beast => land.c.beasts += 1,
                            TokenKind::Disease => land.c.disease += 1,
                            TokenKind::Strife => land.c.strife += 1,
                            TokenKind::Wilds => land.c.wilds += 1,
                            TokenKind::Badlands => land.c.badlands += 1,
                            _ => {}
                        }
                        a.tokens_added += 1;
                    }
                }
            }
        }
        Effect::FearDeckRemove { energy_per_player } => {
            a.notes.push(format!("optional: {energy_per_player} Energy per player to remove the top Fear card from the game; not taken in the simulation"));
        }
        Effect::Ravage => match &ctx.rules {
            Some(rules) => {
                if let Some(o) = crate::invaders::ravage_land(island, target, rules) {
                    a.ravaged.push(o);
                }
            }
            None => a.notes.push("forced Ravage not simulated: no adversary rules in context".into()),
        },
        Effect::Other(s) => a.notes.push(format!("unparsed: {s}")),
    }
}

/// Apply a Power's base effects, plus every threshold the elements reach.
pub fn apply(card_effects: &[Effect], thresholds: &[(ElementSet, Vec<Effect>)], island: &mut Island, target: &si_model::island::LandId, ctx: &Ctx) -> Applied {
    let mut a = Applied::default();
    for e in card_effects {
        apply_one(e, island, target, ctx, &mut a);
    }
    for (need, effects) in thresholds {
        if ctx.elements.satisfies(need) {
            for e in effects {
                apply_one(e, island, target, ctx, &mut a);
            }
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use si_record::Counts;

    fn island(c: Counts) -> Island {
        let id: si_model::island::LandId = "A1".parse().unwrap();
        let land = Land { id: id.clone(), terrain: Some(Terrain::Sands), coastal: false, adjacent: vec![], c, ..Default::default() };
        Island { lands: [(id, land)].into_iter().collect() }
    }

    fn ctx() -> Ctx {
        Ctx::plain(0, ElementSet::new())
    }

    fn three_lands() -> Island {
        let mk = |id: &str, adj: &[&str], c: Counts| {
            let lid: si_model::island::LandId = id.parse().unwrap();
            (lid.clone(), Land { id: lid, terrain: Some(Terrain::Sands), coastal: false, adjacent: adj.iter().map(|a| a.parse().unwrap()).collect(), c, ..Default::default() })
        };
        Island {
            lands: [
                mk("A1", &["A2", "A3"], Counts { dahan: 2, explorers: 1, ..Default::default() }),
                mk("A2", &["A1"], Counts { dahan: 1, ..Default::default() }),
                mk("A3", &["A1"], Counts { dahan: 2, towns: 1, ..Default::default() }),
            ]
            .into_iter()
            .collect(),
        }
    }

    #[test]
    fn a_gather_takes_from_the_land_asked_and_a_push_lands_where_asked() {
        let a1: si_model::island::LandId = "A1".parse().unwrap();
        let asked = Ctx { moves: vec![Move { from: "A2".parse().unwrap(), to: a1.clone(), piece: None, n: 1 }, Move { from: a1.clone(), to: "A3".parse().unwrap(), piece: None, n: 1 }], ..ctx() };
        let mut isl = three_lands();
        let a = apply(&[Effect::Gather { count: 1, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }, Effect::Push { count: 1, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }], &[], &mut isl, &a1, &asked);
        assert_eq!((isl.lands[&a1].c.dahan, isl.lands[&"A2".parse().unwrap()].c.dahan, isl.lands[&"A3".parse().unwrap()].c.dahan), (2, 0, 3), "one Dahan in from A2 (not the fuller A3), one out to A3");
        assert_eq!(a.moves, vec![Move { from: "A2".parse().unwrap(), to: a1.clone(), piece: Some(PieceKind::Dahan), n: 1 }, Move { from: a1.clone(), to: "A3".parse().unwrap(), piece: Some(PieceKind::Dahan), n: 1 }]);
        // nothing asked: the gather takes from the fullest land; a Dahan push is not taken, since where they go is the player's call
        let mut isl = three_lands();
        let a = apply(&[Effect::Gather { count: 1, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }, Effect::Push { count: 1, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }], &[], &mut isl, &a1, &ctx());
        assert_eq!(a.moves.len(), 1);
        assert_eq!(a.moves[0].from, "A3".parse().unwrap(), "the fullest adjacent land");
        assert_eq!((a.pushed, isl.lands[&a1].c.dahan), (0, 3), "the Dahan stay until a line says where");
        assert!(a.notes.iter().any(|n| n.starts_with("push up to 1 Dahan: not taken")), "{:?}", a.notes);
        // an Invader push is taken, to a land without Dahan or presence
        let mut isl = three_lands();
        let a = apply(&[Effect::Push { count: 1, what: vec![PieceFilter::Piece(PieceKind::Explorer)] }], &[], &mut isl, &a1, &ctx());
        assert_eq!((a.pushed, a.moves.len()), (1, 1));
        assert_ne!(a.moves[0].to, a1, "{:?}", a.moves);
    }

    #[test]
    fn manifestation_deals_dahan_times_presence_as_one_amount() {
        // Thunderspeaker T5: 4 Dahan, 3 presence in A1, 12 Damage: 3 Cities, a Town and an Explorer
        let mut isl = island(Counts { dahan: 4, presence: 3, cities: 3, towns: 3, explorers: 1, ..Default::default() });
        let mpg = Effect::Per { per: PieceFilter::Piece(PieceKind::Dahan), then: vec![Effect::Per { per: PieceFilter::Presence, then: vec![Effect::Damage { amount: 1, to: DamageTarget::Invaders }] }] };
        let a = apply(&[mpg], &[], &mut isl, &"A1".parse().unwrap(), &ctx());
        assert_eq!(a.damage, 12);
        assert_eq!(a.destroyed, vec![PieceKind::City, PieceKind::City, PieceKind::City, PieceKind::Town, PieceKind::Explorer]);
    }

    #[test]
    fn presence_goes_with_the_dahan() {
        // Ally of the Dahan: each Dahan moved out of a land with presence takes one along
        let (a1, a2, a3): (LandId, LandId, LandId) = ("A1".parse().unwrap(), "A2".parse().unwrap(), "A3".parse().unwrap());
        let mut isl = three_lands();
        isl.lands.get_mut(&a3).unwrap().c.presence = 1;
        isl.lands.get_mut(&a1).unwrap().c.presence = 1;
        let c = Ctx { presence_with_dahan: true, moves: vec![Move { from: a3.clone(), to: a1.clone(), piece: None, n: 2 }, Move { from: a1.clone(), to: a2.clone(), piece: None, n: 3 }], ..ctx() };
        let a = apply(&[Effect::Gather { count: 2, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }, Effect::Push { count: 3, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }], &[], &mut isl, &a1, &c);
        // A3's one presence came with the first of two Dahan; A1's two presence left with three Dahan pushed
        assert_eq!((isl.lands[&a1].c.presence, isl.lands[&a2].c.presence, isl.lands[&a3].c.presence), (0, 2, 0));
        assert_eq!(a.presence_moved, 3);
        // without the rule the presence stays
        let mut isl = three_lands();
        isl.lands.get_mut(&a3).unwrap().c.presence = 1;
        let c = Ctx { moves: vec![Move { from: a3.clone(), to: a1.clone(), piece: None, n: 2 }], ..ctx() };
        apply(&[Effect::Gather { count: 2, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }], &[], &mut isl, &a1, &c);
        assert_eq!((isl.lands[&a1].c.presence, isl.lands[&a3].c.presence), (0, 1));
    }

    #[test]
    fn an_or_takes_the_branch_the_line_asks_for() {
        // Voice of Thunder: "Push up to 4 Dahan" or "If Invaders are present, 2 Fear"
        let (a1, a2): (LandId, LandId) = ("A1".parse().unwrap(), "A2".parse().unwrap());
        let voice = Effect::Choice(vec![vec![Effect::Push { count: 4, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }], vec![Effect::FearIfInvaders(2)]]);
        let mut isl = three_lands();
        let a = apply(std::slice::from_ref(&voice), &[], &mut isl, &a1, &ctx());
        assert_eq!((a.fear, a.pushed), (2, 0), "no push asked: the Fear");
        let mut isl = three_lands();
        let c = Ctx { moves: vec![Move { from: a1.clone(), to: a2.clone(), piece: None, n: 2 }], ..ctx() };
        let a = apply(&[voice], &[], &mut isl, &a1, &c);
        assert_eq!((a.fear, a.pushed, isl.lands[&a2].c.dahan), (0, 2, 3), "`>A2x2`: the push");
    }

    #[test]
    fn exact_moves_take_nothing_beyond_them() {
        // the record shows Sudden Ambush gathered nothing: no guess from the fullest land
        let a1: LandId = "A1".parse().unwrap();
        let mut isl = three_lands();
        let c = Ctx { moves_exact: true, ..ctx() };
        let a = apply(&[Effect::Gather { count: 1, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }, Effect::Push { count: 1, what: vec![PieceFilter::Piece(PieceKind::Explorer)] }], &[], &mut isl, &a1, &c);
        assert_eq!((a.gathered, a.pushed), (0, 0));
    }

    #[test]
    fn asks_along_one_path_add_up() {
        // the record lists each piece pushed: two 1-piece asks to A2 are one 2-piece ask
        let (a1, a2): (LandId, LandId) = ("A1".parse().unwrap(), "A2".parse().unwrap());
        let mut isl = three_lands();
        isl.lands.get_mut(&a1).unwrap().c.towns = 1;
        let c = Ctx { moves_exact: true, moves: vec![Move { from: a1.clone(), to: a2.clone(), piece: None, n: 1 }, Move { from: a1.clone(), to: a2.clone(), piece: None, n: 1 }], ..ctx() };
        let a = apply(&[Effect::Push { count: 1, what: vec![PieceFilter::Piece(PieceKind::Explorer)] }, Effect::Push { count: 1, what: vec![PieceFilter::Piece(PieceKind::Town)] }], &[], &mut isl, &a1, &c);
        assert_eq!(a.pushed, 2);
    }

    #[test]
    fn an_ask_is_spent_across_the_per_element_runs() {
        // Gather the Warriors with 3 Sun: "Push up to 1 Dahan per Sun" asked for twice to A3 pushes two, not three
        let a1: si_model::island::LandId = "A1".parse().unwrap();
        let mut isl = three_lands();
        let mut c = ctx();
        c.elements = ElementSet::from_list([si_model::Element::parse("Sun").unwrap(); 3]);
        c.moves = vec![Move { from: a1.clone(), to: "A3".parse().unwrap(), piece: None, n: 2 }];
        let per = Effect::Per { per: PieceFilter::parse("{Sun}"), then: vec![Effect::Push { count: 1, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }] };
        let a = apply(&[per], &[], &mut isl, &a1, &c);
        assert_eq!(a.pushed, 2);
        assert_eq!(a.moves, vec![Move { from: a1.clone(), to: "A3".parse().unwrap(), piece: Some(PieceKind::Dahan), n: 2 }]);
        assert_eq!((isl.lands[&a1].c.dahan, isl.lands[&"A3".parse().unwrap()].c.dahan), (0, 4));
    }

    #[test]
    fn a_gather_per_element_counts_the_spirits_elements() {
        // Gather the Warriors: "Gather up to 1 Dahan per Air you have" with 2 Air gathers 2
        let a1: si_model::island::LandId = "A1".parse().unwrap();
        let mut isl = three_lands();
        let mut c = ctx();
        c.elements = ElementSet::from_list([si_model::Element::parse("Air").unwrap(), si_model::Element::parse("Air").unwrap()]);
        let per = Effect::Per { per: PieceFilter::parse("{Air}"), then: vec![Effect::Gather { count: 1, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }] };
        let a = apply(&[per], &[], &mut isl, &a1, &c);
        assert_eq!(a.gathered, 2);
        assert_eq!(isl.lands[&a1].c.dahan, 4);
        assert_eq!(a.moves.iter().map(|m| m.n).sum::<u32>(), 2);
    }

    #[test]
    fn damage_kills_the_most_valuable_set_and_badlands_add() {
        let mut isl = island(Counts { explorers: 1, towns: 1, ..Default::default() });
        let a = apply(&[Effect::Damage { amount: 2, to: DamageTarget::Invaders }], &[], &mut isl, &"A1".parse().unwrap(), &ctx());
        assert_eq!(a.destroyed, vec![PieceKind::Town], "2 damage: the Town dies, worth more than the Explorer");
        // Thunderspeaker T5 (second attempt): Manifestation's 4 damage in A2 took two Towns, not the City
        let mut isl = island(Counts { cities: 1, towns: 2, ..Default::default() });
        let a = apply(&[Effect::Damage { amount: 4, to: DamageTarget::Invaders }], &[], &mut isl, &"A1".parse().unwrap(), &ctx());
        assert_eq!(a.destroyed, vec![PieceKind::Town, PieceKind::Town]);
        let mut isl = island(Counts { towns: 1, blight: 1, ..Default::default() });
        let c = Ctx { blight_as_badlands: true, ..ctx() };
        let a = apply(&[Effect::Damage { amount: 1, to: DamageTarget::Invaders }], &[], &mut isl, &"A1".parse().unwrap(), &c);
        assert_eq!((a.damage, a.destroyed), (2, vec![PieceKind::Town]), "Fiery Vengeance in a Blighted land kills the Town");
    }

    #[test]
    fn per_disease_damage_and_thresholds() {
        let mut isl = island(Counts { cities: 2, disease: 3, ..Default::default() });
        let per = Effect::Per { per: PieceFilter::Token(TokenKind::Disease), then: vec![Effect::Damage { amount: 1, to: DamageTarget::Invaders }] };
        let a = apply(&[per], &[], &mut isl, &"A1".parse().unwrap(), &ctx());
        assert_eq!(a.damage, 3);
        assert_eq!(a.destroyed, vec![PieceKind::City], "'1 Damage per Disease' is one 3-Damage amount: it kills one City");
        let mut isl = island(Counts { cities: 1, disease: 3, blight: 1, ..Default::default() });
        let c = Ctx { blight_as_badlands: true, ..ctx() };
        let per = Effect::Per { per: PieceFilter::Token(TokenKind::Disease), then: vec![Effect::Damage { amount: 1, to: DamageTarget::Invaders }] };
        let a = apply(&[per], &[], &mut isl, &"A1".parse().unwrap(), &c);
        assert_eq!(a.damage, 4, "Vengeance's Epidemics: 3 Disease plus one Blight-as-Badlands, as the engine logged it (4, not 6)");
    }

    #[test]
    fn push_defend_tokens_and_conditionals() {
        let mut isl = island(Counts { explorers: 2, towns: 1, dahan: 1, ..Default::default() });
        let effects = vec![Effect::Push { count: 3, what: vec![PieceFilter::Piece(PieceKind::Explorer), PieceFilter::Piece(PieceKind::Town)] }, Effect::Defend(2), Effect::AddToken { kind: TokenKind::Disease, count: 1 }, Effect::Conditional { condition: "target land has {Dahan}".into(), then: vec![Effect::Fear(1)] }];
        let a = apply(&effects, &[], &mut isl, &"A1".parse().unwrap(), &ctx());
        let l = &isl.lands[&"A1".parse().unwrap()];
        assert_eq!((a.pushed, a.defend, a.tokens_added, a.fear), (3, 2, 1, 1));
        assert_eq!((l.c.explorers, l.c.towns, l.c.disease, l.defend), (0, 0, 1, 2));
    }
}
