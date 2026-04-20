<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import type { GameState, Phase } from './types'
import { fetchState, saveState } from './api'
import Board from './components/Board.vue'
import SpiritPanel from './components/SpiritPanel.vue'
import SetupWizard from './components/SetupWizard.vue'
import SavedGames from './components/SavedGames.vue'
import InvaderDeck from './components/InvaderDeck.vue'
import FearDeck from './components/FearDeck.vue'
import EventDeck from './components/EventDeck.vue'
import Retrospective from './components/Retrospective.vue'
import TerrainTimeline from './components/TerrainTimeline.vue'
import { computeWinProb } from './lib/winprob'
import { fetchSpiritAffinity, type SpiritAffinityMap } from './api'

// ─────────────────────────────────────────────────────────────────────────────
// STATE
// ─────────────────────────────────────────────────────────────────────────────
const state = ref<GameState | null>(null)
const error = ref<string | null>(null)
const saving = ref(false)
const showWizard = ref(false)
const showSavedGames = ref(false)
let saveTimer: number | null = null

const activeSpiritTab = ref<string | null>(null)
const activeView = ref<'game' | 'log'>('game')
const affinityMap = ref<SpiritAffinityMap | null>(null)
const gameOverBannerDismissed = ref(false)

// Fear tracking for auto-banking
let lastFear = 0
let lastBlight = 0
let lastSuppressRound = -1

// ─────────────────────────────────────────────────────────────────────────────
// LIFECYCLE
// ─────────────────────────────────────────────────────────────────────────────
onMounted(async () => {
  try {
    affinityMap.value = await fetchSpiritAffinity()
  } catch { /* non-blocking */ }
  
  try {
    state.value = await fetchState()
    const slugs = Object.keys(state.value?.spirits ?? {})
    activeSpiritTab.value = slugs[0] ?? null
  } catch (e) {
    error.value = (e as Error).message
  }
})

// Auto-save
watch(state, (s) => {
  if (!s) return
  if (saveTimer !== null) window.clearTimeout(saveTimer)
  saveTimer = window.setTimeout(async () => {
    saving.value = true
    try { await saveState(s) } 
    catch (e) { error.value = (e as Error).message }
    finally { saving.value = false }
  }, 400)
}, { deep: true })

// ─────────────────────────────────────────────────────────────────────────────
// COMPUTED
// ─────────────────────────────────────────────────────────────────────────────
const spiritSlugs = computed(() => Object.keys(state.value?.spirits ?? {}))

const currentWinProb = computed(() => {
  if (!state.value) return { mean: 0.5, lo: 0.3, hi: 0.7, endState: 'in-progress' }
  return computeWinProb(state.value, affinityMap.value)
})

const endResult = computed<'won' | 'lost' | 'imminent' | null>(() => {
  if (!state.value) return null
  const wp = currentWinProb.value
  if (wp.endState === 'in-progress') return null
  return wp.endState as 'won' | 'lost' | 'imminent'
})

const fearProgress = computed(() => {
  if (!state.value) return 0
  const { fear_current, fear_threshold } = state.value.pools
  return Math.min(100, (fear_current / fear_threshold) * 100)
})

const blightProgress = computed(() => {
  if (!state.value) return 0
  const { blight_current, blight_cap } = state.value.pools
  return Math.min(100, (blight_current / blight_cap) * 100)
})

const terrorInfo = computed(() => {
  if (!state.value) return { level: 1, t1: 3, t2: 3, t3: 3, earned: 0 }
  const fd = state.value.fear_deck
  if (!fd) return { level: state.value.pools.terror_level, t1: 3, t2: 3, t3: 3, earned: 0 }
  const [t1, t2, t3] = fd.tier_counts ?? [3, 3, 3]
  const earned = fd.resolved.length + fd.earned.length
  return { level: state.value.pools.terror_level, t1, t2, t3, earned }
})

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

const PHASES: Phase[] = ['growth', 'fast', 'event', 'fear', 'invader', 'slow', 'timepasses']
const PHASE_LABELS: Record<Phase, string> = {
  setup: 'Setup',
  growth: 'Spirit Phase',
  fast: 'Fast Powers',
  event: 'Event',
  fear: 'Fear',
  invader: 'Invader Phase',
  slow: 'Slow Powers',
  timepasses: 'Time Passes',
  end: 'Game End'
}

// ─────────────────────────────────────────────────────────────────────────────
// METHODS
// ─────────────────────────────────────────────────────────────────────────────
function humanSlug(slug: string): string {
  return slug.split('-').map(p => p.charAt(0).toUpperCase() + p.slice(1)).join(' ')
}

function appendLog(event: string, details: Record<string, unknown>) {
  if (!state.value) return
  const log = (state.value.log as Array<{ round: number; event: string; details: Record<string, unknown> }>) ?? []
  state.value.log = [...log, { round: state.value.round, event, details }] as unknown as GameState['log']
}

function addFear(amount: number) {
  if (!state.value) return
  state.value.pools.fear_current += amount
}

function adjustBlight(delta: number) {
  if (!state.value) return
  state.value.pools.blight_current = Math.max(0, state.value.pools.blight_current + delta)
}

function advancePhase() {
  if (!state.value) return
  const idx = PHASES.indexOf(state.value.phase)
  if (idx === -1) {
    state.value.phase = 'growth'
  } else if (idx === PHASES.length - 1) {
    state.value.phase = 'growth'
    state.value.round++
  } else {
    state.value.phase = PHASES[idx + 1]
  }
}

function prevPhase() {
  if (!state.value) return
  const idx = PHASES.indexOf(state.value.phase)
  if (idx > 0) {
    state.value.phase = PHASES[idx - 1]
  }
}

function onGameStarted(newState: GameState) {
  state.value = newState
  const slugs = Object.keys(newState.spirits ?? {})
  activeSpiritTab.value = slugs[0] ?? null
  showWizard.value = false
}

function exportGame() {
  if (!state.value) return
  const blob = new Blob([JSON.stringify(state.value, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `spirit-island-r${state.value.round}-${new Date().toISOString().slice(0, 10)}.json`
  a.click()
  URL.revokeObjectURL(url)
}

// Auto-bank fear cards
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
</script>

<template>
  <!-- MODALS -->
  <SetupWizard v-if="showWizard" @close="showWizard = false" @started="onGameStarted" />
  <SavedGames v-if="showSavedGames" @close="showSavedGames = false" @load="(s) => { state = s; showSavedGames = false }" />

  <!-- ERROR STATE -->
  <div v-if="error && !state" class="error-screen">
    <div class="error-box">
      <h2>Unable to Load Game</h2>
      <p>{{ error }}</p>
      <button @click="error = null; showWizard = true">Start New Game</button>
    </div>
  </div>

  <!-- LOADING STATE -->
  <div v-else-if="!state" class="loading-screen">
    <div class="loader"></div>
    <p>Loading...</p>
  </div>

  <!-- MAIN DASHBOARD -->
  <div v-else class="app">
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <!-- HEADER                                                                   -->
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <header class="header">
      <div class="header-left">
        <div class="logo">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2L2 7l10 5 10-5-10-5z"/>
            <path d="M2 17l10 5 10-5"/>
            <path d="M2 12l10 5 10-5"/>
          </svg>
          <span>Spirit Island</span>
        </div>
        <span class="separator"></span>
        <span class="matchup">{{ matchupTag || 'Solo Game' }}</span>
      </div>

      <div class="header-center">
        <button class="nav-arrow" @click="prevPhase" :disabled="PHASES.indexOf(state.phase) <= 0">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M15 18l-6-6 6-6"/></svg>
        </button>
        <div class="phase-badge">
          <span class="phase-name">{{ PHASE_LABELS[state.phase] }}</span>
        </div>
        <button class="nav-arrow" @click="advancePhase">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 18l6-6-6-6"/></svg>
        </button>
      </div>

      <div class="header-right">
        <div class="round-indicator">
          <span class="round-label">Round</span>
          <span class="round-value">{{ state.round }}</span>
        </div>
        <div class="win-indicator" :class="{ good: currentWinProb.mean >= 0.5 }">
          {{ Math.round(currentWinProb.mean * 100) }}%
        </div>
        <div class="header-actions">
          <button class="icon-btn" @click="showSavedGames = true" title="Saved Games">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
          </button>
          <button class="icon-btn" @click="showWizard = true" title="New Game">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
          </button>
          <button class="icon-btn" @click="exportGame" title="Export">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
          </button>
        </div>
      </div>
    </header>

    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <!-- STATUS BAR                                                               -->
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <div class="status-bar">
      <!-- Fear -->
      <div class="status-block">
        <div class="status-top">
          <span class="status-label">Fear</span>
          <span class="status-nums">{{ state.pools.fear_current }}<span class="dim">/{{ state.pools.fear_threshold }}</span></span>
        </div>
        <div class="progress-track">
          <div class="progress-bar fear-bar" :style="{ width: fearProgress + '%' }"></div>
        </div>
        <div class="quick-actions">
          <button class="quick-btn" @click="addFear(1)">+1</button>
          <button class="quick-btn" @click="addFear(2)">+2</button>
          <button class="quick-btn" @click="addFear(4)">+4</button>
        </div>
      </div>

      <!-- Terror Level -->
      <div class="status-block terror-block">
        <div class="status-top">
          <span class="status-label">Terror Level</span>
        </div>
        <div class="terror-track">
          <div class="terror-seg" :class="{ filled: terrorInfo.level >= 1, current: terrorInfo.level === 1 }">I</div>
          <div class="terror-seg" :class="{ filled: terrorInfo.level >= 2, current: terrorInfo.level === 2 }">II</div>
          <div class="terror-seg" :class="{ filled: terrorInfo.level >= 3, current: terrorInfo.level === 3 }">III</div>
          <div class="terror-seg victory" :class="{ filled: terrorInfo.level >= 4 }">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><polyline points="20 6 9 17 4 12"/></svg>
          </div>
        </div>
        <div class="terror-meta">{{ terrorInfo.earned }} / {{ terrorInfo.t1 + terrorInfo.t2 + terrorInfo.t3 }} cards</div>
      </div>

      <!-- Blight -->
      <div class="status-block">
        <div class="status-top">
          <span class="status-label">Blight</span>
          <span class="status-nums" :class="{ danger: blightProgress >= 75 }">{{ state.pools.blight_current }}<span class="dim">/{{ state.pools.blight_cap }}</span></span>
        </div>
        <div class="progress-track blight-track">
          <div class="progress-bar blight-bar" :class="{ danger: blightProgress >= 75 }" :style="{ width: blightProgress + '%' }"></div>
        </div>
        <div class="quick-actions">
          <button class="quick-btn" @click="adjustBlight(-1)">-1</button>
          <button class="quick-btn" @click="adjustBlight(1)">+1</button>
        </div>
      </div>

      <!-- Fear Cards Earned -->
      <div class="status-block compact">
        <span class="status-label">Earned</span>
        <span class="big-num">{{ state.fear_deck?.earned?.length ?? 0 }}</span>
      </div>
    </div>

    <!-- Victory/Defeat Banner -->
    <div v-if="endResult === 'won' && !gameOverBannerDismissed" class="banner victory-banner">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
      <div class="banner-text">
        <strong>Victory!</strong>
        <span>Fear deck exhausted at Terror Level {{ state.pools.terror_level }}</span>
      </div>
      <button class="banner-close" @click="gameOverBannerDismissed = true">Dismiss</button>
    </div>

    <div v-else-if="endResult === 'lost' && !gameOverBannerDismissed" class="banner defeat-banner">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
      <div class="banner-text">
        <strong>Defeat</strong>
        <span>The island has been blighted</span>
      </div>
      <button class="banner-close" @click="gameOverBannerDismissed = true">Dismiss</button>
    </div>

    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <!-- MAIN CONTENT                                                             -->
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <main class="main">
      <!-- Left Column: Spirits -->
      <section class="column spirits-column">
        <div class="section-header">
          <h2>Spirits</h2>
          <div class="tab-row" v-if="spiritSlugs.length > 1">
            <button 
              v-for="slug in spiritSlugs" 
              :key="slug"
              class="tab-btn"
              :class="{ active: activeSpiritTab === slug }"
              @click="activeSpiritTab = slug"
            >
              {{ humanSlug(slug).split(' ').slice(0, 2).join(' ') }}
            </button>
          </div>
        </div>
        <div class="section-body">
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

      <!-- Center Column: Board -->
      <section class="column board-column">
        <div class="section-header">
          <h2>Island</h2>
          <div class="board-tabs">
            <span v-for="b in (state.setup.boards ?? ['A'])" :key="b" class="board-chip">{{ b }}</span>
          </div>
        </div>
        <div class="section-body">
          <Board
            v-for="boardId in (state.setup.boards ?? ['A'])"
            :key="boardId"
            v-model="state.board_state[boardId]"
            :board-id="boardId"
            :spirits="state.spirits"
            @log-event="(event, details) => appendLog(event, details)"
          />
        </div>
      </section>

      <!-- Right Column: Decks -->
      <aside class="column decks-column">
        <div class="section-header">
          <h2>Decks</h2>
        </div>
        <div class="section-body decks-body">
          <!-- Invader Deck -->
          <div class="deck-card">
            <div class="deck-title">Invader Deck</div>
            <InvaderDeck 
              v-model="state.invader_deck" 
              :round="state.round"
              @log-event="(event, details) => appendLog(event, details)" 
            />
          </div>

          <!-- Fear Deck -->
          <div class="deck-card">
            <div class="deck-title">Fear Cards</div>
            <FearDeck 
              v-model="state.fear_deck" 
              :round="state.round"
              :terror-level="state.pools.terror_level"
              @log-event="(event, details) => appendLog(event, details)" 
            />
          </div>

          <!-- Event Deck (if using events) -->
          <div class="deck-card" v-if="state.event_deck">
            <div class="deck-title">Event Deck</div>
            <EventDeck 
              v-model="state.event_deck" 
              :round="state.round"
              @log-event="(event, details) => appendLog(event, details)" 
            />
          </div>

          <!-- Terrain Timeline -->
          <div class="deck-card" v-if="state.setup.scenario">
            <div class="deck-title">Terrain</div>
            <TerrainTimeline :state="state" />
          </div>
        </div>
      </aside>
    </main>
  </div>
</template>

<style scoped>
/* ═══════════════════════════════════════════════════════════════════════════ */
/* BASE LAYOUT                                                                  */
/* ═══════════════════════════════════════════════════════════════════════════ */
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-base);
  overflow: hidden;
}

.loading-screen,
.error-screen {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  gap: var(--sp-4);
  color: var(--text-secondary);
  background: var(--bg-base);
}

.loader {
  width: 24px;
  height: 24px;
  border: 2px solid var(--border-default);
  border-top-color: var(--text-primary);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-box {
  text-align: center;
  padding: var(--sp-8);
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
}

.error-box h2 {
  margin-bottom: var(--sp-2);
  font-size: var(--text-xl);
  color: var(--text-primary);
}

.error-box p {
  color: var(--text-secondary);
  margin-bottom: var(--sp-4);
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* HEADER                                                                       */
/* ═══════════════════════════════════════════════════════════════════════════ */
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 52px;
  padding: 0 var(--sp-5);
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border-default);
  flex-shrink: 0;
}

.header-left,
.header-center,
.header-right {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
}

.logo {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  font-weight: var(--weight-semibold);
  font-size: var(--text-base);
  color: var(--text-primary);
}

.logo svg {
  color: var(--color-accent);
}

.separator {
  width: 1px;
  height: 16px;
  background: var(--border-strong);
}

.matchup {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  text-transform: capitalize;
}

.nav-arrow {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  color: var(--text-secondary);
}

.nav-arrow:hover:not(:disabled) {
  background: var(--bg-hover);
  border-color: var(--border-strong);
  color: var(--text-primary);
}

.nav-arrow:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.phase-badge {
  padding: var(--sp-2) var(--sp-4);
  background: var(--bg-muted);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
}

.phase-name {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
}

.round-indicator {
  display: flex;
  align-items: baseline;
  gap: var(--sp-2);
}

.round-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.round-value {
  font-size: var(--text-lg);
  font-weight: var(--weight-bold);
  font-variant-numeric: tabular-nums;
  color: var(--text-primary);
}

.win-indicator {
  padding: var(--sp-1) var(--sp-3);
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  font-family: var(--font-mono);
  color: var(--color-danger);
  background: rgba(239, 68, 68, 0.12);
  border-radius: var(--radius-sm);
}

.win-indicator.good {
  color: var(--color-success);
  background: rgba(34, 197, 94, 0.12);
}

.header-actions {
  display: flex;
  gap: var(--sp-1);
}

.icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: transparent;
  border: 1px solid transparent;
  color: var(--text-muted);
  border-radius: var(--radius-md);
  transition: all var(--duration-base) var(--ease);
}

.icon-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* STATUS BAR                                                                   */
/* ═══════════════════════════════════════════════════════════════════════════ */
.status-bar {
  display: flex;
  gap: var(--sp-8);
  padding: var(--sp-4) var(--sp-6);
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border-default);
  flex-shrink: 0;
}

.status-block {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
  min-width: 160px;
}

.status-block.compact {
  min-width: auto;
  align-items: center;
  gap: var(--sp-1);
}

.status-block.terror-block {
  min-width: 200px;
}

.status-top {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
}

.status-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.status-nums {
  font-size: var(--text-base);
  font-weight: var(--weight-semibold);
  font-variant-numeric: tabular-nums;
  font-family: var(--font-mono);
  color: var(--text-primary);
}

.status-nums .dim {
  color: var(--text-muted);
  font-weight: var(--weight-normal);
}

.status-nums.danger {
  color: var(--color-danger);
}

.big-num {
  font-size: 32px;
  font-weight: var(--weight-bold);
  font-variant-numeric: tabular-nums;
  font-family: var(--font-mono);
  line-height: 1;
  color: var(--text-primary);
}

.progress-track {
  height: 6px;
  background: var(--bg-muted);
  border-radius: var(--radius-sm);
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  border-radius: var(--radius-sm);
  transition: width 200ms var(--ease);
}

.fear-bar {
  background: var(--color-fear);
}

.blight-bar {
  background: var(--color-blight);
}

.blight-bar.danger {
  background: var(--color-danger);
}

.quick-actions {
  display: flex;
  gap: var(--sp-1);
}

.quick-btn {
  height: 26px;
  padding: 0 var(--sp-2);
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  font-family: var(--font-mono);
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
}

.quick-btn:hover {
  background: var(--bg-hover);
  border-color: var(--border-default);
  color: var(--text-primary);
}

/* Terror Track */
.terror-track {
  display: flex;
  gap: 2px;
}

.terror-seg {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 32px;
  font-size: var(--text-sm);
  font-weight: var(--weight-bold);
  color: var(--text-faint);
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
}

.terror-seg:first-child {
  border-radius: var(--radius-sm) 0 0 var(--radius-sm);
}

.terror-seg.victory {
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
}

.terror-seg.filled {
  background: var(--bg-subtle);
  color: var(--text-secondary);
  border-color: var(--border-default);
}

.terror-seg.current {
  background: var(--color-fear);
  color: var(--bg-base);
  border-color: var(--color-fear);
}

.terror-seg.victory.filled {
  background: var(--color-success);
  color: var(--bg-base);
  border-color: var(--color-success);
}

.terror-meta {
  font-size: var(--text-xs);
  color: var(--text-muted);
  font-family: var(--font-mono);
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* BANNERS                                                                      */
/* ═══════════════════════════════════════════════════════════════════════════ */
.banner {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  padding: var(--sp-3) var(--sp-6);
  flex-shrink: 0;
}

.victory-banner {
  background: rgba(34, 197, 94, 0.1);
  border-bottom: 1px solid rgba(34, 197, 94, 0.2);
  color: var(--color-success);
}

.defeat-banner {
  background: rgba(239, 68, 68, 0.1);
  border-bottom: 1px solid rgba(239, 68, 68, 0.2);
  color: var(--color-danger);
}

.banner-text {
  flex: 1;
  display: flex;
  align-items: baseline;
  gap: var(--sp-2);
}

.banner-text strong {
  font-weight: var(--weight-semibold);
}

.banner-text span {
  font-size: var(--text-sm);
  opacity: 0.8;
}

.banner-close {
  background: transparent;
  border: none;
  color: inherit;
  opacity: 0.6;
  font-size: var(--text-xs);
}

.banner-close:hover {
  opacity: 1;
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* MAIN CONTENT                                                                 */
/* ═══════════════════════════════════════════════════════════════════════════ */
.main {
  display: grid;
  grid-template-columns: 380px 1fr 320px;
  gap: 1px;
  flex: 1;
  min-height: 0;
  background: var(--border-subtle);
}

.column {
  display: flex;
  flex-direction: column;
  background: var(--bg-base);
  overflow: hidden;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--sp-3) var(--sp-4);
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border-default);
  flex-shrink: 0;
}

.section-header h2 {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
}

.section-body {
  flex: 1;
  overflow-y: auto;
  padding: var(--sp-4);
}

/* Spirit Tabs */
.tab-row {
  display: flex;
  gap: var(--sp-1);
  padding: var(--sp-1);
  background: var(--bg-muted);
  border-radius: var(--radius-md);
}

.tab-btn {
  height: 28px;
  padding: 0 var(--sp-3);
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  color: var(--text-muted);
}

.tab-btn:hover {
  color: var(--text-secondary);
}

.tab-btn.active {
  background: var(--bg-surface);
  color: var(--text-primary);
  box-shadow: var(--shadow-sm);
}

/* Board Chips */
.board-tabs {
  display: flex;
  gap: var(--sp-2);
}

.board-chip {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
}

/* Decks Column */
.decks-body {
  display: flex;
  flex-direction: column;
  gap: var(--sp-4);
}

.deck-card {
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.deck-title {
  padding: var(--sp-3) var(--sp-4);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  border-bottom: 1px solid var(--border-subtle);
  background: var(--bg-elevated);
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* RESPONSIVE                                                                   */
/* ═══════════════════════════════════════════════════════════════════════════ */
@media (max-width: 1200px) {
  .main {
    grid-template-columns: 1fr 1fr;
  }
  
  .decks-column {
    grid-column: span 2;
  }
  
  .decks-body {
    flex-direction: row;
    flex-wrap: wrap;
  }
  
  .deck-card {
    flex: 1;
    min-width: 280px;
  }
}

@media (max-width: 768px) {
  .main {
    grid-template-columns: 1fr;
  }
  
  .decks-column {
    grid-column: span 1;
  }
  
  .status-bar {
    flex-wrap: wrap;
    gap: var(--sp-4);
  }
  
  .header-center {
    display: none;
  }
}
</style>
