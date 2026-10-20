import type { GameState } from '../types'
import type { SpiritAffinityMap, SpiritAffinityEntry } from '../api'
import { projectExposure } from './terrain-exposure'

/**
 * Shared win-probability + end-state computation.
 *
 * Hoisted from StatsPanel so the sticky status bar, stats charts, end-banner,
 * and any future analytical view consume a single canonical answer. Both the
 * live snap-to-100% on victory and the 'imminent' warning fire from here.
 */

function clamp(v: number, lo = 0, hi = 1): number {
  return Math.max(lo, Math.min(hi, v))
}

export type EndState = 'won' | 'lost' | 'imminent' | 'in-progress'

export interface WinProbResult {
  mean: number
  lo: number
  hi: number
  endState: EndState
  /** The dominant fear-deck-progress factor (0..1). Useful for UI bars. */
  fearDeckProgress: number
  /** For display — reason the win-prob resolved the way it did. */
  reason: string
}

export function computeWinProb(
  state: GameState,
  affinityMap: SpiritAffinityMap | null,
): WinProbResult {
  const p = state.pools
  const fd = state.fear_deck
  const round = state.round

  const deckSize = fd?.deck_size ?? 9
  const consumed = (fd?.resolved.length ?? 0) + (fd?.earned.length ?? 0)
  const fearDeckProgress = deckSize > 0 ? Math.min(1, consumed / deckSize) : 0

  // --- Definitive WIN: deck fully drawn + terror 3 ----------------------------
  if (fd && consumed >= deckSize && p.terror_level >= 3) {
    return {
      mean: 1, lo: 1, hi: 1,
      endState: 'won',
      fearDeckProgress,
      reason: 'Fear deck drawn at Terror 3',
    }
  }

  // --- Definitive LOSS: blight cap reached on flipped island ------------------
  if (p.blight_current >= p.blight_cap && p.island_blighted) {
    return {
      mean: 0, lo: 0, hi: 0,
      endState: 'lost',
      fearDeckProgress,
      reason: 'Blight cap reached on Blighted Island',
    }
  }

  // --- IMMINENT WIN: deck fully drawn (or 1 off + guaranteed T3) --------------
  // Covers the mid-turn case where the player has banked/resolved every fear
  // card this Slow phase but Terror 3 hasn't been marked yet. Typically the
  // terror watcher will catch up within a tick, but if not — we still signal
  // "you're basically already won" so the UI stops pretending it's 60%.
  //
  // Also covers "one card to go + pool > 0" scenarios where the next bank will
  // definitely clinch Terror 3 (assuming the terror dividers are at deck
  // thirds — which they are for the default 3·N model).
  const oneBankAwayFromDrawn = fd && consumed >= deckSize - 1 && p.fear_current >= p.fear_threshold
  if (fd && (consumed >= deckSize || oneBankAwayFromDrawn) && p.terror_level >= 2) {
    // Still compute mean below as a high number rather than snap — so the
    // chart shows the approach, not a sudden cliff. Use 0.96 as a "very high"
    // imminent-win anchor.
    const mean = 0.96
    return {
      mean,
      lo: 0.9,
      hi: 1,
      endState: 'imminent',
      fearDeckProgress,
      reason: consumed >= deckSize
        ? 'Fear deck drawn — awaiting Terror 3'
        : 'Last fear bank imminent',
    }
  }

  // --- Heuristic calculation for in-progress games ---------------------------
  const fearRatio = p.fear_threshold > 0 ? p.fear_current / p.fear_threshold : 0
  const blightRatio = p.blight_cap > 0 ? p.blight_current / p.blight_cap : 0

  // Terrain concentration × spirit affinity modifier
  let terrainMod = 0
  if (affinityMap) {
    const exposure = projectExposure(state.invader_deck, round, 3)
    let totalScore = 0
    let sampleCount = 0
    for (const turn of exposure) {
      const concentrationPressure = turn.doubled.length + 2 * turn.tripled.length
      if (concentrationPressure === 0) continue
      for (const slug of Object.keys(state.spirits ?? {})) {
        const aff: SpiritAffinityEntry = affinityMap.spirits[slug] ?? affinityMap.default
        totalScore += aff.concentration * concentrationPressure
        sampleCount++
      }
    }
    if (sampleCount > 0) {
      const avg = totalScore / sampleCount
      terrainMod = clamp(avg * 0.04, -0.15, 0.15)
    }
  }

  let mean = 0.50
  mean += fearDeckProgress * 0.45
  if (p.terror_level >= 3) mean += 0.15
  else if (p.terror_level >= 2) mean += 0.08
  mean += fearRatio * 0.05
  mean -= blightRatio * 0.30
  if (p.island_blighted) mean -= 0.15
  mean -= Math.max(0, round - 8) * 0.05
  mean += terrainMod
  mean = clamp(mean)

  // Wilson-ish interval at notional n=12 observations
  const n = 12
  const z = 1.96
  const denom = 1 + (z * z) / n
  const center = (mean + (z * z) / (2 * n)) / denom
  const margin = (z * Math.sqrt((mean * (1 - mean)) / n + (z * z) / (4 * n * n))) / denom

  return {
    mean,
    lo: clamp(center - margin),
    hi: clamp(center + margin),
    endState: 'in-progress',
    fearDeckProgress,
    reason: `deck ${consumed}/${deckSize} · T${p.terror_level} · blight ${p.blight_current}/${p.blight_cap}`,
  }
}

/** Convenient display helpers */
export function fearPct(state: GameState): number {
  const p = state.pools
  return !p.fear_threshold ? 0 : Math.min(100, (p.fear_current / p.fear_threshold) * 100)
}
export function blightPct(state: GameState): number {
  const p = state.pools
  return !p.blight_cap ? 0 : Math.min(100, (p.blight_current / p.blight_cap) * 100)
}
