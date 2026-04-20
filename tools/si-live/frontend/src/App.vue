<script setup lang="ts">
import { onMounted, ref, watch, computed } from 'vue'
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

// Sidebar collapse state
const sidebarCollapsed = ref(false)

// Active spirit tab for multi-spirit games
const activeSpiritTab = ref<string | null>(null)

function onGameStarted(newState: GameState) {
  state.value = newState
  // Set first spirit as active tab
  const slugs = Object.keys(newState.spirits ?? {})
  activeSpiritTab.value = slugs[0] ?? null
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
    // Initialize active spirit tab
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
  return parts.join(' · ')
})

function humanSlug(slug: string): string {
  return slug
    .split('-')
    .map(p => p.charAt(0).toUpperCase() + p.slice(1))
    .join(' ')
}

// Density preference
const density = ref<'compact' | 'comfortable'>('comfortable')
watch(density, (d) => {
  document.documentElement.dataset.density = d
}, { immediate: true })

// Phase-based grid area mapping
const PHASE_GRID: Record<Phase, string> = {
  setup:      '"board decks" "board decks"',
  growth:     '"spirits decks" "spirits stats"',
  fast:       '"spirits board" "spirits board"',
  event:      '"decks board" "decks stats"',
  fear:       '"decks board" "decks stats"',
  invader:    '"decks board" "decks board"',
  slow:       '"spirits board" "spirits board"',
  timepasses: '"retro stats" "retro stats"',
  end:        '"retro stats" "retro stats"',
}

const gridAreas = computed(() => {
  const phase = state.value?.phase ?? 'setup'
  return PHASE_GRID[phase]
})

// Determine which sections are visible (primary) for current phase
const PHASE_PRIMARY: Record<Phase, string[]> = {
  setup:      ['board', 'decks'],
  growth:     ['spirits', 'decks', 'stats'],
  fast:       ['spirits', 'board'],
  event:      ['decks', 'board', 'stats'],
  fear:       ['decks', 'board', 'stats'],
  invader:    ['decks', 'board'],
  slow:       ['spirits', 'board'],
  timepasses: ['retro', 'stats'],
  end:        ['retro', 'stats'],
}

function isPrimary(section: string): boolean {
  const phase = state.value?.phase ?? 'setup'
  return PHASE_PRIMARY[phase].includes(section)
}

// Spirit slugs for tabs
const spiritSlugs = computed(() => Object.keys(state.value?.spirits ?? {}))
</script>

<template>
  <div v-if="error" class="banner error">{{ error }}</div>
  <div v-else-if="!state" class="banner">Loading...</div>

  <div v-else class="dashboard">
    <!-- HEADER: brand + stepper + controls -->
    <header class="dash-header">
      <div class="header-top">
        <div class="brand">
          <h1>si-live</h1>
          <span v-if="matchupTag" class="matchup-tag">{{ matchupTag }}</span>
        </div>
        <div class="header-controls">
          <button class="ghost" @click="archiveCurrentGame" :disabled="archiving">
            {{ archiving ? 'Archiving...' : 'Archive' }}
          </button>
          <button class="ghost" @click="exportCurrentGame">Export</button>
          <button class="ghost" @click="showSavedGames = true">Saved</button>
          <button class="primary" @click="showWizard = true">New Game</button>
          <span v-if="saving" class="saving">saving...</span>
        </div>
      </div>
      <PhaseStepper v-model="state.phase" :round="state.round" />
    </header>

    <!-- STICKY STATUS BAR -->
    <StickyStatus :state="state" class="dash-status" />

    <!-- END GAME BANNER -->
    <div v-if="endBanner === 'won'" class="end-banner won">
      <span class="end-icon">Victory</span>
      <span class="end-detail">Fear deck exhausted at Terror {{ state.pools.terror_level }}</span>
      <button class="ghost" @click="gameOverBannerDismissed = true">Dismiss</button>
    </div>
    <div v-else-if="endBanner === 'lost'" class="end-banner lost">
      <span class="end-icon">Defeat</span>
      <span class="end-detail">Blighted Island capped</span>
      <button class="ghost" @click="gameOverBannerDismissed = true">Dismiss</button>
    </div>
    <div v-else-if="endBanner === 'imminent'" class="end-banner imminent">
      <span class="end-icon">Imminent Victory</span>
      <span class="end-detail">Fear deck drawn — finish the phase</span>
      <button class="ghost" @click="gameOverBannerDismissed = true">Dismiss</button>
    </div>

    <!-- MAIN DASHBOARD GRID -->
    <main class="dash-main" :style="{ gridTemplateAreas: gridAreas }">
      <!-- SPIRITS AREA (with tabs for multi-spirit) -->
      <section class="grid-spirits" :class="{ hidden: !isPrimary('spirits') }">
        <div class="area-header">
          <h2>Spirits</h2>
          <div v-if="spiritSlugs.length > 1" class="spirit-tabs">
            <button
              v-for="slug in spiritSlugs"
              :key="slug"
              type="button"
              class="spirit-tab"
              :class="{ active: activeSpiritTab === slug }"
              @click="activeSpiritTab = slug"
            >{{ humanSlug(slug).split(' ').slice(0, 2).join(' ') }}</button>
          </div>
        </div>
        <div class="spirit-content">
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

      <!-- BOARD AREA -->
      <section class="grid-board" :class="{ hidden: !isPrimary('board') }">
        <div class="area-header">
          <h2>Board</h2>
          <div v-if="Object.keys(state.board_state).length > 1" class="board-chips">
            <span v-for="bid in Object.keys(state.board_state)" :key="bid" class="chip">{{ bid }}</span>
          </div>
        </div>
        <div class="board-content">
          <Board
            v-for="bid in Object.keys(state.board_state)"
            :key="bid"
            v-model="state.board_state[bid]"
            :board-id="bid"
            :spirits="state.spirits"
            @bump-pool="(pool, delta) => bumpPool(pool, delta)"
          />
        </div>
      </section>

      <!-- DECKS AREA (Invader + Fear + Event + Pools) -->
      <section class="grid-decks" :class="{ hidden: !isPrimary('decks') }">
        <div class="area-header"><h2>Decks &amp; Pools</h2></div>
        <div class="decks-grid">
          <div class="deck-card">
            <h3>Pools</h3>
            <Pools
              v-model="state.pools"
              :player-count="Object.keys(state.spirits ?? {}).length"
            />
          </div>
          <div class="deck-card">
            <h3>Invader Deck</h3>
            <InvaderDeck
              v-model="state.invader_deck"
              :adversary="state.setup.adversary"
              :level="state.setup.level"
            />
          </div>
          <div class="deck-card">
            <h3>Fear Deck</h3>
            <FearDeck
              v-model="state.fear_deck"
              :round="state.round"
              :terror-level="state.pools.terror_level"
              :fear-threshold="state.pools.fear_threshold"
              @log-event="(event, details) => appendLog(event, details)"
              @reset-fear-pool="resetFearPool"
            />
          </div>
          <div class="deck-card">
            <h3>Event Deck</h3>
            <EventDeck
              v-model="state.event_deck"
              :round="state.round"
            />
          </div>
          <div class="deck-card terrain-card">
            <h3>Terrain Timeline</h3>
            <TerrainTimeline :state="state" />
          </div>
        </div>
      </section>

      <!-- STATS AREA -->
      <section class="grid-stats" :class="{ hidden: !isPrimary('stats') }">
        <div class="area-header"><h2>Stats</h2></div>
        <StatsPanel :state="state" />
      </section>

      <!-- RETROSPECTIVE AREA -->
      <section class="grid-retro" :class="{ hidden: !isPrimary('retro') }">
        <div class="area-header"><h2>Retrospective</h2></div>
        <Retrospective :state="state" />
      </section>
    </main>

    <!-- FOOTER: Turn Controller -->
    <footer class="dash-footer">
      <TurnController v-model="state" />
    </footer>

    <!-- Modals -->
    <SetupWizard :show="showWizard" @close="showWizard = false" @game-started="onGameStarted" />
    <SavedGames :show="showSavedGames" @close="showSavedGames = false" @game-loaded="onGameStarted" />
  </div>
</template>

<style scoped>
/* ─── DASHBOARD SHELL ─── */
.dashboard {
  display: grid;
  grid-template-rows: auto auto auto 1fr auto;
  height: 100vh;
  max-height: 100vh;
  overflow: hidden;
  background: var(--bg-canvas);
}

.banner {
  padding: var(--sp-6);
  text-align: center;
  color: var(--text-secondary);
  font-size: var(--fs-sm);
}
.banner.error {
  background: rgba(184, 113, 106, 0.12);
  color: var(--status-danger);
  border-radius: var(--r-md);
  margin: var(--sp-4);
}

/* ─── HEADER ─── */
.dash-header {
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border-subtle);
  padding: var(--sp-2) var(--sp-4);
}

.header-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--sp-3);
  margin-bottom: var(--sp-2);
}

.brand {
  display: flex;
  align-items: baseline;
  gap: var(--sp-3);
}

h1 {
  font-family: var(--font-mono);
  font-size: var(--fs-lg);
  font-weight: var(--fw-semibold);
  color: var(--accent);
  letter-spacing: -0.01em;
  margin: 0;
}

.matchup-tag {
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  text-transform: capitalize;
  padding: 2px var(--sp-2);
  background: var(--bg-muted);
  border-radius: var(--r-full);
}

.header-controls {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.saving {
  font-size: var(--fs-xs);
  color: var(--status-success);
  font-style: italic;
}

/* ─── STICKY STATUS ─── */
.dash-status {
  position: relative;
  z-index: 50;
}

/* ─── END BANNERS ─── */
.end-banner {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--sp-3);
  padding: var(--sp-2) var(--sp-4);
  font-size: var(--fs-sm);
  font-weight: var(--fw-semibold);
}
.end-banner.won { background: rgba(82, 183, 136, 0.15); color: var(--status-success); }
.end-banner.lost { background: rgba(184, 113, 106, 0.15); color: var(--status-danger); }
.end-banner.imminent { background: rgba(82, 183, 136, 0.08); color: var(--status-success); border-style: dashed; }
.end-icon { font-weight: var(--fw-bold); }
.end-detail { font-weight: var(--fw-regular); color: var(--text-secondary); }

/* ─── MAIN GRID ─── */
.dash-main {
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr 1fr;
  gap: var(--sp-3);
  padding: var(--sp-3);
  overflow: hidden;
  min-height: 0;
}

.grid-spirits { grid-area: spirits; }
.grid-board   { grid-area: board; }
.grid-decks   { grid-area: decks; }
.grid-stats   { grid-area: stats; }
.grid-retro   { grid-area: retro; }

/* Hidden sections (not primary for current phase) */
.grid-spirits.hidden,
.grid-board.hidden,
.grid-decks.hidden,
.grid-stats.hidden,
.grid-retro.hidden {
  display: none;
}

/* ─── AREA CARDS ─── */
.grid-spirits,
.grid-board,
.grid-decks,
.grid-stats,
.grid-retro {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-lg);
  padding: var(--sp-3);
  overflow: auto;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.area-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-2);
  margin-bottom: var(--sp-2);
  flex-shrink: 0;
}

.area-header h2 {
  font-size: var(--fs-xs);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-secondary);
  font-weight: var(--fw-semibold);
  margin: 0;
}

/* ─── SPIRIT TABS ─── */
.spirit-tabs {
  display: flex;
  gap: 4px;
}

.spirit-tab {
  padding: 3px var(--sp-2);
  font-size: var(--fs-xs);
  border: 1px solid var(--border-subtle);
  background: var(--bg-inset);
  color: var(--text-secondary);
  border-radius: var(--r-sm);
  cursor: pointer;
  transition: all var(--motion-fast);
}
.spirit-tab:hover {
  background: var(--bg-muted);
  color: var(--text-primary);
}
.spirit-tab.active {
  background: var(--accent-soft);
  border-color: var(--accent-blue);
  color: var(--text-white);
}

.spirit-content {
  flex: 1;
  overflow: auto;
  min-height: 0;
}

/* ─── BOARD AREA ─── */
.board-chips {
  display: flex;
  gap: 4px;
}

.board-content {
  flex: 1;
  overflow: auto;
  min-height: 0;
}

/* ─── DECKS GRID ─── */
.decks-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--sp-3);
  flex: 1;
  overflow: auto;
  min-height: 0;
}

.deck-card {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-2);
}

.deck-card h3 {
  font-size: var(--fs-xs);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  margin: 0 0 var(--sp-2) 0;
  font-weight: var(--fw-semibold);
}

.terrain-card {
  grid-column: span 2;
}

/* ─── FOOTER ─── */
.dash-footer {
  background: var(--bg-surface);
  border-top: 1px solid var(--border-subtle);
  padding: var(--sp-2) var(--sp-4);
}

/* ─── RESPONSIVE: single column below 1100px ─── */
@media (max-width: 1100px) {
  .dash-main {
    grid-template-columns: 1fr;
    grid-template-rows: auto;
    grid-template-areas:
      "spirits"
      "board"
      "decks"
      "stats"
      "retro" !important;
    overflow-y: auto;
  }

  .grid-spirits.hidden,
  .grid-board.hidden,
  .grid-decks.hidden,
  .grid-stats.hidden,
  .grid-retro.hidden {
    display: flex;
    opacity: 0.6;
  }

  .terrain-card {
    grid-column: span 1;
  }
}
</style>
