<script setup lang="ts">
import { computed, ref } from 'vue'
import type { GameReview } from './types'
import Panel from './Panel.vue'
import LandChip from './LandChip.vue'
import InvaderCardTile from './InvaderCardTile.vue'
import Icon from '../components/Icon.vue'
import { endingLabel, invaderLabel, parseInvaderId, prettyText, stepLabel } from './format'

const props = defineProps<{ review: GameReview }>()
const R = computed(() => props.review)

// ── Blight attribution: one square per blight, stacked by turn, colored by cause.
// Colors: the first three validated categorical slots (all-pairs safe); the rest fold to "Other".
const CAUSES: { key: string; label: string; color: string }[] = [
  { key: 'Ravage', label: 'Ravage', color: '#d95926' },
  { key: 'cascade', label: 'Cascade', color: '#3987e5' },
  { key: 'Event', label: 'Event', color: '#199e70' },
]
const OTHER = { key: 'other', label: 'Other step', color: '#64748b' }
function causeOf(c: string) { return CAUSES.find((x) => x.key === c) ?? OTHER }
const causeTotals = computed(() => {
  const m = new Map<string, number>()
  for (const e of R.value.blight) {
    const k = causeOf(e.cause).key
    m.set(k, (m.get(k) ?? 0) + e.added)
  }
  return [...CAUSES, OTHER].filter((c) => m.has(c.key)).map((c) => ({ ...c, n: m.get(c.key)! }))
})
const turnsAll = computed(() => Array.from({ length: Math.max(R.value.turns, 1) }, (_, i) => i + 1))
const blightByTurn = computed(() =>
  turnsAll.value.map((t) => {
    const sq: { color: string; title: string }[] = []
    for (const e of R.value.blight.filter((x) => x.turn === t)) {
      const c = causeOf(e.cause)
      for (let k = 0; k < e.added; k++) sq.push({ color: c.color, title: `${prettyText(e.land)}: ${c.label} (${stepLabel(e.step)} step)` })
    }
    return { t, sq }
  }),
)
const maxSq = computed(() => Math.max(1, ...blightByTurn.value.map((x) => x.sq.length)))

// ── Pressure line.
const PW = 420, PH = 150, pl = 30, pr = 12, pt = 18, pb = 22
const pmax = computed(() => Math.max(1, ...R.value.pressure.map(([, v]) => v)))
const pts = computed(() => {
  const n = R.value.pressure.length
  return R.value.pressure.map(([t, v], i) => ({
    t, v,
    x: pl + (n <= 1 ? (PW - pl - pr) / 2 : (i / (n - 1)) * (PW - pl - pr)),
    y: pt + (PH - pt - pb) * (1 - v / pmax.value),
  }))
})
const linePath = computed(() => pts.value.map((p, i) => `${i ? 'L' : 'M'}${p.x},${p.y}`).join(' '))
const jump = computed(() => {
  const j = R.value.largest_jump
  if (!j) return null
  const i = pts.value.findIndex((p) => p.t === j[0])
  if (i <= 0) return null
  return { a: pts.value[i - 1], b: pts.value[i], d: j[1], t: j[0] }
})
const hoverP = ref<number | null>(null)

// ── Flow: Invaders added vs removed, per turn, by step.
const flowRows = computed(() => R.value.flow.map((f) => ({
  turn: f.turn,
  add: Object.values(f.added).reduce((s, v) => s + v, 0),
  rem: Object.values(f.removed).reduce((s, v) => s + v, 0),
})))
const flowMax = computed(() => Math.max(1, ...flowRows.value.flatMap((r) => [r.add, r.rem])))
const FW = 420, FH = 160, fpl = 30, fpr = 8
const fBand = computed(() => (FW - fpl - fpr) / Math.max(1, flowRows.value.length))
const fColW = computed(() => Math.min(24, fBand.value * 0.5))
const mid = FH / 2
const fScale = computed(() => (FH / 2 - 18) / flowMax.value)
function upPath(x: number, h: number) {
  if (h <= 0) return ''
  const r = Math.min(4, h), w = fColW.value, y = mid - h
  return `M${x},${mid - 1} V${y + r} Q${x},${y} ${x + r},${y} H${x + w - r} Q${x + w},${y} ${x + w},${y + r} V${mid - 1} Z`
}
function downPath(x: number, h: number) {
  if (h <= 0) return ''
  const r = Math.min(4, h), w = fColW.value, y = mid + h
  return `M${x},${mid + 1} V${y - r} Q${x},${y} ${x + r},${y} H${x + w - r} Q${x + w},${y} ${x + w},${y - r} V${mid + 1} Z`
}
const hoverF = ref<number | null>(null)
const flowSteps = computed(() => {
  const add = new Set<string>(), rem = new Set<string>()
  for (const f of R.value.flow) {
    Object.keys(f.added).forEach((k) => add.add(k))
    Object.keys(f.removed).forEach((k) => rem.add(k))
  }
  return { add: [...add], rem: [...rem] }
})
const ADD_C = '#d95926'
const REM_C = '#3987e5'
</script>

<template>
  <div class="review">
    <div class="rv-head">
      <h2 class="rv-title">Game review</h2>
      <p class="rv-sub">
        {{ R.turns }} turns<template v-if="R.ending">; ended: {{ endingLabel(R.ending) }}</template><template v-else>; the game is still running, so this covers the turns so far</template>.
        Facts and questions, not a grade.
      </p>
    </div>

    <div class="rv-grid">
      <div class="rv-stack">
        <Panel title="Where the blight came from" question="Each square is one blight, by turn and cause">
          <div class="units">
            <div v-for="col in blightByTurn" :key="col.t" class="u-col">
              <div class="u-stack" :style="{ height: `${maxSq * 16}px` }">
                <i v-for="(s, k) in col.sq" :key="k" :style="{ background: s.color }" :title="s.title" tabindex="0"></i>
              </div>
              <span class="u-t">{{ col.t }}</span>
            </div>
          </div>
          <div class="legend">
            <span v-for="c in causeTotals" :key="c.key"><i :style="{ background: c.color }"></i>{{ c.label }} <b>{{ c.n }}</b></span>
          </div>
          <table class="tbl">
            <thead><tr><th>Turn</th><th>Step</th><th>Land</th><th>Cause</th><th>Added</th></tr></thead>
            <tbody>
              <tr v-for="(e, i) in R.blight" :key="i">
                <td>{{ e.turn }}</td><td>{{ stepLabel(e.step) }}</td><td><LandChip :id="e.land" /></td><td>{{ causeOf(e.cause).label }}</td><td>{{ e.added }}</td>
              </tr>
            </tbody>
          </table>
        </Panel>
        <Panel title="Turn notes" question="What was played, and what was left unused">
          <table class="tbl">
            <thead><tr><th>Turn</th><th>Played</th><th>Energy left</th><th>Plays left</th></tr></thead>
            <tbody>
              <tr v-for="n in R.turn_notes" :key="n.turn">
                <td>{{ n.turn }}</td>
                <td class="played">
                  {{ n.played.join(', ') || '—' }}
                  <details v-if="n.decisions?.length" class="decs">
                    <summary>{{ n.decisions.length }} decisions</summary>
                    <ol><li v-for="(d, k) in n.decisions" :key="k">{{ prettyText(d) }}</li></ol>
                  </details>
                </td>
                <td>{{ n.energy_left }}</td>
                <td>{{ n.plays_left }}</td>
              </tr>
            </tbody>
          </table>
        </Panel>
      </div>
      <div class="rv-stack">
        <Panel title="Invader pressure" question="Total Ravage damage the island's Invaders could deal, end of each turn">
          <div class="chart-wrap">
            <svg :viewBox="`0 0 ${PW} ${PH}`" class="svg" role="img" :aria-label="R.pressure.map(([t, v]) => `Turn ${t}: ${v}`).join('; ')">
              <line :x1="pl" :x2="PW - pr" :y1="pt" :y2="pt" class="grid" />
              <text :x="pl - 5" :y="pt + 3" text-anchor="end" class="tick">{{ pmax }}</text>
              <line :x1="pl" :x2="PW - pr" :y1="PH - pb" :y2="PH - pb" class="axis" />
              <text :x="pl - 5" :y="PH - pb + 3" text-anchor="end" class="tick">0</text>
              <line v-if="jump" :x1="jump.a.x" :y1="jump.a.y" :x2="jump.b.x" :y2="jump.b.y" class="jump" />
              <path :d="linePath" class="line" />
              <g v-for="(p, i) in pts" :key="p.t" @pointerenter="hoverP = i" @pointerleave="hoverP = null" @focus="hoverP = i" @blur="hoverP = null" tabindex="0" class="pt">
                <rect :x="p.x - 16" :y="pt" width="32" :height="PH - pt" class="hit" />
                <circle :cx="p.x" :cy="p.y" r="4.5" class="dot" />
                <text :x="p.x" :y="PH - 6" text-anchor="middle" class="tick">{{ p.t }}</text>
              </g>
              <text v-if="jump" :x="(jump.a.x + jump.b.x) / 2" :y="Math.min(jump.a.y, jump.b.y) - 8" text-anchor="middle" class="ann">+{{ jump.d }} on Turn {{ jump.t }}</text>
              <text v-if="pts.length" :x="pts[pts.length - 1].x" :y="pts[pts.length - 1].y - 10" text-anchor="end" class="endl">{{ pts[pts.length - 1].v }}</text>
            </svg>
            <div v-if="hoverP != null" class="tip" :style="{ left: `${(pts[hoverP].x / PW) * 100}%` }">
              <b>{{ pts[hoverP].v }}</b> damage at the end of Turn {{ pts[hoverP].t }}
            </div>
          </div>
          <p class="note">The highlighted segment is the largest turn-over-turn jump.</p>
        </Panel>
        <Panel title="Invaders added and removed" question="Did removal keep pace with arrival, and in which step?">
          <div class="chart-wrap">
            <svg :viewBox="`0 0 ${FW} ${FH}`" class="svg" role="img" :aria-label="flowRows.map((r) => `Turn ${r.turn}: +${r.add}, -${r.rem}`).join('; ')">
              <line :x1="fpl" :x2="FW - fpr" :y1="mid" :y2="mid" class="axis" />
              <text :x="fpl - 5" :y="18" text-anchor="end" class="tick">+{{ flowMax }}</text>
              <text :x="fpl - 5" :y="FH - 12" text-anchor="end" class="tick">−{{ flowMax }}</text>
              <g v-for="(r, i) in flowRows" :key="r.turn" tabindex="0" class="pt" @pointerenter="hoverF = i" @pointerleave="hoverF = null" @focus="hoverF = i" @blur="hoverF = null">
                <rect :x="fpl + i * fBand" y="0" :width="fBand" :height="FH" class="hit" />
                <path :d="upPath(fpl + i * fBand + (fBand - fColW) / 2, r.add * fScale)" :fill="ADD_C" />
                <path :d="downPath(fpl + i * fBand + (fBand - fColW) / 2, r.rem * fScale)" :fill="REM_C" />
                <text :x="fpl + i * fBand + fBand / 2" :y="FH - 1" text-anchor="middle" class="tick">{{ r.turn }}</text>
              </g>
            </svg>
            <div v-if="hoverF != null" class="tip" :style="{ left: `${((fpl + hoverF * fBand + fBand / 2) / FW) * 100}%` }">
              Turn {{ flowRows[hoverF].turn }}: <b>+{{ flowRows[hoverF].add }}</b> added, <b>−{{ flowRows[hoverF].rem }}</b> removed
            </div>
          </div>
          <div class="legend">
            <span><i :style="{ background: ADD_C }"></i>Added (up)</span>
            <span><i :style="{ background: REM_C }"></i>Removed (down)</span>
          </div>
          <table class="tbl">
            <thead><tr><th>Step</th><th v-for="f in R.flow" :key="f.turn">T{{ f.turn }}</th></tr></thead>
            <tbody>
              <tr v-for="s in flowSteps.add" :key="`a-${s}`">
                <th scope="row">+ {{ stepLabel(s) }}</th>
                <td v-for="f in R.flow" :key="f.turn">{{ f.added[s] ? `+${f.added[s]}` : '' }}</td>
              </tr>
              <tr v-for="s in flowSteps.rem" :key="`r-${s}`">
                <th scope="row">− {{ stepLabel(s) }}</th>
                <td v-for="f in R.flow" :key="f.turn">{{ f.removed[s] ? `−${f.removed[s]}` : '' }}</td>
              </tr>
            </tbody>
          </table>
        </Panel>
        <Panel title="Reflections" question="Questions to ask yourself about this game">
          <ul class="refl">
            <li v-for="(r, i) in R.reflections" :key="i">
              <p class="q">{{ r.question }}</p>
              <ul><li v-for="(f, k) in r.facts" :key="k">{{ prettyText(f) }}</li></ul>
            </li>
          </ul>
        </Panel>
      </div>
      <div class="rv-stack">
        <Panel title="Ravage chains" question="For blight from a Ravage: the same card's Explore and Build in that land, and what stood there at each point">
          <div v-for="(c, i) in R.chains" :key="i" class="chain">
            <div class="ch-h">
              <LandChip :id="c.land" />
              <InvaderCardTile :card="parseInvaderId(c.card)" size="xs" />
              <span>{{ invaderLabel(parseInvaderId(c.card)) }}, Ravaged on Turn {{ c.ravaged_turn }}</span>
            </div>
            <ol class="steps">
              <li v-for="(p, k) in c.points" :key="k" class="stp">
                <span class="when">{{ prettyText(p.when) }}</span>
                <span class="pcs">
                  <span v-if="p.explorers"><Icon name="unit-explorer" :size="12" decorative />{{ p.explorers }}</span>
                  <span v-if="p.towns"><Icon name="unit-town" :size="13" decorative />{{ p.towns }}</span>
                  <span v-if="p.cities"><Icon name="unit-city" :size="14" decorative />{{ p.cities }}</span>
                  <span v-if="!p.explorers && !p.towns && !p.cities" class="muted">no Invaders</span>
                </span>
                <span class="dmg">{{ p.damage }} damage; {{ p.health_to_clear }} health to clear</span>
              </li>
            </ol>
            <div class="acts">
              <span class="acts-l">Your actions on this land while it built</span>
              <ul v-if="c.actions_here?.length"><li v-for="(a, k) in c.actions_here" :key="k">{{ prettyText(a) }}</li></ul>
              <p v-else class="none">No action on this land.</p>
            </div>
          </div>
          <p v-if="!R.chains.length" class="note">No blight from a Ravage yet.</p>
        </Panel>
      </div>
    </div>
  </div>
</template>

<style scoped>
.review { display: flex; flex-direction: column; gap: 12px; }
.rv-head { padding: 4px 2px; }
.rv-title { font-family: var(--font-brand); font-weight: 500; font-size: 24px; color: var(--text-white); margin: 0; }
.rv-sub { margin: 2px 0 0; font-size: 13px; color: var(--text-secondary); }
.rv-grid { display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1.3fr); gap: 12px; align-items: start; }
.rv-stack { display: flex; flex-direction: column; gap: 12px; min-width: 0; }
@media (max-width: 1599px) { .rv-grid { grid-template-columns: minmax(0, 1fr) minmax(0, 1fr); } .rv-stack:last-child { grid-column: 1 / -1; } }
.units { display: flex; gap: 14px; align-items: flex-end; padding: 4px 0 2px; }
.u-col { display: flex; flex-direction: column; align-items: center; gap: 4px; }
.u-stack { display: flex; flex-direction: column-reverse; gap: 2px; justify-content: flex-start; }
.u-stack i { display: block; width: 14px; height: 14px; border-radius: 2px; }
.u-stack i:focus { outline: 2px solid var(--text-white); outline-offset: 1px; }
.u-t { font-size: 11px; color: var(--text-muted); }
.legend { display: flex; gap: 12px; flex-wrap: wrap; font-size: 12px; color: var(--text-secondary); margin: 6px 0; }
.legend i { display: inline-block; width: 10px; height: 10px; border-radius: 2px; margin-right: 4px; vertical-align: -1px; }
.legend b { color: var(--text-primary); }
.tbl { width: 100%; border-collapse: collapse; font-size: 12px; font-variant-numeric: tabular-nums; margin-top: 6px; }
.tbl th { text-align: left; font-weight: 500; color: var(--text-muted); font-size: 11px; padding: 2px 6px; border-bottom: 1px solid var(--border-subtle); }
.tbl tbody th { color: var(--text-secondary); font-size: 12px; border-bottom: 0; white-space: nowrap; }
.tbl td { padding: 2px 6px; color: var(--text-primary); }
.played { color: var(--text-secondary); }
.chart-wrap { position: relative; }
.svg { display: block; width: 100%; height: auto; overflow: visible; }
.grid { stroke: var(--border-subtle); stroke-width: 1; }
.axis { stroke: var(--border-strong); stroke-width: 1; }
.tick { fill: var(--text-muted); font-size: 10px; font-variant-numeric: tabular-nums; }
.line { fill: none; stroke: #3987e5; stroke-width: 2; stroke-linejoin: round; stroke-linecap: round; }
.jump { stroke: #d95926; stroke-width: 6; stroke-linecap: round; opacity: 0.45; }
.dot { fill: #3987e5; stroke: var(--bg-surface); stroke-width: 2; }
.ann { fill: var(--text-primary); font-size: 11px; font-weight: 600; }
.endl { fill: var(--text-primary); font-size: 11px; font-weight: 600; }
.hit { fill: transparent; }
.pt:focus { outline: none; }
.pt:focus .hit, .pt:hover .hit { fill: rgba(148, 163, 184, 0.07); }
.tip { position: absolute; top: 0; transform: translate(-50%, -100%); background: var(--bg-raised); border: 1px solid var(--border-strong); border-radius: 6px; padding: 4px 8px; font-size: 11.5px; white-space: nowrap; pointer-events: none; color: var(--text-secondary); z-index: 5; }
.tip b { color: var(--text-white); }
.note { margin: 6px 0 0; font-size: 11.5px; color: var(--text-muted); }
.chain { padding: 6px 0 8px; border-bottom: 1px solid var(--border-subtle); }
.chain:last-child { border-bottom: 0; }
.ch-h { display: flex; align-items: center; gap: 6px; font-size: 12.5px; color: var(--text-primary); margin-bottom: 4px; }
.steps { list-style: none; margin: 0; padding: 0; display: grid; grid-auto-flow: column; grid-auto-columns: minmax(0, 1fr); gap: 10px; }
.stp { display: flex; flex-direction: column; gap: 2px; padding: 5px 8px; border-radius: 6px; background: var(--bg-inset); font-size: 11.5px; position: relative; }
.stp + .stp::before { content: '→'; position: absolute; left: -5px; top: 50%; transform: translate(-50%, -50%); color: var(--text-faint); font-size: 11px; }
.when { color: var(--text-secondary); }
.pcs { display: flex; gap: 6px; align-items: center; font-weight: 600; font-size: 12.5px; }
.pcs span { display: inline-flex; align-items: center; gap: 2px; }
.dmg { color: var(--text-muted); }
.muted { color: var(--text-muted); font-weight: 400; }
.acts { margin-top: 6px; font-size: 12px; }
.acts-l { color: var(--text-muted); font-size: 11px; }
.acts ul { margin: 2px 0 0; padding-left: 16px; color: var(--text-primary); }
.acts .none { margin: 2px 0 0; color: var(--text-secondary); }
.decs { margin-top: 3px; font-size: 11.5px; }
.decs summary { cursor: pointer; color: var(--text-muted); }
.decs ol { margin: 3px 0 0; padding-left: 18px; color: var(--text-secondary); }
.tbl td { vertical-align: top; }
.refl { margin: 0; padding-left: 0; list-style: none; display: flex; flex-direction: column; gap: 8px; }
.refl .q { margin: 0 0 2px; font-weight: 600; color: var(--text-white); font-size: 13px; }
.refl ul { margin: 0; padding-left: 14px; font-size: 12px; color: var(--text-secondary); }
</style>
