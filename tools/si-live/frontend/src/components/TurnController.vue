<script setup lang="ts">
import { computed, ref } from 'vue'
import type { GameState } from '../types'
import Icon from './Icon.vue'

const props = defineProps<{ modelValue: GameState }>()
defineEmits<{ 'update:modelValue': [value: GameState] }>()

interface LogEntry {
  round: number
  event: string
  details: Record<string, unknown>
}

const logEntries = computed<LogEntry[]>(() => (props.modelValue.log as LogEntry[]) ?? [])
const currentRoundEntries = computed(() => logEntries.value.filter(e => e.round === props.modelValue.round))

// Event draft: user fills in fields, hits "Add to turn log"
const draftInvader = ref('')
const draftEvent = ref('')
const draftFear = ref('')
const draftNote = ref('')
const draftDrafted = ref('')

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
function logDrafted() {
  if (!draftDrafted.value.trim()) return
  addEntry('card_drafted', { card: draftDrafted.value.trim() })
  draftDrafted.value = ''
}

function logFearGenerated(amount: number) {
  addEntry('fear_generated', { amount })
  // Also bump the current fear pool
  const newFear = (props.modelValue.pools.fear_current ?? 0) + amount
  props.modelValue.pools.fear_current = newFear
  // Terror transitions
  const thr = props.modelValue.pools.fear_threshold
  if (thr > 0) {
    if (newFear >= thr * 3 && props.modelValue.pools.terror_level < 3) {
      props.modelValue.pools.terror_level = 3
    } else if (newFear >= thr * 2 && props.modelValue.pools.terror_level < 2) {
      props.modelValue.pools.terror_level = 2
    }
  }
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
  turn_advanced: '',
}

function entryLabel(e: LogEntry): string {
  switch (e.event) {
    case 'invader_card': return `Invader: ${e.details.card}`
    case 'event_card':   return `Event: ${e.details.card}`
    case 'fear_card':    return `Fear card drawn: ${e.details.card}`
    case 'fear_generated': return `+${e.details.amount} Fear generated`
    case 'card_drafted': return `Drafted: ${e.details.card}`
    case 'note':         return e.details.text as string
    case 'turn_advanced': return `→ advanced to round ${(e.details.from_round as number) + 1}`
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
          <option value="invader">Invader</option>
          <option value="slow">Slow</option>
          <option value="timepasses">Time Passes</option>
          <option value="end">End</option>
        </select>
      </div>
      <button class="primary advance-btn" @click="advanceTurn" title="Move Played→Discard, reset elements, increment round">
        Advance Turn →
      </button>
    </div>

    <!-- Event-log inputs -->
    <div class="log-inputs">
      <div class="log-input">
        <span class="label">Invader card</span>
        <input type="text" v-model="draftInvader" placeholder="e.g., Stage I — Mountain" @keyup.enter="logInvader" />
        <button class="ghost" @click="logInvader" :disabled="!draftInvader.trim()">Log</button>
      </div>
      <div class="log-input">
        <span class="label">Event card</span>
        <input type="text" v-model="draftEvent" placeholder="e.g., A Strange Madness Among the Beasts" @keyup.enter="logEvent" />
        <button class="ghost" @click="logEvent" :disabled="!draftEvent.trim()">Log</button>
      </div>
      <div class="log-input">
        <span class="label">Fear card drawn</span>
        <input type="text" v-model="draftFear" placeholder="e.g., Angry Mobs" @keyup.enter="logFear" />
        <button class="ghost" @click="logFear" :disabled="!draftFear.trim()">Log</button>
      </div>
      <div class="log-input">
        <span class="label">Drafted / seen</span>
        <input type="text" v-model="draftDrafted" placeholder="e.g., Concealing Shadows (drafted)" @keyup.enter="logDrafted" />
        <button class="ghost" @click="logDrafted" :disabled="!draftDrafted.trim()">Log</button>
      </div>
      <div class="log-input wide">
        <span class="label">Note</span>
        <input type="text" v-model="draftNote" placeholder="anything else to record this turn" @keyup.enter="logNote" />
        <button class="ghost" @click="logNote" :disabled="!draftNote.trim()">Log</button>
      </div>
    </div>

    <!-- Quick fear-generation buttons for speed -->
    <div class="fear-quick">
      <span class="label">Quick fear:</span>
      <button v-for="n in [1, 2, 3, 4, 5]" :key="n" class="fear-btn" @click="logFearGenerated(n)">+{{ n }} <Icon name="resource-fear" :size="12" decorative /></button>
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

.advance-btn { margin-left: auto; font-size: var(--fs-sm); padding: var(--sp-2) var(--sp-4); }

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

.fear-quick {
  display: flex; gap: var(--sp-1); align-items: center;
  flex-wrap: wrap;
}
.fear-btn {
  display: inline-flex; gap: 3px; align-items: center;
  padding: 2px var(--sp-2); font-size: var(--fs-xs);
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
