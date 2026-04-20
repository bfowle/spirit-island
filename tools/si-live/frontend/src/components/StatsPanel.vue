<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { Chart, registerables } from 'chart.js'
import { Line } from 'vue-chartjs'
import type { GameState } from '../types'
import type { StatsResponse, DrawProbabilityResponse } from '../stats'
import { fetchStats, fetchDrawProbability } from '../stats'
import Icon from './Icon.vue'

Chart.register(...registerables)

const props = defineProps<{ state: GameState }>()

const stats = ref<StatsResponse | null>(null)
const error = ref<string | null>(null)
const drawProbs = ref<Record<string, DrawProbabilityResponse>>({})

async function refresh() {
  try {
    stats.value = await fetchStats()
    const elems = ['moon', 'fire', 'air']
    const results = await Promise.all(
      elems.flatMap(el => [
        ['minor', el] as const,
        ['major', el] as const,
      ]).map(async ([deck, element]) => {
        try {
          const r = await fetchDrawProbability(deck, element, 1, 1, 0)
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
    error.value = (e as Error).message
  }
}

onMounted(refresh)
watch(() => props.state, refresh, { deep: true })

const fearChartData = computed(() => {
  if (!stats.value) return { labels: [], datasets: [] }
  const byRound = [...stats.value.fear_by_round].sort((a, b) => a.round - b.round)
  const labels: string[] = []
  const perTurn: number[] = []
  const cumulative: number[] = []
  let running = 0
  for (let r = 1; r <= Math.max(props.state.round, 8); r++) {
    labels.push(`T${r}`)
    const entry = byRound.find(x => x.round === r)
    const v = entry?.fear_generated ?? 0
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

const fearPct = computed(() =>
  !stats.value || stats.value.fear_threshold === 0
    ? 0
    : Math.min(100, (stats.value.fear_current / stats.value.fear_threshold) * 100),
)
const blightPct = computed(() =>
  !stats.value || stats.value.blight_cap === 0
    ? 0
    : Math.min(100, (stats.value.blight_current / stats.value.blight_cap) * 100),
)

// Heuristic win-probability estimate.
// Model: base 50% at round 1. Fear progress lifts us (each threshold crossing
// ≈ +15%). Blight ratio hurts (each step toward cap ≈ −5%). Round pressure
// hurts slowly (−2% per round past 6). Wilson CI heuristic: width ≈ 1/√N
// where N is a notional "games of similar state" — we treat current state as
// n=12 worth of evidence, which gives a ~±25% band.
function clamp(v: number, lo = 0, hi = 1): number { return Math.max(lo, Math.min(hi, v)) }

const winProbability = computed(() => {
  if (!stats.value) return { mean: 0.5, lo: 0.25, hi: 0.75 }
  const s = stats.value
  const fearRatio = s.fear_threshold > 0 ? s.fear_current / s.fear_threshold : 0
  const blightRatio = s.blight_cap > 0 ? s.blight_current / s.blight_cap : 0
  const round = props.state.round
  let mean = 0.50
  mean += fearRatio * 0.20                          // fear progress helps
  mean += (s.terror_level - 1) * 0.10               // each terror flip ~+10%
  mean -= blightRatio * 0.30                        // blight hurts
  mean -= Math.max(0, round - 6) * 0.05             // round 7+ accumulates risk
  mean = clamp(mean)
  // Wilson-ish interval at notional n=12 observations of this state:
  const n = 12
  const z = 1.96
  const denom = 1 + (z * z) / n
  const center = (mean + (z * z) / (2 * n)) / denom
  const margin = (z * Math.sqrt((mean * (1 - mean)) / n + (z * z) / (4 * n * n))) / denom
  return {
    mean,
    lo: clamp(center - margin),
    hi: clamp(center + margin),
  }
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
  const fearRatio = (stats.value?.fear_threshold ?? 4) > 0 ? fearCurr / (stats.value!.fear_threshold) : 0
  const blightRatio = (stats.value?.blight_cap ?? 3) > 0 ? blightCurr / (stats.value!.blight_cap) : 0
  let m = 0.5 + fearRatio * 0.2 - blightRatio * 0.3 - Math.max(0, round - 6) * 0.05
  return clamp(m)
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

const spiritElements = computed(() => {
  if (!stats.value) return [] as { slug: string; elements: [string, number][] }[]
  return Object.entries(stats.value.elements_per_spirit).map(([slug, elems]) => ({
    slug,
    elements: Object.entries(elems).filter(([, n]) => n > 0),
  }))
})
</script>

<template>
  <div class="stats-panel">
    <div v-if="error" class="error">Error: {{ error }}</div>

    <div class="bars">
      <div class="bar-group">
        <div class="bar-hdr">
          <Icon name="resource-fear" :size="14" decorative />
          <span>Fear</span>
          <span class="bar-count">{{ stats?.fear_current ?? 0 }} / {{ stats?.fear_threshold ?? 0 }}</span>
          <span class="tl">Terror {{ stats?.terror_level ?? 1 }}</span>
        </div>
        <div class="bar"><div class="fill fear" :style="{ width: fearPct + '%' }" /></div>
      </div>
      <div class="bar-group">
        <div class="bar-hdr">
          <Icon name="resource-blight" :size="14" decorative />
          <span>Blight</span>
          <span class="bar-count">{{ stats?.blight_current ?? 0 }} / {{ stats?.blight_cap ?? 0 }}</span>
        </div>
        <div class="bar"><div class="fill blight" :style="{ width: blightPct + '%' }" /></div>
      </div>
    </div>

    <div class="section win-prob-section">
      <div class="section-hdr">
        <h3>Win probability estimate</h3>
        <span class="win-prob-big">
          <span class="wp-mean">{{ winProbPct }}</span>
          <span class="wp-ci">({{ winProbCI }})</span>
        </span>
      </div>
      <div class="chart-box">
        <Line :data="winChartData" :options="chartOptions" />
      </div>
      <p class="wp-note">
        Heuristic: baseline 50%, adjusted by fear progress (+20% × fear/threshold), terror level (+10% per flip), blight pressure (−30% × blight/cap), round pressure (−5%/round past T6). ±20pp band is a notional uncertainty, not a bootstrapped CI — refine later with playlog data.
      </p>
    </div>

    <div class="section">
      <div class="section-hdr">
        <h3>Fear over rounds</h3>
        <span class="subtle">Populated from <code>fear_generated</code> log entries (use the Quick-fear buttons in Turn Controller).</span>
      </div>
      <div class="chart-box">
        <Line :data="fearChartData" :options="chartOptions" />
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

    <div class="section">
      <h3>Elements this turn</h3>
      <div class="spirit-elements">
        <div v-for="s in spiritElements" :key="s.slug" class="spirit-row">
          <span class="spirit-slug">{{ s.slug }}</span>
          <div v-if="s.elements.length" class="chips">
            <span v-for="[el, n] in s.elements" :key="el" class="element-chip">
              <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="12" decorative />
              <span class="el-name">{{ el }}</span>
              <span class="el-count">×{{ n }}</span>
            </span>
          </div>
          <span v-else class="subtle">no elements tallied</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.stats-panel {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-lg);
  padding: var(--sp-4);
  display: flex;
  flex-direction: column;
  gap: var(--sp-4);
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
  font-size: 0.78rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  font-weight: var(--fw-semibold);
  margin: 0;
}

.chart-box {
  height: 200px;
  padding: var(--sp-2);
  background: var(--bg-canvas);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
}

.probs-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: var(--sp-2);
}

.prob-card {
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-2) var(--sp-3);
  display: flex;
  flex-direction: column;
  gap: 2px;
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
