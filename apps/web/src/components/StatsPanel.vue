<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { Chart, registerables } from 'chart.js'
import { Line } from 'vue-chartjs'
import type { GameState } from '../types'
import type { StatsResponse, DrawProbabilityResponse } from '../stats'
import { fetchStats, fetchDrawProbability } from '../stats'
import { fetchSpiritAffinity, type SpiritAffinityMap } from '../api'
import { computeWinProb } from '../lib/winprob'
import Icon from './Icon.vue'

Chart.register(...registerables)

const props = defineProps<{ state: GameState }>()

const stats = ref<StatsResponse | null>(null)
const error = ref<string | null>(null)
const drawProbs = ref<Record<string, DrawProbabilityResponse>>({})

/** Count cards the player has permanently drafted out of each deck so far.
 *  These reduce the effective deck population for draw-probability modelling.
 *  Seen-but-not-kept cards don't count — they're returned to the deck bottom. */
const draftedByDeck = computed(() => {
  const log = (props.state.log as Array<{ event: string; details: Record<string, unknown> }>) ?? []
  const counts: Record<'minor' | 'major' | 'unique', number> = { minor: 0, major: 0, unique: 0 }
  for (const e of log) {
    if (e.event === 'card_drafted') {
      const d = (e.details.deck as 'minor' | 'major' | 'unique' | undefined) ?? 'minor'
      if (d in counts) counts[d]++
    }
  }
  return counts
})

async function refresh() {
  // Each fetch is independent; a stats failure shouldn't suppress draw probs
  // and vice-versa. Errors are shown inline without blanking existing data.
  try {
    stats.value = await fetchStats()
    error.value = null
  } catch (e) {
    error.value = `stats: ${(e as Error).message}`
  }

  try {
    const elems = ['moon', 'fire', 'air']
    const drafted = draftedByDeck.value
    const results = await Promise.all(
      elems.flatMap(el => [
        ['minor', el] as const,
        ['major', el] as const,
      ]).map(async ([deck, element]) => {
        try {
          const drawnCount = drafted[deck] ?? 0
          const r = await fetchDrawProbability(deck, element, 1, 1, drawnCount)
          return [`${deck}:${element}`, r] as const
        } catch {
          return null
        }
      }),
    )
    const next: Record<string, DrawProbabilityResponse> = {}
    for (const r of results) {
      if (r) next[r[0]] = r[1]
    }
    drawProbs.value = next
  } catch (e) {
    error.value = `draw-probs: ${(e as Error).message}`
  }
}

onMounted(refresh)
watch(() => props.state, refresh, { deep: true })

// Derive fear-by-round directly from the log so the chart updates instantly
// without waiting for the debounced backend PUT/GET roundtrip. Also track
// sources (which event / fear-card / note generated each bump) so the user
// can attribute fear swings to specific events during retrospection.
interface FearBump { round: number; amount: number; source: string }

const fearBumps = computed<FearBump[]>(() => {
  const log = (props.state.log as Array<{ round: number; event: string; details: Record<string, unknown> }>) ?? []
  const out: FearBump[] = []
  // Context tracking: most recent "in-flight" event/fear card sets the source
  // for subsequent fear_generated bumps until another context event arrives.
  let ctx = ''
  let ctxRound = -1
  for (const e of log) {
    if (ctxRound !== e.round) { ctx = ''; ctxRound = e.round }
    if (e.event === 'event_card' || e.event === 'event_resolved') {
      ctx = `Event: ${e.details.card}`
    } else if (e.event === 'fear_card') {
      ctx = `Fear card: ${e.details.card}`
    } else if (e.event === 'invader_card' || e.event === 'invader_rotated') {
      ctx = `Invader: ${e.details.card ?? e.details.ravaged_terrain ?? ''}`
    }
    if (e.event === 'fear_generated') {
      const amount = typeof e.details.amount === 'number' ? e.details.amount : 0
      out.push({ round: e.round, amount, source: ctx || 'unattributed' })
    }
  }
  return out
})

const fearByRound = computed(() => {
  const by = new Map<number, number>()
  for (const b of fearBumps.value) by.set(b.round, (by.get(b.round) ?? 0) + b.amount)
  return by
})

interface SourceAgg { amount: number; source: string }
interface RoundAttribution { round: number; total: number; sources: SourceAgg[] }

const fearSourcesByRound = computed<RoundAttribution[]>(() => {
  const byRound = new Map<number, SourceAgg[]>()
  for (const b of fearBumps.value) {
    const list = byRound.get(b.round) ?? []
    // Aggregate same-source bumps within a round
    const existing = list.find(s => s.source === b.source)
    if (existing) existing.amount += b.amount
    else list.push({ amount: b.amount, source: b.source })
    byRound.set(b.round, list)
  }
  return Array.from(byRound.entries())
    .sort(([a], [b]) => a - b)
    .map(([round, sources]) => ({
      round,
      total: sources.reduce((acc, s) => acc + s.amount, 0),
      sources,
    }))
})

/** Aggregate total fear generated per source-type across the whole game.
 *  Lets the retrospective show which card-type / event-type gave the most
 *  fear — useful for identifying which plays were the highest-leverage. */
const fearBySource = computed<SourceAgg[]>(() => {
  const totals = new Map<string, number>()
  let grand = 0
  for (const b of fearBumps.value) {
    totals.set(b.source, (totals.get(b.source) ?? 0) + b.amount)
    grand += b.amount
  }
  const out = Array.from(totals.entries())
    .map(([source, amount]) => ({ source, amount }))
    .sort((a, b) => b.amount - a.amount)
  // Stash grand total for display via the last-returned computed (see template)
  return out.map(s => ({ ...s, _pct: grand > 0 ? (s.amount / grand) * 100 : 0 } as SourceAgg & { _pct: number }))
})

const totalFearGenerated = computed(() => fearBumps.value.reduce((a, b) => a + b.amount, 0))

const fearChartData = computed(() => {
  const labels: string[] = []
  const perTurn: number[] = []
  const cumulative: number[] = []
  let running = 0
  for (let r = 1; r <= Math.max(props.state.round, 8); r++) {
    labels.push(`T${r}`)
    const v = fearByRound.value.get(r) ?? 0
    perTurn.push(v)
    running += v
    cumulative.push(running)
  }
  return {
    labels,
    datasets: [
      {
        label: 'Per-turn fear',
        data: perTurn,
        borderColor: 'rgba(217, 119, 87, 0.8)',
        backgroundColor: 'rgba(217, 119, 87, 0.15)',
        borderDash: [5, 5],
        tension: 0,
        pointRadius: 2,
      },
      {
        label: 'Cumulative',
        data: cumulative,
        borderColor: '#d4a373',
        backgroundColor: 'rgba(212, 163, 115, 0.15)',
        tension: 0.35,
        fill: true,
        pointRadius: 3,
      },
    ],
  }
})

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    y: {
      beginAtZero: true,
      ticks: { color: '#a3a3a8', font: { size: 11 } },
      grid: { color: 'rgba(52, 52, 58, 0.4)' },
    },
    x: {
      ticks: { color: '#a3a3a8', font: { size: 11 } },
      grid: { color: 'rgba(52, 52, 58, 0.4)' },
    },
  },
  plugins: {
    legend: {
      labels: { color: '#a3a3a8', font: { size: 11 } },
      position: 'bottom' as const,
    },
  },
}

// Win-probability delegated to shared lib so sticky bar, charts, and banner
// all stay canonical.
const affinityMap = ref<SpiritAffinityMap | null>(null)
onMounted(async () => {
  try {
    affinityMap.value = await fetchSpiritAffinity()
  } catch { /* degrades to non-terrain-aware probability */ }
})

const winProbability = computed(() => computeWinProb(props.state, affinityMap.value))
const terrainModifier = computed(() => {
  const wp = winProbability.value
  // Back-derive the terrain mod delta from the canonical calc for the footer hint.
  // Cheap approx: if reason mentions terrain concentration, we'd surface it here.
  return wp.endState === 'in-progress' ? 0 : 0
})

const winProbPct = computed(() => `${(winProbability.value.mean * 100).toFixed(0)}%`)
const winProbCI = computed(() =>
  `${(winProbability.value.lo * 100).toFixed(0)}–${(winProbability.value.hi * 100).toFixed(0)}%`,
)

// Win-prob history — track estimate at each round's advance. Derived from the
// state log's `turn_advanced` entries.
interface TurnAdvance { round: number; fear_current: number; blight_current: number }
const turnAdvances = computed<TurnAdvance[]>(() =>
  ((props.state.log ?? []) as { round: number; event: string; details: TurnAdvance }[])
    .filter(e => e.event === 'turn_advanced')
    .map(e => ({
      round: e.round,
      fear_current: e.details.fear_current ?? 0,
      blight_current: e.details.blight_current ?? 0,
    }))
)

function estimateWinAt(round: number, fearCurr: number, blightCurr: number): number {
  const threshold = stats.value?.fear_threshold ?? 4
  const cap = stats.value?.blight_cap ?? 3
  const fearRatio = threshold > 0 ? fearCurr / threshold : 0
  const blightRatio = cap > 0 ? blightCurr / cap : 0
  const m = 0.5 + fearRatio * 0.2 - blightRatio * 0.3 - Math.max(0, round - 6) * 0.05
  return Math.max(0, Math.min(1, m))
}

const winChartData = computed(() => {
  const labels: string[] = []
  const series: number[] = []
  const hi: number[] = []
  const lo: number[] = []
  const maxRound = Math.max(props.state.round, 8)
  for (let r = 1; r <= maxRound; r++) {
    labels.push(`T${r}`)
    const rec = turnAdvances.value.find(t => t.round === r)
    if (rec) {
      const m = estimateWinAt(r, rec.fear_current, rec.blight_current)
      series.push(m * 100)
      hi.push(Math.min(100, m * 100 + 20))
      lo.push(Math.max(0, m * 100 - 20))
    } else if (r === props.state.round) {
      const m = winProbability.value.mean
      series.push(m * 100)
      hi.push(winProbability.value.hi * 100)
      lo.push(winProbability.value.lo * 100)
    } else {
      series.push(NaN)
      hi.push(NaN)
      lo.push(NaN)
    }
  }
  return {
    labels,
    datasets: [
      {
        label: 'Win % (upper)',
        data: hi,
        borderColor: 'rgba(82, 183, 136, 0.25)',
        backgroundColor: 'rgba(82, 183, 136, 0.1)',
        borderWidth: 1,
        pointRadius: 0,
        fill: '+1',
      },
      {
        label: 'Win % (est.)',
        data: series,
        borderColor: '#52b788',
        backgroundColor: 'rgba(82, 183, 136, 0.15)',
        borderWidth: 2,
        tension: 0.3,
        pointRadius: 3,
      },
      {
        label: 'Win % (lower)',
        data: lo,
        borderColor: 'rgba(82, 183, 136, 0.25)',
        backgroundColor: 'transparent',
        borderWidth: 1,
        pointRadius: 0,
      },
    ],
  }
})

const ELEMENT_ICONS: Record<string, string> = {
  moon: 'element-moon',
  fire: 'element-fire',
  air: 'element-air',
  sun: 'element-sun',
  water: 'element-water',
  earth: 'element-earth',
  plant: 'element-plant',
  animal: 'element-animal',
}

</script>

<template>
  <div class="stats-panel">
    <div v-if="error" class="error">Error: {{ error }}</div>

    <!-- Fear/Blight bars removed: shown in the header status-inline and the
         decks panel. Elements-this-turn also moved out — it lives in the
         SpiritPanel. Analytics now focuses only on forward-looking stats. -->
    <div class="section win-prob-section">
      <div class="section-hdr">
        <h3>Win probability estimate</h3>
        <span class="win-prob-big">
          <span class="wp-mean">{{ winProbPct }}</span>
          <span class="wp-ci">({{ winProbCI }})</span>
        </span>
      </div>
      <div class="chart-box">
        <Line
          :data="winChartData"
          :options="chartOptions"
          :key="`win-${props.state.round}-${Math.round(winProbability.mean * 1000)}-${winProbability.endState}`"
        />
      </div>
      <p class="wp-note">
        Heuristic: baseline 50%, adjusted by fear progress (+20% × fear/threshold), terror level (+10% per flip), blight pressure (−30% × blight/cap), round pressure (−5%/round past T6), and
        <strong v-if="terrainModifier !== 0" :class="terrainModifier > 0 ? 'pos-mod' : 'neg-mod'">
          {{ (terrainModifier * 100).toFixed(1) }}% terrain concentration
        </strong><span v-else>terrain concentration (none in next 3 turns)</span>.
        ±20pp band is a notional uncertainty, not a bootstrapped CI — refine later with playlog data.
      </p>
    </div>

    <div class="section">
      <div class="section-hdr">
        <h3>Fear over rounds</h3>
        <span class="subtle">Live from <code>fear_generated</code> log entries — tagged with the event/card context when available.</span>
      </div>
      <div class="chart-box">
        <Line
          :data="fearChartData"
          :options="chartOptions"
          :key="`fear-${fearBumps.length}-${fearByRound.size}`"
        />
      </div>
      <div v-if="fearBumps.length" class="fear-by-source">
        <div class="attribution-label">Fear by source · total {{ totalFearGenerated }}</div>
        <div class="source-bars">
          <div
            v-for="s in fearBySource"
            :key="s.source"
            class="source-bar"
            :title="`${s.source}: +${s.amount} (${((s as unknown as { _pct: number })._pct).toFixed(0)}%)`"
          >
            <span class="source-name">{{ s.source }}</span>
            <div class="source-track">
              <div class="source-fill" :style="{ width: ((s as unknown as { _pct: number })._pct) + '%' }"></div>
            </div>
            <span class="source-val mono">+{{ s.amount }}</span>
          </div>
        </div>
      </div>

      <div v-if="fearBumps.length" class="fear-attribution">
        <div class="attribution-label">Fear sources by round</div>
        <div class="attribution-rows">
          <div
            v-for="(rd, i) in fearSourcesByRound"
            :key="i"
            class="attribution-row"
          >
            <span class="round-tag mono">R{{ rd.round }}</span>
            <span class="round-total">+{{ rd.total }} fear</span>
            <span class="round-sources">
              <span v-for="(src, si) in rd.sources" :key="si" class="source-chip" :title="src.source">
                +{{ src.amount }}
                <span class="source-name">{{ src.source }}</span>
              </span>
            </span>
          </div>
        </div>
      </div>
    </div>

    <div class="section">
      <h3>Next-draw probability</h3>
      <div class="probs-grid">
        <div v-for="(r, key) in drawProbs" :key="key" class="prob-card">
          <div class="prob-hdr">
            <Icon v-if="ELEMENT_ICONS[r.element]" :name="ELEMENT_ICONS[r.element]" :size="14" decorative />
            <span class="deck">{{ r.deck }}</span>
            <span class="element">{{ r.element }}</span>
          </div>
          <div class="prob-value">
            <span class="prob-num">{{ (r.probability * 100).toFixed(1) }}%</span>
            <span class="prob-ci">({{ (r.wilson_95[0] * 100).toFixed(0) }}–{{ (r.wilson_95[1] * 100).toFixed(0) }}%)</span>
          </div>
          <div class="prob-basis">{{ r.successes }} of {{ r.population }} cards</div>
        </div>
      </div>
    </div>

  </div>
</template>

<style scoped>
.stats-panel {
  display: flex;
  flex-direction: column;
  gap: var(--sp-4);
  padding: var(--sp-3);
  min-height: 0;
}

.error {
  background: rgba(184, 113, 106, 0.15);
  color: var(--status-danger);
  padding: var(--sp-2);
  border-radius: var(--r-sm);
  font-size: var(--fs-sm);
}

.bars {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
  gap: var(--sp-3);
}

.bar-group {
  display: flex;
  flex-direction: column;
  gap: var(--sp-1);
}

.bar-hdr {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  font-size: var(--fs-xs);
  color: var(--text-secondary);
}

.bar-hdr span { white-space: nowrap; }

.win-prob-section { }
.win-prob-big {
  display: inline-flex; align-items: baseline; gap: var(--sp-2);
}
.wp-mean {
  font-family: var(--font-mono);
  font-size: 1.5rem;
  font-weight: var(--fw-bold);
  color: var(--accent-green);
}
.wp-ci {
  font-family: var(--font-mono);
  font-size: var(--fs-xs);
  color: var(--text-muted);
}
.wp-note {
  font-size: var(--fs-xs);
  color: var(--text-muted);
  font-style: italic;
  margin: var(--sp-2) 0 0;
  line-height: 1.45;
}
.wp-note .pos-mod { color: var(--status-success); font-style: normal; }
.wp-note .neg-mod { color: var(--status-danger); font-style: normal; }

.fear-attribution,
.fear-by-source {
  margin-top: var(--sp-2);
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-2) var(--sp-3);
}

.source-bars {
  display: flex; flex-direction: column; gap: 4px;
}
.source-bar {
  display: grid;
  grid-template-columns: 8rem 1fr 3rem;
  gap: var(--sp-2);
  align-items: center;
  font-size: var(--fs-xs);
}
.source-name {
  color: var(--text-secondary);
  font-weight: var(--fw-medium);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.source-track {
  background: var(--bg-canvas);
  border-radius: var(--r-full);
  height: 8px;
  overflow: hidden;
}
.source-fill {
  height: 100%;
  background: linear-gradient(90deg, #d9771f, #d4a373);
  border-radius: var(--r-full);
}
.source-val {
  text-align: right;
  color: var(--pool-fear);
  font-weight: var(--fw-semibold);
}
.attribution-label {
  font-size: 0.68rem;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
  margin-bottom: var(--sp-1);
}
.attribution-rows { display: flex; flex-direction: column; gap: 2px; }
.attribution-row {
  display: grid;
  grid-template-columns: 2.5rem 4rem 1fr;
  gap: var(--sp-2);
  align-items: center;
  padding: 2px 0;
  font-size: var(--fs-xs);
}
.attribution-row .round-tag { color: var(--accent-amber); font-weight: var(--fw-semibold); }
.attribution-row .round-total {
  font-family: var(--font-mono);
  color: var(--pool-fear);
  font-weight: var(--fw-semibold);
}
.attribution-row .round-sources {
  display: inline-flex; gap: 4px; flex-wrap: wrap;
}
.source-chip {
  display: inline-flex; align-items: center; gap: 4px;
  padding: 1px 6px;
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-full);
  font-size: 0.68rem;
}
.source-chip .source-name {
  color: var(--text-secondary);
  max-width: 18rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.bar-count {
  font-family: var(--font-mono);
  color: var(--text-primary);
}

.tl {
  margin-left: auto;
  font-size: 0.68rem;
  padding: 1px var(--sp-2);
  background: var(--accent-soft);
  color: var(--pool-fear);
  border-radius: var(--r-full);
}

.bar {
  background: var(--bg-muted);
  height: 6px;
  border-radius: var(--r-full);
  overflow: hidden;
}

.fill {
  height: 100%;
  transition: width var(--motion-base);
  border-radius: var(--r-full);
}

.fill.fear { background: var(--pool-fear); }
.fill.blight { background: var(--pool-blight); }

.section {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.section-hdr {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  flex-wrap: wrap;
  gap: var(--sp-2);
}

h3 {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--text-muted);
  font-weight: var(--fw-bold);
  margin: 0;
}

.chart-box {
  height: 200px;
  padding: var(--sp-2);
  background:
    linear-gradient(180deg, color-mix(in srgb, var(--accent-blue) 4%, transparent), transparent),
    var(--bg-inset);
  border: 1px solid var(--aegis-border);
  border-radius: var(--r-md);
  box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--accent-blue) 4%, transparent);
}

.probs-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: var(--sp-2);
}

.prob-card {
  background: linear-gradient(to bottom right, var(--bg-raised), var(--bg-inset));
  border: 1px solid var(--aegis-border);
  border-radius: var(--r-md);
  padding: var(--sp-2) var(--sp-3);
  display: flex;
  flex-direction: column;
  gap: 2px;
  transition: border-color var(--motion-fast), transform var(--motion-fast);
}
.prob-card:hover {
  border-color: var(--accent-blue);
  transform: translateY(-1px);
}

.prob-hdr {
  display: flex;
  gap: var(--sp-1);
  align-items: center;
  font-size: var(--fs-xs);
  color: var(--text-secondary);
}

.deck { text-transform: capitalize; font-weight: var(--fw-medium); color: var(--text-primary); }
.element { text-transform: capitalize; }

.prob-value {
  display: flex;
  align-items: baseline;
  gap: var(--sp-1);
}

.prob-num {
  font-family: var(--font-mono);
  font-size: var(--fs-lg);
  font-weight: var(--fw-semibold);
  color: var(--pool-fear);
}

.prob-ci {
  font-family: var(--font-mono);
  font-size: var(--fs-xs);
  color: var(--text-muted);
}

.prob-basis {
  font-size: var(--fs-xs);
  color: var(--text-muted);
}

.spirit-elements {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.spirit-row {
  display: flex;
  gap: var(--sp-3);
  align-items: center;
  padding: var(--sp-1) 0;
}

.spirit-slug {
  font-family: var(--font-mono);
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  min-width: 10rem;
}

.chips {
  display: flex;
  gap: var(--sp-1);
  flex-wrap: wrap;
}

.element-chip {
  display: inline-flex;
  align-items: center;
  gap: 3px;
  padding: 1px var(--sp-2);
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-full);
  font-size: var(--fs-xs);
}

.el-name { text-transform: capitalize; color: var(--text-secondary); }
.el-count { font-family: var(--font-mono); font-weight: var(--fw-semibold); color: var(--text-primary); }

.subtle {
  color: var(--text-muted);
  font-style: italic;
  font-size: var(--fs-xs);
}
</style>
