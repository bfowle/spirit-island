//! Lumia's ledgers (reports/Spirit Island companion stat models.md, "Ledgers,
//! growth classes and timelines give every number a budget"): state a hard
//! budget, inventory what this turn spends from it in resolution order,
//! report the shortfall. Every number is a count of game units (Blight,
//! Fear, cards), so each one has a pocket rule.

use super::adversary::LossClock;
use super::{BlightStats, FearStats};
use crate::companion_stats::invaders::{InvaderPhaseForecast, RavageForecast};
use crate::public::PublicSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Ledger {
    pub turn: u32,
    pub budgets: Vec<Budget>,
}

/// One budget: units left, this turn's spend, and the shortfall.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Budget {
    /// "blight", "fear", "deck", or "clock" (an adversary loss condition).
    pub key: String,
    pub title: String,
    /// The unit counted, singular: "Blight", "Fear", "card", "Town/City", …
    pub unit: String,
    /// Units left before the budget runs out.
    pub left: i64,
    /// The budget's full size, for drawing it (left ≤ size).
    pub size: i64,
    /// What happens when it runs out.
    pub at_zero: String,
    /// A goal to reach (Fear), not a limit to keep.
    pub goal: bool,
    /// This turn's spend (gains, for a goal), in resolution order.
    pub items: Vec<Spend>,
    /// Units that must be prevented this turn so the budget holds (for a
    /// goal: still to find after this turn's sure gains).
    pub shortfall: i64,
    /// Next turn's spend if nothing changes, when known.
    pub next: Option<i64>,
    pub pocket_rule: String,
    /// Extra lines: the blighted side's spread, Terror Level marks, …
    pub detail: Vec<String>,
    /// A loss that comes before this turn's spends: a clock checked at the
    /// end of the Fast Phase that is already at its limit.
    #[serde(default)]
    pub urgent: Option<String>,
}

/// One entry in a budget's inventory.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Spend {
    /// The step it resolves in: "Fear", "High Immigration", "Ravage", "Build", "Explore".
    pub step: String,
    pub land: Option<String>,
    pub amount: i64,
    /// What it is, briefly: "5 Damage vs 2, cascades".
    pub label: String,
    /// What prevents it, exactly: "Defend 4".
    pub solve: Option<String>,
    /// The Defend that prevents it, as a number (for a what-if).
    #[serde(default)]
    pub defend: Option<i64>,
    /// False when it depends on a choice or an unseen card.
    pub certain: bool,
}

fn total(items: &[Spend]) -> i64 {
    items.iter().map(|s| s.amount).sum()
}

/// One Ravage's Blight: the land's own, the extra from adversary rules, and
/// the fewest cascade Blight.
fn ravage_spend(r: &RavageForecast, step: &str, cascade_min: i64) -> Spend {
    let l = &r.ledger;
    let cascade = if l.cascades { cascade_min.max(1) } else { 0 };
    let mut label = format!("{} Damage vs {}", l.damage_after_defend, l.land_health);
    if l.extra_blight > 0 {
        label += &format!(", +{} extra", l.extra_blight);
    }
    if cascade > 0 {
        label += ", cascades";
    }
    // Defend that solves the land: damage − Health + 1 (the report's D − h + 1).
    let margin = l.damage_after_defend - l.land_health + 1;
    Spend {
        step: step.into(),
        land: Some(r.land.clone()),
        amount: 1 + l.extra_blight + cascade,
        label,
        solve: Some(format!("Defend {margin}, or remove {margin} Damage")),
        defend: Some(margin),
        certain: true,
    }
}

pub fn ledger(
    s: &PublicSnapshot,
    phase: &InvaderPhaseForecast,
    blight: &BlightStats,
    fear: &FearStats,
    clocks: &[LossClock],
    cascade_min: impl Fn(&str) -> i64,
) -> Ledger {
    let mut budgets = Vec::new();

    // ---- Blight: the card is the budget; it flips (or ends the game) empty.
    let mut items: Vec<Spend> = Vec::new();
    for r in phase.ravages.iter().filter(|r| !r.done && r.ledger.adds_blight) {
        items.push(ravage_spend(r, "Ravage", cascade_min(&r.land)));
    }
    for r in phase.build_ravages.iter().filter(|r| !r.done && r.ledger.adds_blight) {
        items.push(ravage_spend(r, "Build", cascade_min(&r.land)));
    }
    let still_healthy = s.blight.blighted_text.iter().any(|t| t.contains("draw a new Blight Card"));
    let at_zero = if !blight.flipped {
        "the card flips to its blighted side".to_string()
    } else if still_healthy {
        "a new Blight Card is drawn (Still-Healthy)".to_string()
    } else {
        "the spirits lose".to_string()
    };
    let spend = total(&items);
    let mut detail = Vec::new();
    if !blight.flipped && !blight.flipped_side.is_empty() {
        let n: u32 = blight.flipped_side.iter().map(|(_, c)| c).sum();
        let spread: Vec<String> = blight.flipped_side.iter().map(|(v, c)| format!("{c}×{v}")).collect();
        detail.push(format!("Blighted side, {n} unseen cards: {} Blight", spread.join(" · ")));
    }
    budgets.push(Budget {
        key: "blight".into(),
        title: "Blight".into(),
        unit: "Blight".into(),
        left: blight.remaining,
        size: blight.remaining.max(spend).max(blight.remaining + blight.on_island as i64).max(1),
        at_zero,
        goal: false,
        // Keep at least one on the card: prevent everything past left − 1.
        shortfall: (spend - (blight.remaining - 1)).max(0),
        items,
        next: blight.next_turn.as_ref().map(|n| n.adds_min),
        urgent: None,
        pocket_rule: "A land Blights when its Damage after Defend reaches its Health (2). The Defend that solves it is Damage − 1.".into(),
        detail,
    });

    // ---- Fear: a goal. This turn's sure gains are the counterattacks' best case.
    let mut items = Vec::new();
    for r in phase.ravages.iter().filter(|r| !r.done && r.ledger.counterattack_fear > 0) {
        let destroys = r.ledger.counterattack_can_destroy.join(", ");
        items.push(Spend {
            step: "Ravage".into(),
            land: Some(r.land.clone()),
            amount: r.ledger.counterattack_fear,
            label: format!("Dahan counterattack destroys {destroys}"),
            solve: None,
            defend: None,
            // Depends on how the Dahan's Damage is assigned (best case shown).
            certain: false,
        });
    }
    let mut detail = Vec::new();
    if let Some(n) = fear.to_next_level {
        let next = ["I", "II", "III"].get(fear.terror_level as usize).copied().unwrap_or("III");
        detail.push(format!("Terror Level {next} in {n} Fear"));
    }
    detail.push(format!("Fear victory in {} Fear ({})", fear.to_clear_deck, fear.victory_condition));
    if fear.earned_banked > 0 {
        detail.push(format!("{} Fear Card{} banked, resolving in the Fear step", fear.earned_banked, if fear.earned_banked == 1 { "" } else { "s" }));
    }
    let gains = total(&items);
    budgets.push(Budget {
        key: "fear".into(),
        title: "Fear".into(),
        unit: "Fear".into(),
        left: fear.to_next_card,
        size: fear.required_per_card.max(1),
        at_zero: "a Fear Card is earned".into(),
        goal: true,
        shortfall: (fear.to_next_card - gains).max(0),
        items,
        next: None,
        urgent: None,
        pocket_rule: format!("{} Fear per card; destroying a Town gives 1, a City 2.", fear.required_per_card),
        detail,
    });

    // ---- Invader deck: time. An Explore with no card left loses the game.
    let left = s.invaders.upcoming_stages.len() as i64;
    let explores_now = phase.explore.revealed.is_none() && phase.position != "after" && left > 0;
    budgets.push(Budget {
        key: "deck".into(),
        title: "Invader deck".into(),
        unit: "card".into(),
        left,
        size: left.max(1),
        at_zero: "the next Explore loses the game".into(),
        goal: false,
        items: if explores_now {
            vec![Spend { step: "Explore".into(), land: None, amount: 1, label: "this turn's Explore card".into(), solve: None, defend: None, certain: true }]
        } else {
            Vec::new()
        },
        shortfall: 0,
        next: Some(1),
        urgent: None,
        pocket_rule: "One card per turn: the deck is the number of turns left.".into(),
        detail: s.invaders.upcoming_stages.first().map(|st| format!("Next card: Stage {}", ["I", "II", "III"][(*st as usize).clamp(1, 3) - 1])).into_iter().collect(),
    });

    // ---- Each adversary loss condition.
    for c in clocks {
        let over = c.trigger == "over";
        // Units left before it triggers: "at" triggers at the limit, "over" past it.
        let left = if over { c.limit - c.now + 1 } else { c.limit - c.now };
        let spend = c.after.map(|a| a - c.now).unwrap_or(0);
        // Checked at the end of the Fast Phase and already there: the loss
        // comes before this turn's Invader Phase (Habsburg game, Turn 4:
        // Mountain NE7 at 8 was lost at the end of the Fast Phase).
        let before_check = matches!(s.step.as_str(), "SpiritPhase" | "Grow" | "GainEnergy" | "PlayPayPower" | "FastPower");
        let past = if over { c.now - c.limit } else { c.now - c.limit + 1 };
        let urgent = (c.deferred && before_check && past > 0).then(|| {
            let at = c.at.as_deref().map(|l| format!(" from {l}")).unwrap_or_default();
            format!("Invaders win at the end of this Fast Phase: remove {past}{at} now")
        });
        // A deferred check reached this Invader Phase is lost at the end of
        // next turn's Fast Phase: the shortfall is next turn's, not this one's.
        let mut detail = Vec::new();
        let shortfall = if c.deferred && urgent.is_none() {
            if spend > 0 && spend >= left {
                detail.push(format!("reaches {} of {} this Invader Phase: checked at the end of next turn's Fast Phase", c.now + spend, c.limit));
            }
            0
        } else {
            (spend - (left - 1)).max(0)
        };
        if spend == 0 {
            detail.push(c.line.clone());
        }
        budgets.push(Budget {
            urgent,
            key: "clock".into(),
            title: c.title.clone(),
            unit: c.unit.clone(),
            left: left.max(0),
            size: (if over { c.limit + 1 } else { c.limit }).max(1),
            at_zero: format!("the Invaders win{}", if c.checked.is_empty() { String::new() } else { format!(" (checked {})", c.checked) }),
            goal: false,
            items: if spend > 0 {
                vec![Spend { step: "Invader Phase".into(), land: c.at.clone(), amount: spend, label: c.line.clone(), solve: None, defend: None, certain: true }]
            } else {
                Vec::new()
            },
            shortfall,
            next: None,
            pocket_rule: c.note.clone(),
            detail,
        });
    }
    Ledger { turn: phase.turn, budgets }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::RavageLedger;

    fn ravage(land: &str, after: i64, cascades: bool, extra: i64) -> RavageForecast {
        RavageForecast {
            land: land.into(),
            ledger: RavageLedger { damage_after_defend: after, land_health: 2, adds_blight: after >= 2, cascades, extra_blight: extra, ..Default::default() },
            done: false,
        }
    }

    #[test]
    fn blight_budget_counts_cascades_and_shortfall() {
        // Prussia game, Turn 5: 1 Blight left on a Still-Healthy flipped
        // card; B2 and B3 Blight with cascades → 4 spent, 4 to prevent.
        let mut s = PublicSnapshot::default();
        s.blight.blighted_text = vec!["…draw a new Blight Card…".into()];
        let phase = InvaderPhaseForecast { turn: 5, ravages: vec![ravage("B2", 5, true, 0), ravage("B3", 4, true, 0)], ..Default::default() };
        let blight = BlightStats { remaining: 1, flipped: true, ..Default::default() };
        let l = ledger(&s, &phase, &blight, &FearStats::default(), &[], |_| 1);
        let b = &l.budgets[0];
        assert_eq!((b.left, total(&b.items), b.shortfall), (1, 4, 4));
        assert_eq!(b.items[0].solve.as_deref(), Some("Defend 4, or remove 4 Damage"));
        assert!(b.at_zero.contains("new Blight Card"));
    }

    #[test]
    fn heavy_mining_extra_counts() {
        let phase = InvaderPhaseForecast { ravages: vec![ravage("F8", 7, false, 1)], ..Default::default() };
        let blight = BlightStats { remaining: 3, ..Default::default() };
        let l = ledger(&PublicSnapshot::default(), &phase, &blight, &FearStats::default(), &[], |_| 0);
        assert_eq!((total(&l.budgets[0].items), l.budgets[0].shortfall), (2, 0));
    }
}
