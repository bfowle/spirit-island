//! Spirit actions: the parsed `Effect` language executed on a land. Where the
//! rules leave a choice to the player (which Invaders take the Damage, which
//! pieces are Pushed, which branch of an "or"), the executor takes a stated
//! default and says so, so the verifier can tell a choice from a rule gap.

use serde::{Deserialize, Serialize};
use si_model::island::{LandId, PieceKind, Terrain, TokenKind};
use si_model::power::{DamageTarget, Effect, PieceFilter};
use si_model::ElementSet;

use crate::invaders::{Island, Land};

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
            let mut left = total;
            for k in allowed {
                let h = k.health() as u32 + ctx.health_bonus;
                loop {
                    let n = match k {
                        PieceKind::Explorer => &mut l.c.explorers,
                        PieceKind::Town => &mut l.c.towns,
                        PieceKind::City => &mut l.c.cities,
                        PieceKind::Dahan => unreachable!(),
                    };
                    if *n == 0 || left < h {
                        break;
                    }
                    *n -= 1;
                    left -= h;
                    a.destroyed.push(k);
                }
            }
            if total > 0 {
                a.notes.push("damage distribution: cheapest Invaders first (player's choice)".into());
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
            let avail: u32 = what.iter().map(|f| count_of(l, f)).sum();
            let n = if *count == 255 { avail } else if *count == 128 { avail / 2 } else { (*count as u32).min(avail) };
            let removed = remove_pieces(l, what, n, false);
            a.pushed += removed.len() as u32;
            a.pushed_pieces.extend(removed.iter().copied());
            if n < avail {
                a.notes.push("push: which pieces is the player's choice".into());
            }
        }
        Effect::Gather { count, what } => {
            // sources are the adjacent lands: take from the fullest first (player's choice)
            let adjacent = l.adjacent.clone();
            let want = if *count >= 128 { 0 } else { *count as u32 };
            let mut left = want;
            let what = what.clone();
            let target_id = target.clone();
            for f in &what {
                let mut sources: Vec<(u32, si_model::island::LandId)> = adjacent.iter().filter_map(|a| island.lands.get(a).map(|n| (count_of(n, f), a.clone()))).filter(|(n, _)| *n > 0).collect();
                sources.sort_by(|x, y| y.0.cmp(&x.0));
                for (avail, src) in sources {
                    if left == 0 {
                        break;
                    }
                    let take = avail.min(left);
                    if let Some(s) = island.lands.get_mut(&src) {
                        remove_pieces(s, std::slice::from_ref(f), take, true);
                    }
                    if let Some(t) = island.lands.get_mut(&target_id) {
                        match f {
                            PieceFilter::Piece(PieceKind::Dahan) => t.c.dahan += take,
                            PieceFilter::Piece(PieceKind::Explorer) => t.c.explorers += take,
                            PieceFilter::Piece(PieceKind::Town) => t.c.towns += take,
                            PieceFilter::Piece(PieceKind::City) => t.c.cities += take,
                            PieceFilter::Token(TokenKind::Beast) => t.c.beasts += take,
                            _ => {}
                        }
                    }
                    left -= take;
                }
            }
            a.gathered += want - left;
            if want > 0 {
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
        Effect::AddPresence(n) => l.c.presence += *n as u32,
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
            let n = match per {
                PieceFilter::Other(desc) => per_count_other(desc, island, target, ctx),
                _ => count_of(l, per),
            };
            // "N Damage per X" is one Damage amount (Badlands add once), not N instances
            match then.as_slice() {
                [Effect::Damage { amount, to }] => {
                    let l = island.lands.get_mut(target).unwrap();
                    damage(l, *amount as u32 * n, *to, ctx, a);
                }
                _ => {
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
            if let Some(first) = branches.first() {
                first.iter().for_each(|e| apply_one(e, island, target, ctx, a));
            }
            a.notes.push("'or': first branch taken (player's choice)".into());
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
        Ctx { health_bonus: 0, elements: ElementSet::new(), blight_as_badlands: false, rules: None, placements: vec![], picks: vec![] }
    }

    #[test]
    fn damage_kills_cheapest_first_and_badlands_add() {
        let mut isl = island(Counts { explorers: 1, towns: 1, ..Default::default() });
        let a = apply(&[Effect::Damage { amount: 2, to: DamageTarget::Invaders }], &[], &mut isl, &"A1".parse().unwrap(), &ctx());
        assert_eq!(a.destroyed, vec![PieceKind::Explorer], "2 damage: the Explorer dies, the Town (2) is not reached with 1 left");
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
