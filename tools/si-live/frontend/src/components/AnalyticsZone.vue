<script setup lang="ts">
/**
 * AnalyticsZone.vue — Win probability, fear trajectory, terror progression.
 *
 * A compact analytics panel showing key game metrics over time. Replaces
 * the verbose StatsPanel in non-stats phases with a focused dashboard.
 */
import { computed } from 'vue'
import type { GameState } from '../types'

const props = defineProps<{
  state: GameState
  winProb: { mean: number; lo: number; hi: number }
  winProbHistory: number[]
}>()

// Fear trajectory: how many fear cards earned per round
const fearTrajectory = computed(() => {
  const log = (props.state.log ?? []) as Array<{ round: number; event: string }>
  const byRound: number[] = []
  
  for (let r = 1; r <= props.state.round; r++) {
    const earned = log.filter(e => e.round === r && e.event === 'fear_card_earned').length
    byRound.push(earned)
  }
  return byRound
})

// Terror progression: which rounds hit T2, T3
const terrorMilestones = computed(() => {
  const log = (props.state.log ?? []) as Array<{ round: number; event: string; details: { to_level?: number } }>
  const milestones: { level: number; round: number }[] = []
  
  log.forEach(e => {
    if (e.event === 'terror_advanced' && e.details.to_level) {
      milestones.push({ level: e.details.to_level, round: e.round })
    }
  })
  return milestones
})

// Generate SVG sparkline path
function generateSparkline(data: number[], width: number, height: number): string {
  if (data.length < 2) return ''
  const padding = 2
  const maxVal = Math.max(...data, 0.01)
  const step = (width - padding * 2) / (data.length - 1)
  
  let path = ''
  data.forEach((v, i) => {
    const x = padding + i * step
    const y = height - padding - ((v / maxVal) * (height - padding * 2))
    path += i === 0 ? `M ${x} ${y}` : ` L ${x} ${y}`
  })
  return path
}

const winProbPath = computed(() => generateSparkline(props.winProbHistory, 100, 32))
const fearPath = computed(() => generateSparkline(fearTrajectory.value, 80, 24))

// Win prob color based on value
const winProbColor = computed(() => {
  const p = props.winProb.mean
  if (p >= 0.7) return 'var(--accent-green)'
  if (p >= 0.4) return 'var(--accent-amber)'
  return 'var(--accent-red)'
})
</script>

<template>
  <div class="analytics-zone">
    <!-- Win Probability Card -->
    <div class="analytics-card win-prob-card">
      <div class="card-header">
        <span class="card-label">Win Probability</span>
        <span class="card-round">R{{ state.round }}</span>
      </div>
      <div class="win-prob-main">
        <span class="wp-value" :style="{ color: winProbColor }">
          {{ (winProb.mean * 100).toFixed(0) }}%
        </span>
        <svg class="wp-sparkline" viewBox="0 0 100 32" preserveAspectRatio="none">
          <path
            :d="winProbPath"
            fill="none"
            :stroke="winProbColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </div>
      <div class="wp-confidence">
        <span class="ci-range">{{ (winProb.lo * 100).toFixed(0) }}–{{ (winProb.hi * 100).toFixed(0) }}%</span>
        <span class="ci-label">confidence</span>
      </div>
    </div>

    <!-- Terror Progression Card -->
    <div class="analytics-card terror-card">
      <div class="card-header">
        <span class="card-label">Terror Level</span>
        <span class="terror-current">T{{ state.pools.terror_level }}</span>
      </div>
      <div class="terror-track">
        <div
          v-for="level in [1, 2, 3]"
          :key="level"
          class="terror-marker"
          :class="{ active: state.pools.terror_level >= level, current: state.pools.terror_level === level }"
        >
          <span class="marker-level">T{{ level }}</span>
          <span v-if="terrorMilestones.find(m => m.level === level)" class="marker-round">
            R{{ terrorMilestones.find(m => m.level === level)?.round }}
          </span>
        </div>
      </div>
    </div>

    <!-- Fear Trajectory Card -->
    <div class="analytics-card fear-card">
      <div class="card-header">
        <span class="card-label">Fear Earned</span>
        <span class="fear-total">{{ state.fear_deck?.earned.length ?? 0 }} cards</span>
      </div>
      <div class="fear-main">
        <svg class="fear-sparkline" viewBox="0 0 80 24" preserveAspectRatio="none">
          <path
            :d="fearPath"
            fill="none"
            stroke="var(--accent-amber)"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
        <div class="fear-per-round">
          <span v-for="(f, i) in fearTrajectory.slice(-4)" :key="i" class="fpr-dot" :class="{ high: f >= 2 }">
            {{ f }}
          </span>
        </div>
      </div>
    </div>

    <!-- Blight Tracker Card -->
    <div class="analytics-card blight-card">
      <div class="card-header">
        <span class="card-label">Blight</span>
        <span class="blight-status" :class="{ danger: state.pools.blight_current >= (state.pools.blight_cap - 1) }">
          {{ state.pools.blight_current }}/{{ state.pools.blight_cap }}
        </span>
      </div>
      <div class="blight-bar">
        <div
          class="blight-fill"
          :style="{ width: (state.pools.blight_cap ? (state.pools.blight_current / state.pools.blight_cap) * 100 : 0) + '%' }"
          :class="{ danger: state.pools.blight_current >= (state.pools.blight_cap - 1) }"
        ></div>
      </div>
      <div v-if="state.pools.island_blighted" class="blighted-badge">Island Blighted</div>
    </div>
  </div>
</template>

<style scoped>
.analytics-zone {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--sp-3);
}

.analytics-card {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-3);
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.card-label {
  font-size: var(--fs-xxs);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-muted);
  font-weight: var(--fw-semibold);
}

.card-round {
  font-size: var(--fs-xs);
  font-family: var(--font-mono);
  color: var(--text-secondary);
}

/* Win Probability */
.win-prob-card {
  border-left: 3px solid var(--accent-green);
}

.win-prob-main {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
}

.wp-value {
  font-family: var(--font-mono);
  font-size: 28px;
  font-weight: var(--fw-bold);
  line-height: 1;
}

.wp-sparkline {
  flex: 1;
  height: 32px;
  max-width: 100px;
}

.wp-confidence {
  display: flex;
  gap: var(--sp-2);
  font-size: var(--fs-xxs);
  color: var(--text-muted);
}

.ci-range {
  font-family: var(--font-mono);
}

/* Terror */
.terror-card {
  border-left: 3px solid var(--accent-purple);
}

.terror-current {
  font-family: var(--font-mono);
  font-size: var(--fs-md);
  font-weight: var(--fw-bold);
  color: var(--accent-purple);
}

.terror-track {
  display: flex;
  gap: var(--sp-2);
}

.terror-marker {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  padding: var(--sp-1);
  background: var(--bg-muted);
  border-radius: var(--r-sm);
  opacity: 0.4;
  transition: all var(--motion-fast);
}

.terror-marker.active {
  opacity: 1;
  background: rgba(199, 125, 255, 0.15);
}

.terror-marker.current {
  border: 1px solid var(--accent-purple);
  box-shadow: 0 0 8px rgba(199, 125, 255, 0.3);
}

.marker-level {
  font-size: var(--fs-xs);
  font-weight: var(--fw-bold);
  color: var(--accent-purple);
}

.marker-round {
  font-size: var(--fs-xxs);
  font-family: var(--font-mono);
  color: var(--text-muted);
}

/* Fear */
.fear-card {
  border-left: 3px solid var(--accent-amber);
}

.fear-total {
  font-size: var(--fs-xs);
  font-family: var(--font-mono);
  color: var(--accent-amber);
}

.fear-main {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.fear-sparkline {
  flex: 1;
  height: 24px;
  max-width: 80px;
}

.fear-per-round {
  display: flex;
  gap: 4px;
}

.fpr-dot {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: var(--fs-xxs);
  font-family: var(--font-mono);
  font-weight: var(--fw-bold);
  background: var(--bg-muted);
  border-radius: var(--r-sm);
  color: var(--text-muted);
}

.fpr-dot.high {
  background: rgba(233, 196, 106, 0.2);
  color: var(--accent-amber);
}

/* Blight */
.blight-card {
  border-left: 3px solid var(--accent-green);
}

.blight-status {
  font-family: var(--font-mono);
  font-size: var(--fs-sm);
  font-weight: var(--fw-semibold);
  color: var(--accent-green);
}

.blight-status.danger {
  color: var(--accent-red);
}

.blight-bar {
  height: 8px;
  background: var(--bg-canvas);
  border-radius: var(--r-full);
  overflow: hidden;
}

.blight-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--accent-green), var(--accent-teal));
  border-radius: var(--r-full);
  transition: width var(--motion-base);
}

.blight-fill.danger {
  background: linear-gradient(90deg, var(--accent-red), var(--accent-coral));
}

.blighted-badge {
  font-size: var(--fs-xxs);
  font-weight: var(--fw-semibold);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--accent-red);
  padding: var(--sp-1) var(--sp-2);
  background: rgba(220, 47, 2, 0.15);
  border-radius: var(--r-sm);
  text-align: center;
}

@media (max-width: 600px) {
  .analytics-zone {
    grid-template-columns: 1fr;
  }
}
</style>
