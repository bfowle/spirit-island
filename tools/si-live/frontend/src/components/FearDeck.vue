<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import type { FearDeckState, FearCardEntry } from '../types'
import { fetchDeck, type FearCardData } from '../api'

/**
 * Fear deck tracker.
 *
 * 9-slot default: 3 cards per terror level (T1 → T2 → T3 transitions at 1/3
 * and 2/3 of deck depth). Each card has Earned state (drawn into hand when a
 * threshold is crossed) and Resolved state (played in a Fear phase).
 *
 * The component pairs with data/decks/fear.json (served via /api/deck/fear) to
 * provide an autocomplete picker when the player identifies which card came
 * up in their shuffle.
 */

const props = defineProps<{
  modelValue: FearDeckState | null | undefined
  round: number
  terrorLevel: number
  /** Current fear threshold — used when manually banking to record the
   *  equivalent fear-generated amount in the log. */
  fearThreshold?: number
}>()
const emit = defineEmits<{
  'update:modelValue': [value: FearDeckState]
  'log-event': [event: string, details: Record<string, unknown>]
  /** Signal parent to reset `pools.fear_current` to 0 after a manual bank. */
  'reset-fear-pool': []
}>()

const deck = computed<FearDeckState>(() => props.modelValue ?? makeDefault())

function makeDefault(): FearDeckState {
  return {
    deck_size: 9,
    tier_counts: [3, 3, 3],
    earned: [],
    resolved: [],
    unseen: 9,
  }
}

/** Resolved tier counts — explicit if present on state, else evenly-split. */
function effectiveTierCounts(d: FearDeckState): [number, number, number] {
  if (d.tier_counts && d.tier_counts.length === 3) return d.tier_counts
  const each = Math.ceil(d.deck_size / 3)
  const rem = Math.max(0, d.deck_size - 2 * each)
  return [each, each, rem]
}

function setTierCount(tier: 0 | 1 | 2, v: number) {
  const d = deck.value
  const current = effectiveTierCounts(d)
  const next: [number, number, number] = [...current]
  next[tier] = Math.max(0, Math.floor(v))
  const total = next.reduce((a, b) => a + b, 0)
  emit('update:modelValue', {
    ...d,
    tier_counts: next,
    deck_size: total,
    unseen: Math.max(0, total - d.earned.length - d.resolved.length),
  })
}

// ---- deck lookup cache ----
const fearCards = ref<FearCardData[]>([])
const loading = ref(true)
const loadError = ref<string | null>(null)

onMounted(async () => {
  try {
    fearCards.value = await fetchDeck<FearCardData>('fear')
  } catch (e) {
    loadError.value = (e as Error).message
  } finally {
    loading.value = false
  }
})

const cardByName = computed(() => {
  const m = new Map<string, FearCardData>()
  for (const c of fearCards.value) m.set(c.name, c)
  return m
})

// ---- actions ----
function emitUpdate(next: FearDeckState) {
  emit('update:modelValue', next)
}

const newCardName = ref('')
const filterQ = ref('')

const filteredSuggestions = computed(() => {
  const q = filterQ.value.trim().toLowerCase()
  if (!q) return fearCards.value.slice(0, 8)
  return fearCards.value
    .filter(c => c.name.toLowerCase().includes(q))
    .slice(0, 8)
})

/** Bank a face-down fear card (no name yet). Usually auto-called when the
 *  fear pool crosses threshold; manual button available if the player wants
 *  to sync retroactively. Emits a fear_generated log event matching the
 *  threshold so the chart/attribution stay consistent. */
function bankFaceDown() {
  const entry: FearCardEntry = {
    name: '',
    terror_level: props.terrorLevel,
    round: props.round,
  }
  emitUpdate({
    ...deck.value,
    earned: [...deck.value.earned, entry],
    unseen: Math.max(0, deck.value.unseen - 1),
  })
  const amount = props.fearThreshold ?? 4
  emit('log-event', 'fear_generated', {
    amount,
    source: 'Manual bank (face-down)',
  })
  emit('log-event', 'fear_card_earned', {
    terror_level: props.terrorLevel,
    round: props.round,
    manual: true,
  })
  // Reset the fear pool to 0 — matches the SI rule that banking resets the
  // accumulating fear tokens so they can re-accumulate for the next card.
  emit('reset-fear-pool')
}

/** Earn a named card (rare — only if the player names it at earn time). */
function earnCard(name: string) {
  if (!name.trim()) return
  const entry: FearCardEntry = {
    name: name.trim(),
    terror_level: props.terrorLevel,
    round: props.round,
  }
  emitUpdate({
    ...deck.value,
    earned: [...deck.value.earned, entry],
    unseen: Math.max(0, deck.value.unseen - 1),
  })
  newCardName.value = ''
  filterQ.value = ''
}

/** Per-entry naming state — inputs for unnamed earned cards. */
const namingDraft = ref<Record<number, string>>({})

function setNamingDraft(i: number, v: string) {
  namingDraft.value = { ...namingDraft.value, [i]: v }
}

function filteredNameSuggestions(q: string): FearCardData[] {
  const trimmed = (q ?? '').trim().toLowerCase()
  if (!trimmed) return fearCards.value.slice(0, 6)
  return fearCards.value.filter(c => c.name.toLowerCase().includes(trimmed)).slice(0, 8)
}

/** Resolve an earned card — naming happens at this step since the card
 *  was face-down until now. Terror level on the resolved entry is captured
 *  from the CURRENT terror level (props.terrorLevel) because that's the
 *  level whose effect text applied when the card was played in the Fear
 *  phase. The earn-time level stays available via the unresolved entry's
 *  terror_level, which is moot once the card moves to resolved. */
function resolveEarned(i: number, providedName?: string) {
  const d = deck.value
  const entry = d.earned[i]
  if (!entry) return
  const name = (providedName ?? entry.name ?? namingDraft.value[i] ?? '').trim()
  if (!entry.name && !name) {
    // Empty name and no draft — reject; the player must name it
    return
  }
  const remaining = [...d.earned]
  remaining.splice(i, 1)
  emitUpdate({
    ...d,
    earned: remaining,
    resolved: [...d.resolved, {
      ...entry,
      name: name || entry.name,
      terror_level: props.terrorLevel,  // level at resolution time
    }],
  })
  const next = { ...namingDraft.value }
  delete next[i]
  namingDraft.value = next
}

function unearn(i: number) {
  const d = deck.value
  const remaining = [...d.earned]
  remaining.splice(i, 1)
  emitUpdate({ ...d, earned: remaining, unseen: Math.min(d.deck_size, d.unseen + 1) })
}

function resetDeck() {
  if (!confirm('Reset the fear deck to empty (9 unseen cards)?')) return
  emitUpdate(makeDefault())
}

// Visual progress: N dots (T1, T2, T3 per configured tier_counts).
interface Slot {
  idx: number
  terror_level: 1 | 2 | 3
  state: 'unseen' | 'earned' | 'resolved'
  name?: string
}
const slots = computed<Slot[]>(() => {
  const d = deck.value
  const [t1, t2, t3] = effectiveTierCounts(d)
  const slotList: Slot[] = []
  for (let i = 0; i < t1; i++) slotList.push({ idx: slotList.length, terror_level: 1, state: 'unseen' })
  for (let i = 0; i < t2; i++) slotList.push({ idx: slotList.length, terror_level: 2, state: 'unseen' })
  for (let i = 0; i < t3; i++) slotList.push({ idx: slotList.length, terror_level: 3, state: 'unseen' })
  // Fill earned/resolved in order: earliest resolved then earliest earned
  let cursor = 0
  for (const r of d.resolved) {
    if (cursor < slotList.length) {
      slotList[cursor].state = 'resolved'
      slotList[cursor].name = r.name
      slotList[cursor].terror_level = r.terror_level as 1 | 2 | 3
      cursor++
    }
  }
  for (const e of d.earned) {
    if (cursor < slotList.length) {
      slotList[cursor].state = 'earned'
      slotList[cursor].name = e.name
      slotList[cursor].terror_level = e.terror_level as 1 | 2 | 3
      cursor++
    }
  }
  return slotList
})

function cardText(name: string, tl: number): string {
  const c = cardByName.value.get(name)
  if (!c) return ''
  if (tl === 3) return c.text_stage_3 ?? ''
  if (tl === 2) return c.text_stage_2 ?? ''
  return c.text_stage_1 ?? ''
}
</script>

<template>
  <div class="fear-deck card">
    <div class="hdr">
      <div class="hdr-left">
        <h3>Fear Deck</h3>
        <span class="tier-tag" :data-tier="terrorLevel">Terror {{ terrorLevel }}</span>
        <span class="counts">
          <span class="count-chip earned">{{ deck.earned.length }} earned</span>
          <span class="count-chip resolved">{{ deck.resolved.length }} resolved</span>
          <span class="count-chip unseen">{{ deck.unseen }} unseen</span>
        </span>
      </div>
      <div class="tools">
        <button class="ghost" @click="resetDeck" title="Reset deck to empty">Reset</button>
      </div>
    </div>

    <!-- Editable tier-count split: respects player count + adversary. -->
    <div class="tier-editor">
      <span class="field-label">Deck size by terror level:</span>
      <label class="tier-edit">
        <span class="tier-label t1">T1</span>
        <input
          type="number"
          min="0"
          :value="effectiveTierCounts(deck)[0]"
          @change="setTierCount(0, Number(($event.target as HTMLInputElement).value))"
        />
      </label>
      <label class="tier-edit">
        <span class="tier-label t2">T2</span>
        <input
          type="number"
          min="0"
          :value="effectiveTierCounts(deck)[1]"
          @change="setTierCount(1, Number(($event.target as HTMLInputElement).value))"
        />
      </label>
      <label class="tier-edit">
        <span class="tier-label t3">T3</span>
        <input
          type="number"
          min="0"
          :value="effectiveTierCounts(deck)[2]"
          @change="setTierCount(2, Number(($event.target as HTMLInputElement).value))"
        />
      </label>
      <span class="tier-total">= {{ deck.deck_size }} total</span>
    </div>

    <!-- Visual 9-slot stack -->
    <div class="slots-row" role="list" aria-label="Fear deck slots">
      <div
        v-for="s in slots"
        :key="s.idx"
        class="slot"
        :class="[`t${s.terror_level}`, s.state]"
        role="listitem"
        :title="s.name ? `${s.name} (T${s.terror_level})` : `Slot ${s.idx + 1} · T${s.terror_level}`"
      >
        <span class="slot-tl">T{{ s.terror_level }}</span>
        <span v-if="s.state === 'resolved'" class="slot-check">✓</span>
        <span v-else-if="s.state === 'earned'" class="slot-dot">●</span>
        <span v-else class="slot-blank">○</span>
      </div>
    </div>

    <!-- Bank face-down card (threshold crossing) -->
    <div class="bank-row">
      <div class="bank-text">
        <strong>Banking</strong>: auto-triggered when fear crosses {{ props.round ? 'the threshold' : '0' }}.
        Manual bank below — for recording a missed threshold crossing.
      </div>
      <button class="primary" @click="bankFaceDown" title="Record a fear card earned face-down (threshold crossed). Name it at resolve time.">
        🎯 Bank 1 face-down
      </button>
    </div>

    <!-- Optional: pre-name an earned card if already revealed (rare) -->
    <details class="prename">
      <summary>Pre-name a card at earn time (rare)</summary>
      <div class="draw-row">
        <label class="field-label">Named earn:</label>
        <div class="autocomplete">
          <input
            type="text"
            v-model="filterQ"
            placeholder="start typing a fear card name…"
            @keyup.enter="earnCard(filterQ)"
          />
          <div v-if="filterQ && filteredSuggestions.length" class="suggestions">
            <button
              v-for="c in filteredSuggestions"
              :key="c.name"
              type="button"
              class="suggestion"
              @click="earnCard(c.name)"
            >
              <span class="sug-name">{{ c.name }}</span>
              <span class="sug-expansion">{{ c.expansion }}</span>
            </button>
          </div>
        </div>
        <button class="ghost" @click="earnCard(filterQ || newCardName)" :disabled="!filterQ.trim()">
          + Earn named
        </button>
      </div>
    </details>

    <div v-if="loading" class="loading">Loading fear deck data…</div>
    <div v-else-if="loadError" class="banner error">Couldn't load fear.json: {{ loadError }}</div>

    <!-- Earned (awaiting resolution in Fear phase) -->
    <div v-if="deck.earned.length" class="earned-list">
      <div class="col-label">Earned — face-down, awaiting Fear-phase resolution</div>
      <ul>
        <li v-for="(e, i) in deck.earned" :key="i" class="earned-item" :class="`t${e.terror_level}`">
          <span class="row-tl">T{{ e.terror_level }}</span>
          <span v-if="e.name" class="row-name">{{ e.name }}</span>
          <span v-else class="row-name facedown">(face-down · name at resolve)</span>
          <span class="row-round mono">r{{ e.round }}</span>
          <div class="row-text" v-if="e.name && cardText(e.name, e.terror_level)">{{ cardText(e.name, e.terror_level) }}</div>
          <!-- Name-on-resolve for face-down cards -->
          <div v-if="!e.name" class="resolve-name-row">
            <div class="autocomplete">
              <input
                type="text"
                :value="namingDraft[i] ?? ''"
                placeholder="name the revealed card when resolving…"
                @input="setNamingDraft(i, ($event.target as HTMLInputElement).value)"
                @keyup.enter="resolveEarned(i)"
              />
              <div v-if="namingDraft[i] && filteredNameSuggestions(namingDraft[i]).length" class="suggestions">
                <button
                  v-for="c in filteredNameSuggestions(namingDraft[i])"
                  :key="c.name"
                  type="button"
                  class="suggestion"
                  @click="resolveEarned(i, c.name)"
                >
                  <span class="sug-name">{{ c.name }}</span>
                  <span class="sug-expansion">{{ c.expansion }}</span>
                </button>
              </div>
            </div>
          </div>
          <div class="row-actions">
            <button
              v-if="e.name"
              class="ghost tiny"
              @click="resolveEarned(i)"
              title="Mark resolved (Fear phase played)"
            >Resolve →</button>
            <button
              v-else
              class="primary tiny"
              @click="resolveEarned(i)"
              :disabled="!namingDraft[i]?.trim()"
              title="Reveal + resolve — name required"
            >Reveal + Resolve →</button>
            <button class="ghost tiny" @click="unearn(i)" title="Undo earn">×</button>
          </div>
        </li>
      </ul>
    </div>

    <!-- Resolved (history) -->
    <details v-if="deck.resolved.length" class="resolved">
      <summary>Resolved ({{ deck.resolved.length }})</summary>
      <ul>
        <li v-for="(r, i) in deck.resolved" :key="i" class="resolved-item" :class="`t${r.terror_level}`">
          <span class="row-tl">T{{ r.terror_level }}</span>
          <span class="row-name">{{ r.name }}</span>
          <span class="row-round mono">r{{ r.round }}</span>
        </li>
      </ul>
    </details>
  </div>
</template>

<style scoped>
/* ═══════════════════════════════════════════════════════════════════════════ */
/* FEAR DECK — AEGIS DESIGN SYSTEM                                              */
/* ═══════════════════════════════════════════════════════════════════════════ */

.fear-deck {
  display: flex;
  flex-direction: column;
  gap: var(--sp-4);
  padding: var(--sp-4);
}

.hdr {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--sp-3);
}

.hdr-left {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  flex-wrap: wrap;
}

.hdr h3 {
  color: var(--text-primary);
  margin: 0;
  font-size: var(--text-base);
  font-weight: var(--weight-semibold);
}

.tools {
  display: flex;
  gap: var(--sp-2);
}

.tier-tag {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  padding: var(--sp-1) var(--sp-2);
  border-radius: var(--radius-sm);
  background: var(--bg-muted);
}

.tier-tag[data-tier="1"] { color: var(--color-accent); }
.tier-tag[data-tier="2"] { color: var(--color-warning); }
.tier-tag[data-tier="3"] { color: var(--color-danger); }

.counts {
  display: flex;
  gap: 4px;
}

.count-chip {
  display: inline-flex;
  padding: 2px 8px;
  font-size: var(--text-xs);
  font-family: var(--font-mono);
  border-radius: 12px;
  border: 1px solid var(--border-subtle);
  font-weight: var(--weight-medium);
}

.count-chip.earned {
  color: var(--color-fear);
  background: rgba(245, 158, 11, 0.1);
}

.count-chip.resolved {
  color: var(--color-success);
  background: rgba(34, 197, 94, 0.1);
}

.count-chip.unseen {
  color: var(--text-muted);
}

/* ─── TIER EDITOR ─── */
.tier-editor {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-3);
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  flex-wrap: wrap;
}

.tier-edit {
  display: flex;
  align-items: center;
  gap: 4px;
  background: var(--bg-muted);
  padding: var(--sp-1) var(--sp-2);
  border-radius: var(--radius-sm);
}

.tier-edit input {
  width: 2.5rem;
  padding: var(--sp-1);
  font-family: var(--font-mono);
  text-align: center;
  font-size: var(--text-sm);
}

.tier-label {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  font-weight: var(--weight-bold);
}

.tier-label.t1 { color: var(--color-accent); }
.tier-label.t2 { color: var(--color-warning); }
.tier-label.t3 { color: var(--color-danger); }

.tier-total {
  font-size: var(--text-xs);
  color: var(--text-muted);
  font-family: var(--font-mono);
}

/* ─── SLOTS ROW ─── */
.slots-row {
  display: grid;
  grid-template-columns: repeat(9, minmax(0, 1fr));
  gap: 4px;
  padding: var(--sp-3);
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
}

@media (max-width: 540px) {
  .slots-row { grid-template-columns: repeat(3, 1fr); }
}

.slot {
  aspect-ratio: 1 / 1;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  position: relative;
  font-size: var(--text-xs);
  background: var(--bg-muted);
  transition: transform var(--duration-base) var(--ease);
}

.slot.t1 { border-top: 3px solid var(--color-accent); }
.slot.t2 { border-top: 3px solid var(--color-warning); }
.slot.t3 { border-top: 3px solid var(--color-danger); }

.slot.earned { background: rgba(245, 158, 11, 0.1); }
.slot.resolved { background: rgba(34, 197, 94, 0.1); opacity: 0.7; }

.slot-tl {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-muted);
}

.slot-check { color: var(--color-success); font-weight: var(--weight-bold); }
.slot-dot { color: var(--color-fear); }
.slot-blank { color: var(--text-faint); }

/* ─── BANK ROW ─── */
.bank-row {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: var(--sp-3);
  align-items: center;
  padding: var(--sp-3) var(--sp-4);
  background: rgba(245, 158, 11, 0.08);
  border: 1px solid rgba(245, 158, 11, 0.25);
  border-radius: var(--radius-md);
}

.bank-text {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  line-height: 1.5;
}

.bank-text strong { color: var(--color-fear); }

/* ─── PRENAME ─── */
.prename summary {
  list-style: none;
  cursor: pointer;
  color: var(--text-secondary);
  padding: var(--sp-2) 0;
  font-size: var(--text-xs);
}

.prename summary::-webkit-details-marker { display: none; }

.prename summary::before {
  content: ">";
  margin-right: var(--sp-2);
  color: var(--text-muted);
  font-family: var(--font-mono);
  display: inline-block;
  transition: transform var(--duration-base) var(--ease);
}

.prename[open] summary::before { transform: rotate(90deg); }

.draw-row {
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: var(--sp-2);
  align-items: center;
  padding: var(--sp-3);
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
}

.facedown {
  color: var(--text-muted);
  font-style: italic;
}

.resolve-name-row {
  grid-column: 1 / -1;
  margin-top: var(--sp-2);
  position: relative;
}

.resolve-name-row .autocomplete { position: relative; }
.resolve-name-row input { width: 100%; }

.resolve-name-row .suggestions {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  z-index: 30;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  max-height: 14rem;
  overflow-y: auto;
}

.resolve-name-row .suggestion {
  width: 100%;
  display: flex;
  justify-content: space-between;
  padding: var(--sp-2) var(--sp-3);
  border: none;
  background: transparent;
  color: var(--text-secondary);
  text-align: left;
  cursor: pointer;
  font-size: var(--text-xs);
}

.resolve-name-row .suggestion:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.field-label {
  font-size: var(--text-xs);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  font-weight: var(--weight-medium);
}

.autocomplete { position: relative; }
.autocomplete input { width: 100%; }

.suggestions {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  z-index: 30;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  max-height: 16rem;
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
  color: var(--text-secondary);
  cursor: pointer;
}

.suggestion:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.sug-name { font-weight: var(--weight-medium); }
.sug-expansion { color: var(--text-muted); font-size: var(--text-xs); font-style: italic; }

.loading, .banner.error {
  padding: var(--sp-3);
  font-size: var(--text-xs);
  color: var(--text-muted);
}

.banner.error { color: var(--color-danger); }

.col-label {
  font-size: var(--text-xs);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-weight: var(--weight-semibold);
  color: var(--text-muted);
}

/* ─── EARNED & RESOLVED LISTS ─── */
.earned-list ul, .resolved ul {
  list-style: none;
  padding: 0;
  margin: var(--sp-2) 0;
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.earned-item, .resolved-item {
  display: grid;
  grid-template-columns: auto 1fr auto auto;
  gap: var(--sp-2);
  align-items: start;
  padding: var(--sp-3) var(--sp-4);
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
}

.earned-item.t1, .resolved-item.t1 { border-left: 3px solid var(--color-accent); }
.earned-item.t2, .resolved-item.t2 { border-left: 3px solid var(--color-warning); }
.earned-item.t3, .resolved-item.t3 { border-left: 3px solid var(--color-danger); }

.row-tl {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-muted);
  padding-top: 2px;
}

.row-name {
  font-weight: var(--weight-medium);
  color: var(--text-primary);
}

.row-round {
  font-size: var(--text-xs);
  color: var(--text-muted);
  font-family: var(--font-mono);
}

.row-text {
  grid-column: 2 / 3;
  font-size: var(--text-xs);
  color: var(--text-secondary);
  line-height: 1.5;
  margin-top: 2px;
}

.row-actions {
  grid-column: 4 / 5;
  display: flex;
  gap: var(--sp-1);
}

.tiny {
  padding: var(--sp-1) var(--sp-2);
  font-size: var(--text-xs);
}

/* ─── RESOLVED SECTION ─── */
.resolved summary {
  list-style: none;
  cursor: pointer;
  color: var(--text-secondary);
  padding: var(--sp-2) 0;
  font-size: var(--text-sm);
}

.resolved summary::-webkit-details-marker { display: none; }

.resolved summary::before {
  content: ">";
  margin-right: var(--sp-2);
  color: var(--text-muted);
  font-family: var(--font-mono);
  display: inline-block;
  transition: transform var(--duration-base) var(--ease);
}

.resolved[open] summary::before { transform: rotate(90deg); }
.resolved-item { opacity: 0.7; }
</style>
