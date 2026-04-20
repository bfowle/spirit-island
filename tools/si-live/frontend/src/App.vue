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
import SectionNav from './components/SectionNav.vue'
import StickyStatus from './components/StickyStatus.vue'
import SectionStrip from './components/SectionStrip.vue'

const state = ref<GameState | null>(null)
const error = ref<string | null>(null)
const saving = ref(false)
const showWizard = ref(false)
const showSavedGames = ref(false)
let saveTimer: number | null = null

// "focus" determines which sections are prominent vs. collapsed. Default auto-follows
// the active phase; user can pin focus to a specific view via the side tabs to
// temporarily override.
type FocusMode = 'auto' | 'board' | 'decks' | 'spirits' | 'stats' | 'retro' | 'all'
const focusOverride = ref<FocusMode>('auto')

function onGameStarted(newState: GameState) {
  state.value = newState
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
    outcome: endResult.value,  // 'won' | 'lost' | null
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
  // Prime the auto-log watcher's cache so the reset itself doesn't generate
  // a spurious "blight_removed / fear_generated" delta next tick.
  lastFear = 0
}

/** Invoked by Board → LandEditor when the user adjusts a per-land value that
 *  mirrors a board-level pool. Currently only blight uses this — placing/removing
 *  blight on a land must sync with `pools.blight_current`. The pool watcher will
 *  auto-log the delta as blight_added / blight_removed. */
function bumpPool(pool: 'blight', delta: number) {
  if (!state.value) return
  if (pool === 'blight') {
    const current = state.value.pools.blight_current ?? 0
    state.value.pools.blight_current = Math.max(0, current + delta)
  }
}

// Dedicated fear-deck watcher: advances terror level + resyncs unseen based
// on total consumed (resolved + earned) regardless of how the cards got there.
// The pool watcher only catches bank-time crossings; direct resolve/earn
// manipulation via the FearDeck UI needs its own trigger so terror keeps up
// with reality. unseen resync prevents drift from tier-count edits.
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
    // Keep unseen in sync with the authoritative card lists
    const expectedUnseen = Math.max(0, fd.deck_size - total)
    if (fd.unseen !== expectedUnseen) {
      fd.unseen = expectedUnseen
    }
  },
  { deep: true },
)

// --- Game-end detection ----------------------------------------------------
// Delegated to the shared lib so the status bar, charts, and banner all use
// one source of truth. `endState` returns 'won' | 'lost' | 'imminent' | 'in-progress'.
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

// Auto-log fear/blight deltas regardless of whether they came from Quick-fear
// buttons, direct input editing, or any other path. The retrospective chart
// derives from these log entries, so we need *every* change captured.
// ALSO auto-banks fear cards when the pool crosses threshold — moved here
// from TurnController so direct input edits trigger banking too.
let lastFear = 0
let lastBlight = 0
let lastSuppressRound = -1  // prevent loops during round resets
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

    // Auto-bank fear cards if pool crosses threshold. Applies regardless of
    // how fear was bumped (Quick buttons, input edit, watcher-injected). Each
    // crossing banks one face-down card; overflow carries forward.
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

      // Terror advance based on total consumed (resolved + earned) vs deck thirds
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

// Density preference — 'comfortable' adds extra padding for normal-zoom use.
// Stored on the html element so CSS custom props cascade everywhere.
const density = ref<'compact' | 'comfortable'>('comfortable')
watch(density, (d) => {
  document.documentElement.dataset.density = d
}, { immediate: true })

// Each section declares which phases it's "primary" vs "secondary" in. The auto
// focus mode renders primary sections prominently and fades secondary sections;
// the user can pin a specific view via the side controls.
const PHASE_FOCUS: Record<Phase, { primary: FocusMode[]; secondary: FocusMode[] }> = {
  setup:      { primary: ['board', 'decks'],           secondary: ['spirits', 'stats', 'retro'] },
  growth:     { primary: ['spirits'],                  secondary: ['board', 'stats', 'decks', 'retro'] },
  // Fast & Slow phases — spirits play cards that target LAND; both views
  // are load-bearing. Decks / stats / retro step aside.
  fast:       { primary: ['spirits', 'board'],         secondary: ['decks', 'stats', 'retro'] },
  event:      { primary: ['decks'],                    secondary: ['board', 'stats', 'spirits', 'retro'] },
  fear:       { primary: ['decks'],                    secondary: ['board', 'stats', 'spirits', 'retro'] },
  invader:    { primary: ['decks', 'board'],           secondary: ['spirits', 'stats', 'retro'] },
  slow:       { primary: ['spirits', 'board'],         secondary: ['decks', 'stats', 'retro'] },
  timepasses: { primary: ['retro', 'stats'],           secondary: ['spirits', 'board', 'decks'] },
  end:        { primary: ['retro', 'stats'],           secondary: ['spirits', 'board', 'decks'] },
}

function focusClass(section: FocusMode): string {
  if (focusOverride.value !== 'auto') {
    return focusOverride.value === section || focusOverride.value === 'all' ? 'focus-primary' : 'focus-faded'
  }
  const phase = state.value?.phase ?? 'setup'
  const meta = PHASE_FOCUS[phase]
  if (meta.primary.includes(section)) return 'focus-primary'
  if (meta.secondary.includes(section)) return 'focus-secondary'
  return ''
}

/** Whether a section should be collapsed to a strip right now.
 *  Collapsed = not primary for the active phase, unless the user has pinned a
 *  different focus via the Focus tabs or explicitly wants everything ('all'). */
function isSectionCollapsed(section: FocusMode): boolean {
  if (focusOverride.value === 'all') return false
  if (focusOverride.value !== 'auto') return focusOverride.value !== section
  const phase = state.value?.phase ?? 'setup'
  const meta = PHASE_FOCUS[phase]
  return !meta.primary.includes(section)
}

/** Short metric string shown on each section's collapsed strip. */
function sectionMetric(section: FocusMode): string {
  if (!state.value) return ''
  const s = state.value
  switch (section) {
    case 'board': {
      const boards = s.board_state ?? {}
      let invaders = 0, blight = 0
      for (const b of Object.values(boards)) {
        for (const land of Object.values(b.lands ?? {})) {
          invaders += (land.explorers ?? 0) + (land.towns ?? 0) + (land.cities ?? 0)
          blight += land.blight ?? 0
        }
      }
      return `${Object.keys(boards).length} board(s) · ${invaders} invaders · ${blight} blight`
    }
    case 'decks': {
      const inv = s.invader_deck
      const fd = s.fear_deck
      const ev = s.event_deck
      const invExp = inv?.explore?.terrain ?? '—'
      const fearLeft = fd ? Math.max(0, fd.deck_size - (fd.resolved.length + fd.earned.length)) : 0
      const evPrev = ev?.previewed.length ? '1 previewed' : 'no preview'
      return `Explore: ${invExp} · Fear ${fearLeft} left · ${evPrev}`
    }
    case 'spirits': {
      const count = Object.keys(s.spirits ?? {}).length
      return `${count} spirit${count === 1 ? '' : 's'}`
    }
    case 'stats':
      return `Round ${s.round}`
    case 'retro': {
      const log = (s.log ?? []) as Array<{ event: string }>
      const snaps = log.filter(e => e.event === 'phase_snapshot').length
      return `${snaps} snapshot(s)`
    }
    default:
      return ''
  }
}

const focusTabs: { key: FocusMode; label: string }[] = [
  { key: 'auto', label: 'Auto (phase-driven)' },
  { key: 'board', label: 'Board' },
  { key: 'decks', label: 'Decks' },
  { key: 'spirits', label: 'Spirits' },
  { key: 'stats', label: 'Stats' },
  { key: 'retro', label: 'Retrospective' },
  { key: 'all', label: 'Show all' },
]

// Quick-nav items — each anchor matches an id= on a section below. Board and
// spirit entries are generated dynamically from the game state.
const navItems = computed(() => {
  const items: Array<{ id: string; label: string; icon: string }> = [
    { id: 'sec-turn',     label: 'Turn Log',      icon: '⟳' },
    { id: 'sec-pools',    label: 'Pools · Blight', icon: '◯' },
    { id: 'sec-events',   label: 'Event Deck',    icon: '📜' },
    { id: 'sec-fear',     label: 'Fear Deck',     icon: '😱' },
    { id: 'sec-invader',  label: 'Invader Deck',  icon: '⚔' },
    { id: 'sec-terrain',  label: 'Terrain Timeline', icon: '📊' },
    { id: 'sec-stats',    label: 'Stats',         icon: '%' },
    { id: 'sec-retro',    label: 'Retrospective', icon: '🔍' },
  ]
  if (state.value) {
    for (const bid of Object.keys(state.value.board_state)) {
      items.push({ id: `sec-board-${bid}`, label: `Board ${bid}`, icon: '▦' })
    }
    for (const slug of Object.keys(state.value.spirits)) {
      items.push({ id: `sec-spirit-${slug}`, label: humanSlug(slug).split(' ').slice(0, 2).join(' '), icon: '✦' })
    }
  }
  return items
})
</script>

<template>
  <div v-if="error" class="banner error">{{ error }}</div>
  <div v-else-if="!state" class="banner">Loading…</div>
  <main v-else>
    <header class="app-header">
      <div class="brand">
        <h1>si-live</h1>
        <span v-if="matchupTag" class="matchup-tag">{{ matchupTag }}</span>
      </div>

      <div class="app-meta">
        <button
          class="ghost"
          @click="archiveCurrentGame"
          :disabled="archiving"
          title="Save a snapshot of this game to the archive (data/games/) — for later review or statistical aggregation"
        >{{ archiving ? 'Archiving…' : '💾 Archive' }}</button>
        <button class="ghost" @click="exportCurrentGame" title="Download this game's full state as a JSON file">⬇ Export JSON</button>
        <button class="ghost" @click="showSavedGames = true" title="Browse archived games">Saved Games</button>
        <button class="primary new-game-btn" @click="showWizard = true">New Game</button>
        <span v-if="saving" class="saving" aria-live="polite">saving…</span>
      </div>
    </header>

    <SetupWizard :show="showWizard" @close="showWizard = false" @game-started="onGameStarted" />
    <SavedGames :show="showSavedGames" @close="showSavedGames = false" @game-loaded="onGameStarted" />

    <PhaseStepper v-model="state.phase" :round="state.round" />

    <StickyStatus :state="state" />

    <!-- Game-end banner — WIN (fear deck resolved + T3), LOST (blight cap flipped),
         or IMMINENT WIN (deck drawn, awaiting Terror 3 or Fear phase resolution). -->
    <div v-if="endBanner === 'won'" class="end-banner won">
      <div class="end-main">
        <span class="end-icon">🏆</span>
        <div>
          <div class="end-title">Victory — Fear deck exhausted</div>
          <div class="end-sub">All {{ state.fear_deck?.deck_size ?? 9 }} fear cards resolved at Terror {{ state.pools.terror_level }} · Round {{ state.round }}</div>
        </div>
      </div>
      <button class="ghost" @click="gameOverBannerDismissed = true">Dismiss</button>
    </div>
    <div v-else-if="endBanner === 'lost'" class="end-banner lost">
      <div class="end-main">
        <span class="end-icon">💀</span>
        <div>
          <div class="end-title">Defeat — Blighted Island capped</div>
          <div class="end-sub">Blight {{ state.pools.blight_current }} / {{ state.pools.blight_cap }} after flip · Round {{ state.round }}</div>
        </div>
      </div>
      <button class="ghost" @click="gameOverBannerDismissed = true">Dismiss</button>
    </div>
    <div v-else-if="endBanner === 'imminent'" class="end-banner imminent">
      <div class="end-main">
        <span class="end-icon">🎯</span>
        <div>
          <div class="end-title">Imminent Victory — Fear deck drawn</div>
          <div class="end-sub">Finish the current Invader phase without a total loss. Once Terror {{ state.pools.terror_level === 3 ? '3 holds through the phase' : '3 is reached' }}, the win is locked in.</div>
        </div>
      </div>
      <button class="ghost" @click="gameOverBannerDismissed = true">Dismiss</button>
    </div>

    <!-- Side-tabs to pin focus (overrides phase-driven auto focus) -->
    <div class="focus-tabs-row">
      <div class="focus-tabs">
        <button
          v-for="t in focusTabs"
          :key="t.key"
          type="button"
          class="focus-tab"
          :class="{ active: focusOverride === t.key }"
          @click="focusOverride = t.key"
        >{{ t.label }}</button>
      </div>
      <div class="density-toggle">
        <span class="density-label">Density:</span>
        <button
          type="button"
          class="density-btn"
          :class="{ active: density === 'compact' }"
          @click="density = 'compact'"
        >Compact</button>
        <button
          type="button"
          class="density-btn"
          :class="{ active: density === 'comfortable' }"
          @click="density = 'comfortable'"
        >Comfortable</button>
      </div>
    </div>

    <section id="sec-turn" class="section always-visible">
      <TurnController v-model="state" />
    </section>

    <section id="sec-pools" class="section pools-section always-visible">
      <Pools
        v-model="state.pools"
        :player-count="Object.keys(state.spirits ?? {}).length"
      />
    </section>

    <section id="sec-events" class="section" :class="focusClass('decks')">
      <SectionStrip
        title="Event Deck"
        icon="📜"
        :collapsed="isSectionCollapsed('decks')"
        :metric="sectionMetric('decks')"
      >
        <div class="section-hdr"><h2>Event Deck</h2></div>
        <EventDeck
          v-model="state.event_deck"
          :round="state.round"
        />
      </SectionStrip>
    </section>

    <section id="sec-fear" class="section" :class="focusClass('decks')">
      <SectionStrip
        title="Fear Deck"
        icon="😱"
        :collapsed="isSectionCollapsed('decks')"
        :metric="`${state.fear_deck?.resolved.length ?? 0}r + ${state.fear_deck?.earned.length ?? 0}e / ${state.fear_deck?.deck_size ?? 9} · T${state.pools.terror_level}`"
      >
        <div class="section-hdr"><h2>Fear Deck</h2></div>
        <FearDeck
          v-model="state.fear_deck"
          :round="state.round"
          :terror-level="state.pools.terror_level"
          :fear-threshold="state.pools.fear_threshold"
          @log-event="(event, details) => appendLog(event, details)"
          @reset-fear-pool="resetFearPool"
        />
      </SectionStrip>
    </section>

    <section id="sec-invader" class="section" :class="focusClass('decks')">
      <SectionStrip
        title="Invader Deck"
        icon="⚔"
        :collapsed="isSectionCollapsed('decks')"
        :metric="`R: ${state.invader_deck?.ravage?.terrain ?? '—'} · B: ${state.invader_deck?.build?.terrain ?? '—'} · E: ${state.invader_deck?.explore?.terrain ?? '—'}`"
      >
        <div class="section-hdr"><h2>Invader Deck</h2></div>
        <InvaderDeck
          v-model="state.invader_deck"
          :adversary="state.setup.adversary"
          :level="state.setup.level"
        />
      </SectionStrip>
    </section>

    <section id="sec-terrain" class="section" :class="focusClass('decks')">
      <SectionStrip
        title="Terrain Timeline"
        icon="📊"
        :collapsed="isSectionCollapsed('decks')"
      >
        <div class="section-hdr"><h2>Terrain Exposure</h2></div>
        <TerrainTimeline :state="state" />
      </SectionStrip>
    </section>

    <section id="sec-stats" class="section" :class="focusClass('stats')">
      <SectionStrip
        title="Stats"
        icon="%"
        :collapsed="isSectionCollapsed('stats')"
        :metric="sectionMetric('stats')"
      >
        <div class="section-hdr"><h2>Stats</h2></div>
        <StatsPanel :state="state" />
      </SectionStrip>
    </section>

    <section id="sec-retro" class="section" :class="focusClass('retro')">
      <SectionStrip
        title="Retrospective"
        icon="🔍"
        :collapsed="isSectionCollapsed('retro')"
        :metric="sectionMetric('retro')"
      >
        <div class="section-hdr"><h2>Retrospective</h2></div>
        <Retrospective :state="state" />
      </SectionStrip>
    </section>

    <section
      v-for="bid in Object.keys(state.board_state)"
      :id="`sec-board-${bid}`"
      :key="bid"
      class="section"
      :class="focusClass('board')"
    >
      <SectionStrip
        :title="`Board ${bid}`"
        icon="▦"
        :collapsed="isSectionCollapsed('board')"
        :metric="sectionMetric('board')"
      >
        <div class="section-hdr"><h2>Board {{ bid }}</h2></div>
        <Board
          v-model="state.board_state[bid]"
          :board-id="bid"
          :spirits="state.spirits"
          @bump-pool="(pool, delta) => bumpPool(pool, delta)"
        />
      </SectionStrip>
    </section>

    <section
      v-for="slug in Object.keys(state.spirits)"
      :id="`sec-spirit-${slug}`"
      :key="slug"
      class="section"
      :class="focusClass('spirits')"
    >
      <SectionStrip
        :title="humanSlug(slug)"
        icon="✦"
        :collapsed="isSectionCollapsed('spirits')"
        :metric="`${state.spirits[slug]?.energy ?? 0}E · ${state.spirits[slug]?.card_plays ?? 0}CP · ${state.spirits[slug]?.hand?.length ?? 0} hand`"
      >
        <div class="section-hdr">
          <h2>{{ humanSlug(slug) }}</h2>
          <span class="subtle mono">{{ slug }}</span>
        </div>
        <SpiritPanel
          v-model="state.spirits[slug]"
          :slug="slug"
          :round="state.round"
          @log-event="(event, details) => appendLog(event, details)"
        />
      </SectionStrip>
    </section>

    <SectionNav :items="navItems" />
  </main>
</template>

<style scoped>
main {
  max-width: 1180px;
  margin: 0 auto;
  padding: var(--sp-5) var(--sp-4);
  display: flex;
  flex-direction: column;
  gap: var(--sp-5);
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

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--sp-3);
  padding-bottom: var(--sp-3);
  border-bottom: 1px solid var(--border-subtle);
}

.brand {
  display: flex;
  align-items: baseline;
  gap: var(--sp-3);
}

h1 {
  font-family: var(--font-mono);
  font-size: var(--fs-xl);
  font-weight: var(--fw-semibold);
  color: var(--accent);
  letter-spacing: -0.01em;
}

.matchup-tag {
  font-size: var(--fs-sm);
  color: var(--text-secondary);
  text-transform: capitalize;
  font-weight: var(--fw-regular);
  padding: 2px var(--sp-2);
  background: var(--bg-muted);
  border-radius: var(--r-full);
}

.app-meta {
  display: inline-flex;
  align-items: center;
  gap: var(--sp-3);
  flex-wrap: wrap;
}

.saving {
  font-size: var(--fs-xs);
  color: var(--status-success);
  font-style: italic;
}

.end-banner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-3);
  padding: var(--sp-3) var(--sp-4);
  border-radius: var(--r-lg);
  border: 2px solid;
  animation: pulse 2s ease-in-out infinite;
}
.end-banner.won {
  background: rgba(82, 183, 136, 0.15);
  border-color: var(--status-success);
}
.end-banner.lost {
  background: rgba(184, 113, 106, 0.15);
  border-color: var(--status-danger);
}
.end-banner.imminent {
  background: rgba(82, 183, 136, 0.08);
  border-color: rgba(82, 183, 136, 0.6);
  border-style: dashed;
  animation: none;  /* imminent is quieter than full win */
}
.end-main { display: inline-flex; align-items: center; gap: var(--sp-3); }
.end-icon { font-size: 2rem; }
.end-title {
  font-size: 1.1rem;
  font-weight: var(--fw-bold);
  color: var(--text-white);
}
.end-sub {
  font-size: var(--fs-sm);
  color: var(--text-secondary);
  margin-top: 2px;
}
@keyframes pulse {
  0%, 100% { box-shadow: 0 0 0 0 rgba(82, 183, 136, 0.4); }
  50% { box-shadow: 0 0 0 6px rgba(82, 183, 136, 0); }
}
.end-banner.lost { animation: pulse-red 2s ease-in-out infinite; }
@keyframes pulse-red {
  0%, 100% { box-shadow: 0 0 0 0 rgba(184, 113, 106, 0.4); }
  50% { box-shadow: 0 0 0 6px rgba(184, 113, 106, 0); }
}

.focus-tabs-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--sp-3);
  flex-wrap: wrap;
}
.focus-tabs {
  display: inline-flex; flex-wrap: wrap; gap: 4px;
  padding: var(--sp-1) var(--sp-2);
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
}
.density-toggle {
  display: inline-flex; align-items: center; gap: var(--sp-2);
  padding: var(--sp-1) var(--sp-2);
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
}
.density-label {
  font-size: var(--fs-xs);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
}
.density-btn {
  padding: 3px var(--sp-2);
  font-size: var(--fs-xs);
  border: 1px solid transparent;
  background: transparent;
  color: var(--text-secondary);
  border-radius: var(--r-sm);
  cursor: pointer;
}
.density-btn.active {
  background: var(--bg-muted);
  color: var(--text-white);
  border-color: var(--accent-blue);
}
.focus-tab {
  padding: 4px var(--sp-2);
  font-size: var(--fs-xs);
  border: 1px solid transparent;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: var(--r-sm);
  transition: all var(--motion-fast);
}
.focus-tab:hover { background: var(--bg-muted); color: var(--text-primary); }
.focus-tab.active {
  background: var(--bg-muted);
  border-color: var(--accent-blue);
  color: var(--text-white);
}

.section {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
  transition: opacity var(--motion-fast), transform var(--motion-fast);
}

/* Focus states — primary is emphasized, secondary is readable but de-emphasized,
 * unmatched is collapsed. Always-visible sections ignore focus classes. */
.section.focus-primary {
  /* no-op — default appearance IS the primary state */
}
.section.focus-secondary {
  opacity: 0.72;
}
.section.focus-secondary:hover {
  opacity: 1;
}
.section:not(.focus-primary):not(.focus-secondary):not(.always-visible) {
  /* Fully faded — still navigable via focus tabs but de-emphasized */
  opacity: 0.42;
}
.section:not(.focus-primary):not(.focus-secondary):not(.always-visible):hover {
  opacity: 0.9;
}
.section.focus-faded {
  opacity: 0.42;
}
.section.focus-faded:hover { opacity: 0.9; }

.section-hdr {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--sp-2);
}

h2 {
  font-size: 0.82rem;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-secondary);
  font-weight: var(--fw-semibold);
  margin: 0;
}

.pools-section .section-hdr { display: none; }
</style>
