import type { InvaderCard, InvaderDeckState } from '../types'

/**
 * Parse a card's terrain(s) into an array. Single-terrain cards return one
 * entry; pair-terrain cards ("Jungle + Mountain") return both.
 */
export function terrainsOf(card: InvaderCard | null | undefined): string[] {
  if (!card?.terrain) return []
  return card.terrain
    .split(/\s*\+\s*|\s*\/\s*/)
    .map(t => t.trim().toLowerCase())
    .filter(Boolean)
}

export interface TurnExposure {
  /** Absolute turn number. */
  turn: number
  ravage: InvaderCard | null
  build: InvaderCard | null
  explore: InvaderCard | null
  /** Lowercased terrain slugs touched in each phase this turn. */
  ravaged: string[]
  built: string[]
  explored: string[]
  /** Terrains that appear in ≥2 phases the same turn (e.g., Build Jungle +
   *  Ravage Jungle+Mountain → Jungle doubles). */
  doubled: string[]
  /** Terrains that appear in all three phases (triple concentration). */
  tripled: string[]
}

/**
 * Project the deck forward `turns` turns. Each step simulates Advance Turn:
 * Ravage → discarded, Build → Ravage, Explore → Build, upcoming[0] → Explore.
 * The returned timeline lets us spot concentration pressure — whether the
 * same terrain lands in multiple phases the same turn (e.g., PRU6 flips
 * "Jungle + Mountain" then "Jungle" next → Jungle build-up at T{n+1}).
 */
export function projectExposure(
  deck: InvaderDeckState | null | undefined,
  startTurn: number,
  turns: number,
): TurnExposure[] {
  if (!deck) return []
  const out: TurnExposure[] = []

  // Work on a mutable copy so we don't mutate state.
  let ravage: InvaderCard | null = deck.ravage
  let build: InvaderCard | null = deck.build
  let explore: InvaderCard | null = deck.explore
  const upcoming = [...deck.upcoming]

  for (let i = 0; i < turns; i++) {
    const ravaged = terrainsOf(ravage)
    const built = terrainsOf(build)
    const explored = terrainsOf(explore)

    // Intersect to find doubles + triples
    const rSet = new Set(ravaged)
    const bSet = new Set(built)
    const eSet = new Set(explored)
    const allTerrains = new Set([...ravaged, ...built, ...explored])
    const doubled: string[] = []
    const tripled: string[] = []
    for (const t of allTerrains) {
      const count = (rSet.has(t) ? 1 : 0) + (bSet.has(t) ? 1 : 0) + (eSet.has(t) ? 1 : 0)
      if (count === 3) tripled.push(t)
      else if (count === 2) doubled.push(t)
    }

    out.push({
      turn: startTurn + i,
      ravage,
      build,
      explore,
      ravaged,
      built,
      explored,
      doubled,
      tripled,
    })

    // Advance to next turn
    ravage = build
    build = explore
    explore = upcoming.shift() ?? null
  }

  return out
}

/** Terrain affinity map — symbolic slug to display name. */
export const TERRAIN_LABELS: Record<string, string> = {
  mountain: 'Mountain',
  jungle: 'Jungle',
  sands: 'Sands',
  wetland: 'Wetland',
  'coastal lands': 'Coastal Lands',
  coastal: 'Coastal',
}

export function labelFor(terrainSlug: string): string {
  return TERRAIN_LABELS[terrainSlug] ?? terrainSlug.replace(/\b\w/g, c => c.toUpperCase())
}
