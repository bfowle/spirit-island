/**
 * Mirrors of the Steam companion's Rust types (serde JSON shape).
 *
 * Sources of truth:
 *   tools/si-live/common/src/public.rs
 *   tools/si-live/common/src/rules.rs                       (RavageLedger)
 *   tools/si-live/common/src/companion_stats/{mod,invaders,spirits,review}.rs
 *   tools/si-live/steam/src/companion.rs                    (CompanionState)
 *
 * Conventions: field names as serialized (snake_case), Rust tuples as
 * fixed-length arrays, `Option<T>` as `T | null`, `BTreeMap<K, V>` as
 * `Record<K, V>`, unit enums as string unions.
 */

// ─── public.rs ──────────────────────────────────────────────────────────────

export interface PublicSnapshot {
  game_id: string
  engine_build: string
  /** Human turn number (engine TurnIndex + 1; setup belongs to Turn 1). */
  turn: number
  /** StartOfGame, Spirit, FastPower, Invader, SlowPower, TimePasses. */
  phase: string
  /** Grow, FastPower, Ravage, Build, Explore, … */
  step: string
  difficulty: number | null
  adversaries: AdversaryRef[]
  scenario: string | null
  multiplayer: boolean
  boards: PublicBoard[]
  lands: PublicLand[]
  /** Undirected adjacency by land id, Ocean lands included. */
  adjacency: [string, string][]
  spirits: PublicSpirit[]
  invaders: PublicInvaderDeck
  fear: PublicFear
  events: PublicEvents
  powers: PublicPowers
  blight: PublicBlight
  status_effects: PublicStatusEffect[]
  ending: string | null
  warnings: string[]
}

export interface AdversaryRef {
  id: string
  title: string
  level: number
}

export interface PublicBoard {
  id: string
  lands: string[]
}

export interface PublicLand {
  id: string
  board: string
  number: number
  /** Jungle, Mountain, Sands, Wetland, Ocean. */
  terrain: string
  is_ocean: boolean
  /** Adjacent to an Ocean land. */
  coastal: boolean
  explorers: number
  towns: number
  cities: number
  dahan: number
  blight: number
  /** Presence count by spirit id. */
  presence: Record<string, number>
  /** Token count by type (Beast, Wilds, Disease, Badlands, …). */
  tokens: Record<string, number>
  pieces: PublicPiece[]
  damage_taken: number
  defend: number
  health_modifier: number
}

export interface PublicPiece {
  /** Explorer, Town, City, Dahan, Thief. */
  kind: string
  max_health: number
  damage: number
  strife: number
}

export interface PublicSpirit {
  id: string
  title: string
  aspect: string | null
  energy: number
  card_plays: number
  phase_status: string | null
  elements: Record<string, number>
  element_markers: Record<string, number>
  hand: PublicPowerCard[]
  play_area: PublicPowerCard[]
  discard: PublicPowerCard[]
  forgotten: PublicPowerCard[]
  revealed: PublicPowerCard[]
  track: TrackNode[]
  presence_on_island: number
  presence_destroyed: number
  innates: string[]
  innate_powers: PublicInnate[]
  special_rules: NamedText[]
}

export interface TrackNode {
  id: string
  revealed: boolean
  kind: string
  value: number | null
  elements: string[]
}

export interface PublicInnate {
  id: string
  title: string
  speed: string
  range: number | null
  from_sacred_site: boolean
  target: string | null
  levels: InnateLevel[]
}

export interface InnateLevel {
  elements: Record<string, number>
  body: string
}

export interface NamedText {
  id: string
  title: string
  body: string[]
}

export interface PublicPowerCard {
  id: string
  title: string
  cost: number
  /** "Fast" or "Slow". */
  speed: string
  /** "Minor", "Major", "Unique". */
  kind: string
  elements: string[]
  range: number | null
  from_sacred_site: boolean
  target: string | null
  body: string[]
}

export interface PublicInvaderCard {
  id: string
  stage: number
  /** The card carries the Escalation flag (Stage II terrain cards). serde(default). */
  escalation?: boolean
  /** Jungle, Mountain, Sands, Wetland, Coastal, SaltDeposits. */
  terrains: string[]
}

export interface PublicInvaderDeck {
  /** Stage printed on the back of each face-down card, next card first. */
  upcoming_stages: number[]
  explore: PublicInvaderCard[]
  build: PublicInvaderCard[]
  ravage: PublicInvaderCard[]
  discard: PublicInvaderCard[]
  extra_slots: [string, PublicInvaderCard[]][]
  highest_stage_reached: number
  pool: PublicInvaderCard[]
}

export interface PublicFear {
  generated: number
  required_per_card: number
  terror_level: number
  earned_count: number
  /** [TL I, TL II, TL III] */
  deck_by_level: [number, number, number]
  discard: PublicFearCard[]
  removed_unseen: number
}

export interface PublicFearCard {
  id: string
  title: string
  resolved_at_terror_level: number | null
}

export interface PublicEvents {
  deck_count: number
  /** Newest last. */
  discard: NamedCard[]
}

export interface NamedCard {
  id: string
  title: string
}

export interface PublicPowers {
  minor_deck_count: number
  major_deck_count: number
  minor_discard: string[]
  major_discard: string[]
  minor_unseen: PublicPowerCard[]
  major_unseen: PublicPowerCard[]
}

export interface PublicBlight {
  flipped: boolean
  remaining: number
  on_island: number
  healthy_side: string[]
  blighted_card: NamedCard | null
  blighted_text: string[]
  unseen_flipped: (number | null)[]
  unseen_still_healthy: number
}

export interface PublicStatusEffect {
  kind: string
  /** Title of the public card or rule that created it, when known. */
  source: string | null
  /** Engine parameters (non-default only), when the source is public. */
  params: Record<string, StatusParam>
}

/** serde(untagged): Int(i64) | Bool(bool) | List(Vec<String>). */
export type StatusParam = number | boolean | string[]

// ─── rules.rs ───────────────────────────────────────────────────────────────

export interface RavageLedger {
  /** Damage by invader kind before Defend, e.g. [["City", 3], ["Explorer", 2]]. */
  damage_terms: [string, number][]
  invader_damage: number
  defend: number
  damage_after_defend: number
  land_health: number
  adds_blight: boolean
  cascades: boolean
  dahan_before: number
  dahan_destroyed: number
  counterattack_damage: number
  counterattack_can_destroy: string[]
  counterattack_fear: number
  /** The counterattack destroys every Invader whatever the damage assignment. */
  counterattack_clears: boolean
  presence_at_risk: Record<string, number>
  /** Rule effects the ledger applied, with their evidence. */
  applied: string[]
  /** An optional Skip that can cancel this Ravage (source, uses left). */
  skip_available: string | null
  /** Tokens or effects present that the base ledger does not model. */
  unmodeled: string[]
}

// ─── companion_stats/invaders.rs ────────────────────────────────────────────

export interface InvaderPhaseForecast {
  turn: number
  /** "before", "ravage", "build", "explore". */
  position: string
  ravage_cards: PublicInvaderCard[]
  build_cards: PublicInvaderCard[]
  ravages: RavageForecast[]
  builds: BuildForecast[]
  explore: ExploreForecast
  notes: string[]
  /** England L3+: the High Immigration card and its Builds before the Ravage. */
  high_immigration_cards: PublicInvaderCard[]
  high_immigration: BuildForecast[]
  /** A Stage II Escalation that Builds (England), if the Explore card is flagged. */
  escalation: EscalationForecast | null
}

export interface EscalationForecast {
  title: string
  text: string
  /** [flagged cards, possible Explore cards] */
  chance: [number, number]
  builds: BuildForecast[]
}

export interface RavageForecast {
  land: string
  ledger: RavageLedger
  done: boolean
}

export interface BuildForecast {
  land: string
  adds: string
  done: boolean
  notes: string[]
}

export interface ExploreForecast {
  revealed: PublicInvaderCard | null
  next_stage: number | null
  deck_empty: boolean
  candidates: ExploreCandidate[]
  /** Per land: [cards that explore it, candidate cards]. */
  per_land: Record<string, [number, number]>
  pocket_rule: string
}

export interface ExploreCandidate {
  card: PublicInvaderCard
  lands: string[]
  /** Matching lands that won't explore: a bare id (no Explore source in
   *  range) or "SandsC6 (Wilds)" (stopped by a token). */
  blocked: string[]
}

export interface StageCount {
  stage: number
  seen: string[]
  unseen: PublicInvaderCard[]
  left_in_deck: number
}

export type Ravage2 = 'No' | 'Certain' | 'IfExplored'

export interface LandOutlook {
  id: string
  terrain: string
  coastal: boolean
  explorers: number
  towns: number
  cities: number
  dahan: number
  blight: number
  presence: number
  /** "coastal", "fed", "dead", "ocean". */
  explore_access: string
  fed_by: string[]
  sole_source_for: string[]
  ravage_now: RavageLedger | null
  build_now: string | null
  /** [cards that explore it, candidates]. */
  explore_chance: [number, number]
  ravage_next: Ravage2
  build_next_chance: [number, number]
  badges: string[]
  fast_window: string[]
  slow_window: string[]
}

// ─── companion_stats/spirits.rs ─────────────────────────────────────────────

export interface SpiritStats {
  id: string
  title: string
  energy: number
  card_plays: number
  energy_per_turn: number | null
  plays_per_turn: number | null
  next_energy: number | null
  next_plays: number | null
  elements: Record<string, number>
  innates: InnateStatus[]
  draw_odds: DrawOdds[]
  hand_fast: string[]
  hand_slow: string[]
}

export interface InnateStatus {
  title: string
  speed: string
  levels: LevelStatus[]
}

export interface LevelStatus {
  threshold: Record<string, number>
  met_now: boolean
  missing: Record<string, number>
  reachable_with: string[] | null
  body: string
}

export interface DrawOdds {
  element: string
  /** [cards with the element, unseen pool, P(≥1 in a draw of 4)]. */
  minor: [number, number, number]
  major: [number, number, number]
}

// ─── companion_stats/mod.rs ─────────────────────────────────────────────────

export interface CompanionStats {
  invader_phase: InvaderPhaseForecast
  lands: LandOutlook[]
  stages: StageCount[]
  fear: FearStats
  blight: BlightStats
  spirits: SpiritStats[]
  /** Need finder, one per spirit (companion_stats/needs.rs). */
  needs: SpiritNeeds[]
  /** Rules from the register that touch the invader pipeline (rules_register.rs). */
  rules_in_play: RuleInPlay[]
  prompts: Prompt[]
  basis: string[]
}

/** rules_register.rs */
export interface RuleInPlay {
  /** "Brandenburg-Prussia L2", "Escalation", "Shadows Flicker Like Flame", … */
  from: string
  title: string
  kind: string
  text: string
  /** agree / conflict / engine-only / wiki-only. */
  status: string
  /** "reviewed by Brett (…)", "sourced (…)", or empty when unreviewed. */
  review: string
  applied: boolean
}

// ─── companion_stats/needs.rs ───────────────────────────────────────────────

export interface NeedCount {
  /** Cards whose effect meets the need (any option, conditions allowed). */
  effect: number
  /** …and can target the land from this spirit's presence now. */
  reach: number
  /** …and are Fast (act before this turn's Invader Phase). */
  fast_reach: number
  pool: number
  /** P(≥1 in a draw of 4) for effect / reach / fast_reach. */
  p_effect: number
  p_reach: number
  p_fast_reach: number
  examples: string[]
}

export interface SaveLandNeed {
  land: string
  /** Damage to remove for the land to take no blight. */
  margin: number
  minor: NeedCount
  major: NeedCount
  /** Cards in hand that could do it, with reach and speed. */
  in_hand: string[]
  pocket_rule: string
}

/** [need, cards, pool, P(≥1 in a draw of 4)]. */
export type NeedFamily = [string, number, number, number]

export interface SpiritNeeds {
  spirit: string
  save_land: SaveLandNeed[]
  minor_families: NeedFamily[]
  major_families: NeedFamily[]
  basis: string[]
}

export interface Prompt {
  question: string
  panel: string
  facts: string[]
}

export interface FearStats {
  generated: number
  required_per_card: number
  to_next_card: number
  terror_level: number
  cards_left_by_level: [number, number, number]
  /** null at Terror Level III. */
  to_next_level: number | null
  to_clear_deck: number
  earned_banked: number
  victory_condition: string
  victory_blockers: number
  counterattack_fear_best: number
  /** [turn, fear generated that turn]. */
  per_turn: [number, number][]
  pocket_rule: string
}

export interface BlightStats {
  remaining: number
  flipped: boolean
  on_island: number
  adds_this_phase_min: number
  ravage_lands_adding: string[]
  /** "card holds", "card flips", "card runs out (loss)" or "(draw a new Blight Card)". */
  outcome_this_phase: string
  /** [blight the flipped side would add, how many unseen cards show it]. */
  flipped_side: [number, number][]
  /** How many of those unseen cards are Still-Healthy (no loss when emptied). */
  flipped_still_healthy: number
  /** [Blight this phase, chance of losing]. */
  loss_risk: [number, number][]
  /** Chance of losing at this phase's forecast. */
  loss_risk_now: number
  cascade_chains: CascadeChain[]
  /** [turn, blight on island]. */
  per_turn: [number, number][]
  pocket_rule: string
}

export interface CascadeChain {
  land: string
  min_extra: number
  safe_neighbors: string[]
}

export interface RoundSummary {
  turn: number
  last_step: string
  fear_total: number
  fear_cards_earned: number
  terror_level: number
  blight_on_island: number
  blight_remaining: number
  blight_flipped: boolean
  explorers: number
  towns: number
  cities: number
  dahan: number
  invaded_lands: number
  presence_on_island: number
  presence_destroyed: number
  energy: number
  cards_in_hand: number
  snapshots: number
}

// ─── companion_stats/review.rs ──────────────────────────────────────────────

export interface GameReview {
  game_id: string
  ending: string | null
  turns: number
  blight: BlightEvent[]
  chains: InvaderChain[]
  flow: TurnFlow[]
  /** [turn, total Ravage damage the island's Invaders could deal]. */
  pressure: [number, number][]
  /** [turn, jump]. */
  largest_jump: [number, number] | null
  turn_notes: TurnNote[]
  reflections: Reflection[]
}

export interface BlightEvent {
  turn: number
  step: string
  land: string
  added: number
  /** "Ravage", "cascade", or the step. */
  cause: string
}

export interface InvaderChain {
  land: string
  card: string
  ravaged_turn: number
  points: ChainPoint[]
  /** The player's decisions that touched this land while the chain built (empty = none). */
  actions_here: string[]
}

export interface ChainPoint {
  when: string
  explorers: number
  towns: number
  cities: number
  damage: number
  health_to_clear: number
}

export interface TurnFlow {
  turn: number
  added: Record<string, number>
  removed: Record<string, number>
}

export interface TurnNote {
  turn: number
  played: string[]
  energy_left: number
  plays_left: number
  /** The turn's decisions in order ("FastPower: Used Lure of the Unknown on MountainC3"). */
  decisions: string[]
}

export interface Reflection {
  question: string
  facts: string[]
}

// ─── steam/src/companion.rs ─────────────────────────────────────────────────

export interface CompanionState {
  seq: number
  public: PublicSnapshot | null
  stats: CompanionStats | null
  history: RoundSummary[]
  review: GameReview | null
  /** Play-by-play: the latest changes between snapshots, newest last. */
  changes: Change[]
  /** The player's decisions this game so far, from the log. */
  decisions: PublicDecision[]
  source: SourceStatus
}

/** public.rs: one player decision from the game log. */
export interface PublicDecision {
  turn: number
  step: string
  /** growth, gain-power, card-choice, play, power, effect, land-choice, presence, damage, forget. */
  kind: string
  text: string
  land: string | null
  card: string | null
  /** Capture wall clock, ms since midnight (orders decisions against saves). */
  ms: number
}

/** companion_stats/changes.rs */
export interface Change {
  turn: number
  /** Step the change happened in (the earlier snapshot's step). */
  step: string
  text: string
  /** "invader", "blight", "dahan", "presence", "fear", "card", "energy", "event". */
  kind: string
}

export interface SourceStatus {
  /** "live", "replay", or "off". */
  mode: string
  /** "running", "not-running", or "unknown" ("replay" in replay mode). */
  process: string
  last_save: string | null
  updated_at_ms: number
  stale: boolean
  engine_build: string
  decode_errors: number
  replay: ReplayStatus | null
  note: string | null
}

export interface ReplayStatus {
  capture: string
  index: number
  total: number
  playing: boolean
  speed: number
  /** HH:MM:SS */
  clock: string
}

/** `GET /api/companion/health` (backend/src/companion.rs). */
export interface CompanionHealth {
  source: SourceStatus
  seq: number
  has_state: boolean
  warnings: string[]
}

/** Body of `POST /api/companion/replay`. */
export type ReplayCommand =
  | { action: 'play'; speed: number }
  | { action: 'pause' }
  | { action: 'step'; by: number }
  | { action: 'seek'; index: number }
