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
import TerrainTimeline from './components/TerrainTimeline.vue'

const state = ref<GameState | null>(null)
const error = ref<string | null>(null)
const saving = ref(false)
const showWizard = ref(false)
const showSavedGames = ref(false)
let saveTimer: number | null = null

// Active spirit tab for multi-spirit games
const activeSpiritTab = ref<string | null>(null)

// Collapsible sections
const expandedSections = ref<Set<string>>(new Set(['spirits', 'invader']))

function toggleSection(section: string) {
  if (expandedSections.value.has(section)) {
    expandedSections.value.delete(section)
  } else {
    expandedSections.value.add(section)
  }
  expandedSections.value = new Set(expandedSections.value)
}

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

// Phase info
const PHASE_INFO: Record<Phase, { label: string; hint: string; color: string }> = {
  setup: { label: 'Setup', hint: 'Configure game', color: 'var(--accent-blue)' },
  growth: { label: 'Spirit Phase', hint: 'Growth + Gain Energy + Play Cards', color: 'var(--accent-green)' },
  fast: { label: 'Fast Powers', hint: 'Resolve Fast cards & Innates', color: 'var(--accent-purple)' },
  event: { label: 'Event', hint: 'Draw & resolve Event card', color: 'var(--accent-violet)' },
  fear: { label: 'Fear', hint: 'Resolve earned Fear cards', color: 'var(--accent-amber)' },
  invader: { label: 'Invader Phase', hint: 'Ravage → Build → Explore', color: 'var(--accent-red)' },
  slow: { label: 'Slow Powers', hint: 'Resolve Slow cards & Innates', color: 'var(--accent-teal)' },
  timepasses: { label: 'Time Passes', hint: 'Discard cards, clear damage', color: 'var(--text-muted)' },
  end: { label: 'Game End', hint: 'Victory or Defeat', color: 'var(--accent-amber)' },
}

const phaseInfo = computed(() => PHASE_INFO[state.value?.phase ?? 'setup'])

// Quick phase navigation
const PHASE_ORDER: Phase[] = ['growth', 'fast', 'event', 'fear', 'invader', 'slow', 'timepasses']

function nextPhase() {
  if (!state.value) return
  const idx = PHASE_ORDER.indexOf(state.value.phase)
  if (idx >= 0 && idx < PHASE_ORDER.length - 1) {
    state.value.phase = PHASE_ORDER[idx + 1]
  }
}

function prevPhase() {
  if (!state.value) return
  const idx = PHASE_ORDER.indexOf(state.value.phase)
  if (idx > 0) {
    state.value.phase = PHASE_ORDER[idx - 1]
  }
}

// Quick fear buttons
function addFear(amount: number) {
  if (!state.value) return
  state.value.pools.fear_current += amount
}

// Calculate fear progress percentage
const fearProgress = computed(() => {
  if (!state.value) return 0
  const { fear_current, fear_threshold } = state.value.pools
  return Math.min(100, (fear_current / fear_threshold) * 100)
})

// Calculate blight progress percentage
const blightProgress = computed(() => {
  if (!state.value) return 0
  const { blight_current, blight_cap } = state.value.pools
  return Math.min(100, (blight_current / blight_cap) * 100)
})

// Terror track visualization
const terrorTrack = computed(() => {
  if (!state.value) return { level: 1, t1: 3, t2: 3, t3: 3, progress: 0 }
  const fd = state.value.fear_deck
  if (!fd) return { level: 1, t1: 3, t2: 3, t3: 3, progress: 0 }
  const [t1, t2, t3] = fd.tier_counts ?? [3, 3, 3]
  const total = fd.resolved.length + fd.earned.length
  return { level: state.value.pools.terror_level, t1, t2, t3, progress: total }
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

  <div v-else class="dashboard" :data-phase="state.phase">
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <!-- TOP BAR: Phase + Round + Quick Stats                                    -->
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <header class="top-bar">
      <div class="top-left">
        <div class="brand">
          <span class="brand-mark">SI</span>
          <span class="brand-name">Spirit Island Tracker</span>
        </div>
        <div class="matchup" v-if="matchupTag">{{ matchupTag }}</div>
      </div>

      <div class="phase-nav">
        <button class="phase-arrow" @click="prevPhase" :disabled="PHASE_ORDER.indexOf(state.phase) <= 0">
          <span>&#8249;</span>
        </button>
        <div class="phase-current" :style="{ '--phase-color': phaseInfo.color }">
          <span class="phase-label">{{ phaseInfo.label }}</span>
          <span class="phase-hint">{{ phaseInfo.hint }}</span>
        </div>
        <button class="phase-arrow" @click="nextPhase" :disabled="PHASE_ORDER.indexOf(state.phase) >= PHASE_ORDER.length - 1">
          <span>&#8250;</span>
        </button>
      </div>

      <div class="top-right">
        <div class="round-badge">
          <span class="round-label">Round</span>
          <span class="round-num">{{ state.round }}</span>
        </div>
        <div class="win-prob" :class="{ good: currentWinProb.mean >= 0.5, bad: currentWinProb.mean < 0.5 }">
          {{ Math.round(currentWinProb.mean * 100) }}%
        </div>
        <div class="header-actions">
          <button class="btn-icon" @click="showSavedGames = true" title="Saved Games">
            <span>&#128193;</span>
          </button>
          <button class="btn-icon" @click="showWizard = true" title="New Game">
            <span>+</span>
          </button>
          <button class="btn-icon" @click="exportCurrentGame" title="Export">
            <span>&#8595;</span>
          </button>
        </div>
      </div>
    </header>

    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <!-- GAME STATUS BAR: Fear / Blight / Terror                                 -->
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <div class="status-bar">
      <!-- Fear Pool -->
      <div class="status-item fear-status">
        <div class="status-header">
          <span class="status-label">Fear</span>
          <span class="status-value">{{ state.pools.fear_current }} / {{ state.pools.fear_threshold }}</span>
        </div>
        <div class="status-track">
          <div class="track-fill fear-fill" :style="{ width: fearProgress + '%' }"></div>
        </div>
        <div class="quick-btns">
          <button class="quick-btn" @click="addFear(1)">+1</button>
          <button class="quick-btn" @click="addFear(2)">+2</button>
          <button class="quick-btn" @click="addFear(4)">+4</button>
        </div>
      </div>

      <!-- Terror Level -->
      <div class="status-item terror-status">
        <div class="status-header">
          <span class="status-label">Terror Level</span>
          <span class="status-value terror-value">{{ terrorTrack.level }}</span>
        </div>
        <div class="terror-track">
          <div class="terror-tier" :class="{ active: terrorTrack.level >= 1, current: terrorTrack.level === 1 }">
            <span class="tier-num">I</span>
            <span class="tier-cards">{{ terrorTrack.t1 }}</span>
          </div>
          <div class="terror-tier" :class="{ active: terrorTrack.level >= 2, current: terrorTrack.level === 2 }">
            <span class="tier-num">II</span>
            <span class="tier-cards">{{ terrorTrack.t2 }}</span>
          </div>
          <div class="terror-tier" :class="{ active: terrorTrack.level >= 3, current: terrorTrack.level === 3 }">
            <span class="tier-num">III</span>
            <span class="tier-cards">{{ terrorTrack.t3 }}</span>
          </div>
          <div class="terror-tier victory">
            <span class="tier-num">&#10003;</span>
          </div>
        </div>
        <div class="terror-progress">
          {{ terrorTrack.progress }} / {{ terrorTrack.t1 + terrorTrack.t2 + terrorTrack.t3 }} cards earned
        </div>
      </div>

      <!-- Blight Pool -->
      <div class="status-item blight-status">
        <div class="status-header">
          <span class="status-label">Blight</span>
          <span class="status-value">{{ state.pools.blight_current }} / {{ state.pools.blight_cap }}</span>
        </div>
        <div class="status-track">
          <div class="track-fill blight-fill" :class="{ danger: blightProgress >= 80 }" :style="{ width: blightProgress + '%' }"></div>
        </div>
        <div class="blight-warning" v-if="blightProgress >= 80">
          Approaching defeat!
        </div>
      </div>
    </div>

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

    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <!-- MAIN CONTENT: Two-column layout                                         -->
    <!-- ═══════════════════════════════════════════════════════════════════════ -->
    <main class="main-area">
      <!-- LEFT: Spirits + Island Board -->
      <div class="main-left">
        <!-- Spirits Section -->
        <section class="card spirits-card" :class="{ collapsed: !expandedSections.has('spirits') }">
          <header class="card-header" @click="toggleSection('spirits')">
            <h2>
              <span class="collapse-icon">{{ expandedSections.has('spirits') ? '&#9660;' : '&#9654;' }}</span>
              Spirits
            </h2>
            <div class="spirit-tabs" v-if="spiritSlugs.length > 1 && expandedSections.has('spirits')">
              <button
                v-for="slug in spiritSlugs"
                :key="slug"
                class="spirit-tab"
                :class="{ active: activeSpiritTab === slug }"
                @click.stop="activeSpiritTab = slug"
              >
                {{ humanSlug(slug).split(' ').slice(0, 2).join(' ') }}
              </button>
            </div>
            <div class="spirit-summary" v-if="!expandedSections.has('spirits')">
              <span v-for="slug in spiritSlugs" :key="slug" class="spirit-mini">
                {{ humanSlug(slug).split(' ')[0] }}:
                <strong>{{ state.spirits[slug]?.energy ?? 0 }}E</strong>
              </span>
            </div>
          </header>
          <div class="card-body" v-show="expandedSections.has('spirits')">
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

        <!-- Island Board Section -->
        <section class="card board-card" :class="{ collapsed: !expandedSections.has('board') }">
          <header class="card-header" @click="toggleSection('board')">
            <h2>
              <span class="collapse-icon">{{ expandedSections.has('board') ? '&#9660;' : '&#9654;' }}</span>
              Island Board
            </h2>
            <div class="board-chips" v-if="expandedSections.has('board')">
              <span v-for="b in (state.setup.boards ?? ['A'])" :key="b" class="chip">{{ b }}</span>
            </div>
          </header>
          <div class="card-body board-body" v-show="expandedSections.has('board')">
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
      </div>

      <!-- RIGHT: Decks + Actions + Log -->
      <div class="main-right">
        <!-- Invader Deck -->
        <section class="card invader-card" :class="{ collapsed: !expandedSections.has('invader') }">
          <header class="card-header" @click="toggleSection('invader')">
            <h2>
              <span class="collapse-icon">{{ expandedSections.has('invader') ? '&#9660;' : '&#9654;' }}</span>
              Invader Deck
            </h2>
            <div class="invader-preview" v-if="!expandedSections.has('invader') && state.invader_deck">
              <span class="preview-slot ravage">R: {{ state.invader_deck.ravage?.terrain ?? '-' }}</span>
              <span class="preview-slot build">B: {{ state.invader_deck.build?.terrain ?? '-' }}</span>
              <span class="preview-slot explore">E: {{ state.invader_deck.explore?.terrain ?? '-' }}</span>
            </div>
          </header>
          <div class="card-body" v-show="expandedSections.has('invader')">
            <InvaderDeck v-model="state.invader_deck" @log-event="(event, details) => appendLog(event, details)" />
          </div>
        </section>

        <!-- Fear & Event Decks -->
        <section class="card decks-card">
          <header class="card-header">
            <h2>Fear & Events</h2>
          </header>
          <div class="card-body decks-grid">
            <div class="mini-deck fear-deck">
              <h3>Fear Deck</h3>
              <FearDeck v-model="state.fear_deck" :round="state.round" :terror-level="state.pools.terror_level" @log-event="(event, details) => appendLog(event, details)" />
            </div>
            <div class="mini-deck event-deck" v-if="state.event_deck">
              <h3>Event Deck</h3>
              <EventDeck v-model="state.event_deck" :round="state.round" @log-event="(event, details) => appendLog(event, details)" />
            </div>
          </div>
        </section>

        <!-- Turn Controller -->
        <section class="card turn-card">
          <header class="card-header">
            <h2>Turn Log & Actions</h2>
            <span v-if="saving" class="save-badge">Saving...</span>
          </header>
          <div class="card-body turn-body">
            <TurnController v-model="state" />
          </div>
        </section>
      </div>
    </main>

    <!-- Modals -->
    <SetupWizard :show="showWizard" @close="showWizard = false" @game-started="onGameStarted" />
    <SavedGames :show="showSavedGames" @close="showSavedGames = false" @game-loaded="onGameStarted" />
  </div>
</template>

<style scoped>
/* ═══════════════════════════════════════════════════════════════════════════ */
/* DASHBOARD LAYOUT                                                             */
/* ═══════════════════════════════════════════════════════════════════════════ */

.dashboard {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-canvas);
  overflow: hidden;
}

/* ─── ERROR & LOADING ─── */
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
/* TOP BAR                                                                      */
/* ═══════════════════════════════════════════════════════════════════════════ */

.top-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-4);
  padding: var(--sp-2) var(--sp-4);
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.top-left {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
}

.brand {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.brand-mark {
  width: 32px;
  height: 32px;
  background: linear-gradient(135deg, var(--accent-purple), var(--accent-violet));
  border-radius: var(--r-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: var(--fw-bold);
  font-size: var(--fs-sm);
  color: white;
}

.brand-name {
  font-weight: var(--fw-semibold);
  font-size: var(--fs-base);
  color: var(--text-white);
}

.matchup {
  font-size: var(--fs-xs);
  padding: 4px var(--sp-2);
  background: var(--bg-muted);
  border-radius: var(--r-full);
  color: var(--text-secondary);
  text-transform: capitalize;
}

/* Phase Navigation */
.phase-nav {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.phase-arrow {
  width: 32px;
  height: 32px;
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  color: var(--text-secondary);
  font-size: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all var(--motion-fast);
}

.phase-arrow:hover:not(:disabled) {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.phase-arrow:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.phase-current {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--sp-1) var(--sp-4);
  background: color-mix(in srgb, var(--phase-color) 12%, var(--bg-inset));
  border: 1px solid var(--phase-color);
  border-radius: var(--r-md);
  min-width: 180px;
}

.phase-label {
  font-weight: var(--fw-semibold);
  color: var(--text-white);
  font-size: var(--fs-sm);
}

.phase-hint {
  font-size: var(--fs-xs);
  color: var(--text-muted);
}

/* Top Right */
.top-right {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
}

.round-badge {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--sp-1) var(--sp-3);
  background: var(--bg-inset);
  border-radius: var(--r-md);
}

.round-label {
  font-size: 0.6rem;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-muted);
}

.round-num {
  font-family: var(--font-mono);
  font-size: var(--fs-lg);
  font-weight: var(--fw-bold);
  color: var(--accent-amber);
}

.win-prob {
  font-family: var(--font-mono);
  font-size: var(--fs-lg);
  font-weight: var(--fw-bold);
  padding: var(--sp-1) var(--sp-3);
  border-radius: var(--r-md);
}

.win-prob.good {
  background: rgba(82, 183, 136, 0.15);
  color: var(--status-success);
}

.win-prob.bad {
  background: rgba(220, 47, 2, 0.15);
  color: var(--status-danger);
}

.header-actions {
  display: flex;
  gap: var(--sp-1);
}

.btn-icon {
  width: 32px;
  height: 32px;
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  color: var(--text-secondary);
  font-size: var(--fs-base);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all var(--motion-fast);
}

.btn-icon:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* STATUS BAR                                                                   */
/* ═══════════════════════════════════════════════════════════════════════════ */

.status-bar {
  display: grid;
  grid-template-columns: 1fr 1.5fr 1fr;
  gap: var(--sp-3);
  padding: var(--sp-3) var(--sp-4);
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.status-item {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.status-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
}

.status-label {
  font-size: var(--fs-xs);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
}

.status-value {
  font-family: var(--font-mono);
  font-size: var(--fs-sm);
  font-weight: var(--fw-semibold);
  color: var(--text-primary);
}

.status-track {
  height: 8px;
  background: var(--bg-canvas);
  border-radius: var(--r-full);
  overflow: hidden;
}

.track-fill {
  height: 100%;
  border-radius: var(--r-full);
  transition: width var(--motion-base);
}

.fear-fill {
  background: linear-gradient(90deg, var(--accent-amber), #f4a261);
}

.blight-fill {
  background: linear-gradient(90deg, var(--accent-green), var(--accent-teal));
}

.blight-fill.danger {
  background: linear-gradient(90deg, var(--accent-coral), var(--accent-red));
}

.quick-btns {
  display: flex;
  gap: var(--sp-1);
}

.quick-btn {
  padding: 2px var(--sp-2);
  font-size: var(--fs-xs);
  font-family: var(--font-mono);
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  color: var(--accent-amber);
  cursor: pointer;
  transition: all var(--motion-fast);
}

.quick-btn:hover {
  background: var(--bg-hover);
  border-color: var(--accent-amber);
}

/* Terror Track */
.terror-value {
  font-size: var(--fs-lg);
  color: var(--accent-coral);
}

.terror-track {
  display: flex;
  gap: 2px;
}

.terror-tier {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--sp-1);
  background: var(--bg-canvas);
  border-radius: var(--r-sm);
  border: 1px solid var(--border-subtle);
  transition: all var(--motion-fast);
}

.terror-tier.active {
  background: rgba(233, 196, 106, 0.1);
  border-color: var(--accent-amber);
}

.terror-tier.current {
  background: rgba(233, 196, 106, 0.25);
  border-color: var(--accent-amber);
  box-shadow: 0 0 0 2px rgba(233, 196, 106, 0.3);
}

.terror-tier.victory {
  background: rgba(82, 183, 136, 0.1);
  border-color: var(--status-success);
  color: var(--status-success);
}

.tier-num {
  font-weight: var(--fw-bold);
  font-size: var(--fs-xs);
  color: var(--text-secondary);
}

.terror-tier.active .tier-num,
.terror-tier.current .tier-num {
  color: var(--accent-amber);
}

.tier-cards {
  font-family: var(--font-mono);
  font-size: 0.65rem;
  color: var(--text-muted);
}

.terror-progress {
  font-size: var(--fs-xs);
  color: var(--text-muted);
  text-align: center;
}

.blight-warning {
  font-size: var(--fs-xs);
  color: var(--status-danger);
  font-weight: var(--fw-medium);
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* GAME BANNERS                                                                 */
/* ═══════════════════════════════════════════════════════════════════════════ */

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

.btn-primary {
  background: var(--accent-violet);
  color: white;
  border: none;
  padding: var(--sp-2) var(--sp-4);
  border-radius: var(--r-md);
  font-weight: var(--fw-medium);
  cursor: pointer;
}

.btn-primary:hover {
  filter: brightness(1.15);
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* MAIN AREA                                                                    */
/* ═══════════════════════════════════════════════════════════════════════════ */

.main-area {
  display: grid;
  grid-template-columns: 1fr 400px;
  gap: var(--sp-3);
  padding: var(--sp-3) var(--sp-4);
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.main-left,
.main-right {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
  min-height: 0;
  overflow: auto;
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* CARDS                                                                        */
/* ═══════════════════════════════════════════════════════════════════════════ */

.card {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-lg);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-3);
  padding: var(--sp-2) var(--sp-3);
  background: var(--bg-raised);
  border-bottom: 1px solid var(--border-subtle);
  cursor: pointer;
  user-select: none;
  flex-shrink: 0;
}

.card-header h2 {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  font-size: var(--fs-sm);
  font-weight: var(--fw-semibold);
  color: var(--text-white);
  margin: 0;
}

.collapse-icon {
  font-size: 0.7rem;
  color: var(--text-muted);
  transition: transform var(--motion-fast);
}

.card-body {
  flex: 1;
  overflow: auto;
  padding: var(--sp-3);
  min-height: 0;
}

.card.collapsed .card-body {
  display: none;
}

/* ─── Spirits Card ─── */
.spirits-card {
  border-left: 3px solid var(--accent-purple);
}

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

.spirit-summary {
  display: flex;
  gap: var(--sp-3);
}

.spirit-mini {
  font-size: var(--fs-xs);
  color: var(--text-muted);
}

.spirit-mini strong {
  color: var(--accent-amber);
}

/* ─── Board Card ─── */
.board-card {
  border-left: 3px solid var(--accent-teal);
  flex: 1;
  min-height: 200px;
}

.board-body {
  display: flex;
  gap: var(--sp-3);
  flex-wrap: wrap;
}

.board-chips {
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

/* ─── Invader Card ─── */
.invader-card {
  border-left: 3px solid var(--accent-red);
}

.invader-preview {
  display: flex;
  gap: var(--sp-2);
}

.preview-slot {
  font-size: var(--fs-xs);
  font-family: var(--font-mono);
  padding: 2px var(--sp-2);
  background: var(--bg-inset);
  border-radius: var(--r-sm);
  color: var(--text-muted);
}

.preview-slot.ravage { color: var(--accent-red); }
.preview-slot.build { color: var(--accent-amber); }
.preview-slot.explore { color: var(--accent-blue); }

/* ─── Decks Card ─── */
.decks-card {
  border-left: 3px solid var(--accent-amber);
}

.decks-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--sp-3);
}

.mini-deck {
  background: var(--bg-inset);
  border-radius: var(--r-md);
  padding: var(--sp-2);
}

.mini-deck h3 {
  font-size: var(--fs-xs);
  font-weight: var(--fw-semibold);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin: 0 0 var(--sp-2) 0;
}

/* ─── Turn Card ─── */
.turn-card {
  border-left: 3px solid var(--accent-blue);
  flex: 1;
  min-height: 300px;
}

.turn-body {
  padding: 0;
}

.save-badge {
  font-size: var(--fs-xs);
  color: var(--status-success);
  font-style: italic;
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/* RESPONSIVE                                                                   */
/* ═══════════════════════════════════════════════════════════════════════════ */

@media (max-width: 1200px) {
  .main-area {
    grid-template-columns: 1fr;
  }

  .main-right {
    flex-direction: row;
    flex-wrap: wrap;
  }

  .main-right > .card {
    flex: 1;
    min-width: 300px;
  }
}

@media (max-width: 900px) {
  .top-bar {
    flex-wrap: wrap;
    gap: var(--sp-2);
  }

  .phase-nav {
    order: 3;
    width: 100%;
    justify-content: center;
  }

  .status-bar {
    grid-template-columns: 1fr 1fr;
  }

  .terror-status {
    grid-column: span 2;
  }

  .brand-name {
    display: none;
  }

  .decks-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 600px) {
  .status-bar {
    grid-template-columns: 1fr;
  }

  .terror-status {
    grid-column: span 1;
  }

  .top-right {
    gap: var(--sp-2);
  }

  .win-prob {
    font-size: var(--fs-sm);
    padding: var(--sp-1) var(--sp-2);
  }
}
</style>
