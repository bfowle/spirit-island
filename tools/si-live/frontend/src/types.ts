export type Phase = 'setup' | 'growth' | 'fast' | 'event' | 'fear' | 'invader' | 'slow' | 'timepasses' | 'end'

export interface Land {
  terrain: string
  coastal?: boolean
  explorers?: number
  towns?: number
  cities?: number
  dahan?: number
  blight?: number
  tokens?: string[]
}

export interface Board {
  lands: Record<string, Land>
  /** Variant key when game was created via New Game: "balanced" | "thematic". */
  variant?: string
  /** Human-readable variant name (e.g., "Board A" or "North East"). */
  variant_name?: string
}

export interface Spirit {
  energy: number
  card_plays: number
  /** Keys are "{boardId}.{landId}" (e.g., "A.6"). Values are presence count. */
  presence_on_board?: Record<string, number>
  presence_on_track_energy?: string[]
  presence_on_track_cardplay?: string[]
  elements_this_turn?: Record<string, number>
  hand?: string[]
  discard?: string[]
  forgotten?: string[]
  played_this_turn?: string[]
  growth_options?: unknown[]
  /** Per-spirit disc customization (drives the color + style used everywhere
   *  this spirit's presence is rendered). */
  disc_color?: string
  disc_style?: 'glass' | 'wood' | 'solid'
  /** Selected aspect slug (e.g., "reach" for Shadows). null/undefined = base spirit. */
  aspect?: string | null
  /** Display name of the selected aspect (e.g., "Dark Fire"). */
  aspect_name?: string | null
  /** Aspect special-rule overrides (name + text pairs). */
  aspect_special_rules?: Array<{ name: string; text: string }> | null
  /** Raw setup-note string from the aspect (e.g., "Gain Unquenchable Flames (Minor Power)"). */
  aspect_setup_note?: string | null
  /** Replacement innate (when aspect overrides the spirit's base innate). Null when aspect keeps the base. */
  aspect_innate_override?: {
    innate_name?: string
    speed?: string
    range?: string
    target?: string
    innate_thresholds?: string
  } | null
}

export interface SpiritDiscInfo {
  slug: string
  color: string
  style: 'glass' | 'wood' | 'solid'
}

export interface Pools {
  fear_current: number
  fear_threshold: number
  terror_level: number
  blight_current: number
  blight_cap: number
  island_blighted?: boolean
  /** Selected Blight card at setup (name matches data/decks/blight.json). */
  blight_card?: string | null
}

export interface Setup {
  adversary?: string | null
  level?: number | null
  scenario?: string | null
  spirits: string[]
  boards: string[]
  expansions_active: string[]
  /** Aspect assignments — spirit slug → aspect key. Omitted spirits use base. */
  aspects?: Record<string, string>
}

export interface InvaderCard {
  /** 1 | 2 | 3, locked at setup when the stack is built from adversary/level data. */
  stage: number
  /** Revealed when the card is flipped (e.g., "Jungle", "Mountain + Wetland"). */
  terrain?: string | null
  /** Optional notes (adversary escalation effect applied, etc.). */
  notes?: string | null
}

export interface InvaderDeckState {
  ravage: InvaderCard | null
  build: InvaderCard | null
  explore: InvaderCard | null
  /** Ordered upcoming stack; [0] = next to flip into Explore. */
  upcoming: InvaderCard[]
  /** Discarded (post-Ravage) cards — preserved for retrospective analysis. */
  discarded: InvaderCard[]
  /** Human-readable notation of the stack shape (e.g., "3 · 4 · 5"). */
  notation?: string | null
}

export interface FearCardEntry {
  name: string
  terror_level: number
  round: number
}

export interface FearDeckState {
  deck_size: number
  /** Card count at each terror level [T1, T2, T3]. When absent, evenly split from deck_size. */
  tier_counts?: [number, number, number]
  earned: FearCardEntry[]
  resolved: FearCardEntry[]
  unseen: number
}

export interface EventCardEntry {
  name: string
  previewed_on_turn: number
  resolved_on_turn?: number | null
}

export interface EventDeckState {
  /** Face-up cards previewed for next turn. Typically 1 at a time. */
  previewed: EventCardEntry[]
  /** Events that have already resolved in prior turns. */
  resolved: EventCardEntry[]
  /** Rough remaining count in the event deck. */
  unseen: number
}

export interface GameState {
  version: string
  round: number
  phase: Phase
  setup: Setup
  pools: Pools
  spirits: Record<string, Spirit>
  board_state: Record<string, Board>
  log: unknown[]
  invader_deck?: InvaderDeckState
  fear_deck?: FearDeckState
  event_deck?: EventDeckState
}

export const TERRAINS = ['mountain', 'wetland', 'jungle', 'sands'] as const
export const UNIT_KEYS = ['explorers', 'towns', 'cities', 'dahan', 'blight'] as const
export type UnitKey = typeof UNIT_KEYS[number]
