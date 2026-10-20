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
    /// Presence added by a Power (boons, Blazing Renewal).
    AddPresence(u8),
    AddToken { kind: TokenKind, count: u8 },
    RemoveToken { kind: TokenKind, count: u8 },
    /// Remove pieces without Destroying them (no Fear, no counterattack).
    /// `count` 255 is "all", 128 is "half".
    Remove { count: u8, what: Vec<PieceFilter> },
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
    /// "You may do both": the preceding Choice is not exclusive.
    Both,
    /// "For each {X}, …" / "N Damage per {X}".
    Per { per: PieceFilter, then: Vec<Effect> },
    Choice(Vec<Vec<Effect>>),
    Conditional { condition: String, then: Vec<Effect> },
    Other(String),
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
            Conditional { then, .. } | Per { then, .. } => then.iter().any(|e| e.acts_on(slot)),
            SkipAll | SkipAction(_) => true,
            _ => match slot {
                Slot::Ravage => match self {
                    Defend(_) | DamageBonus(_) | SkipRavage | NoParticipation(_) | AddDahan(_) | Downgrade(_) | InvaderDamageMinus(_) => true,
                    Damage { to, .. } => to.hits_invaders(),
                    Push { what, .. } | Destroy { what, .. } | Remove { what, .. } | Move { what, .. } => what.iter().any(PieceFilter::is_invader),
                    Gather { what, .. } => what.iter().any(PieceFilter::is_dahan),
                    AddToken { kind: TokenKind::Strife, .. } => true,
                    Replace { from, .. } => from.is_building(),
                    _ => false,
                },
                Slot::Build => match self {
                    SkipBuild | Downgrade(_) => true,
                    Damage { to, .. } => to.hits_explorers(),
                    Push { what, .. } | Destroy { what, .. } | Remove { what, .. } | Move { what, .. } => what.iter().any(PieceFilter::is_explorer),
                    AddToken { kind: TokenKind::Disease, .. } => true,
                    Replace { from, .. } => from.is_explorer(),
                    _ => false,
                },
                Slot::Explore => matches!(self, SkipExplore | Isolate | AddToken { kind: TokenKind::Wilds, .. }),
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
            Effect::Conditional { then, .. } | Effect::Per { then, .. } => then.iter().for_each(|e| e.others(out)),
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
            AddDahan(_) | AddPresence(_) | AddToken { .. } => self.tokens_added += 1,
            RemoveToken { .. } | Remove { .. } | Replace { .. } | Downgrade(_) => self.destroy += 1,
            Destroy { count, .. } => self.destroy += n(*count),
            Isolate | SkipRavage | SkipBuild | SkipExplore | SkipAll | SkipAction(_) | NoParticipation(_) | DahanImmune | InvaderDamageMinus(_) => self.skip += 1,
            GainEnergy(e) => self.energy += *e as u32,
            GainPowerCard { .. } | GainElement(_) => self.cards += 1,
            RangeBonus(_) | Repeat | MayBeFast { .. } | Both => self.other += 1,
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
