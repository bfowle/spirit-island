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
