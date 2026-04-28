<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import type { GameState } from './types'
import { fetchState, saveState, fetchSpiritAffinity, type SpiritAffinityMap } from './api'
import { computeWinProb } from './lib/winprob'

import AppShell from './components/AppShell.vue'
import SetupWizard from './components/SetupWizard.vue'
import SavedGames from './components/SavedGames.vue'
import UiIcon from './components/UiIcon.vue'

/**
 * Top-level app — state store + cross-cutting watchers + modal management.
 *
 * The Aegis-style dashboard layout lives in AppShell.vue. This file is kept
 * deliberately thin: load state, persist it, run the watchers that auto-log
 * deltas / auto-bank fear / resync deck counters / detect end-state, and
 * open modals (Setup wizard / Saved games browser).
 */

const state = ref<GameState | null>(null)
const error = ref<string | null>(null)
const saving = ref(false)
const showWizard = ref(false)
const showSavedGames = ref(false)
let saveTimer: number | null = null

const density = ref<'compact' | 'comfortable'>('comfortable')
watch(density, (d) => {
  document.documentElement.dataset.density = d
}, { immediate: true })

// ─── Initial load + save debounce ───────────────────────────────────────────
onMounted(async () => {
  try {
    state.value = await fetchState()
  } catch (e) {
    error.value = (e as Error).message
  }
})

watch(state, (s) => {
  if (!s) return
  if (saveTimer !== null) window.clearTimeout(saveTimer)
  saveTimer = window.setTimeout(async () => {
    saving.value = true
    try {
      await saveState(s)
    } catch (e) {
      error.value = (e as Error).message
    } finally {
      saving.value = false
    }
  }, 400)
}, { deep: true })

// ─── Log pipeline ───────────────────────────────────────────────────────────
function appendLog(event: string, details: Record<string, unknown>) {
  if (!state.value) return
  const log = (state.value.log as Array<{ round: number; event: string; details: Record<string, unknown> }>) ?? []
  state.value.log = [...log, { round: state.value.round, event, details }] as unknown as GameState['log']
}

// ─── Pool watcher: auto-log fear/blight deltas + auto-bank fear cards ───────
let lastFear = 0
let lastBlight = 0
let lastSuppressRound = -1
watch(
  () => state.value?.pools,
  (pools) => {
    if (!pools || !state.value) return
    const round = state.value.round
    if (lastSuppressRound !== round) {
      lastFear = pools.fear_current
      lastBlight = pools.blight_current
      lastSuppressRound = round
      return
    }
    const fearDelta = pools.fear_current - lastFear
    const blightDelta = pools.blight_current - lastBlight
    if (fearDelta > 0) appendLog('fear_generated', { amount: fearDelta })
    if (blightDelta > 0) appendLog('blight_added', { amount: blightDelta })
    else if (blightDelta < 0) appendLog('blight_removed', { amount: -blightDelta })

    const thr = pools.fear_threshold
    if (thr > 0 && pools.fear_current >= thr) {
      let remaining = pools.fear_current
      while (remaining >= thr) {
        remaining -= thr
        const fd = state.value.fear_deck ?? { deck_size: 9, tier_counts: [3, 3, 3] as [number, number, number], earned: [], resolved: [], unseen: 9 }
        state.value.fear_deck = {
          ...fd,
          earned: [...fd.earned, { name: '', terror_level: pools.terror_level, round }],
          unseen: Math.max(0, fd.unseen - 1),
        }
        appendLog('fear_card_earned', { terror_level: pools.terror_level, round, auto: true })
      }
      pools.fear_current = remaining
    }

    lastFear = pools.fear_current
    lastBlight = pools.blight_current
  },
  { deep: true },
)

// ─── Fear-deck watcher: advance terror + resync unseen ─────────────────────
watch(
  () => state.value?.fear_deck,
  (fd) => {
    if (!fd || !state.value) return
    const total = fd.resolved.length + fd.earned.length
    const third = Math.ceil(fd.deck_size / 3)
    const counts = fd.tier_counts ?? [third, third, Math.max(0, fd.deck_size - 2 * third)]
    const [t1, t2] = counts
    let nextLvl = 1
    if (total >= t1 + t2) nextLvl = 3
    else if (total >= t1) nextLvl = 2
    if (state.value.pools.terror_level < nextLvl) {
      state.value.pools.terror_level = nextLvl
      appendLog('terror_advanced', { to_level: nextLvl })
    }
    const expectedUnseen = Math.max(0, fd.deck_size - total)
    if (fd.unseen !== expectedUnseen) fd.unseen = expectedUnseen
  },
  { deep: true },
)

// ─── Game-end detection + banner ────────────────────────────────────────────
const affinityMap = ref<SpiritAffinityMap | null>(null)
onMounted(async () => {
  try { affinityMap.value = await fetchSpiritAffinity() } catch { /* optional */ }
})

const endResult = computed<'won' | 'lost' | 'imminent' | null>(() => {
  if (!state.value) return null
  const wp = computeWinProb(state.value, affinityMap.value)
  if (wp.endState === 'in-progress') return null
  return wp.endState
})
const gameOverBannerDismissed = ref(false)
const endBanner = computed(() => gameOverBannerDismissed.value ? null : endResult.value)

// ─── Event handlers passed down to AppShell ────────────────────────────────
function onGameStarted(newState: GameState) {
  state.value = newState
  gameOverBannerDismissed.value = false
}

function resetFearPool() {
  if (!state.value) return
  state.value.pools.fear_current = 0
  lastFear = 0
}

function bumpPool(pool: 'blight', delta: number) {
  if (!state.value) return
  if (pool === 'blight') {
    state.value.pools.blight_current = Math.max(0, (state.value.pools.blight_current ?? 0) + delta)
  }
}

const archiving = ref(false)
async function archiveCurrentGame() {
  archiving.value = true
  try {
    const res = await fetch('/api/saved-games/archive', { method: 'POST' })
    if (!res.ok) throw new Error(`archive failed: ${res.status}`)
    alert('Game archived to data/games/ — visible under Saved Games')
  } catch (e) {
    error.value = (e as Error).message
  } finally {
    archiving.value = false
  }
}

function exportCurrentGame() {
  if (!state.value) return
  const payload = {
    exported_at: new Date().toISOString(),
    schema_version: state.value.version,
    outcome: endResult.value,
    ...state.value,
  }
  const blob = new Blob([JSON.stringify(payload, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  const adv = state.value.setup.adversary ?? 'solo'
  const lvl = state.value.setup.level != null ? `-l${state.value.setup.level}` : ''
  const spirits = (state.value.setup.spirits ?? []).map(s => s.split('-')[0]).join(',') || 'solo'
  const date = new Date().toISOString().slice(0, 10)
  a.href = url
  a.download = `si-${adv}${lvl}-${spirits}-r${state.value.round}-${endResult.value ?? 'inprogress'}-${date}.json`
  a.click()
  URL.revokeObjectURL(url)
}
</script>

<template>
  <div v-if="error" class="banner error">{{ error }}</div>
  <div v-else-if="!state" class="banner">Loading…</div>

  <template v-else>
    <!-- Game-end banner — floats above the shell; dismissible -->
    <div v-if="endBanner" class="end-banner" :class="endBanner">
      <div class="end-main">
        <span class="end-icon">
          <UiIcon v-if="endBanner === 'won'" name="trophy" :size="28" decorative />
          <UiIcon v-else-if="endBanner === 'lost'" name="skull" :size="28" decorative />
          <UiIcon v-else name="target" :size="28" decorative />
        </span>
        <div>
          <div class="end-title">
            <template v-if="endBanner === 'won'">Victory — Fear deck exhausted</template>
            <template v-else-if="endBanner === 'lost'">Defeat — Blighted Island capped</template>
            <template v-else>Imminent Victory — Fear deck drawn</template>
          </div>
          <div class="end-sub">
            <template v-if="endBanner === 'won'">
              All {{ state.fear_deck?.deck_size ?? 9 }} fear cards resolved at Terror {{ state.pools.terror_level }} · Round {{ state.round }}
            </template>
            <template v-else-if="endBanner === 'lost'">
              Blight {{ state.pools.blight_current }} / {{ state.pools.blight_cap }} after flip · Round {{ state.round }}
            </template>
            <template v-else>
              Survive the current Invader phase to clinch it.
            </template>
          </div>
        </div>
      </div>
      <div class="end-actions">
        <span v-if="saving" class="saving-inline">saving…</span>
        <button class="ghost" @click="gameOverBannerDismissed = true">Dismiss</button>
      </div>
    </div>

    <AppShell
      v-model="state"
      @log-event="(event, details) => appendLog(event, details)"
      @reset-fear-pool="resetFearPool"
      @bump-pool="(pool, delta) => bumpPool(pool, delta)"
      @new-game="showWizard = true"
      @saved-games="showSavedGames = true"
      @archive="archiveCurrentGame"
      @export="exportCurrentGame"
    />

    <SetupWizard :show="showWizard" @close="showWizard = false" @game-started="onGameStarted" />
    <SavedGames :show="showSavedGames" @close="showSavedGames = false" @game-loaded="onGameStarted" />
  </template>
</template>

<style scoped>
.banner {
  padding: var(--sp-6);
  text-align: center;
  color: var(--text-secondary);
  font-size: var(--fs-sm);
}
.banner.error {
  background: rgba(239, 68, 68, 0.12);
  color: var(--status-danger);
  border-radius: var(--r-md);
  margin: var(--sp-4);
}

/* Game-end banner — floats at top of viewport */
.end-banner {
  position: fixed;
  top: 12px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 100;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-4);
  padding: var(--sp-3) var(--sp-4);
  border-radius: var(--r-lg);
  border: 2px solid;
  max-width: 720px;
  box-shadow: var(--shadow-lg);
  backdrop-filter: blur(10px);
  animation: end-slide-in 300ms cubic-bezier(0.2, 0.8, 0.2, 1);
}
@keyframes end-slide-in {
  from { opacity: 0; transform: translate(-50%, -20px); }
  to   { opacity: 1; transform: translate(-50%, 0); }
}

.end-banner.won {
  background: color-mix(in srgb, var(--status-success) 15%, var(--bg-surface));
  border-color: var(--status-success);
  box-shadow: var(--shadow-glow-green), var(--shadow-lg);
}
.end-banner.lost {
  background: color-mix(in srgb, var(--status-danger) 15%, var(--bg-surface));
  border-color: var(--status-danger);
  box-shadow: var(--shadow-glow-red), var(--shadow-lg);
}
.end-banner.imminent {
  background: color-mix(in srgb, var(--status-success) 8%, var(--bg-surface));
  border-color: rgba(16, 185, 129, 0.6);
  border-style: dashed;
}

.end-main { display: inline-flex; align-items: center; gap: var(--sp-3); min-width: 0; }
.end-icon { font-size: 1.75rem; flex: 0 0 auto; }
.end-title { font-size: 1rem; font-weight: var(--fw-bold); color: var(--text-white); }
.end-sub { font-size: var(--fs-xs); color: var(--text-secondary); margin-top: 2px; }

.end-actions { display: inline-flex; align-items: center; gap: var(--sp-2); }
.saving-inline { font-size: var(--fs-xs); color: var(--status-success); font-style: italic; }
</style>
