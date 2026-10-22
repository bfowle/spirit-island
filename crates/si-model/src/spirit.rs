//! Spirit panels (printed) and spirit state (in play): presence tracks and
//! what each revealed slot unlocks, growth options and what they yield,
//! innates and their element tiers, special rules with their typed hooks,
//! and the hand / play area / discard / forgotten piles.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::element::{Element, ElementSet};
use crate::island::{LandId, Terrain};
use crate::power::{Range, Speed, TargetKind};
use crate::{CardId, SpiritId};

/// Which printed track a slot belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Track {
    Energy,
    Plays,
    Other,
}

/// One presence slot. Flat facets rather than one enum per engine kind: the
/// engine has 22 node kinds, most of them combinations (energy and element,
/// plays and Reclaim One, …). `kind` keeps the engine's name for audit.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackNode {
    pub id: String,
    pub kind: String,
    pub track: Track,
    /// Sets Energy per turn to this value (the usual energy slot).
    pub energy: Option<u8>,
    /// Sets Card Plays per turn to this value.
    pub plays: Option<u8>,
    /// Adds to the running value instead of setting it (`Plus…` kinds).
    #[serde(default)]
    pub plus: bool,
    /// Element(s) this slot gives every turn while revealed.
    #[serde(default)]
    pub elements: Vec<Element>,
    /// A permanent element marker (counts as an element you "have").
    #[serde(default)]
    pub permanent_element: bool,
    /// Reclaim One: during Growth, take one card back from the discard.
    #[serde(default)]
    pub reclaim_one: bool,
    /// A full-track Reclaim slot with a count (rare).
    pub reclaim: Option<u8>,
    /// Pay Energy to gain a Power Card (Many Minds: pay 2).
    pub pay_to_gain: Option<u8>,
    /// An extra growth option granted by the slot.
    pub growth_option: Option<GrowthOptionDef>,
    /// Fractured Days' impending-energy slots.
    #[serde(default)]
    pub impending: bool,
    /// Slots that must be revealed before this one.
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub next: Vec<String>,
}

impl Default for Track {
    fn default() -> Self {
        Track::Other
    }
}

/// What a growth option is, as the engine names it, with the fields that
/// decide its yield. `kind` is the engine's `growthType`; the spirit-unique
/// kinds (Volcano's "add presence to a Mountain or your presence") keep their
/// name and are decoded by `placement`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrowthOptionDef {
    pub kind: String,
    pub basic: Option<String>,
    pub number: Option<i64>,
    #[serde(default)]
    pub elements: Vec<Element>,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub repeats: Option<u8>,
    /// For `Choice`: the options the player picks one of.
    #[serde(default)]
    pub choices: Vec<GrowthOptionDef>,
}

/// Where a growth may put presence. Decoded from the option kind, sourced to
/// the printed panel; `Any` is the base rule (range from existing presence).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Placement {
    Any { range: u8 },
    Terrains { range: u8, terrains: Vec<Terrain> },
    WithDahan { range: u8 },
    WithInvaders { range: u8 },
    WithTownCityOrBlight { range: u8 },
    NoBlight { range: u8 },
    Inland { range: u8 },
    Coastal { range: u8 },
    Ocean,
    Other { range: u8, kind: String },
}

/// What one growth option gives, in the units the tempo report counts.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Yield {
    pub energy: i64,
    pub cards: u8,
    pub plays_this_turn: u8,
    pub presence: u8,
    pub placement: Option<Placement>,
    pub reclaim: Option<Reclaim>,
    #[serde(default)]
    pub elements: Vec<Element>,
    #[serde(default)]
    pub moves_presence: u8,
    #[serde(default)]
    pub other: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reclaim {
    All,
    Half,
    One,
}

impl GrowthOptionDef {
    pub fn yield_(&self) -> Yield {
        let n = self.number.unwrap_or(0);
        let range = n.clamp(0, 9) as u8;
        let mut y = Yield::default();
        match self.kind.as_str() {
            // a choice yields what its most presence-rich option yields
            "Choice" if !self.choices.is_empty() => {
                return self.choices.iter().map(|c| c.yield_()).max_by_key(|y| (y.presence, y.cards, y.energy)).unwrap_or_default();
            }
            "ReclaimCards" => y.reclaim = Some(Reclaim::All),
            "ReclaimHalf" => y.reclaim = Some(Reclaim::Half),
            "GainPowerCard" | "GainPowerCardFromDays" => y.cards = n.max(1) as u8,
            "GainEnergy" => y.energy = n,
            "CardPlayThisTurn" => y.plays_this_turn = n.max(1) as u8,
            "GainElements" => y.elements = self.elements.clone(),
            "MovePresence" => y.moves_presence = n.max(1) as u8,
            "AddPresence" => {
                y.presence = 1;
                y.placement = Some(Placement::Any { range });
            }
            k if k.starts_with("AddPresence") => {
                y.presence = 1;
                y.placement = Some(match k {
                    "AddPresenceDahan" => Placement::WithDahan { range },
                    "AddPresenceDahanInvaders" => Placement::WithInvaders { range },
                    "AddPresenceTownCityBlight" => Placement::WithTownCityOrBlight { range },
                    "AddPresenceNoBlight" => Placement::NoBlight { range },
                    "AddPresenceInland" => Placement::Inland { range },
                    "AddPresenceCoastal" => Placement::Coastal { range },
                    "AddPresenceOcean" => Placement::Ocean,
                    "AddPresenceMountainSands" => Placement::Terrains { range, terrains: vec![Terrain::Mountain, Terrain::Sands] },
                    "AddPresenceMountainWetland" => Placement::Terrains { range, terrains: vec![Terrain::Mountain, Terrain::Wetland] },
                    "AddPresenceJungleWetland" => Placement::Terrains { range, terrains: vec![Terrain::Jungle, Terrain::Wetland] },
                    other => Placement::Other { range, kind: other.to_string() },
                });
            }
            other => y.other.push(other.to_string()),
        }
        y
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrowthGroup {
    pub number: u8,
    pub options: Vec<GrowthOptionDef>,
}

impl GrowthGroup {
    pub fn yield_(&self) -> Yield {
        let mut total = Yield::default();
        for o in &self.options {
            let y = o.yield_();
            total.energy += y.energy;
            total.cards += y.cards;
            total.plays_this_turn += y.plays_this_turn;
            total.presence += y.presence;
            if total.placement.is_none() {
                total.placement = y.placement;
            }
            if y.reclaim.is_some() {
                total.reclaim = y.reclaim;
            }
            total.elements.extend(y.elements);
            total.moves_presence += y.moves_presence;
            total.other.extend(y.other);
        }
        total
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrowthRules {
    pub groups: Vec<GrowthGroup>,
    /// How many groups are chosen each turn (1 for most spirits, 2 for some).
    pub pick: u8,
    /// A group every growth takes before the chosen ones (Rampant Green's "Add a
    /// Presence to Jungle or Wetland"); the engine's `alwaysGrowthGroup`.
    #[serde(default)]
    pub always: Option<GrowthGroup>,
}

/// One element tier of an innate.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tier {
    pub elements: ElementSet,
    pub body: String,
    #[serde(default)]
    pub effects: Vec<crate::power::Effect>,
    /// A tier that changes another tier (e.g. "This Power may be Fast")
    /// rather than acting on its own.
    #[serde(default)]
    pub modifier: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Innate {
    pub id: String,
    pub title: String,
    pub speed: Speed,
    pub range: Option<Range>,
    pub target: TargetKind,
    pub tiers: Vec<Tier>,
    /// Sentences printed on the innate above its tiers ("This Power's Damage is done (separately) to both Invaders and Dahan").
    #[serde(default)]
    pub rider: Vec<String>,
    /// What the innate does on every use, before its tiers: its rider read as effects
    /// (Explosive Eruption's "Destroy X (1 or more) of your {Presence} in target land").
    #[serde(default)]
    pub effects: Vec<crate::power::Effect>,
}

impl Innate {
    /// Every tier `have` reaches. The engine lists an innate's tiers as
    /// independent element options, each active when its elements are met;
    /// modifier tiers only matter when another tier is active too.
    pub fn active<'a>(&'a self, have: &ElementSet) -> Vec<&'a Tier> {
        let met: Vec<&Tier> = self.tiers.iter().filter(|t| have.satisfies(&t.elements)).collect();
        if met.iter().all(|t| t.modifier) {
            return vec![];
        }
        met
    }

    /// How many acting (non-modifier) tiers `have` reaches, and the last one.
    pub fn reached(&self, have: &ElementSet) -> (usize, Option<&Tier>) {
        let acting: Vec<&Tier> = self.active(have).into_iter().filter(|t| !t.modifier).collect();
        (acting.len(), acting.last().copied())
    }

    /// Elements still missing for the next unreached acting tier, if any.
    pub fn next_shortfall(&self, have: &ElementSet) -> Option<(&Tier, u32)> {
        self.tiers.iter().filter(|t| !t.modifier && !have.satisfies(&t.elements)).map(|t| (t, have.shortfall(&t.elements))).min_by_key(|(_, n)| *n)
    }
}

/// A typed consequence of a special rule, assigned by rule identifier in
/// `si-defs` (never inferred from text). Each one names the force it bends.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleHook {
    /// Presence may only be added or moved into these terrains (Volcano).
    PlacementOnly(Vec<Terrain>),
    /// Presence may move along with Dahan leaving the land (Thunderspeaker).
    PresenceMovesWithDahan,
    /// After Invaders destroy Dahan in a Ravage, destroy one presence within
    /// `range` per Dahan destroyed (Thunderspeaker).
    PresenceDestroyedPerDahanKilled { range: u8 },
    /// Powers gain `bonus` Range when the origin land holds at least
    /// `presence` of yours (Volcano).
    RangeBonusFromOrigin { presence: u8, bonus: u8 },
    /// Destroyed presence deals `per_presence` damage to Invaders and Dahan in
    /// that land (Volcano).
    DamageOnPresenceDestroyed { per_presence: u8 },
    /// Blight does not cascade or destroy presence unless it outnumbers your
    /// presence there (Stone).
    BlightDoesNotCascadeUnlessOutnumbered,
    /// Disease where your presence is destroyed by a Ravage or non-Spirit
    /// effect (Vengeance).
    DiseaseOnPresenceDestroyed,
    /// Disease lets a Build happen for Fear instead of being blocked
    /// (Vengeance).
    DiseaseBuildForFear { fear: u8 },
    /// The spirit's actions treat Blight on the island as Badlands too
    /// (Vengeance: Wreak Vengeance for the Land's Corruption).
    BlightCountsAsBadlands,
    /// Targeting a land with a Power, the spirit may Gather 1 of its presence into the
    /// target or an adjacent land first (Shroud: Mists Shift and Flow).
    GatherPresenceOnTargeting,
    /// Whenever Invaders would Ravage or Build in a land with the spirit's Sacred Site, the
    /// spirit may skip it by destroying one of its presence there (Rampant Green: Choke the
    /// Land with Green). Which skips are taken is the player's call.
    SkipStepByDestroyingPresence,
    /// For every one of this element the spirit has, 1 of its Slow Powers may be used as if
    /// Fast (Lightning: Swiftness of Lightning, per Air).
    SlowAsFastPer(Element),
    /// When the spirit Gathers or Pushes Beasts, they may come from or go to lands up to 2
    /// distant (Many Minds: Fly Fast as Thought).
    BeastsMoveTwoLands,
    /// The spirit's Sacred Site may also count as a Beast; changing that Beast changes 2 of
    /// its presence there (Many Minds: A Joining of Swarms and Flocks).
    SacredSiteIsBeast,
    /// Not yet typed; the text is still carried on the rule.
    Untyped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialRule {
    pub id: String,
    pub title: String,
    pub body: Vec<String>,
    #[serde(default)]
    pub requires_activation: bool,
    #[serde(default)]
    pub hooks: Vec<RuleHook>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Aspect {
    pub id: String,
    #[serde(default)]
    pub replaces: Vec<String>,
    #[serde(default)]
    pub special_rules: Vec<SpecialRule>,
    #[serde(default)]
    pub expansion: String,
}

/// What a spirit's own rules say about the ledger: how much a Dahan or a
/// presence lost weighs for it, relative to the generic weight (1.0). Derived
/// from the panel's typed hooks and innates; each factor carries its reason.
/// Consumers (hindsight, the companion) read this instead of naming spirits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Priorities {
    pub dahan: f64,
    pub presence: f64,
    pub reasons: Vec<String>,
}

impl Default for Priorities {
    fn default() -> Self {
        Priorities { dahan: 1.0, presence: 1.0, reasons: vec![] }
    }
}

/// A spirit as printed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpiritPanel {
    pub id: SpiritId,
    pub title: String,
    /// The engine's short name (`Volcano`, `Many Minds`), used in its log lines.
    #[serde(default)]
    pub short_title: String,
    pub expansion: String,
    pub complexity: String,
    pub setup: String,
    pub tracks: Vec<TrackNode>,
    pub growth: GrowthRules,
    pub innates: Vec<Innate>,
    pub special_rules: Vec<SpecialRule>,
    pub uniques: Vec<CardId>,
    #[serde(default)]
    pub aspects: Vec<Aspect>,
}

/// Energy income, card plays and elements for a set of revealed slots.
impl SpiritPanel {
    /// Every typed hook on the panel's special rules.
    pub fn hooks(&self) -> impl Iterator<Item = &RuleHook> {
        self.special_rules.iter().flat_map(|r| r.hooks.iter())
    }

    /// The ledger factors the panel's rules imply.
    pub fn priorities(&self) -> Priorities {
        let mut p = Priorities::default();
        if self.hooks().any(|h| matches!(h, RuleHook::DiseaseOnPresenceDestroyed)) {
            p.presence = 0.5;
            p.reasons.push("presence destroyed by a Ravage leaves Disease behind (Lingering Pestilence): half a loss".into());
        }
        if self.hooks().any(|h| matches!(h, RuleHook::PresenceMovesWithDahan | RuleHook::PresenceDestroyedPerDahanKilled { .. })) {
            p.dahan = 1.5;
            p.reasons.push("presence moves with the Dahan and dies with them (Ally of the Dahan, Sworn to Victory): a Dahan lost weighs more".into());
        }
        // an innate whose Damage also hits the Dahan: the spirit does not fight beside them
        if self.innates.iter().any(|i| i.rider.iter().any(|r| r.contains("both Invaders and {Dahan}"))) {
            p.dahan = 0.5;
            p.reasons.push("its innate's Damage hits the Dahan too (Epidemics Run Rampant): Dahan are not its allies, half a loss".into());
        }
        p
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackState {
    pub income: i64,
    pub plays: i64,
    pub elements: ElementSet,
    pub reclaim_one: bool,
    pub pay_to_gain: Option<u8>,
    pub growth_options: Vec<GrowthOptionDef>,
    pub revealed: usize,
}

impl SpiritPanel {
    /// Walk the revealed slots: a slot that *sets* income or plays takes the
    /// highest set value, `Plus` slots add on top. Elements accumulate.
    pub fn track_state(&self, revealed: &BTreeSet<String>) -> TrackState {
        let mut st = TrackState::default();
        let mut energy_set = 0i64;
        let mut plays_set = 0i64;
        let mut energy_plus = 0i64;
        let mut plays_plus = 0i64;
        for n in self.tracks.iter().filter(|n| revealed.contains(&n.id)) {
            st.revealed += 1;
            if let Some(e) = n.energy {
                if n.plus {
                    energy_plus += e as i64
                } else {
                    energy_set = energy_set.max(e as i64)
                }
            }
            if let Some(p) = n.plays {
                if n.plus {
                    plays_plus += p as i64
                } else {
                    plays_set = plays_set.max(p as i64)
                }
            }
            for e in &n.elements {
                st.elements.add(*e, 1);
            }
            st.reclaim_one |= n.reclaim_one;
            if n.pay_to_gain.is_some() {
                st.pay_to_gain = n.pay_to_gain;
            }
            if let Some(g) = &n.growth_option {
                st.growth_options.push(g.clone());
            }
        }
        st.income = energy_set + energy_plus;
        st.plays = plays_set + plays_plus;
        st
    }

    /// The slots a track starts with revealed (the engine marks the rest
    /// `startsCovered`; here: a slot with no requirements).
    pub fn starting_revealed(&self) -> BTreeSet<String> {
        self.tracks.iter().filter(|n| n.requires.is_empty()).map(|n| n.id.clone()).collect()
    }

    pub fn hook(&self, pred: impl Fn(&RuleHook) -> bool) -> Option<&RuleHook> {
        self.special_rules.iter().flat_map(|r| r.hooks.iter()).find(|h| pred(h))
    }
}

/// A track-tempo target in Rei's `x/ycp` notation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spike {
    pub income: i64,
    pub plays: i64,
    pub label: String,
    pub by_turn: Option<u32>,
    pub source: String,
}

impl Spike {
    pub fn new(income: i64, plays: i64, source: &str) -> Self {
        Spike { income, plays, label: format!("{income}/{plays}cp"), by_turn: None, source: source.to_string() }
    }

    pub fn met_by(&self, t: &TrackState) -> bool {
        t.income >= self.income && t.plays >= self.plays
    }
}

/// A spirit in play.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpiritState {
    pub id: SpiritId,
    pub aspect: Option<String>,
    pub revealed: BTreeSet<String>,
    pub energy: i64,
    pub hand: Vec<CardId>,
    pub play_area: Vec<CardId>,
    pub discard: Vec<CardId>,
    pub forgotten: Vec<CardId>,
    /// Elements from cards played this turn (Fast and Slow both count all turn).
    pub played_elements: ElementSet,
    /// Permanent element markers.
    pub markers: ElementSet,
    pub plays_this_turn: i64,
    pub presence_destroyed: u32,
    /// Lands holding this spirit's presence (counts live on the lands).
    #[serde(default)]
    pub lands: BTreeSet<LandId>,
}

impl SpiritState {
    /// Everything that counts toward thresholds this turn.
    pub fn element_bank(&self, panel: &SpiritPanel) -> ElementSet {
        let mut bank = panel.track_state(&self.revealed).elements;
        bank.extend(&self.markers);
        bank.extend(&self.played_elements);
        bank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str, kind: &str, track: Track) -> TrackNode {
        TrackNode { id: id.into(), kind: kind.into(), track, ..Default::default() }
    }

    fn panel() -> SpiritPanel {
        // Thunderspeaker's tracks as the engine defines them.
        let mut t = Vec::new();
        for (i, (e, el)) in [(Some(1), None), (None, Some(Element::Air)), (Some(2), None), (None, Some(Element::Fire)), (None, Some(Element::Sun)), (Some(3), None)].iter().enumerate() {
            let mut n = node(&format!("E{}", i + 1), "EnergyPerTurn", Track::Energy);
            n.energy = *e;
            n.elements = el.iter().copied().collect();
            if i > 0 {
                n.requires = vec![format!("E{i}")];
            }
            t.push(n);
        }
        for (i, p) in [Some(1), Some(2), Some(2), Some(3), None, Some(3), Some(4)].iter().enumerate() {
            let mut n = node(&format!("P{}", i + 1), "CardPlaysPerTurn", Track::Plays);
            n.plays = *p;
            n.reclaim_one = p.is_none();
            if i > 0 {
                n.requires = vec![format!("P{i}")];
            }
            t.push(n);
        }
        SpiritPanel {
            id: "Thunderspeaker".into(),
            title: "Thunderspeaker".into(),
            short_title: "Thunderspeaker".into(),
            expansion: "Core".into(),
            complexity: "Moderate".into(),
            setup: String::new(),
            tracks: t,
            growth: GrowthRules::default(),
            innates: vec![],
            special_rules: vec![],
            uniques: vec![],
            aspects: vec![],
        }
    }

    #[test]
    fn revealed_slots_give_income_plays_elements_and_reclaim_one() {
        let p = panel();
        let start = p.starting_revealed();
        assert_eq!(start.len(), 2);
        let s = p.track_state(&start);
        assert_eq!((s.income, s.plays), (1, 1));
        let mut r = start.clone();
        for id in ["E2", "E3", "P2", "P3", "P4", "P5"] {
            r.insert(id.into());
        }
        let s = p.track_state(&r);
        assert_eq!((s.income, s.plays), (2, 3));
        assert_eq!(s.elements.count(Element::Air), 1);
        assert!(s.reclaim_one, "the fifth plays slot is Reclaim One");
        assert!(Spike::new(2, 3, "test").met_by(&s));
        assert!(!Spike::new(3, 3, "test").met_by(&s));
    }

    #[test]
    fn growth_yields_are_counted_from_the_option_kind() {
        let g = GrowthGroup {
            number: 3,
            options: vec![
                GrowthOptionDef { kind: "AddPresence".into(), number: Some(1), ..Default::default() },
                GrowthOptionDef { kind: "GainEnergy".into(), number: Some(4), ..Default::default() },
            ],
        };
        let y = g.yield_();
        assert_eq!((y.presence, y.energy), (1, 4));
        let dahan = GrowthOptionDef { kind: "AddPresenceDahan".into(), number: Some(2), ..Default::default() }.yield_();
        assert_eq!(dahan.placement, Some(Placement::WithDahan { range: 2 }));
    }

    #[test]
    fn innate_tiers_are_reached_in_order() {
        let tier = |els: &[(Element, u8)], body: &str| Tier {
            elements: { let mut s = ElementSet::new(); for (e, n) in els { s.add(*e, *n); } s },
            body: body.into(),
            effects: vec![],
            modifier: false,
        };
        let inn = Innate {
            id: "GatherTheWarriors".into(),
            title: "Gather the Warriors".into(),
            speed: Speed::Slow,
            range: Some(Range { distance: 1, from_sacred_site: false, origin_terrains: vec![] }),
            target: TargetKind("AnyLand".into()),
            tiers: vec![
                tier(&[(Element::Air, 1)], "Gather 1 Dahan"),
                tier(&[(Element::Air, 2), (Element::Fire, 1)], "Gather 2 Dahan"),
                Tier { modifier: true, ..tier(&[(Element::Air, 4)], "This Power may be Fast") },
            ],
            rider: vec![],
            effects: vec![],
        };
        let mut have = ElementSet::new();
        have.add(Element::Air, 2);
        assert_eq!(inn.reached(&have).0, 1);
        assert_eq!(inn.next_shortfall(&have).map(|(_, n)| n), Some(1));
        have.add(Element::Fire, 1);
        assert_eq!(inn.reached(&have).0, 2);
        have.add(Element::Air, 2);
        assert!(inn.active(&have).iter().any(|t| t.modifier), "4 Air turns the innate Fast");
        let only_modifier = ElementSet::from_list([Element::Sun, Element::Sun, Element::Sun, Element::Sun]);
        assert!(inn.active(&only_modifier).is_empty());
    }
}
