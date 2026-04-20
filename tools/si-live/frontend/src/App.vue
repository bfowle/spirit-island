<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import type { GameState, Phase } from './types'
import { fetchState, saveState } from './api'
import Board from './components/Board.vue'
import SpiritPanel from './components/SpiritPanel.vue'
import Pools from './components/Pools.vue'
import StatsPanel from './components/StatsPanel.vue'
import SetupWizard from './components/SetupWizard.vue'
import SavedGames from './components/SavedGames.vue'
import TurnController from './components/TurnController.vue'
import InvaderDeck from './components/InvaderDeck.vue'
import FearDeck from './components/FearDeck.vue'
import EventDeck from './components/EventDeck.vue'
import Retrospective from './components/Retrospective.vue'
import PhaseStepper from './components/PhaseStepper.vue'
import TerrainTimeline from './components/TerrainTimeline.vue'
import StickyStatus from './components/StickyStatus.vue'

const state = ref<GameState | null>(null)
const error = ref<string | null>(null)
const saving = ref(false)
const showWizard = ref(false)
const showSavedGames = ref(false)
let saveTimer: number | null = null

// Active spirit tab for multi-spirit games
const activeSpiritTab = ref<string | null>(null)

// Stats drawer state
const statsDrawerOpen = ref(true)

// Active main tab
const activeMainTab = ref<'game' | 'analytics' | 'log'>('game')

function onGameStarted(newState: GameState) {
  state.value = newState
  const slugs = Object.keys(newState.spirits ?? {})
  activeSpiritTab.value = slugs[0] ?? null
}

const archiving = ref(false)
async function archiveCurrentGame() {
  archiving.value = true
  try {
    const res = await fetch('/api/saved-games/archive', { method: 'POST' })
    if (!res.ok) throw new Error(`archive failed: ${res.status}`)
    alert('Game archived to data/games/')
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

function appendLog(event: string, details: Record<string, unknown>) {
  if (!state.value) return
  const log = (state.value.log as Array<{ round: number; event: string; details: Record<string, unknown> }>) ?? []
  state.value.log = [...log, { round: state.value.round, event, details }] as unknown as GameState['log']
}

function resetFearPool() {
  if (!state.value) return
  state.value.pools.fear_current = 0
  lastFear = 0
}

function bumpPool(pool: 'blight', delta: number) {
  if (!state.value) return
  if (pool === 'blight') {
    const current = state.value.pools.blight_current ?? 0
    state.value.pools.blight_current = Math.max(0, current + delta)
  }
}

// Fear deck watcher
watch(
  () => state.value?.fear_deck,
  (fd) => {
    if (!fd || !state.value) return
    const total = fd.resolved.length + fd.earned.length
    const counts = fd.tier_counts ?? [Math.ceil(fd.deck_size / 3), Math.ceil(fd.deck_size / 3), Math.max(0, fd.deck_size - 2 * Math.ceil(fd.deck_size / 3))]
    const [t1, t2] = counts
    let nextLvl = 1
    if (total >= t1 + t2) nextLvl = 3
    else if (total >= t1) nextLvl = 2
    if (state.value.pools.terror_level < nextLvl) {
      state.value.pools.terror_level = nextLvl
      appendLog('terror_advanced', { to_level: nextLvl })
    }
    const expectedUnseen = Math.max(0, fd.deck_size - total)
    if (fd.unseen !== expectedUnseen) {
      fd.unseen = expectedUnseen
    }
  },
  { deep: true },
)

// Game-end detection
import { computeWinProb } from './lib/winprob'
import { fetchSpiritAffinity, type SpiritAffinityMap } from './api'

const affinityMap = ref<SpiritAffinityMap | null>(null)
onMounted(async () => {
  try {
    affinityMap.value = await fetchSpiritAffinity()
  } catch { /* non-blocking */ }
})

const endResult = computed<'won' | 'lost' | 'imminent' | null>(() => {
  if (!state.value) return null
  const wp = computeWinProb(state.value, affinityMap.value)
  if (wp.endState === 'in-progress') return null
  return wp.endState
})

const gameOverBannerDismissed = ref(false)
const endBanner = computed(() => {
  if (gameOverBannerDismissed.value) return null
  return endResult.value
})

// Auto-log fear/blight deltas + auto-bank fear cards
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
    if (fearDelta > 0) {
      appendLog('fear_generated', { amount: fearDelta })
    }
    if (blightDelta > 0) {
      appendLog('blight_added', { amount: blightDelta })
    } else if (blightDelta < 0) {
      appendLog('blight_removed', { amount: -blightDelta })
    }

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

      const fd2 = state.value.fear_deck
      if (fd2) {
        const total = fd2.resolved.length + fd2.earned.length
        const [t1, t2] = fd2.tier_counts ?? [Math.ceil(fd2.deck_size / 3), Math.ceil(fd2.deck_size / 3), 0]
        let nextLvl = 1
        if (total >= t1 + t2) nextLvl = 3
        else if (total >= t1) nextLvl = 2
        if (pools.terror_level < nextLvl) {
          pools.terror_level = nextLvl
          appendLog('terror_advanced', { to_level: nextLvl })
        }
      }
    }

    lastFear = pools.fear_current
    lastBlight = pools.blight_current
  },
  { deep: true },
)

onMounted(async () => {
  try {
    state.value = await fetchState()
    const slugs = Object.keys(state.value?.spirits ?? {})
    activeSpiritTab.value = slugs[0] ?? null
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

const matchupTag = computed(() => {
  if (!state.value) return ''
  const setup = state.value.setup
  const parts: string[] = []
  if (setup.adversary) {
    const advName = setup.adversary.replace(/-/g, ' ')
    parts.push(`${advName}${setup.level != null ? ` L${setup.level}` : ''}`)
  }
  if (setup.scenario) parts.push(setup.scenario.replace(/-/g, ' '))
  return parts.join(' / ')
})

function humanSlug(slug: string): string {
  return slug
    .split('-')
    .map(p => p.charAt(0).toUpperCase() + p.slice(1))
    .join(' ')
}

// Spirit slugs for tabs
const spiritSlugs = computed(() => Object.keys(state.value?.spirits ?? {}))

// Win probability for display
const currentWinProb = computed(() => {
  if (!state.value) return { mean: 0.5, lo: 0.3, hi: 0.7 }
  return computeWinProb(state.value, affinityMap.value)
})

// Phase display name
const phaseDisplayName = computed(() => {
  const p = state.value?.phase
  if (!p) return ''
  const names: Record<Phase, string> = {
    setup: 'Setup',
    growth: 'Spirit Growth',
    fast: 'Fast Powers',
    event: 'Event',
    fear: 'Fear',
    invader: 'Invader',
    slow: 'Slow Powers',
    timepasses: 'Time Passes',
    end: 'Game End',
  }
  return names[p] || p
})
</script>

<template>
  <div v-if="error" class="error-page">
    <div class="error-card">
      <div class="error-icon">!</div>
      <h2>Error Loading Game</h2>
      <p>{{ error }}</p>
      <button class="btn-primary" @click="error = null; showWizard = true">Start New Game</button>
    </div>
  </div>

  <div v-else-if="!state" class="loading-page">
    <div class="loading-spinner"></div>
    <p>Loading game state...</p>
  </div>

  <div v-else class="app-shell" :data-phase="state.phase">
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <!-- SIDEBAR NAVIGATION (Aegis-style)                                        -->
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <aside class="sidebar">
      <div class="sidebar-brand">
        <div class="brand-icon">SI</div>
        <span class="brand-text">Spirit Island</span>
      </div>

      <nav class="sidebar-nav">
        <button 
          class="nav-item" 
          :class="{ active: activeMainTab === 'game' }"
          @click="activeMainTab = 'game'"
        >
          <span class="nav-icon">&#9654;</span>
          <span class="nav-label">Game</span>
        </button>
        <button 
          class="nav-item" 
          :class="{ active: activeMainTab === 'analytics' }"
          @click="activeMainTab = 'analytics'"
        >
          <span class="nav-icon">&#9733;</span>
          <span class="nav-label">Analytics</span>
        </button>
        <button 
          class="nav-item" 
          :class="{ active: activeMainTab === 'log' }"
          @click="activeMainTab = 'log'"
        >
          <span class="nav-icon">&#9776;</span>
          <span class="nav-label">Log</span>
        </button>
      </nav>

      <div class="sidebar-spacer"></div>

      <div class="sidebar-actions">
        <button class="nav-item" @click="showSavedGames = true">
          <span class="nav-icon">&#128193;</span>
          <span class="nav-label">Saved</span>
        </button>
        <button class="nav-item" @click="showWizard = true">
          <span class="nav-icon">+</span>
          <span class="nav-label">New</span>
        </button>
      </div>
    </aside>

    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <!-- MAIN CONTENT AREA                                                       -->
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <main class="main-content">
      <!-- HEADER BAR -->
      <header class="content-header">
        <div class="header-left">
          <h1 class="page-title">{{ phaseDisplayName }}</h1>
          <div class="matchup-badge" v-if="matchupTag">{{ matchupTag }}</div>
        </div>

        <div class="header-center">
          <PhaseStepper v-model="state.phase" :round="state.round" />
        </div>

        <div class="header-right">
          <div class="quick-stats">
            <div class="quick-stat">
              <span class="stat-label">Round</span>
              <span class="stat-value">{{ state.round }}</span>
            </div>
            <div class="quick-stat fear">
              <span class="stat-label">Fear</span>
              <span class="stat-value">{{ state.pools.fear_current }}/{{ state.pools.fear_threshold }}</span>
            </div>
            <div class="quick-stat blight">
              <span class="stat-label">Blight</span>
              <span class="stat-value">{{ state.pools.blight_current }}/{{ state.pools.blight_cap }}</span>
            </div>
            <div class="quick-stat terror">
              <span class="stat-label">Terror</span>
              <span class="stat-value">{{ state.pools.terror_level }}</span>
            </div>
          </div>

          <div class="header-actions">
            <button class="btn-ghost" @click="archiveCurrentGame" :disabled="archiving">
              {{ archiving ? 'Saving...' : 'Archive' }}
            </button>
            <button class="btn-ghost" @click="exportCurrentGame">Export</button>
            <span v-if="saving" class="save-indicator">Saving...</span>
          </div>
        </div>
      </header>

      <!-- END GAME BANNERS -->
      <div v-if="endBanner === 'won'" class="game-banner won">
        <span class="banner-icon">&#10003;</span>
        <div class="banner-content">
          <strong>Victory!</strong>
          <span>Fear deck exhausted at Terror Level {{ state.pools.terror_level }}</span>
        </div>
        <button class="btn-ghost" @click="gameOverBannerDismissed = true">Dismiss</button>
      </div>
      <div v-else-if="endBanner === 'lost'" class="game-banner lost">
        <span class="banner-icon">&#10007;</span>
        <div class="banner-content">
          <strong>Defeat</strong>
          <span>The island has been blighted beyond recovery</span>
        </div>
        <button class="btn-ghost" @click="gameOverBannerDismissed = true">Dismiss</button>
      </div>

      <!-- MAIN DASHBOARD GRID -->
      <div class="dashboard-grid" v-if="activeMainTab === 'game'">
        <!-- LEFT COLUMN: Spirits -->
        <section class="panel spirits-panel">
          <div class="panel-header">
            <h2>Spirits</h2>
            <div class="spirit-tabs" v-if="spiritSlugs.length > 1">
              <button
                v-for="slug in spiritSlugs"
                :key="slug"
                class="spirit-tab"
                :class="{ active: activeSpiritTab === slug }"
                @click="activeSpiritTab = slug"
              >
                {{ humanSlug(slug).split(' ').slice(0, 2).join(' ') }}
              </button>
            </div>
          </div>

          <div class="panel-body">
            <SpiritPanel
              v-for="slug in spiritSlugs"
              v-show="activeSpiritTab === slug || spiritSlugs.length === 1"
              :key="slug"
              v-model="state.spirits[slug]"
              :slug="slug"
              :round="state.round"
              @log-event="(event, details) => appendLog(event, details)"
            />
          </div>
        </section>

        <!-- CENTER COLUMN: Board + Decks -->
        <section class="panel board-panel">
          <div class="panel-header">
            <h2>Island</h2>
            <div class="board-tabs">
              <span class="chip" v-for="b in (state.setup.boards ?? ['A'])" :key="b">Board {{ b }}</span>
            </div>
          </div>

          <div class="panel-body">
            <Board
              v-model="state.board"
              :boards="state.setup.boards ?? ['A']"
              :spirits="state.spirits"
              @log-event="(event, details) => appendLog(event, details)"
            />
          </div>
        </section>

        <!-- RIGHT COLUMN: Decks -->
        <section class="panel decks-panel">
          <div class="panel-header">
            <h2>Decks</h2>
          </div>

          <div class="panel-body decks-stack">
            <div class="deck-card invader-accent">
              <h3>Invader Deck</h3>
              <InvaderDeck v-model="state.invader_deck" @log-event="(event, details) => appendLog(event, details)" />
            </div>

            <div class="deck-card fear-accent">
              <h3>Fear Deck</h3>
              <FearDeck v-model="state.fear_deck" :terror-level="state.pools.terror_level" @log-event="(event, details) => appendLog(event, details)" />
            </div>

            <div class="deck-card event-accent" v-if="state.setup.scenario">
              <h3>Event Deck</h3>
              <EventDeck v-model="state.event_deck" @log-event="(event, details) => appendLog(event, details)" />
            </div>

            <div class="deck-card terrain-accent">
              <h3>Terrain Timeline</h3>
              <TerrainTimeline v-model="state.terrain_deck" :round="state.round" />
            </div>
          </div>
        </section>
      </div>

      <!-- ANALYTICS VIEW -->
      <div class="analytics-grid" v-if="activeMainTab === 'analytics'">
        <section class="panel analytics-panel">
          <div class="panel-header">
            <h2>Win Probability</h2>
          </div>
          <div class="panel-body">
            <div class="big-stat">
              <span class="big-value" :class="currentWinProb.mean > 0.5 ? 'positive' : 'negative'">
                {{ Math.round(currentWinProb.mean * 100) }}%
              </span>
              <span class="big-label">Estimated Win Rate</span>
            </div>
            <div class="confidence-interval">
              <span>{{ Math.round(currentWinProb.lo * 100) }}%</span>
              <div class="ci-bar">
                <div class="ci-fill" :style="{ left: (currentWinProb.lo * 100) + '%', width: ((currentWinProb.hi - currentWinProb.lo) * 100) + '%' }"></div>
                <div class="ci-marker" :style="{ left: (currentWinProb.mean * 100) + '%' }"></div>
              </div>
              <span>{{ Math.round(currentWinProb.hi * 100) }}%</span>
            </div>
          </div>
        </section>

        <section class="panel analytics-panel">
          <div class="panel-header">
            <h2>Game Statistics</h2>
          </div>
          <div class="panel-body">
            <StatsPanel :state="state" />
          </div>
        </section>

        <section class="panel analytics-panel wide">
          <div class="panel-header">
            <h2>Pools</h2>
          </div>
          <div class="panel-body">
            <Pools v-model="state.pools" @reset-fear="resetFearPool" @bump-pool="bumpPool" />
          </div>
        </section>
      </div>

      <!-- LOG VIEW -->
      <div class="log-grid" v-if="activeMainTab === 'log'">
        <section class="panel log-panel">
          <div class="panel-header">
            <h2>Game Retrospective</h2>
          </div>
          <div class="panel-body">
            <Retrospective :state="state" />
          </div>
        </section>
      </div>

      <!-- FOOTER: Turn Controller -->
      <footer class="content-footer">
        <TurnController v-model="state" />
      </footer>
    </main>

    <!-- Modals -->
    <SetupWizard :show="showWizard" @close="showWizard = false" @game-started="onGameStarted" />
    <SavedGames :show="showSavedGames" @close="showSavedGames = false" @game-loaded="onGameStarted" />
  </div>
</template>

<style scoped>
/* ═══════════════════════════════════════════════════════════════════════════ */
/* AEGIS-INSPIRED LAYOUT SHELL                                                  */
/* ═══════════════════════════════════════════════════════════════════════════ */

.app-shell {
  display: grid;
  grid-template-columns: 220px 1fr;
  height: 100vh;
  background: var(--bg-canvas);
  overflow: hidden;
}

/* ─── ERROR & LOADING STATES ─── */
.error-page,
.loading-page {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  gap: var(--sp-4);
  color: var(--text-secondary);
}

.error-card {
  background: var(--bg-surface);
  border: 1px solid var(--status-danger);
  border-radius: var(--r-lg);
  padding: var(--sp-6);
  text-align: center;
  max-width: 400px;
}

.error-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--r-full);
  background: rgba(220, 47, 2, 0.15);
  color: var(--status-danger);
  font-size: 24px;
  font-weight: bold;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto var(--sp-4);
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--border-subtle);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* SIDEBAR                                                                      */
/* ═══════════════════════════════════════════════════════════════════════════ */

.sidebar {
  background: var(--bg-surface);
  border-right: 1px solid var(--border-subtle);
  display: flex;
  flex-direction: column;
  padding: var(--sp-4) 0;
}

.sidebar-brand {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  padding: 0 var(--sp-4) var(--sp-4);
  border-bottom: 1px solid var(--border-subtle);
  margin-bottom: var(--sp-4);
}

.brand-icon {
  width: 36px;
  height: 36px;
  background: linear-gradient(135deg, var(--accent-purple), var(--accent-violet));
  border-radius: var(--r-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: var(--fw-bold);
  font-size: var(--fs-sm);
  color: white;
}

.brand-text {
  font-weight: var(--fw-semibold);
  font-size: var(--fs-md);
  color: var(--text-white);
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: var(--sp-1);
  padding: 0 var(--sp-2);
}

.nav-item {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  padding: var(--sp-2) var(--sp-3);
  background: transparent;
  border: none;
  border-radius: var(--r-md);
  color: var(--text-secondary);
  font-size: var(--fs-sm);
  cursor: pointer;
  transition: all var(--motion-fast);
  text-align: left;
}

.nav-item:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.nav-item.active {
  background: var(--accent-soft);
  color: var(--accent);
}

.nav-icon {
  width: 20px;
  text-align: center;
  font-size: var(--fs-base);
}

.sidebar-spacer {
  flex: 1;
}

.sidebar-actions {
  display: flex;
  flex-direction: column;
  gap: var(--sp-1);
  padding: var(--sp-4) var(--sp-2) 0;
  border-top: 1px solid var(--border-subtle);
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* MAIN CONTENT                                                                 */
/* ═══════════════════════════════════════════════════════════════════════════ */

.main-content {
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* ─── HEADER ─── */
.content-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-4);
  padding: var(--sp-3) var(--sp-4);
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
}

.page-title {
  font-size: var(--fs-lg);
  font-weight: var(--fw-semibold);
  color: var(--text-white);
  margin: 0;
}

.matchup-badge {
  font-size: var(--fs-xs);
  padding: 4px var(--sp-2);
  background: var(--bg-muted);
  border-radius: var(--r-full);
  color: var(--text-secondary);
  text-transform: capitalize;
}

.header-center {
  flex: 1;
  display: flex;
  justify-content: center;
}

.header-right {
  display: flex;
  align-items: center;
  gap: var(--sp-4);
}

.quick-stats {
  display: flex;
  gap: var(--sp-3);
}

.quick-stat {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--sp-1) var(--sp-2);
  background: var(--bg-inset);
  border-radius: var(--r-md);
  min-width: 60px;
}

.stat-label {
  font-size: 0.65rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
}

.stat-value {
  font-family: var(--font-mono);
  font-size: var(--fs-sm);
  font-weight: var(--fw-semibold);
  color: var(--text-primary);
}

.quick-stat.fear .stat-value { color: var(--accent-amber); }
.quick-stat.blight .stat-value { color: var(--accent-green); }
.quick-stat.terror .stat-value { color: var(--accent-coral); }

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.save-indicator {
  font-size: var(--fs-xs);
  color: var(--status-success);
  font-style: italic;
}

/* ─── BUTTONS ─── */
.btn-primary {
  background: var(--accent-violet);
  color: white;
  border: none;
  padding: var(--sp-2) var(--sp-4);
  border-radius: var(--r-md);
  font-weight: var(--fw-medium);
  cursor: pointer;
  transition: filter var(--motion-fast);
}

.btn-primary:hover {
  filter: brightness(1.15);
}

.btn-ghost {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid transparent;
  padding: var(--sp-1) var(--sp-3);
  border-radius: var(--r-md);
  font-size: var(--fs-sm);
  cursor: pointer;
  transition: all var(--motion-fast);
}

.btn-ghost:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

/* ─── GAME BANNERS ─── */
.game-banner {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  padding: var(--sp-3) var(--sp-4);
  margin: var(--sp-3) var(--sp-4) 0;
  border-radius: var(--r-lg);
  flex-shrink: 0;
}

.game-banner.won {
  background: rgba(82, 183, 136, 0.12);
  border: 1px solid var(--status-success);
}

.game-banner.lost {
  background: rgba(220, 47, 2, 0.12);
  border: 1px solid var(--status-danger);
}

.banner-icon {
  font-size: var(--fs-xl);
}

.game-banner.won .banner-icon { color: var(--status-success); }
.game-banner.lost .banner-icon { color: var(--status-danger); }

.banner-content {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.banner-content strong {
  color: var(--text-white);
}

.banner-content span {
  font-size: var(--fs-sm);
  color: var(--text-secondary);
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* DASHBOARD GRID                                                               */
/* ═══════════════════════════════════════════════════════════════════════════ */

.dashboard-grid {
  display: grid;
  grid-template-columns: 1fr 1.4fr 280px;
  gap: var(--sp-3);
  padding: var(--sp-3) var(--sp-4);
  flex: 1;
  overflow: hidden;
  min-height: 0;
}

.analytics-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--sp-3);
  padding: var(--sp-3) var(--sp-4);
  flex: 1;
  overflow: auto;
}

.log-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: var(--sp-3);
  padding: var(--sp-3) var(--sp-4);
  flex: 1;
  overflow: hidden;
}

/* ─── PANEL CARDS ─── */
.panel {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-lg);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}

.panel.wide {
  grid-column: span 2;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-3);
  padding: var(--sp-3) var(--sp-4);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.panel-header h2 {
  font-size: var(--fs-sm);
  font-weight: var(--fw-semibold);
  color: var(--text-white);
  margin: 0;
}

.panel-body {
  flex: 1;
  overflow: auto;
  padding: var(--sp-3);
  min-height: 0;
}

/* ─── SPIRIT TABS ─── */
.spirit-tabs {
  display: flex;
  gap: 4px;
}

.spirit-tab {
  padding: 4px var(--sp-2);
  font-size: var(--fs-xs);
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--motion-fast);
}

.spirit-tab:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.spirit-tab.active {
  background: var(--accent-soft);
  border-color: var(--accent);
  color: var(--accent);
}

/* ─── BOARD TABS ─── */
.board-tabs {
  display: flex;
  gap: 4px;
}

.chip {
  padding: 2px var(--sp-2);
  font-size: var(--fs-xs);
  background: var(--bg-muted);
  border-radius: var(--r-full);
  color: var(--text-secondary);
}

/* ─── DECKS STACK ─── */
.decks-stack {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
}

.deck-card {
  background: var(--bg-inset);
  border-radius: var(--r-md);
  padding: var(--sp-3);
  border-left: 3px solid var(--border-subtle);
}

.deck-card h3 {
  font-size: var(--fs-xs);
  font-weight: var(--fw-semibold);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin: 0 0 var(--sp-2) 0;
}

.deck-card.invader-accent { border-left-color: var(--accent-red); }
.deck-card.fear-accent { border-left-color: var(--accent-amber); }
.deck-card.event-accent { border-left-color: var(--accent-violet); }
.deck-card.terrain-accent { border-left-color: var(--accent-teal); }

/* ─── ANALYTICS PANEL ─── */
.big-stat {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--sp-4);
}

.big-value {
  font-family: var(--font-mono);
  font-size: 48px;
  font-weight: var(--fw-bold);
  line-height: 1;
}

.big-value.positive { color: var(--status-success); }
.big-value.negative { color: var(--status-danger); }

.big-label {
  font-size: var(--fs-sm);
  color: var(--text-secondary);
  margin-top: var(--sp-2);
}

.confidence-interval {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-3);
  font-size: var(--fs-xs);
  color: var(--text-muted);
  font-family: var(--font-mono);
}

.ci-bar {
  flex: 1;
  height: 8px;
  background: var(--bg-canvas);
  border-radius: var(--r-full);
  position: relative;
}

.ci-fill {
  position: absolute;
  top: 0;
  height: 100%;
  background: var(--accent-soft);
  border-radius: var(--r-full);
}

.ci-marker {
  position: absolute;
  top: -2px;
  width: 4px;
  height: 12px;
  background: var(--accent);
  border-radius: 2px;
  transform: translateX(-50%);
}

/* ─── FOOTER ─── */
.content-footer {
  padding: var(--sp-2) var(--sp-4);
  background: var(--bg-surface);
  border-top: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* RESPONSIVE                                                                   */
/* ═══════════════════════════════════════════════════════════════════════════ */

@media (max-width: 1200px) {
  .dashboard-grid {
    grid-template-columns: 1fr 1fr;
  }

  .decks-panel {
    grid-column: span 2;
  }

  .decks-stack {
    flex-direction: row;
    flex-wrap: wrap;
  }

  .deck-card {
    flex: 1;
    min-width: 200px;
  }
}

@media (max-width: 900px) {
  .app-shell {
    grid-template-columns: 60px 1fr;
  }

  .brand-text,
  .nav-label {
    display: none;
  }

  .sidebar-brand {
    justify-content: center;
    padding: 0 var(--sp-2) var(--sp-4);
  }

  .nav-item {
    justify-content: center;
    padding: var(--sp-2);
  }

  .nav-icon {
    width: auto;
  }

  .dashboard-grid {
    grid-template-columns: 1fr;
  }

  .decks-panel {
    grid-column: span 1;
  }

  .header-center {
    display: none;
  }

  .quick-stats {
    display: none;
  }
}
</style>
