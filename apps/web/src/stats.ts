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

export async function fetchStats(): Promise<StatsResponse> {
  const res = await fetch('/api/stats')
  if (!res.ok) throw new Error(`GET /api/stats → ${res.status}`)
  return res.json()
}

export async function fetchDrawProbability(
  deck: 'minor' | 'major' | 'unique',
  element: string,
  draws = 1,
  atLeast = 1,
  drawn = 0,
): Promise<DrawProbabilityResponse> {
  const params = new URLSearchParams({
    deck,
    element: element.toLowerCase(),
    draws: String(draws),
    at_least: String(atLeast),
    drawn: String(drawn),
  })
  const res = await fetch(`/api/draw-probability?${params}`)
  if (!res.ok) throw new Error(`GET /api/draw-probability → ${res.status}`)
  return res.json()
}
