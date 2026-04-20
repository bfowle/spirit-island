<script setup lang="ts">
import { computed, ref } from 'vue'
import type { GameState, Phase, InvaderCard, FearCardEntry, EventCardEntry } from '../types'

/**
 * Retrospective view — walks the log's phase_snapshot + turn_advanced entries
 * and renders per-phase deltas for post-game self-evaluation.
 *
 * The purpose isn't to make decisions in-the-moment; it's to surface where
 * things went sideways across the arc of a game (fear stalled round 3, blight
 * jumped 4 in one invader phase, presence regressed round 5 after a destroy,
 * etc.) so the player can see and learn.
 */

interface LogEntry {
  round: number
  event: string
  details: Record<string, unknown>
}

interface DeckSnapshot {
  invader: {
    ravage: InvaderCard | null
    build: InvaderCard | null
    explore: InvaderCard | null
    upcoming: InvaderCard[]
    discarded: InvaderCard[]
    notation?: string | null
  } | null
  fear: {
    deck_size: number
    earned: FearCardEntry[]
    resolved: FearCardEntry[]
    unseen: number
  } | null
  event: {
    previewed: EventCardEntry[]
    resolved: EventCardEntry[]
    unseen: number
  } | null
}

interface SnapshotData {
  round: number
  phase: string
  fear_current: number
  terror_level: number
  blight_current: number
  spirits: Record<string, { energy: number; card_plays: number; hand: number; played: number; discard: number; presence_on_board: number }>
  boards: Record<string, { explorers: number; towns: number; cities: number; dahan: number; blight: number }>
  decks?: DeckSnapshot
}

const props = defineProps<{ state: GameState }>()

const log = computed<LogEntry[]>(() => (props.state.log as LogEntry[]) ?? [])

function writeLog(next: LogEntry[]) {
  ;(props.state as unknown as { log: LogEntry[] }).log = next
}

function deleteSnapshot(round: number, phase: string) {
  const next = log.value.filter(
    e => !(e.event === 'phase_snapshot' && e.round === round && e.details.phase === phase),
  )
  writeLog(next)
}

function jumpTo(round: number, phase: string) {
  props.state.round = round
  props.state.phase = phase as Phase
}

function captureCurrent() {
  const currentRound = props.state.round
  const currentPhase = props.state.phase
  const snapshot: SnapshotData = {
    round: currentRound,
    phase: currentPhase,
    fear_current: props.state.pools.fear_current,
    terror_level: props.state.pools.terror_level,
    blight_current: props.state.pools.blight_current,
    spirits: {},
    boards: {},
  }
  for (const [slug, sp] of Object.entries(props.state.spirits ?? {})) {
    const presenceTotal = Object.values(sp.presence_on_board ?? {}).reduce((a, b) => a + (b ?? 0), 0)
    snapshot.spirits[slug] = {
      energy: sp.energy ?? 0,
      card_plays: sp.card_plays ?? 0,
      hand: sp.hand?.length ?? 0,
      played: sp.played_this_turn?.length ?? 0,
      discard: sp.discard?.length ?? 0,
      presence_on_board: presenceTotal,
    }
  }
  for (const [bid, board] of Object.entries(props.state.board_state ?? {})) {
    const t = { explorers: 0, towns: 0, cities: 0, dahan: 0, blight: 0 }
    for (const land of Object.values(board.lands ?? {})) {
      t.explorers += land.explorers ?? 0
      t.towns += land.towns ?? 0
      t.cities += land.cities ?? 0
      t.dahan += land.dahan ?? 0
      t.blight += land.blight ?? 0
    }
    snapshot.boards[bid] = t
  }
  const existing = log.value.findIndex(
    e => e.event === 'phase_snapshot' && e.round === currentRound && e.details.phase === currentPhase,
  )
  const totalPresence = Object.values(snapshot.spirits).reduce((a, sp) => a + sp.presence_on_board, 0)
  const totalInvaders = Object.values(snapshot.boards).reduce((a, b) => a + b.explorers + b.towns + b.cities, 0)
  const entry: LogEntry = {
    round: currentRound,
    event: 'phase_snapshot',
    details: {
      phase: currentPhase,
      from_phase: currentPhase,
      summary: `Fear ${snapshot.fear_current}·T${snapshot.terror_level} · Blight ${snapshot.blight_current} · Invaders ${totalInvaders} · Presence ${totalPresence}`,
      data: snapshot,
    },
  }
  if (existing >= 0) {
    const next = [...log.value]
    next[existing] = entry
    writeLog(next)
  } else {
    writeLog([...log.value, entry])
  }
}


const snapshots = computed<Array<{ round: number; phase: string; from: string; summary: string; data: SnapshotData }>>(() =>
  log.value
    .filter(e => e.event === 'phase_snapshot')
    .map(e => ({
      round: e.round,
      phase: e.details.phase as string,
      from: e.details.from_phase as string,
      summary: e.details.summary as string,
      data: e.details.data as SnapshotData,
    })),
)

interface Delta {
  from: { round: number; phase: string }
  to: { round: number; phase: string }
  fear_delta: number
  terror_delta: number
  blight_delta: number
  invaders_delta: number
  presence_delta: number
  explorers_delta: number
  towns_delta: number
  cities_delta: number
  dahan_delta: number
  notable: string[]
}

function totalInvaders(s: SnapshotData): number {
  return Object.values(s.boards).reduce((a, b) => a + b.explorers + b.towns + b.cities, 0)
}
function totalPresence(s: SnapshotData): number {
  return Object.values(s.spirits).reduce((a, sp) => a + sp.presence_on_board, 0)
}
function totalBoardStat(s: SnapshotData, key: 'explorers' | 'towns' | 'cities' | 'dahan'): number {
  return Object.values(s.boards).reduce((a, b) => a + b[key], 0)
}

const deltas = computed<Delta[]>(() => {
  const snaps = snapshots.value
  const result: Delta[] = []
  for (let i = 1; i < snaps.length; i++) {
    const a = snaps[i - 1]
    const b = snaps[i]
    const fearDelta = b.data.fear_current - a.data.fear_current
    const blightDelta = b.data.blight_current - a.data.blight_current
    const invadersDelta = totalInvaders(b.data) - totalInvaders(a.data)
    const presenceDelta = totalPresence(b.data) - totalPresence(a.data)
    const explorersDelta = totalBoardStat(b.data, 'explorers') - totalBoardStat(a.data, 'explorers')
    const townsDelta = totalBoardStat(b.data, 'towns') - totalBoardStat(a.data, 'towns')
    const citiesDelta = totalBoardStat(b.data, 'cities') - totalBoardStat(a.data, 'cities')
    const dahanDelta = totalBoardStat(b.data, 'dahan') - totalBoardStat(a.data, 'dahan')
    const terrorDelta = b.data.terror_level - a.data.terror_level

    const notable: string[] = []
    if (fearDelta >= 3) notable.push(`+${fearDelta} fear — strong round`)
    if (fearDelta === 0 && b.data.phase !== 'growth') notable.push('No fear earned this phase — review threat prioritization')
    if (blightDelta >= 2) notable.push(`+${blightDelta} blight — breakdown of defensive coverage`)
    if (terrorDelta > 0) notable.push(`Terror tier rose to ${b.data.terror_level}`)
    if (presenceDelta < 0) notable.push(`${presenceDelta} presence — destroyed this phase`)
    if (citiesDelta > 0) notable.push(`+${citiesDelta} cities built — watch for ravage spikes`)
    if (dahanDelta < 0) notable.push(`${dahanDelta} Dahan lost`)

    result.push({
      from: { round: a.round, phase: a.phase },
      to: { round: b.round, phase: b.phase },
      fear_delta: fearDelta,
      terror_delta: terrorDelta,
      blight_delta: blightDelta,
      invaders_delta: invadersDelta,
      presence_delta: presenceDelta,
      explorers_delta: explorersDelta,
      towns_delta: townsDelta,
      cities_delta: citiesDelta,
      dahan_delta: dahanDelta,
      notable,
    })
  }
  return result
})

function deltaClass(n: number, bad: 'positive' | 'negative'): string {
  if (n === 0) return 'neutral'
  const isBad = bad === 'positive' ? n > 0 : n < 0
  return isBad ? 'negative' : 'positive'
}

function deltaStr(n: number): string {
  if (n === 0) return '0'
  return n > 0 ? `+${n}` : `${n}`
}

const showDetails = ref<Set<string>>(new Set())

function toggleDetails(key: string) {
  if (showDetails.value.has(key)) showDetails.value.delete(key)
  else showDetails.value.add(key)
  showDetails.value = new Set(showDetails.value)
}

// Aggregate per-round rollup: total fear/blight/presence change across the round's phase transitions.
const perRound = computed(() => {
  const byRound = new Map<number, { fear: number; blight: number; invaders: number; presence: number; terror: number; phases: number }>()
  for (const d of deltas.value) {
    const r = d.from.round
    const prev = byRound.get(r) ?? { fear: 0, blight: 0, invaders: 0, presence: 0, terror: 0, phases: 0 }
    prev.fear += d.fear_delta
    prev.blight += d.blight_delta
    prev.invaders += d.invaders_delta
    prev.presence += d.presence_delta
    prev.terror += d.terror_delta
    prev.phases += 1
    byRound.set(r, prev)
  }
  return Array.from(byRound.entries())
    .sort(([a], [b]) => a - b)
    .map(([round, v]) => ({ round, ...v }))
})

const hasData = computed(() => snapshots.value.length > 1)

/** Fetch the deck snapshot for a given (round, phase) pair. Used in the
 *  delta-detail panel to show what R/B/E looked like at that moment. */
function snapshotDecks(round: number, phase: string): DeckSnapshot | undefined {
  const snap = snapshots.value.find(s => s.round === round && s.phase === phase)
  return snap?.data.decks
}
</script>

<template>
  <div class="retrospective card">
    <div class="hdr">
      <h3>Retrospective</h3>
      <span class="subtle">phase-by-phase deltas for post-game self-eval · edit/jump as needed</span>
      <div class="hdr-tools">
        <button
          class="ghost"
          @click="captureCurrent"
          title="Manually capture (or update) a snapshot for the current round + phase"
        >📸 Snapshot now</button>
      </div>
    </div>

    <div v-if="!hasData" class="empty">
      <p>
        Phase snapshots appear here as you advance through the game. Each phase transition (Growth → Fast → Invader → Slow → Time Passes)
        captures the board state so you can later review where the game turned.
      </p>
      <p class="hint">Snapshots captured: {{ snapshots.length }}</p>
    </div>

    <template v-else>
      <!-- Per-round rollup -->
      <div class="round-rollup">
        <div class="rollup-header">
          <span class="col-label">Round</span>
          <span class="col-label num">Fear</span>
          <span class="col-label num">Blight</span>
          <span class="col-label num">Invaders</span>
          <span class="col-label num">Presence</span>
          <span class="col-label num">Terror</span>
        </div>
        <div v-for="r in perRound" :key="r.round" class="rollup-row">
          <span class="round-cell">R{{ r.round }}</span>
          <span class="num" :class="deltaClass(r.fear, 'negative')">{{ deltaStr(r.fear) }}</span>
          <span class="num" :class="deltaClass(r.blight, 'positive')">{{ deltaStr(r.blight) }}</span>
          <span class="num" :class="deltaClass(r.invaders, 'positive')">{{ deltaStr(r.invaders) }}</span>
          <span class="num" :class="deltaClass(r.presence, 'negative')">{{ deltaStr(r.presence) }}</span>
          <span class="num" :class="deltaClass(r.terror, 'negative')">{{ deltaStr(r.terror) }}</span>
        </div>
      </div>

      <!-- Phase-delta detail list -->
      <div class="deltas-list">
        <div class="list-label">Per-phase deltas</div>
        <div
          v-for="(d, i) in deltas"
          :key="i"
          class="delta-row"
          :data-tier="d.terror_delta > 0 ? 'bad' : d.fear_delta > 2 ? 'good' : 'neutral'"
        >
          <div class="delta-hdr" @click="toggleDetails(`${i}`)">
            <span class="arrow" :class="{ open: showDetails.has(String(i)) }">▸</span>
            <span class="delta-round mono">R{{ d.from.round }}</span>
            <span class="delta-phase">{{ d.from.phase }} → {{ d.to.phase }}</span>
            <span class="delta-inline">
              <span class="chip" :class="deltaClass(d.fear_delta, 'negative')">{{ deltaStr(d.fear_delta) }} fear</span>
              <span class="chip" :class="deltaClass(d.blight_delta, 'positive')">{{ deltaStr(d.blight_delta) }} blight</span>
              <span v-if="d.invaders_delta !== 0" class="chip" :class="deltaClass(d.invaders_delta, 'positive')">{{ deltaStr(d.invaders_delta) }} invaders</span>
              <span v-if="d.presence_delta !== 0" class="chip" :class="deltaClass(d.presence_delta, 'negative')">{{ deltaStr(d.presence_delta) }} presence</span>
            </span>
            <div class="row-actions" @click.stop>
              <button
                class="ghost tiny"
                @click="jumpTo(d.to.round, d.to.phase)"
                title="Jump state to this round + phase (for re-entry / corrections)"
              >⤴ jump</button>
              <button
                class="ghost tiny"
                @click="deleteSnapshot(d.to.round, d.to.phase)"
                title="Delete this snapshot — deltas recalculate automatically"
              >×</button>
            </div>
          </div>
          <div v-if="showDetails.has(String(i))" class="delta-detail">
            <div class="breakdown">
              <span class="b-label">Explorers</span><span class="b-val" :class="deltaClass(d.explorers_delta, 'positive')">{{ deltaStr(d.explorers_delta) }}</span>
              <span class="b-label">Towns</span><span class="b-val" :class="deltaClass(d.towns_delta, 'positive')">{{ deltaStr(d.towns_delta) }}</span>
              <span class="b-label">Cities</span><span class="b-val" :class="deltaClass(d.cities_delta, 'positive')">{{ deltaStr(d.cities_delta) }}</span>
              <span class="b-label">Dahan</span><span class="b-val" :class="deltaClass(d.dahan_delta, 'negative')">{{ deltaStr(d.dahan_delta) }}</span>
            </div>
            <ul v-if="d.notable.length" class="notable">
              <li v-for="(n, ni) in d.notable" :key="ni">{{ n }}</li>
            </ul>
            <!-- Deck-state snapshot: what the invader / fear / event decks
                 looked like at the END of this phase transition. Captured
                 during buildSnapshot in TurnController — renders a read-only
                 summary so the user can verify "what was in Build at R7". -->
            <div v-if="snapshotDecks(d.to.round, d.to.phase)" class="deck-snapshot">
              <div class="snapshot-label">Decks at R{{ d.to.round }} · {{ d.to.phase }}</div>
              <div class="snapshot-row">
                <span class="snap-key">Invader R→B→E:</span>
                <span class="snap-val">
                  <span v-if="snapshotDecks(d.to.round, d.to.phase)?.invader?.ravage" class="snap-card rav">
                    R: {{ snapshotDecks(d.to.round, d.to.phase)?.invader?.ravage?.stage === 1 ? 'I' : snapshotDecks(d.to.round, d.to.phase)?.invader?.ravage?.stage === 2 ? 'II' : 'III' }} {{ snapshotDecks(d.to.round, d.to.phase)?.invader?.ravage?.terrain ?? '?' }}
                  </span>
                  <span v-else class="snap-empty">R: —</span>
                  <span v-if="snapshotDecks(d.to.round, d.to.phase)?.invader?.build" class="snap-card bld">
                    B: {{ snapshotDecks(d.to.round, d.to.phase)?.invader?.build?.stage === 1 ? 'I' : snapshotDecks(d.to.round, d.to.phase)?.invader?.build?.stage === 2 ? 'II' : 'III' }} {{ snapshotDecks(d.to.round, d.to.phase)?.invader?.build?.terrain ?? '?' }}
                  </span>
                  <span v-else class="snap-empty">B: —</span>
                  <span v-if="snapshotDecks(d.to.round, d.to.phase)?.invader?.explore" class="snap-card exp">
                    E: {{ snapshotDecks(d.to.round, d.to.phase)?.invader?.explore?.stage === 1 ? 'I' : snapshotDecks(d.to.round, d.to.phase)?.invader?.explore?.stage === 2 ? 'II' : 'III' }} {{ snapshotDecks(d.to.round, d.to.phase)?.invader?.explore?.terrain ?? '?' }}
                  </span>
                  <span v-else class="snap-empty">E: —</span>
                </span>
              </div>
              <div v-if="(snapshotDecks(d.to.round, d.to.phase)?.fear?.resolved?.length ?? 0) + (snapshotDecks(d.to.round, d.to.phase)?.fear?.earned?.length ?? 0) > 0" class="snapshot-row">
                <span class="snap-key">Fear:</span>
                <span class="snap-val">
                  {{ snapshotDecks(d.to.round, d.to.phase)?.fear?.resolved?.length ?? 0 }} resolved
                  + {{ snapshotDecks(d.to.round, d.to.phase)?.fear?.earned?.length ?? 0 }} earned
                  / {{ snapshotDecks(d.to.round, d.to.phase)?.fear?.deck_size ?? 9 }}
                </span>
              </div>
              <div v-if="(snapshotDecks(d.to.round, d.to.phase)?.event?.previewed?.length ?? 0) > 0" class="snapshot-row">
                <span class="snap-key">Previewed:</span>
                <span class="snap-val">
                  {{ snapshotDecks(d.to.round, d.to.phase)?.event?.previewed?.[0]?.name || '(face-down)' }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.retrospective { display: flex; flex-direction: column; gap: var(--sp-3); }

.hdr { display: flex; align-items: baseline; gap: var(--sp-3); flex-wrap: wrap; }
.hdr h3 { color: var(--text-white); margin: 0; }
.hdr .subtle { font-size: var(--fs-xs); color: var(--text-muted); font-style: italic; }
.hdr-tools { margin-left: auto; display: inline-flex; gap: var(--sp-1); }
.row-actions {
  display: inline-flex; gap: 2px; margin-left: var(--sp-2);
  justify-self: end;
}
.tiny { padding: 0 var(--sp-1); font-size: var(--fs-xs); color: var(--text-muted); }
.tiny:hover { color: var(--accent-blue); background: transparent; border: none; }

.empty {
  padding: var(--sp-3);
  background: var(--bg-inset);
  border: 1px dashed var(--border-subtle);
  border-radius: var(--r-md);
  font-size: var(--fs-sm);
  color: var(--text-secondary);
}
.empty .hint {
  margin-top: var(--sp-2); font-size: var(--fs-xs);
  color: var(--text-muted); font-style: italic;
}

.round-rollup {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-2) var(--sp-3);
}
.rollup-header, .rollup-row {
  display: grid;
  grid-template-columns: 3rem repeat(5, 1fr);
  gap: var(--sp-2);
  align-items: center;
  padding: var(--sp-1) 0;
  font-size: var(--fs-sm);
}
.rollup-header { border-bottom: 1px solid var(--border-subtle); }
.col-label {
  font-size: 0.68rem;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
}
.col-label.num { text-align: right; }
.round-cell { font-family: var(--font-mono); color: var(--accent-amber); font-weight: var(--fw-semibold); }
.rollup-row .num { text-align: right; font-family: var(--font-mono); }
.num.positive { color: var(--status-success); }
.num.negative { color: var(--status-danger); }
.num.neutral  { color: var(--text-muted); }

.list-label {
  font-size: 0.68rem;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
}

.deltas-list { display: flex; flex-direction: column; gap: 4px; }

.delta-row {
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  background: var(--bg-inset);
}
.delta-row[data-tier="bad"] { border-left: 3px solid var(--status-danger); }
.delta-row[data-tier="good"] { border-left: 3px solid var(--status-success); }

.delta-hdr {
  display: grid;
  grid-template-columns: auto auto auto 1fr auto;
  gap: var(--sp-2);
  align-items: center;
  padding: var(--sp-2) var(--sp-3);
  cursor: pointer;
  font-size: var(--fs-sm);
}
.delta-hdr:hover { background: var(--bg-muted); }
.arrow { color: var(--text-muted); transition: transform var(--motion-fast); display: inline-block; }
.arrow.open { transform: rotate(90deg); }
.delta-round { color: var(--accent-amber); }
.delta-phase { text-transform: capitalize; color: var(--text-secondary); }
.delta-inline {
  display: inline-flex; gap: 4px; flex-wrap: wrap; justify-content: flex-end;
}

.chip {
  display: inline-flex; padding: 1px 6px;
  font-size: 0.68rem;
  font-family: var(--font-mono);
  border-radius: var(--r-full);
  border: 1px solid var(--border-subtle);
}
.chip.positive { color: var(--status-success); border-color: rgba(82, 183, 136, 0.3); }
.chip.negative { color: var(--status-danger); border-color: rgba(184, 113, 106, 0.3); }
.chip.neutral  { color: var(--text-muted); }

.delta-detail {
  padding: var(--sp-2) var(--sp-3);
  border-top: 1px dashed var(--border-subtle);
  font-size: var(--fs-xs);
  display: flex; flex-direction: column; gap: var(--sp-2);
}
.breakdown {
  display: grid;
  grid-template-columns: repeat(8, auto);
  gap: var(--sp-2) var(--sp-3);
  align-items: center;
}
.b-label { color: var(--text-muted); }
.b-val { font-family: var(--font-mono); text-align: right; }
.notable {
  list-style: none; padding: 0; margin: 0;
  display: flex; flex-direction: column; gap: 2px;
}
.notable li {
  color: var(--text-secondary);
  padding-left: var(--sp-2);
  border-left: 2px solid var(--accent-amber);
}

.deck-snapshot {
  margin-top: var(--sp-2);
  padding: var(--sp-2);
  background: var(--bg-canvas);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  font-size: var(--fs-xs);
}
.snapshot-label {
  font-family: var(--font-mono);
  font-size: 0.68rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  margin-bottom: 4px;
}
.snapshot-row {
  display: grid;
  grid-template-columns: 6rem 1fr;
  gap: var(--sp-2);
  padding: 2px 0;
}
.snap-key { color: var(--text-muted); }
.snap-val { display: inline-flex; gap: 6px; flex-wrap: wrap; }
.snap-card {
  padding: 1px 6px;
  border-radius: var(--r-sm);
  font-family: var(--font-mono);
  font-size: 0.68rem;
  border: 1px solid var(--border-subtle);
}
.snap-card.rav { background: rgba(220, 47, 2, 0.15); color: var(--accent-red); }
.snap-card.bld { background: rgba(233, 196, 106, 0.15); color: var(--accent-amber); }
.snap-card.exp { background: rgba(123, 184, 245, 0.15); color: var(--accent-blue); }
.snap-empty { color: var(--text-muted); opacity: 0.5; }
</style>
