<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import type { GameState, Phase, Spirit, InvaderDeckState, FearDeckState, EventDeckState } from './types'
import { fetchState, saveState } from './api'
import SetupWizard from './components/SetupWizard.vue'
import SavedGames from './components/SavedGames.vue'
import SpiritZone from './components/SpiritZone.vue'
import BoardZone from './components/BoardZone.vue'
import DecksZone from './components/DecksZone.vue'
import AnalyticsZone from './components/AnalyticsZone.vue'
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

const affinityMap = ref<SpiritAffinityMap | null>(null)
const gameOverBannerDismissed = ref(false)
const sidebarCollapsed = ref(false)

// ─────────────────────────────────────────────────────────────────────────────
// DEMO DATA for INVADER PHASE (realistic two-spirit game at round 5)
// ─────────────────────────────────────────────────────────────────────────────
function createDemoState(): GameState {
  return {
    version: '1.0',
    round: 5,
    phase: 'invader' as Phase,
    setup: {
      adversary: 'england',
      level: 4,
      scenario: null,
      spirits: ['river-surges-in-sunlight', 'lightning-swift-strike'],
      boards: ['A', 'B'],
      expansions_active: ['base', 'branch-and-claw', 'jagged-earth']
    },
    pools: {
      fear_current: 2,
      fear_threshold: 4,
      terror_level: 2,
      blight_current: 4,
      blight_cap: 8,
      island_blighted: false,
      blight_card: 'Aid from Lesser Spirits'
    },
    spirits: {
      'river-surges-in-sunlight': {
        energy: 7,
        card_plays: 3,
        presence_on_board: { 'A.2': 2, 'A.5': 1, 'A.6': 1, 'B.3': 1 },
        presence_on_track_energy: ['2', '3'],
        presence_on_track_cardplay: ['2', '3'],
        elements_this_turn: { Sun: 2, Water: 3, Earth: 1, Plant: 0, Animal: 1, Fire: 0, Air: 0, Moon: 0 },
        hand: ['Flash Floods', 'Wash Away', 'Boon of Vigor'],
        discard: ['Rivers Bounty', 'Massive Flooding'],
        forgotten: [],
        played_this_turn: ['Flash Floods', 'Wash Away'],
        disc_color: '#3b82f6',
        disc_style: 'glass'
      } as Spirit,
      'lightning-swift-strike': {
        energy: 4,
        card_plays: 4,
        presence_on_board: { 'B.1': 1, 'B.4': 2, 'B.7': 1, 'A.8': 1 },
        presence_on_track_energy: ['1', '2', '3'],
        presence_on_track_cardplay: ['2', '3', '4'],
        elements_this_turn: { Fire: 3, Air: 2, Sun: 1, Water: 0, Earth: 0, Plant: 0, Animal: 0, Moon: 0 },
        hand: ['Raging Storm', 'Shatter Homesteads'],
        discard: ['Harbingers of the Lightning', 'Lightning Shatters The Wilds'],
        forgotten: ['Pillar of Living Flame'],
        played_this_turn: ['Raging Storm', 'Shatter Homesteads', 'Harbingers of the Lightning'],
        disc_color: '#eab308',
        disc_style: 'solid'
      } as Spirit
    },
    board_state: {
      A: {
        lands: {
          '1': { terrain: 'mountain', coastal: true, explorers: 0, towns: 0, cities: 1, dahan: 2, blight: 0, tokens: [] },
          '2': { terrain: 'wetland', coastal: true, explorers: 2, towns: 1, cities: 0, dahan: 1, blight: 0, tokens: ['strife'] },
          '3': { terrain: 'jungle', coastal: true, explorers: 1, towns: 0, cities: 0, dahan: 0, blight: 1, tokens: [] },
          '4': { terrain: 'sands', coastal: false, explorers: 0, towns: 2, cities: 0, dahan: 3, blight: 0, tokens: [] },
          '5': { terrain: 'wetland', coastal: false, explorers: 3, towns: 1, cities: 0, dahan: 0, blight: 1, tokens: ['disease'] },
          '6': { terrain: 'mountain', coastal: false, explorers: 1, towns: 0, cities: 1, dahan: 2, blight: 0, tokens: [] },
          '7': { terrain: 'sands', coastal: false, explorers: 0, towns: 0, cities: 0, dahan: 1, blight: 0, tokens: [] },
          '8': { terrain: 'jungle', coastal: false, explorers: 2, towns: 2, cities: 0, dahan: 0, blight: 2, tokens: [] }
        },
        variant: 'balanced',
        variant_name: 'Board A'
      },
      B: {
        lands: {
          '1': { terrain: 'jungle', coastal: true, explorers: 1, towns: 1, cities: 0, dahan: 2, blight: 0, tokens: [] },
          '2': { terrain: 'wetland', coastal: true, explorers: 0, towns: 0, cities: 1, dahan: 0, blight: 1, tokens: [] },
          '3': { terrain: 'sands', coastal: true, explorers: 2, towns: 0, cities: 0, dahan: 1, blight: 0, tokens: ['strife'] },
          '4': { terrain: 'mountain', coastal: false, explorers: 0, towns: 1, cities: 0, dahan: 2, blight: 0, tokens: [] },
          '5': { terrain: 'jungle', coastal: false, explorers: 1, towns: 2, cities: 1, dahan: 0, blight: 0, tokens: [] },
          '6': { terrain: 'wetland', coastal: false, explorers: 0, towns: 0, cities: 0, dahan: 3, blight: 0, tokens: [] },
          '7': { terrain: 'sands', coastal: false, explorers: 3, towns: 1, cities: 0, dahan: 1, blight: 0, tokens: ['disease'] },
          '8': { terrain: 'mountain', coastal: false, explorers: 0, towns: 0, cities: 0, dahan: 0, blight: 0, tokens: [] }
        },
        variant: 'balanced',
        variant_name: 'Board B'
      }
    },
    invader_deck: {
      ravage: { stage: 2, terrain: 'Wetland', notes: null },
      build: { stage: 2, terrain: 'Mountain', notes: null },
      explore: { stage: 2, terrain: 'Jungle', notes: null },
      upcoming: [
        { stage: 2, terrain: null, notes: null },
        { stage: 2, terrain: null, notes: null },
        { stage: 3, terrain: null, notes: null },
        { stage: 3, terrain: null, notes: null },
        { stage: 3, terrain: null, notes: null }
      ],
      discarded: [
        { stage: 1, terrain: 'Jungle', notes: null },
        { stage: 1, terrain: 'Wetland', notes: null },
        { stage: 1, terrain: 'Mountain', notes: null },
        { stage: 1, terrain: 'Sands', notes: null }
      ],
      notation: '2·2·5'
    } as InvaderDeckState,
    fear_deck: {
      deck_size: 9,
      tier_counts: [3, 3, 3] as [number, number, number],
      earned: [
        { name: 'Dahan Raid', terror_level: 2, round: 5 }
      ],
      resolved: [
        { name: 'Overseas Trade Seems Risky', terror_level: 1, round: 2 },
        { name: 'Isolation', terror_level: 1, round: 3 },
        { name: 'Flee the Pestilent Land', terror_level: 1, round: 4 }
      ],
      unseen: 5
    } as FearDeckState,
    event_deck: {
      previewed: [{ name: 'Slave Rebellion', previewed_on_turn: 5, resolved_on_turn: null }],
      resolved: [
        { name: 'Missionaries Arrive', previewed_on_turn: 1, resolved_on_turn: 2 },
        { name: 'War Touches the Islands Shores', previewed_on_turn: 2, resolved_on_turn: 3 },
        { name: 'Promising Farmlands', previewed_on_turn: 3, resolved_on_turn: 4 },
        { name: 'Urbanization', previewed_on_turn: 4, resolved_on_turn: 5 }
      ],
      unseen: 20
    } as EventDeckState,
    log: [
      { round: 1, event: 'phase_snapshot', details: { data: { round: 1, terror_level: 1, blight_current: 0, fear_current: 1 } } },
      { round: 2, event: 'phase_snapshot', details: { data: { round: 2, terror_level: 1, blight_current: 1, fear_current: 3 } } },
      { round: 2, event: 'fear_generated', details: { source: 'fast', amount: 2 } },
      { round: 2, event: 'fear_generated', details: { source: 'slow', amount: 1 } },
      { round: 3, event: 'phase_snapshot', details: { data: { round: 3, terror_level: 1, blight_current: 2, fear_current: 2 } } },
      { round: 3, event: 'fear_generated', details: { source: 'fast', amount: 3 } },
      { round: 3, event: 'fear_generated', details: { source: 'innate', amount: 2 } },
      { round: 3, event: 'fear_generated', details: { source: 'ravage', amount: 1 } },
      { round: 4, event: 'phase_snapshot', details: { data: { round: 4, terror_level: 2, blight_current: 3, fear_current: 1 } } },
      { round: 4, event: 'fear_generated', details: { source: 'fast', amount: 4 } },
      { round: 4, event: 'fear_generated', details: { source: 'slow', amount: 2 } },
      { round: 4, event: 'fear_generated', details: { source: 'event', amount: 1 } },
      { round: 5, event: 'phase_snapshot', details: { data: { round: 5, terror_level: 2, blight_current: 4, fear_current: 2 } } },
      { round: 5, event: 'fear_generated', details: { source: 'fast', amount: 3 } },
      { round: 5, event: 'fear_generated', details: { source: 'slow', amount: 4 } },
      { round: 5, event: 'fear_generated', details: { source: 'innate', amount: 2 } }
    ]
  }
}

// ─────────────────────────────────────────────────────────────────────────────
// LIFECYCLE
// ─────────────────────────────────────────────────────────────────────────────
onMounted(async () => {
  try {
    affinityMap.value = await fetchSpiritAffinity()
  } catch { /* non-blocking */ }
  
  try {
    const loaded = await fetchState()
    // Use demo data if no existing game or for showcase
    state.value = loaded ?? createDemoState()
  } catch (e) {
    // Fallback to demo data on error
    state.value = createDemoState()
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
  growth: 'Growth, Gain Energy, Play Cards',
  fast: 'Resolve fast powers and innates',
  event: 'Draw and resolve event card',
  fear: 'Resolve earned fear cards',
  invader: 'Ravage, Build, Explore',
  slow: 'Resolve slow powers and innates',
  timepasses: 'Discard played cards, clear elements',
  end: 'Game complete'
}

// Phase-aware layout: which zone is primary
const PHASE_PRIMARY: Record<Phase, 'spirits' | 'board' | 'decks' | 'analytics'> = {
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

const primaryZone = computed(() => state.value ? PHASE_PRIMARY[state.value.phase] : 'board')

// Grid template columns based on primary zone
const gridTemplate = computed(() => {
  const p = primaryZone.value
  // Layout: spirits | board | decks+analytics (stacked)
  switch (p) {
    case 'spirits': return '1fr 1fr 300px'
    case 'board': return '320px 1fr 300px'
    case 'decks': return '320px 1fr 380px'
    case 'analytics': return '1fr 1fr 1fr'
    default: return '320px 1fr 300px'
  }
})

// ─────────────────────────────────────────────────────────────────────────────
// METHODS
// ─────────────────────────────────────────────────────────────────────────────
function appendLog(event: string, details: Record<string, unknown>) {
  if (!state.value) return
  const log = (state.value.log as Array<{ round: number; event: string; details: Record<string, unknown> }>) ?? []
  state.value.log = [...log, { round: state.value.round, event, details }] as unknown as GameState['log']
}

function addFear(amount: number) {
  if (!state.value) return
  state.value.pools.fear_current += amount
  appendLog('fear_generated', { source: 'manual', amount })
  
  // Auto-bank fear cards
  const thr = state.value.pools.fear_threshold
  if (thr > 0 && state.value.pools.fear_current >= thr) {
    let remaining = state.value.pools.fear_current
    while (remaining >= thr) {
      remaining -= thr
      const fd = state.value.fear_deck ?? { deck_size: 9, tier_counts: [3, 3, 3] as [number, number, number], earned: [], resolved: [], unseen: 9 }
      state.value.fear_deck = {
        ...fd,
        earned: [...fd.earned, { name: '', terror_level: state.value.pools.terror_level, round: state.value.round }],
        unseen: Math.max(0, fd.unseen - 1),
      }
      appendLog('fear_card_earned', { terror_level: state.value.pools.terror_level, round: state.value.round, auto: true })
    }
    state.value.pools.fear_current = remaining
  }
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
      <div class="sidebar-header">
        <div class="logo">
          <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M12 2L2 7l10 5 10-5-10-5z"/>
            <path d="M2 17l10 5 10-5"/>
            <path d="M2 12l10 5 10-5"/>
          </svg>
          <span v-if="!sidebarCollapsed">Spirit Island</span>
        </div>
        <button class="sidebar-toggle" @click="sidebarCollapsed = !sidebarCollapsed" :title="sidebarCollapsed ? 'Expand' : 'Collapse'">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
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
            <span class="stat-label">Win %</span>
          </div>
        </div>
        <div class="matchup-info" v-if="matchupTag">{{ matchupTag }}</div>
      </div>

      <!-- Phase Navigator -->
      <div class="sidebar-section phase-section">
        <div class="sidebar-label" v-if="!sidebarCollapsed">Phase</div>
        <nav class="phase-nav">
          <button 
            v-for="phase in PHASES" 
            :key="phase"
            class="phase-btn"
            :class="{ active: state.phase === phase, completed: PHASES.indexOf(phase) < PHASES.indexOf(state.phase) }"
            @click="goToPhase(phase)"
            :title="sidebarCollapsed ? PHASE_LABELS[phase] : undefined"
          >
            <span class="phase-dot"></span>
            <span class="phase-label" v-if="!sidebarCollapsed">{{ PHASE_LABELS[phase] }}</span>
          </button>
        </nav>
      </div>

      <!-- Quick Stats -->
      <div class="sidebar-section" v-if="!sidebarCollapsed">
        <div class="sidebar-label">Pools</div>
        <div class="pool-stats">
          <div class="pool-row">
            <span class="pool-name">Fear</span>
            <div class="pool-bar"><div class="pool-fill fear" :style="{ width: fearProgress + '%' }"></div></div>
            <span class="pool-val">{{ state.pools.fear_current }}/{{ state.pools.fear_threshold }}</span>
          </div>
          <div class="pool-row">
            <span class="pool-name">Blight</span>
            <div class="pool-bar"><div class="pool-fill blight" :class="{ danger: blightProgress >= 80 }" :style="{ width: blightProgress + '%' }"></div></div>
            <span class="pool-val">{{ state.pools.blight_current }}/{{ state.pools.blight_cap }}</span>
          </div>
          <div class="pool-row">
            <span class="pool-name">Terror</span>
            <div class="terror-pips">
              <span :class="{ on: terrorInfo.level >= 1 }">I</span>
              <span :class="{ on: terrorInfo.level >= 2 }">II</span>
              <span :class="{ on: terrorInfo.level >= 3 }">III</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer Actions -->
      <div class="sidebar-footer">
        <button class="sidebar-btn" @click="showSavedGames = true" :title="sidebarCollapsed ? 'Load Game' : undefined">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
          <span v-if="!sidebarCollapsed">Load Game</span>
        </button>
        <button class="sidebar-btn" @click="showWizard = true" :title="sidebarCollapsed ? 'New Game' : undefined">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
          <span v-if="!sidebarCollapsed">New Game</span>
        </button>
        <button class="sidebar-btn" @click="exportGame" :title="sidebarCollapsed ? 'Export JSON' : undefined">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
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
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M15 18l-6-6 6-6"/></svg>
            </button>
            <div class="phase-info">
              <span class="phase-name">{{ PHASE_LABELS[state.phase] }}</span>
              <span class="phase-hint">{{ PHASE_HINTS[state.phase] }}</span>
            </div>
            <button class="phase-arrow next" @click="advancePhase">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M9 18l6-6-6-6"/></svg>
            </button>
          </div>
        </div>

        <div class="topbar-center">
          <div class="quick-pool">
            <span class="ql">Fear</span>
            <div class="quick-btns">
              <button @click="addFear(1)">+1</button>
              <button @click="addFear(2)">+2</button>
              <button @click="addFear(4)">+4</button>
            </div>
          </div>
          <div class="quick-pool">
            <span class="ql">Blight</span>
            <div class="quick-btns">
              <button @click="adjustBlight(-1)">-1</button>
              <button @click="adjustBlight(1)">+1</button>
            </div>
          </div>
        </div>

        <div class="topbar-right">
          <div class="save-status" :class="{ active: saving }">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/></svg>
            {{ saving ? 'Saving...' : 'Saved' }}
          </div>
        </div>
      </header>

      <!-- Victory/Defeat Banner -->
      <div v-if="endResult === 'won' && !gameOverBannerDismissed" class="banner victory">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
        <div class="banner-text"><strong>Victory!</strong> Fear deck exhausted at Terror Level {{ state.pools.terror_level }}</div>
        <button @click="gameOverBannerDismissed = true">Dismiss</button>
      </div>
      <div v-else-if="endResult === 'lost' && !gameOverBannerDismissed" class="banner defeat">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        <div class="banner-text"><strong>Defeat</strong> The island has been blighted</div>
        <button @click="gameOverBannerDismissed = true">Dismiss</button>
      </div>

      <!-- ═══════════════════════════════════════════════════════════════════════ -->
      <!-- WORKSPACE GRID                                                           -->
      <!-- ═══════════════════════════════════════════════════════════════════════ -->
      <main class="grid" :style="{ gridTemplateColumns: gridTemplate }">
        
        <!-- SPIRITS ZONE -->
        <section class="zone spirits" :class="{ primary: primaryZone === 'spirits' }">
          <SpiritZone 
            :spirits="state.spirits" 
            :slugs="Object.keys(state.spirits)"
            :round="state.round"
            @update:spirits="(s) => state.spirits = s"
            @log-event="appendLog"
          />
        </section>

        <!-- BOARD ZONE -->
        <section class="zone board" :class="{ primary: primaryZone === 'board' }">
          <BoardZone
            :boards="state.board_state"
            :spirits="state.spirits"
            @update:board="(id, b) => state.board_state[id] = b"
          />
        </section>

        <!-- RIGHT PANEL: Decks + Analytics stacked -->
        <section class="zone right-panel" :class="{ primary: primaryZone === 'decks' || primaryZone === 'analytics' }">
          <div class="decks-area">
            <DecksZone
              :invader-deck="state.invader_deck"
              :fear-deck="state.fear_deck"
              :event-deck="state.event_deck"
              :terror-level="state.pools.terror_level"
              :phase="state.phase"
              :round="state.round"
              @update:invader-deck="(d) => state.invader_deck = d"
              @update:fear-deck="(d) => state.fear_deck = d"
              @update:event-deck="(d) => state.event_deck = d"
              @log-event="appendLog"
            />
          </div>
          <div class="analytics-area" :class="{ expanded: primaryZone === 'analytics' }">
            <AnalyticsZone
              :state="state"
              :win-prob="currentWinProb"
              :expanded="primaryZone === 'analytics'"
            />
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
  max-height: 100vh;
  background: var(--color-surface-base, #0f0f12);
  overflow: hidden;
}

.loading-screen,
.error-screen {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  gap: 16px;
  color: var(--color-text-secondary, #888);
  background: var(--color-surface-base, #0f0f12);
}

.loader {
  width: 28px;
  height: 28px;
  border: 2px solid var(--color-border, #2a2a32);
  border-top-color: var(--color-accent, #6366f1);
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}

@keyframes spin { to { transform: rotate(360deg); } }

.error-box {
  text-align: center;
  padding: 32px;
  background: var(--color-surface-raised, #1e1e24);
  border: 1px solid var(--color-border, #2a2a32);
  border-radius: 12px;
  max-width: 360px;
}

.error-box h2 { margin-bottom: 8px; font-size: 18px; color: var(--color-text-primary, #fff); }
.error-box p { color: var(--color-text-secondary, #888); margin-bottom: 16px; }

/* ═══════════════════════════════════════════════════════════════════════════ */
/* SIDEBAR                                                                      */
/* ═══════════════════════════════════════════════════════════════════════════ */
.sidebar {
  display: flex;
  flex-direction: column;
  width: 200px;
  background: var(--color-surface-raised, #1e1e24);
  border-right: 1px solid var(--color-border, #2a2a32);
  transition: width 180ms ease;
  flex-shrink: 0;
}

.sidebar.collapsed { width: 52px; }

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  border-bottom: 1px solid var(--color-border, #2a2a32);
  min-height: 48px;
}

.logo {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
  font-size: 13px;
  color: var(--color-text-primary, #fff);
  white-space: nowrap;
}
.logo svg { color: var(--color-accent, #6366f1); flex-shrink: 0; }

.sidebar-toggle {
  width: 24px;
  height: 24px;
  padding: 0;
  background: transparent;
  border: none;
  color: var(--color-text-tertiary, #666);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
}
.sidebar-toggle:hover { background: var(--color-surface-hover, #2a2a32); color: var(--color-text-primary, #fff); }

.sidebar.collapsed .sidebar-toggle { margin: 0 auto; }

.sidebar-section {
  padding: 12px;
  border-bottom: 1px solid var(--color-border, #2a2a32);
}

.sidebar-label {
  font-size: 9px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--color-text-tertiary, #666);
  margin-bottom: 10px;
}

/* Game Info */
.game-info {
  display: flex;
  gap: 16px;
  margin-bottom: 8px;
}

.game-stat { display: flex; flex-direction: column; }
.stat-value {
  font-family: var(--font-mono, monospace);
  font-size: 22px;
  font-weight: 700;
  color: var(--color-text-primary, #fff);
  line-height: 1;
}
.stat-value.good { color: #22c55e; }
.stat-label {
  font-size: 10px;
  color: var(--color-text-tertiary, #666);
  margin-top: 2px;
}

.matchup-info {
  font-size: 10px;
  color: var(--color-text-secondary, #888);
  text-transform: capitalize;
  padding: 6px 8px;
  background: var(--color-surface-sunken, #16161a);
  border-radius: 4px;
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
  gap: 10px;
  height: 32px;
  padding: 0 10px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: var(--color-text-tertiary, #666);
  transition: all 150ms ease;
}
.sidebar.collapsed .phase-btn { justify-content: center; padding: 0; }
.phase-btn:hover { background: var(--color-surface-hover, #2a2a32); color: var(--color-text-secondary, #888); }
.phase-btn.active { background: var(--color-surface-sunken, #16161a); color: var(--color-text-primary, #fff); }
.phase-btn.completed { color: var(--color-text-secondary, #888); }

.phase-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--color-surface-sunken, #16161a);
  flex-shrink: 0;
  transition: all 150ms ease;
}
.phase-btn.active .phase-dot { background: var(--color-accent, #6366f1); box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.2); }
.phase-btn.completed .phase-dot { background: #22c55e; }

.phase-label { font-size: 12px; white-space: nowrap; }

/* Pool Stats */
.pool-stats { display: flex; flex-direction: column; gap: 10px; }

.pool-row {
  display: grid;
  grid-template-columns: 42px 1fr auto;
  gap: 8px;
  align-items: center;
}
.pool-name { font-size: 10px; color: var(--color-text-tertiary, #666); }
.pool-bar { height: 5px; background: var(--color-surface-sunken, #16161a); border-radius: 3px; overflow: hidden; }
.pool-fill { height: 100%; border-radius: 3px; transition: width 200ms ease; }
.pool-fill.fear { background: #f59e0b; }
.pool-fill.blight { background: #22c55e; }
.pool-fill.blight.danger { background: #ef4444; }
.pool-val { font-family: var(--font-mono, monospace); font-size: 10px; color: var(--color-text-secondary, #888); }

.terror-pips { display: flex; gap: 3px; }
.terror-pips span {
  font-family: var(--font-mono, monospace);
  font-size: 9px;
  font-weight: 700;
  color: var(--color-text-faint, #444);
  padding: 2px 5px;
  background: var(--color-surface-sunken, #16161a);
  border-radius: 3px;
}
.terror-pips span.on { background: #ef4444; color: #fff; }

/* Sidebar Footer */
.sidebar-footer {
  margin-top: auto;
  padding: 8px;
  border-top: 1px solid var(--color-border, #2a2a32);
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.sidebar-btn {
  display: flex;
  align-items: center;
  gap: 10px;
  height: 32px;
  padding: 0 10px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: var(--color-text-tertiary, #666);
  font-size: 12px;
}
.sidebar.collapsed .sidebar-btn { justify-content: center; padding: 0; }
.sidebar-btn:hover { background: var(--color-surface-hover, #2a2a32); color: var(--color-text-primary, #fff); }
.sidebar-btn svg { flex-shrink: 0; }

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
  height: 44px;
  padding: 0 16px;
  background: var(--color-surface-raised, #1e1e24);
  border-bottom: 1px solid var(--color-border, #2a2a32);
  flex-shrink: 0;
}

.topbar-left, .topbar-center, .topbar-right { display: flex; align-items: center; gap: 16px; }

.phase-display { display: flex; align-items: center; gap: 8px; }

.phase-arrow {
  width: 26px;
  height: 26px;
  padding: 0;
  background: var(--color-surface-sunken, #16161a);
  border: 1px solid var(--color-border, #2a2a32);
  border-radius: 6px;
  color: var(--color-text-secondary, #888);
  display: flex;
  align-items: center;
  justify-content: center;
}
.phase-arrow:hover:not(:disabled) { background: var(--color-surface-hover, #2a2a32); color: var(--color-text-primary, #fff); }
.phase-arrow:disabled { opacity: 0.3; }
.phase-arrow.next { background: var(--color-accent, #6366f1); border-color: var(--color-accent, #6366f1); color: #fff; }
.phase-arrow.next:hover { filter: brightness(1.1); }

.phase-info { display: flex; flex-direction: column; min-width: 120px; }
.phase-name { font-size: 14px; font-weight: 600; color: var(--color-text-primary, #fff); }
.phase-hint { font-size: 10px; color: var(--color-text-tertiary, #666); }

/* Quick Pools */
.quick-pool { display: flex; align-items: center; gap: 6px; }
.ql { font-size: 10px; font-weight: 500; color: var(--color-text-tertiary, #666); text-transform: uppercase; letter-spacing: 0.03em; }
.quick-btns { display: flex; gap: 2px; }
.quick-btns button {
  height: 24px;
  padding: 0 8px;
  font-size: 11px;
  font-family: var(--font-mono, monospace);
  background: var(--color-surface-sunken, #16161a);
  border: 1px solid var(--color-border, #2a2a32);
  border-radius: 4px;
  color: var(--color-text-secondary, #888);
}
.quick-btns button:hover { background: var(--color-surface-hover, #2a2a32); color: var(--color-text-primary, #fff); }

.save-status {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 10px;
  color: var(--color-text-tertiary, #666);
}
.save-status.active { color: var(--color-accent, #6366f1); }

/* Banner */
.banner {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 16px;
  flex-shrink: 0;
}
.banner.victory { background: rgba(34, 197, 94, 0.1); border-bottom: 1px solid rgba(34, 197, 94, 0.2); color: #22c55e; }
.banner.defeat { background: rgba(239, 68, 68, 0.1); border-bottom: 1px solid rgba(239, 68, 68, 0.2); color: #ef4444; }
.banner-text { flex: 1; font-size: 13px; }
.banner-text strong { font-weight: 600; }
.banner button { background: transparent; border: none; color: inherit; opacity: 0.6; font-size: 11px; padding: 4px 8px; }
.banner button:hover { opacity: 1; }

/* ═══════════════════════════════════════════════════════════════════════════ */
/* WORKSPACE GRID                                                               */
/* ═══════════════════════════════════════════════════════════════════════════ */
.grid {
  display: grid;
  gap: 1px;
  flex: 1;
  min-height: 0;
  background: var(--color-border, #2a2a32);
  transition: grid-template-columns 250ms ease;
}

.zone {
  display: flex;
  flex-direction: column;
  background: var(--color-surface-base, #0f0f12);
  overflow: hidden;
  min-height: 0;
}

.zone.primary { background: color-mix(in oklch, var(--color-accent, #6366f1) 3%, var(--color-surface-base, #0f0f12)); }

.spirits { overflow-y: auto; }
.board { overflow: hidden; }

.right-panel {
  display: flex;
  flex-direction: column;
  gap: 1px;
  background: var(--color-border, #2a2a32);
}

.decks-area {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  background: var(--color-surface-base, #0f0f12);
}

.analytics-area {
  height: 160px;
  flex-shrink: 0;
  background: var(--color-surface-base, #0f0f12);
  padding: 12px;
  overflow: hidden;
  transition: height 250ms ease;
}

.analytics-area.expanded {
  height: 320px;
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* RESPONSIVE                                                                   */
/* ═══════════════════════════════════════════════════════════════════════════ */
@media (max-width: 1200px) {
  .sidebar { width: 52px; }
  .sidebar .logo span,
  .sidebar .sidebar-label,
  .sidebar .phase-label,
  .sidebar .sidebar-btn span,
  .sidebar .game-info,
  .sidebar .matchup-info,
  .sidebar .pool-stats { display: none; }
  .sidebar .phase-btn { justify-content: center; padding: 0; }
  .sidebar .sidebar-btn { justify-content: center; padding: 0; }
  
  .grid {
    grid-template-columns: 1fr 1fr !important;
    grid-template-rows: 1fr auto;
  }
  .right-panel { grid-column: 1 / -1; flex-direction: row; height: 200px; }
  .decks-area { flex: 2; }
  .analytics-area { flex: 1; height: auto; }
}

@media (max-width: 900px) {
  .grid { grid-template-columns: 1fr !important; }
  .right-panel { flex-direction: column; height: auto; }
  .topbar-center { display: none; }
}
</style>
