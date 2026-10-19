//! The rules register's machine-readable half (`data/invader-rules.json`):
//! every adversary level, spirit, scenario, and Event effect that changes
//! the invader pipeline, with engine and Wiki evidence, an agreement status,
//! and a review status (reviewed by Brett, sourced from the rulebooks/FAQ,
//! or unreviewed; see `data/references/rules/register.md`).

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::OnceLock;

static REGISTER: OnceLock<Value> = OnceLock::new();

fn register() -> &'static Value {
    REGISTER.get_or_init(|| serde_json::from_str(include_str!("../data/invader-rules.json")).unwrap_or(Value::Null))
}

/// Kinds the companion's forecast already applies (through public data or
/// rules). Everything else is shown as a rule in play, not yet applied.
const APPLIED: &[&str] = &["deck_recipe", "fear_deck", "piece_health", "setup_pieces"];

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RuleInPlay {
    /// "Brandenburg-Prussia L2", "Escalation", "Shadows Flicker Like Flame", …
    pub from: String,
    pub title: String,
    pub kind: String,
    pub text: String,
    /// agree / conflict / engine-only / wiki-only.
    pub status: String,
    /// "reviewed by Brett (…)", "sourced (…)", or empty when unreviewed.
    pub review: String,
    pub applied: bool,
}

fn s(v: &Value, k: &str) -> String {
    v.get(k).and_then(Value::as_str).unwrap_or("").to_string()
}

fn effect(from: &str, title: &str, e: &Value) -> RuleInPlay {
    let kind = s(e, "kind");
    RuleInPlay {
        from: from.to_string(),
        title: title.to_string(),
        applied: APPLIED.contains(&kind.as_str()),
        kind,
        text: s(e, "text"),
        status: s(e, "status"),
        review: s(e, "review"),
    }
}

/// Effects in play for an adversary at a level (levels are cumulative).
pub fn adversary_rules(id: &str, level: i64) -> Vec<RuleInPlay> {
    let Some(a) = register().pointer(&format!("/adversaries/{id}")) else { return Vec::new() };
    let name = a.get("short_title").and_then(Value::as_str).unwrap_or(id).to_string();
    let mut out = Vec::new();
    for lvl in 0..=level.clamp(0, 6) {
        let Some(l) = a.pointer(&format!("/levels/{lvl}")) else { continue };
        let title = s(l, "title");
        for e in l.get("effects").and_then(Value::as_array).into_iter().flatten() {
            out.push(effect(&format!("{name} L{lvl}"), &title, e));
        }
    }
    if let Some(loss) = a.get("loss_condition").filter(|l| l.get("text").is_some_and(|t| !t.is_null())) {
        out.push(RuleInPlay {
            from: format!("{name} loss condition"),
            title: s(loss, "title"),
            kind: "loss_condition".into(),
            text: s(loss, "text"),
            status: s(loss, "status"),
            review: s(loss, "review"),
            applied: false,
        });
    }
    out
}

/// Pipeline effects of a spirit's special rules, innates, and its aspect.
pub fn spirit_rules(id: &str, title: &str, aspect: Option<&str>) -> Vec<RuleInPlay> {
    let Some(list) = register().pointer(&format!("/spirits/{id}")).and_then(Value::as_array) else { return Vec::new() };
    list.iter()
        .filter(|e| match e.get("aspect").and_then(Value::as_str) {
            None => true,
            Some(a) => Some(a) == aspect,
        })
        .map(|e| effect(title, &s(e, "source"), e))
        .collect()
}

/// The adversary's Stage II Escalation text, if the register has it.
pub fn escalation(id: &str) -> Option<(String, String, String)> {
    let e = register().pointer(&format!("/adversaries/{id}/escalation"))?;
    Some((s(e, "title"), s(e, "text"), s(e, "status")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_loads_and_prussia_l2_is_a_deck_recipe() {
        let r = adversary_rules("TheKingdomOfBrandenburgPrussia", 2);
        assert!(r.iter().any(|e| e.kind == "deck_recipe" && e.applied), "{r:#?}");
        assert!(r.iter().any(|e| e.kind == "escalation"));
        let england = adversary_rules("TheKingdomOfEngland", 3);
        assert!(england.iter().any(|e| e.kind == "extra_invader_action" && !e.applied));
        assert!(escalation("TheKingdomOfEngland").is_some());
    }
}
