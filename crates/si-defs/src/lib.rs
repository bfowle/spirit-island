//! Definitions → model. The input is the digital edition's definition tables
//! (one JSON object per identifier, as `si-steam-dump defs` writes them and as
//! `si-steam::engine_defs` holds them in memory). The output is `si-model`
//! types. Nothing here is inferred from prose except where a function says so.

pub mod effects;
pub mod hooks;
pub mod text;

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

use si_model::decks::{BlightCard, EventCard, EventSection, FearCard, InvaderCard, InvaderTarget};
use si_model::island::{BoardDef, InitialPieces, LandDef, LandId, Terrain};
use si_model::power::{PowerCard, PowerKind, Range, Speed, TargetKind, Threshold};
use si_model::setup::{Adversary, AdversaryLevel, Scenario};
use si_model::spirit::{Aspect, GrowthGroup, GrowthOptionDef, GrowthRules, Innate, SpecialRule, SpiritPanel, Tier, Track, TrackNode};
use si_model::{Element, ElementSet};

#[derive(Debug, Default, Clone)]
pub struct Definitions {
    pub spirits: BTreeMap<String, SpiritPanel>,
    pub cards: BTreeMap<String, PowerCard>,
    pub boards: BTreeMap<String, BoardDef>,
    pub invader_cards: BTreeMap<String, InvaderCard>,
    pub adversaries: BTreeMap<String, Adversary>,
    pub fear_cards: BTreeMap<String, FearCard>,
    pub event_cards: BTreeMap<String, EventCard>,
    pub blight_cards: BTreeMap<String, BlightCard>,
    pub scenarios: BTreeMap<String, Scenario>,
    /// Where the tables came from (a directory, or an engine build hash).
    pub source: String,
}

pub const TABLES: [&str; 9] = ["spirits", "power_cards", "boards", "invader_cards", "adversaries", "fear_cards", "event_cards", "blight_cards", "scenarios"];

impl Definitions {
    /// Read `<dir>/<table>.json` for each table (missing tables are empty).
    pub fn from_dir(dir: &Path) -> Result<Self> {
        let mut tables: BTreeMap<String, Value> = BTreeMap::new();
        for t in TABLES {
            let p = dir.join(format!("{t}.json"));
            if p.exists() {
                let text = std::fs::read_to_string(&p).with_context(|| format!("reading {}", p.display()))?;
                tables.insert(t.to_string(), serde_json::from_str(&text).with_context(|| format!("parsing {}", p.display()))?);
            }
        }
        let mut d = Self::from_tables(&tables)?;
        d.source = dir.display().to_string();
        Ok(d)
    }

    /// Build from in-memory tables: each value is an object keyed by identifier
    /// (or an array of objects with an `identifier` field).
    pub fn from_tables(tables: &BTreeMap<String, Value>) -> Result<Self> {
        let mut d = Definitions::default();
        let items = |name: &str| -> Vec<Value> {
            match tables.get(name) {
                Some(Value::Object(m)) => m.values().cloned().collect(),
                Some(Value::Array(a)) => a.clone(),
                _ => vec![],
            }
        };
        for v in items("spirits") {
            let s = spirit(&v)?;
            d.spirits.insert(s.id.clone(), s);
        }
        for v in items("power_cards") {
            let c = power_card(&v)?;
            d.cards.insert(c.id.clone(), c);
        }
        for v in items("boards") {
            let b = board(&v)?;
            d.boards.insert(b.id.clone(), b);
        }
        for v in items("invader_cards") {
            let c = invader_card(&v)?;
            d.invader_cards.insert(c.id.clone(), c);
        }
        for v in items("adversaries") {
            let a = adversary(&v)?;
            d.adversaries.insert(a.id.clone(), a);
        }
        for v in items("fear_cards") {
            let c = FearCard { id: id(&v)?, title: str_of(&v, "title"), body: strings(&v, "body"), expansion: str_of(&v, "expansion") };
            d.fear_cards.insert(c.id.clone(), c);
        }
        for v in items("event_cards") {
            let sections = v
                .get("eventSections")
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .map(|s| EventSection { title: str_of(s, "eventSectionTitle"), body: strings(s, "eventSectionBody"), kinds: strings(s, "eventSectionTypes") })
                        .collect()
                })
                .unwrap_or_default();
            let c = EventCard { id: id(&v)?, sections, expansion: str_of(&v, "expansion") };
            d.event_cards.insert(c.id.clone(), c);
        }
        for v in items("blight_cards") {
            let c = BlightCard {
                id: id(&v)?,
                title: str_of(&v, "title"),
                healthy_per_board: num(&v, "healthyIslandNumberPerStartingBoard") as u8,
                flipped_per_board: num(&v, "flippedNumberPerStartingBoard") as u8,
                flipped_effect_title: str_of(&v, "flippedEffectTitle"),
                flipped_effect: str_of(&v, "flippedEffectBody"),
                expansion: str_of(&v, "expansion"),
            };
            d.blight_cards.insert(c.id.clone(), c);
        }
        for v in items("scenarios") {
            let s = Scenario { id: id(&v)?, title: str_of(&v, "title"), difficulty: num(&v, "difficulty") as i8, expansion: str_of(&v, "expansion") };
            d.scenarios.insert(s.id.clone(), s);
        }
        Ok(d)
    }

    pub fn spirit_by_title(&self, title: &str) -> Option<&SpiritPanel> {
        let t = title.trim().to_ascii_lowercase();
        self.spirits.values().find(|s| s.title.to_ascii_lowercase() == t || s.id.to_ascii_lowercase() == t)
    }

    /// Match a spirit by engine id, title or short title (`Volcano`).
    pub fn spirit_by_any_name(&self, name: &str) -> Option<&SpiritPanel> {
        let t = name.trim().to_ascii_lowercase();
        self.spirits.values().find(|s| s.id.to_ascii_lowercase() == t || s.title.to_ascii_lowercase() == t || s.short_title.to_ascii_lowercase() == t)
    }

    /// The spirit whose unique card or innate this power id belongs to.
    pub fn owner_of_power(&self, power_id: &str) -> Option<&SpiritPanel> {
        self.spirits.values().find(|s| s.uniques.iter().any(|u| u == power_id) || s.innates.iter().any(|i| i.id == power_id))
    }

    /// A board by the short code the log uses (`D`, `NW`), or by its full id.
    pub fn board_def(&self, code: &str) -> Option<&BoardDef> {
        self.boards.get(code).or_else(|| self.boards.values().find(|b| LandId::board_code(&b.id) == code))
    }

    pub fn card_by_title(&self, title: &str) -> Option<&PowerCard> {
        let t = title.trim().to_ascii_lowercase();
        self.cards.values().find(|c| c.title.to_ascii_lowercase() == t)
    }
}

// ---- helpers ---------------------------------------------------------------

fn id(v: &Value) -> Result<String> {
    v.get("identifier").and_then(Value::as_str).map(str::to_string).context("definition without identifier")
}

fn str_of(v: &Value, key: &str) -> String {
    match v.get(key) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(a)) => a.iter().filter_map(Value::as_str).collect::<Vec<_>>().join("\n"),
        _ => String::new(),
    }
}

fn strings(v: &Value, key: &str) -> Vec<String> {
    match v.get(key) {
        Some(Value::Array(a)) => a.iter().filter_map(Value::as_str).map(str::to_string).collect(),
        Some(Value::String(s)) => vec![s.clone()],
        _ => vec![],
    }
}

fn num(v: &Value, key: &str) -> i64 {
    v.get(key).and_then(Value::as_i64).unwrap_or(0)
}

fn opt_num(v: &Value, key: &str) -> Option<i64> {
    v.get(key).and_then(Value::as_i64)
}

fn flag(v: &Value, key: &str) -> bool {
    v.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn elements_of(v: &Value, key: &str) -> Vec<Element> {
    strings(v, key).iter().filter_map(|s| Element::parse(s)).collect()
}

/// `{Sun}{Sun}{Fire}` or `costSun: 2` style element requirements.
fn element_set_from_costs(v: &Value) -> ElementSet {
    let mut set = ElementSet::new();
    if let Some(m) = v.as_object() {
        for (k, val) in m {
            if let Some(e) = k.strip_prefix("cost").and_then(Element::parse) {
                set.add(e, val.as_i64().unwrap_or(0).clamp(0, 9) as u8);
            }
        }
    }
    set
}

fn element_set_from_tokens(text: &str) -> ElementSet {
    let mut set = ElementSet::new();
    let mut rest = text;
    while let Some(i) = rest.find('{') {
        let Some(j) = rest[i..].find('}') else { break };
        if let Some(e) = Element::parse(&rest[i + 1..i + j]) {
            set.add(e, 1);
        }
        rest = &rest[i + j + 1..];
    }
    set
}

fn range_of(v: &Value) -> Option<Range> {
    let distance = opt_num(v, "rangeDistance")?;
    Some(Range {
        distance: distance.clamp(0, 9) as u8,
        from_sacred_site: flag(v, "rangeSacredSite") || flag(v, "rangedSacredSite"),
        origin_terrains: strings(v, "rangeLandTypes").iter().filter_map(|s| Terrain::parse(s)).collect(),
    })
}

fn speed_of(v: &Value) -> Speed {
    if flag(v, "speed") { Speed::Fast } else { Speed::Slow }
}

// ---- spirits ---------------------------------------------------------------

fn growth_option(v: &Value) -> GrowthOptionDef {
    let mut elements = elements_of(v, "elementTypes");
    if let Some(e) = v.get("elementType").and_then(Value::as_str).and_then(Element::parse) {
        elements.push(e);
    }
    GrowthOptionDef {
        kind: str_of(v, "growthType"),
        basic: v.get("basicGrowthType").and_then(Value::as_str).map(str::to_string),
        number: opt_num(v, "number").or_else(|| opt_num(v, "energyCost")),
        elements,
        optional: flag(v, "optional"),
        repeats: opt_num(v, "repeats").map(|n| n as u8),
        choices: v.get("growthOptionChoices").and_then(Value::as_array).map(|a| a.iter().map(growth_option).collect()).unwrap_or_default(),
    }
}

fn track_node(v: &Value) -> TrackNode {
    let id = str_of(v, "identifier");
    let kind = str_of(v, "trackNodeType");
    let track = if id.contains("Energy") {
        Track::Energy
    } else if id.contains("CardPlays") {
        Track::Plays
    } else {
        Track::Other
    };
    let number = opt_num(v, "number").map(|n| n.clamp(0, 99) as u8);
    let plus = kind.starts_with("Plus");
    let mut n = TrackNode { id, kind: kind.clone(), track, plus, requires: strings(v, "requiredNodeIdentifiers"), next: strings(v, "nextNodeIdentifiers"), ..Default::default() };
    if kind.contains("EnergyPerTurn") {
        n.energy = number;
        n.impending = kind.contains("Impending");
    }
    if kind.contains("CardPlaysPerTurn") {
        n.plays = number;
        n.impending = n.impending || kind.contains("Impending");
    }
    if kind.contains("Element") {
        n.elements = elements_of(v, "elementTypes");
        n.permanent_element = kind.contains("PermanentElement");
    }
    if kind.contains("ReclaimOne") {
        n.reclaim_one = true;
    }
    if kind == "ReclaimCards" {
        match number {
            Some(1) => n.reclaim_one = true,
            other => n.reclaim = other,
        }
    }
    if kind.contains("PayEnergyToGainPowerCard") {
        n.pay_to_gain = v.get("growthOption").and_then(|g| g.get("energyCost")).and_then(Value::as_i64).map(|c| c as u8).or(Some(2));
    }
    if kind.contains("GrowthOption") {
        n.growth_option = v.get("growthOption").map(growth_option);
    }
    n
}

fn innate(v: &Value) -> Innate {
    let tiers = v
        .get("elementOptions")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .map(|t| {
                    let body = str_of(t, "body");
                    Tier { elements: element_set_from_costs(t), effects: effects::parse(&[body.clone()]), body, modifier: flag(t, "requiresAnotherElementOption") }
                })
                .collect()
        })
        .unwrap_or_default();
    Innate {
        id: str_of(v, "identifier"),
        title: str_of(v, "title"),
        speed: speed_of(v),
        range: range_of(v),
        target: TargetKind(str_of(v, "targetType")),
        tiers,
        rider: strings(v, "body"),
    }
}

fn special_rule(spirit: &str, v: &Value) -> SpecialRule {
    let title = str_of(v, "title");
    let id = str_of(v, "identifier");
    SpecialRule { hooks: hooks::for_rule(spirit, &id, &title), id, title, body: strings(v, "body"), requires_activation: flag(v, "requiresActivation") }
}

fn spirit(v: &Value) -> Result<SpiritPanel> {
    let id = id(v)?;
    let tracks = v.get("presenceTrackNodes").and_then(Value::as_array).map(|a| a.iter().map(track_node).collect()).unwrap_or_default();
    let groups = v
        .get("growthGroups")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .map(|g| GrowthGroup {
                    number: num(g, "groupNumber") as u8,
                    options: g.get("growthOptions").and_then(Value::as_array).map(|o| o.iter().map(growth_option).collect()).unwrap_or_default(),
                })
                .collect()
        })
        .unwrap_or_default();
    let aspects = v
        .get("aspects")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .map(|x| Aspect {
                    id: str_of(x, "identifier"),
                    replaces: strings(x, "replacedSpecialRuleIdentifiers"),
                    special_rules: x.get("specialRules").and_then(Value::as_array).map(|r| r.iter().map(|r| special_rule(&id, r)).collect()).unwrap_or_default(),
                    expansion: str_of(x, "expansion"),
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(SpiritPanel {
        title: str_of(v, "title"),
        short_title: str_of(v, "shortTitle"),
        expansion: str_of(v, "expansion"),
        complexity: str_of(v, "complexity"),
        setup: str_of(v, "setup"),
        tracks,
        growth: GrowthRules { groups, pick: num(v, "numberOfGrowthGroupsToUse").max(1) as u8 },
        innates: v.get("innatePowers").and_then(Value::as_array).map(|a| a.iter().map(innate).collect()).unwrap_or_default(),
        special_rules: v.get("specialRules").and_then(Value::as_array).map(|a| a.iter().map(|r| special_rule(&id, r)).collect()).unwrap_or_default(),
        uniques: strings(v, "uniquePowerCardIdentifiers"),
        aspects,
        id,
    })
}

// ---- cards, boards, decks, adversaries -------------------------------------

fn power_card(v: &Value) -> Result<PowerCard> {
    let kind = match str_of(v, "powerType").as_str() {
        "Minor" => PowerKind::Minor,
        "Major" => PowerKind::Major,
        _ => PowerKind::Unique,
    };
    let mut elements = ElementSet::new();
    for e in elements_of(v, "elementTypes") {
        elements.add(e, 1);
    }
    // Element thresholds are `elementOptions` (cost fields + body); text
    // conditions are `conditionalTitle`/`conditionalBody`.
    let mut thresholds: Vec<Threshold> = v
        .get("elementOptions")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .map(|t| {
                    let body = strings(t, "body");
                    Threshold { elements: element_set_from_costs(t), title: String::new(), effects: effects::parse(&body), body }
                })
                .collect()
        })
        .unwrap_or_default();
    if v.get("conditionalBody").is_some() {
        let title = str_of(v, "conditionalTitle");
        let body = strings(v, "conditionalBody");
        thresholds.push(Threshold { elements: element_set_from_tokens(&title), title, effects: effects::parse(&body), body });
    }
    let body = strings(v, "body");
    let effects = effects::parse(&body);
    Ok(PowerCard {
        id: id(v)?,
        title: str_of(v, "title"),
        kind,
        cost: num(v, "energyCost"),
        speed: speed_of(v),
        range: range_of(v),
        target: TargetKind(str_of(v, "targetType")),
        elements,
        body,
        thresholds,
        effects,
        expansion: str_of(v, "expansion"),
    })
}

fn board(v: &Value) -> Result<BoardDef> {
    let bid = id(v)?;
    let lands = v
        .get("lands")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|l| {
                    let terrain = Terrain::parse(l.get("landType")?.as_str()?)?;
                    let number = num(l, "number") as u8;
                    Some(LandDef {
                        id: LandId::new(&LandId::board_code(&bid), number),
                        terrain,
                        adjacent: l.get("adjacentLands").and_then(Value::as_array).map(|x| x.iter().filter_map(Value::as_i64).map(|n| n as u8).collect()).unwrap_or_default(),
                        initial: InitialPieces {
                            explorers: num(l, "initialExplorers") as u8,
                            towns: num(l, "initialTowns") as u8,
                            cities: num(l, "initialCities") as u8,
                            dahan: num(l, "initialDahan") as u8,
                            blight: num(l, "initialBlight") as u8,
                            beasts: num(l, "initialBeasts") as u8,
                            wilds: num(l, "initialWilds") as u8,
                            disease: num(l, "initialDisease") as u8,
                            strife: num(l, "initialStrife") as u8,
                            badlands: num(l, "initialBadlands") as u8,
                        },
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(BoardDef { title: str_of(v, "title"), id: bid, lands })
}

fn invader_card(v: &Value) -> Result<InvaderCard> {
    Ok(InvaderCard { id: id(v)?, stage: num(v, "stage") as u8, targets: strings(v, "landTypes").iter().map(|s| InvaderTarget::parse(s)).collect(), escalation: flag(v, "escalation") })
}

fn adversary(v: &Value) -> Result<Adversary> {
    let levels = v
        .get("levels")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .map(|l| {
                    let fm = str_of(l, "fearCardModifiers");
                    let mut mods = [0u8; 3];
                    for (i, p) in fm.split('/').take(3).enumerate() {
                        mods[i] = p.trim().parse().unwrap_or(0);
                    }
                    let titles = strings(l, "additionalGameEffectTitles");
                    let bodies = strings(l, "additionalGameEffectBodies");
                    let body = str_of(l, "gameEffectBody");
                    let deck_order = regex::Regex::new(r"[Dd]eck [Oo]rder: ([0-9CS-]+)").unwrap().captures(&body).map(|c| c[1].to_string());
                    let land_list = |key: &str| opt_num(l, key).map(|n| vec![n as u8]).unwrap_or_default();
                    let setup = si_model::setup::SetupAdditions {
                        towns_in_land: land_list("addTownToLandNumberOnEachBoard"),
                        cities_in_land: land_list("addCityToLandNumberOnEachBoard"),
                        blight_in_land: land_list("addBlightToLandNumberOnEachBoard"),
                        move_blight: l.get("moveBlightFromLandNumberToLandNumber").and_then(Value::as_array).and_then(|a| Some((a.first()?.as_i64()? as u8, a.get(1)?.as_i64()? as u8))),
                        explorer_highest_without_town_city: flag(l, "addExplorerToHighestNumberLandWithoutTownCityOnEachBoard"),
                        beast_highest_without_town_city: flag(l, "addBeastToHighestNumberLandWithoutTownCityOnEachBoard"),
                        town_highest_without_town: flag(l, "addTownToHighestNumberLandWithoutTownOnEachBoard"),
                    };
                    AdversaryLevel {
                        number: num(l, "levelNumber") as u8,
                        difficulty: num(l, "difficulty") as u8,
                        fear_modifiers: mods,
                        title: str_of(l, "gameEffectTitle"),
                        accelerate: body.contains("Accelerate the Invader Deck"),
                        body,
                        extra: titles.into_iter().zip(bodies).collect(),
                        deck_order,
                        setup,
                    }
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(Adversary {
        id: id(v)?,
        title: str_of(v, "title"),
        base_difficulty: num(v, "baseDifficulty") as u8,
        levels,
        escalation: (str_of(v, "stageIIEscalationTitle"), str_of(v, "stageIIEscalationBody")),
        loss_condition: (str_of(v, "additionalLossConditionTitle"), str_of(v, "additionalLossCondition")),
        expansion: str_of(v, "expansion"),
    })
}
