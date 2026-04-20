<script setup lang="ts">
import { onMounted, ref } from 'vue'
import type { GameState } from '../types'

interface SavedGame {
  id: string
  filename: string
  created_at: number
  round: number
  phase: string
  adversary: string | null
  level: number | null
  scenario: string | null
  spirits: string[]
  boards: string[]
}

const emit = defineEmits<{ 'game-loaded': [state: GameState]; close: [] }>()
defineProps<{ show: boolean }>()

const games = ref<SavedGame[]>([])
const error = ref<string | null>(null)
const busy = ref<string | null>(null)

async function refresh() {
  error.value = null
  try {
    const res = await fetch('/api/saved-games')
    if (!res.ok) throw new Error(`${res.status}`)
    const data = await res.json()
    games.value = data.games || []
  } catch (e) {
    error.value = (e as Error).message
  }
}

async function loadGame(id: string) {
  busy.value = id
  try {
    const res = await fetch(`/api/saved-games/${encodeURIComponent(id)}/load`, { method: 'POST' })
    if (!res.ok) throw new Error(`${res.status}`)
    const state = await res.json()
    emit('game-loaded', state)
    emit('close')
  } catch (e) {
    error.value = `load failed: ${(e as Error).message}`
  } finally {
    busy.value = null
  }
}

async function archiveCurrent() {
  busy.value = '__archive'
  try {
    const res = await fetch('/api/saved-games/archive', { method: 'POST' })
    if (!res.ok) throw new Error(`${res.status}`)
    await refresh()
  } catch (e) {
    error.value = `archive failed: ${(e as Error).message}`
  } finally {
    busy.value = null
  }
}

async function deleteGame(id: string) {
  if (!confirm(`Delete saved game ${id}?`)) return
  busy.value = id
  try {
    const res = await fetch(`/api/saved-games/${encodeURIComponent(id)}`, { method: 'DELETE' })
    if (!res.ok) throw new Error(`${res.status}`)
    await refresh()
  } catch (e) {
    error.value = `delete failed: ${(e as Error).message}`
  } finally {
    busy.value = null
  }
}

onMounted(refresh)

function formatDate(epoch: number): string {
  try {
    return new Date(epoch * 1000).toLocaleString()
  } catch {
    return '—'
  }
}

function humanSlug(s: string | null): string {
  if (!s) return ''
  return s.replace(/-/g, ' ')
}
</script>

<template>
  <div v-if="show" class="modal-bg" @click.self="emit('close')">
    <div class="modal">
      <header>
        <h2>Saved Games</h2>
        <div class="hdr-actions">
          <button @click="archiveCurrent" :disabled="busy === '__archive'" title="Archive the currently-active game to data/games/">
            {{ busy === '__archive' ? 'archiving…' : 'Archive current' }}
          </button>
          <button @click="refresh">Refresh</button>
          <button class="close" @click="emit('close')" aria-label="close">×</button>
        </div>
      </header>

      <div v-if="error" class="error">{{ error }}</div>

      <div v-if="!games.length" class="empty">
        No saved games yet. Click "Archive current" to save the running game, or start a New Game with archival enabled.
      </div>

      <ul v-else class="games">
        <li v-for="g in games" :key="g.id" class="game">
          <div class="row-hdr">
            <div class="match">
              <span class="adv">{{ humanSlug(g.adversary) || 'no adversary' }}</span>
              <span v-if="g.level != null" class="level">L{{ g.level }}</span>
              <span v-if="g.scenario" class="scenario">· {{ humanSlug(g.scenario) }}</span>
            </div>
            <span class="round-tag">Round {{ g.round }} <span class="phase">· {{ g.phase }}</span></span>
          </div>
          <div class="row-sub">
            <span class="spirits"><strong>Spirits:</strong> {{ g.spirits.map(humanSlug).join(', ') || '—' }}</span>
            <span class="boards"><strong>Boards:</strong> {{ g.boards.join(', ') || '—' }}</span>
          </div>
          <div class="row-foot">
            <span class="date">{{ formatDate(g.created_at) }}</span>
            <span class="id mono">{{ g.id }}</span>
            <div class="actions">
              <button class="primary" @click="loadGame(g.id)" :disabled="busy === g.id">
                {{ busy === g.id ? 'loading…' : 'Load' }}
              </button>
              <button class="ghost" @click="deleteGame(g.id)" :disabled="busy === g.id">Delete</button>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.modal-bg {
  position: fixed; inset: 0;
  background: rgba(0, 0, 0, 0.65);
  display: flex; align-items: center; justify-content: center;
  z-index: 100;
}
.modal {
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--r-lg);
  padding: var(--sp-4);
  width: min(92vw, 760px);
  max-height: 85vh;
  overflow-y: auto;
  box-shadow: var(--shadow-lg);
}
header {
  display: flex; justify-content: space-between; align-items: center;
  margin-bottom: var(--sp-3); padding-bottom: var(--sp-2);
  border-bottom: 1px solid var(--border-subtle);
}
h2 { margin: 0; font-size: var(--fs-md); }
.hdr-actions { display: inline-flex; gap: var(--sp-2); align-items: center; }
.close { background: transparent; border: none; color: var(--text-secondary); font-size: 1.5rem; cursor: pointer; padding: 0 var(--sp-2); }
.close:hover { color: var(--text-primary); background: transparent; }
.error {
  background: rgba(184, 113, 106, 0.15);
  color: var(--status-danger);
  padding: var(--sp-2);
  border-radius: var(--r-sm);
  font-size: var(--fs-sm);
  margin-bottom: var(--sp-3);
}
.empty {
  padding: var(--sp-5);
  text-align: center;
  color: var(--text-muted);
  font-style: italic;
  font-size: var(--fs-sm);
}
.games {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex; flex-direction: column;
  gap: var(--sp-2);
}
.game {
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-3);
  display: flex; flex-direction: column;
  gap: var(--sp-1);
}
.row-hdr { display: flex; justify-content: space-between; align-items: baseline; gap: var(--sp-2); }
.match { display: inline-flex; gap: var(--sp-2); align-items: baseline; flex-wrap: wrap; }
.adv { text-transform: capitalize; font-weight: var(--fw-semibold); color: var(--text-primary); font-size: var(--fs-sm); }
.level {
  font-family: var(--font-mono);
  font-size: var(--fs-xs);
  padding: 1px var(--sp-2);
  background: var(--accent-soft);
  color: var(--pool-fear);
  border-radius: var(--r-full);
}
.scenario { text-transform: capitalize; color: var(--text-secondary); font-size: var(--fs-xs); }
.round-tag {
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  font-family: var(--font-mono);
}
.phase { text-transform: capitalize; }
.row-sub { display: flex; gap: var(--sp-3); font-size: var(--fs-xs); color: var(--text-secondary); flex-wrap: wrap; text-transform: capitalize; }
.row-sub strong { color: var(--text-primary); font-weight: var(--fw-medium); }
.row-foot {
  display: flex; align-items: center; gap: var(--sp-3);
  font-size: 0.7rem; color: var(--text-muted);
  margin-top: var(--sp-1);
}
.id { opacity: 0.5; }
.actions { margin-left: auto; display: inline-flex; gap: var(--sp-1); }
</style>
