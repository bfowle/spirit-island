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
const affinityMap = ref<SpiritAffinityMap | null>(null)
const gameOverBannerDismissed = ref(false)
const sidebarCollapsed = ref(false)
const activeBoardId = ref<string>('A')

// Zone expansion states (for manual override)
const expandedZones = ref<Set<string>>(new Set())

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
    activeBoardId.value = state.value?.setup?.boards?.[0] ?? 'A'
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
const boardIds = computed(() => state.value?.setup?.boards ?? ['A'])

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

const PHASE_HINTS: Record<Phase, string> = {
  setup: 'Set up the game',
  growth: 'Growth -> Gain Energy -> Play Cards',
  fast: 'Resolve fast powers and innates',
  event: 'Draw and resolve event card',
  fear: 'Resolve earned fear cards',
  invader: 'Ravage -> Build -> Explore',
  slow: 'Resolve slow powers and innates',
  timepasses: 'Discard played cards, clear elements',
  end: 'Game complete'
}

// Phase-aware zone priorities
// Each phase has a "primary" zone that gets more space
const PHASE_PRIMARY_ZONE: Record<Phase, 'spirits' | 'board' | 'decks' | 'analytics'> = {
  setup: 'board',
  growth: 'spirits',
  fast: 'spirits',
  event: 'decks',
  fear: 'decks',
  invader: 'board',
  slow: 'spirits',
  timepasses: 'analytics',
  end: 'analytics'
}

const primaryZone = computed(() => state.value ? PHASE_PRIMARY_ZONE[state.value.phase] : 'board')

// Grid template based on primary zone
const gridTemplate = computed(() => {
  const zone = primaryZone.value
  switch (zone) {
    case 'spirits':
      return { columns: '1fr 1fr 320px', areas: '"spirits board decks"' }
    case 'board':
      return { columns: '340px 1fr 320px', areas: '"spirits board decks"' }
    case 'decks':
      return { columns: '340px 1fr 400px', areas: '"spirits board decks"' }
    case 'analytics':
      return { columns: '1fr 1fr 1fr', areas: '"spirits board decks"' }
    default:
      return { columns: '340px 1fr 320px', areas: '"spirits board decks"' }
  }
})

// Check if a zone is primary for the current phase
function isZonePrimary(zone: 'spirits' | 'board' | 'decks' | 'analytics'): boolean {
  return primaryZone.value === zone
}

// Toggle zone expansion
function toggleZone(zone: string) {
  if (expandedZones.value.has(zone)) {
    expandedZones.value.delete(zone)
  } else {
    expandedZones.value.add(zone)
  }
  expandedZones.value = new Set(expandedZones.value)
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

function goToPhase(phase: Phase) {
  if (!state.value) return
  state.value.phase = phase
}

function onGameStarted(newState: GameState) {
  state.value = newState
  const slugs = Object.keys(newState.spirits ?? {})
  activeSpiritTab.value = slugs[0] ?? null
  activeBoardId.value = newState.setup?.boards?.[0] ?? 'A'
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
    <p>Loading game state...</p>
  </div>

  <!-- MAIN WORKSPACE -->
  <div v-else class="workspace" :data-phase="state.phase" :data-primary="primaryZone">
    
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <!-- SIDEBAR                                                                  -->
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <aside class="sidebar" :class="{ collapsed: sidebarCollapsed }">
      <!-- Logo -->
      <div class="sidebar-header">
        <div class="logo">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M12 2L2 7l10 5 10-5-10-5z"/>
            <path d="M2 17l10 5 10-5"/>
            <path d="M2 12l10 5 10-5"/>
          </svg>
          <span v-if="!sidebarCollapsed">Spirit Island</span>
        </div>
        <button class="sidebar-toggle" @click="sidebarCollapsed = !sidebarCollapsed">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path v-if="sidebarCollapsed" d="M9 18l6-6-6-6"/>
            <path v-else d="M15 18l-6-6 6-6"/>
          </svg>
        </button>
      </div>

      <!-- Game Info -->
      <div class="sidebar-section" v-if="!sidebarCollapsed">
        <div class="sidebar-label">Current Game</div>
        <div class="game-info">
          <div class="game-stat">
            <span class="stat-value">{{ state.round }}</span>
            <span class="stat-label">Round</span>
          </div>
          <div class="game-stat">
            <span class="stat-value" :class="{ good: currentWinProb.mean >= 0.5 }">{{ Math.round(currentWinProb.mean * 100) }}%</span>
            <span class="stat-label">Win Prob</span>
          </div>
        </div>
        <div class="matchup-info" v-if="matchupTag">{{ matchupTag }}</div>
      </div>

      <!-- Phase Navigator -->
      <div class="sidebar-section">
        <div class="sidebar-label" v-if="!sidebarCollapsed">Phase</div>
        <nav class="phase-nav">
          <button 
            v-for="phase in PHASES" 
            :key="phase"
            class="phase-btn"
            :class="{ 
              active: state.phase === phase,
              completed: PHASES.indexOf(phase) < PHASES.indexOf(state.phase)
            }"
            @click="goToPhase(phase)"
            :title="sidebarCollapsed ? PHASE_LABELS[phase] : undefined"
          >
            <span class="phase-indicator"></span>
            <span class="phase-label" v-if="!sidebarCollapsed">{{ PHASE_LABELS[phase] }}</span>
          </button>
        </nav>
      </div>

      <!-- Quick Stats -->
      <div class="sidebar-section" v-if="!sidebarCollapsed">
        <div class="sidebar-label">Pools</div>
        <div class="pool-stats">
          <div class="pool-row">
            <span class="pool-label">Fear</span>
            <div class="pool-bar">
              <div class="pool-fill fear" :style="{ width: fearProgress + '%' }"></div>
            </div>
            <span class="pool-value">{{ state.pools.fear_current }}/{{ state.pools.fear_threshold }}</span>
          </div>
          <div class="pool-row">
            <span class="pool-label">Blight</span>
            <div class="pool-bar">
              <div class="pool-fill blight" :class="{ danger: blightProgress >= 75 }" :style="{ width: blightProgress + '%' }"></div>
            </div>
            <span class="pool-value">{{ state.pools.blight_current }}/{{ state.pools.blight_cap }}</span>
          </div>
          <div class="pool-row">
            <span class="pool-label">Terror</span>
            <div class="terror-mini">
              <span :class="{ active: terrorInfo.level >= 1 }">I</span>
              <span :class="{ active: terrorInfo.level >= 2 }">II</span>
              <span :class="{ active: terrorInfo.level >= 3 }">III</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Actions -->
      <div class="sidebar-footer">
        <button class="sidebar-btn" @click="showSavedGames = true" :title="sidebarCollapsed ? 'Saved Games' : undefined">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
          <span v-if="!sidebarCollapsed">Saved Games</span>
        </button>
        <button class="sidebar-btn" @click="showWizard = true" :title="sidebarCollapsed ? 'New Game' : undefined">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
          <span v-if="!sidebarCollapsed">New Game</span>
        </button>
        <button class="sidebar-btn" @click="exportGame" :title="sidebarCollapsed ? 'Export' : undefined">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
          <span v-if="!sidebarCollapsed">Export</span>
        </button>
      </div>
    </aside>

    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <!-- MAIN CONTENT                                                             -->
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <div class="main-content">
      <!-- Header Bar -->
      <header class="topbar">
        <div class="topbar-left">
          <div class="phase-display">
            <button class="phase-arrow" @click="prevPhase" :disabled="PHASES.indexOf(state.phase) <= 0">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M15 18l-6-6 6-6"/></svg>
            </button>
            <div class="phase-info">
              <span class="phase-name">{{ PHASE_LABELS[state.phase] }}</span>
              <span class="phase-hint">{{ PHASE_HINTS[state.phase] }}</span>
            </div>
            <button class="phase-arrow" @click="advancePhase">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M9 18l6-6-6-6"/></svg>
            </button>
          </div>
        </div>

        <div class="topbar-center">
          <!-- Fear Quick Actions -->
          <div class="quick-pool">
            <span class="quick-label">Fear</span>
            <div class="quick-btns">
              <button @click="addFear(1)">+1</button>
              <button @click="addFear(2)">+2</button>
              <button @click="addFear(4)">+4</button>
            </div>
          </div>
          <div class="quick-pool">
            <span class="quick-label">Blight</span>
            <div class="quick-btns">
              <button @click="adjustBlight(-1)">-1</button>
              <button @click="adjustBlight(1)">+1</button>
            </div>
          </div>
        </div>

        <div class="topbar-right">
          <div class="save-indicator" :class="{ active: saving }">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
            <span>{{ saving ? 'Saving...' : 'Saved' }}</span>
          </div>
        </div>
      </header>

      <!-- Victory/Defeat Banner -->
      <div v-if="endResult === 'won' && !gameOverBannerDismissed" class="banner victory">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
        <div class="banner-content">
          <strong>Victory!</strong>
          <span>Fear deck exhausted at Terror Level {{ state.pools.terror_level }}</span>
        </div>
        <button @click="gameOverBannerDismissed = true">Dismiss</button>
      </div>

      <div v-else-if="endResult === 'lost' && !gameOverBannerDismissed" class="banner defeat">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        <div class="banner-content">
          <strong>Defeat</strong>
          <span>The island has been blighted</span>
        </div>
        <button @click="gameOverBannerDismissed = true">Dismiss</button>
      </div>

      <!-- ═══════════════════════════════════════════════════════════════════════ -->
      <!-- WORKSPACE GRID - Phase-aware layout                                     -->
      <!-- ═══════════════════════════════════════════════════════════════════════ -->
      <main 
        class="workspace-grid"
        :style="{ 
          gridTemplateColumns: gridTemplate.columns,
          gridTemplateAreas: gridTemplate.areas
        }"
      >
        <!-- SPIRITS ZONE -->
        <section class="zone spirits-zone" :class="{ primary: isZonePrimary('spirits') }">
          <div class="zone-header">
            <h2>Spirits</h2>
            <div class="zone-tabs" v-if="spiritSlugs.length > 1">
              <button 
                v-for="slug in spiritSlugs" 
                :key="slug"
                class="zone-tab"
                :class="{ active: activeSpiritTab === slug }"
                @click="activeSpiritTab = slug"
              >
                {{ humanSlug(slug).split(' ').slice(0, 2).join(' ') }}
              </button>
            </div>
          </div>
          <div class="zone-body">
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

        <!-- BOARD ZONE -->
        <section class="zone board-zone" :class="{ primary: isZonePrimary('board') }">
          <div class="zone-header">
            <h2>Island</h2>
            <div class="zone-tabs" v-if="boardIds.length > 1">
              <button 
                v-for="bid in boardIds" 
                :key="bid"
                class="zone-tab"
                :class="{ active: activeBoardId === bid }"
                @click="activeBoardId = bid"
              >
                Board {{ bid }}
              </button>
            </div>
          </div>
          <div class="zone-body">
            <Board
              v-for="boardId in boardIds"
              v-show="activeBoardId === boardId || boardIds.length === 1"
              :key="boardId"
              v-model="state.board_state[boardId]"
              :board-id="boardId"
              :spirits="state.spirits"
              @log-event="(event, details) => appendLog(event, details)"
            />
          </div>
        </section>

        <!-- DECKS ZONE -->
        <section class="zone decks-zone" :class="{ primary: isZonePrimary('decks') }">
          <div class="zone-header">
            <h2>Decks</h2>
          </div>
          <div class="zone-body decks-stack">
            <!-- Invader Deck -->
            <div class="deck-panel" :class="{ expanded: state.phase === 'invader' }">
              <div class="deck-header">
                <span class="deck-name">Invader Deck</span>
                <span class="deck-badge invader">{{ state.invader_deck?.stack?.length ?? 0 }} cards</span>
              </div>
              <div class="deck-content">
                <InvaderDeck 
                  v-model="state.invader_deck" 
                  :round="state.round"
                  @log-event="(event, details) => appendLog(event, details)" 
                />
              </div>
            </div>

            <!-- Fear Deck -->
            <div class="deck-panel" :class="{ expanded: state.phase === 'fear' }">
              <div class="deck-header">
                <span class="deck-name">Fear Cards</span>
                <span class="deck-badge fear">{{ state.fear_deck?.earned?.length ?? 0 }} earned</span>
              </div>
              <div class="deck-content">
                <FearDeck 
                  v-model="state.fear_deck" 
                  :round="state.round"
                  :terror-level="state.pools.terror_level"
                  @log-event="(event, details) => appendLog(event, details)" 
                />
              </div>
            </div>

            <!-- Event Deck -->
            <div class="deck-panel" v-if="state.event_deck" :class="{ expanded: state.phase === 'event' }">
              <div class="deck-header">
                <span class="deck-name">Event Deck</span>
                <span class="deck-badge event">{{ state.event_deck?.resolved?.length ?? 0 }} resolved</span>
              </div>
              <div class="deck-content">
                <EventDeck 
                  v-model="state.event_deck" 
                  :round="state.round"
                  @log-event="(event, details) => appendLog(event, details)" 
                />
              </div>
            </div>

            <!-- Terrain Timeline -->
            <div class="deck-panel" v-if="state.setup.scenario">
              <div class="deck-header">
                <span class="deck-name">Terrain</span>
              </div>
              <div class="deck-content">
                <TerrainTimeline :state="state" />
              </div>
            </div>
          </div>
        </section>
      </main>
    </div>
  </div>
</template>

<style scoped>
/* ═══════════════════════════════════════════════════════════════════════════ */
/* WORKSPACE SHELL                                                              */
/* ═══════════════════════════════════════════════════════════════════════════ */
.workspace {
  display: flex;
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
  width: 32px;
  height: 32px;
  border: 2px solid var(--border-default);
  border-top-color: var(--color-accent);
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
  border-radius: var(--radius-xl);
  max-width: 400px;
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
/* SIDEBAR                                                                      */
/* ═══════════════════════════════════════════════════════════════════════════ */
.sidebar {
  display: flex;
  flex-direction: column;
  width: 240px;
  background: var(--bg-surface);
  border-right: 1px solid var(--border-default);
  transition: width 200ms var(--ease);
  flex-shrink: 0;
}

.sidebar.collapsed {
  width: 56px;
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--sp-4);
  border-bottom: 1px solid var(--border-subtle);
}

.logo {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  font-weight: var(--weight-semibold);
  font-size: var(--text-sm);
  color: var(--text-primary);
  white-space: nowrap;
}

.logo svg {
  color: var(--color-accent);
  flex-shrink: 0;
}

.sidebar-toggle {
  width: 28px;
  height: 28px;
  padding: 0;
  background: transparent;
  border: none;
  color: var(--text-muted);
  border-radius: var(--radius-sm);
}

.sidebar-toggle:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.sidebar.collapsed .sidebar-toggle {
  margin: 0 auto;
}

.sidebar-section {
  padding: var(--sp-4);
  border-bottom: 1px solid var(--border-subtle);
}

.sidebar-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  margin-bottom: var(--sp-3);
}

/* Game Info */
.game-info {
  display: flex;
  gap: var(--sp-4);
  margin-bottom: var(--sp-2);
}

.game-stat {
  display: flex;
  flex-direction: column;
}

.stat-value {
  font-family: var(--font-mono);
  font-size: var(--text-2xl);
  font-weight: var(--weight-bold);
  color: var(--text-primary);
  line-height: 1;
}

.stat-value.good {
  color: var(--color-success);
}

.stat-label {
  font-size: var(--text-xs);
  color: var(--text-muted);
  margin-top: 2px;
}

.matchup-info {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  text-transform: capitalize;
  padding: var(--sp-2) var(--sp-3);
  background: var(--bg-muted);
  border-radius: var(--radius-sm);
  margin-top: var(--sp-2);
}

/* Phase Navigation */
.phase-nav {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.phase-btn {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  height: 36px;
  padding: 0 var(--sp-3);
  background: transparent;
  border: none;
  border-radius: var(--radius-md);
  color: var(--text-muted);
  justify-content: flex-start;
  transition: all var(--duration-base) var(--ease);
}

.sidebar.collapsed .phase-btn {
  justify-content: center;
  padding: 0;
}

.phase-btn:hover {
  background: var(--bg-hover);
  color: var(--text-secondary);
}

.phase-btn.active {
  background: var(--bg-muted);
  color: var(--text-primary);
}

.phase-btn.completed {
  color: var(--text-secondary);
}

.phase-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--bg-subtle);
  flex-shrink: 0;
  transition: all var(--duration-base) var(--ease);
}

.phase-btn.active .phase-indicator {
  background: var(--color-accent);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
}

.phase-btn.completed .phase-indicator {
  background: var(--color-success);
}

.phase-label {
  font-size: var(--text-sm);
  white-space: nowrap;
}

/* Pool Stats */
.pool-stats {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
}

.pool-row {
  display: grid;
  grid-template-columns: 48px 1fr auto;
  gap: var(--sp-2);
  align-items: center;
}

.pool-label {
  font-size: var(--text-xs);
  color: var(--text-muted);
}

.pool-bar {
  height: 6px;
  background: var(--bg-muted);
  border-radius: 3px;
  overflow: hidden;
}

.pool-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 200ms var(--ease);
}

.pool-fill.fear {
  background: var(--color-fear);
}

.pool-fill.blight {
  background: var(--color-blight);
}

.pool-fill.blight.danger {
  background: var(--color-danger);
}

.pool-value {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-secondary);
}

.terror-mini {
  display: flex;
  gap: 4px;
}

.terror-mini span {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  font-weight: var(--weight-bold);
  color: var(--text-faint);
  padding: 2px 6px;
  background: var(--bg-muted);
  border-radius: var(--radius-sm);
}

.terror-mini span.active {
  background: var(--color-fear);
  color: var(--bg-base);
}

/* Sidebar Footer */
.sidebar-footer {
  margin-top: auto;
  padding: var(--sp-3);
  border-top: 1px solid var(--border-subtle);
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.sidebar-btn {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  height: 36px;
  padding: 0 var(--sp-3);
  background: transparent;
  border: none;
  border-radius: var(--radius-md);
  color: var(--text-muted);
  justify-content: flex-start;
  font-size: var(--text-sm);
}

.sidebar.collapsed .sidebar-btn {
  justify-content: center;
  padding: 0;
}

.sidebar-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.sidebar-btn svg {
  flex-shrink: 0;
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* MAIN CONTENT                                                                 */
/* ═══════════════════════════════════════════════════════════════════════════ */
.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  overflow: hidden;
}

/* Top Bar */
.topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 52px;
  padding: 0 var(--sp-5);
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border-default);
  flex-shrink: 0;
}

.topbar-left,
.topbar-center,
.topbar-right {
  display: flex;
  align-items: center;
  gap: var(--sp-4);
}

.phase-display {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
}

.phase-arrow {
  width: 28px;
  height: 28px;
  padding: 0;
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  color: var(--text-secondary);
}

.phase-arrow:hover:not(:disabled) {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.phase-arrow:disabled {
  opacity: 0.3;
}

.phase-info {
  display: flex;
  flex-direction: column;
  min-width: 140px;
}

.phase-name {
  font-size: var(--text-base);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
}

.phase-hint {
  font-size: var(--text-xs);
  color: var(--text-muted);
}

/* Quick Pools */
.quick-pool {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.quick-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

.quick-btns {
  display: flex;
  gap: 2px;
}

.quick-btns button {
  height: 26px;
  padding: 0 var(--sp-2);
  font-size: var(--text-xs);
  font-family: var(--font-mono);
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
}

.quick-btns button:hover {
  background: var(--bg-hover);
}

.save-indicator {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  font-size: var(--text-xs);
  color: var(--text-muted);
}

.save-indicator.active {
  color: var(--color-accent);
}

/* Banner */
.banner {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  padding: var(--sp-3) var(--sp-5);
  flex-shrink: 0;
}

.banner.victory {
  background: rgba(34, 197, 94, 0.1);
  border-bottom: 1px solid rgba(34, 197, 94, 0.2);
  color: var(--color-success);
}

.banner.defeat {
  background: rgba(239, 68, 68, 0.1);
  border-bottom: 1px solid rgba(239, 68, 68, 0.2);
  color: var(--color-danger);
}

.banner-content {
  flex: 1;
  display: flex;
  align-items: baseline;
  gap: var(--sp-2);
}

.banner-content strong {
  font-weight: var(--weight-semibold);
}

.banner-content span {
  font-size: var(--text-sm);
  opacity: 0.8;
}

.banner button {
  background: transparent;
  border: none;
  color: inherit;
  opacity: 0.6;
  font-size: var(--text-xs);
  padding: var(--sp-1) var(--sp-2);
}

.banner button:hover {
  opacity: 1;
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* WORKSPACE GRID                                                               */
/* ═══════════════════════════════════════════════════════════════════════════ */
.workspace-grid {
  display: grid;
  gap: 1px;
  flex: 1;
  min-height: 0;
  background: var(--border-subtle);
  transition: grid-template-columns 300ms var(--ease);
}

/* Zone Base Styles */
.zone {
  display: flex;
  flex-direction: column;
  background: var(--bg-base);
  overflow: hidden;
  transition: all 300ms var(--ease);
}

.zone.primary {
  /* Primary zone has subtle highlight */
}

.zone-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--sp-3) var(--sp-4);
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.zone-header h2 {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
}

.zone.primary .zone-header h2 {
  color: var(--text-secondary);
}

.zone-tabs {
  display: flex;
  gap: 2px;
  padding: 2px;
  background: var(--bg-muted);
  border-radius: var(--radius-sm);
}

.zone-tab {
  height: 26px;
  padding: 0 var(--sp-3);
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  background: transparent;
  border: none;
  color: var(--text-muted);
  border-radius: var(--radius-sm);
}

.zone-tab:hover {
  color: var(--text-secondary);
}

.zone-tab.active {
  background: var(--bg-surface);
  color: var(--text-primary);
  box-shadow: var(--shadow-sm);
}

.zone-body {
  flex: 1;
  overflow-y: auto;
  padding: var(--sp-4);
}

/* Grid Area Assignments */
.spirits-zone { grid-area: spirits; }
.board-zone { grid-area: board; }
.decks-zone { grid-area: decks; }

/* Decks Stack */
.decks-stack {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
}

.deck-panel {
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  overflow: hidden;
  transition: all 200ms var(--ease);
}

.deck-panel.expanded {
  border-color: var(--color-accent);
  box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.1);
}

.deck-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--sp-3) var(--sp-4);
  background: var(--bg-elevated);
  border-bottom: 1px solid var(--border-subtle);
}

.deck-name {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
}

.deck-badge {
  font-size: var(--text-xs);
  font-family: var(--font-mono);
  padding: 2px 8px;
  border-radius: 10px;
  background: var(--bg-muted);
  color: var(--text-muted);
}

.deck-badge.invader {
  background: rgba(239, 68, 68, 0.12);
  color: var(--color-danger);
}

.deck-badge.fear {
  background: rgba(245, 158, 11, 0.12);
  color: var(--color-fear);
}

.deck-badge.event {
  background: rgba(168, 85, 247, 0.12);
  color: var(--color-blight);
}

.deck-content {
  /* Component renders inside */
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* RESPONSIVE                                                                   */
/* ═══════════════════════════════════════════════════════════════════════════ */
@media (max-width: 1200px) {
  .sidebar {
    width: 56px;
  }
  
  .sidebar .logo span,
  .sidebar .sidebar-label,
  .sidebar .phase-label,
  .sidebar .sidebar-btn span,
  .sidebar .sidebar-section:not(.phase-nav):has(.sidebar-label),
  .sidebar .game-info,
  .sidebar .matchup-info,
  .sidebar .pool-stats {
    display: none;
  }
  
  .sidebar .phase-btn {
    justify-content: center;
    padding: 0;
  }
  
  .sidebar .sidebar-btn {
    justify-content: center;
    padding: 0;
  }
  
  .workspace-grid {
    grid-template-columns: 1fr 1fr !important;
    grid-template-areas: 
      "spirits board"
      "decks decks" !important;
  }
}

@media (max-width: 900px) {
  .workspace-grid {
    grid-template-columns: 1fr !important;
    grid-template-areas: 
      "spirits"
      "board"
      "decks" !important;
  }
  
  .topbar-center {
    display: none;
  }
}
</style>
