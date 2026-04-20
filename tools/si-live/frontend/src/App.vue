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
import AnalyticsZone from './components/AnalyticsZone.vue'
import SpiritTabs from './components/SpiritTabs.vue'
import DeckStation from './components/DeckStation.vue'

const state = ref<GameState | null>(null)
const error = ref<string | null>(null)
const saving = ref(false)
const showWizard = ref(false)
const showSavedGames = ref(false)
let saveTimer: number | null = null

// Active spirit tab for multi-spirit games
const activeSpiritTab = ref<string | null>(null)

// Stats drawer state
const statsDrawerOpen = ref(false)

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

// ─────────────────────────────────────────────────────────────────────────────
// Phase transition animation state
// ─────────────────────────────────────────────────────────────────────────────
const phaseTransitioning = ref(false)
const previousPhase = ref<Phase | null>(null)

watch(
  () => state.value?.phase,
  (newPhase, oldPhase) => {
    if (newPhase && oldPhase && newPhase !== oldPhase) {
      previousPhase.value = oldPhase
      phaseTransitioning.value = true
      // Reset after animation completes
      setTimeout(() => {
        phaseTransitioning.value = false
      }, 300)
    }
  }
)

// Deck expansion state (for DeckStation compact/expanded toggle)
const expandedDecks = ref<Set<string>>(new Set())

function toggleDeckExpansion(deck: string) {
  if (expandedDecks.value.has(deck)) {
    expandedDecks.value.delete(deck)
  } else {
    expandedDecks.value.add(deck)
  }
  expandedDecks.value = new Set(expandedDecks.value) // trigger reactivity
}

// Auto-expand decks when their phase is primary
watch(
  () => state.value?.phase,
  (phase) => {
    if (!phase) return
    const newExpanded = new Set<string>()
    if (phase === 'invader') newExpanded.add('invader')
    if (phase === 'fear') newExpanded.add('fear')
    if (phase === 'event') newExpanded.add('event')
    expandedDecks.value = newExpanded
  },
  { immediate: true }
)

// ─────────────────────────────────────────────────────────────────────────────
// SESSION 2: Phase-based grid area mapping
// ─────────────────────────────────────────────────────────────────────────────
// Each phase defines which sections are PRIMARY (get the biggest space) and
// which are SECONDARY (collapsible strips or hidden). The grid dynamically
// switches layouts.

type GridLayout = {
  columns: string
  rows: string
  areas: string
}

const PHASE_LAYOUTS: Record<Phase, GridLayout> = {
  setup: {
    columns: '1fr 1fr',
    rows: '1fr 1fr',
    areas: '"board decks" "board decks"',
  },
  growth: {
    columns: '2fr 1fr',
    rows: '1fr auto',
    areas: '"spirits decks" "spirits stats"',
  },
  fast: {
    columns: '1fr 1.2fr',
    rows: '1fr',
    areas: '"spirits board"',
  },
  event: {
    columns: '1fr 1.5fr',
    rows: '2fr 1fr',
    areas: '"decks board" "stats board"',
  },
  fear: {
    columns: '1fr 1.5fr',
    rows: '2fr 1fr',
    areas: '"decks board" "stats board"',
  },
  invader: {
    columns: '1fr 1.5fr',
    rows: '1fr',
    areas: '"decks board"',
  },
  slow: {
    columns: '1fr 1.2fr',
    rows: '1fr',
    areas: '"spirits board"',
  },
  timepasses: {
    columns: '1.5fr 1fr',
    rows: '1fr',
    areas: '"retro stats"',
  },
  end: {
    columns: '1.5fr 1fr',
    rows: '1fr',
    areas: '"retro stats"',
  },
}

const currentLayout = computed(() => {
  const phase = state.value?.phase ?? 'setup'
  return PHASE_LAYOUTS[phase]
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

// ─────────────────────────────────────────────────────────────────────────────
// SESSION 3: Spirit multi-panel with tabs + pinned mini-view
// ─────────────────────────────────────────────────────────────────────────────
// When multiple spirits exist, show a tab bar. The active spirit gets the full
// panel. Other spirits show a "pinned mini" strip with key stats (energy,
// card plays, elements) so the player can see all spirits at a glance.

const showMiniPanels = computed(() => spiritSlugs.value.length > 1)

// Mini-panel data for non-active spirits
function getMiniData(slug: string) {
  const spirit = state.value?.spirits?.[slug]
  if (!spirit) return null
  return {
    slug,
    name: humanSlug(slug).split(' ').slice(0, 2).join(' '),
    energy: spirit.energy ?? 0,
    cardPlays: spirit.card_plays ?? 1,
    handCount: spirit.hand?.length ?? 0,
    discardCount: spirit.discard?.length ?? 0,
    elements: spirit.elements_this_turn ?? {},
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// SESSION 4: Win-prob sparkline data for stats drawer
// ─────────────────────────────────────────────────────────────────────────────
// Track win probability over rounds for a sparkline. Derived from log's
// `turn_advanced` entries.

const winProbHistory = computed(() => {
  if (!state.value) return []
  const log = (state.value.log ?? []) as Array<{ round: number; event: string; details: Record<string, unknown> }>
  const history: number[] = []
  
  for (let r = 1; r <= state.value.round; r++) {
    const turnAdvance = log.find(e => e.event === 'turn_advanced' && e.round === r)
    if (turnAdvance) {
      // Simple heuristic estimation for historical rounds
      const fearCurr = (turnAdvance.details.fear_current as number) ?? 0
      const blightCurr = (turnAdvance.details.blight_current as number) ?? 0
      const threshold = state.value.pools.fear_threshold || 4
      const cap = state.value.pools.blight_cap || 3
      const fearRatio = threshold > 0 ? fearCurr / threshold : 0
      const blightRatio = cap > 0 ? blightCurr / cap : 0
      const m = 0.5 + fearRatio * 0.2 - blightRatio * 0.3 - Math.max(0, r - 6) * 0.05
      history.push(Math.max(0, Math.min(1, m)))
    } else if (r === state.value.round) {
      // Current round: use live calculation
      const wp = computeWinProb(state.value, affinityMap.value)
      history.push(wp.mean)
    }
  }
  return history
})

// Current win probability for display
const currentWinProb = computed(() => {
  if (!state.value) return { mean: 0.5, lo: 0.3, hi: 0.7 }
  return computeWinProb(state.value, affinityMap.value)
})

// Generate SVG sparkline path
const sparklinePath = computed(() => {
  const data = winProbHistory.value
  if (data.length < 2) return ''
  const width = 80
  const height = 24
  const padding = 2
  const step = (width - padding * 2) / (data.length - 1)
  
  let path = ''
  data.forEach((v, i) => {
    const x = padding + i * step
    const y = height - padding - (v * (height - padding * 2))
    path += i === 0 ? `M ${x} ${y}` : ` L ${x} ${y}`
  })
  return path
})
</script>

<template>
  <div v-if="error" class="banner error">{{ error }}</div>
  <div v-else-if="!state" class="banner">Loading...</div>

  <div v-else class="dashboard" :class="{ 'phase-transitioning': phaseTransitioning }" :data-phase="state.phase">
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
    <main
      class="dash-main"
      :style="{
        gridTemplateColumns: currentLayout.columns,
        gridTemplateRows: currentLayout.rows,
        gridTemplateAreas: currentLayout.areas,
      }"
    >
      <!-- ═══════════════════════════════════════════════════════════════════ -->
      <!-- SESSION 3: SPIRITS AREA with tabs + mini-panels -->
      <!-- ═══════════════════════════════════════════════════════════════════ -->
      <section class="grid-spirits" :class="{ hidden: !isPrimary('spirits') }">
        <div class="area-header">
          <h2>Spirits</h2>
          <!-- Tab bar for multi-spirit -->
          <div v-if="showMiniPanels" class="spirit-tabs">
            <button
              v-for="slug in spiritSlugs"
              :key="slug"
              type="button"
              class="spirit-tab"
              :class="{ active: activeSpiritTab === slug }"
              @click="activeSpiritTab = slug"
            >
              <span class="tab-name">{{ humanSlug(slug).split(' ').slice(0, 2).join(' ') }}</span>
              <span class="tab-stats">
                <span class="tab-energy">{{ state.spirits[slug]?.energy ?? 0 }}E</span>
                <span class="tab-cards">{{ state.spirits[slug]?.hand?.length ?? 0 }}H</span>
              </span>
            </button>
          </div>
        </div>

        <!-- Mini-panels strip for non-active spirits -->
        <div v-if="showMiniPanels" class="spirit-minis">
          <div
            v-for="slug in spiritSlugs.filter(s => s !== activeSpiritTab)"
            :key="slug"
            class="mini-panel"
            @click="activeSpiritTab = slug"
          >
            <span class="mini-name">{{ humanSlug(slug).split(' ')[0] }}</span>
            <div class="mini-stats">
              <span class="mini-stat energy">{{ state.spirits[slug]?.energy ?? 0 }}E</span>
              <span class="mini-stat plays">{{ state.spirits[slug]?.card_plays ?? 1 }}CP</span>
              <span class="mini-stat hand">{{ state.spirits[slug]?.hand?.length ?? 0 }}H</span>
            </div>
            <div class="mini-elements">
              <span
                v-for="(count, el) in (state.spirits[slug]?.elements_this_turn ?? {})"
                :key="el"
                class="mini-el"
                :class="`el-${el}`"
              >{{ count }}</span>
            </div>
          </div>
        </div>

        <!-- Active spirit full panel -->
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

      <!-- STATS AREA (with AnalyticsZone for quick view) -->
      <section class="grid-stats" :class="{ hidden: !isPrimary('stats') }">
        <div class="area-header">
          <h2>Analytics</h2>
          <button class="ghost" @click="statsDrawerOpen = !statsDrawerOpen">
            {{ statsDrawerOpen ? 'Hide Details' : 'Full Stats' }}
          </button>
        </div>
        <AnalyticsZone
          :state="state"
          :win-prob="currentWinProb"
          :win-prob-history="winProbHistory"
        />
        <div class="stats-detail" v-if="statsDrawerOpen">
          <StatsPanel :state="state" />
        </div>
      </section>

      <!-- RETROSPECTIVE AREA -->
      <section class="grid-retro" :class="{ hidden: !isPrimary('retro') }">
        <div class="area-header"><h2>Retrospective</h2></div>
        <Retrospective :state="state" />
      </section>
    </main>

    <!-- ═══════════════════════════════════════════════════════════════════ -->
    <!-- SESSION 4: STATS DRAWER (collapsible right edge) -->
    <!-- ═══════════════════════════════════════════════════════════════════ -->
    <aside class="stats-drawer" :class="{ open: statsDrawerOpen }">
      <button class="drawer-toggle" @click="statsDrawerOpen = !statsDrawerOpen">
        <span class="toggle-icon">{{ statsDrawerOpen ? '›' : '‹' }}</span>
        <span class="toggle-label">Stats</span>
      </button>
      <div class="drawer-content">
        <!-- Win probability sparkline -->
        <div class="drawer-section">
          <div class="drawer-label">Win Probability</div>
          <div class="win-prob-row">
            <span class="wp-value">{{ (currentWinProb.mean * 100).toFixed(0) }}%</span>
            <svg class="sparkline" viewBox="0 0 80 24" preserveAspectRatio="none">
              <path :d="sparklinePath" fill="none" stroke="var(--accent-green)" stroke-width="1.5" />
            </svg>
          </div>
          <div class="wp-ci">{{ (currentWinProb.lo * 100).toFixed(0) }}–{{ (currentWinProb.hi * 100).toFixed(0) }}% CI</div>
        </div>

        <!-- Quick pool bars -->
        <div class="drawer-section">
          <div class="drawer-label">Fear</div>
          <div class="mini-bar">
            <div
              class="mini-fill fear"
              :style="{ width: (state.pools.fear_threshold ? (state.pools.fear_current / state.pools.fear_threshold) * 100 : 0) + '%' }"
            ></div>
          </div>
          <div class="mini-stat-row">
            <span>{{ state.pools.fear_current }}/{{ state.pools.fear_threshold }}</span>
            <span class="terror-badge">T{{ state.pools.terror_level }}</span>
          </div>
        </div>

        <div class="drawer-section">
          <div class="drawer-label">Blight</div>
          <div class="mini-bar">
            <div
              class="mini-fill blight"
              :style="{ width: (state.pools.blight_cap ? (state.pools.blight_current / state.pools.blight_cap) * 100 : 0) + '%' }"
            ></div>
          </div>
          <div class="mini-stat-row">
            <span>{{ state.pools.blight_current }}/{{ state.pools.blight_cap }}</span>
          </div>
        </div>

        <!-- Round + Phase quick view -->
        <div class="drawer-section">
          <div class="drawer-label">Round</div>
          <div class="round-display">{{ state.round }}</div>
          <div class="phase-display">{{ state.phase }}</div>
        </div>
      </div>
    </aside>

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
  grid-template-columns: 1fr auto;
  height: 100vh;
  max-height: 100vh;
  overflow: hidden;
  background: var(--bg-canvas);
}

.dash-header,
.dash-status,
.end-banner,
.dash-footer {
  grid-column: 1 / -1;
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
  gap: var(--sp-3);
  padding: var(--sp-3);
  overflow: hidden;
  min-height: 0;
  transition: grid-template-columns var(--motion-base), grid-template-rows var(--motion-base);
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* PHASE TRANSITION ANIMATIONS                                                  */
/* ═══════════════════════════════════════════════════════════════════════════ */
.dashboard.phase-transitioning .dash-main {
  animation: grid-reflow 300ms ease-out;
}

@keyframes grid-reflow {
  0% {
    opacity: 0.8;
    transform: scale(0.995);
  }
  100% {
    opacity: 1;
    transform: scale(1);
  }
}

/* Phase-specific accent colors on the dashboard */
.dashboard[data-phase="setup"] { --phase-accent: var(--accent-blue); }
.dashboard[data-phase="growth"] { --phase-accent: var(--accent-green); }
.dashboard[data-phase="fast"] { --phase-accent: var(--accent-purple); }
.dashboard[data-phase="event"] { --phase-accent: var(--accent-violet); }
.dashboard[data-phase="fear"] { --phase-accent: var(--accent-amber); }
.dashboard[data-phase="invader"] { --phase-accent: var(--accent-red); }
.dashboard[data-phase="slow"] { --phase-accent: var(--accent-teal); }
.dashboard[data-phase="timepasses"] { --phase-accent: var(--accent-blue); }
.dashboard[data-phase="end"] { --phase-accent: var(--accent-green); }

/* Sections entering view animate in */
.grid-spirits,
.grid-board,
.grid-decks,
.grid-stats,
.grid-retro {
  animation: section-enter 250ms ease-out;
}

@keyframes section-enter {
  0% {
    opacity: 0;
    transform: translateY(8px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Stats detail toggle */
.stats-detail {
  margin-top: var(--sp-3);
  padding-top: var(--sp-3);
  border-top: 1px solid var(--border-subtle);
  animation: fade-in 200ms ease-out;
}

@keyframes fade-in {
  0% { opacity: 0; }
  100% { opacity: 1; }
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
  overflow: hidden;
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

/* ═══════════════════════════════════════════════════════════════════════════ */
/* SESSION 3: SPIRIT TABS + MINI PANELS                                        */
/* ═══════════════════════════════════════════════════════════════════════════ */
.spirit-tabs {
  display: flex;
  gap: 4px;
}

.spirit-tab {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  padding: 4px var(--sp-2);
  font-size: var(--fs-xs);
  border: 1px solid var(--border-subtle);
  background: var(--bg-inset);
  color: var(--text-secondary);
  border-radius: var(--r-sm);
  cursor: pointer;
  transition: all var(--motion-fast);
  min-width: 80px;
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

.tab-name {
  font-weight: var(--fw-semibold);
  white-space: nowrap;
}
.tab-stats {
  display: flex;
  gap: var(--sp-2);
  font-size: 0.68rem;
  color: var(--text-muted);
  font-family: var(--font-mono);
}
.spirit-tab.active .tab-stats { color: var(--text-secondary); }

.tab-energy { color: var(--pool-energy); }
.tab-cards { color: var(--text-muted); }

/* Mini-panel strip for non-active spirits */
.spirit-minis {
  display: flex;
  gap: var(--sp-2);
  margin-bottom: var(--sp-2);
  flex-shrink: 0;
}

.mini-panel {
  flex: 1;
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-1) var(--sp-2);
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  cursor: pointer;
  transition: all var(--motion-fast);
}
.mini-panel:hover {
  background: var(--bg-muted);
  border-color: var(--border-default);
}

.mini-name {
  font-size: var(--fs-xs);
  font-weight: var(--fw-semibold);
  color: var(--text-secondary);
  min-width: 60px;
}

.mini-stats {
  display: flex;
  gap: var(--sp-1);
  font-size: 0.68rem;
  font-family: var(--font-mono);
}

.mini-stat {
  padding: 1px 4px;
  border-radius: var(--r-sm);
  background: var(--bg-muted);
}
.mini-stat.energy { color: var(--pool-energy); }
.mini-stat.plays { color: var(--accent-purple); }
.mini-stat.hand { color: var(--text-muted); }

.mini-elements {
  display: flex;
  gap: 2px;
  margin-left: auto;
}

.mini-el {
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.6rem;
  font-weight: var(--fw-bold);
  border-radius: var(--r-full);
  background: var(--bg-muted);
  color: var(--text-primary);
}
.mini-el.el-sun { background: #d9a45e33; color: #d9a45e; }
.mini-el.el-moon { background: #7bb8f533; color: #7bb8f5; }
.mini-el.el-fire { background: #dc2f0233; color: #dc2f02; }
.mini-el.el-air { background: #c77dff33; color: #c77dff; }
.mini-el.el-water { background: #2a9d8f33; color: #2a9d8f; }
.mini-el.el-earth { background: #b85c6433; color: #b85c64; }
.mini-el.el-plant { background: #6fa66133; color: #6fa661; }
.mini-el.el-animal { background: #e9c46a33; color: #e9c46a; }

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
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
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

/* ═══════════════════════════════════════════════════════════════════════════ */
/* SESSION 4: STATS DRAWER                                                     */
/* ═══════════════════════════════════════════════════════════════════════════ */
.stats-drawer {
  grid-row: 4;
  grid-column: 2;
  width: 48px;
  background: var(--bg-surface);
  border-left: 1px solid var(--border-subtle);
  display: flex;
  flex-direction: column;
  transition: width var(--motion-base);
  overflow: hidden;
}

.stats-drawer.open {
  width: 180px;
}

.drawer-toggle {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  padding: var(--sp-2) var(--sp-1);
  background: transparent;
  border: none;
  border-bottom: 1px solid var(--border-subtle);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--motion-fast);
}
.drawer-toggle:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.toggle-icon {
  font-size: var(--fs-lg);
  font-weight: var(--fw-bold);
  color: var(--accent);
}

.toggle-label {
  font-size: 0.6rem;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  writing-mode: vertical-lr;
  text-orientation: mixed;
}
.stats-drawer.open .toggle-label {
  writing-mode: horizontal-tb;
}

.drawer-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--sp-2);
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
  opacity: 0;
  pointer-events: none;
  transition: opacity var(--motion-fast);
}

.stats-drawer.open .drawer-content {
  opacity: 1;
  pointer-events: auto;
}

.drawer-section {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.drawer-label {
  font-size: 0.6rem;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--text-muted);
  font-weight: var(--fw-semibold);
}

.win-prob-row {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.wp-value {
  font-family: var(--font-mono);
  font-size: var(--fs-lg);
  font-weight: var(--fw-bold);
  color: var(--accent-green);
}

.sparkline {
  flex: 1;
  height: 24px;
  max-width: 80px;
}

.wp-ci {
  font-size: 0.6rem;
  color: var(--text-muted);
  font-family: var(--font-mono);
}

.mini-bar {
  height: 6px;
  background: var(--bg-canvas);
  border-radius: var(--r-full);
  overflow: hidden;
}

.mini-fill {
  height: 100%;
  border-radius: var(--r-full);
  transition: width var(--motion-fast);
}
.mini-fill.fear { background: linear-gradient(90deg, #d4a373, #d9771f); }
.mini-fill.blight { background: linear-gradient(90deg, #52b788, #2a9d8f); }

.mini-stat-row {
  display: flex;
  justify-content: space-between;
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  font-family: var(--font-mono);
}

.terror-badge {
  padding: 1px 4px;
  background: var(--accent-amber);
  color: var(--bg-canvas);
  font-size: 0.6rem;
  font-weight: var(--fw-bold);
  border-radius: var(--r-sm);
}

.round-display {
  font-family: var(--font-mono);
  font-size: var(--fs-xl);
  font-weight: var(--fw-bold);
  color: var(--text-primary);
  line-height: 1;
}

.phase-display {
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  text-transform: capitalize;
}

/* ─── FOOTER ─── */
.dash-footer {
  background: var(--bg-surface);
  border-top: 1px solid var(--border-subtle);
  padding: var(--sp-2) var(--sp-4);
}

/* ─── RESPONSIVE: single column below 1100px ─── */
@media (max-width: 1100px) {
  .dashboard {
    grid-template-columns: 1fr;
  }

  .dash-main {
    grid-template-columns: 1fr !important;
    grid-template-rows: auto !important;
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

  .stats-drawer {
    display: none;
  }

  .spirit-minis {
    flex-wrap: wrap;
  }

  .mini-panel {
    flex: 1 1 calc(50% - var(--sp-1));
    min-width: 140px;
  }
}
</style>
