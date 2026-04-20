<script setup lang="ts">
import { onMounted, ref, watch, computed } from 'vue'
import type { GameState } from './types'
import { fetchState, saveState } from './api'
import Board from './components/Board.vue'
import SpiritPanel from './components/SpiritPanel.vue'
import Pools from './components/Pools.vue'
import StatsPanel from './components/StatsPanel.vue'
import SetupWizard from './components/SetupWizard.vue'
import SavedGames from './components/SavedGames.vue'
import TurnController from './components/TurnController.vue'

const state = ref<GameState | null>(null)
const error = ref<string | null>(null)
const saving = ref(false)
const showWizard = ref(false)
const showSavedGames = ref(false)
let saveTimer: number | null = null

function onGameStarted(newState: GameState) {
  state.value = newState
}

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
        <button class="ghost" @click="showSavedGames = true" title="Browse archived games">Saved Games</button>
        <button class="primary new-game-btn" @click="showWizard = true">New Game</button>
        <span v-if="saving" class="saving" aria-live="polite">saving…</span>
      </div>
    </header>

    <SetupWizard :show="showWizard" @close="showWizard = false" @game-started="onGameStarted" />
    <SavedGames :show="showSavedGames" @close="showSavedGames = false" @game-loaded="onGameStarted" />

    <section class="section">
      <TurnController v-model="state" />
    </section>

    <section class="section pools-section">
      <Pools v-model="state.pools" />
    </section>

    <section class="section">
      <div class="section-hdr">
        <h2>Stats</h2>
      </div>
      <StatsPanel :state="state" />
    </section>

    <section v-for="bid in Object.keys(state.board_state)" :key="bid" class="section">
      <div class="section-hdr">
        <h2>Board {{ bid }}</h2>
      </div>
      <Board v-model="state.board_state[bid]" :board-id="bid" />
    </section>

    <section v-for="slug in Object.keys(state.spirits)" :key="slug" class="section">
      <div class="section-hdr">
        <h2>{{ humanSlug(slug) }}</h2>
        <span class="subtle mono">{{ slug }}</span>
      </div>
      <SpiritPanel v-model="state.spirits[slug]" :slug="slug" />
    </section>
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

.turn-indicator {
  display: inline-flex;
  align-items: baseline;
  gap: var(--sp-2);
  padding: var(--sp-1) var(--sp-3);
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
}

.turn-label {
  font-size: var(--fs-xs);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
}

.turn-value {
  font-family: var(--font-mono);
  font-size: var(--fs-lg);
  font-weight: var(--fw-bold);
  color: var(--text-primary);
}

.phase-select {
  display: inline-flex;
  align-items: center;
  gap: var(--sp-2);
}

.field-label {
  font-size: var(--fs-xs);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
}

.phase-select select {
  text-transform: capitalize;
}

.saving {
  font-size: var(--fs-xs);
  color: var(--status-success);
  font-style: italic;
}

.section {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
}

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
