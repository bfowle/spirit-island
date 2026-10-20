<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import type { GameState } from '../types'
import { computeWinProb, fearPct, blightPct, type WinProbResult } from '../lib/winprob'
import { fetchSpiritAffinity, type SpiritAffinityMap } from '../api'
import Icon from './Icon.vue'
import UiIcon from './UiIcon.vue'

/**
 * Tableside always-visible status bar. Pinned beneath the PhaseStepper so the
 * player can glance up and see the game's pulse without scrolling:
 *   Round · Phase · Fear % (pool + terror) · Blight % (+ flipped) · Win-prob · end-state
 *
 * Uses the shared `computeWinProb` so the number is canonical with the charts
 * and the end-banner.
 */

const props = defineProps<{ state: GameState }>()

const affinityMap = ref<SpiritAffinityMap | null>(null)
onMounted(async () => {
  try {
    affinityMap.value = await fetchSpiritAffinity()
  } catch { /* non-blocking */ }
})

const wp = computed<WinProbResult>(() => computeWinProb(props.state, affinityMap.value))
const fearBarPct = computed(() => fearPct(props.state))
const blightBarPct = computed(() => blightPct(props.state))

const winPct = computed(() => `${Math.round(wp.value.mean * 100)}%`)
const winClass = computed(() => {
  if (wp.value.endState === 'won') return 'state-won'
  if (wp.value.endState === 'lost') return 'state-lost'
  if (wp.value.endState === 'imminent') return 'state-imminent'
  if (wp.value.mean >= 0.75) return 'state-good'
  if (wp.value.mean <= 0.35) return 'state-bad'
  return 'state-neutral'
})

const phaseLabel = computed(() => {
  const p = props.state.phase
  return p.charAt(0).toUpperCase() + p.slice(1)
})
</script>

<template>
  <div class="sticky-status" :class="winClass">
    <div class="status-left">
      <span class="round-pill">R<strong>{{ state.round }}</strong></span>
      <span class="phase-pill">{{ phaseLabel }}</span>
    </div>

    <div class="status-pools">
      <div class="pool-group fear" :title="`Fear ${state.pools.fear_current}/${state.pools.fear_threshold} · Terror ${state.pools.terror_level}`">
        <Icon name="resource-fear" :size="14" decorative />
        <div class="pool-bar">
          <div class="pool-fill fear-fill" :style="{ width: fearBarPct + '%' }"></div>
        </div>
        <span class="pool-text mono">{{ state.pools.fear_current }}/{{ state.pools.fear_threshold }}</span>
        <span class="terror-pill" :data-tl="state.pools.terror_level">T{{ state.pools.terror_level }}</span>
      </div>

      <div class="pool-group blight" :title="`Blight ${state.pools.blight_current}/${state.pools.blight_cap}${state.pools.island_blighted ? ' · ISLAND BLIGHTED' : ''}`">
        <Icon name="resource-blight" :size="14" decorative />
        <div class="pool-bar">
          <div class="pool-fill blight-fill" :style="{ width: blightBarPct + '%' }"></div>
        </div>
        <span class="pool-text mono">{{ state.pools.blight_current }}/{{ state.pools.blight_cap }}</span>
        <UiIcon v-if="state.pools.island_blighted" name="moon" :size="12" class="blighted-tag" label="Blighted island" />
      </div>
    </div>

    <div class="status-right">
      <div class="winprob-display" :title="wp.reason">
        <span class="winprob-label">Win</span>
        <span class="winprob-value mono">{{ winPct }}</span>
        <span v-if="wp.endState !== 'in-progress'" class="winprob-state">
          <span v-if="wp.endState === 'won'"><UiIcon name="trophy" :size="12" decorative /> WON</span>
          <span v-else-if="wp.endState === 'lost'"><UiIcon name="skull" :size="12" decorative /> LOST</span>
          <span v-else-if="wp.endState === 'imminent'"><UiIcon name="target" :size="12" decorative /> IMMINENT</span>
        </span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.sticky-status {
  position: sticky;
  top: 0;
  z-index: 48;
  display: grid;
  grid-template-columns: auto 1fr auto;
  align-items: center;
  gap: var(--sp-3);
  padding: var(--sp-2) var(--sp-4);
  background: linear-gradient(180deg, var(--bg-surface) 0%, rgba(26, 26, 46, 0.96) 100%);
  backdrop-filter: blur(8px);
  border-bottom: 2px solid var(--border-subtle);
  border-radius: var(--r-md);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  transition: border-color var(--motion-base), background var(--motion-base);
}
.sticky-status.state-won { border-bottom-color: var(--status-success); }
.sticky-status.state-lost { border-bottom-color: var(--status-danger); }
.sticky-status.state-imminent {
  border-bottom-color: var(--status-success);
  background: linear-gradient(180deg, rgba(82, 183, 136, 0.12) 0%, rgba(26, 26, 46, 0.96) 100%);
}

.status-left { display: inline-flex; align-items: center; gap: var(--sp-2); }
.round-pill {
  font-family: var(--font-mono);
  font-size: var(--fs-sm);
  padding: 3px var(--sp-2);
  background: var(--bg-muted);
  border-radius: var(--r-md);
  color: var(--text-secondary);
}
.round-pill strong { color: var(--accent-amber); font-weight: var(--fw-bold); }
.phase-pill {
  font-size: var(--fs-xs);
  padding: 3px var(--sp-2);
  background: rgba(199, 125, 255, 0.15);
  color: var(--accent-purple);
  border-radius: var(--r-md);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  font-weight: var(--fw-semibold);
}

.status-pools {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--sp-4);
}
.pool-group {
  display: inline-grid;
  grid-template-columns: auto 1fr auto auto;
  gap: var(--sp-2);
  align-items: center;
  font-size: var(--fs-xs);
}
.pool-bar {
  height: 8px;
  background: var(--bg-inset);
  border-radius: var(--r-full);
  overflow: hidden;
  min-width: 4rem;
}
.pool-fill { height: 100%; border-radius: var(--r-full); transition: width var(--motion-base); }
.fear-fill { background: linear-gradient(90deg, #d9771f, var(--pool-fear)); }
.blight-fill { background: linear-gradient(90deg, #6b4423, var(--accent-purple)); }
.pool-text {
  color: var(--text-primary);
  font-size: var(--fs-xs);
  font-weight: var(--fw-semibold);
}
.terror-pill {
  font-family: var(--font-mono);
  font-size: 0.7rem;
  font-weight: var(--fw-bold);
  padding: 1px 6px;
  border-radius: var(--r-sm);
  background: var(--bg-canvas);
}
.terror-pill[data-tl="1"] { color: var(--accent-blue); }
.terror-pill[data-tl="2"] { color: var(--accent-amber); }
.terror-pill[data-tl="3"] { color: var(--accent-red); }
.blighted-tag { font-size: 0.85rem; }

.status-right { display: inline-flex; align-items: center; }
.winprob-display {
  display: inline-flex;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-1) var(--sp-3);
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
}
.winprob-label {
  font-size: 0.68rem;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-muted);
}
.winprob-value {
  font-size: var(--fs-lg);
  font-weight: var(--fw-bold);
}
.state-won .winprob-value,
.state-imminent .winprob-value,
.state-good .winprob-value { color: var(--status-success); }
.state-lost .winprob-value,
.state-bad .winprob-value { color: var(--status-danger); }
.state-neutral .winprob-value { color: var(--text-primary); }

.winprob-state {
  font-family: var(--font-mono);
  font-size: 0.72rem;
  font-weight: var(--fw-bold);
  letter-spacing: 0.08em;
  padding: 2px var(--sp-2);
  border-radius: var(--r-sm);
}
.state-won .winprob-state { background: rgba(82, 183, 136, 0.2); color: var(--status-success); }
.state-lost .winprob-state { background: rgba(184, 113, 106, 0.2); color: var(--status-danger); }
.state-imminent .winprob-state {
  background: rgba(82, 183, 136, 0.2);
  color: var(--status-success);
  animation: imminent-pulse 1.5s ease-in-out infinite;
}
@keyframes imminent-pulse {
  0%, 100% { box-shadow: 0 0 0 0 rgba(82, 183, 136, 0.5); }
  50% { box-shadow: 0 0 0 4px rgba(82, 183, 136, 0); }
}

@media (max-width: 900px) {
  .sticky-status { grid-template-columns: 1fr; }
  .status-pools { grid-template-columns: 1fr; }
}
</style>
