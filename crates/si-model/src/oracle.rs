//! Oracles and the fuzzing that drives them, shared by every crate.
//!
//! An oracle is a rule's law stated from outside the code that applies it: given what
//! went in and what came out, the ways the result breaks the rule. Each type that
//! applies a rule keeps its oracle beside it (`effects::oracle` beside the effect
//! executor, `invaders::oracle` beside the Invader steps, a line's notation law beside
//! its parser) and a fuzz test in its own unit tests that feeds the oracle random
//! inputs from that type's generator. The corpus fuzzer (`si fuzz`) calls the same
//! oracles on every recorded game, so a law is written once.
//!
//! What lives here is what they all share: a seeded generator that is the same on every
//! machine, the violation a law reports, and `property`, the loop a fuzz test runs.

use std::fmt;

/// One way a result breaks a rule: the law's name and what was seen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub law: &'static str,
    pub detail: String,
}

impl Violation {
    pub fn new(law: &'static str, detail: impl Into<String>) -> Violation {
        Violation { law, detail: detail.into() }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.law, self.detail)
    }
}

/// Push a violation when `holds` is false.
pub fn check(out: &mut Vec<Violation>, holds: bool, law: &'static str, detail: impl FnOnce() -> String) {
    if !holds {
        out.push(Violation::new(law, detail()));
    }
}

/// xorshift64*: small, seeded, the same everywhere. A finding names its seed, and the
/// seed builds the same case again.
#[derive(Debug, Clone)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    /// A number in `0..n` (0 when n is 0).
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 { 0 } else { (self.next_u64() % n as u64) as usize }
    }
    /// A number in `lo..=hi`.
    pub fn between(&mut self, lo: u32, hi: u32) -> u32 {
        lo + self.below((hi - lo + 1) as usize) as u32
    }
    pub fn coin(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }
    /// True one time in `n`.
    pub fn one_in(&mut self, n: usize) -> bool {
        self.below(n) == 0
    }
    pub fn pick<'a, T>(&mut self, v: &'a [T]) -> Option<&'a T> {
        (!v.is_empty()).then(|| &v[self.below(v.len())])
    }
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }
}

/// The seed of the `i`th case of a property run: one per case, so a failure replays alone.
pub fn case_seed(seed: u64, i: usize) -> u64 {
    seed.wrapping_mul(1_000_003).wrapping_add(i as u64)
}

/// A fuzz test's loop: `cases` inputs from `generate`, each held to `law`. Panics on the
/// first input that breaks it, naming the property, the case's seed and every violation,
/// so `generate(&mut Rng::new(seed))` builds it again.
///
/// `SI_FUZZ_CASES=n` runs n cases instead and `SI_FUZZ_SEED=s` starts from another seed,
/// for a deeper run than the test suite's (`SI_FUZZ_CASES=200000 cargo test -p si-rules`).
pub fn property<T: fmt::Debug>(name: &str, seed: u64, cases: usize, mut generate: impl FnMut(&mut Rng) -> T, mut law: impl FnMut(&T) -> Vec<Violation>) {
    let cases = std::env::var("SI_FUZZ_CASES").ok().and_then(|v| v.parse().ok()).unwrap_or(cases);
    let seed = std::env::var("SI_FUZZ_SEED").ok().and_then(|v| v.parse().ok()).unwrap_or(seed);
    for i in 0..cases {
        let s = case_seed(seed, i);
        let input = generate(&mut Rng::new(s));
        let broken = law(&input);
        if !broken.is_empty() {
            let list: Vec<String> = broken.iter().map(|v| v.to_string()).collect();
            let shown = format!("{input:?}");
            let shown = if shown.len() > 6000 { format!("{}...", &shown[..6000]) } else { shown };
            panic!("{name}: case {i} (seed {s}) breaks the law:\n  {}\ninput: {shown}", list.join("\n  "));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_generator_is_the_same_everywhere() {
        let a: Vec<u64> = (0..4).map(|_| Rng::new(7).next_u64()).collect();
        assert!(a.windows(2).all(|w| w[0] == w[1]), "a seed builds the same numbers");
        let mut r = Rng::new(7);
        for _ in 0..1000 {
            assert!(r.below(5) < 5);
            let b = r.between(2, 4);
            assert!((2..=4).contains(&b));
        }
        assert_eq!(r.below(0), 0);
    }

    #[test]
    fn a_property_names_the_seed_that_breaks_it() {
        let caught = std::panic::catch_unwind(|| property("odd", 3, 50, |r| r.below(10), |n| if n % 2 == 1 { vec![Violation::new("even", format!("{n}"))] } else { vec![] }));
        let msg = caught.expect_err("an odd number comes up").downcast::<String>().unwrap();
        assert!(msg.contains("seed") && msg.contains("even:"), "{msg}");
    }
}
