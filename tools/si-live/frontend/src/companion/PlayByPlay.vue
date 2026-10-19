<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { Change, PublicDecision } from './types'
import Panel from './Panel.vue'
import { useCompanionCtx } from './context'
import { prettyText, stepLabel } from './format'

/**
 * Play by play: changes between saves and the player's own decisions from
 * the log, grouped by turn and step, newest group at the top. Inside a group
 * the decisions come first (they cause the changes), then the changes.
 */
const props = defineProps<{ changes: Change[]; decisions: PublicDecision[]; fill?: boolean }>()
const ctx = useCompanionCtx()

const view = ref<'all' | 'changes' | 'decisions'>('all')

const CHANGE_LABEL: Record<string, string> = {
  invader: 'Invaders', blight: 'Blight', dahan: 'Dahan', presence: 'Presence',
  fear: 'Fear', card: 'Card', energy: 'Energy', event: 'Event',
}
const DECISION_LABEL: Record<string, string> = {
  growth: 'Growth', 'gain-power': 'Gain power', 'card-choice': 'Card choice', play: 'Play', power: 'Power',
  effect: 'Effect', 'land-choice': 'Land choice', presence: 'Presence', damage: 'Damage', forget: 'Forget',
}

// Engine per-turn step order (steam/src/companion.rs STATE_ORDER).
const STEP_ORDER = [
  'PlayerSetup', 'InvadersStartingAction', 'SpiritPhase', 'Grow', 'GainEnergy', 'PlayPayPower',
  'FastPower', 'BlightedIsland', 'Event', 'Fear', 'HighImmigration', 'Ravage', 'Build', 'Explore',
  'AdvanceInvaderCards', 'SlowPower', 'TimePasses',
]
const rank = (step: string) => {
  const i = STEP_ORDER.indexOf(step)
  return i < 0 ? STEP_ORDER.length : i
}

const keyOf = (c: Change) => `${c.turn}|${c.step}|${c.kind}|${c.text}`
const dKey = (d: PublicDecision) => `d|${d.ms}|${d.text}`
const seen = ref<Set<string>>(new Set())
const fresh = ref<Set<string>>(new Set())
watch(
  () => [props.changes, props.decisions] as const,
  ([ch, de]) => {
    const prev = seen.value
    const isFirst = prev.size === 0
    const now = new Set([...ch.map(keyOf), ...de.map(dKey)])
    fresh.value = isFirst ? new Set() : new Set([...now].filter((k) => !prev.has(k)))
    seen.value = now
  },
  { immediate: true },
)

type Item =
  | { type: 'decision'; d: PublicDecision; key: string }
  | { type: 'change'; c: Change; key: string }
interface Group { turn: number; step: string; items: Item[] }

const groups = computed<Group[]>(() => {
  const map = new Map<string, Group>()
  const order: Group[] = []
  const get = (turn: number, step: string) => {
    const k = `${turn}|${step}`
    let g = map.get(k)
    if (!g) {
      g = { turn, step, items: [] }
      map.set(k, g)
      order.push(g)
    }
    return g
  }
  if (view.value !== 'changes') {
    for (const d of [...props.decisions].sort((a, b) => a.ms - b.ms)) get(d.turn, d.step).items.push({ type: 'decision', d, key: dKey(d) })
  }
  if (view.value !== 'decisions') {
    props.changes.forEach((c, i) => get(c.turn, c.step).items.push({ type: 'change', c, key: `${keyOf(c)}|${i}` }))
  }
  // Newest group first; inside a group: decisions (oldest first), then changes (oldest first).
  return order.sort((a, b) => b.turn - a.turn || rank(b.step) - rank(a.step))
})
const isFresh = (it: Item) => fresh.value.has(it.type === 'decision' ? it.key : keyOf(it.c))
</script>

<template>
  <Panel title="Play by play" question="What you did, and what changed between saves" anchor="panel-feed" flush :fill="fill">
    <template #meta>
      <div class="seg" role="radiogroup" aria-label="Show">
        <button v-for="v in (['all', 'decisions', 'changes'] as const)" :key="v" type="button" role="radio" :aria-checked="view === v" :class="{ on: view === v }" @click="view = v">
          {{ v === 'all' ? 'Both' : v === 'decisions' ? 'Decisions' : 'Changes' }}
        </button>
      </div>
    </template>
    <div class="feed" :class="{ fill }" aria-live="polite">
      <section v-for="g in groups" :key="`${g.turn}-${g.step}`" class="grp">
        <h3 class="g-h"><span class="g-t">Turn {{ g.turn }}</span> {{ stepLabel(g.step) }}</h3>
        <ul>
          <template v-for="it in g.items" :key="it.key">
            <li v-if="it.type === 'decision'" class="row dec" :class="{ fresh: isFresh(it) }">
              <i class="mark" aria-hidden="true"></i>
              <span class="kind">{{ DECISION_LABEL[it.d.kind] ?? it.d.kind }}</span>
              <button
                v-if="it.d.land"
                type="button"
                class="txt link"
                :title="`Show ${prettyText(it.d.land)} on the map`"
                @click="ctx?.selectLand(it.d.land)"
              >{{ prettyText(it.d.text) }}</button>
              <span v-else class="txt">{{ prettyText(it.d.text) }}</span>
            </li>
            <li v-else class="row" :class="[`k-${it.c.kind}`, { fresh: isFresh(it) }]">
              <i class="dot" aria-hidden="true"></i>
              <span class="kind">{{ CHANGE_LABEL[it.c.kind] ?? it.c.kind }}</span>
              <span class="txt">{{ prettyText(it.c.text) }}</span>
            </li>
          </template>
        </ul>
      </section>
      <p v-if="!groups.length" class="empty">Changes and decisions appear here as the game saves.</p>
    </div>
    <div class="key">
      <span><i class="mark"></i> your decision</span>
      <span><i class="dot"></i> change between saves</span>
    </div>
  </Panel>
</template>

<style scoped>
.feed { max-height: 460px; overflow-y: auto; padding: 4px 0 8px; }
@media (min-width: 1600px) { .feed.fill { max-height: none; overflow: visible; } }
.grp { padding: 4px 14px 2px; }
.g-h { position: sticky; top: 0; margin: 0; padding: 4px 0; font-size: 12px; font-weight: 600; color: var(--text-secondary); background: var(--bg-surface); z-index: 1; }
.g-t { color: var(--text-white); margin-right: 4px; }
ul { list-style: none; margin: 0; padding: 0; }
.row { display: grid; grid-template-columns: 8px 70px minmax(0, 1fr); gap: 6px; align-items: baseline; padding: 2px 4px; border-radius: 4px; font-size: 12.5px; }
.dot { width: 7px; height: 7px; border-radius: 50%; background: var(--text-muted); align-self: start; margin-top: 6px; display: inline-block; }
.mark { width: 7px; height: 7px; transform: rotate(45deg); background: var(--text-white); align-self: start; margin-top: 6px; display: inline-block; border-radius: 1px; }
.kind { font-size: 11px; color: var(--text-muted); }
.txt { color: var(--text-primary); }
.dec .txt { color: var(--text-white); font-weight: 500; }
.txt.link { all: unset; cursor: pointer; color: var(--text-white); font-weight: 500; text-decoration: underline; text-decoration-color: var(--border-strong); text-underline-offset: 2px; }
.txt.link:hover { text-decoration-color: var(--accent-blue-hover); }
.txt.link:focus-visible { outline: 2px solid var(--accent-blue-hover); outline-offset: 1px; border-radius: 2px; }
.row.k-invader .dot { background: var(--accent-red); }
.row.k-blight .dot { background: #a78bfa; }
.row.k-dahan .dot { background: var(--accent-green); }
.row.k-presence .dot { background: var(--accent-blue-hover); }
.row.k-fear .dot { background: var(--accent-amber); }
.row.k-card .dot { background: var(--accent-teal); }
.row.k-energy .dot { background: var(--pool-energy); }
.row.k-event .dot { background: var(--accent-coral); }
.row.fresh { animation: fresh 2.4s ease-out; }
@keyframes fresh {
  from { background: rgba(59, 130, 246, 0.28); }
  to { background: transparent; }
}
@media (prefers-reduced-motion: reduce) {
  .row.fresh { animation: none; background: rgba(59, 130, 246, 0.14); }
}
.seg { display: inline-flex; gap: 2px; padding: 2px; border-radius: 6px; background: var(--bg-inset); }
.seg button { padding: 1px 8px; font-size: 11.5px; border: 0; background: transparent; color: var(--text-secondary); border-radius: 4px; }
.seg button.on { background: var(--bg-active); color: var(--text-white); }
.key { display: flex; gap: 14px; padding: 6px 14px 8px; border-top: 1px solid var(--border-subtle); font-size: 11px; color: var(--text-muted); }
.key span { display: inline-flex; align-items: center; gap: 6px; }
.key .dot, .key .mark { margin-top: 0; }
.empty { margin: 8px 14px; font-size: 12.5px; color: var(--text-muted); }
</style>
