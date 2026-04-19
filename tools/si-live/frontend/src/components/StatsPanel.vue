<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { Chart, registerables } from 'chart.js'
import { Line } from 'vue-chartjs'
import type { GameState } from '../types'
import type { StatsResponse, DrawProbabilityResponse } from '../stats'
import { fetchStats, fetchDrawProbability } from '../stats'

Chart.register(...registerables)

const props = defineProps<{ state: GameState }>()

const stats = ref<StatsResponse | null>(null)
const error = ref<string | null>(null)
const drawProbs = ref<Record<string, DrawProbabilityResponse>>({})

async function refresh() {
  try {
    stats.value = await fetchStats()
    // Also refresh the draw-probability ticker for key elements
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

// Fear-over-rounds chart config
const fearChartData = computed(() => {
  if (!stats.value) return { labels: [], datasets: [] }
  // Build a running total series
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
        label: 'Fear generated this turn',
        data: perTurn,
        borderColor: 'rgba(220, 120, 50, 0.8)',
        backgroundColor: 'rgba(220, 120, 50, 0.25)',
        borderDash: [4, 4],
        tension: 0,
      },
      {
        label: 'Cumulative fear',
        data: cumulative,
        borderColor: '#da8',
        backgroundColor: 'rgba(220, 170, 136, 0.2)',
        tension: 0.3,
        fill: true,
      },
    ],
  }
})

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    y: { beginAtZero: true, ticks: { color: '#aaa' }, grid: { color: '#333' } },
    x: { ticks: { color: '#aaa' }, grid: { color: '#333' } },
  },
  plugins: {
    legend: { labels: { color: '#ccc' } },
  },
}

// Pool progress bars
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

// Per-spirit element display
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

    <div class="pools">
      <div class="pool">
        <div class="pool-hdr">
          Fear — {{ stats?.fear_current ?? 0 }} / {{ stats?.fear_threshold ?? 0 }}
          <span class="terror">(Terror {{ stats?.terror_level ?? 1 }})</span>
        </div>
        <div class="bar"><div class="fill fear" :style="{ width: fearPct + '%' }" /></div>
      </div>
      <div class="pool">
        <div class="pool-hdr">
          Blight — {{ stats?.blight_current ?? 0 }} / {{ stats?.blight_cap ?? 0 }}
        </div>
        <div class="bar"><div class="fill blight" :style="{ width: blightPct + '%' }" /></div>
      </div>
    </div>

    <div class="chart-wrap">
      <h3>Fear over rounds</h3>
      <div class="chart-box">
        <Line :data="fearChartData" :options="chartOptions" />
      </div>
      <p class="hint">Fear events must be written into <code>log</code> with <code>event: "fear_generated"</code> and <code>details.amount: N</code> to appear here.</p>
    </div>

    <div class="draw-probs">
      <h3>Next-draw probability</h3>
      <table>
        <thead>
          <tr><th>Pool</th><th>Element</th><th>Success count</th><th>P(next draw)</th><th>Wilson 95%</th></tr>
        </thead>
        <tbody>
          <tr v-for="(r, key) in drawProbs" :key="key">
            <td>{{ r.deck }}</td>
            <td>{{ r.element }}</td>
            <td>{{ r.successes }} / {{ r.population }}</td>
            <td><strong>{{ (r.probability * 100).toFixed(1) }}%</strong></td>
            <td>({{ (r.wilson_95[0] * 100).toFixed(0) }}–{{ (r.wilson_95[1] * 100).toFixed(0) }}%)</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="spirit-elements">
      <h3>Elements this turn (per spirit)</h3>
      <div v-for="s in spiritElements" :key="s.slug" class="spirit-row">
        <strong>{{ s.slug }}</strong>:
        <span v-if="!s.elements.length" class="muted">no elements tallied this turn</span>
        <span v-for="[el, n] in s.elements" :key="el" class="elem">{{ el }}×{{ n }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.stats-panel { background: #1a1a1e; border: 1px solid #333; border-radius: 6px; padding: .75rem; }
.error { background: #522; color: #fcc; padding: .5rem; border-radius: 4px; margin-bottom: .5rem; }
h3 { font-size: .95rem; margin: .5rem 0 .25rem; color: #eee; }
.pools { display: flex; gap: 1rem; flex-wrap: wrap; margin-bottom: .75rem; }
.pool { flex: 1 1 200px; }
.pool-hdr { font-size: .85rem; color: #ccc; margin-bottom: .25rem; }
.terror { color: #a86; font-size: .8rem; margin-left: .3rem; }
.bar { background: #2a2a30; height: 8px; border-radius: 4px; overflow: hidden; }
.fill { height: 100%; transition: width .3s; }
.fill.fear { background: #da8; }
.fill.blight { background: #8d4; }
.chart-wrap { margin: .75rem 0; }
.chart-box { height: 200px; }
.hint { font-size: .75rem; color: #777; font-style: italic; margin-top: .25rem; }
.draw-probs table { width: 100%; border-collapse: collapse; font-size: .85rem; }
.draw-probs th, .draw-probs td { padding: .25rem .5rem; border-bottom: 1px solid #2a2a2a; text-align: left; }
.draw-probs th { color: #999; font-weight: normal; }
.draw-probs td strong { color: #fca; }
.spirit-elements { margin-top: .75rem; }
.spirit-row { font-size: .85rem; margin: .25rem 0; }
.spirit-row .elem { padding: .1rem .35rem; background: #2a2a35; border-radius: 3px; font-family: monospace; margin-left: .4rem; font-size: .8rem; }
.muted { color: #666; font-style: italic; margin-left: .5rem; }
</style>
