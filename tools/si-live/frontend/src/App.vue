<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import type { GameState } from './types'
import { fetchState, saveState } from './api'
import Board from './components/Board.vue'
import SpiritPanel from './components/SpiritPanel.vue'
import Pools from './components/Pools.vue'

const state = ref<GameState | null>(null)
const error = ref<string | null>(null)
const saving = ref(false)
let saveTimer: number | null = null

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
</script>

<template>
  <div v-if="error" class="banner error">Error: {{ error }}</div>
  <div v-else-if="!state" class="banner">Loading…</div>
  <main v-else>
    <header>
      <h1>si-live</h1>
      <div class="meta">
        Round {{ state.round }} — Phase
        <select v-model="state.phase">
          <option value="setup">Setup</option>
          <option value="growth">Growth</option>
          <option value="fast">Fast</option>
          <option value="invader">Invader</option>
          <option value="slow">Slow</option>
          <option value="timepasses">Time Passes</option>
          <option value="end">End</option>
        </select>
        <span v-if="saving" class="saving">saving…</span>
      </div>
    </header>

    <Pools v-model="state.pools" />

    <section v-for="bid in Object.keys(state.board_state)" :key="bid" class="board-section">
      <h2>Board {{ bid }}</h2>
      <Board v-model="state.board_state[bid]" />
    </section>

    <section v-for="slug in Object.keys(state.spirits)" :key="slug" class="spirit-section">
      <h2>{{ slug }}</h2>
      <SpiritPanel v-model="state.spirits[slug]" />
    </section>
  </main>
</template>

<style scoped>
main { max-width: 1100px; margin: 0 auto; padding: 1rem; }
header { display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #444; padding-bottom: .5rem; margin-bottom: 1rem; }
h1 { margin: 0; font-size: 1.4rem; }
.meta { display: flex; gap: .75rem; align-items: center; font-size: .9rem; }
.banner { padding: 1rem; text-align: center; }
.banner.error { background: #522; color: #fcc; }
.saving { color: #8a8; font-style: italic; }
.board-section, .spirit-section { margin-top: 1.5rem; }
h2 { font-size: 1.1rem; margin: 0 0 .5rem; }
</style>
