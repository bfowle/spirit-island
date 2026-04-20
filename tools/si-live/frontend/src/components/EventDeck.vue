<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import type { EventDeckState, EventCardEntry } from '../types'
import { fetchDeck, type EventCardData } from '../api'

/**
 * Event deck tracker.
 *
 * **Rule reminder**: The first Event card is flipped face-up during Setup so
 * the spirits know what's coming on Turn 2. The previewed card does NOT
 * resolve on Turn 1. On each Advance Turn, the previewed card moves to
 * resolved, and a new card is previewed for the following turn.
 *
 * The tool doesn't shuffle programmatically — the player identifies which
 * card they flipped via an autocomplete against data/decks/event.json (62
 * Branch & Claw / Jagged Earth / Nature Incarnate events).
 */

const props = defineProps<{
  modelValue: EventDeckState | null | undefined
  /** Current round; used to tag when a preview/resolve was recorded. */
  round: number
}>()
const emit = defineEmits<{ 'update:modelValue': [value: EventDeckState] }>()

const deck = computed<EventDeckState>(() => props.modelValue ?? makeDefault())

function makeDefault(): EventDeckState {
  return { previewed: [], resolved: [], unseen: 62 }
}

// ---- card lookup ----
const events = ref<EventCardData[]>([])
const loading = ref(true)
const loadError = ref<string | null>(null)
onMounted(async () => {
  try {
    events.value = await fetchDeck<EventCardData>('event')
  } catch (e) {
    loadError.value = (e as Error).message
  } finally {
    loading.value = false
  }
})

const cardByName = computed(() => {
  const m = new Map<string, EventCardData>()
  for (const c of events.value) m.set(c.name, c)
  return m
})

const filterQ = ref('')
const suggestions = computed(() => {
  const q = filterQ.value.trim().toLowerCase()
  if (!q) return events.value.slice(0, 8)
  return events.value
    .filter(c => c.name.toLowerCase().includes(q))
    .slice(0, 10)
})

function emitUpdate(next: EventDeckState) {
  emit('update:modelValue', next)
}

function previewCard(name: string) {
  if (!name.trim()) return
  const entry: EventCardEntry = {
    name: name.trim(),
    previewed_on_turn: props.round,
    resolved_on_turn: null,
  }
  // Replace any existing preview — typically only one Event is face-up at a time.
  emitUpdate({
    ...deck.value,
    previewed: [...deck.value.previewed, entry],
    unseen: Math.max(0, deck.value.unseen - 1),
  })
  filterQ.value = ''
}

function resolvePreview(i: number) {
  const d = deck.value
  const entry = d.previewed[i]
  if (!entry) return
  const remaining = [...d.previewed]
  remaining.splice(i, 1)
  emitUpdate({
    ...d,
    previewed: remaining,
    resolved: [...d.resolved, { ...entry, resolved_on_turn: props.round }],
  })
}

function removePreview(i: number) {
  const d = deck.value
  const remaining = [...d.previewed]
  remaining.splice(i, 1)
  emitUpdate({
    ...d,
    previewed: remaining,
    unseen: Math.min(d.unseen + 1, d.unseen + 1),
  })
}

function clearAll() {
  if (!confirm('Clear all tracked events?')) return
  emitUpdate(makeDefault())
}

function stageText(entry: EventCardEntry): string[] {
  const card = cardByName.value.get(entry.name)
  if (!card?.stages) return []
  return card.stages.map(s => {
    const name = s.event_name ? ` (${s.event_name})` : ''
    return `${s.event_type}${name}: ${s.text}`
  })
}
</script>

<template>
  <div class="event-deck">
    <div class="deck-intro">
      <span class="rule-hint">T1 previews T2 - first event doesn&apos;t resolve on T1</span>
      <div class="deck-stats">
        <span class="stat previewed">{{ deck.previewed.length }} previewed</span>
        <span class="stat resolved">{{ deck.resolved.length }} resolved</span>
        <span class="stat unseen">{{ deck.unseen }} unseen</span>
      </div>
      <button class="reset-btn" @click="clearAll">Reset</button>
    </div>

    <div class="preview-input">
      <div class="autocomplete">
        <input
          type="text"
          v-model="filterQ"
          placeholder="Type event card name..."
          @keyup.enter="previewCard(filterQ)"
        />
        <div v-if="filterQ && suggestions.length" class="suggestions">
          <button
            v-for="c in suggestions"
            :key="c.name"
            type="button"
            class="suggestion"
            @click="previewCard(c.name)"
          >
            <span class="sug-name">{{ c.name }}</span>
            <span class="sug-expansion">{{ c.expansion }}</span>
          </button>
        </div>
      </div>
      <button class="preview-btn" @click="previewCard(filterQ)" :disabled="!filterQ.trim()">
        Preview
      </button>
    </div>

    <div v-if="loading" class="loading-msg">Loading event deck data...</div>
    <div v-else-if="loadError" class="error-msg">Couldn&apos;t load event.json: {{ loadError }}</div>

    <!-- Previewed cards (face-up; will resolve next turn) -->
    <div v-if="deck.previewed.length" class="preview-list">
      <div class="list-label">Previewed - resolves on Turn {{ round + 1 }}</div>
      <div v-for="(e, i) in deck.previewed" :key="i" class="preview-card">
        <div class="preview-header">
          <span class="preview-name">{{ e.name }}</span>
          <span class="preview-turn">T{{ e.previewed_on_turn }}</span>
        </div>
        <div class="preview-actions">
          <button @click="resolvePreview(i)">Resolve</button>
          <button class="ghost" @click="removePreview(i)">Remove</button>
        </div>
        <ul v-if="stageText(e).length" class="stage-lines">
          <li v-for="(line, li) in stageText(e)" :key="li">{{ line }}</li>
        </ul>
      </div>
    </div>

    <!-- Resolved (history) -->
    <details v-if="deck.resolved.length" class="resolved-section">
      <summary>Resolved events ({{ deck.resolved.length }})</summary>
      <ul>
        <li v-for="(r, i) in deck.resolved" :key="i" class="resolved-item">
          <span class="item-name">{{ r.name }}</span>
          <span class="item-turn">T{{ r.previewed_on_turn }} → T{{ r.resolved_on_turn }}</span>
        </li>
      </ul>
    </details>
  </div>
</template>

<style scoped>
.event-deck {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
  padding: var(--sp-4);
}

.deck-intro {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  flex-wrap: wrap;
}

.rule-hint {
  font-size: var(--text-xs);
  padding: var(--sp-1) var(--sp-2);
  border-radius: var(--radius-sm);
  background: rgba(168, 85, 247, 0.1);
  color: var(--color-blight);
  border: 1px solid rgba(168, 85, 247, 0.2);
  font-style: italic;
}

.deck-stats {
  display: flex;
  gap: var(--sp-2);
}

.stat {
  font-size: var(--text-xs);
  font-family: var(--font-mono);
  padding: 2px var(--sp-2);
  border-radius: 10px;
  background: var(--bg-muted);
}

.stat.previewed { color: var(--color-blight); background: rgba(168, 85, 247, 0.1); }
.stat.resolved { color: var(--color-success); background: rgba(34, 197, 94, 0.1); }
.stat.unseen { color: var(--text-muted); }

.reset-btn {
  margin-left: auto;
  height: 28px;
  padding: 0 var(--sp-3);
  font-size: var(--text-xs);
  background: transparent;
  border: 1px solid var(--border-default);
  color: var(--text-muted);
}

.reset-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.preview-input {
  display: flex;
  gap: var(--sp-2);
}

.autocomplete {
  flex: 1;
  position: relative;
}

.autocomplete input {
  width: 100%;
}

.suggestions {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  z-index: 30;
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  max-height: 240px;
  overflow-y: auto;
}

.suggestion {
  width: 100%;
  display: flex;
  justify-content: space-between;
  gap: var(--sp-2);
  padding: var(--sp-2) var(--sp-3);
  text-align: left;
  border: none;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  border-radius: 0;
  font-size: var(--text-sm);
  height: auto;
}

.suggestion:hover {
  background: var(--bg-hover);
}

.sug-name {
  font-weight: var(--weight-medium);
}

.sug-expansion {
  color: var(--text-muted);
  font-size: var(--text-xs);
  font-style: italic;
}

.preview-btn {
  background: var(--color-blight);
  color: white;
  border-color: var(--color-blight);
}

.preview-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.loading-msg,
.error-msg {
  padding: var(--sp-2);
  font-size: var(--text-xs);
  color: var(--text-muted);
}

.error-msg {
  color: var(--color-danger);
}

.list-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  margin-bottom: var(--sp-2);
}

.preview-list {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.preview-card {
  background: var(--bg-muted);
  border: 1px solid rgba(168, 85, 247, 0.2);
  border-left: 3px solid var(--color-blight);
  border-radius: var(--radius-md);
  padding: var(--sp-3);
}

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--sp-2);
}

.preview-name {
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
}

.preview-turn {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-muted);
}

.preview-actions {
  display: flex;
  gap: var(--sp-2);
}

.preview-actions button {
  height: 28px;
  padding: 0 var(--sp-3);
  font-size: var(--text-xs);
}

.preview-actions button.ghost {
  background: transparent;
  border-color: transparent;
  color: var(--text-muted);
}

.stage-lines {
  list-style: none;
  padding: 0;
  margin: var(--sp-3) 0 0;
  font-size: var(--text-xs);
  color: var(--text-secondary);
  display: flex;
  flex-direction: column;
  gap: var(--sp-1);
  border-top: 1px dashed var(--border-subtle);
  padding-top: var(--sp-2);
}

.resolved-section summary {
  cursor: pointer;
  color: var(--text-secondary);
  padding: var(--sp-2) 0;
  font-size: var(--text-sm);
  list-style: none;
}

.resolved-section summary::-webkit-details-marker {
  display: none;
}

.resolved-section summary::before {
  content: '▸';
  margin-right: var(--sp-2);
  color: var(--text-muted);
  display: inline-block;
  transition: transform var(--duration-base) var(--ease);
}

.resolved-section[open] summary::before {
  transform: rotate(90deg);
}

.resolved-section ul {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.resolved-item {
  display: flex;
  justify-content: space-between;
  padding: var(--sp-1) var(--sp-2);
  font-size: var(--text-xs);
  opacity: 0.7;
}

.item-name {
  font-weight: var(--weight-medium);
}

.item-turn {
  font-family: var(--font-mono);
  color: var(--text-muted);
}
</style>
