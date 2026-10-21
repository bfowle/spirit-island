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

re!(TAGS, r"<i>.*?</i>|<[^>]+>|\{BR\}|\{Bullet\}|\{EndBullet\}|\((?:[Mm]ax|[Mm]in|[Cc]ount max)\.?[^)]*\)|\(\d+x/game\)");
re!(SPACES, r"\s+");
re!(LEADERS, r"^(?:Then,? |Then: |First, |Also, |Now: |Ongoing: |Bargain: |Instead: |Instead, |Otherwise, |If you do, )");
re!(FEAR, r"^\+?(\d+) \{Fear\}(?:, before the Terror Level check| there)?$");
re!(FEAR_IF_ANY, r"^(\d+) \{Fear\} if (.+)$");
re!(AFTER, r"^After (.+?)[,:] (.+)$");
re!(CHOOSE, r"^Choose (?:up to )?\d+ (?:different )?(?:Invaders|(?:\{\w+\}/?)+)$");
re!(FEAR_IF, r"^(\d+) \{Fear\} if Invaders are present(?: or adjacent)?$|^If Invaders are present, (\d+) \{Fear\}$");
re!(FEAR_PER, r"^\+?(\d+) \{Fear\} (?:for each|per) ((?:\{\w+\}/?)+|Invader|type of Invader present|\d+ \{\w+\})(?: in target land)?(?: and (?:for each|per) (?:of your )?(\{\w+\}))?");
re!(SPREAD, r"^Add (\d+) \{(\w+)\} among (.+)$");
re!(RAVAGE, r"^(?:Then,? )?Invaders in target land Ravage");
re!(TRIGGER_DESTROYED, r"^The next time an Invader is destroyed in target land this turn,? (.+)$");
re!(TRIGGER_AFTER_POWER, r"^Once this turn, after target Spirit uses a Power targeting a land, they may (.+)$");
re!(TRIGGER_FEAR, r"^When Powers generate \{Fear\} in target land, (.+)$");
re!(TRIGGER_NO_CASCADE, r"^When \{Blight\} is added to target land, it doesn't cascade$");
re!(CHOOSE_LAND, r"^Choose (one of (?:target|Target) Spirit's lands|a land where you and target Spirit both have \{Presence\}|one of your lands)$");
re!(TOKEN_CHOICE, r"^Choose a type of token from ((?:\{\w+\}/?)+) that exists in an adjacent land(?:; choosing \{Disease\} costs (\d+) Energy)?$");
re!(ADD_CHOSEN, r"^Add (\d+) of that type of token to target land$");
re!(ADD_EACH_ADJACENT, r"^(?:They |Target Spirit |You )?[Aa]dds? (\d+) \{(\w+)\} to each adjacent land$");
re!(REPEAT_TARGET, r"^(?:Once (?:this turn|per turn), )?[Tt]arget(?: Spirit)? may Repeat (?:a|one|\d+|up to \d+ other|the lowest-cost) Power Cards?");
re!(PLAY_ANOTHER, r"^(?:Target Spirit may (?:immediately )?|They may |You may )?[Pp]lay (?:another Power Card|a Power Card|it|the Major Power)(?: immediately)? by paying its cost");
re!(MAKE_FAST, r"^(?:This turn, )?[Tt]arget Spirit may (?:use (?:up to )?(\d+) \{Slow\} Powers? as if (?:they were|it were) \{Fast\}|make (\d+) Power which says .+ \{Fast\})");
re!(RECLAIM_N, r"^(?:Target Spirit may |You may )?Reclaims? (?:up to )?(\d+) (?:Power )?[Cc]ards?");
re!(RECLAIM_OR_REGAIN, r"^Target Spirit either Reclaims (\d+) Power Card or re-gains a Unique Power they previously forgot$");
re!(ENERGY_PER, r"^(?:Otherwise, )?(?:[Tt]arget Spirit |[Yy]ou )?gains? (\d+) Energy per (.+)$");
re!(ENERGY_EQUAL, r"gain Energy equal to (\{\w+\})");
re!(INVADER_RULE, r"^(?:During Ravage(?: actions)?,? .+|Invaders do not heal Damage at end of turn|\{Explorer\} fight alongside \{Dahan\}.*|\{Dahan\} in target land do not fight back|\{Dahan\} in target land cannot be changed|\{Blight\} cannot be added|This Power ignores Health bonuses|Do not destroy any \{Dahan\}|Also deal half of the Damage Invaders did to the land.*|In your lands, Invaders and \{Dahan\} have -\d+ Health.*|[Ii]f Invaders Ravage in target land, they Damage Invaders in adjacent lands.*|Before Pushing, \{Explorer\} and \{Town\}/\{City\} do damage to each other)$");
re!(DESTROY_PRESENCE, r"^(?:Destroy|Remove) (\d+) of your \{Presence\}(?: on the island from the game)?$");
re!(REMOVE_HEALTH, r"^Remove up to (\d+) Health worth of Invaders per \{Dahan\}$");
re!(INFIGHTING, r"^(?:Each|(\d+)) Invaders? with \{Strife\} deals? Damage to other Invaders(?: in target land)?$");
re!(DUEL, r"^(\d+) Invaders? and (\d+) \{Dahan\} deal Damage to each other$");
re!(NO_BLIGHT, r"^When \{Blight\} would be added to target land, instead leave it on the card$");
re!(BLIGHT_ADJ_IF_NONE, r"^In each land adjacent to the target, (.+) if it doesn't have any$");
re!(ADD_PRESENCE_ANY, r"[Aa]dds? (?:up to )?(\d+) (?:of (?:their|your) )?\{DestroyedPresenceNew\}");
re!(GATHER_ANOTHER, r"^(?:[Yy]ou may )?Gather another (\{\w+\})$");
re!(REMOVE_AN_INVADER, r"^(?:[Yy]ou may )?Remove an Invader(.*)$");
re!(REMOVE_ANOTHER_BLIGHT, r"^Remove another \{Blight\}$");
re!(MAY_SLOW, r"^This [Pp]ower may be \{Slow\}$");
re!(BUILD_THEN_RAVAGE, r"^they Build, then Ravage$");
re!(DESTROY_THEM, r"^Destroy them$");
re!(FOR_EACH_ELEMENTS, r"^For each ((?:\d+ )?\{\w+\}(?: (?:\d+ )?\{\w+\})*)(?: pair)? you have, (.+)$");
re!(IF_GATHER_DO_SO, r"^If you can Gather (\d+) \{Beast\}, do so, then (.+)$");
re!(VARIABLE, r"^X (Damage|\{Fear\})(?:, where X is (.+))?$");
re!(CHOOSE_PIECE, r"^(?:For each \{\w+\} present, )?choose a different Invader$|^Choose a Spirit with \{Presence\} in target land$|^Target Spirit may choose one of their \{SacredSite\}$|^Choose (?:any|one) land");
re!(RANGE_BONUS, r"\{\+(\d)Range\}");
re!(TIMING, r"^(?i)(at end of turn[^:,]*|during time passes|each \{slow\} phase|each slow phase|once per turn|when [^,:]+|after [^,:]+|before [^,:]+)[,:]?\s*(.*)$");
re!(ICON_ONLY, r"^(?:\{\w+\}(?:/\{\w+\})*\s*)+$");
re!(MARK_IT, r"^Mark it$");
re!(USE_ITS_TEXT, r"^Use its text effects on target land immediately");
re!(DAHAN_HEALTH, r"^\{Dahan\} have \+(\d+) Health(?: while in target land)?$");
re!(UNMODELED, r"^(?:Destroy the board containing target land|.*Add a random new Island Board|Swap the top card of the Invader Deck|Target land and a land adjacent to it become a single land|Until the end of the \{Slow\} phase, target land and everything in it cease to exist|On the board moved (?:from|to): .+|From now on, Build Cards .+|Turn the top card of the Fear Deck face-up|Discard the top card of the Minor Power Deck|Remove all pieces, then Add the pieces matching target land's Setup symbols|Mark both target land and another land .+|Mark it .+|Ignore its Setup icons.+)");
re!(FEAR_DECK_REMOVE, r"^Spirits may jointly spend (\d+) Energy per player .*to remove the top card of the Fear Deck from the game");
re!(DAMAGE, r"^(?:deal |do )?(\+)?(\d+) Damage(.*)$");
re!(DAMAGE_EQUAL, r"^Damage equal to");
re!(DOES_BONUS, r"^(?:Target Spirit |You |They )?(?:does|do|deal|deals) \+(\d+) Damage");
re!(DEFEND, r"^(?:In one of (?:t|T)arget Spirit's lands, )?Defend (\d+)(?: in target(?: land)? and all adjacent lands| there| in that land)?$");
re!(DEFEND_BONUS, r"^(?:Defend \+(\d+)|\+(\d+) Defend)$");
re!(DEFEND_PER, r"^(?:In one of (?:t|T)arget Spirit's lands, )?Defend (\d+) per (\{\w+\}|Invader)");
re!(DEFEND_GRANT, r"(?:grants|provides) Defend (\d+)");
re!(DEFEND_EACH_LAND, r"^Target Spirit provides Defend (\d+) in each of its lands\.?$");
re!(ADD_PRESENCE_RANGE, r"^(?:Target Spirit |You |They )?adds? (\d+) \{Presence\} (?:up to )?\{(\d+)Range\} from (?:their|your) \{Presence\}\.?$");
re!(PUSH, r"^(?:[Yy]ou may |Target Spirit may |may )?(?:Gather or )?Push (?:up to |another )?(\d+|all|any number of) (?:those |of (?:your|their) )?((?:(?:\{\w+\}|Invaders?)/?)+)(?: and (?:up to )?(\d+) ((?:(?:\{\w+\}|Invaders?)/?)+))?(?: per (\{\w+\}))?");
re!(PUSH_HALF, r"^Push up to half \(round (?:up|down)\) of (\{\w+\}|Invaders)");
re!(PUSH_IT, r"^(?:You may )?Push (?:that|it|them|the) ?(\{\w+\})?(?: you added \{\w+\} to)?$");
re!(GATHER, r"^(?:[Yy]ou may |Target Spirit )?Gathers? (?:up to )?(\d+|all) (?:of (?:your|their) )?((?:(?:\{\w+\}|Invaders?)/?)+)(?: and (?:up to )?(\d+) ((?:(?:\{\w+\}|Invaders?)/?)+))?(?: per (\{\w+\}))?");
re!(MOVE, r"^(?:You may )?Move (?:up to )?(\d+) (?:of (?:your|their|target Spirit's) )?((?:(?:\{\w+\}|Invaders?)/?)+)");
re!(ADD, r"^(?:Target Spirit |They |You )?(?:Add|add)s? ((?:(?:up to )?\d+ (?:additional )?\{\w+\}(?:, | and |, and )?)+)");
re!(PIECE_N, r"(\d+) (?:additional )?\{(\w+)\}");
re!(REMOVE, r"^(?:Remove|remove) (?:up to )?(\d+|all|any number of) (?:Damaged )?((?:(?:\{\w+\}|Invaders?)/?)+)");
re!(REMOVE_HALF, r"^Remove up to half \(round (?:up|down)\) of (\{\w+\}|Invaders)");
re!(ABDUCT, r"^Abduct (?:up to )?(\d+|it) ?((?:(?:\{\w+\}|Invaders?)/?)*)");
re!(DESTROY, r"^(?:Destroy|destroy) (?:up to )?(all|\d+) ((?:(?:\{\w+\}|Invaders?)/?)+)(?: per ((?:\{\w+\}/?)+))?");
re!(DROWN, r"^Drown (.+)$");
re!(REPLACE, r"^(?:[Yy]ou may )?(?:Replace|replace) (\d+|each|all) (?:additional |of (?:their|your) )?\{(\w+)\} with (?:(\d+) )?\{(\w+)\}");
re!(ENERGY, r"^(?:[Tt]arget Spirit |they |you |They |You )?(?:also )?(?:gains?|Gain) \+?(\d+) Energy$");
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
re!(FOR_EACH, r"^For (?:each|every) (\{\w+\}(?:/\{\w+\})?|Invader|type of token you added|token removed|of your \{SacredSite\}|adjacent land with your \{Presence\})( in target land| present| Removed| you have| in/adjacent to target land| in or adjacent to target land| in adjacent lands| past the first| Pushed by this Power| on your “Deep Slumber” track)?,? (.+)$");
re!(EACH_DOES, r"^Each (?:remaining )?(\{\w+\}|Invader) (does|deals|takes|destroys) (.+)$");
re!(IF_COMMA, r"^[Ii]f (.+?), (.+)$");
re!(IF_COLON, r"^If (.+?): ?(.+)$");
re!(OTHERWISE, r"^Otherwise,? ?:? ?(.+)$");
re!(INSTEAD, r"^[Ii]nstead,? (.+)$");
re!(MAY_INSTEAD, r"^(?:You|you|They|they|Target Spirit|target Spirit) may instead (.+)$");
re!(PAY, r"^(?:You may |Target Spirit may |Spirits may )?[Pp]ay (\d+) Energy to (?:instead )?(.+)$");
re!(AREA, r"^In (each land within \{\d+Range\}|one land within \{\d+Range\}|any number of lands with \{\w+\}|every land in the game|a land with \{Blight\} where you have \{Presence\}|each land where you removed \{\w+\}|your lands|each adjacent land|both origin land and target land|up to \d+ adjacent lands(?: with \{\w+\})?|a land adjacent to both origin and target|each other Coastal land on the same board|lands where you did Damage|that land and each adjacent land|one of (?:target|Target) Spirit's lands|one of your lands|each target land|that land|lands you Pushed \{Dahan\} to|any \d+ lands with \d+ or more Invaders|any \d+ lands on the island|both target and one adjacent land|a single land within \{\d+Range\} of your \{Presence\})[,:]? ?(.+)$");
re!(SOMEONE_MAY, r"^(?:Target Spirit|They|You|you) (?:may |also |now |may also )(.+)$");

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
    let mut depth = 0i32;
    for i in 0..chars.len() {
        match chars[i] {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if depth <= 0 && chars[i] == '.' && i + 2 < chars.len() && chars[i + 1] == ' ' && (chars[i + 2].is_ascii_uppercase() || matches!(chars[i + 2], '{' | '+' | '(') || chars[i + 2].is_ascii_digit()) {
            out.push(chars[start..=i].iter().collect::<String>());
            start = i + 2;
        }
    }
    out.push(chars[start..].iter().collect::<String>());
    // a parenthetical at the end of a sentence is a clarification of its own
    let mut split: Vec<String> = Vec::new();
    for s in out {
        let s = s.trim().trim_end_matches('.').trim().to_string();
        if let Some(i) = s.rfind(" (") {
            if s.ends_with(')') && !s.starts_with('(') {
                split.push(s[..i].trim().trim_end_matches('.').to_string());
                split.push(s[i + 1..].to_string());
                continue;
            }
        }
        split.push(s);
    }
    split.into_iter().filter(|s| !s.is_empty() && *s != "{Or}").collect()
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
    if let Some(e) = fallback(&s) {
        return vec![e];
    }
    vec![Effect::Other(s)]
}

/// The last resort: a typed kind from what the sentence talks about. Each kind
/// says what it is (Spirit economy, a special token, timing, movement, a deck,
/// a distribution, rules text); none of them hides board effects.
fn fallback(s: &str) -> Option<Effect> {
    let t = s.trim();
    let lower = t.to_ascii_lowercase();
    if t.starts_with('(') || (t.ends_with(')') && !t.contains('(')) || ICON_ONLY.is_match(t) || t == "If the Terror Level is" {
        return Some(Effect::Clarification(t.to_string()));
    }
    for tok in ["{Quake}", "{Impending}", "{MarkedBeast}", "{BreathOfDarknessDownYourSpine}", "{TheEndlessDark}", "{WanderingVoiceKeensDelirium}", "{Vitality}", "Absorbed", "Empower"] {
        if t.contains(tok) {
            return Some(Effect::SpecialToken { token: tok.trim_matches(|c| c == '{' || c == '}').to_string(), text: t.to_string() });
        }
    }
    if let Some(c) = TIMING.captures(t) {
        return Some(Effect::Timing { when: c[1].trim().to_string(), text: c[2].trim().to_string() });
    }
    let has = |ws: &[&str]| ws.iter().any(|w| lower.contains(w));
    if has(&["energy", "power card", "element", "reclaim", "forget", "repeat", "major power", "minor power", "presence track", "{presence} track", "play it", "play a power", "resolve 1 {slow}", "gift each other", "use each other's", "take a minor power", "unique power", "they draw", "did not keep", "do likewise"]) {
        return Some(Effect::SpiritEffect(t.to_string()));
    }
    if has(&["deck", "discard pile", "face-up", "face-down", "look at the top card", "shuffle"]) {
        return Some(Effect::DeckEffect(t.to_string()));
    }
    if has(&["split", "however desired", "may be dealt in adjacent", "divide", "more within"]) {
        return Some(Effect::Distribution(t.to_string()));
    }
    if has(&["move", "push", "gather", "bring along", "brings", "drowning", "return up to"]) {
        return Some(Effect::Movement(t.to_string()));
    }
    if has(&["ravage", "health", "damage invaders did"]) {
        return Some(Effect::InvaderRule(t.to_string()));
    }
    if has(&["cannot", "can't", "can target", "ignoring", "do not affect", "does not", "don't", "may count as", "ignores", "removed from the game", "leaves the game", "is not considered", "treat all", "must be used", "elemental thresholds", "the cost is", "cost limit", "may not", "no effect", "not by adding"]) {
        return Some(Effect::Clarification(t.to_string()));
    }
    None
}

fn count(s: &str) -> u8 {
    match s {
        "all" | "any number of" | "each" => 255,
        "an" | "a" | "another" => 1,
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
    if lower.contains("may move with each pushed piece") {
        return one(Effect::Move { count: 1, what: vec![PieceFilter::Token(TokenKind::Disease)] });
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
    // --- board effects with their own shapes ------------------------------
    if let Some(c) = DESTROY_PRESENCE.captures(s) {
        return one(Effect::DestroyPresence(count(&c[1])));
    }
    if let Some(c) = REMOVE_HEALTH.captures(s) {
        return one(Effect::Per { per: PieceFilter::Piece(si_model::island::PieceKind::Dahan), then: vec![Effect::RemoveHealth(count(&c[1]))] });
    }
    if let Some(c) = INFIGHTING.captures(s) {
        return one(Effect::Infighting(c.get(1).map(|m| count(m.as_str())).unwrap_or(255)));
    }
    if let Some(c) = DUEL.captures(s) {
        return one(Effect::Duel { invaders: count(&c[1]), dahan: count(&c[2]) });
    }
    if NO_BLIGHT.is_match(s) {
        return one(Effect::NoBlight);
    }
    if let Some(c) = BLIGHT_ADJ_IF_NONE.captures(s) {
        let then = parse_sentence(&c[1]);
        if typed(&then) {
            return one(Effect::Area { scope: "each adjacent land".into(), then: vec![Effect::Conditional { condition: "the land has no {blight}".into(), then }] });
        }
    }
    if let Some(c) = GATHER_ANOTHER.captures(s) {
        return one(Effect::Gather { count: 1, what: vec![PieceFilter::parse(&c[1])] });
    }
    if REMOVE_AN_INVADER.is_match(s) {
        return one(Effect::Remove { count: 1, what: vec![PieceFilter::AnyInvader] });
    }
    if REMOVE_ANOTHER_BLIGHT.is_match(s) {
        return one(Effect::RemoveBlight(1));
    }
    if MAY_SLOW.is_match(s) {
        return one(Effect::MaySlow);
    }
    if BUILD_THEN_RAVAGE.is_match(s) {
        return Some(vec![Effect::Build, Effect::Ravage]);
    }
    if DESTROY_THEM.is_match(s) {
        return one(Effect::Destroy { count: 255, what: vec![PieceFilter::AnyInvader, PieceFilter::Piece(si_model::island::PieceKind::Dahan)] });
    }
    if let Some(c) = FOR_EACH_ELEMENTS.captures(s) {
        let then = parse_sentence(&c[2]);
        if typed(&then) {
            return one(Effect::Per { per: PieceFilter::Other(format!("elements: {}", &c[1])), then });
        }
    }
    if let Some(c) = IF_GATHER_DO_SO.captures(s) {
        let mut then = vec![Effect::Gather { count: count(&c[1]), what: vec![PieceFilter::Token(TokenKind::Beast)] }];
        let rest = parse_sentence(&c[2]);
        if typed(&rest) {
            then.extend(rest);
            return one(Effect::Conditional { condition: "a {beast} can be gathered".into(), then });
        }
    }
    if let Some(c) = VARIABLE.captures(s) {
        return one(Effect::Variable { what: c[1].replace(['{', '}'], ""), by: c.get(2).map(|m| m.as_str().to_string()).unwrap_or_else(|| "X as the card defines it".into()) });
    }
    if CHOOSE_PIECE.is_match(s) {
        return one(Effect::Choose(s.to_string()));
    }
    if MARK_IT.is_match(s) {
        return one(Effect::SpecialToken { token: "MarkedBeast".into(), text: s.to_string() });
    }
    if USE_ITS_TEXT.is_match(s) {
        return one(Effect::Unmodeled(s.to_string()));
    }
    if let Some(c) = ADD_PRESENCE_RANGE.captures(s) {
        return one(Effect::AddPresence { count: count(&c[1]), range: Some(count(&c[2])) });
    }
    if let Some(c) = ADD_PRESENCE_ANY.captures(s) {
        return one(Effect::AddPresence { count: count(&c[1]), range: None });
    }
    // --- triggers, choices, spirit economy, rules, out-of-reach ----------
    if let Some(c) = TRIGGER_DESTROYED.captures(s) {
        let then = parse_sentence(&c[1]);
        if typed(&then) {
            return one(Effect::Trigger { when: "an Invader is destroyed in target land".into(), then, once: true });
        }
    }
    if let Some(c) = TRIGGER_AFTER_POWER.captures(s) {
        let then = parse_sentence(&c[1]);
        if typed(&then) {
            return one(Effect::Trigger { when: "after target Spirit uses a Power targeting a land".into(), then, once: true });
        }
    }
    if let Some(c) = TRIGGER_FEAR.captures(s) {
        let then = parse_sentence(&c[1]);
        if typed(&then) {
            return one(Effect::Trigger { when: "Powers generate {Fear} in target land".into(), then, once: false });
        }
    }
    if TRIGGER_NO_CASCADE.is_match(s) {
        return one(Effect::Trigger { when: "{Blight} is added to target land".into(), then: vec![Effect::NoCascade], once: false });
    }
    if let Some(c) = CHOOSE_LAND.captures(s) {
        return one(Effect::ChooseLand(c[1].to_string()));
    }
    if let Some(c) = TOKEN_CHOICE.captures(s) {
        let options: Vec<TokenKind> = c[1].split('/').filter_map(|w| match PieceFilter::parse(w) { PieceFilter::Token(k) => Some(k), _ => None }).collect();
        return one(Effect::TokenChoice { options, disease_cost: c.get(2).map(|m| count(m.as_str())).unwrap_or(0) });
    }
    if let Some(c) = ADD_CHOSEN.captures(s) {
        return one(Effect::AddChosenToken(count(&c[1])));
    }
    if let Some(c) = ADD_EACH_ADJACENT.captures(s) {
        let n = count(&c[1]);
        let inner = match &c[2] {
            "Presence" | "DestroyedPresenceNew" => Effect::AddPresence { count: n, range: None },
            "Dahan" => Effect::AddDahan(n),
            "Blight" => Effect::AddBlight(n),
            w => match TokenKind::parse(w) {
                Some(k) => Effect::AddToken { kind: k, count: n },
                None => Effect::Other(s.to_string()),
            },
        };
        return one(Effect::Area { scope: "each adjacent land".into(), then: vec![inner] });
    }
    if REPEAT_TARGET.is_match(s) {
        return one(Effect::Repeat);
    }
    if PLAY_ANOTHER.is_match(s) {
        return one(Effect::PlayAnother);
    }
    if let Some(c) = MAKE_FAST.captures(s) {
        return one(Effect::MakeFast(c.get(1).or(c.get(2)).map(|m| count(m.as_str())).unwrap_or(1)));
    }
    if let Some(c) = RECLAIM_OR_REGAIN.captures(s) {
        return one(Effect::Choice(vec![vec![Effect::Reclaim(count(&c[1]))], vec![Effect::RegainForgotten]]));
    }
    if let Some(c) = RECLAIM_N.captures(s) {
        return one(Effect::Reclaim(count(&c[1])));
    }
    if let Some(c) = ENERGY_PER.captures(s) {
        return one(Effect::EnergyPer { n: count(&c[1]), per: c[2].trim_end_matches('.').to_string() });
    }
    if let Some(c) = ENERGY_EQUAL.captures(s) {
        return one(Effect::EnergyPer { n: 1, per: c[1].to_string() });
    }
    if let Some(c) = DAHAN_HEALTH.captures(s) {
        return one(Effect::DahanHealth(count(&c[1])));
    }
    if INVADER_RULE.is_match(s) {
        return one(Effect::InvaderRule(s.to_string()));
    }
    if UNMODELED.is_match(s) {
        return one(Effect::Unmodeled(s.to_string()));
    }
    if let Some(c) = RANGE_BONUS.captures(s) {
        if s.len() < 120 {
            return one(Effect::RangeBonus(count(&c[1])));
        }
    }
    // --- structure ------------------------------------------------------
    if let Some(c) = TERROR.captures(s) {
        let then = parse_sentence(&c[2]);
        if typed(&then) {
            return one(Effect::Conditional { condition: c[1].to_string(), then });
        }
    }
    if let Some(c) = FOR_EACH.captures(s) {
        let then = parse_sentence(&c[3]);
        if typed(&then) {
            let suffix = c.get(2).map(|m| m.as_str().trim()).unwrap_or("");
            let per = match suffix {
                "in or adjacent to target land" | "in adjacent lands" | "past the first" | "Pushed by this Power" => PieceFilter::Other(format!("{} {}", &c[1], suffix)),
                _ => PieceFilter::parse(c[1].split(' ').next().unwrap()),
            };
            return one(Effect::Per { per, then });
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
            return one(Effect::Area { scope: c[1].to_string(), then });
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
        let n = count(&c[1]);
        let mut out: Vec<Effect> = c[2].split('/').map(|w| Effect::Per { per: PieceFilter::parse(w.split(' ').last().unwrap()), then: vec![Effect::Fear(n)] }).collect();
        if let Some(extra) = c.get(3) {
            out.push(Effect::Per { per: PieceFilter::parse(extra.as_str()), then: vec![Effect::Fear(n)] });
        }
        return Some(out);
    }
    if RAVAGE.is_match(s) {
        return one(Effect::Ravage);
    }
    if let Some(c) = FEAR_DECK_REMOVE.captures(s) {
        return one(Effect::FearDeckRemove { energy_per_player: count(&c[1]) });
    }
    if let Some(c) = SPREAD.captures(s) {
        if let PieceFilter::Token(kind) = PieceFilter::parse(&format!("{{{}}}", &c[2])) {
            return one(Effect::Spread { kind, count: count(&c[1]), scope: c[3].trim_end_matches('.').to_string() });
        }
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
    if let Some(c) = DEFEND_EACH_LAND.captures(s) {
        // a Spirit-target card: Defend in every land where that Spirit has presence
        return one(Effect::Area { scope: "each land where target Spirit has presence".into(), then: vec![Effect::Defend(count(&c[1]))] });
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
                "Presence" | "DestroyedPresenceNew" => Effect::AddPresence { count: n, range: None },
                w => match TokenKind::parse(w) {
                    Some(kind) => Effect::AddToken { kind, count: n },
                    None => Effect::SpecialToken { token: w.to_string(), text: s.to_string() },
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
        let mut out = vec![match what.as_slice() {
            [PieceFilter::Blight] => Effect::RemoveBlight(n),
            [PieceFilter::Token(kind)] => Effect::RemoveToken { kind: *kind, count: n },
            _ => Effect::Remove { count: n, what },
        }];
        // "Remove 1 City, 1 Town, and 1 Explorer": the list after the first item
        let rest = &s[c.get(0).unwrap().end()..];
        for p in PIECE_N.captures_iter(rest) {
            out.push(Effect::Remove { count: count(&p[1]), what: vec![PieceFilter::parse(&p[2])] });
        }
        return Some(out);
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
        let d = Effect::Destroy { count: count(&c[1]), what: filters(&c[2]) };
        return one(match c.get(3) {
            Some(per) => Effect::Per { per: PieceFilter::Other(per.as_str().to_string()), then: vec![d] },
            None => d,
        });
    }
    if let Some(c) = DROWN.captures(s) {
        let out: Vec<Effect> = PIECE_N.captures_iter(&c[1]).map(|p| Effect::Destroy { count: count(&p[1]), what: vec![PieceFilter::parse(&p[2])] }).collect();
        if !out.is_empty() {
            return Some(out);
        }
    }
    if let Some(c) = REPLACE.captures(s) {
        let n = count(&c[1]);
        let to = PieceFilter::parse(&c[4]);
        let mut out = vec![Effect::Replace { count: n, from: PieceFilter::parse(&c[2]), to: to.clone() }];
        if let (Some(m), PieceFilter::Piece(kind)) = (c.get(3), &to) {
            let total = count(m.as_str());
            if total > n && n != 255 {
                out.push(Effect::AddInvaders { kind: *kind, count: total - n });
            }
        }
        return Some(out);
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
        assert_eq!(p(&["Defend 3.", "During Ravage, {Dahan} in target land deal damage simultaneously with Invaders."]), vec![Effect::Defend(3), Effect::InvaderRule("During Ravage, {Dahan} in target land deal damage simultaneously with Invaders".into())]);
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
        assert!(matches!(&manifest[1], Effect::Per { per: PieceFilter::Piece(si_model::island::PieceKind::Dahan), .. }));
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
