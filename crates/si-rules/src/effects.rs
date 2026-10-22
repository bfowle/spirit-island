//! Spirit actions: the parsed `Effect` language executed on a land. Where the
//! rules leave a choice to the player (which Invaders take the Damage, which
//! pieces are Pushed, which branch of an "or"), the executor takes a stated
//! default and says so, so the verifier can tell a choice from a rule gap.

use serde::{Deserialize, Serialize};
use si_model::island::{LandId, PieceKind, Terrain, TokenKind};
use si_model::power::{DamageTarget, Effect, MoveReach, PieceFilter};
use si_model::ElementSet;

use crate::invaders::{Island, Land};

/// One piece or token a Power moved: gathered into its land (`to` is the target) or pushed
/// out of it (`from` is the target). In a line's request `piece` None is the effect's own
/// kind; a kind named (`>A4:T`) is moved first.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Move {
    pub from: LandId,
    pub to: LandId,
    #[serde(default)]
    pub piece: Option<PieceFilter>,
    #[serde(default = "one")]
    pub n: u32,
}

fn one() -> u32 {
    1
}

/// A moved kind's code in a line's bracket (`>A4:T`): E, T, C, Dh for Dahan, and the tokens
/// Be(ast), Di(sease), St(rife), Wi(lds), Ba(dlands).
pub fn piece_code(f: &PieceFilter) -> Option<&'static str> {
    Some(match f {
        PieceFilter::Piece(PieceKind::Explorer) => "E",
        PieceFilter::Piece(PieceKind::Town) => "T",
        PieceFilter::Piece(PieceKind::City) => "C",
        PieceFilter::Piece(PieceKind::Dahan) => "Dh",
        PieceFilter::Token(TokenKind::Beast) => "Be",
        PieceFilter::Token(TokenKind::Disease) => "Di",
        PieceFilter::Token(TokenKind::Strife) => "St",
        PieceFilter::Token(TokenKind::Wilds) => "Wi",
        PieceFilter::Token(TokenKind::Badlands) => "Ba",
        PieceFilter::Presence => "Pr",
        _ => return None,
    })
}

/// The kind a bracket's code names (`T`, `Be`; case does not matter).
pub fn piece_of_code(code: &str) -> Option<PieceFilter> {
    let c = code.to_ascii_lowercase();
    Some(match c.as_str() {
        "e" => PieceFilter::Piece(PieceKind::Explorer),
        "t" => PieceFilter::Piece(PieceKind::Town),
        "c" => PieceFilter::Piece(PieceKind::City),
        "d" | "dh" => PieceFilter::Piece(PieceKind::Dahan),
        "be" | "b" => PieceFilter::Token(TokenKind::Beast),
        "di" => PieceFilter::Token(TokenKind::Disease),
        "st" => PieceFilter::Token(TokenKind::Strife),
        "wi" | "w" => PieceFilter::Token(TokenKind::Wilds),
        "ba" => PieceFilter::Token(TokenKind::Badlands),
        "pr" => PieceFilter::Presence,
        _ => return None,
    })
}

/// What a Power did to its target land.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Applied {
    pub damage: u32,
    pub destroyed: Vec<PieceKind>,
    /// Each piece destroyed and the land it was in.
    #[serde(default)]
    pub destroyed_in: Vec<(LandId, PieceKind)>,
    /// Pieces Removed (not Destroyed: no Fear, no "destroyed" triggers).
    #[serde(default)]
    pub removed: Vec<PieceKind>,
    /// The tokens it removed, by kind (Many Minds' Sacred Site removed as a Beast counts one):
    /// "For each {Beast} Removed".
    #[serde(default)]
    pub removed_tokens: Vec<TokenKind>,
    pub defend: u32,
    /// The Defend by land: Wrap in Wings of Sunlight's goes where the Dahan were moved.
    #[serde(default)]
    pub defended: Vec<(LandId, u32)>,
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
    /// What the Damage and the Destroys took where the player chooses, by land: what a
    /// bracket writes `-T`.
    #[serde(default)]
    pub killed: Vec<(LandId, PieceFilter)>,
    /// Presence that went along with the Dahan this Power moved (Ally of the Dahan).
    #[serde(default)]
    pub presence_moved: u32,
    /// The spirit's own presence the Power destroyed ("destroy 1 of your presence").
    #[serde(default)]
    pub presence_destroyed: u32,
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
#[derive(Debug, Clone)]
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
    /// The pieces the line says this use destroys, by land (the player's choice of which
    /// pieces the Damage or the Destroy takes: the record's, or `-T` in a bracket). Stated,
    /// they are all it destroys: the rest of its Damage falls where it kills nothing.
    pub kills: Option<Vec<(LandId, PieceFilter)>>,
    /// The Spirit's Sacred Site counts as a Beast (Many Minds: A Joining of Swarms and Flocks).
    pub site_beast: bool,
    /// The Spirit's Beasts gather from and push to lands up to 2 away (Fly Fast as Thought).
    pub beasts_far: bool,
    /// Damage per presence destroyed, dealt to both Invaders and Dahan in that land
    /// (Volcano's Collapse in a Blast of Lava and Steam); 0 for any other Spirit.
    pub collapse: u32,
}

impl Ctx {
    /// A context with nothing recorded and no spirit rules: the elements and Health only.
    pub fn plain(health_bonus: u32, elements: ElementSet) -> Ctx {
        Ctx { health_bonus, elements, blight_as_badlands: false, rules: None, placements: vec![], picks: vec![], moves: vec![], moves_exact: false, branches: vec![], presence_with_dahan: false, kills: None, site_beast: false, beasts_far: false, collapse: 0 }
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
fn per_n(per: &PieceFilter, island: &Island, target: &LandId, ctx: &Ctx, a: &Applied) -> u32 {
    match per {
        PieceFilter::Other(desc) => per_count_other(desc, island, target, ctx, a),
        // the Spirit's Sacred Site counts as a Beast (A Joining of Swarms and Flocks)
        PieceFilter::Token(TokenKind::Beast) if ctx.site_beast => island.lands.get(target).map(|l| l.c.beasts + (l.c.presence >= 2) as u32).unwrap_or(0),
        _ => island.lands.get(target).map(|l| count_of(l, per)).unwrap_or(0),
    }
}

/// The one Damage amount a "per" holds, as the engine deals it: `N Damage per X`, and
/// a "per" inside a "per" (`each Dahan deals Damage equal to your presence`) multiplied out.
fn per_damage(then: &[Effect], island: &Island, target: &LandId, ctx: &Ctx, a: &Applied) -> Option<(u32, DamageTarget)> {
    match then {
        [Effect::Damage { amount, to }] => Some((*amount as u32, *to)),
        [Effect::Per { per, then }] => per_damage(then, island, target, ctx, a).map(|(n, to)| (n * per_n(per, island, target, ctx, a), to)),
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

/// Does the effect list gather or push this kind of piece?
fn moves_kind(es: &[Effect], kind: &PieceFilter) -> bool {
    es.iter().any(|e| match e {
        Effect::Push { what, .. } | Effect::Gather { what, .. } => allows(what, kind),
        Effect::Per { then, .. } | Effect::Conditional { then, .. } => moves_kind(then, kind),
        _ => false,
    })
}

/// Which branch of an "or" the line takes: the one whose kind of piece the line names
/// (`>A4:T`, or a Dahan move written bare for a Power that moves Dahan), else the one the
/// record has for this Power this turn (by its printed order, else by its words), else the
/// Dahan move a line asks for without naming a kind, when there are Dahan for it to move,
/// else the first that does not wait on a Dahan move the line does not ask for (Voice of
/// Thunder without `[>A4]` takes its Fear).
fn choose_branch(branches: &[Vec<Effect>], island: &Island, target: &LandId, ctx: &Ctx) -> usize {
    let asked: Vec<&Move> = ctx.moves.iter().filter(|m| m.from == *target || m.to == *target).collect();
    // a move has something to move: the kind out of the land, or in from where it asks
    let has = |m: &Move, k: &PieceFilter| island.lands.get(if m.from == *target { target } else { &m.from }).is_some_and(|l| count_of(l, k) > 0);
    if let Some(i) = branches.iter().position(|b| asked.iter().any(|m| m.piece.as_ref().is_some_and(|k| moves_kind(b, k) && has(m, k)))) {
        return i;
    }
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
    // a bare move asked, with Dahan to make it: out of the land, or in from where it asks
    let dahan = PieceFilter::Piece(PieceKind::Dahan);
    if asked.iter().any(|m| m.piece.is_none() && has(m, &dahan)) {
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
fn per_count_other(desc: &str, island: &Island, target: &LandId, ctx: &Ctx, a: &Applied) -> u32 {
    let lower = desc.to_ascii_lowercase();
    // Explosive Eruption's X: the presence the line says it destroys for the Power
    if lower == si_model::power::X_DESTROYED {
        return ctx.kills.as_ref().map(|k| k.iter().filter(|(_, f)| *f == PieceFilter::Presence).count() as u32).unwrap_or(0);
    }
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
    // what this use did: "For each {Dahan} Pushed by this Power", "For each {Beast} Removed"
    if lower.contains("pushed by this power") {
        let invader = lower.starts_with("invader");
        let kind = desc.split(['{', '}']).nth(1).map(|k| PieceFilter::parse(&format!("{{{k}}}")));
        return a.moves.iter().filter(|m| m.from == *target && m.to != *target && m.piece.as_ref().is_some_and(|p| if invader { matches!(p, PieceFilter::Piece(k) if *k != PieceKind::Dahan) } else { Some(p) == kind.as_ref() })).map(|m| m.n).sum();
    }
    if lower.ends_with(" removed") {
        let kind = desc.split(['{', '}']).nth(1).and_then(TokenKind::parse);
        return a.removed_tokens.iter().filter(|t| kind.is_none_or(|k| **t == k)).count() as u32;
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

/// The moves a line asked for that this application has not made yet: a "per Air"
/// gather runs the effect once per element, and each run takes its share of the ask.
fn remaining(ctx: &Ctx, a: &Applied, which: impl Fn(&Move) -> bool) -> Vec<Move> {
    // asks along the same path count once, summed (the record lists each piece moved)
    let mut asks: Vec<Move> = Vec::new();
    for m in ctx.moves.iter().filter(|m| which(m)) {
        match asks.iter_mut().find(|x| x.from == m.from && x.to == m.to && x.piece == m.piece) {
            Some(x) => x.n += m.n,
            None => asks.push(m.clone()),
        }
    }
    asks.iter()
        .filter_map(|m| {
            // what this application moved along the path of a kind the ask takes
            let done: u32 = a.moves.iter().filter(|d| d.from == m.from && d.to == m.to && m.piece.as_ref().is_none_or(|k| d.piece.as_ref().is_some_and(|u| k == u || allows(std::slice::from_ref(k), u)))).map(|d| d.n).sum();
            (m.n > done).then(|| Move { n: m.n - done, ..m.clone() })
        })
        .collect()
}

fn record_move(a: &mut Applied, from: LandId, to: LandId, piece: Option<PieceFilter>, n: u32) {
    if let Some(m) = a.moves.iter_mut().find(|m| m.from == from && m.to == to && m.piece == piece) {
        m.n += n;
    } else {
        a.moves.push(Move { from, to, piece, n });
    }
}

/// Does an effect's list of pieces take this kind (`Invaders` takes an Explorer)?
fn allows(what: &[PieceFilter], kind: &PieceFilter) -> bool {
    what.iter().any(|f| f == kind || (*f == PieceFilter::AnyInvader && matches!(kind, PieceFilter::Piece(k) if *k != PieceKind::Dahan)))
}

fn token_mut(l: &mut Land, t: TokenKind) -> Option<&mut u32> {
    match t {
        TokenKind::Beast => Some(&mut l.c.beasts),
        TokenKind::Disease => Some(&mut l.c.disease),
        TokenKind::Strife => Some(&mut l.c.strife),
        TokenKind::Wilds => Some(&mut l.c.wilds),
        TokenKind::Badlands => Some(&mut l.c.badlands),
        _ => None,
    }
}

/// One piece or token of the filter's kind taken off the land, as the kind it was (`Invaders`
/// takes the cheapest or the dearest, by `cheapest_first`); None when the land has none.
fn take_one(l: &mut Land, f: &PieceFilter, cheapest_first: bool) -> Option<PieceFilter> {
    match f {
        PieceFilter::Piece(_) | PieceFilter::AnyInvader => remove_pieces(l, std::slice::from_ref(f), 1, cheapest_first).pop().map(PieceFilter::Piece),
        PieceFilter::Token(t) => {
            let n = token_mut(l, *t)?;
            if *n == 0 {
                return None;
            }
            *n -= 1;
            Some(f.clone())
        }
        PieceFilter::Presence => {
            if l.c.presence == 0 {
                return None;
            }
            l.c.presence -= 1;
            Some(PieceFilter::Presence)
        }
        _ => None,
    }
}

/// Up to `n` pieces or tokens the effect's list takes off the land: the kinds in `prefer`
/// first (the record's choice), then the dearest Invaders, then tokens.
fn take_many(l: &mut Land, what: &[PieceFilter], n: u32, prefer: &[PieceFilter]) -> Vec<PieceFilter> {
    let mut out: Vec<PieceFilter> = Vec::new();
    for k in prefer.iter().filter(|k| allows(what, k)) {
        if out.len() as u32 >= n {
            break;
        }
        if let Some(u) = take_one(l, k, false) {
            out.push(u);
        }
    }
    let pieces: Vec<PieceFilter> = what.iter().filter(|f| matches!(f, PieceFilter::Piece(_) | PieceFilter::AnyInvader)).cloned().collect();
    let rest = n.saturating_sub(out.len() as u32);
    out.extend(remove_pieces(l, &pieces, rest, false).into_iter().map(PieceFilter::Piece));
    for f in what.iter().filter(|f| matches!(f, PieceFilter::Token(_) | PieceFilter::Presence)) {
        while (out.len() as u32) < n {
            match take_one(l, f, false) {
                Some(u) => out.push(u),
                None => break,
            }
        }
    }
    out
}

/// The kinds the record has this use destroying in the land, less those this application
/// has destroyed already.
fn recorded_kills(ctx: &Ctx, land: &LandId, a: &Applied) -> Vec<PieceFilter> {
    // what this application has destroyed in the land already, tokens too
    let mut done: Vec<PieceFilter> = a.killed.iter().filter(|(l, _)| l == land).map(|(_, k)| k.clone()).collect();
    let mut out = Vec::new();
    for (_, k) in ctx.kills.iter().flatten().filter(|(l, _)| l == land) {
        if let Some(i) = done.iter().position(|d| d == k) {
            done.remove(i);
            continue;
        }
        out.push(k.clone());
    }
    out
}

/// One piece or token put down in the land.
fn put_one(l: &mut Land, f: &PieceFilter) {
    match f {
        PieceFilter::Piece(k) => add_piece(l, *k),
        PieceFilter::Token(t) => {
            if let Some(n) = token_mut(l, *t) {
                *n += 1;
            }
        }
        PieceFilter::Presence => l.c.presence += 1,
        _ => {}
    }
}

/// Where a pushed piece goes when the line does not say: the player's likeliest choice.
fn push_destination(island: &Island, adjacent: &[LandId], k: &PieceFilter) -> Option<LandId> {
    let dahan = *k == PieceFilter::Piece(PieceKind::Dahan);
    let mut best: Option<(i64, LandId)> = None;
    for id in adjacent {
        let Some(n) = island.lands.get(id) else { continue };
        if n.terrain == Some(Terrain::Ocean) {
            continue;
        }
        let score: i64 = if dahan { n.c.invaders() as i64 * 10 - n.c.dahan as i64 } else { n.c.dahan as i64 * 10 + n.c.presence as i64 * 5 + n.c.invaders() as i64 };
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
/// Russia's A Sense for Impending Disaster (level 2): "The first time each Action would
/// Destroy {Explorer}: If possible, 1 of those {Explorer} is instead Pushed; 1 {Fear} when you
/// do so." The Power's use is the Action: its first Explorer destroyed goes where the line
/// says (`>A8:E`), else to the player's likeliest pick.
fn explorer_flees(island: &mut Island, ctx: &Ctx, a: &mut Applied) {
    let Some(i) = a.destroyed_in.iter().position(|(_, k)| *k == PieceKind::Explorer) else { return };
    let land = a.destroyed_in[i].0.clone();
    // not out of an Isolated land, nor into one: then it is destroyed after all
    let isolated = |x: &LandId| ctx.rules.as_ref().is_some_and(|r| r.isolated.contains(x)) || island.lands.get(x).is_some_and(|n| n.isolated);
    if isolated(&land) {
        return;
    }
    let adjacent: Vec<LandId> = island.lands.get(&land).map(|l| l.adjacent.iter().filter(|x| !isolated(x) && island.lands.get(*x).is_some_and(|n| n.terrain != Some(Terrain::Ocean))).cloned().collect()).unwrap_or_default();
    let explorer = PieceFilter::Piece(PieceKind::Explorer);
    let stated = ctx.moves.iter().find(|m| m.from == land && m.piece.as_ref() == Some(&explorer) && adjacent.contains(&m.to)).map(|m| m.to.clone());
    let Some(dest) = stated.or_else(|| push_destination(island, &adjacent, &explorer)) else { return };
    a.destroyed_in.remove(i);
    if let Some(j) = a.destroyed.iter().position(|k| *k == PieceKind::Explorer) {
        a.destroyed.remove(j);
    }
    if let Some(j) = a.killed.iter().position(|(at, f)| *at == land && *f == explorer) {
        a.killed.remove(j);
    }
    if let Some(d) = island.lands.get_mut(&dest) {
        d.c.explorers += 1;
    }
    a.fear += 1;
    record_move(a, land.clone(), dest.clone(), Some(explorer), 1);
    a.notes.push(format!("an Explorer destroyed in {land} was Pushed to {dest} instead, 1 Fear (Russia's A Sense for Impending Disaster)"));
}

/// Volcano's Collapse in a Blast of Lava and Steam: "When your {Presence} is Destroyed, in
/// that land, deal 1 Damage per Destroyed {Presence} to both Invaders and to {Dahan}."
fn collapse(l: &mut Land, destroyed: u32, ctx: &Ctx, a: &mut Applied) {
    if ctx.collapse == 0 || destroyed == 0 {
        return;
    }
    let d = destroyed * ctx.collapse;
    damage(l, d, DamageTarget::Invaders, ctx, a);
    damage(l, d, DamageTarget::Dahan, ctx, a);
    a.notes.push(format!("{destroyed} presence destroyed: {d} Damage to the Invaders and to the Dahan (Collapse in a Blast of Lava and Steam)"));
}

fn damage(l: &mut Land, amount: u32, to: DamageTarget, ctx: &Ctx, a: &mut Applied) {
    let badlands = l.c.badlands + if ctx.blight_as_badlands { l.c.blight } else { 0 };
    let total = amount + if amount > 0 { badlands } else { 0 };
    let hb = ctx.health_bonus;
    let dahan_full = 2 + l.dahan_health;
    // a piece's Health with its bonuses; Damage on it stays until Time Passes (`Land::wounds`)
    let full = move |k: PieceKind| if k == PieceKind::Dahan { dahan_full } else { k.health() as u32 + hb };
    let land = l.id.clone();
    let died = |a: &mut Applied, killed: Vec<PieceKind>, chosen: bool| {
        for k in killed {
            a.destroyed.push(k);
            a.destroyed_in.push((land.clone(), k));
            if chosen {
                a.killed.push((land.clone(), PieceFilter::Piece(k)));
            }
        }
    };
    match to {
        DamageTarget::Dahan => {
            let k = l.take_dahan_damage(total, dahan_full);
            died(a, vec![PieceKind::Dahan; k as usize], false);
            a.damage += total;
        }
        DamageTarget::EachDahan => {
            let k = l.take_each_damage(total, &[PieceKind::Dahan], &full);
            a.damage += total * (l.c.dahan + k.len() as u32).max(1);
            died(a, k, false);
        }
        DamageTarget::EachInvader => {
            // the engine logs one Damage amount for an "each" effect; so do we
            let k = l.take_each_damage(total, &[PieceKind::Explorer, PieceKind::Town, PieceKind::City], &full);
            died(a, k, false);
            a.damage += total;
        }
        DamageTarget::EachTownCity => {
            let k = l.take_each_damage(total, &[PieceKind::Town, PieceKind::City], &full);
            died(a, k, false);
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
            // the pieces the line says it destroys go first; stated, nothing else dies and the
            // rest stays as Damage where it kills nothing. Unstated, the player spends it on
            // the most valuable set it destroys, each piece at the Health it has left
            let first: Vec<PieceKind> = recorded_kills(ctx, &l.id, a).into_iter().filter_map(|k| if let PieceFilter::Piece(p) = k { Some(p) } else { None }).collect();
            let k = l.take_damage(total, &allowed, &full, &first, ctx.kills.is_some());
            died(a, k, true);
            if total > 0 {
                a.notes.push(if ctx.kills.is_some() { "damage distribution: as the line states".into() } else { "damage distribution: the most valuable Invaders it kills (player's choice)".into() });
            }
        }
    }
}

/// The lands two steps from `id` (not it, not its neighbors, no Ocean): where Many Minds'
/// Beasts may come from or go to.
/// Does one of these effects Move a piece of this kind from `from` to `to`, as far as its
/// printed reach goes ("to any land", "within {3Range} to target land")? For the line's law
/// that moves go between adjacent lands unless a Power says farther.
pub fn move_reaches(effects: &[Effect], kind: Option<&PieceFilter>, island: &Island, from: &LandId, to: &LandId) -> bool {
    effects.iter().any(|e| match e {
        Effect::Move { what, reach, .. } if kind.is_none_or(|k| allows(what, k) || *k == PieceFilter::Presence) => match reach {
            MoveReach::From(n) | MoveReach::Into(n) | MoveReach::SameTerrain(n) => within(island, from, to, *n as u32),
            MoveReach::Between | MoveReach::ToPresence => true,
            // along a Beast's steps (two lands for Many Minds); step by step
            MoveReach::Along(_) => within(island, from, to, 2),
            MoveReach::Path(_) => within(island, from, to, 1),
        },
        Effect::Per { then, .. } | Effect::Conditional { then, .. } | Effect::Area { then, .. } => move_reaches(then, kind, island, from, to),
        Effect::Choice(bs) => bs.iter().any(|b| move_reaches(b, kind, island, from, to)),
        _ => false,
    })
}

/// Is `to` within `n` lands of `from`, stepping only through lands (not Oceans)? 255 is any.
pub fn within(island: &Island, from: &LandId, to: &LandId, n: u32) -> bool {
    if from == to || n >= 255 {
        return island.lands.contains_key(to);
    }
    let mut seen: Vec<LandId> = vec![from.clone()];
    let mut frontier: Vec<LandId> = vec![from.clone()];
    for _ in 0..n {
        let mut next = Vec::new();
        for f in &frontier {
            for x in island.lands.get(f).map(|l| l.adjacent.clone()).unwrap_or_default() {
                if x == *to {
                    return true;
                }
                if !seen.contains(&x) && island.lands.get(&x).is_some_and(|l| l.terrain != Some(Terrain::Ocean)) {
                    seen.push(x.clone());
                    next.push(x);
                }
            }
        }
        frontier = next;
    }
    false
}

/// A piece named in a card's words (`Dahan`, `Beast`, `Presence`).
fn piece_of_word(w: &str) -> Option<PieceFilter> {
    match w.to_ascii_lowercase().as_str() {
        "presence" => Some(PieceFilter::Presence),
        "dahan" => Some(PieceFilter::Piece(PieceKind::Dahan)),
        "explorer" => Some(PieceFilter::Piece(PieceKind::Explorer)),
        "town" => Some(PieceFilter::Piece(PieceKind::Town)),
        "city" => Some(PieceFilter::Piece(PieceKind::City)),
        k => TokenKind::parse(&(k[..1].to_ascii_uppercase() + &k[1..])).map(PieceFilter::Token),
    }
}

/// A Move: the line's stated moves of its kinds (which pieces and where are the player's
/// choice), each within the reach the card prints, up to its count. Unstated, nothing
/// moves, and the note says so; a stated move beyond the reach is not made.
fn apply_move(count: u8, what: &[PieceFilter], reach: MoveReach, island: &mut Island, target: &LandId, ctx: &Ctx, a: &mut Applied) {
    // Many Minds' Sacred Site moves as a Beast: 2 of its presence together (`:Pr`)
    let site = ctx.site_beast && allows(what, &PieceFilter::Token(TokenKind::Beast));
    // a bare move is a Dahan's (the notation's rule)
    let kind_of = |m: &Move| -> Option<PieceFilter> {
        let k = m.piece.clone().unwrap_or(PieceFilter::Piece(PieceKind::Dahan));
        (allows(what, &k) || (site && k == PieceFilter::Presence)).then_some(k)
    };
    // the asks of its kinds, a bare one named a Dahan's, less what this Power already moved
    // of that kind along the same path
    let mut asked: Vec<Move> = Vec::new();
    for m in ctx.moves.iter().filter(|m| m.from != m.to) {
        let Some(k) = kind_of(m) else { continue };
        match asked.iter_mut().find(|x| x.from == m.from && x.to == m.to && x.piece.as_ref() == Some(&k)) {
            Some(x) => x.n += m.n,
            None => asked.push(Move { piece: Some(k), ..m.clone() }),
        }
    }
    for x in asked.iter_mut() {
        let done: u32 = a.moves.iter().filter(|d| d.from == x.from && d.to == x.to && d.piece == x.piece).map(|d| d.n).sum();
        x.n = x.n.saturating_sub(done);
    }
    asked.retain(|x| x.n > 0);
    if asked.is_empty() {
        a.notes.push(format!("move {}: where they go is the player's choice, not stated (write [>A4:{}])", what.iter().map(|f| f.words()).collect::<Vec<_>>().join("/"), what.first().and_then(piece_code).unwrap_or("Dh")));
        return;
    }
    let presence: Vec<LandId> = island.lands.iter().filter(|(_, l)| l.c.presence > 0).map(|(id, _)| id.clone()).collect();
    let terrain = island.lands.get(target).and_then(|l| l.terrain);
    // what this Power moved of other kinds before, for a Move "along" with them
    let carried: Vec<Move> = a.moves.iter().filter(|m| m.piece.as_ref().is_none_or(|k| !allows(what, k))).cloned().collect();
    let mut left = if count == 255 { u32::MAX } else { count as u32 };
    let mut reached: Vec<LandId> = vec![target.clone()];
    let mut steps = 0u32;
    // a path's steps in the order they join (the record may list the way back first): each
    // pass takes the asks that fit now, until none does
    let mut pending: Vec<Move> = asked;
    loop {
        let before = pending.len();
        let mut later: Vec<Move> = Vec::new();
        for m in std::mem::take(&mut pending) {
            let Some(k) = kind_of(&m) else { continue };
            if left == 0 {
                later.push(m);
                continue;
            }
            let adjacent = island.lands.get(&m.from).is_some_and(|l| l.adjacent.contains(&m.to));
            let cap = match reach {
                MoveReach::From(n) => (m.from == *target && within(island, target, &m.to, n as u32)).then_some(u32::MAX),
                MoveReach::Into(n) => (m.to == *target && within(island, &m.from, target, n as u32)).then_some(u32::MAX),
                MoveReach::Path(n) => (adjacent && reached.contains(&m.from)).then(|| (n as u32 * count.min(254) as u32).saturating_sub(steps)),
                MoveReach::Along(per) => {
                    let with: u32 = carried.iter().filter(|c| c.from == m.from && c.to == m.to).map(|c| c.n).sum();
                    let done: u32 = a.moves.iter().filter(|x| x.from == m.from && x.to == m.to && x.piece.as_ref() == Some(&k)).map(|x| x.n).sum();
                    Some((with * per as u32).saturating_sub(done))
                }
                MoveReach::Between => ((m.from == *target && presence.contains(&m.to)) || (m.to == *target && presence.contains(&m.from))).then_some(u32::MAX),
                MoveReach::ToPresence => (presence.contains(&m.from) && presence.contains(&m.to)).then_some(u32::MAX),
                MoveReach::SameTerrain(n) => (m.from == *target && within(island, target, &m.to, n as u32) && island.lands.get(&m.to).and_then(|l| l.terrain) == terrain).then_some(u32::MAX),
            };
            let Some(cap) = cap else {
                later.push(m);
                continue;
            };
            // the Sacred Site as a Beast moves its pair per Beast
            let per = if site && k == PieceFilter::Presence { 2 } else { 1 };
            let mut moved = 0;
            for _ in 0..(m.n / per).min(left).min(cap) {
                if island.lands.get(&m.from).is_none_or(|l| count_of(l, &k) < per) {
                    break;
                }
                for _ in 0..per {
                    let Some(u) = island.lands.get_mut(&m.from).and_then(|l| take_one(l, &k, false)) else { break };
                    if let Some(d) = island.lands.get_mut(&m.to) {
                        put_one(d, &u);
                    }
                    if u == PieceFilter::Piece(PieceKind::Dahan) {
                        presence_along(island, &m.from, &m.to, ctx, a);
                    }
                }
                moved += 1;
            }
            if moved > 0 {
                record_move(a, m.from.clone(), m.to.clone(), Some(k), moved * per);
                // a path counts its steps (one Beast, two lands); any other Move its pieces
                if !matches!(reach, MoveReach::Path(_)) {
                    left = left.saturating_sub(moved);
                }
                steps += moved;
                if !reached.contains(&m.to) {
                    reached.push(m.to.clone());
                }
            }
        }
        pending = later;
        if pending.is_empty() || pending.len() == before || left == 0 {
            break;
        }
    }
    for m in pending.iter().filter(|_| left > 0) {
        a.notes.push(format!("move {}>{}: beyond the card's reach ({})", m.from, m.to, reach.words()));
    }
}

pub fn two_away(island: &Island, id: &LandId) -> Vec<LandId> {
    let near: Vec<LandId> = island.lands.get(id).map(|l| l.adjacent.clone()).unwrap_or_default();
    let mut out: Vec<LandId> = Vec::new();
    for n in &near {
        for m in island.lands.get(n).map(|l| l.adjacent.clone()).unwrap_or_default() {
            if m != *id && !near.contains(&m) && !out.contains(&m) && island.lands.get(&m).is_some_and(|x| x.terrain != Some(Terrain::Ocean)) {
                out.push(m);
            }
        }
    }
    out
}

/// The Spirit's Sacred Site moved as a Beast: 2 of its presence along each path the line
/// asks (`:Pr`), within reach. A Joining of Swarms and Flocks.
fn move_site_beasts(island: &mut Island, target: &LandId, ctx: &Ctx, reach_out: &[LandId], reach_in: &[LandId], a: &mut Applied) {
    for m in ctx.moves.iter().filter(|m| m.piece == Some(PieceFilter::Presence)) {
        let (from, to) = (m.from.clone(), m.to.clone());
        let ok = (from == *target && reach_out.contains(&to)) || (to == *target && reach_in.contains(&from));
        let have = island.lands.get(&from).map(|l| l.c.presence).unwrap_or(0);
        let n = m.n.min(have);
        if !ok || n == 0 {
            continue;
        }
        if let Some(f) = island.lands.get_mut(&from) {
            f.c.presence -= n;
        }
        if let Some(t) = island.lands.get_mut(&to) {
            t.c.presence += n;
        }
        record_move(a, from, to, Some(PieceFilter::Presence), n);
        a.presence_moved += n;
    }
}

fn apply_one(e: &Effect, island: &mut Island, target: &si_model::island::LandId, ctx: &Ctx, a: &mut Applied) {
    let beasty = |what: &[PieceFilter]| what.contains(&PieceFilter::Token(TokenKind::Beast));
    let far: Vec<LandId> = match e {
        Effect::Push { what, .. } | Effect::Gather { what, .. } if ctx.beasts_far && beasty(what) => two_away(island, target),
        _ => Vec::new(),
    };
    if ctx.site_beast {
        if let Effect::Push { what, .. } | Effect::Gather { what, .. } = e {
            if beasty(what) {
                let near: Vec<LandId> = island.lands.get(target).map(|l| l.adjacent.clone()).unwrap_or_default();
                let reach: Vec<LandId> = near.into_iter().chain(far.iter().cloned()).collect();
                let (out_, in_): (Vec<LandId>, Vec<LandId>) = if matches!(e, Effect::Push { .. }) { (reach, vec![]) } else { (vec![], reach) };
                move_site_beasts(island, target, ctx, &out_, &in_, a);
            }
        }
    }
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
            a.defended.push((target.clone(), *n as u32));
        }
        Effect::Push { count, what } => {
            let adjacent = l.adjacent.clone();
            // a Beast may go two lands (Fly Fast as Thought): an ask for one may name such a land
            let beast_ask = |m: &Move| m.piece.as_ref().map_or(what.iter().all(|f| *f == PieceFilter::Token(TokenKind::Beast)), |k| *k == PieceFilter::Token(TokenKind::Beast));
            let mut asked: Vec<Move> = remaining(ctx, a, |m| m.from == *target && (adjacent.contains(&m.to) || (far.contains(&m.to) && beast_ask(m))) && m.piece.as_ref().is_none_or(|k| allows(what, k)));
            // a Dahan push is a positional choice the line must make (`>A4`): unasked, the
            // Dahan stay, and the note says so. Invaders pushed leave either way.
            let dahan_only = what.iter().all(|f| matches!(f, PieceFilter::Piece(PieceKind::Dahan)));
            // a presence push ("Target Spirit may Push up to 1 of their Presence") is the
            // player's call too: unasked, the presence stays
            let presence_only = what.iter().all(|f| matches!(f, PieceFilter::Presence));
            if ctx.moves_exact && asked.is_empty() {
                // the record (or the player's bracket) pushed nothing here
            } else if presence_only && asked.is_empty() {
                a.notes.push("push presence: not taken, where it goes is the player's choice (write >A4:Pr)".into());
            } else if dahan_only && asked.is_empty() && count_of(l, &PieceFilter::Piece(PieceKind::Dahan)) > 0 {
                a.notes.push(format!("push up to {} Dahan: not taken, where they go is the player's choice (write >A4 to push one to A4)", if *count == 255 { "all".to_string() } else { count.to_string() }));
            } else {
            let avail: u32 = what.iter().map(|f| count_of(l, f)).sum();
            let n = if *count == 255 { avail } else if *count == 128 { avail / 2 } else { (*count as u32).min(avail) };
            let n = if asked.is_empty() { n } else { n.min(asked.iter().map(|m| m.n).sum::<u32>()) };
            // the kinds the line names go first, then the dearest Invaders, then tokens
            let prefer: Vec<PieceFilter> = asked.iter().filter_map(|m| m.piece.clone().map(|k| std::iter::repeat_n(k, m.n as usize))).flatten().collect();
            let removed = if ctx.moves_exact {
                // exact: ask by ask, its kind (or the effect's own), and nothing beyond
                let mut took: Vec<PieceFilter> = Vec::new();
                for m in &asked {
                    let room = n.saturating_sub(took.len() as u32).min(m.n);
                    match &m.piece {
                        Some(k) => (0..room).map_while(|_| take_one(l, k, false)).for_each(|u| took.push(u)),
                        None => took.extend(take_many(l, what, room, &[])),
                    }
                }
                took
            } else {
                take_many(l, what, n, &prefer)
            };
            a.pushed += removed.len() as u32;
            a.pushed_pieces.extend(removed.iter().filter_map(|u| match u {
                PieceFilter::Piece(k) => Some(*k),
                _ => None,
            }));
            if n < avail {
                a.notes.push("push: which pieces is the player's choice".into());
            }
            // each piece goes to an adjacent land: the one the line asks for (its kind named
            // first), else a guess at the player's choice (an Invader to the land with the
            // fewest Dahan and no presence)
            let guessed = asked.is_empty() && !removed.is_empty();
            for u in removed {
                // the ask that names its kind, else one whose kind takes it (Invaders an
                // Explorer), else one that names none
                let ask = asked
                    .iter()
                    .position(|m| m.n > 0 && m.piece.as_ref() == Some(&u))
                    .or_else(|| asked.iter().position(|m| m.n > 0 && m.piece.as_ref().is_some_and(|k| allows(std::slice::from_ref(k), &u))))
                    .or_else(|| asked.iter().position(|m| m.n > 0 && m.piece.is_none()));
                let dest = match ask {
                    Some(i) => {
                        asked[i].n -= 1;
                        Some(asked[i].to.clone())
                    }
                    None => push_destination(island, &adjacent, &u),
                };
                let Some(dest) = dest else {
                    a.notes.push(format!("{u:?} pushed with no adjacent land to go to"));
                    continue;
                };
                if let Some(d) = island.lands.get_mut(&dest) {
                    put_one(d, &u);
                }
                if u == PieceFilter::Piece(PieceKind::Dahan) {
                    presence_along(island, target, &dest, ctx, a);
                }
                record_move(a, target.clone(), dest, Some(u), 1);
            }
            if guessed {
                a.notes.push("push: where the pieces go is the player's choice".into());
            }
            }
        }
        Effect::Gather { count, what } => {
            // sources are the adjacent lands: the ones the line asks for first (the kind it
            // names, else the effect's own), then the fullest (the player's choice)
            let adjacent = l.adjacent.clone();
            let want = if *count >= 128 { 0 } else { *count as u32 };
            let mut left = want;
            let what = what.clone();
            let target_id = target.clone();
            let beast_ask = |m: &Move| m.piece.as_ref().map_or(what.iter().all(|f| *f == PieceFilter::Token(TokenKind::Beast)), |k| *k == PieceFilter::Token(TokenKind::Beast));
            let asked: Vec<Move> = remaining(ctx, a, |m| m.to == target_id && (adjacent.contains(&m.from) || (far.contains(&m.from) && beast_ask(m))) && m.piece.as_ref().is_none_or(|k| allows(&what, k)));
            // what is left of each ask, used up across the kinds the effect names
            let mut ask_left: Vec<u32> = asked.iter().map(|m| m.n).collect();
            for f in &what {
                // (ask index or none, room, source, kind)
                let mut plan: Vec<(Option<usize>, u32, LandId, PieceFilter)> = asked.iter().enumerate().filter(|(_, m)| m.piece.as_ref().is_none_or(|k| allows(std::slice::from_ref(f), k))).map(|(i, m)| (Some(i), ask_left[i], m.from.clone(), m.piece.clone().unwrap_or_else(|| f.clone()))).collect();
                let mut rest: Vec<(Option<usize>, u32, LandId, PieceFilter)> = adjacent.iter().filter(|a| !asked.iter().any(|m| m.from == **a)).filter_map(|a| island.lands.get(a).filter(|n| n.terrain != Some(Terrain::Ocean)).map(|n| (None, count_of(n, f), a.clone(), f.clone()))).filter(|(_, n, _, _)| *n > 0).collect();
                rest.sort_by(|x, y| y.1.cmp(&x.1));
                if !ctx.moves_exact {
                    plan.extend(rest);
                }
                for (ask, cap, src, kind) in plan {
                    let cap = ask.map(|i| ask_left[i].min(cap)).unwrap_or(cap);
                    let mut took = 0;
                    while took < cap && left > 0 {
                        let Some(u) = island.lands.get_mut(&src).and_then(|s| take_one(s, &kind, true)) else { break };
                        if let Some(t) = island.lands.get_mut(&target_id) {
                            put_one(t, &u);
                        }
                        if u == PieceFilter::Piece(PieceKind::Dahan) {
                            presence_along(island, &src, &target_id, ctx, a);
                        }
                        record_move(a, src.clone(), target_id.clone(), Some(u), 1);
                        took += 1;
                        left -= 1;
                    }
                    if let Some(i) = ask {
                        ask_left[i] -= took;
                    }
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
            // "Add" may take a presence already on the island: from where the line says
            // (`:Pr` paths into the land), else from the track
            let n = *count as u32;
            let sources: Vec<(LandId, u32)> = ctx.moves.iter().filter(|m| m.to == *target && m.from != *target && m.piece == Some(PieceFilter::Presence)).map(|m| (m.from.clone(), m.n)).collect();
            let mut from_board = 0;
            for (src, k) in sources {
                let have = island.lands.get(&src).map(|x| x.c.presence).unwrap_or(0);
                let take = k.min(n - from_board).min(have);
                if take > 0 {
                    if let Some(x) = island.lands.get_mut(&src) {
                        x.c.presence -= take;
                    }
                    record_move(a, src, target.clone(), Some(PieceFilter::Presence), take);
                    from_board += take;
                }
                if from_board >= n {
                    break;
                }
            }
            if let Some(t) = island.lands.get_mut(target) {
                t.c.presence += n;
            }
            a.presence_added += n - from_board;
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
            // Many Minds' Sacred Site is a Beast too, removed as one (its 2 presence, `-Prx2`)
            let site = ctx.site_beast && *kind == TokenKind::Beast && l.c.presence >= 2;
            let have = token_mut(l, *kind).map(|n| *n).unwrap_or(0) + site as u32;
            let mut n = if *count >= 128 { have / 2 } else { (*count as u32).min(have) };
            // which, and how many of the "up to", the line states as the use's kills (`-Be`);
            // unstated, as many as it may, the tokens before the Sacred Site
            let token = PieceFilter::Token(*kind);
            let (site_n, token_n) = match &ctx.kills {
                Some(k) => {
                    let here = |f: &PieceFilter| k.iter().filter(|(at, x)| at == target && x == f).count() as u32;
                    (if site { (here(&PieceFilter::Presence) / 2).min(1) } else { 0 }, here(&token))
                }
                None => (0, n),
            };
            if site_n > 0 && n > 0 {
                l.c.presence -= 2;
                a.presence_destroyed += 2;
                a.killed.extend(std::iter::repeat_n((target.clone(), PieceFilter::Presence), 2));
                a.removed_tokens.push(*kind);
                n -= 1;
            }
            if let Some(s) = token_mut(l, *kind) {
                let k = n.min(token_n).min(*s);
                *s -= k;
                a.removed_tokens.extend(std::iter::repeat_n(*kind, k as usize));
                a.killed.extend(std::iter::repeat_n((target.clone(), token.clone()), k as usize));
            }
        }
        Effect::Destroy { count, what } => {
            let n = if *count == 255 { what.iter().map(|f| count_of(l, f)).sum() } else { *count as u32 };
            let prefer = recorded_kills(ctx, target, a);
            // stated: exactly those and nothing else ("up to" is the player's call)
            let removed = if ctx.kills.is_some() {
                let mut took = Vec::new();
                for k in prefer.iter().filter(|k| allows(what, k)) {
                    if took.len() as u32 >= n {
                        break;
                    }
                    took.extend(take_one(l, k, false));
                }
                took
            } else {
                take_many(l, what, n, &prefer)
            };
            a.killed.extend(removed.iter().map(|u| (target.clone(), u.clone())));
            let presence = removed.iter().filter(|u| **u == PieceFilter::Presence).count() as u32;
            a.presence_destroyed += presence;
            collapse(l, presence, ctx, a);
            for u in &removed {
                if let PieceFilter::Piece(k) = u {
                    a.destroyed.push(*k);
                    a.destroyed_in.push((target.clone(), *k));
                }
            }
        }
        Effect::Remove { count, what } => {
            let n = if *count == 255 { what.iter().map(|f| count_of(l, f)).sum() } else if *count == 128 { what.iter().map(|f| count_of(l, f)).sum::<u32>() / 2 } else { *count as u32 };
            let removed = take_many(l, what, n, &[]);
            a.presence_destroyed += removed.iter().filter(|u| **u == PieceFilter::Presence).count() as u32;
            a.removed.extend(removed.iter().filter_map(|u| match u {
                PieceFilter::Piece(k) => Some(*k),
                _ => None,
            }));
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
                a.destroyed_in.push((target.clone(), PieceKind::Explorer));
                left -= 1;
            }
        }
        // X presence destroyed together is one destruction (Collapse deals its Damage as one
        // amount); the line's kills say how many (`-Pr`), so they are its kills
        Effect::Per { per, then } if matches!(then.as_slice(), [Effect::DestroyPresence(1)]) => {
            let n = per_n(per, island, target, ctx, a).min(255) as u8;
            let before = a.presence_destroyed;
            apply_one(&Effect::DestroyPresence(n), island, target, ctx, a);
            a.killed.extend(std::iter::repeat_n((target.clone(), PieceFilter::Presence), (a.presence_destroyed - before) as usize));
        }
        Effect::Per { per, then } => {
            let n = per_n(per, island, target, ctx, a);
            // "N Damage per X" is one Damage amount (Badlands add once), not N instances;
            // so is "each Dahan deals Damage equal to your presence" (12 for 4 Dahan and 3 presence)
            match per_damage(then, island, target, ctx, a) {
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
        // "If you moved at least 1 {Dahan}" (Wrap in Wings of Sunlight): out of the target, by this Power
        Effect::Conditional { condition, then } if condition.to_ascii_lowercase().starts_with("you moved at least 1 {") => {
            let kind = condition.split(['{', '}']).nth(1).and_then(piece_of_word);
            let moved = a.moves.iter().any(|m| m.from == *target && m.to != *target && m.n > 0 && m.piece == kind);
            if moved {
                then.iter().for_each(|e| apply_one(e, island, target, ctx, a));
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
            let i = choose_branch(branches, island, target, ctx);
            if let Some(b) = branches.get(i) {
                b.iter().for_each(|e| apply_one(e, island, target, ctx, a));
                a.notes.push(format!("'or': {} (player's choice)", b.iter().map(Effect::words).collect::<Vec<_>>().join("; ")));
            }
        }
        // "skip one Invader Action": the land keeps it for the Invader steps, spent where the player says
        Effect::SkipAction(n) => {
            if let Some(l) = island.lands.get_mut(target) {
                l.skip_actions += *n as u32;
            }
        }
        Effect::Isolate | Effect::SkipRavage | Effect::SkipBuild | Effect::SkipExplore | Effect::SkipAll | Effect::NoParticipation(_) | Effect::DahanImmune | Effect::InvaderDamageMinus(_) => {
            // Invader-phase effects: read by the Invader simulator from the engine's status effects
        }
        Effect::GainEnergy(_) | Effect::GainPowerCard { .. } | Effect::GainElement(_) | Effect::RangeBonus(_) | Effect::MayBeFast { .. } | Effect::Both | Effect::PlayAnother | Effect::MakeFast(_) | Effect::Reclaim(_) | Effect::RegainForgotten | Effect::EnergyPer { .. } | Effect::InvaderRule(_) | Effect::Unmodeled(_) | Effect::SpiritEffect(_) | Effect::SpecialToken { .. } | Effect::Timing { .. } | Effect::Movement(_) | Effect::DeckEffect(_) | Effect::Variable { .. } | Effect::Choose(_) => {
            a.notes.push(format!("not simulated: {}", e.words()));
        }
        // a Repeat is the Power's next use, a play of its own in the line
        Effect::Clarification(_) | Effect::MaySlow | Effect::Instead | Effect::First | Effect::Requires { .. } | Effect::Repeat(_) => {}
        Effect::Move { count, what, reach } => apply_move(*count, what, *reach, island, target, ctx, a),
        Effect::Distribution(t) => a.notes.push(format!("distribution kept on the target (player's choice): {t}")),
        Effect::DestroyPresence(n) => {
            let n = (*n as u32).min(l.c.presence);
            l.c.presence -= n;
            a.presence_destroyed += n;
            a.notes.push(format!("{n} of your presence destroyed"));
            collapse(l, n, ctx, a);
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
            } else if let Some(n) = sc.strip_prefix("each land within {").and_then(|r| r.split("range").next()).and_then(|n| n.parse::<u32>().ok()) {
                // "In each land within {1Range}": the target and every land that near, Oceans aside
                island.lands.keys().filter(|id| within(island, target, id, n) && island.lands.get(*id).is_some_and(|l| l.terrain != Some(Terrain::Ocean))).cloned().collect()
            } else if let Some(kind) = sc.strip_prefix("lands you pushed {").and_then(|r| r.strip_suffix("} to")).and_then(piece_of_word) {
                // Vigor of the Breaking Dawn: where this Power pushed them
                let mut to: Vec<LandId> = a.moves.iter().filter(|m| m.from == *target && m.to != *target && m.n > 0 && m.piece.as_ref() == Some(&kind)).map(|m| m.to.clone()).collect();
                to.sort();
                to.dedup();
                to
            } else if sc == "the land each invader was pushed to" {
                // Violent Windstorms: once for each Invader, where it went
                a.moves.iter().filter(|m| m.from == *target && m.to != *target && matches!(m.piece, Some(PieceFilter::Piece(k)) if k != PieceKind::Dahan)).flat_map(|m| std::iter::repeat_n(m.to.clone(), m.n as usize)).collect()
            } else if let Some(kind) = sc.strip_prefix("the land you moved {").and_then(|r| r.strip_suffix("} to")).and_then(piece_of_word) {
                // Wrap in Wings of Sunlight's "that land": where this Power moved them out of the target
                let mut to: Vec<LandId> = a.moves.iter().filter(|m| m.from == *target && m.to != *target && m.n > 0 && m.piece.as_ref() == Some(&kind)).map(|m| m.to.clone()).collect();
                to.dedup();
                to
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
/// The effects a Power runs with these elements: its own, then each tier the elements reach
/// in order, a tier that says "Instead" replacing everything reached before it ("Instead, 3
/// Fear and Defend 4" is 3 Fear and Defend 4, not 5 and 6).
pub fn reached_effects(card_effects: &[Effect], thresholds: &[(ElementSet, Vec<Effect>)], elements: &ElementSet) -> Vec<Effect> {
    reached_effects_in(card_effects, thresholds, elements, None, None)
}

/// `reached_effects`, a tier's costs beyond elements checked too: the Beasts in `land` (the
/// Spirit's Sacred Site counting as one where `site_beast`), the presence the line says it
/// destroys there for the Power (`ctx.kills`, Explosive Eruption's X: unstated, unmet);
/// Energy and Power Card costs are taken as paid (the simulator keeps no purse for them).
pub fn reached_effects_in(card_effects: &[Effect], thresholds: &[(ElementSet, Vec<Effect>)], elements: &ElementSet, land: Option<&Land>, ctx: Option<&Ctx>) -> Vec<Effect> {
    let met = |what: &str, n: u8| -> bool {
        match what {
            "Beast" => land.is_some_and(|l| l.c.beasts + (ctx.is_some_and(|c| c.site_beast) && l.c.presence >= 2) as u32 >= n as u32),
            "DestroyedPresence" => {
                let x = ctx.and_then(|c| c.kills.as_ref()).map(|k| k.iter().filter(|(at, f)| land.is_some_and(|l| l.id == *at) && *f == PieceFilter::Presence).count()).unwrap_or(0);
                x >= n as usize
            }
            _ => true,
        }
    };
    let mut out: Vec<Effect> = card_effects.to_vec();
    // where a "First, ..." threshold goes: before the card's own effects, in printed order
    let mut first_at = 0usize;
    for (_, effects) in thresholds.iter().filter(|(need, _)| elements.satisfies(need)) {
        let costs: Vec<(&String, u8)> = effects.iter().filter_map(|e| if let Effect::Requires { what, count } = e { Some((what, *count)) } else { None }).collect();
        if !costs.iter().all(|(w, n)| met(w, *n)) {
            continue;
        }
        let effects: Vec<Effect> = effects.iter().filter(|e| !matches!(e, Effect::Requires { .. })).cloned().collect();
        match effects.split_first() {
            Some((Effect::Instead, rest)) => {
                out.clear();
                out.extend(rest.iter().cloned());
            }
            Some((Effect::First, rest)) => {
                let at = first_at.min(out.len());
                out.splice(at..at, rest.iter().cloned());
                first_at = at + rest.len();
            }
            _ => out.extend(effects.iter().cloned()),
        }
    }
    out
}

pub fn apply(card_effects: &[Effect], thresholds: &[(ElementSet, Vec<Effect>)], island: &mut Island, target: &si_model::island::LandId, ctx: &Ctx) -> Applied {
    let mut a = Applied::default();
    let chosen = reached_effects_in(card_effects, thresholds, &ctx.elements, island.lands.get(target), Some(ctx));
    // Russia 2: an Explorer the line moves out of the land though the Power pushes none is the
    // one it would have destroyed and that fled: its kill stands among the stated ones
    let flees = ctx.rules.as_ref().is_some_and(|r| r.explorers_flee());
    let explorer = PieceFilter::Piece(PieceKind::Explorer);
    let fled = flees && ctx.kills.is_some() && !moves_kind(&chosen, &explorer) && ctx.moves.iter().any(|m| m.from == *target && m.piece.as_ref() == Some(&explorer));
    let widened;
    let ctx = if fled {
        let mut c = ctx.clone();
        c.kills.get_or_insert_with(Vec::new).push((target.clone(), explorer.clone()));
        widened = c;
        &widened
    } else {
        ctx
    };
    for e in chosen {
        apply_one(&e, island, target, ctx, &mut a);
    }
    if ctx.rules.as_ref().is_some_and(|r| r.explorers_flee()) {
        explorer_flees(island, ctx, &mut a);
    }
    // a piece that left took its wound along: no wound outlives its piece
    for l in island.lands.values_mut() {
        l.trim_wounds();
    }
    a
}

/// The laws an effect application keeps, stated from outside the executor, and the
/// generators its fuzz test draws from (`tests::every_effect_keeps_its_laws`). The corpus
/// fuzzer and `si fuzz --rules` hold every card and innate tier in the definitions to the
/// same laws.
pub mod oracle {
    use super::*;
    use si_model::oracle::{check, Rng, Violation};
    use si_record::Counts;
    use std::collections::BTreeMap;

    /// The kinds the laws count, as the notation names them.
    pub const KINDS: [&str; 11] = ["E", "T", "C", "Dh", "Bl", "Pr", "Be", "Di", "St", "Wi", "Ba"];

    pub fn count(c: &Counts, i: usize) -> u32 {
        [c.explorers, c.towns, c.cities, c.dahan, c.blight, c.presence, c.beasts, c.disease, c.strife, c.wilds, c.badlands][i]
    }

    fn index_of(f: &PieceFilter) -> Option<usize> {
        Some(match f {
            PieceFilter::Piece(PieceKind::Explorer) => 0,
            PieceFilter::Piece(PieceKind::Town) => 1,
            PieceFilter::Piece(PieceKind::City) => 2,
            PieceFilter::Piece(PieceKind::Dahan) => 3,
            PieceFilter::Blight => 4,
            PieceFilter::Presence => 5,
            PieceFilter::Token(TokenKind::Beast) => 6,
            PieceFilter::Token(TokenKind::Disease) => 7,
            PieceFilter::Token(TokenKind::Strife) => 8,
            PieceFilter::Token(TokenKind::Wilds) => 9,
            PieceFilter::Token(TokenKind::Badlands) => 10,
            _ => return None,
        })
    }

    pub fn totals(island: &Island) -> [u32; 11] {
        let mut t = [0u32; 11];
        for l in island.lands.values() {
            for (i, x) in t.iter_mut().enumerate() {
                *x += count(&l.c, i);
            }
        }
        t
    }

    /// The kinds an effect list may put on the island (by index into `KINDS`): what it
    /// adds, replaces with, builds, ravages or spreads. Every other kind can only move or go.
    pub fn may_add(es: &[Effect]) -> [bool; 11] {
        let mut m = [false; 11];
        fn walk(es: &[Effect], m: &mut [bool; 11]) {
            for e in es {
                match e {
                    Effect::AddDahan(_) => m[3] = true,
                    Effect::AddBlight(_) => m[4] = true,
                    Effect::AddPresence { .. } => m[5] = true,
                    Effect::AddToken { kind, .. } | Effect::Spread { kind, .. } => {
                        if let Some(i) = index_of(&PieceFilter::Token(*kind)) {
                            m[i] = true;
                        }
                    }
                    Effect::AddChosenToken(_) | Effect::TokenChoice { .. } => m[6..].iter_mut().for_each(|x| *x = true),
                    Effect::Replace { to, .. } => {
                        if let Some(i) = index_of(to) {
                            m[i] = true;
                        }
                    }
                    Effect::AddInvaders { kind, .. } => {
                        if let Some(i) = index_of(&PieceFilter::Piece(*kind)) {
                            m[i] = true;
                        }
                    }
                    Effect::Downgrade(_) => {
                        m[0] = true;
                        m[1] = true;
                    }
                    Effect::Build | Effect::Ravage | Effect::InvaderRule(_) => {
                        m[0..3].iter_mut().for_each(|x| *x = true);
                        m[4] = true;
                    }
                    Effect::Per { then, .. } | Effect::Conditional { then, .. } | Effect::Area { then, .. } | Effect::Trigger { then, .. } => walk(then, m),
                    Effect::Choice(bs) => bs.iter().for_each(|b| walk(b, m)),
                    _ => {}
                }
            }
        }
        walk(es, &mut m);
        m
    }

    fn only(es: &[Effect], ok: &dyn Fn(&Effect) -> bool) -> bool {
        es.iter().all(|e| match e {
            Effect::Per { then, .. } | Effect::Conditional { then, .. } => only(then, ok),
            Effect::Choice(bs) => bs.iter().all(|b| only(b, ok)),
            _ => ok(e),
        })
    }

    /// The effects `apply` runs: the card's own and every threshold the elements reach.
    pub fn reached(effects: &[Effect], thresholds: &[(ElementSet, Vec<Effect>)], ctx: &Ctx, land: Option<&Land>) -> Vec<Effect> {
        reached_effects_in(effects, thresholds, &ctx.elements, land, Some(ctx))
    }

    /// The laws of one `apply` on `target`, from `before` to `after`:
    /// - no piece from nowhere: a kind the effects cannot add never grows in number;
    /// - moves only along the island's own edges, each of at least one piece;
    /// - a list of gathers and pushes only keeps every kind's number;
    /// - what it says it destroyed was there to destroy;
    /// - stated kills are all it destroys; an exact bracket is all it moves;
    /// - a list that acts in one land changes that land and the lands its moves touch only;
    /// - Damage kills no more Health than it deals.
    pub fn apply_holds(effects: &[Effect], thresholds: &[(ElementSet, Vec<Effect>)], target: &LandId, ctx: &Ctx, before: &Island, after: &Island, a: &Applied) -> Vec<Violation> {
        let mut out = Vec::new();
        let all = reached(effects, thresholds, ctx, before.lands.get(target));
        let adds = may_add(&all);
        let (t0, t1) = (totals(before), totals(after));
        for i in 0..11 {
            check(&mut out, adds[i] || t1[i] <= t0[i], "no piece from nowhere", || format!("{} on the island {} -> {}", KINDS[i], t0[i], t1[i]));
        }
        for m in &a.moves {
            // a Beast (and a Sacred Site moved as one) may go two lands with Fly Fast as Thought
            let beast = matches!(m.piece, Some(PieceFilter::Token(TokenKind::Beast)) | Some(PieceFilter::Presence));
            // a presence added from the island comes from as far as the Power's range
            let placed = m.piece == Some(PieceFilter::Presence);
            let adjacent = placed || before.lands.get(&m.from).is_some_and(|l| l.adjacent.contains(&m.to)) || (ctx.beasts_far && beast && two_away(before, &m.from).contains(&m.to));
            check(&mut out, adjacent && m.n > 0, "moves along the island's edges", || format!("{} from {} to {}", m.n, m.from, m.to));
        }
        if only(&all, &|e| matches!(e, Effect::Push { .. } | Effect::Gather { .. })) {
            for i in 0..11 {
                check(&mut out, t1[i] == t0[i], "moving keeps every kind's number", || format!("{} {} -> {}", KINDS[i], t0[i], t1[i]));
            }
        }
        // what it destroyed was there: before in the land, or moved in by this application
        let mut by: BTreeMap<(LandId, usize), u32> = BTreeMap::new();
        for (l, k) in &a.killed {
            if let Some(i) = index_of(k) {
                *by.entry((l.clone(), i)).or_default() += 1;
            }
        }
        // (presence may also arrive along with Dahan or as a Sacred Site moved as a Beast)
        let presence_travels = ctx.presence_with_dahan || ctx.site_beast;
        for ((l, i), n) in by.iter().filter(|((_, i), _)| !adds[*i] && !(*i == 5 && presence_travels)) {
            let had = before.lands.get(l).map(|x| count(&x.c, *i)).unwrap_or(0);
            let came: u32 = a.moves.iter().filter(|m| m.to == *l && m.piece.as_ref().and_then(index_of).is_none_or(|j| j == *i)).map(|m| m.n).sum();
            check(&mut out, *n <= had + came, "destroys only what is there", || format!("{n} {} in {l}, {had} there", KINDS[*i]));
        }
        if let Some(stated) = &ctx.kills {
            let mut left: Vec<(LandId, PieceFilter)> = stated.clone();
            for k in &a.killed {
                match left.iter().position(|x| x == k) {
                    Some(p) => {
                        left.remove(p);
                    }
                    None => out.push(Violation::new("stated kills are all it destroys", format!("{} {:?} not stated", k.0, k.1))),
                }
            }
        }
        if ctx.moves_exact {
            // along each path, no more than the asks on it (an ask that names no kind, or a
            // kind that takes the piece, Invaders an Explorer)
            for m in &a.moves {
                let covers = |x: &Move, p: &Option<PieceFilter>| x.piece.as_ref().is_none_or(|k| p.as_ref().is_some_and(|u| k == u || allows(std::slice::from_ref(k), u)));
                let asked: u32 = ctx.moves.iter().filter(|x| x.from == m.from && x.to == m.to && covers(x, &m.piece)).map(|x| x.n).sum();
                let made: u32 = a.moves.iter().filter(|x| x.from == m.from && x.to == m.to).map(|x| x.n).sum();
                check(&mut out, made <= asked.max(ctx.moves.iter().filter(|x| x.from == m.from && x.to == m.to).map(|x| x.n).sum()) && asked > 0, "an exact bracket is all it moves", || format!("{made} from {} to {}, {asked} asked", m.from, m.to));
            }
        }
        let local = only(&all, &|e| !matches!(e, Effect::Area { .. } | Effect::Trigger { .. } | Effect::Ravage | Effect::Build | Effect::InvaderRule(_) | Effect::Spread { .. }));
        if local {
            for (id, l) in &after.lands {
                let was = before.lands.get(id).map(|x| x.c).unwrap_or_default();
                let mut c0 = was;
                let mut c1 = l.c;
                // presence moves with Dahan and gathers toward targets by spirit rules
                c0.presence = 0;
                c1.presence = 0;
                let touched = id == target || a.moves.iter().any(|m| m.from == *id || m.to == *id);
                check(&mut out, touched || c0 == c1, "acts where it says", || format!("{id} changed: {was:?} -> {:?}", l.c));
            }
        }
        // pooled Damage only: "to each" deals its amount to every piece
        let damages_only = only(&all, &|e| !matches!(e, Effect::Damage { to: DamageTarget::EachInvader | DamageTarget::EachTownCity | DamageTarget::EachDahan, .. } | Effect::Destroy { .. } | Effect::Remove { .. } | Effect::RemoveHealth(_) | Effect::Ravage | Effect::Infighting(_) | Effect::Duel { .. } | Effect::Downgrade(_) | Effect::InvaderRule(_) | Effect::Replace { .. } | Effect::Area { .. } | Effect::Trigger { .. }));
        if damages_only {
            // the Damage already on the target's Invaders this turn counts toward their Health
            let wounded: u32 = before.lands.get(target).map(|l| l.wounds.iter().filter(|(k, _)| k.is_invader()).map(|(_, d)| *d).sum()).unwrap_or(0);
            let health: u32 = a.destroyed.iter().filter(|k| k.is_invader()).map(|k| k.health() as u32 + ctx.health_bonus).sum();
            check(&mut out, health <= a.damage + wounded, "Damage kills no more Health than it deals", || format!("{health} Health destroyed by {} Damage, {wounded} on them already", a.damage));
        }
        out.extend(crate::invaders::oracle::wounds_hold(after, ctx.health_bonus, 0));
        out
    }

    /// One plain effect's own numbers, when it is the whole list: Fear n is n Fear, Defend n
    /// is n Defend, an Add adds what it says in its land, Remove Blight takes what is there
    /// up to n, a Push or Gather moves no more than it says (and, unasked, all it may of
    /// Invaders), a Destroy takes what it says while there is any.
    pub fn simple_holds(e: &Effect, target: &LandId, ctx: &Ctx, before: &Island, after: &Island, a: &Applied) -> Vec<Violation> {
        let mut out = Vec::new();
        let (Some(b), Some(x)) = (before.lands.get(target), after.lands.get(target)) else { return out };
        // each kind the list names, once (Invaders and Explorer overlap)
        let of = |c: &Counts, what: &[PieceFilter]| -> u32 {
            let mut kinds: Vec<usize> = what.iter().flat_map(|f| match f { PieceFilter::AnyInvader => vec![0, 1, 2], f => index_of(f).into_iter().collect() }).collect();
            kinds.sort();
            kinds.dedup();
            kinds.iter().map(|i| count(c, *i)).sum()
        };
        let limit = |n: u8, avail: u32| -> u32 { if n == 255 { avail } else if n == 128 { avail / 2 } else { (n as u32).min(avail) } };
        match e {
            Effect::Fear(n) => check(&mut out, a.fear == *n as u32, "Fear as stated", || format!("{} for Fear {n}", a.fear)),
            Effect::Defend(n) => check(&mut out, x.defend == b.defend + *n as u32, "Defend as stated", || format!("{} -> {} for Defend {n}", b.defend, x.defend)),
            Effect::AddDahan(n) => check(&mut out, x.c.dahan == b.c.dahan + *n as u32, "adds as stated", || format!("Dahan {} -> {} for {n}", b.c.dahan, x.c.dahan)),
            Effect::AddBlight(n) => check(&mut out, x.c.blight == b.c.blight + *n as u32, "adds as stated", || format!("Blight {} -> {} for {n}", b.c.blight, x.c.blight)),
            Effect::AddToken { kind, count: n } => {
                if let Some(i) = index_of(&PieceFilter::Token(*kind)) {
                    check(&mut out, count(&x.c, i) == count(&b.c, i) + *n as u32, "adds as stated", || format!("{} {} -> {} for {n}", KINDS[i], count(&b.c, i), count(&x.c, i)));
                }
            }
            Effect::RemoveBlight(n) => check(&mut out, x.c.blight == b.c.blight - (*n as u32).min(b.c.blight), "removes as stated", || format!("Blight {} -> {} for {n}", b.c.blight, x.c.blight)),
            Effect::Push { count: n, what } => {
                let (have, left) = (of(&b.c, what), of(&x.c, what));
                let gone = have.saturating_sub(left);
                check(&mut out, gone <= limit(*n, have), "pushes no more than it says", || format!("{gone} of {have} for Push {n}"));
                let invaders_only = what.iter().all(|f| !matches!(f, PieceFilter::Piece(PieceKind::Dahan)));
                if invaders_only && ctx.moves.is_empty() && !ctx.moves_exact && after.lands.values().any(|l| l.id != *target && b.adjacent.contains(&l.id) && l.terrain != Some(Terrain::Ocean)) {
                    check(&mut out, gone == limit(*n, have), "pushes all it says, unasked", || format!("{gone} of {have} for Push {n}"));
                }
            }
            Effect::Gather { count: n, what } => {
                let came = of(&x.c, what).saturating_sub(of(&b.c, what));
                if *n < 128 {
                    check(&mut out, came <= *n as u32, "gathers no more than it says", || format!("{came} for Gather {n}"));
                }
            }
            Effect::Destroy { count: n, what } if ctx.kills.is_none() => {
                let (have, left) = (of(&b.c, what), of(&x.c, what));
                check(&mut out, have - left == limit(*n, have), "destroys as stated", || format!("{} of {have} for Destroy {n}", have - left));
            }
            _ => {}
        }
        out
    }

    /// A small island to act on (the Invader steps' generator: an Ocean and six lands in a
    /// ring with two chords, random pieces, tokens, Defend and skips).
    pub fn arbitrary_island(rng: &mut Rng) -> Island {
        crate::invaders::oracle::arbitrary_island(rng)
    }

    pub fn arbitrary_filter(rng: &mut Rng) -> PieceFilter {
        let all = [
            PieceFilter::Piece(PieceKind::Explorer),
            PieceFilter::Piece(PieceKind::Town),
            PieceFilter::Piece(PieceKind::City),
            PieceFilter::Piece(PieceKind::Dahan),
            PieceFilter::AnyInvader,
            PieceFilter::Token(TokenKind::Beast),
            PieceFilter::Token(TokenKind::Disease),
            PieceFilter::Token(TokenKind::Wilds),
            PieceFilter::Token(TokenKind::Strife),
            PieceFilter::Token(TokenKind::Badlands),
        ];
        all[rng.below(all.len())].clone()
    }

    fn filters(rng: &mut Rng) -> Vec<PieceFilter> {
        let mut v = vec![arbitrary_filter(rng)];
        if rng.one_in(3) {
            let f = arbitrary_filter(rng);
            if !v.contains(&f) {
                v.push(f);
            }
        }
        v
    }

    fn count_word(rng: &mut Rng) -> u8 {
        if rng.one_in(8) { 255 } else { rng.between(1, 3) as u8 }
    }

    /// An effect the simulator executes, nested up to `depth` (a per, an "or", an "if").
    pub fn arbitrary_effect(rng: &mut Rng, depth: u32) -> Effect {
        let targets = [DamageTarget::Invaders, DamageTarget::TownCityOnly, DamageTarget::ExplorerTownOnly, DamageTarget::Town, DamageTarget::Dahan, DamageTarget::EachInvader, DamageTarget::EachTownCity, DamageTarget::EachDahan];
        let tokens = [TokenKind::Beast, TokenKind::Disease, TokenKind::Wilds, TokenKind::Strife, TokenKind::Badlands];
        let k = rng.below(if depth > 0 { 15 } else { 12 });
        match k {
            0 => Effect::Damage { amount: rng.between(0, 6) as u8, to: targets[rng.below(targets.len())] },
            1 => Effect::Defend(rng.between(1, 4) as u8),
            2 => Effect::Push { count: count_word(rng), what: filters(rng) },
            3 => Effect::Gather { count: count_word(rng), what: filters(rng) },
            4 => Effect::Fear(rng.between(1, 3) as u8),
            5 => Effect::RemoveBlight(rng.between(1, 2) as u8),
            6 => Effect::AddBlight(1),
            7 => Effect::AddDahan(rng.between(1, 2) as u8),
            8 => Effect::AddToken { kind: tokens[rng.below(tokens.len())], count: rng.between(1, 2) as u8 },
            9 => Effect::Destroy { count: count_word(rng), what: filters(rng) },
            10 => Effect::Remove { count: count_word(rng), what: filters(rng) },
            11 => Effect::FearIfInvaders(rng.between(1, 2) as u8),
            12 => Effect::Per { per: arbitrary_filter(rng), then: vec![arbitrary_effect(rng, depth - 1)] },
            13 => Effect::Choice(vec![vec![arbitrary_effect(rng, depth - 1)], vec![arbitrary_effect(rng, depth - 1)]]),
            _ => Effect::Conditional { condition: ["Invaders are present", "target land has {Dahan}", "target land has {Beast}"][rng.below(3)].into(), then: vec![arbitrary_effect(rng, depth - 1)] },
        }
    }

    /// What a line may say of a play: asks to and from the neighbors (a kind named or not),
    /// a bracket exact or not, kills stated or not, presence along with the Dahan or not.
    pub fn arbitrary_ctx(rng: &mut Rng, island: &Island, target: &LandId) -> Ctx {
        let mut ctx = Ctx::plain(rng.between(0, 1), ElementSet::new());
        let around: Vec<LandId> = island.lands.get(target).map(|l| l.adjacent.clone()).unwrap_or_default();
        for _ in 0..rng.below(3) {
            let Some(n) = rng.pick(&around).cloned() else { break };
            let piece = if rng.coin() { None } else { Some(arbitrary_filter(rng)) };
            let (from, to) = if rng.coin() { (n, target.clone()) } else { (target.clone(), n) };
            ctx.moves.push(Move { from, to, piece, n: rng.between(1, 3) });
        }
        ctx.moves_exact = !ctx.moves.is_empty() && rng.coin();
        if rng.one_in(3) {
            let l = island.lands.get(target);
            let mut k = Vec::new();
            for _ in 0..rng.below(3) {
                let f = [PieceFilter::Piece(PieceKind::Explorer), PieceFilter::Piece(PieceKind::Town), PieceFilter::Piece(PieceKind::City), PieceFilter::Token(TokenKind::Beast), PieceFilter::Presence][rng.below(5)].clone();
                if l.is_some() {
                    k.push((target.clone(), f));
                }
            }
            ctx.kills = Some(k);
        }
        ctx.presence_with_dahan = rng.one_in(4);
        // Many Minds' rules now and then: Beasts two lands away, the Sacred Site as a Beast
        ctx.beasts_far = rng.one_in(5);
        ctx.site_beast = rng.one_in(5);
        // Volcano's Collapse now and then: Damage to both sides where presence is destroyed
        ctx.collapse = rng.one_in(5) as u32;
        if ctx.site_beast && rng.coin() {
            if let Some(n) = rng.pick(&around).cloned() {
                let (from, to) = if rng.coin() { (n, target.clone()) } else { (target.clone(), n) };
                ctx.moves.push(Move { from, to, piece: Some(PieceFilter::Presence), n: 2 });
            }
        }
        ctx
    }
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
    fn russias_explorer_flees_the_first_destroy_of_an_action() {
        let (a1, a2): (LandId, LandId) = ("A1".parse().unwrap(), "A2".parse().unwrap());
        let russia = crate::invaders::Rules { adversary: Some("TheTsardomOfRussia".into()), level: 2, ..Default::default() };
        let ctx2 = Ctx { rules: Some(russia), ..ctx() };
        // 3 Damage on 1 Explorer and 1 Town: the Town dies, the Explorer goes to A2, 1 Fear
        let mut isl = three_lands();
        isl.lands.get_mut(&a1).unwrap().c = Counts { explorers: 1, towns: 1, ..Default::default() };
        let a = apply(&[Effect::Damage { amount: 3, to: DamageTarget::Invaders }], &[], &mut isl, &a1, &Ctx { moves: vec![Move { from: a1.clone(), to: a2.clone(), piece: Some(PieceFilter::Piece(PieceKind::Explorer)), n: 1 }], ..ctx2.clone() });
        assert_eq!((isl.lands[&a1].c.explorers, isl.lands[&a1].c.towns, isl.lands[&a2].c.explorers, a.fear), (0, 0, 1, 1), "{:?}", a.notes);
        assert_eq!(a.destroyed, vec![PieceKind::Town]);
        // only the first: two Explorers destroyed, one flees
        let mut isl = three_lands();
        isl.lands.get_mut(&a1).unwrap().c = Counts { explorers: 2, ..Default::default() };
        let a = apply(&[Effect::Damage { amount: 2, to: DamageTarget::Invaders }], &[], &mut isl, &a1, &ctx2);
        assert_eq!((a.destroyed.len(), a.fear), (1, 1));
        // no Russia, no flight
        let mut isl = three_lands();
        isl.lands.get_mut(&a1).unwrap().c = Counts { explorers: 1, ..Default::default() };
        let a = apply(&[Effect::Damage { amount: 1, to: DamageTarget::Invaders }], &[], &mut isl, &a1, &ctx());
        assert_eq!((a.destroyed.len(), a.fear), (1, 0));
    }

    #[test]
    fn a_move_goes_where_the_line_says_within_its_reach() {
        let (a1, a2, a3): (LandId, LandId, LandId) = ("A1".parse().unwrap(), "A2".parse().unwrap(), "A3".parse().unwrap());
        let be = PieceFilter::Token(TokenKind::Beast);
        let dh = PieceFilter::Piece(PieceKind::Dahan);
        let mv = |from: &LandId, to: &LandId, piece: Option<PieceFilter>, n: u32| Move { from: from.clone(), to: to.clone(), piece, n };
        // Guide the Way on Feathered Wings: the Beast out to A2 and back (listed back first, as
        // the record may), 2 Dahan along on the way out (bare: a Dahan's move)
        let guide = [Effect::Move { count: 1, what: vec![be.clone()], reach: MoveReach::Path(2) }, Effect::Move { count: 2, what: vec![dh.clone()], reach: MoveReach::Along(2) }];
        let mut isl = three_lands();
        isl.lands.get_mut(&a1).unwrap().c.beasts = 1;
        let asked = Ctx { moves: vec![mv(&a2, &a1, Some(be.clone()), 1), mv(&a1, &a2, Some(be.clone()), 1), mv(&a1, &a2, None, 2)], moves_exact: true, ..ctx() };
        apply(&guide, &[], &mut isl, &a1, &asked);
        assert_eq!((isl.lands[&a1].c.beasts, isl.lands[&a1].c.dahan, isl.lands[&a2].c.dahan), (1, 0, 3), "the Beast back home, the 2 Dahan left in A2");
        // a Dahan cannot go where the Beast did not
        let mut isl = three_lands();
        isl.lands.get_mut(&a1).unwrap().c.beasts = 1;
        let asked = Ctx { moves: vec![mv(&a1, &a2, Some(be.clone()), 1), mv(&a1, &a3, None, 2)], ..ctx() };
        apply(&guide, &[], &mut isl, &a1, &asked);
        assert_eq!(isl.lands[&a3].c.dahan, 2, "no Dahan to A3");
        // Wrap in Wings of Sunlight: to any land, unstated nothing
        let wrap = [Effect::Move { count: 5, what: vec![dh.clone()], reach: MoveReach::From(255) }];
        let mut isl = three_lands();
        let a = apply(&wrap, &[], &mut isl, &a1, &ctx());
        assert_eq!(isl.lands[&a1].c.dahan, 2);
        assert!(a.notes.iter().any(|n| n.contains("player's choice")), "{:?}", a.notes);
        // A2 to A3 is two lands away: "to any land" takes it
        let mut isl = three_lands();
        apply(&wrap, &[], &mut isl, &a1, &Ctx { moves: vec![mv(&a1, &a3, Some(dh.clone()), 2)], ..ctx() });
        assert_eq!(isl.lands[&a3].c.dahan, 4);
    }

    #[test]
    fn a_gather_takes_from_the_land_asked_and_a_push_lands_where_asked() {
        let a1: si_model::island::LandId = "A1".parse().unwrap();
        let asked = Ctx { moves: vec![Move { from: "A2".parse().unwrap(), to: a1.clone(), piece: None, n: 1 }, Move { from: a1.clone(), to: "A3".parse().unwrap(), piece: None, n: 1 }], ..ctx() };
        let mut isl = three_lands();
        let a = apply(&[Effect::Gather { count: 1, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }, Effect::Push { count: 1, what: vec![PieceFilter::Piece(PieceKind::Dahan)] }], &[], &mut isl, &a1, &asked);
        assert_eq!((isl.lands[&a1].c.dahan, isl.lands[&"A2".parse().unwrap()].c.dahan, isl.lands[&"A3".parse().unwrap()].c.dahan), (2, 0, 3), "one Dahan in from A2 (not the fuller A3), one out to A3");
        assert_eq!(a.moves, vec![Move { from: "A2".parse().unwrap(), to: a1.clone(), piece: Some(PieceFilter::Piece(PieceKind::Dahan)), n: 1 }, Move { from: a1.clone(), to: "A3".parse().unwrap(), piece: Some(PieceFilter::Piece(PieceKind::Dahan)), n: 1 }]);
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
        assert_eq!(a.moves, vec![Move { from: a1.clone(), to: "A3".parse().unwrap(), piece: Some(PieceFilter::Piece(PieceKind::Dahan)), n: 2 }]);
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

    #[test]
    fn every_effect_keeps_its_laws() {
        use super::oracle::*;
        use si_model::oracle::{property, Violation};
        property(
            "effects",
            1,
            6000,
            |rng| {
                let island = arbitrary_island(rng);
                let target = LandId::new("A", rng.between(1, 6) as u8);
                let effects: Vec<Effect> = (0..rng.between(1, 3)).map(|_| arbitrary_effect(rng, 2)).collect();
                let ctx = arbitrary_ctx(rng, &island, &target);
                (island, target, effects, ctx)
            },
            |(island, target, effects, ctx)| {
                let mut after = island.clone();
                let ran = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| apply(effects, &[], &mut after, target, ctx)));
                let Ok(a) = ran else { return vec![Violation::new("no panic", "apply panicked")] };
                let mut v = apply_holds(effects, &[], target, ctx, island, &after, &a);
                if let [e] = effects.as_slice() {
                    v.extend(simple_holds(e, target, ctx, island, &after, &a));
                }
                v
            },
        );
    }

    #[test]
    fn an_instead_tier_replaces_the_tiers_before_it() {
        // Beset and Confound the Invaders, two tiers reached: 3 Fear and Defend 4, not 5 and 6
        let air = |n: u8| {
            let mut e = ElementSet::new();
            e.add(si_model::element::Element::Air, n);
            e
        };
        let tiers = vec![(air(1), vec![Effect::Fear(2), Effect::Defend(2)]), (air(2), vec![Effect::Instead, Effect::Fear(3), Effect::Defend(4)])];
        let mut isl = three_lands();
        let a1: LandId = "A1".parse().unwrap();
        let a = apply(&[], &tiers, &mut isl, &a1, &Ctx::plain(0, air(2)));
        assert_eq!((a.fear, a.defend), (3, 4));
        let a = apply(&[], &tiers, &mut three_lands(), &a1, &Ctx::plain(0, air(1)));
        assert_eq!((a.fear, a.defend), (2, 2), "the first tier alone");
    }
}
