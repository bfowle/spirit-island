<script setup lang="ts">
import { computed, ref } from 'vue'
import type { InvaderCard, InvaderDeckState } from '../types'
import { fetchInvaderStack } from '../api'

/**
 * Invader deck tracker.
 *
 * Stack sequence is KNOWN at setup — it comes from data/adversary-stacks.json
 * based on the chosen adversary/level (e.g., base 3·4·5 = [1,1,1,2,2,2,2,3,3,3,3,3]).
 * Each upcoming card has a predetermined STAGE; the TERRAIN is what the player
 * reveals as they flip.
 *
 * Flow:
 *   1. At setup, the Upcoming column shows the full stack with only stages
 *      visible (terrain blank).
 *   2. On "Reveal first card" the top upcoming card moves into Explore with
 *      the player setting its terrain.
 *   3. On Advance Turn (wired into TurnController) the cards rotate:
 *        ravage → discarded, build → ravage, explore → build, upcoming[0] → explore.
 *   4. The next card's terrain is revealed when it hits Explore.
 */

const props = defineProps<{
  modelValue: InvaderDeckState | null | undefined
  /** Passed from App so Rebuild fetches the canonical stack for this matchup. */
  adversary?: string | null
  level?: number | null
}>()
const emit = defineEmits<{ 'update:modelValue': [value: InvaderDeckState] }>()

const deck = computed<InvaderDeckState>(() => props.modelValue ?? makeDefault())

function makeDefault(): InvaderDeckState {
  return {
    ravage: null,
    build: null,
    explore: null,
    upcoming: [1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3].map(stage => ({ stage, terrain: null })),
    discarded: [],
    notation: '3 · 4 · 5',
  }
}

function emitUpdate(next: InvaderDeckState) {
  emit('update:modelValue', next)
}

/** Reveal the top upcoming card into Explore. Used during initial setup. */
function revealNext() {
  const d = deck.value
  if (!d.upcoming.length) return
  const [next, ...rest] = d.upcoming
  emitUpdate({
    ...d,
    explore: { ...next, terrain: next.terrain ?? '' },
    upcoming: rest,
  })
}

function editCellTerrain(field: 'ravage' | 'build' | 'explore', value: string) {
  const d = deck.value
  const card = d[field]
  if (!card) return
  emitUpdate({ ...d, [field]: { ...card, terrain: value } })
}

function editCellNotes(field: 'ravage' | 'build' | 'explore', value: string) {
  const d = deck.value
  const card = d[field]
  if (!card) return
  emitUpdate({ ...d, [field]: { ...card, notes: value } })
}

function editUpcomingTerrain(i: number, value: string) {
  const d = deck.value
  const next = [...d.upcoming]
  next[i] = { ...next[i], terrain: value }
  emitUpdate({ ...d, upcoming: next })
}

function reset() {
  if (!confirm('Reset the invader deck to the default base-game stack?')) return
  emitUpdate(makeDefault())
}

/**
 * Advance the invader deck by one step, independent of the Turn controller.
 *   Ravage → Discarded
 *   Build  → Ravage
 *   Explore → Build
 *   Upcoming[0] → Explore (new reveal)
 * This lets the player rotate cards mid-round (e.g., to pre-reveal the next
 * Explore during Invader phase) without triggering a full Advance Turn.
 */
function advanceDeck() {
  const d = deck.value
  const ravaged = d.ravage
  const newRavage = d.build
  const newBuild = d.explore
  const newExplore = d.upcoming.length ? { ...d.upcoming[0], terrain: d.upcoming[0].terrain ?? '' } : null
  const rest = d.upcoming.slice(1)
  const newDiscarded = ravaged ? [...d.discarded, ravaged] : d.discarded
  emitUpdate({
    ...d,
    ravage: newRavage,
    build: newBuild,
    explore: newExplore,
    upcoming: rest,
    discarded: newDiscarded,
  })
}

const rebuilding = ref(false)
const rebuildError = ref<string | null>(null)

async function rebuildFromAdversary() {
  const advLabel = props.adversary
    ? `${props.adversary}${props.level != null ? ` L${props.level}` : ''}`
    : 'base game'
  if (!confirm(
    `Rebuild the upcoming stack from the canonical sequence for ${advLabel}?\n\n` +
    `Already-flipped cards (Ravage / Build / Explore / Discarded) are preserved.\n` +
    `Only the remaining upcoming cards are replaced.`,
  )) return
  rebuilding.value = true
  rebuildError.value = null
  try {
    const fresh = await fetchInvaderStack(props.adversary, props.level)
    const revealedCount = deck.value.discarded.length
      + (deck.value.ravage ? 1 : 0)
      + (deck.value.build ? 1 : 0)
      + (deck.value.explore ? 1 : 0)
    // Skip the first N entries from the canonical sequence since those already got revealed.
    const freshSeq = (fresh.stack_sequence ?? []).slice(revealedCount)
    const newUpcoming: InvaderCard[] = freshSeq.map(stage => ({ stage, terrain: null }))
    emitUpdate({
      ...deck.value,
      upcoming: newUpcoming,
      notation: fresh.notation ?? deck.value.notation,
    })
  } catch (e) {
    rebuildError.value = (e as Error).message
  } finally {
    rebuilding.value = false
  }
}

function undo() {
  const d = deck.value
  const restored: InvaderCard | undefined = d.discarded.length ? d.discarded[d.discarded.length - 1] : undefined
  const newDiscarded = d.discarded.slice(0, -1)
  const newUpcoming = d.explore ? [d.explore, ...d.upcoming] : d.upcoming
  emitUpdate({
    ravage: restored ?? null,
    build: d.ravage,
    explore: d.build,
    upcoming: newUpcoming,
    discarded: newDiscarded,
    notation: d.notation,
  })
}

const stageClass = (stage: number | undefined) =>
  stage === 1 ? 'stage-i' : stage === 2 ? 'stage-ii' : 'stage-iii'

const stageLabel = (stage: number | undefined) =>
  stage === 1 ? 'I' : stage === 2 ? 'II' : 'III'

// Group upcoming by stage for a clean visual stack
interface StackGroup {
  stage: number
  cards: { idx: number; card: InvaderCard }[]
}
const groupedUpcoming = computed<StackGroup[]>(() => {
  const groups: StackGroup[] = []
  let current: StackGroup | null = null
  deck.value.upcoming.forEach((card, idx) => {
    if (!current || current.stage !== card.stage) {
      current = { stage: card.stage, cards: [] }
      groups.push(current)
    }
    current.cards.push({ idx, card })
  })
  return groups
})

// Total-remaining stage counts include exposed cells (Ravage/Build/Explore)
// plus upcoming — the player cares about "how many Stage III cards are still
// in play" not just "how many are in the face-down stack".
const stageCounts = computed(() => {
  const counts = { I: 0, II: 0, III: 0 }
  const tally = (stage: number | undefined) => {
    if (stage === 1) counts.I++
    else if (stage === 2) counts.II++
    else if (stage === 3) counts.III++
  }
  if (deck.value.ravage) tally(deck.value.ravage.stage)
  if (deck.value.build) tally(deck.value.build.stage)
  if (deck.value.explore) tally(deck.value.explore.stage)
  for (const c of deck.value.upcoming) tally(c.stage)
  return counts
})

const exposedAndUpcoming = computed(() => {
  const exposed = [deck.value.ravage, deck.value.build, deck.value.explore].filter(Boolean).length
  return exposed + deck.value.upcoming.length
})

const showSetupReveal = computed(
  () => !deck.value.explore && !deck.value.build && !deck.value.ravage && deck.value.upcoming.length > 0,
)

const topStage = computed(() => deck.value.upcoming[0]?.stage ?? 1)

// Terrain catalog keyed by stage — matches the canonical base-game card identities.
//   Stage I   (4 cards): single-terrain
//   Stage II  (5 cards): single-terrain + Coastal Lands
//   Stage III (4 cards): two-terrain pair cards
function terrainOptionsForStage(stage: number | undefined): string[] {
  if (stage === 1) return ['Mountain', 'Jungle', 'Sands', 'Wetland']
  if (stage === 2) return ['Mountain', 'Jungle', 'Sands', 'Wetland', 'Coastal Lands']
  if (stage === 3) return [
    'Jungle + Mountain',
    'Jungle + Sands',
    'Jungle + Wetland',
    'Mountain + Sands',
    'Mountain + Wetland',
    'Sands + Wetland',
  ]
  return []
}

// Tracks which terrain input is currently focused (drives the dropdown visibility).
// Key format: 'ravage' | 'build' | 'explore' | `upcoming:${idx}`
const focusedField = ref<string | null>(null)

function filteredFor(stage: number | undefined, currentValue: string | null | undefined): string[] {
  const all = terrainOptionsForStage(stage)
  const q = (currentValue ?? '').trim().toLowerCase()
  if (!q) return all
  // Case-insensitive substring filter; keep order of the canonical list.
  const matches = all.filter(s => s.toLowerCase().includes(q))
  // If the user types something novel, still show all options as alternatives
  return matches.length > 0 ? matches : all
}

function setCellTerrain(field: 'ravage' | 'build' | 'explore', value: string) {
  editCellTerrain(field, value)
  focusedField.value = null
}
function setUpcomingTerrain(i: number, value: string) {
  editUpcomingTerrain(i, value)
  focusedField.value = null
}
function handleBlur() {
  // delay so a click on a suggestion button registers before the dropdown closes
  setTimeout(() => { focusedField.value = null }, 150)
}
</script>

<template>
  <div class="invader-deck card">
    <div class="hdr">
      <div class="hdr-left">
        <h3>Invader Deck</h3>
        <span class="notation">{{ deck.notation ?? '3 · 4 · 5' }}</span>
        <span class="counts">
          <span class="count-chip stage-i">{{ stageCounts.I }}·I</span>
          <span class="count-chip stage-ii">{{ stageCounts.II }}·II</span>
          <span class="count-chip stage-iii">{{ stageCounts.III }}·III</span>
          <span class="count-chip muted" title="Discarded (ravaged)">{{ deck.discarded.length }} discarded</span>
        </span>
      </div>
      <div class="tools">
        <button
          class="primary"
          @click="advanceDeck"
          :disabled="!deck.upcoming.length && !deck.explore && !deck.build && !deck.ravage"
          title="Rotate: Ravage→discarded, Build→Ravage, Explore→Build, next Upcoming→Explore"
        >
          Advance deck →
        </button>
        <button
          class="ghost"
          @click="undo"
          :disabled="!deck.discarded.length && !deck.ravage && !deck.build && !deck.explore"
          title="Undo last rotation"
        >Undo</button>
        <button
          class="ghost"
          @click="rebuildFromAdversary"
          :disabled="rebuilding"
          :title="`Rebuild remaining upcoming from canonical ${adversary ?? 'base'} L${level ?? 0} data`"
        >
          {{ rebuilding ? 'Rebuilding…' : 'Rebuild' }}
        </button>
        <button class="ghost" @click="reset" title="Reset to base-game default stack">Reset</button>
      </div>
    </div>

    <div v-if="rebuildError" class="rebuild-err">Rebuild failed: {{ rebuildError }}</div>

    <!-- Setup: no cards revealed yet -->
    <div v-if="showSetupReveal" class="setup-reveal">
      <p class="hint">
        Stack built: <span class="mono seq">{{ deck.notation }}</span> ({{ exposedAndUpcoming }} cards total).
        The top card of this deck is
        <strong :class="stageClass(topStage)">Stage {{ stageLabel(topStage) }}</strong>
        — reveal it and set its terrain.
      </p>
      <button class="primary" @click="revealNext">
        Reveal top card → <span class="reveal-stage" :class="stageClass(topStage)">Stage {{ stageLabel(topStage) }}</span>
      </button>
    </div>

    <!-- Three columns: Ravage / Build / Explore -->
    <div class="columns">
      <div class="column ravage">
        <div class="col-label">Ravage</div>
        <div v-if="deck.ravage" class="card-cell active" :class="stageClass(deck.ravage.stage)">
          <span class="stage-badge" :class="stageClass(deck.ravage.stage)">Stage {{ stageLabel(deck.ravage.stage) }}</span>
          <div class="terrain-input-host">
            <input
              type="text"
              :value="deck.ravage.terrain ?? ''"
              placeholder="terrain…"
              @input="editCellTerrain('ravage', ($event.target as HTMLInputElement).value)"
              @focus="focusedField = 'ravage'"
              @blur="handleBlur"
            />
            <div v-if="focusedField === 'ravage'" class="terrain-dropdown">
              <button
                v-for="opt in filteredFor(deck.ravage.stage, deck.ravage.terrain)"
                :key="opt"
                type="button"
                class="terrain-opt"
                @mousedown.prevent
                @click="setCellTerrain('ravage', opt)"
              >{{ opt }}</button>
            </div>
          </div>
          <input
            type="text"
            :value="deck.ravage.notes ?? ''"
            placeholder="notes"
            class="notes"
            @input="editCellNotes('ravage', ($event.target as HTMLInputElement).value)"
          />
        </div>
        <div v-else class="card-cell empty">
          <span class="empty-label">empty</span>
        </div>
      </div>

      <div class="column build">
        <div class="col-label">Build</div>
        <div v-if="deck.build" class="card-cell active" :class="stageClass(deck.build.stage)">
          <span class="stage-badge" :class="stageClass(deck.build.stage)">Stage {{ stageLabel(deck.build.stage) }}</span>
          <div class="terrain-input-host">
            <input
              type="text"
              :value="deck.build.terrain ?? ''"
              placeholder="terrain…"
              @input="editCellTerrain('build', ($event.target as HTMLInputElement).value)"
              @focus="focusedField = 'build'"
              @blur="handleBlur"
            />
            <div v-if="focusedField === 'build'" class="terrain-dropdown">
              <button
                v-for="opt in filteredFor(deck.build.stage, deck.build.terrain)"
                :key="opt"
                type="button"
                class="terrain-opt"
                @mousedown.prevent
                @click="setCellTerrain('build', opt)"
              >{{ opt }}</button>
            </div>
          </div>
          <input
            type="text"
            :value="deck.build.notes ?? ''"
            placeholder="notes"
            class="notes"
            @input="editCellNotes('build', ($event.target as HTMLInputElement).value)"
          />
        </div>
        <div v-else class="card-cell empty">
          <span class="empty-label">empty</span>
        </div>
      </div>

      <div class="column explore">
        <div class="col-label">Explore</div>
        <div v-if="deck.explore" class="card-cell active" :class="stageClass(deck.explore.stage)">
          <span class="stage-badge" :class="stageClass(deck.explore.stage)">Stage {{ stageLabel(deck.explore.stage) }}</span>
          <div class="terrain-input-host">
            <input
              type="text"
              :value="deck.explore.terrain ?? ''"
              placeholder="terrain… (reveal)"
              autofocus
              @input="editCellTerrain('explore', ($event.target as HTMLInputElement).value)"
              @focus="focusedField = 'explore'"
              @blur="handleBlur"
            />
            <div v-if="focusedField === 'explore'" class="terrain-dropdown">
              <button
                v-for="opt in filteredFor(deck.explore.stage, deck.explore.terrain)"
                :key="opt"
                type="button"
                class="terrain-opt"
                @mousedown.prevent
                @click="setCellTerrain('explore', opt)"
              >{{ opt }}</button>
            </div>
          </div>
          <input
            type="text"
            :value="deck.explore.notes ?? ''"
            placeholder="notes"
            class="notes"
            @input="editCellNotes('explore', ($event.target as HTMLInputElement).value)"
          />
        </div>
        <div v-else class="card-cell empty">
          <span class="empty-label">empty</span>
        </div>
      </div>
    </div>

    <!-- Upcoming stack: stage-visible, terrain-blank -->
    <div v-if="deck.upcoming.length" class="upcoming-wrap">
      <div class="col-label upcoming-label">Upcoming stack · next → last</div>
      <div class="stack-groups">
        <div v-for="(grp, gi) in groupedUpcoming" :key="gi" class="stack-group" :class="stageClass(grp.stage)">
          <div class="group-header">
            <span class="stage-badge" :class="stageClass(grp.stage)">Stage {{ stageLabel(grp.stage) }}</span>
            <span class="group-count">×{{ grp.cards.length }}</span>
          </div>
          <div class="group-cards">
            <div v-for="(c, ci) in grp.cards" :key="ci" class="mini-card" :title="`position ${c.idx + 1} from top`">
              <span class="mini-stage" :class="stageClass(grp.stage)">{{ stageLabel(grp.stage) }}</span>
              <div class="mini-input-host">
                <input
                  type="text"
                  :value="c.card.terrain ?? ''"
                  placeholder="?"
                  class="mini-terrain"
                  @input="editUpcomingTerrain(c.idx, ($event.target as HTMLInputElement).value)"
                  @focus="focusedField = `upcoming:${c.idx}`"
                  @blur="handleBlur"
                />
                <div v-if="focusedField === `upcoming:${c.idx}`" class="terrain-dropdown mini">
                  <button
                    v-for="opt in filteredFor(grp.stage, c.card.terrain)"
                    :key="opt"
                    type="button"
                    class="terrain-opt"
                    @mousedown.prevent
                    @click="setUpcomingTerrain(c.idx, opt)"
                  >{{ opt }}</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Discarded (ravaged) cards -->
    <details v-if="deck.discarded.length" class="discarded">
      <summary>Discarded ({{ deck.discarded.length }}) — ravaged cards for retrospective analysis</summary>
      <ul>
        <li v-for="(c, i) in deck.discarded" :key="i">
          <span class="stage-badge" :class="stageClass(c.stage)">Stage {{ stageLabel(c.stage) }}</span>
          <span class="disc-terrain">{{ c.terrain || '(terrain unset)' }}</span>
          <span v-if="c.notes" class="disc-notes">— {{ c.notes }}</span>
        </li>
      </ul>
    </details>
  </div>
</template>

<style scoped>
.invader-deck { display: flex; flex-direction: column; gap: var(--sp-3); }

.hdr {
  display: flex; justify-content: space-between; align-items: center;
  flex-wrap: wrap; gap: var(--sp-3);
}
.hdr-left {
  display: inline-flex; align-items: center; gap: var(--sp-3); flex-wrap: wrap;
}
.hdr h3 { color: var(--text-white); margin: 0; }
.notation {
  font-family: var(--font-mono);
  font-size: var(--fs-sm);
  color: var(--accent-amber);
  padding: 2px var(--sp-2);
  background: var(--bg-muted);
  border-radius: var(--r-sm);
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
.count-chip.muted { color: var(--text-muted); }
.count-chip.stage-i   { background: var(--badge-approved-bg); color: var(--badge-approved-fg); border-color: rgba(82, 183, 136, 0.4); }
.count-chip.stage-ii  { background: var(--badge-warn-bg);     color: var(--badge-warn-fg);     border-color: rgba(233, 196, 106, 0.4); }
.count-chip.stage-iii { background: var(--badge-chosen-bg);   color: var(--badge-chosen-fg);   border-color: rgba(199, 125, 255, 0.4); }

.tools { display: inline-flex; gap: var(--sp-2); }

.setup-reveal {
  padding: var(--sp-3);
  background: var(--bg-inset);
  border: 1px dashed var(--border-subtle);
  border-radius: var(--r-md);
  display: flex; flex-direction: column; gap: var(--sp-2); align-items: flex-start;
}
.setup-reveal .hint { margin: 0; color: var(--text-secondary); font-size: var(--fs-sm); }
.rebuild-err {
  padding: var(--sp-2) var(--sp-3);
  font-size: var(--fs-xs);
  color: var(--status-danger);
  background: rgba(184, 113, 106, 0.1);
  border: 1px solid rgba(184, 113, 106, 0.3);
  border-radius: var(--r-sm);
}
.setup-reveal .seq { color: var(--accent-amber); font-family: var(--font-mono); font-size: var(--fs-xs); }
.setup-reveal strong.stage-i   { color: #52b788; }
.setup-reveal strong.stage-ii  { color: #e9c46a; }
.setup-reveal strong.stage-iii { color: #c77dff; }
.reveal-stage {
  font-family: var(--font-mono);
  padding: 1px 6px; border-radius: var(--r-sm);
  font-size: 0.72rem; font-weight: var(--fw-bold);
  margin-left: var(--sp-1);
}
.reveal-stage.stage-i   { background: rgba(82, 183, 136, 0.25); color: #52b788; }
.reveal-stage.stage-ii  { background: rgba(233, 196, 106, 0.25); color: #e9c46a; }
.reveal-stage.stage-iii { background: rgba(199, 125, 255, 0.25); color: #c77dff; }

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
.column.ravage  .col-label  { color: var(--accent-red); }
.column.build   .col-label  { color: var(--accent-amber); }
.column.explore .col-label  { color: var(--accent-blue); }

.card-cell {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-2) var(--sp-3);
  display: flex; flex-direction: column; gap: var(--sp-1);
  min-height: 5.5rem;
  position: relative;
  transition: border-color var(--motion-fast);
}
.card-cell.empty {
  background: transparent; border-style: dashed;
  align-items: center; justify-content: center;
}
.card-cell .empty-label {
  color: var(--text-muted); font-style: italic; font-size: var(--fs-xs);
}
.card-cell input[type="text"] { font-size: var(--fs-sm); width: 100%; }
.card-cell input.notes {
  font-size: var(--fs-xs); color: var(--text-secondary);
  background: transparent; border: none; padding: 0;
}

.terrain-input-host { position: relative; }
.terrain-dropdown {
  position: absolute;
  top: calc(100% + 2px);
  left: 0; right: 0;
  z-index: 20;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  box-shadow: 0 4px 14px rgba(0, 0, 0, 0.4);
  max-height: 14rem;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}
.terrain-dropdown.mini {
  min-width: 10rem;
  right: auto;
  width: max-content;
  max-width: 16rem;
}
.terrain-opt {
  display: block;
  width: 100%;
  text-align: left;
  padding: 4px var(--sp-2);
  border: none;
  background: transparent;
  color: inherit;
  cursor: pointer;
  font-size: var(--fs-xs);
  border-radius: 0;
}
.terrain-opt:hover { background: var(--bg-muted); color: var(--text-white); }

.column.ravage  .card-cell.active  { border-color: rgba(220, 47, 2, 0.5); }
.column.build   .card-cell.active  { border-color: rgba(233, 196, 106, 0.5); }
.column.explore .card-cell.active  { border-color: rgba(123, 184, 245, 0.5); box-shadow: 0 0 0 1px rgba(123, 184, 245, 0.2); }

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

.upcoming-wrap {
  border-top: 1px solid var(--border-subtle);
  padding-top: var(--sp-3);
  display: flex; flex-direction: column; gap: var(--sp-2);
}
.upcoming-label { color: var(--text-muted); }

.stack-groups {
  display: flex;
  gap: var(--sp-2);
  flex-wrap: wrap;
  align-items: flex-start;
}
.stack-group {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-2);
  display: flex; flex-direction: column; gap: var(--sp-1);
  min-width: 9rem;
}
.stack-group.stage-i { border-top: 3px solid #52b788; }
.stack-group.stage-ii { border-top: 3px solid #e9c46a; }
.stack-group.stage-iii { border-top: 3px solid #c77dff; }
.group-header {
  display: flex; justify-content: space-between; align-items: center;
}
.group-count {
  font-size: 0.7rem; color: var(--text-muted); font-family: var(--font-mono);
}
.group-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(4rem, 1fr));
  gap: 4px;
}
.mini-card {
  display: flex; align-items: center; gap: 4px;
  background: var(--bg-canvas);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  padding: 2px 4px;
  font-size: 0.7rem;
}
.mini-stage {
  font-family: var(--font-mono); font-size: 0.65rem;
  padding: 0 3px; border-radius: 2px;
  font-weight: var(--fw-bold);
}
.mini-stage.stage-i   { background: rgba(82, 183, 136, 0.2); color: #52b788; }
.mini-stage.stage-ii  { background: rgba(233, 196, 106, 0.2); color: #e9c46a; }
.mini-stage.stage-iii { background: rgba(199, 125, 255, 0.2); color: #c77dff; }
.mini-input-host { position: relative; flex: 1; min-width: 0; }
.mini-terrain {
  width: 100%;
  background: transparent; border: none; padding: 0;
  font-size: 0.7rem; color: var(--text-secondary);
}
.mini-terrain:focus { outline: 1px solid var(--accent-blue); background: var(--bg-muted); }

.discarded { font-size: var(--fs-xs); }
.discarded summary {
  list-style: none; cursor: pointer; color: var(--text-secondary);
  padding: var(--sp-1) 0;
}
.discarded summary::-webkit-details-marker { display: none; }
.discarded summary::before {
  content: '▸'; margin-right: var(--sp-2); color: var(--text-muted);
  display: inline-block;
  transition: transform var(--motion-fast);
}
.discarded[open] summary::before { transform: rotate(90deg); }
.discarded ul { list-style: none; padding: 0; margin: var(--sp-1) 0; display: flex; flex-direction: column; gap: 2px; }
.discarded li { display: inline-flex; gap: var(--sp-2); align-items: center; }
.disc-terrain { font-family: var(--font-mono); color: var(--text-primary); }
.disc-notes { color: var(--text-muted); font-style: italic; }
</style>
