//! The printed effect text of Power Cards and innate tiers, parsed into
//! `si_model::power::Effect`. Sentence by sentence; whatever does not match a
//! known shape stays `Other(sentence)` so coverage can be reported and the
//! grammar grown against the real cards (`si cards --unparsed`).

use std::sync::LazyLock;

use regex::Regex;
use si_model::island::TokenKind;
use si_model::power::{DamageTarget, Effect, PieceFilter, PowerKind};

macro_rules! re {
    ($name:ident, $pat:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($pat).unwrap());
    };
}

re!(TAGS, r"<i>.*?</i>|<[^>]+>|\{BR\}|\{Bullet\}|\{EndBullet\}|\((?:max|min|count max)\.?[^)]*\)|\(\d+x/game\)");
re!(SPACES, r"\s+");
re!(LEADERS, r"^(?:Then,? |Then: |First, |Also, |Now: |Ongoing: |Bargain: |Instead: )");
re!(FEAR, r"^\+?(\d+) \{Fear\}(?:, before the Terror Level check| there)?$");
re!(FEAR_IF_ANY, r"^(\d+) \{Fear\} if (.+)$");
re!(AFTER, r"^After (.+?), (.+)$");
re!(CHOOSE, r"^Choose (?:up to )?\d+ (?:different )?(?:Invaders|(?:\{\w+\}/?)+)$");
re!(FEAR_IF, r"^(\d+) \{Fear\} if Invaders are present(?: or adjacent)?$|^If Invaders are present, (\d+) \{Fear\}$");
re!(FEAR_PER, r"^\+?(\d+) \{Fear\} (?:for each|per) (\{\w+\}|Invader|type of Invader present|\d+ \{\w+\})");
re!(DAMAGE, r"^(?:deal |do )?(\+)?(\d+) Damage(.*)$");
re!(DAMAGE_EQUAL, r"^Damage equal to");
re!(DOES_BONUS, r"^(?:Target Spirit |You |They )?(?:does|do|deal|deals) \+(\d+) Damage");
re!(DEFEND, r"^(?:In one of (?:t|T)arget Spirit's lands, )?Defend (\d+)(?: in target(?: land)? and all adjacent lands| there| in that land)?$");
re!(DEFEND_BONUS, r"^(?:Defend \+(\d+)|\+(\d+) Defend)$");
re!(DEFEND_PER, r"^(?:In one of (?:t|T)arget Spirit's lands, )?Defend (\d+) per (\{\w+\}|Invader)");
re!(DEFEND_GRANT, r"(?:grants|provides) Defend (\d+)");
re!(PUSH, r"^(?:You may |Target Spirit may )?(?:Gather or )?Push (?:up to |another )?(\d+|all|any number of) (?:of (?:your|their) )?((?:(?:\{\w+\}|Invaders?)/?)+)(?: and (?:up to )?(\d+) ((?:(?:\{\w+\}|Invaders?)/?)+))?(?: per (\{\w+\}))?");
re!(PUSH_HALF, r"^Push up to half \(round (?:up|down)\) of (\{\w+\}|Invaders)");
re!(PUSH_IT, r"^(?:You may )?Push (?:that|it|them|the) ?(\{\w+\})?(?: you added \{\w+\} to)?$");
re!(GATHER, r"^(?:You may |Target Spirit )?Gathers? (?:up to )?(\d+|all) (?:of (?:your|their) )?((?:(?:\{\w+\}|Invaders?)/?)+)(?: and (?:up to )?(\d+) ((?:(?:\{\w+\}|Invaders?)/?)+))?(?: per (\{\w+\}))?");
re!(MOVE, r"^(?:You may )?Move (?:up to )?(\d+) (?:of (?:your|their|target Spirit's) )?((?:(?:\{\w+\}|Invaders?)/?)+)");
re!(ADD, r"^(?:Target Spirit |They |You )?(?:Add|add)s? ((?:(?:up to )?\d+ (?:additional )?\{\w+\}(?:, | and |, and )?)+)");
re!(PIECE_N, r"(\d+) (?:additional )?\{(\w+)\}");
re!(REMOVE, r"^(?:Remove|remove) (?:up to )?(\d+|all|any number of) (?:Damaged )?((?:(?:\{\w+\}|Invaders?)/?)+)");
re!(REMOVE_HALF, r"^Remove up to half \(round (?:up|down)\) of (\{\w+\}|Invaders)");
re!(ABDUCT, r"^Abduct (?:up to )?(\d+|it) ?((?:(?:\{\w+\}|Invaders?)/?)*)");
re!(DESTROY, r"^(?:Destroy|destroy) (all|\d+) ((?:(?:\{\w+\}|Invaders?)/?)+)");
re!(DROWN, r"^Drown (.+)$");
re!(REPLACE, r"^(?:You may )?Replace (\d+) (?:additional )?\{(\w+)\} with (?:(\d+) )?\{(\w+)\}");
re!(ENERGY, r"^(?:[Tt]arget Spirit |they |you |They |You )?(?:also )?(?:gains?|Gain) (\d+) Energy$");
re!(CARD, r"^(?:[Tt]arget Spirit |they |you |They |You )?(?:also )?(?:gains?|Gain) an? (Minor|Major)? ?Power(?: Card)?$");
re!(ELEMENT, r"^(?:[Tt]arget Spirit |you |You )?(?:gains?|Gain) (\d+) (?:different Elements|\{\w+\})|^Prepare (\d+) Element Marker|^Target Spirit gains those Elements");
re!(RANGE, r"^(?:Target Spirit gets|[Tt]his Power has|Increase this Power's Range to) \{\+?(\d+)Range\}");
re!(DOWNGRADE, r"^Downgrade (?:up to )?(\d+|any number of) (?:Invader|\{)");
re!(NO_PART, r"^(?:Up to )?(\d+) (?:damaged )?Invaders (?:do not|don't) participate in Ravage");
re!(DAMAGE_MINUS, r"^(?:Invaders do|Each Invader does) -(\d+) Damage");
re!(SKIP_N, r"^(?:Invaders )?[Ss]kip (?:up to )?(\d+|one) (?:Invader |Ravage/Build )?Actions?");
re!(FAST_COST, r"^You may pay (\d+) Energy to make this [Pp]ower \{Fast\}$");
re!(FAST, r"^This [Pp]ower (?:may be|is) \{Fast\}$");
re!(TERROR, r"^\{(Terror\w+)\} ?: ?(.+)$");
re!(FOR_EACH, r"^For (?:each|every) (\{\w+\}(?:/\{\w+\})?|Invader|type of token you added|token removed|of your \{SacredSite\}|adjacent land with your \{Presence\})(?: in target land| present| Removed| you have| in/adjacent to target land| on your “Deep Slumber” track)?,? (.+)$");
re!(EACH_DOES, r"^Each (?:remaining )?(\{\w+\}|Invader) (does|deals|takes|destroys) (.+)$");
re!(IF_COMMA, r"^If (.+?), (.+)$");
re!(IF_COLON, r"^If (.+?): ?(.+)$");
re!(OTHERWISE, r"^Otherwise,? ?:? ?(.+)$");
re!(INSTEAD, r"^[Ii]nstead,? (.+)$");
re!(MAY_INSTEAD, r"^(?:You|you|They|they|Target Spirit|target Spirit) may instead (.+)$");
re!(PAY, r"^(?:You may |Target Spirit may |Spirits may )?[Pp]ay (\d+) Energy to (?:instead )?(.+)$");
re!(AREA, r"^In (each adjacent land|both origin land and target land|up to \d+ adjacent lands(?: with \{\w+\})?|a land adjacent to both origin and target|each other Coastal land on the same board|lands where you did Damage|that land and each adjacent land|one of (?:target|Target) Spirit's lands|one of your lands|each target land|that land|lands you Pushed \{Dahan\} to|any \d+ lands with \d+ or more Invaders|any \d+ lands on the island|both target and one adjacent land|a single land within \{\d+Range\} of your \{Presence\})[,:]? ?(.+)$");
re!(SOMEONE_MAY, r"^(?:Target Spirit|They|You) (?:may |also |now |may also )(.+)$");

/// Strip the engine's markup, reminder text and "(max. N)" notes, collapse whitespace.
pub fn clean(s: &str) -> String {
    let s = TAGS.replace_all(s, " ");
    SPACES.replace_all(s.trim(), " ").replace(" .", ".").replace(" ,", ",").trim().to_string()
}

/// Split at ". " when a new sentence starts (capital, `{`, `+` or a digit).
fn sentences(line: &str) -> Vec<String> {
    let text = clean(line);
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut start = 0;
    for i in 0..chars.len() {
        if chars[i] == '.' && i + 2 < chars.len() && chars[i + 1] == ' ' && (chars[i + 2].is_ascii_uppercase() || matches!(chars[i + 2], '{' | '+') || chars[i + 2].is_ascii_digit()) {
            out.push(chars[start..=i].iter().collect::<String>());
            start = i + 2;
        }
    }
    out.push(chars[start..].iter().collect::<String>());
    out.into_iter().map(|s| s.trim().trim_end_matches('.').trim().to_string()).filter(|s| !s.is_empty() && *s != "{Or}").collect()
}

/// Parse a body (one string per printed line). `{Or}` lines split the body
/// into the branches of a `Choice`.
pub fn parse(lines: &[String]) -> Vec<Effect> {
    let groups: Vec<Vec<&String>> = lines.iter().fold(vec![vec![]], |mut acc, l| {
        if l.trim() == "{Or}" {
            acc.push(vec![]);
        } else {
            acc.last_mut().unwrap().push(l);
        }
        acc
    });
    if groups.len() > 1 {
        return vec![Effect::Choice(groups.iter().map(|g| parse_lines(g)).collect())];
    }
    parse_lines(&groups[0])
}

fn parse_lines(lines: &[&String]) -> Vec<Effect> {
    lines.iter().flat_map(|l| sentences(l)).flat_map(|s| parse_sentence(&s)).collect()
}

fn typed(v: &[Effect]) -> bool {
    !v.is_empty() && !v.iter().any(Effect::is_other)
}

/// Parse one sentence (no trailing period). A header sentence ("Then
/// either:") parses to nothing; an unknown one to `Other`.
pub fn parse_sentence(raw: &str) -> Vec<Effect> {
    let s = LEADERS.replace(raw.trim(), "").to_string();
    if s.is_empty() {
        return vec![];
    }
    if let Some(e) = direct(&s) {
        return e;
    }
    // "A and B", "A, B and C", "A or B": every part must parse on its own.
    for sep in [", and ", " and ", ", ", "; "] {
        if s.contains(sep) {
            let parts: Vec<Vec<Effect>> = s.split(sep).map(parse_sentence).collect();
            if parts.iter().all(|p| typed(p)) {
                return parts.into_iter().flatten().collect();
            }
        }
    }
    if s.contains(" or ") {
        let parts: Vec<Vec<Effect>> = s.split(" or ").map(parse_sentence).collect();
        if parts.iter().all(|p| typed(p)) {
            return vec![Effect::Choice(parts)];
        }
    }
    if let Some(c) = SOMEONE_MAY.captures(&s) {
        let then = parse_sentence(&c[1]);
        if typed(&then) {
            return vec![Effect::Conditional { condition: "target Spirit may".into(), then }];
        }
    }
    if s.ends_with(':') {
        return vec![];
    }
    vec![Effect::Other(s)]
}

fn count(s: &str) -> u8 {
    match s {
        "all" | "any number of" => 255,
        "one" => 1,
        n => n.parse().unwrap_or(1),
    }
}

fn filters(s: &str) -> Vec<PieceFilter> {
    s.split('/').filter(|p| !p.is_empty()).map(PieceFilter::parse).collect()
}

fn direct(s: &str) -> Option<Vec<Effect>> {
    let one = |e: Effect| Some(vec![e]);
    let lower = s.to_ascii_lowercase();
    // --- whole-sentence idioms -------------------------------------------
    if lower.starts_with("isolate") {
        return one(Effect::Isolate);
    }
    if lower.contains("repeat this power") || lower.starts_with("repeat on an adjacent land") {
        return one(Effect::Repeat);
    }
    if lower.starts_with("you may do both") || lower.starts_with("they may do both") {
        return one(Effect::Both);
    }
    if lower.contains("do likewise") {
        return one(Effect::Repeat);
    }
    if CHOOSE.is_match(s) {
        return Some(vec![]);
    }
    if lower.contains("may be immediately pushed") {
        return one(Effect::Push { count: 255, what: vec![PieceFilter::AnyInvader] });
    }
    if let Some(c) = FEAR_IF.captures(s) {
        return one(Effect::FearIfInvaders(count(c.get(1).or(c.get(2)).unwrap().as_str())));
    }
    if let Some(c) = FEAR_IF_ANY.captures(s) {
        return one(Effect::Conditional { condition: c[2].to_string(), then: vec![Effect::Fear(count(&c[1]))] });
    }
    if lower.contains("skip ravage actions") || lower.contains("skip all ravage actions") || lower.contains("do not ravage in target land") {
        return one(Effect::SkipRavage);
    }
    if lower.contains("skip all actions") || lower.contains("skip all invader actions") {
        return one(Effect::SkipAll);
    }
    if lower.contains("skip the next build action") || lower.contains("do not build in target land") || lower.contains("do not build there") {
        return one(Effect::SkipBuild);
    }
    if let Some(c) = SKIP_N.captures(s) {
        return one(Effect::SkipAction(count(&c[1])));
    }
    if lower.starts_with("skip one invader action") || lower.contains("invaders skip one action") {
        return one(Effect::SkipAction(1));
    }
    if lower.contains("with {strife} don't participate in ravage") {
        return one(Effect::NoParticipation(255));
    }
    if let Some(c) = NO_PART.captures(s) {
        return one(Effect::NoParticipation(count(&c[1])));
    }
    if lower.starts_with("{dahan} take no damage from ravaging invaders") || lower.starts_with("{dahan} cannot be damaged") || lower.contains("would be destroyed in target land, destroy") || lower.ends_with("{dahan} does not participate in ravage") {
        return one(Effect::DahanImmune);
    }
    if let Some(c) = DAMAGE_MINUS.captures(s) {
        return one(Effect::InvaderDamageMinus(count(&c[1])));
    }
    if let Some(c) = FAST_COST.captures(s) {
        return one(Effect::MayBeFast { cost: Some(count(&c[1])) });
    }
    if FAST.is_match(s) {
        return one(Effect::MayBeFast { cost: None });
    }
    // --- structure ------------------------------------------------------
    if let Some(c) = TERROR.captures(s) {
        let then = parse_sentence(&c[2]);
        if typed(&then) {
            return one(Effect::Conditional { condition: c[1].to_string(), then });
        }
    }
    if let Some(c) = FOR_EACH.captures(s) {
        let then = parse_sentence(&c[2]);
        if typed(&then) {
            return one(Effect::Per { per: PieceFilter::parse(c[1].split(' ').next().unwrap()), then });
        }
    }
    if let Some(c) = EACH_DOES.captures(s) {
        let then = if &c[2] == "destroys" {
            PIECE_N.captures_iter(&c[3]).map(|p| Effect::Destroy { count: count(&p[1]), what: vec![PieceFilter::parse(&p[2])] }).collect()
        } else {
            parse_sentence(&c[3])
        };
        if typed(&then) {
            return one(Effect::Per { per: PieceFilter::parse(&c[1]), then });
        }
    }
    if let Some(c) = AFTER.captures(s) {
        let then = parse_sentence(&c[2]);
        if typed(&then) {
            return one(Effect::Conditional { condition: format!("after {}", &c[1]), then });
        }
    }
    if let Some(c) = AREA.captures(s) {
        let then = parse_sentence(&c[2]);
        if typed(&then) {
            return one(Effect::Conditional { condition: format!("in {}", &c[1]), then });
        }
    }
    if let Some(c) = PAY.captures(s) {
        let then = parse_sentence(&c[2]);
        if typed(&then) {
            return one(Effect::Conditional { condition: format!("pay {} Energy", &c[1]), then });
        }
    }
    for (re, name) in [(&*IF_COMMA, None), (&*IF_COLON, None), (&*OTHERWISE, Some("otherwise")), (&*INSTEAD, Some("instead")), (&*MAY_INSTEAD, Some("instead"))] {
        if let Some(c) = re.captures(s) {
            let (cond, rest) = match name {
                Some(n) => (n.to_string(), c[1].to_string()),
                None => (c[1].to_string(), c[2].to_string()),
            };
            let then = parse_sentence(&rest);
            if typed(&then) {
                return one(Effect::Conditional { condition: cond, then });
            }
        }
    }
    // --- atoms ----------------------------------------------------------
    if let Some(c) = FEAR.captures(s) {
        return one(Effect::Fear(count(&c[1])));
    }
    if let Some(c) = FEAR_PER.captures(s) {
        return one(Effect::Per { per: PieceFilter::parse(c[2].split(' ').last().unwrap()), then: vec![Effect::Fear(count(&c[1]))] });
    }
    if let Some(c) = DEFEND_PER.captures(s) {
        return one(Effect::Per { per: PieceFilter::parse(&c[2]), then: vec![Effect::Defend(count(&c[1]))] });
    }
    if let Some(c) = DEFEND.captures(s) {
        return one(Effect::Defend(count(&c[1])));
    }
    if let Some(c) = DEFEND_BONUS.captures(s) {
        return one(Effect::Defend(count(c.get(1).or(c.get(2)).unwrap().as_str())));
    }
    if let Some(c) = DEFEND_GRANT.captures(s) {
        let d = Effect::Defend(count(&c[1]));
        return one(if lower.contains("{presence}") { Effect::Per { per: PieceFilter::Presence, then: vec![d] } } else { d });
    }
    if let Some(c) = DOES_BONUS.captures(s) {
        return one(Effect::DamageBonus(count(&c[1])));
    }
    if DAMAGE_EQUAL.is_match(s) {
        return one(Effect::Damage { amount: 1, to: DamageTarget::Invaders });
    }
    if let Some(c) = DAMAGE.captures(s) {
        let bonus = c.get(1).is_some();
        let amount = count(&c[2]);
        let rest = c[3].trim().to_string();
        if let Some(per) = rest.strip_prefix("per ") {
            let per_word = per.split([' ', ',']).next().unwrap_or(per);
            let to = damage_target(&rest);
            return one(Effect::Per { per: PieceFilter::parse(per_word), then: vec![Effect::Damage { amount, to }] });
        }
        if bonus {
            return one(Effect::DamageBonus(amount));
        }
        return one(Effect::Damage { amount, to: damage_target(&rest) });
    }
    if let Some(c) = PUSH.captures(s) {
        let push = Effect::Push { count: count(&c[1]), what: filters(&c[2]) };
        let mut out = match c.get(5) {
            Some(per) => vec![Effect::Per { per: PieceFilter::parse(per.as_str()), then: vec![push] }],
            None => vec![push],
        };
        if let (Some(n), Some(w)) = (c.get(3), c.get(4)) {
            out.push(Effect::Push { count: count(n.as_str()), what: filters(w.as_str()) });
        }
        return Some(out);
    }
    if let Some(c) = PUSH_HALF.captures(s) {
        return one(Effect::Push { count: 128, what: filters(&c[1]) });
    }
    if let Some(c) = PUSH_IT.captures(s) {
        return one(Effect::Push { count: 1, what: vec![c.get(1).map(|m| PieceFilter::parse(m.as_str())).unwrap_or(PieceFilter::Other("it".into()))] });
    }
    if let Some(c) = GATHER.captures(s) {
        let gather = Effect::Gather { count: count(&c[1]), what: filters(&c[2]) };
        let mut out = match c.get(5) {
            Some(per) => vec![Effect::Per { per: PieceFilter::parse(per.as_str()), then: vec![gather] }],
            None => vec![gather],
        };
        if let (Some(n), Some(w)) = (c.get(3), c.get(4)) {
            out.push(Effect::Gather { count: count(n.as_str()), what: filters(w.as_str()) });
        }
        return Some(out);
    }
    if let Some(c) = MOVE.captures(s) {
        return one(Effect::Move { count: count(&c[1]), what: filters(&c[2]) });
    }
    if let Some(c) = ADD.captures(s) {
        let mut out = Vec::new();
        for p in PIECE_N.captures_iter(&c[1]) {
            let n = count(&p[1]);
            out.push(match &p[2] {
                "Blight" => Effect::AddBlight(n),
                "Dahan" => Effect::AddDahan(n),
                "Presence" | "DestroyedPresenceNew" => Effect::AddPresence(n),
                w => match TokenKind::parse(w) {
                    Some(kind) => Effect::AddToken { kind, count: n },
                    None => Effect::Other(s.to_string()),
                },
            });
        }
        if !out.is_empty() {
            return Some(out);
        }
    }
    if s == "Remove {Blight}" || s == "remove it" {
        return one(Effect::RemoveBlight(1));
    }
    if let Some(c) = REMOVE.captures(s) {
        let n = count(&c[1]);
        let what = filters(&c[2]);
        return one(match what.as_slice() {
            [PieceFilter::Blight] => Effect::RemoveBlight(n),
            [PieceFilter::Token(kind)] => Effect::RemoveToken { kind: *kind, count: n },
            _ => Effect::Remove { count: n, what },
        });
    }
    if let Some(c) = REMOVE_HALF.captures(s) {
        return one(match filters(&c[1]).as_slice() {
            [PieceFilter::Token(kind)] => Effect::RemoveToken { kind: *kind, count: 128 },
            what => Effect::Remove { count: 128, what: what.to_vec() },
        });
    }
    if let Some(c) = ABDUCT.captures(s) {
        let what = if c[2].is_empty() { vec![PieceFilter::AnyInvader] } else { filters(&c[2]) };
        return one(Effect::Remove { count: if &c[1] == "it" { 1 } else { count(&c[1]) }, what });
    }
    if let Some(c) = DESTROY.captures(s) {
        return one(Effect::Destroy { count: count(&c[1]), what: filters(&c[2]) });
    }
    if let Some(c) = DROWN.captures(s) {
        let out: Vec<Effect> = PIECE_N.captures_iter(&c[1]).map(|p| Effect::Destroy { count: count(&p[1]), what: vec![PieceFilter::parse(&p[2])] }).collect();
        if !out.is_empty() {
            return Some(out);
        }
    }
    if let Some(c) = REPLACE.captures(s) {
        return one(Effect::Replace { count: count(&c[1]), from: PieceFilter::parse(&c[2]), to: PieceFilter::parse(&c[4]) });
    }
    if let Some(c) = DOWNGRADE.captures(s) {
        return one(Effect::Downgrade(count(&c[1])));
    }
    if let Some(c) = ENERGY.captures(s) {
        return one(Effect::GainEnergy(count(&c[1])));
    }
    if let Some(c) = CARD.captures(s) {
        let kind = c.get(1).map(|k| if k.as_str() == "Minor" { PowerKind::Minor } else { PowerKind::Major });
        return one(Effect::GainPowerCard { kind });
    }
    if let Some(c) = ELEMENT.captures(s) {
        return one(Effect::GainElement(c.get(1).or(c.get(2)).map(|m| count(m.as_str())).unwrap_or(1)));
    }
    if let Some(c) = RANGE.captures(s) {
        return one(Effect::RangeBonus(count(&c[1])));
    }
    None
}

fn damage_target(rest: &str) -> DamageTarget {
    let r = rest.to_ascii_lowercase();
    if r.contains("each {dahan}") {
        DamageTarget::EachDahan
    } else if r.contains("{dahan}") {
        DamageTarget::Dahan
    } else if r.contains("each invader") || r.contains("each added or moved invader") || r.contains("each of those invaders") || r.contains("to a different invader") {
        DamageTarget::EachInvader
    } else if r.contains("each {town}/{city}") {
        DamageTarget::EachTownCity
    } else if r.contains("{town}/{city}") {
        DamageTarget::TownCityOnly
    } else if r.contains("{explorer}/{town}") {
        DamageTarget::ExplorerTownOnly
    } else if r.contains("{town}") {
        DamageTarget::Town
    } else {
        DamageTarget::Invaders
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use si_model::island::PieceKind;

    fn p(lines: &[&str]) -> Vec<Effect> {
        parse(&lines.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }

    #[test]
    fn plain_sentences() {
        assert_eq!(p(&["1 {Fear}.", "Invaders skip Ravage Actions."]), vec![Effect::Fear(1), Effect::SkipRavage]);
        assert_eq!(p(&["Defend 3.", "During Ravage, {Dahan} in target land deal damage simultaneously with Invaders."]), vec![Effect::Defend(3), Effect::Other("During Ravage, {Dahan} in target land deal damage simultaneously with Invaders".into())]);
        assert_eq!(p(&["1 {Fear}.", "1 Damage to each Invader.", "1 Damage to each {Dahan}."]), vec![Effect::Fear(1), Effect::Damage { amount: 1, to: DamageTarget::EachInvader }, Effect::Damage { amount: 1, to: DamageTarget::EachDahan }]);
        assert_eq!(p(&["Push up to 3 {Explorer}/{Town}."]), vec![Effect::Push { count: 3, what: vec![PieceFilter::Piece(PieceKind::Explorer), PieceFilter::Piece(PieceKind::Town)] }]);
        assert_eq!(p(&["Add 1 {Badlands} and 1 {Wilds}."]), vec![Effect::AddToken { kind: TokenKind::Badlands, count: 1 }, Effect::AddToken { kind: TokenKind::Wilds, count: 1 }]);
        assert_eq!(p(&["4 Damage.", "Remove 1 {Blight}."]), vec![Effect::Damage { amount: 4, to: DamageTarget::Invaders }, Effect::RemoveBlight(1)]);
        assert_eq!(p(&["Replace 1 {Town} with 1 {Explorer}."]), vec![Effect::Replace { count: 1, from: PieceFilter::Piece(PieceKind::Town), to: PieceFilter::Piece(PieceKind::Explorer) }]);
        assert_eq!(p(&["Add 2 {Beast}."]), vec![Effect::AddToken { kind: TokenKind::Beast, count: 2 }]);
        assert_eq!(p(&["6 {Fear}.", "Destroy all Invaders."]), vec![Effect::Fear(6), Effect::Destroy { count: 255, what: vec![PieceFilter::AnyInvader] }]);
    }

    #[test]
    fn conditionals_choices_and_per() {
        let plague = p(&["1 {Fear} if Invaders are present.", "For each {Disease}, Push 2 {Explorer}/{Town}/{Dahan}.", "1 {Disease} may move with each Pushed piece."]);
        assert_eq!(plague.len(), 3);
        assert!(matches!(&plague[1], Effect::Per { per: PieceFilter::Token(TokenKind::Disease), then } if matches!(then[0], Effect::Push { count: 2, .. })));
        let voice = p(&["Push up to 4 {Dahan}.", "{Or}", "If Invaders are present, 2 {Fear}."]);
        assert!(matches!(&voice[0], Effect::Choice(b) if b.len() == 2 && b[1] == vec![Effect::FearIfInvaders(2)]));
        let tide = p(&["Remove up to half (round down) of {Beast} in target land.", "For each {Beast} Removed, 2 {Fear} and skip one Invader Action."]);
        assert_eq!(tide[0], Effect::RemoveToken { kind: TokenKind::Beast, count: 128 });
        assert!(matches!(&tide[1], Effect::Per { then, .. } if then == &vec![Effect::Fear(2), Effect::SkipAction(1)]));
        let floods = p(&["1 Damage.", "If target land is Coastal, +1 Damage."]);
        assert_eq!(floods[1], Effect::Conditional { condition: "target land is Coastal".into(), then: vec![Effect::DamageBonus(1)] });
        let pall = p(&["1 {Fear}. Up to 3 damaged Invaders do not participate in Ravage.", "{Or}", "1 {Fear}. Defend 1 per {Presence} you have in target land <i>(when this Power is used)</i>."]);
        assert!(matches!(&pall[0], Effect::Choice(b) if b[0] == vec![Effect::Fear(1), Effect::NoParticipation(3)] && matches!(b[1][1], Effect::Per { per: PieceFilter::Presence, .. })));
        let ambush = p(&["You may Gather 1 {Dahan}.", "Each {Dahan} destroys 1 {Explorer}."]);
        assert!(matches!(&ambush[1], Effect::Per { then, .. } if matches!(then[0], Effect::Destroy { count: 1, .. })));
        let guard = p(&["Gather up to 1 {Dahan}.", "Then, if {Dahan} are present, either:", "Defend 1 per {Dahan}.", "{Or}", "After Invaders are added or moved to target land, 1 Damage to each added or moved Invader."]);
        assert!(matches!(&guard[0], Effect::Choice(b) if b[0].len() == 2 && b[1].len() == 1), "the header sentence parses to nothing: {guard:?}");
        let terror = p(&["{TerrorI} : 1 {Fear}.", "{TerrorII} : 2 {Fear} or add 1 {Strife}."]);
        assert!(matches!(&terror[1], Effect::Conditional { condition, then } if condition == "TerrorII" && matches!(then[0], Effect::Choice(_))));
        let manifest = p(&["1 {Fear}.", "Each {Dahan} deals Damage equal to the number of your {Presence} in target land."]);
        assert!(matches!(&manifest[1], Effect::Per { per: PieceFilter::Piece(PieceKind::Dahan), .. }));
        let sea = p(&["If Invaders are present, 2 {Fear} per {Beast} (max. 6 {Fear})."]);
        assert!(matches!(&sea[0], Effect::Conditional { then, .. } if matches!(&then[0], Effect::Per { per: PieceFilter::Token(TokenKind::Beast), .. })));
    }

    #[test]
    fn markup_is_stripped() {
        assert_eq!(clean("Destroy 1 {Town}. <i>(This cannot trigger itself.)</i>"), "Destroy 1 {Town}.");
        assert_eq!(clean("If no {Dahan}/Invaders are present: <nobr>Remove 1 {Blight}.</nobr>"), "If no {Dahan}/Invaders are present: Remove 1 {Blight}.");
        assert_eq!(clean("Add 2 {Beast}, then 1 Damage per {Beast} (count max. 4 {Beast})."), "Add 2 {Beast}, then 1 Damage per {Beast}.");
    }
}
