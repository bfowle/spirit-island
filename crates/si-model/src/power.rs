//! Power Cards and the effect language.

use serde::{Deserialize, Serialize};

use crate::element::ElementSet;
use crate::island::{PieceKind, Terrain, TokenKind};
use crate::CardId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Speed {
    Fast,
    Slow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerKind {
    Minor,
    Major,
    Unique,
}

/// Where a Power may be aimed, as the engine names it (`AnyLand`,
/// `LandWithDahan`, `Coastal`, `AnySpirit`, …). Kept as the engine string with
/// typed questions on top, because the list is long and still growing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetKind(pub String);

impl TargetKind {
    pub fn is_spirit(&self) -> bool {
        matches!(self.0.as_str(), "AnySpirit" | "AnotherSpirit" | "You")
    }

    pub fn requires_dahan(&self) -> bool {
        self.0.starts_with("LandWithDahan") || self.0 == "LandWithTwoDahan"
    }

    pub fn requires_invaders(&self) -> bool {
        matches!(self.0.as_str(), "LandWithInvaders" | "LandWithBlightAndInvaders" | "LandWithTownCity" | "LandWithCity" | "CoastalLandWithCity")
    }

    pub fn requires_no_blight(&self) -> bool {
        self.0 == "LandWithNoBlight"
    }

    pub fn coastal_only(&self) -> bool {
        matches!(self.0.as_str(), "Coastal" | "CoastalLandWithCity" | "CoastalOrWetland")
    }

    pub fn inland_only(&self) -> bool {
        self.0 == "Inland"
    }

    /// Terrains allowed when the target names them (`JungleWetland` …).
    pub fn terrains(&self) -> Option<Vec<Terrain>> {
        let mut out = Vec::new();
        let mut rest = self.0.as_str();
        if rest == "NonWetland" {
            return Some(vec![Terrain::Jungle, Terrain::Mountain, Terrain::Sands]);
        }
        while !rest.is_empty() {
            let Some((t, len)) = ["Jungle", "Mountain", "Sands", "Wetland"].iter().find_map(|p| rest.starts_with(p).then(|| (Terrain::parse(p).unwrap(), p.len()))) else {
                return None;
            };
            out.push(t);
            rest = &rest[len..];
        }
        (!out.is_empty()).then_some(out)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    pub distance: u8,
    /// Range is measured from a Sacred Site (2+ presence) when true.
    #[serde(default)]
    pub from_sacred_site: bool,
    /// Range must originate from a land of one of these terrains.
    #[serde(default)]
    pub origin_terrains: Vec<Terrain>,
}

/// What a card does, as structure. Parsed from the printed text
/// (`si_defs::effects`); an unparsed sentence keeps `Other(text)` so the
/// replay can report it as uncovered rather than silently wrong.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Effect {
    Damage { amount: u8, to: DamageTarget },
    /// "+N Damage" on top of another effect.
    DamageBonus(u8),
    Defend(u8),
    /// `count` 255 means "all".
    Push { count: u8, what: Vec<PieceFilter> },
    Gather { count: u8, what: Vec<PieceFilter> },
    Fear(u8),
    FearIfInvaders(u8),
    RemoveBlight(u8),
    AddBlight(u8),
    AddDahan(u8),
    /// Presence added by a Power (boons, Blazing Renewal); `range` when the card says
    /// where it may go ("up to Range 1 from their Presence").
    AddPresence { count: u8, range: Option<u8> },
    AddToken { kind: TokenKind, count: u8 },
    RemoveToken { kind: TokenKind, count: u8 },
    /// Remove pieces without Destroying them (no Fear, no counterattack).
    /// `count` 255 is "all", 128 is "half".
    Remove { count: u8, what: Vec<PieceFilter> },
    /// Tokens added among other lands of the player's choosing ("Add 4 Disease
    /// among Coastal lands other than target land"); `scope` is the printed scope.
    Spread { kind: TokenKind, count: u8, scope: String },
    /// "Invaders in target land Ravage": a Ravage the Power forces now.
    Ravage,
    /// Spirits may jointly pay Energy to remove the top card of the Fear Deck
    /// from the game (Plague Ships Sail to Distant Ports). Optional.
    FearDeckRemove { energy_per_player: u8 },
    /// Effects that apply over a scope of lands ("In each adjacent land", "In that
    /// land and each adjacent land", "both target and one adjacent land").
    Area { scope: String, then: Vec<Effect> },
    /// Effects that fire later this turn on an event ("The next time an Invader is
    /// destroyed in target land", "after target Spirit uses a Power targeting a land").
    Trigger { when: String, then: Vec<Effect>, once: bool },
    /// Blight added to the land does not cascade (Entrap the Forces of Corruption).
    NoCascade,
    /// The player picks the land the rest of the card acts on ("Choose one of target Spirit's lands").
    ChooseLand(String),
    /// Pick a token type present in an adjacent land (Hazards Spread Across the Island).
    TokenChoice { options: Vec<TokenKind>, disease_cost: u8 },
    /// Add N of the chosen token type to the target land.
    AddChosenToken(u8),
    /// Play another Power Card by paying its cost.
    PlayAnother,
    /// Up to N Slow Powers may be used as Fast this turn.
    MakeFast(u8),
    /// Reclaim N Power Cards.
    Reclaim(u8),
    /// Re-gain a forgotten Unique Power.
    RegainForgotten,
    /// Gain N Energy per something ("per Power Card they played", "equal to Dahan").
    EnergyPer { n: u8, per: String },
    /// A rule for the Invader phase in words ("During Ravage, Dahan deal damage simultaneously").
    InvaderRule(String),
    /// Dahan in the land have +N Health.
    DahanHealth(u8),
    /// During Ravage the land's Dahan deal their damage with the Invaders', so the
    /// Dahan the Ravage destroys strike back too (Words of Warning).
    DahanSimultaneous,
    /// Typed as out of the model's reach (boards appear, lands merge, decks are rearranged).
    Unmodeled(String),
    /// Acts on a Spirit's economy (Energy, cards, elements, presence on the tracks), not on the board.
    SpiritEffect(String),
    /// Rules text with nothing to execute ("This cannot target an Ocean", a parenthetical).
    Clarification(String),
    /// An expansion spirit's own token or marker (Quake, Impending, the Wandering Voice…).
    SpecialToken { token: String, text: String },
    /// Something that happens in a later phase or on a later event this turn or game.
    Timing { when: String, text: String },
    /// Movement beyond a plain Push or Gather (pieces travel with presence, Dahan ride along…).
    Movement(String),
    /// Looks at, reorders or discards from a deck.
    DeckEffect(String),
    /// The player distributes this Power's Damage or pieces among lands; the simulator keeps them on the target.
    Distribution(String),
    /// Destroy N of your own presence in the land.
    DestroyPresence(u8),
    /// Remove Invaders worth up to N Health, cheapest first.
    RemoveHealth(u8),
    /// Invaders with Strife (N of them, 255 = each) deal their Damage to the other Invaders in the land.
    Infighting(u8),
    /// N Invaders and N Dahan deal Damage to each other.
    Duel { invaders: u8, dahan: u8 },
    /// Blight is not added to the land this turn (it stays on the card).
    NoBlight,
    /// Add Invader pieces (a Replace that yields more than one).
    AddInvaders { kind: PieceKind, count: u8 },
    /// Invaders in the land Build now.
    Build,
    /// The Power may be used in the Slow phase.
    MaySlow,
    /// An amount that depends on something the card names ("X Damage, where X is …").
    Variable { what: String, by: String },
    /// The player picks a piece or Spirit the rest of the card acts on.
    Choose(String),
    /// Move pieces between lands (not a Push or Gather from the target).
    Move { count: u8, what: Vec<PieceFilter> },
    Isolate,
    /// Invaders skip their Ravage in the land.
    SkipRavage,
    SkipBuild,
    SkipExplore,
    /// Invaders skip every Action in the land this turn.
    SkipAll,
    /// Skip `n` Invader Actions of the player's choice.
    SkipAction(u8),
    /// `n` Invaders don't participate in the Ravage (255: all with Strife).
    NoParticipation(u8),
    /// Dahan take no damage from the Ravage (or fewer are destroyed).
    DahanImmune,
    /// Invaders deal `n` less Damage when they Ravage.
    InvaderDamageMinus(u8),
    /// `count` 255 means "all".
    Destroy { count: u8, what: Vec<PieceFilter> },
    Replace { count: u8, from: PieceFilter, to: PieceFilter },
    Downgrade(u8),
    GainEnergy(u8),
    GainPowerCard { kind: Option<PowerKind> },
    GainElement(u8),
    RangeBonus(u8),
    /// The Power may be used in the Fast phase (sometimes for a cost).
    MayBeFast { cost: Option<u8> },
    Repeat,
    /// The tier replaces the tiers reached before it ("Instead, 3 Fear and Defend 4"): the
    /// first effect of such a tier, a marker with no effect of its own.
    Instead,
    /// A tier's cost beyond elements, at the head of its effects (`what` the cost's name:
    /// `Beast` in the target land, `DestroyedPresence` the presence destroyed for it,
    /// `Energy`, `PowerCards`): the tier is reached only when it is met.
    Requires { what: String, count: u8 },
    /// "You may do both": the preceding Choice is not exclusive.
    Both,
    /// "For each {X}, …" / "N Damage per {X}".
    Per { per: PieceFilter, then: Vec<Effect> },
    Choice(Vec<Vec<Effect>>),
    Conditional { condition: String, then: Vec<Effect> },
    Other(String),
}

impl Effect {
    /// The effect in game words: "3 Damage to Invaders", "Push 1 Explorer and 1 Town",
    /// "for each Disease: 1 Damage". What `si cards` prints beside the printed text.
    pub fn words(&self) -> String {
        use Effect::*;
        let n = |c: u8| -> String {
            match c {
                255 => "all".into(),
                128 => "half of the".into(),
                c => c.to_string(),
            }
        };
        let list = |what: &[PieceFilter]| what.iter().map(PieceFilter::words).collect::<Vec<_>>().join("/");
        match self {
            Damage { amount, to } => format!("{amount} Damage {}", to.words()),
            DamageBonus(a) => format!("+{a} Damage"),
            Defend(d) => format!("Defend {d}"),
            Push { count, what } => format!("Push {} {}", n(*count), list(what)),
            Gather { count, what } => format!("Gather {} {}", n(*count), list(what)),
            Fear(f) => format!("{f} Fear"),
            FearIfInvaders(f) => format!("{f} Fear if Invaders are present"),
            RemoveBlight(b) => format!("remove {b} Blight"),
            AddBlight(b) => format!("add {b} Blight"),
            AddDahan(d) => format!("add {d} Dahan"),
            AddPresence { count, range } => match range {
                Some(r) => format!("add {count} presence within Range {r} of your presence"),
                None => format!("add {count} presence"),
            },
            AddToken { kind, count } => format!("add {count} {}", token_word(*kind)),
            RemoveToken { kind, count } => format!("remove {count} {}", token_word(*kind)),
            Remove { count, what } => format!("remove {} {} (not destroyed: no Fear)", n(*count), list(what)),
            Spread { kind, count, scope } => format!("add {count} {} among {scope} (player's choice of lands)", token_word(*kind)),
            Ravage => "Invaders in the target land Ravage now".into(),
            FearDeckRemove { energy_per_player } => format!("optional: {energy_per_player} Energy per player to remove the top Fear card from the game"),
            Move { count, what } => format!("move {} {}", n(*count), list(what)),
            Isolate => "Isolate the land".into(),
            SkipRavage => "Invaders skip their Ravage here".into(),
            SkipBuild => "Invaders skip their Build here".into(),
            SkipExplore => "Invaders skip their Explore here".into(),
            SkipAll => "Invaders skip every Action here this turn".into(),
            SkipAction(k) => format!("skip {k} Invader Action(s) of your choice"),
            NoParticipation(k) if *k == 255 => "Invaders with Strife do not take part in the Ravage".into(),
            NoParticipation(k) => format!("{k} Invader(s) do not take part in the Ravage"),
            DahanImmune => "Dahan take no damage from the Ravage".into(),
            InvaderDamageMinus(k) => format!("Invaders do -{k} Damage on their Ravage"),
            Destroy { count, what } => format!("destroy {} {}", n(*count), list(what)),
            Replace { count, from, to } => format!("replace {count} {} with {}", from.words(), to.words()),
            Downgrade(k) => format!("downgrade {k} Invader(s): City to Town, Town to Explorer, Explorer destroyed"),
            GainEnergy(e) => format!("gain {e} Energy"),
            GainPowerCard { kind: Some(k) } => format!("gain a {k:?} Power Card"),
            GainPowerCard { kind: None } => "gain a Power Card".into(),
            GainElement(k) => format!("gain {k} element(s)"),
            RangeBonus(k) => format!("+{k} Range"),
            MayBeFast { cost: Some(c) } => format!("may be Fast for {c} Energy"),
            MayBeFast { cost: None } => "may be Fast".into(),
            Repeat => "repeat this Power".into(),
            Instead => "instead of the tiers before".into(),
            Requires { what, count } => format!("needs {count} {what}"),
            Both => "you may do both".into(),
            Per { per, then } => format!("for each {}: {}", per.words(), then.iter().map(Effect::words).collect::<Vec<_>>().join("; ")),
            Choice(branches) => format!("either {}", branches.iter().map(|b| b.iter().map(Effect::words).collect::<Vec<_>>().join(", ")).collect::<Vec<_>>().join(" — or — ")),
            Conditional { condition, then } => format!("{condition}: {}", then.iter().map(Effect::words).collect::<Vec<_>>().join("; ")),
            Area { scope, then } => format!("in {scope}: {}", then.iter().map(Effect::words).collect::<Vec<_>>().join("; ")),
            Trigger { when, then, once } => format!("{}when {when}: {}", if *once { "once, " } else { "" }, then.iter().map(Effect::words).collect::<Vec<_>>().join("; ")),
            NoCascade => "Blight added here does not cascade".into(),
            ChooseLand(scope) => format!("choose {scope} (the card acts there)"),
            TokenChoice { options, disease_cost } => format!("choose a token type present in an adjacent land from {}{}", options.iter().map(|k| token_word(*k)).collect::<Vec<_>>().join("/"), if *disease_cost > 0 { format!(" (Disease costs {disease_cost} Energy)") } else { String::new() }),
            AddChosenToken(k) => format!("add {k} of the chosen token"),
            PlayAnother => "play another Power Card by paying its cost".into(),
            MakeFast(k) => format!("up to {k} Slow Power(s) may be used as Fast"),
            Reclaim(k) => format!("Reclaim {k} Power Card(s)"),
            RegainForgotten => "re-gain a forgotten Unique Power".into(),
            EnergyPer { n, per } => format!("gain {n} Energy per {per}"),
            InvaderRule(r) => format!("Invader-phase rule: {r}"),
            DahanHealth(k) => format!("Dahan here have +{k} Health"),
            DahanSimultaneous => "during Ravage, Dahan here deal damage simultaneously with the Invaders".into(),
            Unmodeled(r) => format!("out of the model's reach: {r}"),
            SpiritEffect(t) => format!("Spirit economy: {t}"),
            Clarification(t) => format!("rules text: {t}"),
            SpecialToken { token, text } => format!("{token} token: {text}"),
            Timing { when, text } => format!("{when}: {text}"),
            Movement(t) => format!("movement: {t}"),
            DeckEffect(t) => format!("deck: {t}"),
            Distribution(t) => format!("distribution (player's choice; kept on the target here): {t}"),
            DestroyPresence(k) => format!("destroy {k} of your presence"),
            RemoveHealth(k) => format!("remove Invaders worth up to {k} Health"),
            Infighting(k) if *k == 255 => "each Invader with Strife deals its Damage to the other Invaders".into(),
            Infighting(k) => format!("{k} Invader(s) with Strife deal their Damage to the other Invaders"),
            Duel { invaders, dahan } => format!("{invaders} Invader(s) and {dahan} Dahan deal Damage to each other"),
            NoBlight => "Blight is not added here (it stays on the card)".into(),
            AddInvaders { kind, count } => format!("add {count} {kind:?}"),
            Build => "Invaders in the land Build now".into(),
            MaySlow => "may be Slow".into(),
            Variable { what, by } => format!("X {what}, X = {by}"),
            Choose(t) => format!("choose: {t}"),
            Other(s) => format!("NOT READ: \"{s}\""),
        }
    }

    /// Does the simulator execute this effect? Typed effects it only names are
    /// reported as caveats (`PowerCard::not_simulated`), unread ones block judgment.
    pub fn simulated(&self) -> bool {
        use Effect::*;
        match self {
            Other(_) | Unmodeled(_) | Repeat | PlayAnother | MakeFast(_) | Reclaim(_) | RegainForgotten | EnergyPer { .. } | InvaderRule(_) | FearDeckRemove { .. } | GainPowerCard { .. } | GainElement(_) | RangeBonus(_) | MayBeFast { .. } | Both | GainEnergy(_) | TokenChoice { .. } | ChooseLand(_) => false,
            SpiritEffect(_) | SpecialToken { .. } | Timing { .. } | Movement(_) | DeckEffect(_) | Variable { .. } | Choose(_) => false,
            Clarification(_) | Distribution(_) | MaySlow => true,
            Per { per: PieceFilter::Other(s), then } => {
                let l = s.to_ascii_lowercase();
                (l.starts_with("elements:") || l.contains("adjacent") || l.contains("past the first") || l.contains('{')) && then.iter().all(Effect::simulated)
            }
            Conditional { condition, then } => !condition.to_ascii_lowercase().starts_with("after") && then.iter().all(Effect::simulated),
            Trigger { when, then, .. } => {
                let w = when.to_ascii_lowercase();
                (w.contains("invader is destroyed") || w.contains("uses a power targeting a land") || w.contains("{blight} is added")) && then.iter().all(Effect::simulated)
            }
            Area { then, .. } | Per { then, .. } => then.iter().all(Effect::simulated),
            Choice(b) => b.iter().all(|x| x.iter().all(Effect::simulated)),
            _ => true,
        }
    }
}

fn token_word(k: TokenKind) -> &'static str {
    match k {
        TokenKind::Beast => "Beasts",
        TokenKind::Wilds => "Wilds",
        TokenKind::Disease => "Disease",
        TokenKind::Strife => "Strife",
        TokenKind::Badlands => "Badlands",
        TokenKind::Vitality => "Vitality",
        TokenKind::Isolate => "Isolate",
    }
}

impl DamageTarget {
    pub fn words(self) -> &'static str {
        match self {
            DamageTarget::Invaders => "to Invaders",
            DamageTarget::Dahan => "to Dahan",
            DamageTarget::EachDahan => "to each Dahan",
            DamageTarget::EachInvader => "to each Invader",
            DamageTarget::EachTownCity => "to each Town and City",
            DamageTarget::TownCityOnly => "to a Town or City",
            DamageTarget::ExplorerTownOnly => "to an Explorer or Town",
            DamageTarget::Town => "to a Town",
        }
    }
}

impl PieceFilter {
    pub fn words(&self) -> String {
        match self {
            PieceFilter::Piece(PieceKind::Explorer) => "Explorer".into(),
            PieceFilter::Piece(PieceKind::Town) => "Town".into(),
            PieceFilter::Piece(PieceKind::City) => "City".into(),
            PieceFilter::Piece(PieceKind::Dahan) => "Dahan".into(),
            PieceFilter::Token(k) => token_word(*k).into(),
            PieceFilter::Presence => "presence".into(),
            PieceFilter::Blight => "Blight".into(),
            PieceFilter::AnyInvader => "Invader".into(),
            PieceFilter::Other(s) => format!("\"{s}\""),
        }
    }
}

impl Range {
    pub fn words(&self) -> String {
        let mut w = format!("Range {}", self.distance);
        if self.from_sacred_site {
            w.push_str(" from a Sacred Site");
        }
        if !self.origin_terrains.is_empty() {
            w.push_str(&format!(" from {}", self.origin_terrains.iter().map(|t| t.name()).collect::<Vec<_>>().join("/")));
        }
        w
    }
}

impl TargetKind {
    /// The engine's target name in words: `CoastalLandWithCity` → "a Coastal land with a City".
    pub fn words(&self) -> String {
        let mut out = String::new();
        for (i, ch) in self.0.chars().enumerate() {
            if ch.is_ascii_uppercase() && i > 0 {
                out.push(' ');
            }
            out.push(ch);
        }
        let lower = out.to_lowercase();
        let lower = lower.replace("any land", "any land").replace(" with ", " with a ").replace("land with a dahan", "land with Dahan").replace("land with a blight", "land with Blight").replace("land with a disease", "land with Disease").replace("land with a wilds", "land with Wilds").replace("land with a beast", "land with Beasts").replace("land with a presence", "land with your presence").replace("land with a invaders", "land with Invaders").replace("land with a town city", "land with a Town or City").replace("land with a city", "land with a City").replace("land with a town", "land with a Town").replace("land with a explorer", "land with an Explorer");
        if self.is_spirit() {
            match self.0.as_str() {
                "You" => "yourself".into(),
                "AnotherSpirit" => "another Spirit".into(),
                _ => "any Spirit".into(),
            }
        } else {
            format!("a {lower}").replace("a any land", "any land")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageTarget {
    Invaders,
    Dahan,
    EachDahan,
    EachInvader,
    EachTownCity,
    TownCityOnly,
    ExplorerTownOnly,
    Town,
}

impl DamageTarget {
    pub fn hits_invaders(self) -> bool {
        !matches!(self, DamageTarget::Dahan | DamageTarget::EachDahan)
    }

    /// Damage that can reach an Explorer (so it can stop a Build).
    pub fn hits_explorers(self) -> bool {
        matches!(self, DamageTarget::Invaders | DamageTarget::EachInvader | DamageTarget::ExplorerTownOnly)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceFilter {
    Piece(PieceKind),
    Token(TokenKind),
    Presence,
    Blight,
    AnyInvader,
    /// A piece word the parser knows is a piece but has no kind for (Incarna, "Invader/Dahan/…").
    Other(String),
}

impl PieceFilter {
    /// `{Explorer}`, `{Town}`, `Invaders`, `{Beast}` … to a filter.
    pub fn parse(word: &str) -> PieceFilter {
        let w = word.trim().trim_matches(|c| c == '{' || c == '}');
        if let Some(p) = PieceKind::parse(w) {
            return PieceFilter::Piece(p);
        }
        if let Some(t) = TokenKind::parse(w) {
            return PieceFilter::Token(t);
        }
        match w {
            "Presence" | "SacredSite" => PieceFilter::Presence,
            "Blight" => PieceFilter::Blight,
            "Invader" | "Invaders" => PieceFilter::AnyInvader,
            _ => PieceFilter::Other(w.to_string()),
        }
    }

    pub fn is_invader(&self) -> bool {
        matches!(self, PieceFilter::AnyInvader | PieceFilter::Piece(PieceKind::Explorer | PieceKind::Town | PieceKind::City))
    }

    pub fn is_explorer(&self) -> bool {
        matches!(self, PieceFilter::AnyInvader | PieceFilter::Piece(PieceKind::Explorer))
    }

    pub fn is_building(&self) -> bool {
        matches!(self, PieceFilter::AnyInvader | PieceFilter::Piece(PieceKind::Town | PieceKind::City))
    }

    pub fn is_dahan(&self) -> bool {
        matches!(self, PieceFilter::Piece(PieceKind::Dahan))
    }
}

/// The Invader step a Spirit effect can act against, on Rei's land clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Slot {
    /// Stop or blunt the Explore (Wilds, Isolate, a skip).
    Explore,
    /// Stop the Build: no Explorer left, Disease, a skip.
    Build,
    /// Stop the Blight at the Ravage: Defend, kill or move the Invaders,
    /// Dahan to counterattack, Strife, a skip.
    Ravage,
}

impl Effect {
    /// Could this effect, played in time, act on the given Invader step?
    /// A capability, not a guarantee: "Defend 1" acts at the Ravage even when
    /// 1 is not enough.
    pub fn acts_on(&self, slot: Slot) -> bool {
        use Effect::*;
        match self {
            Choice(branches) => branches.iter().any(|b| b.iter().any(|e| e.acts_on(slot))),
            Conditional { then, .. } | Per { then, .. } | Area { then, .. } | Trigger { then, .. } => then.iter().any(|e| e.acts_on(slot)),
            SkipAll | SkipAction(_) => true,
            NoCascade | DahanHealth(_) | DahanSimultaneous | NoBlight | Infighting(_) | Duel { .. } | RemoveHealth(_) => slot == Slot::Ravage,
            Build => slot == Slot::Build,
            InvaderRule(r) => slot == Slot::Ravage && r.to_ascii_lowercase().contains("ravage"),
            _ => match slot {
                Slot::Ravage => match self {
                    Defend(_) | DamageBonus(_) | SkipRavage | NoParticipation(_) | AddDahan(_) | Downgrade(_) | InvaderDamageMinus(_) | Ravage => true,
                    Spread { kind: TokenKind::Strife, .. } => true,
                    Damage { to, .. } => to.hits_invaders(),
                    Push { what, .. } | Destroy { what, .. } | Remove { what, .. } | Move { what, .. } => what.iter().any(PieceFilter::is_invader),
                    Gather { what, .. } => what.iter().any(PieceFilter::is_dahan),
                    AddToken { kind: TokenKind::Strife, .. } => true,
                    Replace { from, .. } => from.is_building(),
                    _ => false,
                },
                Slot::Build => match self {
                    SkipBuild | Downgrade(_) => true,
                    Spread { kind: TokenKind::Disease, .. } => true,
                    Damage { to, .. } => to.hits_explorers(),
                    Push { what, .. } | Destroy { what, .. } | Remove { what, .. } | Move { what, .. } => what.iter().any(PieceFilter::is_explorer),
                    AddToken { kind: TokenKind::Disease, .. } => true,
                    Replace { from, .. } => from.is_explorer(),
                    _ => false,
                },
                Slot::Explore => matches!(self, SkipExplore | Isolate | AddToken { kind: TokenKind::Wilds, .. } | Spread { kind: TokenKind::Wilds, .. }),
            },
        }
    }

    pub fn is_other(&self) -> bool {
        matches!(self, Effect::Other(_))
    }

    /// Every `Other` sentence under this effect.
    pub fn others<'a>(&'a self, out: &mut Vec<&'a str>) {
        match self {
            Effect::Other(s) => out.push(s),
            Effect::Choice(b) => b.iter().flatten().for_each(|e| e.others(out)),
            Effect::Conditional { then, .. } | Effect::Per { then, .. } | Effect::Area { then, .. } | Effect::Trigger { then, .. } => then.iter().for_each(|e| e.others(out)),
            _ => {}
        }
    }
}

/// Counts of what a card does, for the pricing fit (Section 4 of the tempo
/// spec). A `Choice` contributes the per-field maximum over its branches.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Features {
    pub damage: u32,
    /// Damage dealt to each Invader / each Town and City (area damage).
    pub damage_each: u32,
    pub defend: u32,
    pub push_invaders: u32,
    pub push_dahan: u32,
    pub gather_dahan: u32,
    pub gather_invaders: u32,
    pub fear: u32,
    pub fear_conditional: u32,
    pub blight_removed: u32,
    pub blight_added: u32,
    pub tokens_added: u32,
    pub destroy: u32,
    pub skip: u32,
    pub energy: u32,
    pub cards: u32,
    pub conditional: u32,
    pub per: u32,
    pub choice: u32,
    pub other: u32,
}

impl Features {
    pub fn of(effects: &[Effect]) -> Features {
        let mut f = Features::default();
        for e in effects {
            f.add(e);
        }
        f
    }

    fn add(&mut self, e: &Effect) {
        use Effect::*;
        let n = |c: u8| if c == 255 { 3 } else { c as u32 };
        match e {
            Damage { amount, to } => {
                if matches!(to, DamageTarget::EachInvader | DamageTarget::EachTownCity | DamageTarget::EachDahan) {
                    self.damage_each += *amount as u32
                } else {
                    self.damage += *amount as u32
                }
            }
            DamageBonus(a) => self.damage += *a as u32,
            Defend(d) => self.defend += *d as u32,
            Spread { count, .. } => self.tokens_added += n(*count),
            Ravage | FearDeckRemove { .. } => {}
            Push { count, what } | Move { count, what } => {
                if what.iter().any(PieceFilter::is_invader) {
                    self.push_invaders += n(*count)
                }
                if what.iter().any(PieceFilter::is_dahan) {
                    self.push_dahan += n(*count)
                }
            }
            Gather { count, what } => {
                if what.iter().any(PieceFilter::is_dahan) {
                    self.gather_dahan += n(*count)
                }
                if what.iter().any(PieceFilter::is_invader) {
                    self.gather_invaders += n(*count)
                }
            }
            Fear(f) => self.fear += *f as u32,
            FearIfInvaders(f) => self.fear_conditional += *f as u32,
            RemoveBlight(b) => self.blight_removed += *b as u32,
            AddBlight(b) => self.blight_added += *b as u32,
            AddDahan(_) | AddPresence { .. } | AddToken { .. } => self.tokens_added += 1,
            RemoveToken { .. } | Remove { .. } | Replace { .. } | Downgrade(_) => self.destroy += 1,
            Destroy { count, .. } => self.destroy += n(*count),
            Isolate | SkipRavage | SkipBuild | SkipExplore | SkipAll | SkipAction(_) | NoParticipation(_) | DahanImmune | InvaderDamageMinus(_) => self.skip += 1,
            GainEnergy(e) => self.energy += *e as u32,
            GainPowerCard { .. } | GainElement(_) => self.cards += 1,
            RangeBonus(_) | Repeat | MayBeFast { .. } | Both | MakeFast(_) | Unmodeled(_) | SpiritEffect(_) | SpecialToken { .. } | Timing { .. } | Movement(_) | DeckEffect(_) | Variable { .. } | Choose(_) | MaySlow => self.other += 1,
            Instead | Requires { .. } => {}
            Clarification(_) | Distribution(_) | Build => {}
            DestroyPresence(_) | NoBlight => self.skip += 1,
            RemoveHealth(k) => self.destroy += *k as u32,
            Infighting(_) | Duel { .. } => self.damage += 1,
            AddInvaders { .. } => self.tokens_added += 1,
            Area { then, .. } => then.iter().for_each(|e| self.add(e)),
            Trigger { then, .. } => {
                self.conditional += 1;
                then.iter().for_each(|e| self.add(e));
            }
            NoCascade | DahanHealth(_) | DahanSimultaneous | InvaderRule(_) => self.skip += 1,
            ChooseLand(_) | TokenChoice { .. } => {}
            AddChosenToken(k) => self.tokens_added += *k as u32,
            PlayAnother | RegainForgotten => self.cards += 1,
            Reclaim(k) => self.cards += *k as u32,
            EnergyPer { n, .. } => self.energy += *n as u32,
            Per { then, .. } => {
                self.per += 1;
                then.iter().for_each(|e| self.add(e));
            }
            Conditional { then, .. } => {
                self.conditional += 1;
                then.iter().for_each(|e| self.add(e));
            }
            Choice(branches) => {
                self.choice += 1;
                let best = branches.iter().map(|b| Features::of(b)).fold(Features::default(), Features::max);
                *self = self.plus(&best);
            }
            Other(_) => self.other += 1,
        }
    }

    fn max(self, o: Features) -> Features {
        Features {
            damage: self.damage.max(o.damage),
            damage_each: self.damage_each.max(o.damage_each),
            defend: self.defend.max(o.defend),
            push_invaders: self.push_invaders.max(o.push_invaders),
            push_dahan: self.push_dahan.max(o.push_dahan),
            gather_dahan: self.gather_dahan.max(o.gather_dahan),
            gather_invaders: self.gather_invaders.max(o.gather_invaders),
            fear: self.fear.max(o.fear),
            fear_conditional: self.fear_conditional.max(o.fear_conditional),
            blight_removed: self.blight_removed.max(o.blight_removed),
            blight_added: self.blight_added.max(o.blight_added),
            tokens_added: self.tokens_added.max(o.tokens_added),
            destroy: self.destroy.max(o.destroy),
            skip: self.skip.max(o.skip),
            energy: self.energy.max(o.energy),
            cards: self.cards.max(o.cards),
            conditional: self.conditional.max(o.conditional),
            per: self.per.max(o.per),
            choice: self.choice.max(o.choice),
            other: self.other.max(o.other),
        }
    }

    fn plus(&self, o: &Features) -> Features {
        Features {
            damage: self.damage + o.damage,
            damage_each: self.damage_each + o.damage_each,
            defend: self.defend + o.defend,
            push_invaders: self.push_invaders + o.push_invaders,
            push_dahan: self.push_dahan + o.push_dahan,
            gather_dahan: self.gather_dahan + o.gather_dahan,
            gather_invaders: self.gather_invaders + o.gather_invaders,
            fear: self.fear + o.fear,
            fear_conditional: self.fear_conditional + o.fear_conditional,
            blight_removed: self.blight_removed + o.blight_removed,
            blight_added: self.blight_added + o.blight_added,
            tokens_added: self.tokens_added + o.tokens_added,
            destroy: self.destroy + o.destroy,
            skip: self.skip + o.skip,
            energy: self.energy + o.energy,
            cards: self.cards + o.cards,
            conditional: self.conditional + o.conditional,
            per: self.per + o.per,
            choice: self.choice + o.choice,
            other: self.other + o.other,
        }
    }

    /// The feature vector in a fixed order, for a regression.
    pub fn vector(&self) -> Vec<f64> {
        [self.damage, self.damage_each, self.defend, self.push_invaders, self.push_dahan, self.gather_dahan, self.gather_invaders, self.fear, self.fear_conditional, self.blight_removed, self.blight_added, self.tokens_added, self.destroy, self.skip, self.energy, self.cards, self.conditional, self.per, self.choice]
            .iter()
            .map(|v| *v as f64)
            .collect()
    }

    /// The non-zero features in words, for the price fit: "3 Damage, 1 Fear, destroys 1".
    pub fn words(&self) -> String {
        let mut p: Vec<String> = Vec::new();
        let mut add = |n: u32, one: &str, many: &str| {
            if n > 0 {
                p.push(if n == 1 { one.to_string() } else { many.replace("N", &n.to_string()) });
            }
        };
        add(self.damage, "1 Damage", "N Damage");
        add(self.damage_each, "1 Damage to each", "N Damage to each");
        add(self.defend, "Defend 1", "Defend N");
        add(self.push_invaders, "pushes 1 Invader", "pushes N Invaders");
        add(self.push_dahan, "pushes 1 Dahan", "pushes N Dahan");
        add(self.gather_dahan, "gathers 1 Dahan", "gathers N Dahan");
        add(self.gather_invaders, "gathers 1 Invader", "gathers N Invaders");
        add(self.fear, "1 Fear", "N Fear");
        add(self.fear_conditional, "1 conditional Fear", "N conditional Fear");
        add(self.blight_removed, "removes 1 Blight", "removes N Blight");
        add(self.blight_added, "adds 1 Blight", "adds N Blight");
        add(self.tokens_added, "adds 1 token", "adds N tokens");
        add(self.destroy, "destroys or removes 1", "destroys or removes N");
        add(self.skip, "1 skip", "N skips");
        add(self.energy, "1 Energy", "N Energy");
        add(self.cards, "1 card", "N cards");
        add(self.conditional, "1 condition", "N conditions");
        add(self.per, "1 per-piece effect", "N per-piece effects");
        add(self.choice, "1 choice", "N choices");
        add(self.other, "1 sentence not read", "N sentences not read");
        if p.is_empty() { "nothing counted".into() } else { p.join(", ") }
    }

    pub const NAMES: [&'static str; 19] = ["damage", "damage_each", "defend", "push_invaders", "push_dahan", "gather_dahan", "gather_invaders", "fear", "fear_conditional", "blight_removed", "blight_added", "tokens_added", "destroy", "skip", "energy", "cards", "conditional", "per", "choice"];
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Threshold {
    pub elements: ElementSet,
    pub title: String,
    pub body: Vec<String>,
    #[serde(default)]
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerCard {
    pub id: CardId,
    pub title: String,
    pub kind: PowerKind,
    pub cost: i64,
    pub speed: Speed,
    /// None for powers that target a Spirit rather than a land.
    pub range: Option<Range>,
    pub target: TargetKind,
    pub elements: ElementSet,
    pub body: Vec<String>,
    /// Element thresholds (elements set) and text conditions (elements empty).
    #[serde(default)]
    pub thresholds: Vec<Threshold>,
    #[serde(default)]
    pub effects: Vec<Effect>,
    pub expansion: String,
}

impl PowerCard {
    /// Element thresholds only.
    pub fn element_thresholds(&self) -> impl Iterator<Item = &Threshold> {
        self.thresholds.iter().filter(|t| !t.elements.is_empty())
    }

    pub fn targets_land(&self) -> bool {
        !self.target.is_spirit()
    }

    /// True when every effect is typed (no `Other`), thresholds included.
    pub fn fully_parsed(&self) -> bool {
        !self.effects.is_empty() && self.unparsed().is_empty()
    }

    /// Typed effects the simulator does not execute yet, in words (base text and thresholds).
    pub fn not_simulated(&self) -> Vec<String> {
        self.all_effects().filter(|e| !e.is_other() && !e.simulated()).map(Effect::words).collect()
    }

    /// Every sentence the parser left as `Other`, base text and thresholds.
    pub fn unparsed(&self) -> Vec<&str> {
        let mut out = Vec::new();
        for e in self.effects.iter().chain(self.thresholds.iter().flat_map(|t| t.effects.iter())) {
            e.others(&mut out);
        }
        out
    }

    /// Base effects plus every threshold's (what the card could do with the
    /// elements in hand).
    pub fn all_effects(&self) -> impl Iterator<Item = &Effect> {
        self.effects.iter().chain(self.thresholds.iter().flat_map(|t| t.effects.iter()))
    }

    /// Could the card act on this Invader step, with its thresholds counted?
    pub fn acts_on(&self, slot: Slot) -> bool {
        self.all_effects().any(|e| e.acts_on(slot))
    }

    pub fn features(&self) -> Features {
        Features::of(&self.effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_kinds_answer_typed_questions() {
        assert!(TargetKind("LandWithDahan".into()).requires_dahan());
        assert!(TargetKind("AnotherSpirit".into()).is_spirit());
        assert_eq!(TargetKind("JungleWetland".into()).terrains(), Some(vec![Terrain::Jungle, Terrain::Wetland]));
        assert_eq!(TargetKind("AnyLand".into()).terrains(), None);
        assert_eq!(TargetKind("NonWetland".into()).terrains().map(|t| t.len()), Some(3));
    }
}
