//! The forces that decide a turn. A living list: every force named here has
//! a type in this crate and a column somewhere in the tempo report. Brett's
//! first seven (2026-10-20) plus the ones the model already carried.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Force {
    /// Energy to pay for cards, and a reserve for Event choices.
    Energy,
    /// Card plays: direct effects, and the elements they bank.
    CardPlays,
    /// Elements and innates: the "free" effects on top of the cards.
    ElementsAndInnates,
    /// Track slots beyond income and plays: permanent elements, Reclaim One, pay-to-gain, growth options.
    TrackUnlocks,
    /// Where presence may go, range, Sacred Sites, target kinds, origin rules.
    RangeAndTargeting,
    /// What the growth option itself yields.
    GrowthYield,
    /// Cards in hand against cards in the discard: combos, cycles, Reclaim.
    HandAndDiscard,
    /// Where a Spirit action lands on a land's Explore → Build → Ravage clock.
    LandTempo,
    /// Invader stage step-ups and adversary escalations.
    InvaderStage,
    /// Fear generated, Fear cards, Terror Level.
    Fear,
    /// Blight added, cascades, the Blight card.
    Blight,
    /// Dahan: counterattacks, movement, each spirit's dependence on them.
    Dahan,
    /// Event cards and the odds they carry.
    Events,
    /// Adversary and scenario rule changes.
    RuleModifiers,
    /// Other spirits at the table: gifts, shared lands, combined elements.
    SpiritSynergy,
}

impl Force {
    pub const ALL: [Force; 15] = [
        Force::Energy,
        Force::CardPlays,
        Force::ElementsAndInnates,
        Force::TrackUnlocks,
        Force::RangeAndTargeting,
        Force::GrowthYield,
        Force::HandAndDiscard,
        Force::LandTempo,
        Force::InvaderStage,
        Force::Fear,
        Force::Blight,
        Force::Dahan,
        Force::Events,
        Force::RuleModifiers,
        Force::SpiritSynergy,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Force::Energy => "energy economy",
            Force::CardPlays => "card plays",
            Force::ElementsAndInnates => "elements and innates",
            Force::TrackUnlocks => "track unlocks",
            Force::RangeAndTargeting => "range and targeting",
            Force::GrowthYield => "growth yield",
            Force::HandAndDiscard => "hand vs discard",
            Force::LandTempo => "land tempo",
            Force::InvaderStage => "invader stage",
            Force::Fear => "fear",
            Force::Blight => "blight",
            Force::Dahan => "dahan",
            Force::Events => "events",
            Force::RuleModifiers => "rule modifiers",
            Force::SpiritSynergy => "spirit synergy",
        }
    }

    /// The type in this crate that carries the force.
    pub fn carried_by(self) -> &'static str {
        match self {
            Force::Energy => "spirit::SpiritState.energy, spirit::TrackState.income, spirit::Yield.energy",
            Force::CardPlays => "spirit::TrackState.plays, spirit::SpiritState.plays_this_turn / played_elements",
            Force::ElementsAndInnates => "spirit::Innate, spirit::Tier, element::ElementSet::satisfies",
            Force::TrackUnlocks => "spirit::TrackNode (elements, permanent_element, reclaim_one, pay_to_gain, growth_option)",
            Force::RangeAndTargeting => "power::Range, power::TargetKind, spirit::Placement, spirit::RuleHook::{PlacementOnly, RangeBonusFromOrigin}",
            Force::GrowthYield => "spirit::GrowthOptionDef::yield_, spirit::Yield",
            Force::HandAndDiscard => "spirit::SpiritState.{hand, play_area, discard, forgotten}, spirit::Reclaim, TrackNode.reclaim_one",
            Force::LandTempo => "event::InvaderStep, event::Event::{InvaderCardResolved, PowerUsed, BlightAdded}",
            Force::InvaderStage => "decks::InvaderCard.stage / escalation, setup::AdversaryLevel",
            Force::Fear => "decks::FearState, island::PieceKind::fear_on_destroy",
            Force::Blight => "island::Land.blight, decks::BlightState, event::Event::BlightAdded.cascade",
            Force::Dahan => "island::PieceKind::Dahan, spirit::RuleHook::{PresenceMovesWithDahan, PresenceDestroyedPerDahanKilled}",
            Force::Events => "decks::EventCard, decks::EventSection",
            Force::RuleModifiers => "modifier::RuleSet, modifier::Source",
            Force::SpiritSynergy => "setup::GameSetup.seats, power::TargetKind::is_spirit",
        }
    }
}
