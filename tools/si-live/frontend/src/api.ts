import type { GameState } from './types'
import sampleState from '../../../data/current-game.json'

// v0-mode: mock all backend calls. Use the sample JSON as the live state.
// When I wire this back into the real project I'll restore the HTTP fetches.
let liveState: GameState = sampleState as unknown as GameState

export async function fetchState(): Promise<GameState> {
  return JSON.parse(JSON.stringify(liveState))
}

export async function saveState(state: GameState): Promise<GameState> {
  liveState = state
  return state
}

export type DeckKind = 'fear' | 'event' | 'minor' | 'major' | 'unique' | 'blight'
export interface FearCardData { name: string; expansion?: string; text_stage_1?: string; text_stage_2?: string; text_stage_3?: string }
export interface EventCardData { name: string; expansion?: string; stages?: Array<{ event_type: string; event_name: string | null; text: string }> }
export async function fetchDeck<T = unknown>(_kind: DeckKind): Promise<T[]> { return [] }

export interface CanonicalInvaderStack { stack_sequence: number[]; notation?: string; notes?: string }
export async function fetchInvaderStack(): Promise<CanonicalInvaderStack> {
  return { stack_sequence: [1,1,1,2,2,2,2,3,3,3,3,3], notation: '3 · 4 · 5' }
}

export interface SpiritAffinityEntry { terrain_affinity: Record<string, number>; concentration: number; rationale?: string }
export interface SpiritAffinityMap { spirits: Record<string, SpiritAffinityEntry>; default: SpiritAffinityEntry }
export async function fetchSpiritAffinity(): Promise<SpiritAffinityMap> {
  return { spirits: {}, default: { terrain_affinity: { mountain: 0, jungle: 0, sands: 0, wetland: 0 }, concentration: 0 } }
}
