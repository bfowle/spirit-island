//! Track-tempo targets by spirit, each with its source. Only spirits with a
//! guide or a chapter we wrote get one; nothing is invented. A `by_turn` is
//! set only where the guide states one.

use si_model::spirit::Spike;

pub fn spikes_for(spirit_id: &str) -> Vec<Spike> {
    let s = |income, plays, src: &str| Spike::new(income, plays, src);
    let by = |income, plays, by: u32, src: &str| {
        let mut sp = Spike::new(income, plays, src);
        sp.by_turn = Some(by);
        sp
    };
    match spirit_id {
        "VengeanceAsABurningPlague" => vec![
            s(3, 3, "Rei: unlocking to 3/3cp is your power spike"),
            by(0, 3, 4, "Rei: getting to 3 plays by Turn 4 scales you at a faster rate than most Adversaries can handle"),
        ],
        "ManyMindsMoveAsOne" => vec![
            s(2, 4, "Rei: for Minors, 2/4cp is your power spike"),
            s(3, 3, "Rei: for Majors, 3/3cp is your power spike"),
            by(2, 3, 4, "Rei: the Turn Timer example, 2e/3 card plays by Turn 4"),
        ],
        "ShroudOfSilentMist" => vec![
            s(1, 3, "Rei: your power spike at 1/3cp"),
            s(1, 4, "Rei: 1/4cp + reclaim is when you Transcend"),
        ],
        _ => vec![],
    }
}
