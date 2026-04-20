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
  <div class="event-deck card">
    <div class="hdr">
      <div class="hdr-left">
        <h3>Event Deck</h3>
        <span class="rule-tag" title="The first Event card is face-up on Turn 1 but does NOT resolve until Turn 2">
          T1 previews T2 · first event doesn't resolve on T1
        </span>
        <span class="counts">
          <span class="count-chip previewed">{{ deck.previewed.length }} previewed</span>
          <span class="count-chip resolved">{{ deck.resolved.length }} resolved</span>
          <span class="count-chip unseen">{{ deck.unseen }} unseen</span>
        </span>
      </div>
      <div class="tools">
        <button class="ghost" @click="clearAll" title="Clear all tracked events">Reset</button>
      </div>
    </div>

    <div class="preview-input">
      <label class="field-label">Preview next event (reveal face-up):</label>
      <div class="autocomplete">
        <input
          type="text"
          v-model="filterQ"
          placeholder="start typing an event card name…"
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
      <button class="primary" @click="previewCard(filterQ)" :disabled="!filterQ.trim()">
        🔎 Preview (face-up)
      </button>
    </div>

    <div v-if="loading" class="loading">Loading event deck data…</div>
    <div v-else-if="loadError" class="banner error">Couldn't load event.json: {{ loadError }}</div>

    <!-- Previewed cards (face-up; will resolve next turn) -->
    <div v-if="deck.previewed.length" class="preview-list">
      <div class="col-label">Previewed — resolves on Turn {{ round + 1 }}</div>
      <div v-for="(e, i) in deck.previewed" :key="i" class="preview-card">
        <div class="preview-hdr">
          <span class="preview-name">{{ e.name }}</span>
          <span class="preview-turn mono">previewed T{{ e.previewed_on_turn }}</span>
          <div class="row-actions">
            <button class="ghost tiny" @click="resolvePreview(i)" title="Mark this event resolved (played during this turn's Invader phase)">Resolve now →</button>
            <button class="ghost tiny" @click="removePreview(i)" title="Remove this preview (mistake)">×</button>
          </div>
        </div>
        <ul v-if="stageText(e).length" class="stage-lines">
          <li v-for="(line, li) in stageText(e)" :key="li">{{ line }}</li>
        </ul>
      </div>
    </div>

    <!-- Resolved (history) -->
    <details v-if="deck.resolved.length" class="resolved">
      <summary>Resolved events ({{ deck.resolved.length }})</summary>
      <ul>
        <li v-for="(r, i) in deck.resolved" :key="i" class="resolved-item">
          <span class="row-name">{{ r.name }}</span>
          <span class="row-turn mono">previewed T{{ r.previewed_on_turn }} · resolved T{{ r.resolved_on_turn }}</span>
        </li>
      </ul>
    </details>
  </div>
</template>

<style scoped>
.event-deck { display: flex; flex-direction: column; gap: var(--sp-3); }

.hdr {
  display: flex; justify-content: space-between; align-items: center;
  flex-wrap: wrap; gap: var(--sp-3);
}
.hdr-left { display: inline-flex; align-items: center; gap: var(--sp-3); flex-wrap: wrap; }
.hdr h3 { color: var(--text-white); margin: 0; }
.tools { display: inline-flex; gap: var(--sp-2); }

.rule-tag {
  font-size: 0.7rem;
  padding: 2px var(--sp-2);
  border-radius: var(--r-sm);
  background: rgba(199, 125, 255, 0.12);
  color: var(--accent-purple);
  border: 1px solid rgba(199, 125, 255, 0.3);
  font-style: italic;
}

.counts { display: inline-flex; gap: 4px; }
.count-chip {
  display: inline-flex; padding: 1px 6px;
  font-size: 0.68rem;
  font-family: var(--font-mono);
  border-radius: var(--r-full);
  border: 1px solid var(--border-subtle);
  font-weight: var(--fw-medium);
}
.count-chip.previewed { color: var(--accent-purple); background: rgba(199, 125, 255, 0.1); }
.count-chip.resolved  { color: var(--accent-green); background: rgba(82, 183, 136, 0.1); }
.count-chip.unseen    { color: var(--text-muted); }

.preview-input {
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: var(--sp-2);
  align-items: center;
  padding: var(--sp-2);
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
}
.field-label {
  font-size: var(--fs-xs);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
}

.autocomplete { position: relative; }
.autocomplete input { width: 100%; }
.suggestions {
  position: absolute; top: calc(100% + 4px); left: 0; right: 0;
  z-index: 30;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  box-shadow: 0 4px 14px rgba(0, 0, 0, 0.4);
  max-height: 18rem; overflow-y: auto;
}
.suggestion {
  width: 100%;
  display: flex; justify-content: space-between; gap: var(--sp-2);
  padding: var(--sp-1) var(--sp-2);
  text-align: left;
  border: none; background: transparent; color: inherit;
  cursor: pointer;
  border-radius: 0;
  font-size: var(--fs-xs);
}
.suggestion:hover { background: var(--bg-muted); }
.sug-name { font-weight: var(--fw-medium); }
.sug-expansion { color: var(--text-muted); font-size: 0.68rem; font-style: italic; }

.loading, .banner.error {
  padding: var(--sp-2); font-size: var(--fs-xs);
  color: var(--text-muted);
}
.banner.error { color: var(--status-danger); }

.col-label {
  font-size: 0.72rem;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-weight: var(--fw-bold);
  color: var(--text-secondary);
}

.preview-list { display: flex; flex-direction: column; gap: var(--sp-2); }
.preview-card {
  background: var(--bg-inset);
  border: 1px solid rgba(199, 125, 255, 0.3);
  border-left: 3px solid var(--accent-purple);
  border-radius: var(--r-md);
  padding: var(--sp-2) var(--sp-3);
}
.preview-hdr {
  display: grid;
  grid-template-columns: 1fr auto auto;
  gap: var(--sp-2);
  align-items: center;
  font-size: var(--fs-sm);
}
.preview-name { font-weight: var(--fw-semibold); color: var(--text-primary); }
.preview-turn { font-size: var(--fs-xs); color: var(--text-muted); }
.row-actions { display: inline-flex; gap: var(--sp-1); }
.tiny { padding: 2px var(--sp-1); font-size: var(--fs-xs); }

.stage-lines {
  list-style: none; padding: 0; margin: var(--sp-2) 0 0;
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  display: flex; flex-direction: column; gap: 4px;
  border-top: 1px dashed var(--border-subtle);
  padding-top: var(--sp-2);
}
.stage-lines li { line-height: 1.4; }

.resolved summary {
  list-style: none; cursor: pointer; color: var(--text-secondary);
  padding: var(--sp-1) 0; font-size: var(--fs-sm);
}
.resolved summary::-webkit-details-marker { display: none; }
.resolved summary::before {
  content: '▸'; margin-right: var(--sp-2); color: var(--text-muted);
  display: inline-block;
  transition: transform var(--motion-fast);
}
.resolved[open] summary::before { transform: rotate(90deg); }
.resolved ul { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 2px; }
.resolved-item {
  display: flex; gap: var(--sp-2); align-items: baseline;
  padding: 2px var(--sp-2);
  font-size: var(--fs-xs);
  opacity: 0.75;
}
.row-name { font-weight: var(--fw-medium); }
.row-turn { color: var(--text-muted); font-size: 0.68rem; }
</style>
