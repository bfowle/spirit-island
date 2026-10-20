//! Typed hooks for special rules, by spirit and rule. Each entry was read from
//! the engine's rule text on 2026-10-20 (`si-steam-dump defs`), which matches
//! the printed panel for these rules. A rule not listed here is `Untyped`
//! and the replay reports it as such; nothing is guessed from prose.

use si_model::spirit::RuleHook;
use si_model::Terrain;

pub fn for_rule(spirit: &str, rule_id: &str, title: &str) -> Vec<RuleHook> {
    let t = title.trim();
    match (spirit, t) {
        // Volcano Looming High
        ("VolcanoLoomingHigh", "Mountain Home") => vec![RuleHook::PlacementOnly(vec![Terrain::Mountain])],
        ("VolcanoLoomingHigh", "Collapse in a Blast of Lava and Steam") => vec![RuleHook::DamageOnPresenceDestroyed { per_presence: 1 }],
        ("VolcanoLoomingHigh", "Volcanic Peaks Tower Over the Landscape") => vec![RuleHook::RangeBonusFromOrigin { presence: 3, bonus: 1 }],
        // Thunderspeaker
        ("Thunderspeaker", "Ally of the Dahan") => vec![RuleHook::PresenceMovesWithDahan],
        ("Thunderspeaker", "Sworn to Victory") => vec![RuleHook::PresenceDestroyedPerDahanKilled { range: 1 }],
        // Stone's Unyielding Defiance
        ("StonesUnyieldingDefiance", "Bestow the Endurance of Bedrock") => vec![RuleHook::BlightDoesNotCascadeUnlessOutnumbered],
        _ => {
            let _ = rule_id;
            vec![RuleHook::Untyped]
        }
    }
}
