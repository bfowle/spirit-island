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

const emit = defineEmits<{ load: [state: GameState]; close: [] }>()

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
    emit('load', state)
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
    return '-'
  }
}

function humanSlug(s: string | null): string {
  if (!s) return ''
  return s.replace(/-/g, ' ')
}
</script>

<template>
  <div class="modal-overlay" @click.self="emit('close')">
    <div class="modal">
      <header class="modal-header">
        <h2>Saved Games</h2>
        <div class="header-actions">
          <button @click="archiveCurrent" :disabled="busy === '__archive'">
            {{ busy === '__archive' ? 'Archiving...' : 'Archive Current' }}
          </button>
          <button @click="refresh">Refresh</button>
          <button class="close-btn" @click="emit('close')" aria-label="close">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        </div>
      </header>

      <div v-if="error" class="error-banner">{{ error }}</div>

      <div class="modal-body">
        <div v-if="!games.length" class="empty-state">
          No saved games yet. Click &quot;Archive Current&quot; to save the running game, or start a New Game with archival enabled.
        </div>

        <ul v-else class="games-list">
          <li v-for="g in games" :key="g.id" class="game-card">
            <div class="game-header">
              <div class="game-matchup">
                <span class="adversary">{{ humanSlug(g.adversary) || 'No adversary' }}</span>
                <span v-if="g.level != null" class="level-badge">L{{ g.level }}</span>
                <span v-if="g.scenario" class="scenario">{{ humanSlug(g.scenario) }}</span>
              </div>
              <span class="round-info">
                Round {{ g.round }}
                <span class="phase-tag">{{ g.phase }}</span>
              </span>
            </div>
            <div class="game-details">
              <span class="detail"><strong>Spirits:</strong> {{ g.spirits.map(humanSlug).join(', ') || '-' }}</span>
              <span class="detail"><strong>Boards:</strong> {{ g.boards.join(', ') || '-' }}</span>
            </div>
            <div class="game-footer">
              <span class="date">{{ formatDate(g.created_at) }}</span>
              <span class="game-id">{{ g.id }}</span>
              <div class="game-actions">
                <button class="primary" @click="loadGame(g.id)" :disabled="busy === g.id">
                  {{ busy === g.id ? 'Loading...' : 'Load' }}
                </button>
                <button class="ghost" @click="deleteGame(g.id)" :disabled="busy === g.id">Delete</button>
              </div>
            </div>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  backdrop-filter: blur(4px);
}

.modal {
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-xl);
  width: min(92vw, 760px);
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  box-shadow: var(--shadow-lg);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--sp-5);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.modal-header h2 {
  margin: 0;
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
}

.header-actions {
  display: flex;
  gap: var(--sp-2);
  align-items: center;
}

.close-btn {
  width: 32px;
  height: 32px;
  padding: 0;
  background: transparent;
  border: none;
  color: var(--text-muted);
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.error-banner {
  margin: var(--sp-4);
  padding: var(--sp-3);
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.2);
  border-radius: var(--radius-md);
  color: var(--color-danger);
  font-size: var(--text-sm);
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: var(--sp-4);
}

.empty-state {
  padding: var(--sp-8);
  text-align: center;
  color: var(--text-muted);
  font-style: italic;
  font-size: var(--text-sm);
}

.games-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
}

.game-card {
  background: var(--bg-muted);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  padding: var(--sp-4);
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.game-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  gap: var(--sp-3);
}

.game-matchup {
  display: flex;
  gap: var(--sp-2);
  align-items: baseline;
  flex-wrap: wrap;
}

.adversary {
  text-transform: capitalize;
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  font-size: var(--text-sm);
}

.level-badge {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  padding: 2px var(--sp-2);
  background: rgba(245, 158, 11, 0.15);
  color: var(--color-fear);
  border-radius: 10px;
}

.scenario {
  text-transform: capitalize;
  color: var(--text-secondary);
  font-size: var(--text-xs);
}

.scenario::before {
  content: '/ ';
  color: var(--text-faint);
}

.round-info {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  font-family: var(--font-mono);
}

.phase-tag {
  text-transform: capitalize;
  margin-left: var(--sp-1);
}

.phase-tag::before {
  content: '- ';
}

.game-details {
  display: flex;
  gap: var(--sp-4);
  font-size: var(--text-xs);
  color: var(--text-secondary);
  flex-wrap: wrap;
  text-transform: capitalize;
}

.detail strong {
  color: var(--text-primary);
  font-weight: var(--weight-medium);
}

.game-footer {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  font-size: var(--text-xs);
  color: var(--text-muted);
  margin-top: var(--sp-2);
  padding-top: var(--sp-2);
  border-top: 1px solid var(--border-subtle);
}

.game-id {
  font-family: var(--font-mono);
  opacity: 0.5;
}

.game-actions {
  margin-left: auto;
  display: flex;
  gap: var(--sp-2);
}

.game-actions button {
  height: 30px;
  padding: 0 var(--sp-3);
  font-size: var(--text-xs);
}

.game-actions button.ghost {
  background: transparent;
  border-color: transparent;
  color: var(--text-muted);
}

.game-actions button.ghost:hover:not(:disabled) {
  background: var(--bg-hover);
  color: var(--text-primary);
}
</style>
