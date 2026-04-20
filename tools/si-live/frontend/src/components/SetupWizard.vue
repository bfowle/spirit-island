<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import type { GameState } from '../types'

interface BoardSummary {
  file: string
  board_id: string
  expansion: string
  variants: { key: string; name: string }[]
}

interface RegistryResponse {
  spirits: { spirits: { slug: string; name: string; expansion: string; complexity?: string }[] }
  adversaries: { adversaries: { slug: string; name: string; expansion: string; difficulty_levels: number[] }[] }
  scenarios: { scenarios: { slug: string; name: string; expansion: string }[] }
  boards: BoardSummary[]
}

const emit = defineEmits<{ started: [state: GameState]; close: [] }>()

const registry = ref<RegistryResponse | null>(null)
const error = ref<string | null>(null)

const adversary = ref<string>('england')
const level = ref<number>(3)
const scenario = ref<string>('')
const selectedSpirits = ref<string[]>(['shadows-flicker-like-flame'])
const selectedBoards = ref<string[]>(['A'])
const expansions = ref<string[]>(['base'])
const archiveCurrent = ref<boolean>(true)
const boardVariant = ref<'balanced' | 'thematic'>('balanced')

onMounted(async () => {
  try {
    const res = await fetch('/api/registry')
    if (!res.ok) throw new Error(`GET /api/registry → ${res.status}`)
    registry.value = await res.json()
  } catch (e) {
    error.value = (e as Error).message
  }
})

interface LevelOption {
  level: number
  rating: number
  label: string
}

const availableLevels = computed((): LevelOption[] => {
  const adv = registry.value?.adversaries.adversaries.find(a => a.slug === adversary.value)
  const ratings = adv?.difficulty_levels ?? []
  const out: LevelOption[] = [{ level: 0, rating: 0, label: 'L0 (base, difficulty 0)' }]
  for (let i = 0; i < Math.min(ratings.length, 6); i++) {
    out.push({
      level: i + 1,
      rating: ratings[i],
      label: `L${i + 1} (difficulty ${ratings[i]})`,
    })
  }
  return out
})

const availableBoards = computed((): BoardSummary[] => {
  if (!registry.value) return []
  return [...registry.value.boards].sort((a, b) => a.board_id.localeCompare(b.board_id))
})

const variantAvailableForAllSelected = computed(() => {
  if (!registry.value) return { balanced: true, thematic: true }
  const hasVariant = (key: string) =>
    selectedBoards.value.every(bid => {
      const b = registry.value!.boards.find(x => x.board_id === bid)
      return !b || b.variants.some(v => v.key === key)
    })
  return { balanced: hasVariant('balanced'), thematic: hasVariant('thematic') }
})

function toggleSpirit(slug: string) {
  const i = selectedSpirits.value.indexOf(slug)
  if (i >= 0) selectedSpirits.value.splice(i, 1)
  else selectedSpirits.value.push(slug)
}

function toggleBoard(b: string) {
  const i = selectedBoards.value.indexOf(b)
  if (i >= 0) selectedBoards.value.splice(i, 1)
  else selectedBoards.value.push(b)
}

function toggleExpansion(e: string) {
  const i = expansions.value.indexOf(e)
  if (i >= 0) expansions.value.splice(i, 1)
  else expansions.value.push(e)
}

async function startGame() {
  error.value = null
  try {
    if (archiveCurrent.value) {
      try {
        await fetch('/api/saved-games/archive', { method: 'POST' })
      } catch (e) {
        console.warn('archive current game failed', e)
      }
    }
    const res = await fetch('/api/new-game', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        board_variant: boardVariant.value,
        adversary: adversary.value || null,
        level: level.value,
        scenario: scenario.value || null,
        spirits: selectedSpirits.value,
        boards: selectedBoards.value,
        expansions_active: expansions.value,
      }),
    })
    if (!res.ok) {
      const txt = await res.text()
      throw new Error(`POST /api/new-game → ${res.status}: ${txt}`)
    }
    const state = await res.json()
    emit('started', state)
    emit('close')
  } catch (e) {
    error.value = (e as Error).message
  }
}
</script>

<template>
  <div class="modal-overlay" @click.self="emit('close')">
    <div class="modal">
      <header class="modal-header">
        <h2>New Game Setup</h2>
        <button class="close-btn" @click="emit('close')" aria-label="close">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </header>

      <div v-if="error" class="error-banner">{{ error }}</div>
      <div v-else-if="!registry" class="loading-state">Loading...</div>
      
      <div v-else class="modal-body">
        <!-- Adversary & Level -->
        <section class="form-section">
          <div class="form-row">
            <label class="form-field">
              <span class="field-label">Adversary</span>
              <select v-model="adversary">
                <option value="">(none)</option>
                <option v-for="a in registry.adversaries.adversaries" :key="a.slug" :value="a.slug">
                  {{ a.name }}
                </option>
              </select>
            </label>
            <label class="form-field">
              <span class="field-label">Level</span>
              <select v-model.number="level">
                <option v-for="l in availableLevels" :key="l.level" :value="l.level">{{ l.label }}</option>
              </select>
            </label>
          </div>
        </section>

        <!-- Scenario -->
        <section class="form-section">
          <label class="form-field">
            <span class="field-label">Scenario</span>
            <select v-model="scenario">
              <option value="">(none)</option>
              <option v-for="s in registry.scenarios.scenarios" :key="s.slug" :value="s.slug">
                {{ s.name }}
              </option>
            </select>
          </label>
        </section>

        <!-- Spirits -->
        <section class="form-section">
          <div class="section-header">Spirits</div>
          <div class="chip-grid">
            <label v-for="s in registry.spirits.spirits" :key="s.slug" class="chip" :class="{ selected: selectedSpirits.includes(s.slug) }">
              <input type="checkbox" :checked="selectedSpirits.includes(s.slug)" @change="toggleSpirit(s.slug)" />
              <span class="chip-name">{{ s.name }}</span>
              <span class="chip-meta">{{ s.complexity || '?' }}</span>
            </label>
          </div>
        </section>

        <!-- Boards -->
        <section class="form-section">
          <div class="section-header">Boards</div>
          <div class="chip-grid">
            <label v-for="b in availableBoards" :key="b.board_id" class="chip" :class="{ selected: selectedBoards.includes(b.board_id) }">
              <input type="checkbox" :checked="selectedBoards.includes(b.board_id)" @change="toggleBoard(b.board_id)" />
              <span class="chip-name">{{ b.board_id }}</span>
              <span class="chip-meta">{{ b.expansion }}</span>
            </label>
          </div>
          <div class="variant-row">
            <span class="variant-label">Variant</span>
            <div class="variant-options">
              <label class="variant-option">
                <input type="radio" v-model="boardVariant" value="balanced" :disabled="!variantAvailableForAllSelected.balanced" />
                <span>Balanced</span>
              </label>
              <label class="variant-option">
                <input type="radio" v-model="boardVariant" value="thematic" :disabled="!variantAvailableForAllSelected.thematic" />
                <span>Thematic</span>
                <span v-if="!variantAvailableForAllSelected.thematic" class="variant-note">(unavailable)</span>
              </label>
            </div>
          </div>
        </section>

        <!-- Expansions -->
        <section class="form-section">
          <div class="section-header">Expansions Active</div>
          <div class="chip-grid">
            <label v-for="e in ['base', 'branch-and-claw', 'jagged-earth', 'nature-incarnate', 'promo-2']" :key="e" class="chip" :class="{ selected: expansions.includes(e) }">
              <input type="checkbox" :checked="expansions.includes(e)" @change="toggleExpansion(e)" />
              <span class="chip-name">{{ e }}</span>
            </label>
          </div>
        </section>
      </div>

      <footer class="modal-footer">
        <label class="archive-toggle">
          <input type="checkbox" v-model="archiveCurrent" />
          <span>Archive current game first</span>
        </label>
        <div class="footer-actions">
          <button @click="emit('close')">Cancel</button>
          <button class="primary" :disabled="!selectedSpirits.length || !selectedBoards.length" @click="startGame">
            Start Game
          </button>
        </div>
      </footer>
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
  width: min(90vw, 720px);
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

.loading-state {
  padding: var(--sp-10);
  text-align: center;
  color: var(--text-muted);
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: var(--sp-5);
  display: flex;
  flex-direction: column;
  gap: var(--sp-5);
}

.form-section {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
}

.section-header {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--sp-4);
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.field-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--text-secondary);
}

.chip-grid {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sp-2);
}

.chip {
  display: inline-flex;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-2) var(--sp-3);
  background: var(--bg-muted);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  cursor: pointer;
  font-size: var(--text-sm);
  transition: all var(--duration-base) var(--ease);
}

.chip:hover {
  border-color: var(--border-strong);
}

.chip.selected {
  background: var(--bg-hover);
  border-color: var(--color-accent);
}

.chip input {
  display: none;
}

.chip-name {
  font-weight: var(--weight-medium);
  color: var(--text-primary);
}

.chip-meta {
  font-size: var(--text-xs);
  color: var(--text-muted);
}

.variant-row {
  display: flex;
  align-items: center;
  gap: var(--sp-4);
  margin-top: var(--sp-2);
  padding-top: var(--sp-3);
  border-top: 1px solid var(--border-subtle);
}

.variant-label {
  font-size: var(--text-xs);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
}

.variant-options {
  display: flex;
  gap: var(--sp-4);
}

.variant-option {
  display: inline-flex;
  align-items: center;
  gap: var(--sp-2);
  font-size: var(--text-sm);
  color: var(--text-secondary);
  cursor: pointer;
}

.variant-option:has(input:disabled) {
  color: var(--text-faint);
  cursor: not-allowed;
}

.variant-note {
  font-size: var(--text-xs);
  color: var(--text-faint);
}

.modal-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--sp-4) var(--sp-5);
  border-top: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.archive-toggle {
  display: inline-flex;
  align-items: center;
  gap: var(--sp-2);
  font-size: var(--text-xs);
  color: var(--text-secondary);
  cursor: pointer;
}

.footer-actions {
  display: flex;
  gap: var(--sp-2);
}
</style>
