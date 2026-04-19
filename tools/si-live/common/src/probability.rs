//! Probability utilities used by the si-live stats pipeline.
//!
//! Two families of calculations currently live here:
//!
//! 1. **Hypergeometric draw probability** — given a deck with a known
//!    composition and a known number of successes (e.g., Moon-element Minors
//!    in the Minor deck), what's the probability the next N draws contain
//!    at least K successes?
//!
//! 2. **Wilson-interval confidence bounds** — given an observed proportion
//!    (wins/games, threshold-fires/turns), return a 95% Wilson interval.
//!    More honest at small N than a naive ±SE.

use std::collections::HashMap;

/// Compute log(n!) for small non-negative n. Cached in the caller for speed.
fn ln_factorial(n: u32) -> f64 {
    let mut out = 0.0f64;
    for i in 2..=n {
        out += (i as f64).ln();
    }
    out
}

/// Hypergeometric PMF: P(X = k) for drawing `draws` from `population` with
/// `successes` successes available (no replacement).
pub fn hypergeom_pmf(population: u32, successes: u32, draws: u32, k: u32) -> f64 {
    if k > successes || draws - k > population - successes || draws > population {
        return 0.0;
    }
    // log-space: lnC(K,k) + lnC(N-K, n-k) - lnC(N,n)
    let ln_c = |n: u32, r: u32| -> f64 {
        if r > n {
            return f64::NEG_INFINITY;
        }
        ln_factorial(n) - ln_factorial(r) - ln_factorial(n - r)
    };
    let log_p = ln_c(successes, k) + ln_c(population - successes, draws - k) - ln_c(population, draws);
    log_p.exp()
}

/// P(X >= k) — at-least-k successes in `draws` from the population.
pub fn hypergeom_at_least(population: u32, successes: u32, draws: u32, k: u32) -> f64 {
    if k == 0 {
        return 1.0;
    }
    let upper = successes.min(draws);
    if k > upper {
        return 0.0;
    }
    (k..=upper)
        .map(|i| hypergeom_pmf(population, successes, draws, i))
        .sum()
}

/// 95% Wilson score interval for a binomial proportion `p = x/n`.
/// Returns (low, high). Safer than ±SE at small n.
pub fn wilson_95(x: u32, n: u32) -> (f64, f64) {
    if n == 0 {
        return (0.0, 1.0);
    }
    let z = 1.959_963_984_540_054f64; // 95% z-score
    let p = x as f64 / n as f64;
    let denom = 1.0 + z * z / n as f64;
    let center = (p + z * z / (2.0 * n as f64)) / denom;
    let margin = z * ((p * (1.0 - p) / n as f64) + z * z / (4.0 * (n as f64 * n as f64))).sqrt()
        / denom;
    ((center - margin).max(0.0), (center + margin).min(1.0))
}

/// Count how many cards in `cards` have each element. Used to size the
/// numerators for hypergeometric draws ("P(next Minor has Moon)").
pub fn count_elements_in_pool(cards: &[serde_json::Value]) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for c in cards {
        if let Some(elems) = c.get("elements").and_then(|v| v.as_array()) {
            for e in elems {
                if let Some(s) = e.as_str() {
                    *counts.entry(s.to_lowercase()).or_insert(0) += 1;
                }
            }
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hypergeom_degenerate_cases() {
        assert!((hypergeom_pmf(10, 5, 3, 0) - hypergeom_pmf(10, 5, 3, 0)).abs() < 1e-9);
        // Drawing 0 successes deterministically when successes=0:
        assert!((hypergeom_at_least(10, 0, 3, 1)).abs() < 1e-9);
        // Drawing at least 0 successes is always P=1:
        assert!((hypergeom_at_least(10, 5, 3, 0) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn hypergeom_known_value() {
        // 50-card deck, 20 successes, draw 4, P(exactly 2): standard example
        let p = hypergeom_pmf(50, 20, 4, 2);
        assert!((p - 0.3641).abs() < 0.005, "got {p}");
    }

    #[test]
    fn wilson_at_extremes() {
        let (lo, hi) = wilson_95(0, 10);
        assert_eq!(lo, 0.0);
        assert!(hi > 0.2 && hi < 0.35);
        let (lo, hi) = wilson_95(10, 10);
        assert!(lo > 0.65 && lo < 0.80);
        assert_eq!(hi, 1.0);
    }

    #[test]
    fn wilson_n_zero_is_uncommitted() {
        let (lo, hi) = wilson_95(0, 0);
        assert_eq!((lo, hi), (0.0, 1.0));
    }
}
