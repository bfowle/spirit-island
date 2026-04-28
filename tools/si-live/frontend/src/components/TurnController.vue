<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import type { GameState, Phase } from '../types'
import Icon from './Icon.vue'
import ExpansionBadge from './ExpansionBadge.vue'
import { fetchDeck, type FearCardData, type EventCardData } from '../api'

const props = defineProps<{ modelValue: GameState }>()
defineEmits<{ 'update:modelValue': [value: GameState] }>()

// Take a structural snapshot of key metrics — used for phase-delta tracking
function buildSnapshot(state: GameState) {
  const spiritTotals: Record<string, { energy: number; card_plays: number; hand: number; played: number; discard: number; presence_on_board: number }> = {}
  for (const [slug, sp] of Object.entries(state.spirits ?? {})) {
    const presenceTotal = Object.values(sp.presence_on_board ?? {}).reduce((a, b) => a + (b ?? 0), 0)
    spiritTotals[slug] = {
      energy: sp.energy ?? 0,
      card_plays: sp.card_plays ?? 0,
      hand: sp.hand?.length ?? 0,
      played: sp.played_this_turn?.length ?? 0,
      discard: sp.discard?.length ?? 0,
      presence_on_board: presenceTotal,
    }
  }
  const boardTotals: Record<string, { explorers: number; towns: number; cities: number; dahan: number; blight: number }> = {}
  for (const [bid, board] of Object.entries(state.board_state ?? {})) {
    const t = { explorers: 0, towns: 0, cities: 0, dahan: 0, blight: 0 }
    for (const land of Object.values(board.lands ?? {})) {
      t.explorers += land.explorers ?? 0
      t.towns += land.towns ?? 0
      t.cities += land.cities ?? 0
      t.dahan += land.dahan ?? 0
      t.blight += land.blight ?? 0
    }
    boardTotals[bid] = t
  }
  // Deep-clone deck state so the retrospective can replay the invader /
  // fear / event decks at each phase. Previously only totals were captured,
  // which made jumping back to T7 show today's deck (misleading).
  const deckSnapshot = {
    invader: state.invader_deck ? JSON.parse(JSON.stringify(state.invader_deck)) : null,
    fear: state.fear_deck ? JSON.parse(JSON.stringify(state.fear_deck)) : null,
    event: state.event_deck ? JSON.parse(JSON.stringify(state.event_deck)) : null,
  }
  return {
    round: state.round,
    phase: state.phase,
    fear_current: state.pools.fear_current,
    terror_level: state.pools.terror_level,
    blight_current: state.pools.blight_current,
    spirits: spiritTotals,
    boards: boardTotals,
    decks: deckSnapshot,
  }
}

function summarizeSnapshot(s: ReturnType<typeof buildSnapshot>): string {
  const totalPresence = Object.values(s.spirits).reduce((a, sp) => a + sp.presence_on_board, 0)
  const totalInvaders = Object.values(s.boards).reduce((a, b) => a + b.explorers + b.towns + b.cities, 0)
  return `Fear ${s.fear_current}·T${s.terror_level} · Blight ${s.blight_current} · Invaders ${totalInvaders} · Presence ${totalPresence}`
}

// Auto-snapshot on phase transitions for retrospective delta analysis.
// Dedupes by (round, phase): transitioning Growth → Fast → Growth → Fast UPDATES
// the existing Fast snapshot rather than appending a duplicate. This keeps deltas
// correct if the player navigates back-and-forth to fix mistakes.
watch(
  () => props.modelValue.phase,
  (newPhase: Phase, oldPhase: Phase | undefined) => {
    if (!oldPhase || newPhase === oldPhase) return
    const snapshot = buildSnapshot(props.modelValue)
    upsertSnapshot({
      phase: newPhase,
      from_phase: oldPhase,
      summary: summarizeSnapshot(snapshot),
      data: snapshot,
    })
  },
)

function upsertSnapshot(details: Record<string, unknown>) {
  const round = props.modelValue.round
  const existing = logEntries.value.findIndex(
    e => e.event === 'phase_snapshot' && e.round === round && e.details.phase === details.phase,
  )
  const entry = { round, event: 'phase_snapshot', details }
  if (existing >= 0) {
    const next = [...logEntries.value]
    next[existing] = entry
    ;(props.modelValue as unknown as { log: LogEntry[] }).log = next
  } else {
    addEntry('phase_snapshot', details)
  }
}

interface LogEntry {
  round: number
  event: string
  details: Record<string, unknown>
}

const logEntries = computed<LogEntry[]>(() => (props.modelValue.log as LogEntry[]) ?? [])
const currentRoundEntries = computed(() => logEntries.value.filter(e => e.round === props.modelValue.round))

// Fear-source attribution — the currently-selected source tag for Quick
// Fear buttons. Every +N bump logs with this source so the retrospective
// can slice fear generation by source type.
type FearSourceKey = 'fast-card' | 'slow-card' | 'innate' | 'event' | 'fear-card' | 'ravage' | 'adversary' | 'unattributed'
const fearSource = ref<FearSourceKey>('fast-card')
const SOURCE_LABELS: Record<FearSourceKey, string> = {
  'fast-card': 'Fast card',
  'slow-card': 'Slow card',
  'innate': 'Innate power',
  'event': 'Event card',
  'fear-card': 'Fear card',
  'ravage': 'Ravage',
  'adversary': 'Adversary effect',
  'unattributed': 'Unattributed',
}
function sourceLabel(key: FearSourceKey): string {
  return SOURCE_LABELS[key]
}

const customFearAmt = ref<number | null>(null)
function logCustomFear() {
  if (!customFearAmt.value || customFearAmt.value < 1) return
  logFearGenerated(customFearAmt.value, sourceLabel(fearSource.value))
  customFearAmt.value = null
}

// Event draft: user fills in fields, hits "Add to turn log"
const draftInvader = ref('')
const draftEvent = ref('')
const draftFear = ref('')
const draftNote = ref('')
const draftDrafted = ref('')

// Deck lookups for autocomplete on Event + Fear cards + drafted powers
const fearCards = ref<FearCardData[]>([])
const eventCards = ref<EventCardData[]>([])
interface NamedCard { name: string; expansion?: string; cost?: number; speed?: string }
const minorCards = ref<NamedCard[]>([])
const majorCards = ref<NamedCard[]>([])
onMounted(async () => {
  try {
    fearCards.value = await fetchDeck<FearCardData>('fear')
  } catch { /* best-effort; autocomplete degrades to free-form */ }
  try {
    eventCards.value = await fetchDeck<EventCardData>('event')
  } catch { /* best-effort */ }
  try {
    minorCards.value = await fetchDeck<NamedCard>('minor')
  } catch { /* best-effort */ }
  try {
    majorCards.value = await fetchDeck<NamedCard>('major')
  } catch { /* best-effort */ }
})

const fearMatches = computed(() => {
  const q = draftFear.value.trim().toLowerCase()
  if (!q) return []
  return fearCards.value.filter(c => c.name.toLowerCase().includes(q)).slice(0, 6)
})
const eventMatches = computed(() => {
  const q = draftEvent.value.trim().toLowerCase()
  if (!q) return []
  return eventCards.value.filter(c => c.name.toLowerCase().includes(q)).slice(0, 6)
})

// Invader-card autocomplete from a static catalog matching the base-game
// composition:
//   Stage I   (4 cards): single terrain — Mountain / Jungle / Sands / Wetland
//   Stage II  (5 cards): single terrain + Coastal Lands
//   Stage III (4 cards): two-terrain pair cards
// Users can still free-type anything outside this list.
const INVADER_CATALOG: string[] = [
  'Stage I — Mountain', 'Stage I — Jungle', 'Stage I — Sands', 'Stage I — Wetland',
  'Stage II — Mountain', 'Stage II — Jungle', 'Stage II — Sands',
  'Stage II — Wetland', 'Stage II — Coastal Lands',
  'Stage III — Jungle + Mountain', 'Stage III — Jungle + Sands',
  'Stage III — Jungle + Wetland', 'Stage III — Mountain + Sands',
  'Stage III — Mountain + Wetland', 'Stage III — Sands + Wetland',
]
const invaderMatches = computed(() => {
  const q = draftInvader.value.trim().toLowerCase()
  if (!q) return []
  return INVADER_CATALOG.filter(s => s.toLowerCase().includes(q)).slice(0, 6)
})

// Drafted card autocomplete — fuzzy match across minor + major power decks
// (tagged so the user can tell which deck a name came from at a glance).
interface DraftedMatch { name: string; expansion?: string; deck: 'minor' | 'major' }
const draftedMatches = computed<DraftedMatch[]>(() => {
  const q = draftDrafted.value.trim().toLowerCase()
  if (!q) return []
  const matches: DraftedMatch[] = []
  for (const c of minorCards.value) {
    if (c.name.toLowerCase().includes(q)) {
      matches.push({ name: c.name, expansion: c.expansion, deck: 'minor' })
      if (matches.length >= 8) return matches
    }
  }
  for (const c of majorCards.value) {
    if (c.name.toLowerCase().includes(q)) {
      matches.push({ name: c.name, expansion: c.expansion, deck: 'major' })
      if (matches.length >= 8) return matches
    }
  }
  return matches
})

function pickFear(name: string) {
  draftFear.value = name
  logFear()
}
function pickEvent(name: string) {
  draftEvent.value = name
  logEvent()
}
function pickInvader(value: string) {
  draftInvader.value = value
  logInvader()
}
function pickDrafted(m: DraftedMatch) {
  // Just populate the input; don't auto-log — user picks kept vs seen next
  draftDrafted.value = m.name
  draftDeck.value = m.deck
}

function addEntry(event: string, details: Record<string, unknown>) {
  const newLog = [...logEntries.value, { round: props.modelValue.round, event, details }]
  ;(props.modelValue as unknown as { log: LogEntry[] }).log = newLog
}

function logInvader() {
  if (!draftInvader.value.trim()) return
  addEntry('invader_card', { card: draftInvader.value.trim() })
  draftInvader.value = ''
}
function logEvent() {
  if (!draftEvent.value.trim()) return
  addEntry('event_card', { card: draftEvent.value.trim() })
  draftEvent.value = ''
}
function logFear() {
  if (!draftFear.value.trim()) return
  addEntry('fear_card', { card: draftFear.value.trim() })
  draftFear.value = ''
}
function logNote() {
  if (!draftNote.value.trim()) return
  addEntry('note', { text: draftNote.value.trim() })
  draftNote.value = ''
}
// Draft disposition — "kept" actually adds the card to the selected spirit's
// hand; "seen" is passive logging of a revealed-but-not-chosen card.
const draftDeck = ref<'minor' | 'major' | 'unique'>('minor')
const draftTargetSpirit = ref<string>('')
const allSpirits = computed(() => Object.keys(props.modelValue.spirits ?? {}))

function ensureDraftTargetSelected() {
  if (!draftTargetSpirit.value && allSpirits.value.length > 0) {
    draftTargetSpirit.value = allSpirits.value[0]
  }
}

function logDrafted(disposition: 'kept' | 'seen') {
  const cardName = draftDrafted.value.trim()
  if (!cardName) return
  ensureDraftTargetSelected()
  const spiritSlug = draftTargetSpirit.value || null
  if (disposition === 'kept') {
    addEntry('card_drafted', { card: cardName, deck: draftDeck.value, spirit: spiritSlug })
    // Add to the spirit's hand so it shows up in the Spirit Panel
    if (spiritSlug && props.modelValue.spirits[spiritSlug]) {
      const sp = props.modelValue.spirits[spiritSlug]
      sp.hand = [...(sp.hand ?? []), cardName]
    }
  } else {
    addEntry('card_seen', { card: cardName, deck: draftDeck.value })
  }
  draftDrafted.value = ''
}

function logFearGenerated(amount: number, source?: string) {
  // Just bump the pool + log; the App-level pools watcher handles auto-bank
  // and terror advancement uniformly so direct input edits work the same way.
  addEntry('fear_generated', source ? { amount, source } : { amount })
  props.modelValue.pools.fear_current = (props.modelValue.pools.fear_current ?? 0) + amount
}

function rewindTurn() {
  // Decrement the round so subsequent log entries attach to the previous
  // round. Does NOT revert state — pools, boards, spirits stay where they are.
  // Use the Retrospective's ⤴ jump for full phase-level reversal, or edit
  // specific log entries there. This is just "I was recording the wrong round".
  if (props.modelValue.round <= 1) return
  if (!confirm(
    `Rewind from Round ${props.modelValue.round} → ${props.modelValue.round - 1}?\n\n` +
    `Board/spirit/pool state is NOT reverted — only the round counter changes. ` +
    `Use the Retrospective panel to jump back to a specific phase or edit snapshots.`,
  )) return
  addEntry('turn_rewound', { from_round: props.modelValue.round, to_round: props.modelValue.round - 1 })
  props.modelValue.round = props.modelValue.round - 1
  props.modelValue.phase = 'timepasses'
}

function advanceTurn() {
  // Time Passes: move played_this_turn → discard for each spirit, reset elements
  const spirits = props.modelValue.spirits ?? {}
  for (const spirit of Object.values(spirits)) {
    if (spirit.played_this_turn?.length) {
      spirit.discard = [...(spirit.discard ?? []), ...spirit.played_this_turn]
      spirit.played_this_turn = []
    }
    spirit.elements_this_turn = {}
  }

  // Auto-resolve any previewed events (they fire during this turn's Invader phase).
  // Per rule: the first Event card is previewed on Setup/T1 and doesn't resolve
  // until T2. So on each Advance Turn, we move previewed → resolved at the
  // current round, which will become the "resolved_on_turn" stamp.
  const evDeck = props.modelValue.event_deck
  if (evDeck && evDeck.previewed.length) {
    const resolving = evDeck.previewed.map(e => ({ ...e, resolved_on_turn: props.modelValue.round }))
    props.modelValue.event_deck = {
      ...evDeck,
      previewed: [],
      resolved: [...evDeck.resolved, ...resolving],
    }
    for (const r of resolving) {
      addEntry('event_resolved', { card: r.name, previewed_on_turn: r.previewed_on_turn })
    }
  }

  // Auto-rotate invader deck: ravage→discarded, build→ravage, explore→build, upcoming[0]→explore
  const deck = props.modelValue.invader_deck
  if (deck) {
    const newRavage = deck.build
    const newBuild = deck.explore
    const upcoming = deck.upcoming ?? []
    const newExplore = upcoming.length ? upcoming[0] : null
    const rest = upcoming.slice(1)
    const ravagedCard = deck.ravage
    const prevDiscarded = Array.isArray(deck.discarded) ? deck.discarded : []
    const newDiscarded = ravagedCard ? [...prevDiscarded, ravagedCard] : prevDiscarded
    props.modelValue.invader_deck = {
      ravage: newRavage,
      build: newBuild,
      explore: newExplore,
      upcoming: rest,
      discarded: newDiscarded,
      notation: deck.notation,
    }
    if (ravagedCard) {
      addEntry('invader_rotated', {
        ravaged_stage: ravagedCard.stage,
        ravaged_terrain: ravagedCard.terrain,
      })
    }
  }

  // Log the turn advance
  addEntry('turn_advanced', {
    from_round: props.modelValue.round,
    fear_current: props.modelValue.pools.fear_current,
    blight_current: props.modelValue.pools.blight_current,
  })
  props.modelValue.round = props.modelValue.round + 1
  props.modelValue.phase = 'growth'
}

const entryIcon: Record<string, string> = {
  invader_card: 'resource-sacred-site',
  event_card: 'resource-sacred-site',
  fear_card: 'resource-fear',
  fear_generated: 'resource-fear',
  note: '',
  card_drafted: '',
  card_seen: '',
  turn_advanced: '',
  invader_rotated: '',
  phase_snapshot: '',
  event_resolved: '',
  innate_fired: 'resource-sacred-site',
  fear_card_earned: 'resource-fear',
  terror_advanced: 'resource-fear',
  turn_rewound: '',
  blight_added: 'resource-blight',
  blight_removed: 'resource-blight',
}

function entryLabel(e: LogEntry): string {
  switch (e.event) {
    case 'invader_card': return `Invader: ${e.details.card}`
    case 'event_card':   return `Event: ${e.details.card}`
    case 'fear_card':    return `Fear card drawn: ${e.details.card}`
    case 'fear_generated': return `+${e.details.amount} Fear generated`
    case 'card_drafted': {
      const tgt = e.details.spirit ? ` → ${e.details.spirit}` : ''
      const d = e.details.deck ? ` [${e.details.deck}]` : ''
      return `Drafted (kept)${d}: ${e.details.card}${tgt}`
    }
    case 'card_seen': {
      const d = e.details.deck ? ` [${e.details.deck}]` : ''
      return `Seen (not kept)${d}: ${e.details.card}`
    }
    case 'event_resolved': return `Event resolved: ${e.details.card}`
    case 'innate_fired': return `[innate] ${e.details.spirit} fired ${e.details.innate} at L${e.details.tier}`
    case 'turn_rewound': return `← Rewound: R${e.details.from_round} → R${e.details.to_round}`
    case 'fear_card_earned': return `[fear] card earned (face-down) — T${e.details.terror_level}`
    case 'terror_advanced': return `[terror] level advanced to ${e.details.to_level}`
    case 'note':         return e.details.text as string
    case 'turn_advanced': return `→ advanced to round ${(e.details.from_round as number) + 1}`
    case 'invader_rotated': return `Invader deck rotated — ravaged: Stage ${e.details.ravaged_stage} ${e.details.ravaged_terrain}`
    case 'phase_snapshot': return `[snap] ${e.details.phase} — ${e.details.summary}`
    case 'blight_added': return `[blight] +${e.details.amount}`
    case 'blight_removed': return `[blight] −${e.details.amount} removed`
    default: return `${e.event}: ${JSON.stringify(e.details)}`
  }
}

function removeEntry(entryRef: LogEntry) {
  const idx = logEntries.value.indexOf(entryRef)
  if (idx < 0) return
  const newLog = [...logEntries.value]
  newLog.splice(idx, 1)
  ;(props.modelValue as unknown as { log: LogEntry[] }).log = newLog
}
</script>

<template>
  <div class="turn-ctl card">
    <div class="hdr">
      <div class="round-block">
        <span class="label">Round</span>
        <span class="round-num">{{ modelValue.round }}</span>
      </div>
      <div class="phase-block">
        <span class="label">Phase</span>
        <select v-model="modelValue.phase">
          <option value="setup">Setup</option>
          <option value="growth">Growth</option>
          <option value="fast">Fast</option>
          <option value="event">Event</option>
          <option value="fear">Fear</option>
          <option value="invader">Invader</option>
          <option value="slow">Slow</option>
          <option value="timepasses">Time Passes</option>
          <option value="end">End</option>
        </select>
      </div>
      <div class="turn-actions">
        <button
          class="ghost"
          @click="rewindTurn"
          :disabled="modelValue.round <= 1"
          title="Decrement round — for correcting mistimed log entries (state is not reverted)"
        >← Previous Turn</button>
        <button class="primary advance-btn" @click="advanceTurn" title="Move Played→Discard, reset elements, increment round">
          Advance Turn →
        </button>
      </div>
    </div>

    <!-- Event-log inputs -->
    <div class="log-inputs">
      <div class="log-input autocomplete-host">
        <span class="label">Invader card</span>
        <input type="text" v-model="draftInvader" placeholder="e.g., Stage I — Mountain" @keyup.enter="logInvader" />
        <button class="ghost" @click="logInvader" :disabled="!draftInvader.trim()">Log</button>
        <div v-if="invaderMatches.length" class="suggestions">
          <button v-for="m in invaderMatches" :key="m" type="button" class="suggestion" @click="pickInvader(m)">
            <span class="sug-name">{{ m }}</span>
          </button>
        </div>
      </div>
      <div class="log-input autocomplete-host">
        <span class="label">Event card</span>
        <input type="text" v-model="draftEvent" placeholder="e.g., A Strange Madness Among the Beasts" @keyup.enter="logEvent" />
        <button class="ghost" @click="logEvent" :disabled="!draftEvent.trim()">Log</button>
        <div v-if="eventMatches.length" class="suggestions">
          <button v-for="m in eventMatches" :key="m.name" type="button" class="suggestion" @click="pickEvent(m.name)">
            <span class="sug-name">{{ m.name }}</span>
            <ExpansionBadge :slug="m.expansion" :size="14" class="sug-expansion" />
          </button>
        </div>
      </div>
      <div class="log-input autocomplete-host">
        <span class="label">Fear card drawn</span>
        <input type="text" v-model="draftFear" placeholder="e.g., Angry Mobs" @keyup.enter="logFear" />
        <button class="ghost" @click="logFear" :disabled="!draftFear.trim()">Log</button>
        <div v-if="fearMatches.length" class="suggestions">
          <button v-for="m in fearMatches" :key="m.name" type="button" class="suggestion" @click="pickFear(m.name)">
            <span class="sug-name">{{ m.name }}</span>
            <ExpansionBadge :slug="m.expansion" :size="14" class="sug-expansion" />
          </button>
        </div>
      </div>
      <div class="log-input draft-row autocomplete-host wide">
        <span class="label">Draft</span>
        <div class="draft-controls">
          <input
            type="text"
            v-model="draftDrafted"
            placeholder="card name (e.g., Call of the Dahan Ways)"
            class="draft-name"
            @keyup.enter="logDrafted('kept')"
          />
          <select v-model="draftDeck" class="draft-deck" title="Which deck this card came from">
            <option value="minor">Minor</option>
            <option value="major">Major</option>
            <option value="unique">Unique</option>
          </select>
          <select v-model="draftTargetSpirit" class="draft-spirit" title="Which spirit drafts this card (if kept, added to their hand)">
            <option value="" disabled>spirit…</option>
            <option v-for="s in allSpirits" :key="s" :value="s">{{ s }}</option>
          </select>
          <button
            class="primary"
            @click="logDrafted('kept')"
            :disabled="!draftDrafted.trim() || !draftTargetSpirit"
            title="Kept — adds to the spirit's hand"
          >
            Keep → hand
          </button>
          <button
            class="ghost"
            @click="logDrafted('seen')"
            :disabled="!draftDrafted.trim()"
            title="Seen but not kept — logs for retrospective, doesn't modify spirit state"
          >
            Seen only
          </button>
        </div>
        <div v-if="draftedMatches.length" class="suggestions draft-suggestions">
          <button v-for="m in draftedMatches" :key="`${m.deck}:${m.name}`" type="button" class="suggestion" @click="pickDrafted(m)">
            <span class="sug-name">{{ m.name }}</span>
            <span class="sug-meta">
              <span class="sug-deck" :class="m.deck">{{ m.deck }}</span>
              <ExpansionBadge v-if="m.expansion" :slug="m.expansion" :size="14" class="sug-expansion" />
            </span>
          </button>
        </div>
      </div>
      <div class="log-input wide">
        <span class="label">Note</span>
        <input type="text" v-model="draftNote" placeholder="anything else to record this turn" @keyup.enter="logNote" />
        <button class="ghost" @click="logNote" :disabled="!draftNote.trim()">Log</button>
      </div>
    </div>

    <!-- Quick fear-generation buttons with source-attribution so retrospective
         stats can slice fear by source type (card plays vs events vs ravages).
         Free-form input handles big bumps (+7 fear cards, +N innate chains). -->
    <div class="fear-quick">
      <span class="label">Quick fear:</span>
      <select v-model="fearSource" class="fear-source-select" title="Source category for this fear generation">
        <option value="fast-card">Fast card</option>
        <option value="slow-card">Slow card</option>
        <option value="innate">Innate power</option>
        <option value="event">Event card</option>
        <option value="fear-card">Fear card</option>
        <option value="ravage">Ravage</option>
        <option value="adversary">Adversary effect</option>
        <option value="unattributed">Unattributed</option>
      </select>
      <button v-for="n in [1, 2, 3, 4, 5]" :key="n" class="fear-btn" @click="logFearGenerated(n, sourceLabel(fearSource))">+{{ n }} <Icon name="resource-fear" :size="12" decorative /></button>
      <span class="fear-custom-wrap">
        <input
          type="number"
          min="1"
          v-model.number="customFearAmt"
          class="fear-custom-input"
          placeholder="+N"
          title="Enter any amount — for +7 fear cards, innate chains, etc."
          @keyup.enter="logCustomFear"
        />
        <button
          class="fear-btn"
          @click="logCustomFear"
          :disabled="!customFearAmt || customFearAmt < 1"
        >+ <Icon name="resource-fear" :size="12" decorative /></button>
      </span>
    </div>

    <!-- This turn's log entries -->
    <div v-if="currentRoundEntries.length" class="turn-log">
      <div class="log-label">Round {{ modelValue.round }} log</div>
      <ul>
        <li v-for="(e, i) in currentRoundEntries" :key="i" class="log-entry" :data-event="e.event">
          <Icon v-if="entryIcon[e.event]" :name="entryIcon[e.event]" :size="12" decorative />
          <span class="entry-text">{{ entryLabel(e) }}</span>
          <button class="ghost tiny" @click="removeEntry(e)" title="remove">×</button>
        </li>
      </ul>
    </div>

    <!-- Prior rounds summary -->
    <details v-if="logEntries.length > currentRoundEntries.length" class="prior-rounds">
      <summary>Prior rounds ({{ logEntries.length - currentRoundEntries.length }} entries)</summary>
      <ul>
        <li v-for="(e, i) in logEntries.filter(x => x.round !== modelValue.round)" :key="i" class="log-entry prior">
          <span class="entry-round mono">R{{ e.round }}</span>
          <span class="entry-text">{{ entryLabel(e) }}</span>
        </li>
      </ul>
    </details>
  </div>
</template>

<style scoped>
.turn-ctl { display: flex; flex-direction: column; gap: var(--sp-3); }

.hdr {
  display: flex; align-items: flex-end; gap: var(--sp-4);
  flex-wrap: wrap;
  padding-bottom: var(--sp-3);
  border-bottom: 1px solid var(--border-subtle);
}

.round-block, .phase-block {
  display: flex; flex-direction: column; gap: 2px;
}

.round-num {
  font-family: var(--font-mono);
  font-size: 2rem;
  font-weight: var(--fw-bold);
  line-height: 1;
  color: var(--accent-amber);
}

.phase-block select { text-transform: capitalize; min-width: 8rem; }

.advance-btn { font-size: var(--fs-sm); padding: var(--sp-2) var(--sp-4); }
.turn-actions { margin-left: auto; display: inline-flex; gap: var(--sp-2); align-items: center; }

.log-inputs {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
  gap: var(--sp-2);
}
.log-input {
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: var(--sp-1) var(--sp-2);
  align-items: center;
}
.log-input.wide { grid-column: 1 / -1; }
.log-input .label { min-width: 5.5rem; }
.log-input input { min-width: 0; }
.log-input.autocomplete-host { position: relative; }
.log-input.autocomplete-host .suggestions {
  position: absolute; top: calc(100% + 2px); left: 5.5rem; right: 3rem;
  z-index: 25;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  box-shadow: 0 4px 14px rgba(0, 0, 0, 0.35);
  max-height: 14rem; overflow-y: auto;
}
.log-input.autocomplete-host .suggestion {
  width: 100%;
  display: flex; justify-content: space-between; gap: var(--sp-2);
  padding: 4px var(--sp-2);
  border: none; background: transparent; color: inherit;
  text-align: left; cursor: pointer; border-radius: 0;
  font-size: var(--fs-xs);
}
.log-input.autocomplete-host .suggestion:hover { background: var(--bg-muted); }
.log-input.autocomplete-host .sug-name { font-weight: var(--fw-medium); }
.log-input.autocomplete-host .sug-expansion { color: var(--text-muted); font-style: italic; font-size: 0.68rem; }
.log-input.autocomplete-host .sug-meta { display: inline-flex; gap: 4px; align-items: center; }
.log-input.autocomplete-host .sug-deck {
  font-family: var(--font-mono); font-size: 0.62rem;
  padding: 0 4px; border-radius: var(--r-sm);
  text-transform: uppercase; letter-spacing: 0.06em;
}
.log-input.autocomplete-host .sug-deck.minor { background: rgba(82, 183, 136, 0.2); color: #52b788; }
.log-input.autocomplete-host .sug-deck.major { background: rgba(199, 125, 255, 0.2); color: #c77dff; }

.draft-row .draft-controls {
  display: grid;
  grid-template-columns: 1fr auto auto auto auto;
  gap: var(--sp-1);
  align-items: center;
  min-width: 0;
}
@media (max-width: 820px) {
  .draft-row .draft-controls {
    grid-template-columns: 1fr auto auto;
    grid-template-rows: auto auto;
  }
  .draft-row .primary, .draft-row .ghost { grid-column: span 1; }
}
.draft-row .draft-name { min-width: 0; }
.draft-row .draft-deck, .draft-row .draft-spirit {
  font-family: var(--font-mono);
  font-size: var(--fs-xs);
  text-transform: capitalize;
}
.draft-row .draft-spirit { max-width: 12rem; }
.draft-suggestions { left: 5.5rem; right: 0; }

.fear-quick {
  display: flex; gap: var(--sp-1); align-items: center;
  flex-wrap: wrap;
}
.fear-btn {
  display: inline-flex; gap: 3px; align-items: center;
  padding: 2px var(--sp-2); font-size: var(--fs-xs);
}
.fear-source-select {
  font-size: var(--fs-xs);
  padding: 2px var(--sp-1);
  max-width: 9rem;
}
.fear-custom-wrap {
  display: inline-flex; gap: 2px; align-items: center;
  margin-left: var(--sp-1);
  padding-left: var(--sp-2);
  border-left: 1px solid var(--border-subtle);
}
.fear-custom-input {
  width: 3rem;
  padding: 2px var(--sp-1);
  font-size: var(--fs-xs);
  text-align: center;
  font-family: var(--font-mono);
}

.turn-log {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-2) var(--sp-3);
}
.log-label {
  font-size: var(--fs-xxs);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
  margin-bottom: var(--sp-1);
}
.turn-log ul, .prior-rounds ul { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 2px; }

.log-entry {
  display: flex; align-items: center; gap: var(--sp-2);
  font-size: var(--fs-sm); padding: 2px 0;
}
.log-entry.prior { color: var(--text-secondary); font-size: var(--fs-xs); }
.entry-round {
  font-family: var(--font-mono); font-size: var(--fs-xs);
  color: var(--text-muted); min-width: 2rem;
}
.entry-text { flex: 1; }

.log-entry[data-event="fear_card"] .entry-text,
.log-entry[data-event="fear_generated"] .entry-text { color: var(--accent-amber); }
.log-entry[data-event="event_card"] .entry-text { color: var(--accent-purple); }
.log-entry[data-event="invader_card"] .entry-text { color: var(--accent-blue); }
.log-entry[data-event="card_drafted"] .entry-text { color: var(--accent-green); }
.log-entry[data-event="turn_advanced"] .entry-text { color: var(--text-muted); font-style: italic; }

.tiny { padding: 0 var(--sp-1); font-size: var(--fs-xs); color: var(--text-muted); }
.tiny:hover { color: var(--status-danger); background: transparent; border: none; }

.prior-rounds { font-size: var(--fs-xs); }
.prior-rounds summary {
  list-style: none; cursor: pointer; padding: var(--sp-1) 0;
  color: var(--text-secondary);
}
.prior-rounds summary::-webkit-details-marker { display: none; }
.prior-rounds summary::before {
  content: '▸';
  margin-right: var(--sp-2);
  color: var(--text-muted);
  transition: transform var(--motion-fast);
  display: inline-block;
}
.prior-rounds[open] summary::before { transform: rotate(90deg); }
</style>
