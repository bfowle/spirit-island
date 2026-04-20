export type Phase = 'setup' | 'growth' | 'fast' | 'invader' | 'slow' | 'timepasses' | 'end'

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
  presence_on_board?: Record<string, number>
  presence_on_track_energy?: string[]
  presence_on_track_cardplay?: string[]
  elements_this_turn?: Record<string, number>
  hand?: string[]
  discard?: string[]
  forgotten?: string[]
  played_this_turn?: string[]
  growth_options?: unknown[]
}

export interface Pools {
  fear_current: number
  fear_threshold: number
  terror_level: number
  blight_current: number
  blight_cap: number
  island_blighted?: boolean
}

export interface Setup {
  adversary?: string | null
  level?: number | null
  scenario?: string | null
  spirits: string[]
  boards: string[]
  expansions_active: string[]
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
}

export const TERRAINS = ['mountain', 'wetland', 'jungle', 'sands'] as const
export const UNIT_KEYS = ['explorers', 'towns', 'cities', 'dahan', 'blight'] as const
export type UnitKey = typeof UNIT_KEYS[number]
