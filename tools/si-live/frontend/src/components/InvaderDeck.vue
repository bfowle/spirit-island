<script setup lang="ts">
import { computed } from 'vue'

/**
 * Invader deck tracker — shows the 3 columns (Ravage / Build / Explore) + the
 * upcoming stack. Rotates on Advance Turn: ravage → discarded, build →
 * ravage, explore → build, next upcoming → explore.
 *
 * Data model lives on state.invader_deck as a loose JSON object:
 *   {
 *     ravage:   { stage, terrain, notes } | null
 *     build:    { stage, terrain, notes } | null
 *     explore:  { stage, terrain, notes } | null
 *     upcoming: [{ stage, terrain, notes }, ...]
 *     discarded: number   // count of discarded cards
 *   }
 */

export interface InvaderCard {
  stage: number
  terrain: string
  notes?: string
}

export interface InvaderDeckState {
  ravage: InvaderCard | null
  build: InvaderCard | null
  explore: InvaderCard | null
  upcoming: InvaderCard[]
  discarded: number
}

const props = defineProps<{ modelValue: InvaderDeckState | null | undefined }>()
const emit = defineEmits<{ 'update:modelValue': [value: InvaderDeckState] }>()

const deck = computed<InvaderDeckState>(() => props.modelValue ?? makeDefault())

function makeDefault(): InvaderDeckState {
  // Default upcoming stack = base-game 12 terrain-cards; stages per SI rules:
  // Stage I (3 cards), Stage II (4), Stage III (5). Terrains are an
  // illustrative shuffle; user edits to match their actual shuffle.
  return {
    ravage: null,
    build: null,
    explore: null,
    upcoming: [
      { stage: 1, terrain: 'Jungle' },
      { stage: 1, terrain: 'Sands' },
      { stage: 1, terrain: 'Mountain' },
      { stage: 2, terrain: 'Jungle + Sands' },
      { stage: 2, terrain: 'Mountain + Wetland' },
      { stage: 2, terrain: 'Jungle + Wetland' },
      { stage: 2, terrain: 'Mountain + Sands' },
      { stage: 3, terrain: 'Jungle' },
      { stage: 3, terrain: 'Sands' },
      { stage: 3, terrain: 'Wetland' },
      { stage: 3, terrain: 'Mountain' },
      { stage: 3, terrain: 'Coastal Lands' },
    ],
    discarded: 0,
  }
}

function emitUpdate(next: InvaderDeckState) {
  emit('update:modelValue', next)
}

function advance() {
  const d = { ...deck.value }
  const newRavage = d.build
  const newBuild = d.explore
  const newExplore = d.upcoming.length ? d.upcoming[0] : null
  const rest = d.upcoming.slice(1)
  emitUpdate({
    ravage: newRavage,
    build: newBuild,
    explore: newExplore,
    upcoming: rest,
    discarded: d.discarded + (d.ravage ? 1 : 0),
  })
}

function undo() {
  // Reverse: explore → upcoming[0], build → explore, ravage → build. We can't
  // recover the discarded card without history — leave ravage null unless the
  // user restores manually.
  const d = { ...deck.value }
  const newUpcoming: InvaderCard[] = d.explore ? [d.explore, ...d.upcoming] : d.upcoming
  emitUpdate({
    ravage: null,
    build: d.ravage,
    explore: d.build,
    upcoming: newUpcoming,
    discarded: Math.max(0, d.discarded - (d.ravage ? 1 : 0)),
  })
}

function editCell(field: 'ravage' | 'build' | 'explore', key: 'stage' | 'terrain' | 'notes', value: string) {
  const d = { ...deck.value }
  const card = d[field] ? { ...d[field]! } : { stage: 1, terrain: '', notes: '' }
  if (key === 'stage') card.stage = Math.max(1, Math.min(3, Number(value) || 1))
  else if (key === 'terrain') card.terrain = value
  else if (key === 'notes') card.notes = value
  emitUpdate({ ...d, [field]: card })
}

function clearCell(field: 'ravage' | 'build' | 'explore') {
  emitUpdate({ ...deck.value, [field]: null })
}

function editUpcoming(i: number, key: 'stage' | 'terrain', value: string) {
  const d = { ...deck.value }
  const next = [...d.upcoming]
  const card = { ...next[i] }
  if (key === 'stage') card.stage = Math.max(1, Math.min(3, Number(value) || 1))
  else card.terrain = value
  next[i] = card
  emitUpdate({ ...d, upcoming: next })
}

function removeUpcoming(i: number) {
  const d = { ...deck.value }
  const next = [...d.upcoming]
  next.splice(i, 1)
  emitUpdate({ ...d, upcoming: next })
}

function addUpcoming() {
  const d = { ...deck.value }
  emitUpdate({ ...d, upcoming: [...d.upcoming, { stage: 1, terrain: '' }] })
}

function reset() {
  if (!confirm('Reset the invader deck to the default base-game stack?')) return
  emitUpdate(makeDefault())
}

const stageClass = (stage: number | undefined) =>
  stage === 1 ? 'stage-i' : stage === 2 ? 'stage-ii' : 'stage-iii'

const stageLabel = (stage: number | undefined) =>
  stage === 1 ? 'I' : stage === 2 ? 'II' : 'III'
</script>

<template>
  <div class="invader-deck card">
    <div class="hdr">
      <h3>Invader Deck</h3>
      <div class="tools">
        <button class="ghost" @click="undo" title="Move cards back one position (doesn't restore discarded)">Undo</button>
        <button class="ghost" @click="reset" title="Reset to base-game default stack">Reset</button>
        <button class="primary" @click="advance" title="Ravage → discarded, Build → Ravage, Explore → Build, next → Explore">
          Advance deck →
        </button>
      </div>
    </div>

    <div class="columns">
      <div class="column ravage">
        <div class="col-label">Ravage</div>
        <CardCell
          :card="deck.ravage"
          @edit="(k, v) => editCell('ravage', k as 'stage'|'terrain'|'notes', v)"
          @clear="clearCell('ravage')"
        />
      </div>
      <div class="column build">
        <div class="col-label">Build</div>
        <CardCell
          :card="deck.build"
          @edit="(k, v) => editCell('build', k as 'stage'|'terrain'|'notes', v)"
          @clear="clearCell('build')"
        />
      </div>
      <div class="column explore">
        <div class="col-label">Explore</div>
        <CardCell
          :card="deck.explore"
          @edit="(k, v) => editCell('explore', k as 'stage'|'terrain'|'notes', v)"
          @clear="clearCell('explore')"
        />
      </div>
    </div>

    <details class="upcoming">
      <summary>
        <span>Upcoming stack ({{ deck.upcoming.length }})</span>
        <span class="muted"> — discarded: {{ deck.discarded }}</span>
      </summary>
      <div class="upcoming-list">
        <div v-for="(card, i) in deck.upcoming" :key="i" class="upcoming-row">
          <span class="stage-badge" :class="stageClass(card.stage)">{{ stageLabel(card.stage) }}</span>
          <input type="text" :value="card.terrain" placeholder="terrain…" @input="(e) => editUpcoming(i, 'terrain', (e.target as HTMLInputElement).value)" />
          <input type="number" min="1" max="3" :value="card.stage" class="stage-num" @input="(e) => editUpcoming(i, 'stage', (e.target as HTMLInputElement).value)" />
          <button class="ghost tiny" @click="removeUpcoming(i)" title="remove">×</button>
        </div>
        <button class="ghost add-upcoming" @click="addUpcoming">+ Add card</button>
      </div>
    </details>
  </div>
</template>

<script lang="ts">
// Inline sub-component for the Ravage/Build/Explore cell editor.
import { defineComponent } from 'vue'
export const CardCell = defineComponent({
  name: 'CardCell',
  props: {
    card: { type: Object as () => InvaderCard | null, default: null },
  },
  emits: ['edit', 'clear'],
  setup(_props, { emit }) {
    const stageClass = (stage: number | undefined) =>
      stage === 1 ? 'stage-i' : stage === 2 ? 'stage-ii' : 'stage-iii'
    const stageLabel = (stage: number | undefined) =>
      stage === 1 ? 'I' : stage === 2 ? 'II' : 'III'
    return { stageClass, stageLabel, emit }
  },
  template: `
    <div v-if="card" class="card-cell active" :class="stageClass(card.stage)">
      <span class="stage-badge" :class="stageClass(card.stage)">Stage {{ stageLabel(card.stage) }}</span>
      <input type="text" :value="card.terrain" placeholder="terrain…" @input="emit('edit', 'terrain', ($event.target).value)" />
      <input type="text" :value="card.notes || ''" placeholder="notes (optional)" @input="emit('edit', 'notes', ($event.target).value)" class="notes" />
      <button class="ghost tiny" @click="emit('clear')" title="Clear this cell">×</button>
    </div>
    <div v-else class="card-cell empty" @click="emit('edit', 'terrain', '')">
      <span class="empty-label">click to add</span>
    </div>
  `,
})
</script>

<style scoped>
.invader-deck { display: flex; flex-direction: column; gap: var(--sp-3); }

.hdr {
  display: flex; justify-content: space-between; align-items: center;
  flex-wrap: wrap; gap: var(--sp-2);
}
.hdr h3 { color: var(--text-white); }
.tools { display: inline-flex; gap: var(--sp-2); }

.columns {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--sp-2);
}
@media (max-width: 720px) { .columns { grid-template-columns: 1fr; } }

.column { display: flex; flex-direction: column; gap: var(--sp-1); }

.col-label {
  font-size: 0.72rem;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-weight: var(--fw-bold);
}
.column.ravage .col-label  { color: var(--accent-red); }
.column.build .col-label   { color: var(--accent-amber); }
.column.explore .col-label { color: var(--accent-blue); }

:deep(.card-cell) {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-2) var(--sp-3);
  display: flex; flex-direction: column; gap: var(--sp-1);
  min-height: 5rem;
  position: relative;
  transition: border-color var(--motion-fast);
}
:deep(.card-cell.empty) {
  background: transparent;
  border-style: dashed;
  cursor: pointer;
  align-items: center; justify-content: center;
}
:deep(.card-cell.empty:hover) { border-color: var(--accent-blue); background: var(--bg-muted); }
:deep(.card-cell .empty-label) {
  color: var(--text-muted); font-style: italic; font-size: var(--fs-xs);
}
:deep(.card-cell input[type="text"]) { font-size: var(--fs-sm); }
:deep(.card-cell input.notes) {
  font-size: var(--fs-xs); color: var(--text-secondary);
  background: transparent; border: none; padding: 0;
}

.column.ravage  :deep(.card-cell.active)  { border-color: rgba(220, 47, 2, 0.5); }
.column.build   :deep(.card-cell.active)  { border-color: rgba(233, 196, 106, 0.5); }
.column.explore :deep(.card-cell.active)  { border-color: rgba(123, 184, 245, 0.5); }

:deep(.card-cell .tiny) {
  position: absolute; top: 4px; right: 4px;
  padding: 0 6px; font-size: 0.85rem;
  color: var(--text-muted);
}
:deep(.card-cell .tiny:hover) { color: var(--status-danger); background: transparent; border: none; }

.stage-badge {
  display: inline-block;
  font-family: var(--font-mono);
  font-size: 0.7rem;
  padding: 1px 6px;
  border-radius: var(--r-sm);
  font-weight: var(--fw-bold);
  letter-spacing: 0.05em;
}
.stage-badge.stage-i   { background: var(--badge-approved-bg); color: var(--badge-approved-fg); border: 1px solid rgba(82, 183, 136, 0.4); }
.stage-badge.stage-ii  { background: var(--badge-warn-bg);     color: var(--badge-warn-fg);     border: 1px solid rgba(233, 196, 106, 0.4); }
.stage-badge.stage-iii { background: var(--badge-chosen-bg);   color: var(--badge-chosen-fg);   border: 1px solid rgba(199, 125, 255, 0.4); }

.upcoming {
  border-top: 1px solid var(--border-subtle);
  padding-top: var(--sp-2);
}
.upcoming summary {
  list-style: none; cursor: pointer;
  font-size: var(--fs-sm); color: var(--text-secondary);
}
.upcoming summary::-webkit-details-marker { display: none; }
.upcoming summary::before {
  content: '▸'; margin-right: var(--sp-2); color: var(--text-muted);
  display: inline-block;
  transition: transform var(--motion-fast);
}
.upcoming[open] summary::before { transform: rotate(90deg); }

.upcoming-list {
  display: flex; flex-direction: column;
  gap: 2px; margin-top: var(--sp-2);
  max-height: 24rem; overflow-y: auto;
}
.upcoming-row {
  display: grid;
  grid-template-columns: auto 1fr auto auto;
  gap: var(--sp-2);
  align-items: center;
  padding: 2px var(--sp-1);
  font-size: var(--fs-xs);
}
.upcoming-row:hover { background: var(--bg-muted); border-radius: var(--r-sm); }
.upcoming-row input[type="number"] { width: 3rem; }
.upcoming-row .stage-num { text-align: center; }
.add-upcoming { justify-self: start; font-size: var(--fs-xs); margin-top: var(--sp-2); }

.tiny { padding: 0 var(--sp-1); font-size: var(--fs-xs); color: var(--text-muted); }
.tiny:hover { color: var(--status-danger); background: transparent; border: none; }
.muted { color: var(--text-muted); }
</style>
