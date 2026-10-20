import type { GameState } from './types'

export async function fetchState(): Promise<GameState> {
  const res = await fetch('/api/state')
  if (!res.ok) throw new Error(`GET /api/state → ${res.status}`)
  return res.json()
}

export async function saveState(state: GameState): Promise<GameState> {
  const res = await fetch('/api/state', {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(state),
  })
  if (!res.ok) throw new Error(`PUT /api/state → ${res.status}`)
  return res.json()
}

export type DeckKind = 'fear' | 'event' | 'minor' | 'major' | 'unique' | 'blight'

export interface FearCardData {
  name: string
  expansion?: string
  text_stage_1?: string
  text_stage_2?: string
  text_stage_3?: string
  status?: string
}

export interface EventCardData {
  name: string
  expansion?: string
  stages?: Array<{ event_type: string; event_name: string | null; text: string }>
  status?: string
}

interface DeckResponse<T> {
  cards: T[]
}

const deckCache = new Map<DeckKind, unknown>()

export async function fetchDeck<T = unknown>(kind: DeckKind): Promise<T[]> {
  if (deckCache.has(kind)) return deckCache.get(kind) as T[]
  const res = await fetch(`/api/deck/${kind}`)
  if (!res.ok) throw new Error(`GET /api/deck/${kind} → ${res.status}`)
  const body = (await res.json()) as DeckResponse<T>
  const cards = body.cards ?? []
  deckCache.set(kind, cards)
  return cards
}

export interface CanonicalInvaderStack {
  stack_sequence: number[]
  notation?: string
  notes?: string
}

export async function fetchInvaderStack(adversary?: string | null, level?: number | null): Promise<CanonicalInvaderStack> {
  const qp = new URLSearchParams()
  if (adversary) qp.set('adversary', adversary)
  if (level != null) qp.set('level', String(level))
  const res = await fetch(`/api/invader-stack?${qp.toString()}`)
  if (!res.ok) throw new Error(`GET /api/invader-stack → ${res.status}`)
  return res.json()
}

export interface SpiritAffinityEntry {
  terrain_affinity: Record<string, number>
  concentration: number
  rationale?: string
}

export interface SpiritAffinityMap {
  spirits: Record<string, SpiritAffinityEntry>
  default: SpiritAffinityEntry
}

let affinityCache: SpiritAffinityMap | null = null

export async function fetchSpiritAffinity(): Promise<SpiritAffinityMap> {
  if (affinityCache) return affinityCache
  const res = await fetch('/api/spirit-affinity')
  if (!res.ok) throw new Error(`GET /api/spirit-affinity → ${res.status}`)
  const body = await res.json()
  affinityCache = body as SpiritAffinityMap
  return affinityCache
}
