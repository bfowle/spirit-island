export interface PileCounts {
  hand: number
  discard: number
  played_this_turn: number
  forgotten: number
  presence_placed: number
}

export interface StatsResponse {
  round: number
  fear_current: number
  fear_threshold: number
  terror_level: number
  blight_current: number
  blight_cap: number
  elements_per_spirit: Record<string, Record<string, number>>
  pile_counts: Record<string, PileCounts>
  fear_by_round: { round: number; fear_generated: number }[]
}

export interface DrawProbabilityResponse {
  deck: string
  element: string
  population: number
  successes: number
  draws: number
  at_least: number
  probability: number
  wilson_95: [number, number]
}

// v0-mode: mock all stats fetches with empty/default data.
export async function fetchStats(): Promise<StatsResponse> {
  return {
    round: 1,
    fear_current: 0,
    fear_threshold: 4,
    terror_level: 1,
    blight_current: 0,
    blight_cap: 4,
    elements_per_spirit: {},
    pile_counts: {},
    fear_by_round: [],
  }
}

export async function fetchDrawProbability(
  deck: 'minor' | 'major' | 'unique',
  element: string,
  draws = 1,
  atLeast = 1,
  _drawn = 0,
): Promise<DrawProbabilityResponse> {
  return {
    deck,
    element: element.toLowerCase(),
    population: 0,
    successes: 0,
    draws,
    at_least: atLeast,
    probability: 0,
    wilson_95: [0, 0],
  }
}
