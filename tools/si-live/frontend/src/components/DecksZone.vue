<script setup lang="ts">
import { computed, ref, onMounted } from 'vue'
import type { InvaderDeckState, FearDeckState, EventDeckState, Phase, InvaderCard, FearCardEntry } from '../types'
import { fetchDeck, fetchInvaderStack, type FearCardData, type EventCardData } from '../api'

/**
 * DecksZone — unified display for Invader, Fear, and Event decks.
 *
 * Each deck has two modes:
 *   - Compact (peripheral): 200px tall, shows key slots + counters only
 *   - Expanded (active phase): 600px+, full editing and detail
 *
 * The active deck is determined by the current game phase.
 */

const props = defineProps<{
  invaderDeck: InvaderDeckState | null | undefined
  fearDeck: FearDeckState | null | undefined
  eventDeck: EventDeckState | null | undefined
  phase: Phase
  terrorLevel: number
  round: number
  adversary?: string | null
  level?: number | null
  fearThreshold?: number
}>()

const emit = defineEmits<{
  'update:invaderDeck': [value: InvaderDeckState]
  'update:fearDeck': [value: FearDeckState]
  'update:eventDeck': [value: EventDeckState]
  'log-event': [event: string, details: Record<string, unknown>]
  'reset-fear-pool': []
}>()

// ─────────────────────────────────────────────────────────────────────────────
// Deck Data Fetching
// ─────────────────────────────────────────────────────────────────────────────

const fearCards = ref<FearCardData[]>([])
const eventCards = ref<EventCardData[]>([])
const loading = ref(true)

onMounted(async () => {
  try {
    const [fear, event] = await Promise.all([
      fetchDeck<FearCardData>('fear'),
      fetchDeck<EventCardData>('event'),
    ])
    fearCards.value = fear
    eventCards.value = event
  } catch (e) {
    console.error('Failed to load deck data:', e)
  } finally {
    loading.value = false
  }
})

const fearCardByName = computed(() => {
  const m = new Map<string, FearCardData>()
  for (const c of fearCards.value) m.set(c.name, c)
  return m
})

const eventCardByName = computed(() => {
  const m = new Map<string, EventCardData>()
  for (const c of eventCards.value) m.set(c.name, c)
  return m
})

// ─────────────────────────────────────────────────────────────────────────────
// Which deck is expanded based on phase
// ─────────────────────────────────────────────────────────────────────────────

type DeckType = 'invader' | 'fear' | 'event'

const activeDeck = computed<DeckType | null>(() => {
  switch (props.phase) {
    case 'invader':
      return 'invader'
    case 'fear':
      return 'fear'
    case 'event':
      return 'event'
    default:
      return null
  }
})

// Allow manual override
const forcedExpand = ref<DeckType | null>(null)

function isExpanded(deck: DeckType): boolean {
  if (forcedExpand.value) return forcedExpand.value === deck
  return activeDeck.value === deck
}

function toggleExpand(deck: DeckType) {
  forcedExpand.value = forcedExpand.value === deck ? null : deck
}

// ─────────────────────────────────────────────────────────────────────────────
// Invader Deck Helpers
// ─────────────────────────────────────────────────────────────────────────────

const invader = computed<InvaderDeckState>(() => props.invaderDeck ?? {
  ravage: null,
  build: null,
  explore: null,
  upcoming: [],
  discarded: [],
  notation: '3 · 4 · 5',
})

function stageLabel(stage: number | undefined): string {
  return stage === 1 ? 'I' : stage === 2 ? 'II' : 'III'
}

function stageClass(stage: number | undefined): string {
  return stage === 1 ? 'stage-i' : stage === 2 ? 'stage-ii' : 'stage-iii'
}

function terrainOptionsForStage(stage: number | undefined): string[] {
  if (stage === 1) return ['Mountain', 'Jungle', 'Sands', 'Wetland']
  if (stage === 2) return ['Mountain', 'Jungle', 'Sands', 'Wetland', 'Coastal Lands']
  if (stage === 3) return [
    'Jungle + Mountain', 'Jungle + Sands', 'Jungle + Wetland',
    'Mountain + Sands', 'Mountain + Wetland', 'Sands + Wetland',
  ]
  return []
}

const invaderFocusedField = ref<string | null>(null)

function filteredTerrainFor(stage: number | undefined, currentValue: string | null | undefined): string[] {
  const all = terrainOptionsForStage(stage)
  const q = (currentValue ?? '').trim().toLowerCase()
  if (!q) return all
  return all.filter(s => s.toLowerCase().includes(q))
}

function updateInvaderDeck(next: InvaderDeckState) {
  emit('update:invaderDeck', next)
}

function editCellTerrain(field: 'ravage' | 'build' | 'explore', value: string) {
  const d = invader.value
  const card = d[field]
  if (!card) return
  updateInvaderDeck({ ...d, [field]: { ...card, terrain: value } })
}

function setCellTerrain(field: 'ravage' | 'build' | 'explore', value: string) {
  editCellTerrain(field, value)
  invaderFocusedField.value = null
}

function advanceInvaderDeck() {
  const d = invader.value
  const ravaged = d.ravage
  const newRavage = d.build
  const newBuild = d.explore
  const newExplore = d.upcoming.length ? { ...d.upcoming[0], terrain: d.upcoming[0].terrain ?? '' } : null
  const rest = d.upcoming.slice(1)
  const newDiscarded = ravaged ? [...d.discarded, ravaged] : d.discarded
  updateInvaderDeck({
    ...d,
    ravage: newRavage,
    build: newBuild,
    explore: newExplore,
    upcoming: rest,
    discarded: newDiscarded,
  })
}

function revealFirstCard() {
  const d = invader.value
  if (!d.upcoming.length) return
  const [next, ...rest] = d.upcoming
  updateInvaderDeck({
    ...d,
    explore: { ...next, terrain: next.terrain ?? '' },
    upcoming: rest,
  })
}

const showSetupReveal = computed(
  () => !invader.value.explore && !invader.value.build && !invader.value.ravage && invader.value.upcoming.length > 0,
)

// ─────────────────────────────────────────────────────────────────────────────
// Fear Deck Helpers
// ─────────────────────────────────────────────────────────────────────────────

const fear = computed<FearDeckState>(() => props.fearDeck ?? {
  deck_size: 9,
  tier_counts: [3, 3, 3],
  earned: [],
  resolved: [],
  unseen: 9,
})

function effectiveTierCounts(d: FearDeckState): [number, number, number] {
  if (d.tier_counts && d.tier_counts.length === 3) return d.tier_counts
  const each = Math.ceil(d.deck_size / 3)
  const rem = Math.max(0, d.deck_size - 2 * each)
  return [each, each, rem]
}

interface FearSlot {
  idx: number
  terrorLevel: 1 | 2 | 3
  state: 'unseen' | 'earned' | 'resolved'
  name?: string
}

const fearSlots = computed<FearSlot[]>(() => {
  const d = fear.value
  const [t1, t2, t3] = effectiveTierCounts(d)
  const slotList: FearSlot[] = []
  for (let i = 0; i < t1; i++) slotList.push({ idx: slotList.length, terrorLevel: 1, state: 'unseen' })
  for (let i = 0; i < t2; i++) slotList.push({ idx: slotList.length, terrorLevel: 2, state: 'unseen' })
  for (let i = 0; i < t3; i++) slotList.push({ idx: slotList.length, terrorLevel: 3, state: 'unseen' })
  let cursor = 0
  for (const r of d.resolved) {
    if (cursor < slotList.length) {
      slotList[cursor].state = 'resolved'
      slotList[cursor].name = r.name
      slotList[cursor].terrorLevel = r.terror_level as 1 | 2 | 3
      cursor++
    }
  }
  for (const e of d.earned) {
    if (cursor < slotList.length) {
      slotList[cursor].state = 'earned'
      slotList[cursor].name = e.name
      slotList[cursor].terrorLevel = e.terror_level as 1 | 2 | 3
      cursor++
    }
  }
  return slotList
})

function updateFearDeck(next: FearDeckState) {
  emit('update:fearDeck', next)
}

function bankFearCard() {
  const entry: FearCardEntry = {
    name: '',
    terror_level: props.terrorLevel,
    round: props.round,
  }
  updateFearDeck({
    ...fear.value,
    earned: [...fear.value.earned, entry],
    unseen: Math.max(0, fear.value.unseen - 1),
  })
  emit('log-event', 'fear_card_earned', { terror_level: props.terrorLevel, round: props.round })
  emit('reset-fear-pool')
}

const fearNamingDraft = ref<Record<number, string>>({})
const fearFilterQ = ref('')

const fearSuggestions = computed(() => {
  const q = fearFilterQ.value.trim().toLowerCase()
  if (!q) return fearCards.value.slice(0, 6)
  return fearCards.value.filter(c => c.name.toLowerCase().includes(q)).slice(0, 8)
})

function resolveFearCard(i: number, providedName?: string) {
  const d = fear.value
  const entry = d.earned[i]
  if (!entry) return
  const name = (providedName ?? entry.name ?? fearNamingDraft.value[i] ?? '').trim()
  if (!entry.name && !name) return
  const remaining = [...d.earned]
  remaining.splice(i, 1)
  updateFearDeck({
    ...d,
    earned: remaining,
    resolved: [...d.resolved, { ...entry, name: name || entry.name, terror_level: props.terrorLevel }],
  })
  const next = { ...fearNamingDraft.value }
  delete next[i]
  fearNamingDraft.value = next
}

// ─────────────────────────────────────────────────────────────────────────────
// Event Deck Helpers
// ─────────────────────────────────────────────────────────────────────────────

const event = computed<EventDeckState>(() => props.eventDeck ?? {
  previewed: [],
  resolved: [],
  unseen: 62,
})

function updateEventDeck(next: EventDeckState) {
  emit('update:eventDeck', next)
}

const eventFilterQ = ref('')

const eventSuggestions = computed(() => {
  const q = eventFilterQ.value.trim().toLowerCase()
  if (!q) return eventCards.value.slice(0, 8)
  return eventCards.value.filter(c => c.name.toLowerCase().includes(q)).slice(0, 10)
})

function previewEventCard(name: string) {
  if (!name.trim()) return
  updateEventDeck({
    ...event.value,
    previewed: [...event.value.previewed, {
      name: name.trim(),
      previewed_on_turn: props.round,
      resolved_on_turn: null,
    }],
    unseen: Math.max(0, event.value.unseen - 1),
  })
  eventFilterQ.value = ''
}

function resolvePreviewedEvent(i: number) {
  const d = event.value
  const entry = d.previewed[i]
  if (!entry) return
  const remaining = [...d.previewed]
  remaining.splice(i, 1)
  updateEventDeck({
    ...d,
    previewed: remaining,
    resolved: [...d.resolved, { ...entry, resolved_on_turn: props.round }],
  })
}

function removePreviewedEvent(i: number) {
  const d = event.value
  const remaining = [...d.previewed]
  remaining.splice(i, 1)
  updateEventDeck({
    ...d,
    previewed: remaining,
    unseen: d.unseen + 1,
  })
}
</script>

<template>
  <div class="decks-zone">
    <!-- ═══════════════════════════════════════════════════════════════════ -->
    <!-- INVADER DECK                                                        -->
    <!-- ═══════════════════════════════════════════════════════════════════ -->
    <div
      class="deck-panel invader"
      :class="{ expanded: isExpanded('invader'), compact: !isExpanded('invader') }"
    >
      <div class="deck-header" @click="toggleExpand('invader')">
        <div class="deck-title">
          <span class="deck-icon invader-icon"></span>
          <span class="deck-name">Invader</span>
          <span class="deck-notation">{{ invader.notation ?? '3 · 4 · 5' }}</span>
        </div>
        <div class="deck-counts">
          <span class="count-chip" :class="stageClass(1)">{{ invader.upcoming.filter(c => c.stage === 1).length + (invader.ravage?.stage === 1 ? 1 : 0) + (invader.build?.stage === 1 ? 1 : 0) + (invader.explore?.stage === 1 ? 1 : 0) }} I</span>
          <span class="count-chip" :class="stageClass(2)">{{ invader.upcoming.filter(c => c.stage === 2).length + (invader.ravage?.stage === 2 ? 1 : 0) + (invader.build?.stage === 2 ? 1 : 0) + (invader.explore?.stage === 2 ? 1 : 0) }} II</span>
          <span class="count-chip" :class="stageClass(3)">{{ invader.upcoming.filter(c => c.stage === 3).length + (invader.ravage?.stage === 3 ? 1 : 0) + (invader.build?.stage === 3 ? 1 : 0) + (invader.explore?.stage === 3 ? 1 : 0) }} III</span>
        </div>
        <button class="expand-toggle" :title="isExpanded('invader') ? 'Collapse' : 'Expand'">
          <span class="chevron" :class="{ rotated: isExpanded('invader') }"></span>
        </button>
      </div>

      <!-- COMPACT: horizontal slots -->
      <div v-if="!isExpanded('invader')" class="compact-content invader-compact">
        <div class="slot-row">
          <div class="slot ravage" :class="invader.ravage ? stageClass(invader.ravage.stage) : ''">
            <span class="slot-label">Ravage</span>
            <div v-if="invader.ravage" class="slot-card">
              <span class="slot-stage">{{ stageLabel(invader.ravage.stage) }}</span>
              <span class="slot-terrain">{{ invader.ravage.terrain || '?' }}</span>
            </div>
            <span v-else class="slot-empty">-</span>
          </div>
          <div class="slot build" :class="invader.build ? stageClass(invader.build.stage) : ''">
            <span class="slot-label">Build</span>
            <div v-if="invader.build" class="slot-card">
              <span class="slot-stage">{{ stageLabel(invader.build.stage) }}</span>
              <span class="slot-terrain">{{ invader.build.terrain || '?' }}</span>
            </div>
            <span v-else class="slot-empty">-</span>
          </div>
          <div class="slot explore" :class="invader.explore ? stageClass(invader.explore.stage) : ''">
            <span class="slot-label">Explore</span>
            <div v-if="invader.explore" class="slot-card">
              <span class="slot-stage">{{ stageLabel(invader.explore.stage) }}</span>
              <span class="slot-terrain">{{ invader.explore.terrain || '?' }}</span>
            </div>
            <span v-else class="slot-empty">-</span>
          </div>
        </div>
        <div class="compact-actions">
          <button class="action-btn" @click.stop="advanceInvaderDeck" :disabled="!invader.upcoming.length && !invader.explore">
            Advance
          </button>
        </div>
      </div>

      <!-- EXPANDED: full editing -->
      <div v-else class="expanded-content invader-expanded">
        <div v-if="showSetupReveal" class="setup-reveal">
          <p>Stack ready: <strong>{{ invader.notation }}</strong> ({{ invader.upcoming.length }} cards). Reveal the first card:</p>
          <button class="primary" @click="revealFirstCard">
            Reveal top card (Stage {{ stageLabel(invader.upcoming[0]?.stage) }})
          </button>
        </div>

        <div class="invader-columns">
          <div
            v-for="field in (['ravage', 'build', 'explore'] as const)"
            :key="field"
            class="invader-column"
            :class="[field, invader[field] ? stageClass(invader[field]!.stage) : '']"
          >
            <div class="column-label">{{ field.charAt(0).toUpperCase() + field.slice(1) }}</div>
            <div v-if="invader[field]" class="card-cell">
              <span class="stage-badge" :class="stageClass(invader[field]!.stage)">
                Stage {{ stageLabel(invader[field]!.stage) }}
              </span>
              <div class="terrain-input-wrap">
                <input
                  type="text"
                  :value="invader[field]!.terrain ?? ''"
                  placeholder="terrain..."
                  @input="editCellTerrain(field, ($event.target as HTMLInputElement).value)"
                  @focus="invaderFocusedField = field"
                  @blur="setTimeout(() => invaderFocusedField = null, 150)"
                />
                <div v-if="invaderFocusedField === field" class="terrain-dropdown">
                  <button
                    v-for="opt in filteredTerrainFor(invader[field]!.stage, invader[field]!.terrain)"
                    :key="opt"
                    type="button"
                    class="terrain-opt"
                    @mousedown.prevent
                    @click="setCellTerrain(field, opt)"
                  >{{ opt }}</button>
                </div>
              </div>
            </div>
            <div v-else class="card-cell empty">
              <span class="empty-label">empty</span>
            </div>
          </div>
        </div>

        <div class="invader-actions">
          <button class="primary" @click="advanceInvaderDeck" :disabled="!invader.upcoming.length && !invader.explore">
            Advance deck
          </button>
          <span class="upcoming-count">{{ invader.upcoming.length }} upcoming</span>
          <span class="discarded-count">{{ invader.discarded.length }} discarded</span>
        </div>

        <div v-if="invader.upcoming.length" class="upcoming-stack">
          <span class="stack-label">Upcoming:</span>
          <div class="stack-pills">
            <span
              v-for="(c, i) in invader.upcoming.slice(0, 8)"
              :key="i"
              class="stack-pill"
              :class="stageClass(c.stage)"
              :title="`Position ${i + 1}: Stage ${stageLabel(c.stage)}${c.terrain ? ' · ' + c.terrain : ''}`"
            >{{ stageLabel(c.stage) }}</span>
            <span v-if="invader.upcoming.length > 8" class="stack-more">+{{ invader.upcoming.length - 8 }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- ═══════════════════════════════════════════════════════════════════ -->
    <!-- FEAR DECK                                                           -->
    <!-- ═══════════════════════════════════════════════════════════════════ -->
    <div
      class="deck-panel fear"
      :class="{ expanded: isExpanded('fear'), compact: !isExpanded('fear') }"
    >
      <div class="deck-header" @click="toggleExpand('fear')">
        <div class="deck-title">
          <span class="deck-icon fear-icon"></span>
          <span class="deck-name">Fear</span>
          <span class="terror-badge" :data-level="terrorLevel">T{{ terrorLevel }}</span>
        </div>
        <div class="deck-counts">
          <span class="count-chip earned">{{ fear.earned.length }} earned</span>
          <span class="count-chip resolved">{{ fear.resolved.length }} resolved</span>
        </div>
        <button class="expand-toggle" :title="isExpanded('fear') ? 'Collapse' : 'Expand'">
          <span class="chevron" :class="{ rotated: isExpanded('fear') }"></span>
        </button>
      </div>

      <!-- COMPACT: slot visualization -->
      <div v-if="!isExpanded('fear')" class="compact-content fear-compact">
        <div class="fear-slots-compact">
          <div
            v-for="(slot, i) in fearSlots"
            :key="i"
            class="fear-slot"
            :class="[`t${slot.terrorLevel}`, slot.state]"
            :title="slot.name || `T${slot.terrorLevel} · ${slot.state}`"
          >
            <span v-if="slot.state === 'resolved'" class="slot-mark">&#10003;</span>
            <span v-else-if="slot.state === 'earned'" class="slot-mark">&#9679;</span>
            <span v-else class="slot-mark">&#9675;</span>
          </div>
        </div>
        <div class="compact-actions">
          <button class="action-btn" @click.stop="bankFearCard">
            Bank card
          </button>
        </div>
      </div>

      <!-- EXPANDED: full editing -->
      <div v-else class="expanded-content fear-expanded">
        <div class="fear-slots-grid">
          <div class="tier-row" v-for="tier in [1, 2, 3]" :key="tier">
            <span class="tier-label" :class="`t${tier}`">T{{ tier }}</span>
            <div class="tier-slots">
              <div
                v-for="(slot, i) in fearSlots.filter(s => s.terrorLevel === tier)"
                :key="i"
                class="fear-slot-lg"
                :class="[slot.state]"
                :title="slot.name || slot.state"
              >
                <span v-if="slot.state === 'resolved'" class="check">&#10003;</span>
                <span v-else-if="slot.state === 'earned'" class="dot">&#9679;</span>
                <span v-else class="empty">&#9675;</span>
              </div>
            </div>
          </div>
        </div>

        <div class="fear-bank-row">
          <button class="primary" @click="bankFearCard">
            Bank 1 face-down
          </button>
          <span class="bank-hint">Threshold crossed — record a face-down card</span>
        </div>

        <div v-if="fear.earned.length" class="earned-list">
          <div class="list-header">Earned (awaiting resolution)</div>
          <div
            v-for="(e, i) in fear.earned"
            :key="i"
            class="earned-item"
            :class="`t${e.terror_level}`"
          >
            <span class="item-tl">T{{ e.terror_level }}</span>
            <span v-if="e.name" class="item-name">{{ e.name }}</span>
            <span v-else class="item-name facedown">(face-down)</span>
            <span class="item-round">r{{ e.round }}</span>
            <div v-if="!e.name" class="name-input">
              <input
                type="text"
                :value="fearNamingDraft[i] ?? ''"
                placeholder="name when resolving..."
                @input="fearNamingDraft[i] = ($event.target as HTMLInputElement).value"
                @keyup.enter="resolveFearCard(i)"
              />
            </div>
            <button class="resolve-btn" @click="resolveFearCard(i)" :disabled="!e.name && !fearNamingDraft[i]?.trim()">
              Resolve
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- ═══════════════════════════════════════════════════════════════════ -->
    <!-- EVENT DECK                                                          -->
    <!-- ═══════════════════════════════════════════════════════════════════ -->
    <div
      class="deck-panel event"
      :class="{ expanded: isExpanded('event'), compact: !isExpanded('event') }"
    >
      <div class="deck-header" @click="toggleExpand('event')">
        <div class="deck-title">
          <span class="deck-icon event-icon"></span>
          <span class="deck-name">Event</span>
        </div>
        <div class="deck-counts">
          <span class="count-chip previewed">{{ event.previewed.length }} previewed</span>
          <span class="count-chip resolved">{{ event.resolved.length }} resolved</span>
        </div>
        <button class="expand-toggle" :title="isExpanded('event') ? 'Collapse' : 'Expand'">
          <span class="chevron" :class="{ rotated: isExpanded('event') }"></span>
        </button>
      </div>

      <!-- COMPACT: preview card -->
      <div v-if="!isExpanded('event')" class="compact-content event-compact">
        <div v-if="event.previewed.length" class="event-preview-compact">
          <div class="preview-card-mini">
            <span class="preview-name">{{ event.previewed[0].name }}</span>
            <span class="preview-turn">T{{ event.previewed[0].previewed_on_turn + 1 }}</span>
          </div>
          <span class="resolves-label">resolves next turn</span>
        </div>
        <div v-else class="no-preview">
          <span>No event previewed</span>
        </div>
      </div>

      <!-- EXPANDED: full editing -->
      <div v-else class="expanded-content event-expanded">
        <div class="event-rule-hint">
          T1 previews T2 - first event doesn&apos;t resolve on T1
        </div>

        <div class="event-input-row">
          <div class="autocomplete">
            <input
              type="text"
              v-model="eventFilterQ"
              placeholder="Type event card name..."
              @keyup.enter="previewEventCard(eventFilterQ)"
            />
            <div v-if="eventFilterQ && eventSuggestions.length" class="suggestions">
              <button
                v-for="c in eventSuggestions"
                :key="c.name"
                type="button"
                class="suggestion"
                @click="previewEventCard(c.name)"
              >
                <span class="sug-name">{{ c.name }}</span>
                <span class="sug-expansion">{{ c.expansion }}</span>
              </button>
            </div>
          </div>
          <button class="primary" @click="previewEventCard(eventFilterQ)" :disabled="!eventFilterQ.trim()">
            Preview
          </button>
        </div>

        <div v-if="event.previewed.length" class="previewed-list">
          <div class="list-header">Previewed (resolves Turn {{ round + 1 }})</div>
          <div
            v-for="(e, i) in event.previewed"
            :key="i"
            class="previewed-item"
          >
            <span class="item-name">{{ e.name }}</span>
            <span class="item-turn">T{{ e.previewed_on_turn }}</span>
            <button class="resolve-btn" @click="resolvePreviewedEvent(i)">Resolve</button>
            <button class="ghost-btn" @click="removePreviewedEvent(i)">Remove</button>
          </div>
        </div>

        <details v-if="event.resolved.length" class="resolved-section">
          <summary>Resolved ({{ event.resolved.length }})</summary>
          <ul>
            <li v-for="(r, i) in event.resolved" :key="i">
              {{ r.name }} (T{{ r.previewed_on_turn }} → T{{ r.resolved_on_turn }})
            </li>
          </ul>
        </details>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* ═══════════════════════════════════════════════════════════════════════════ */
/* DECKS ZONE — COMPACT / EXPANDED MODES                                       */
/* ═══════════════════════════════════════════════════════════════════════════ */

.decks-zone {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
  height: 100%;
  overflow-y: auto;
}

/* ─── DECK PANEL ─── */
.deck-panel {
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  transition: all var(--duration-base) var(--ease);
  overflow: hidden;
}

.deck-panel.compact {
  min-height: 140px;
  max-height: 200px;
}

.deck-panel.expanded {
  min-height: 400px;
  flex: 1;
}

/* Phase-active highlighting */
.deck-panel.invader { border-left: 3px solid var(--color-danger); }
.deck-panel.fear { border-left: 3px solid var(--color-fear); }
.deck-panel.event { border-left: 3px solid var(--color-blight); }

.deck-panel.expanded.invader { background: rgba(239, 68, 68, 0.03); }
.deck-panel.expanded.fear { background: rgba(245, 158, 11, 0.03); }
.deck-panel.expanded.event { background: rgba(168, 85, 247, 0.03); }

/* ─── DECK HEADER ─── */
.deck-header {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  padding: var(--sp-3) var(--sp-4);
  cursor: pointer;
  border-bottom: 1px solid var(--border-subtle);
  background: var(--bg-elevated);
}

.deck-header:hover {
  background: var(--bg-hover);
}

.deck-title {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  flex: 1;
}

.deck-icon {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.invader-icon { background: var(--color-danger); }
.fear-icon { background: var(--color-fear); }
.event-icon { background: var(--color-blight); }

.deck-name {
  font-weight: var(--weight-semibold);
  font-size: var(--text-sm);
  color: var(--text-primary);
}

.deck-notation {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-muted);
  padding: 2px var(--sp-2);
  background: var(--bg-muted);
  border-radius: var(--radius-sm);
}

.terror-badge {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  font-weight: var(--weight-bold);
  padding: 2px var(--sp-2);
  border-radius: var(--radius-sm);
  background: var(--bg-muted);
}

.terror-badge[data-level="1"] { color: var(--color-accent); }
.terror-badge[data-level="2"] { color: var(--color-warning); }
.terror-badge[data-level="3"] { color: var(--color-danger); }

.deck-counts {
  display: flex;
  gap: 4px;
}

.count-chip {
  font-size: var(--text-xs);
  font-family: var(--font-mono);
  padding: 2px 6px;
  border-radius: 10px;
  background: var(--bg-muted);
  color: var(--text-secondary);
}

.count-chip.stage-i { color: var(--color-stage-1); background: rgba(34, 197, 94, 0.1); }
.count-chip.stage-ii { color: var(--color-stage-2); background: rgba(234, 179, 8, 0.1); }
.count-chip.stage-iii { color: var(--color-stage-3); background: rgba(239, 68, 68, 0.1); }
.count-chip.earned { color: var(--color-fear); background: rgba(245, 158, 11, 0.1); }
.count-chip.resolved { color: var(--color-success); background: rgba(34, 197, 94, 0.1); }
.count-chip.previewed { color: var(--color-blight); background: rgba(168, 85, 247, 0.1); }

.expand-toggle {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
}

.chevron {
  width: 0;
  height: 0;
  border-left: 5px solid transparent;
  border-right: 5px solid transparent;
  border-top: 6px solid currentColor;
  transition: transform var(--duration-base) var(--ease);
}

.chevron.rotated {
  transform: rotate(180deg);
}

/* ─── COMPACT CONTENT ─── */
.compact-content {
  padding: var(--sp-3) var(--sp-4);
}

.compact-actions {
  margin-top: var(--sp-2);
}

.action-btn {
  font-size: var(--text-xs);
  padding: var(--sp-1) var(--sp-3);
  background: var(--bg-muted);
  border: 1px solid var(--border-default);
  color: var(--text-secondary);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.action-btn:hover:not(:disabled) {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* ─── INVADER COMPACT ─── */
.slot-row {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--sp-2);
}

.slot {
  padding: var(--sp-2);
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  text-align: center;
}

.slot.ravage { border-top: 3px solid var(--color-danger); }
.slot.build { border-top: 3px solid var(--color-warning); }
.slot.explore { border-top: 3px solid var(--color-accent); }

.slot-label {
  display: block;
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  margin-bottom: 4px;
}

.slot-card {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.slot-stage {
  font-family: var(--font-mono);
  font-weight: var(--weight-bold);
  font-size: var(--text-sm);
}

.slot.stage-i .slot-stage { color: var(--color-stage-1); }
.slot.stage-ii .slot-stage { color: var(--color-stage-2); }
.slot.stage-iii .slot-stage { color: var(--color-stage-3); }

.slot-terrain {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.slot-empty {
  color: var(--text-faint);
  font-style: italic;
}

/* ─── INVADER EXPANDED ─── */
.expanded-content {
  padding: var(--sp-4);
  overflow-y: auto;
}

.setup-reveal {
  padding: var(--sp-4);
  background: var(--bg-elevated);
  border: 1px dashed var(--border-default);
  border-radius: var(--radius-md);
  margin-bottom: var(--sp-4);
  text-align: center;
}

.setup-reveal p {
  margin: 0 0 var(--sp-3);
  font-size: var(--text-sm);
  color: var(--text-secondary);
}

.invader-columns {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--sp-3);
  margin-bottom: var(--sp-4);
}

.invader-column {
  background: var(--bg-muted);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  padding: var(--sp-3);
}

.invader-column.ravage { border-top: 3px solid var(--color-danger); }
.invader-column.build { border-top: 3px solid var(--color-warning); }
.invader-column.explore { border-top: 3px solid var(--color-accent); }

.column-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  margin-bottom: var(--sp-2);
}

.card-cell {
  min-height: 60px;
}

.card-cell.empty {
  display: flex;
  align-items: center;
  justify-content: center;
}

.empty-label {
  color: var(--text-faint);
  font-style: italic;
  font-size: var(--text-xs);
}

.stage-badge {
  display: inline-block;
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  font-weight: var(--weight-bold);
  padding: 2px var(--sp-2);
  border-radius: var(--radius-sm);
  margin-bottom: var(--sp-2);
}

.stage-badge.stage-i { color: var(--color-stage-1); background: rgba(34, 197, 94, 0.15); }
.stage-badge.stage-ii { color: var(--color-stage-2); background: rgba(234, 179, 8, 0.15); }
.stage-badge.stage-iii { color: var(--color-stage-3); background: rgba(239, 68, 68, 0.15); }

.terrain-input-wrap {
  position: relative;
}

.terrain-input-wrap input {
  width: 100%;
  font-size: var(--text-sm);
}

.terrain-dropdown {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  z-index: 20;
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  max-height: 180px;
  overflow-y: auto;
}

.terrain-opt {
  width: 100%;
  padding: var(--sp-2) var(--sp-3);
  text-align: left;
  border: none;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  font-size: var(--text-sm);
  height: auto;
  border-radius: 0;
}

.terrain-opt:hover {
  background: var(--bg-hover);
}

.invader-actions {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  margin-bottom: var(--sp-3);
}

.upcoming-count,
.discarded-count {
  font-size: var(--text-xs);
  color: var(--text-muted);
  font-family: var(--font-mono);
}

.upcoming-stack {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-2);
  background: var(--bg-muted);
  border-radius: var(--radius-sm);
}

.stack-label {
  font-size: var(--text-xs);
  color: var(--text-muted);
  font-weight: var(--weight-medium);
}

.stack-pills {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}

.stack-pill {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  font-weight: var(--weight-bold);
  border-radius: var(--radius-sm);
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
}

.stack-pill.stage-i { color: var(--color-stage-1); border-color: rgba(34, 197, 94, 0.3); }
.stack-pill.stage-ii { color: var(--color-stage-2); border-color: rgba(234, 179, 8, 0.3); }
.stack-pill.stage-iii { color: var(--color-stage-3); border-color: rgba(239, 68, 68, 0.3); }

.stack-more {
  font-size: var(--text-xs);
  color: var(--text-muted);
  font-family: var(--font-mono);
}

/* ─── FEAR COMPACT ─── */
.fear-slots-compact {
  display: grid;
  grid-template-columns: repeat(9, 1fr);
  gap: 4px;
}

@media (max-width: 400px) {
  .fear-slots-compact {
    grid-template-columns: repeat(3, 1fr);
  }
}

.fear-slot {
  aspect-ratio: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
}

.fear-slot.t1 { border-top: 2px solid var(--color-accent); }
.fear-slot.t2 { border-top: 2px solid var(--color-warning); }
.fear-slot.t3 { border-top: 2px solid var(--color-danger); }

.fear-slot.earned { background: rgba(245, 158, 11, 0.15); }
.fear-slot.resolved { background: rgba(34, 197, 94, 0.15); opacity: 0.7; }

.slot-mark {
  color: var(--text-muted);
}

.fear-slot.earned .slot-mark { color: var(--color-fear); }
.fear-slot.resolved .slot-mark { color: var(--color-success); }

/* ─── FEAR EXPANDED ─── */
.fear-slots-grid {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
  margin-bottom: var(--sp-4);
}

.tier-row {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.tier-label {
  width: 28px;
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  font-weight: var(--weight-bold);
}

.tier-label.t1 { color: var(--color-accent); }
.tier-label.t2 { color: var(--color-warning); }
.tier-label.t3 { color: var(--color-danger); }

.tier-slots {
  display: flex;
  gap: 6px;
}

.fear-slot-lg {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-muted);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
}

.fear-slot-lg.earned { background: rgba(245, 158, 11, 0.15); }
.fear-slot-lg.resolved { background: rgba(34, 197, 94, 0.15); opacity: 0.7; }

.fear-slot-lg .check { color: var(--color-success); }
.fear-slot-lg .dot { color: var(--color-fear); }
.fear-slot-lg .empty { color: var(--text-faint); }

.fear-bank-row {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  padding: var(--sp-3);
  background: rgba(245, 158, 11, 0.08);
  border: 1px solid rgba(245, 158, 11, 0.2);
  border-radius: var(--radius-md);
  margin-bottom: var(--sp-4);
}

.bank-hint {
  font-size: var(--text-xs);
  color: var(--text-muted);
}

.earned-list,
.previewed-list {
  margin-top: var(--sp-3);
}

.list-header {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  margin-bottom: var(--sp-2);
}

.earned-item,
.previewed-item {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-2) var(--sp-3);
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  margin-bottom: var(--sp-2);
  flex-wrap: wrap;
}

.item-tl {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  font-weight: var(--weight-bold);
  width: 24px;
}

.earned-item.t1 .item-tl { color: var(--color-accent); }
.earned-item.t2 .item-tl { color: var(--color-warning); }
.earned-item.t3 .item-tl { color: var(--color-danger); }

.item-name {
  flex: 1;
  font-size: var(--text-sm);
  color: var(--text-primary);
}

.item-name.facedown {
  color: var(--text-muted);
  font-style: italic;
}

.item-round,
.item-turn {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-muted);
}

.name-input {
  width: 100%;
  margin-top: var(--sp-2);
}

.name-input input {
  width: 100%;
  font-size: var(--text-sm);
}

.resolve-btn {
  font-size: var(--text-xs);
  padding: var(--sp-1) var(--sp-2);
  background: var(--color-accent);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.resolve-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.ghost-btn {
  font-size: var(--text-xs);
  padding: var(--sp-1) var(--sp-2);
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
}

.ghost-btn:hover {
  color: var(--text-secondary);
}

/* ─── EVENT COMPACT ─── */
.event-preview-compact {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.preview-card-mini {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--sp-2) var(--sp-3);
  background: rgba(168, 85, 247, 0.1);
  border: 1px solid rgba(168, 85, 247, 0.25);
  border-radius: var(--radius-md);
}

.preview-name {
  font-weight: var(--weight-semibold);
  font-size: var(--text-sm);
  color: var(--text-primary);
}

.preview-turn {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-muted);
}

.resolves-label {
  font-size: var(--text-xs);
  color: var(--color-blight);
  font-style: italic;
}

.no-preview {
  color: var(--text-muted);
  font-size: var(--text-sm);
  font-style: italic;
}

/* ─── EVENT EXPANDED ─── */
.event-rule-hint {
  font-size: var(--text-xs);
  color: var(--color-blight);
  font-style: italic;
  padding: var(--sp-2) var(--sp-3);
  background: rgba(168, 85, 247, 0.08);
  border-radius: var(--radius-sm);
  margin-bottom: var(--sp-3);
}

.event-input-row {
  display: flex;
  gap: var(--sp-2);
  margin-bottom: var(--sp-4);
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
  font-size: var(--text-sm);
  height: auto;
  border-radius: 0;
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

.resolved-section summary {
  cursor: pointer;
  color: var(--text-secondary);
  font-size: var(--text-sm);
  padding: var(--sp-2) 0;
  list-style: none;
}

.resolved-section summary::-webkit-details-marker {
  display: none;
}

.resolved-section summary::before {
  content: '>';
  margin-right: var(--sp-2);
  color: var(--text-muted);
  font-family: var(--font-mono);
  display: inline-block;
  transition: transform var(--duration-base) var(--ease);
}

.resolved-section[open] summary::before {
  transform: rotate(90deg);
}

.resolved-section ul {
  list-style: none;
  padding: 0;
  margin: var(--sp-2) 0 0;
}

.resolved-section li {
  font-size: var(--text-xs);
  color: var(--text-muted);
  padding: var(--sp-1) 0;
}

/* ─── SHARED BUTTON STYLES ─── */
.primary {
  background: var(--color-accent);
  color: white;
  border: none;
  padding: var(--sp-2) var(--sp-4);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  cursor: pointer;
}

.primary:hover:not(:disabled) {
  opacity: 0.9;
}

.primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
