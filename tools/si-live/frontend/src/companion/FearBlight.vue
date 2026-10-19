<script setup lang="ts">
import { computed, ref } from 'vue'
import type { BlightStats, FearStats, PublicBlight, PublicFear } from './types'
import Panel from './Panel.vue'
import Pips from './Pips.vue'
import LandChip from './LandChip.vue'
import Icon from '../components/Icon.vue'
import { plainRule, prettyText, roman } from './format'

const props = defineProps<{ fear: FearStats; blight: BlightStats; pFear: PublicFear; pBlight: PublicBlight }>()

const F = computed(() => props.fear)
const B = computed(() => props.blight)
const levels = computed(() => F.value.cards_left_by_level.map((n, i) => ({ lvl: i + 1, n })))

// Flipped-side distribution: one column per value.
const dist = computed(() => B.value.flipped_side)
const distTotal = computed(() => dist.value.reduce((s, [, c]) => s + c, 0))
const distMax = computed(() => Math.max(1, ...dist.value.map(([, c]) => c)))
const CH = 70
const colW = 22
const colGap = 14
const chartW = computed(() => Math.max(1, dist.value.length) * (colW + colGap) + colGap)
const hover = ref<number | null>(null)
/** Column with a 4px rounded data end, square at the baseline. */
function barPath(i: number, c: number): string {
  const h = Math.max(2, (c / distMax.value) * CH)
  const x = colGap + i * (colW + colGap)
  const base = CH + 14
  const y = base - h
  const r = Math.min(4, h)
  return `M${x},${base} V${y + r} Q${x},${y} ${x + r},${y} H${x + colW - r} Q${x + colW},${y} ${x + colW},${y + r} V${base} Z`
}
</script>

<template>
  <Panel title="Fear and blight" question="How far is each clock from its end?" anchor="panel-fear">
    <div class="fb-wrap"><div class="halves">
      <section class="half" aria-label="Fear">
        <h3 class="sub"><Icon name="resource-fear" :size="15" decorative /> Fear</h3>

        <div class="tl">
          <div class="tl-now">
            <span class="tl-l">Now at</span>
            <span class="tl-n">Terror Level <b>{{ roman(F.terror_level) }}</b></span>
          </div>
          <div class="tl-deck" aria-label="Fear cards left in the deck by Terror Level">
            <div v-for="l in levels" :key="l.lvl" class="tl-col" :class="{ cur: l.lvl === F.terror_level, past: l.lvl < F.terror_level }">
              <span class="stack">
                <i v-for="k in l.n" :key="k"></i>
                <em v-if="!l.n">0</em>
              </span>
              <span class="tl-lab">{{ roman(l.lvl) }}<template v-if="l.n">: {{ l.n }}</template></span>
            </div>
          </div>
        </div>

        <div class="line">
          <Pips :hit="F.generated" :total="F.required_per_card" tone="fear" :size="9" />
          <span><b>{{ F.generated }}</b> of {{ F.required_per_card }} toward the next card ({{ F.to_next_card }} to go)</span>
        </div>
        <dl class="kv">
          <dt>Next Terror Level</dt>
          <dd><template v-if="F.to_next_level != null"><b>{{ F.to_next_level }}</b> fear</template><template v-else>at the last level</template></dd>
          <dt>Fear victory</dt>
          <dd><b>{{ F.to_clear_deck }}</b> fear empties the Fear deck (an immediate win)</dd>
          <dt>Earned, face down</dt>
          <dd><b>{{ F.earned_banked }}</b> {{ F.earned_banked === 1 ? 'card' : 'cards' }}</dd>
          <dt>Counterattacks</dt>
          <dd>up to <b>{{ F.counterattack_fear_best }}</b> fear this phase</dd>
          <dt>Victory now</dt>
          <dd>{{ F.victory_condition }}: <b>{{ F.victory_blockers }}</b> still on the island</dd>
        </dl>
        <p class="pocket">{{ prettyText(F.pocket_rule) }}</p>
        <p v-if="pFear.discard.length || pFear.removed_unseen" class="small">
          Resolved: {{ pFear.discard.map((c) => c.title).join(', ') || 'none' }}<template v-if="pFear.removed_unseen">; {{ pFear.removed_unseen }} removed unseen</template>
        </p>
      </section>

      <section class="half" aria-label="Blight">
        <h3 class="sub"><Icon name="resource-blight" :size="15" decorative /> Blight</h3>
        <div class="b-top">
          <div class="b-num">
            <span class="b-n">{{ B.remaining }}</span>
            <span class="b-l">left on the card</span>
          </div>
          <div class="b-num">
            <span class="b-n">{{ B.on_island }}</span>
            <span class="b-l">on the island</span>
          </div>
          <div class="side" :class="{ flipped: B.flipped }">
            <template v-if="B.flipped">Blighted: {{ pBlight.blighted_card?.title ?? 'flipped' }}</template>
            <template v-else>Healthy side up</template>
          </div>
        </div>
        <div class="line">
          <span>This phase if nothing changes: at least <b>{{ B.adds_this_phase_min }}</b>, so the <b>{{ B.outcome_this_phase }}</b></span>
        </div>

        <figure v-if="!B.flipped && dist.length" class="dist">
          <figcaption>If the card flips, the blighted side adds (unseen cards, {{ distTotal }} in all)</figcaption>
          <svg :width="chartW" :height="CH + 34" role="img" :aria-label="dist.map(([v, c]) => `${v} blight: ${c} of ${distTotal} cards`).join('; ')">
            <line :x1="0" :x2="chartW" :y1="CH + 14" :y2="CH + 14" class="base" />
            <g
              v-for="([v, c], i) in dist"
              :key="v"
              class="col"
              :class="{ hov: hover === i }"
              tabindex="0"
              @pointerenter="hover = i"
              @pointerleave="hover = null"
              @focus="hover = i"
              @blur="hover = null"
            >
              <rect :x="colGap + i * (colW + colGap) - colGap / 2" y="0" :width="colW + colGap" :height="CH + 34" class="hit" />
              <path
                :d="barPath(i, c)"
                class="bar"
              />
              <text :x="colGap + i * (colW + colGap) + colW / 2" :y="CH + 14 - Math.max(2, (c / distMax) * CH) - 4" text-anchor="middle" class="cap">{{ c }}</text>
              <text :x="colGap + i * (colW + colGap) + colW / 2" :y="CH + 28" text-anchor="middle" class="xl">{{ v }}</text>
              <title>{{ c }} of {{ distTotal }} unseen blight cards add {{ v }}</title>
            </g>
          </svg>
          <p class="small">{{ hover != null ? `${dist[hover][1]} of ${distTotal} cards add ${dist[hover][0]} blight` : 'Bottom: blight added. Top: how many cards.' }}</p>
          <p v-if="B.flipped_still_healthy" class="small">{{ B.flipped_still_healthy }} of {{ distTotal }} are Still-Healthy: when that side empties, draw a new Blight Card instead of losing.</p>
        </figure>

        <div v-if="B.cascade_chains.length" class="chains">
          <div class="chains-h">Cascades: if blight lands again where blight already is</div>
          <div v-for="c in B.cascade_chains" :key="c.land" class="chain">
            <LandChip :id="c.land" />
            <span>+{{ c.min_extra }} more at least</span>
            <span class="stop">
              <template v-if="c.safe_neighbors.length">stops at <LandChip v-for="n in c.safe_neighbors" :key="n" :id="n" /></template>
              <template v-else>no unblighted neighbor</template>
            </span>
          </div>
        </div>
        <p class="pocket">{{ prettyText(B.pocket_rule) }}</p>
        <p class="small rule">{{ (B.flipped ? pBlight.blighted_text : pBlight.healthy_side).map(plainRule).join(' ') }}</p>
      </section>
    </div></div>
  </Panel>
</template>

<style scoped>
.fb-wrap { container-type: inline-size; }
.halves { display: grid; grid-template-columns: minmax(0, 1fr); gap: 14px; }
.half + .half { border-top: 1px solid var(--border-subtle); padding-top: 12px; }
@container (min-width: 640px) {
  .halves { grid-template-columns: minmax(0, 1fr) minmax(0, 1fr); gap: 18px; }
  .half + .half { border-top: 0; padding-top: 0; border-left: 1px solid var(--border-subtle); padding-left: 18px; }
}
.sub { display: flex; align-items: center; gap: 6px; font-family: var(--font-brand); font-weight: 500; font-size: 15px; color: var(--text-white); margin: 0 0 8px; }
.tl { display: flex; gap: 14px; align-items: flex-end; margin-bottom: 8px; }
.tl-now { display: flex; flex-direction: column; line-height: 1; }
.tl-l { font-size: 11px; color: var(--text-muted); margin-bottom: 4px; }
.tl-n { font-size: 17px; font-weight: 600; color: var(--text-white); white-space: nowrap; }
.tl-n b { color: var(--accent-amber); letter-spacing: 0.06em; }
.tl-deck { display: flex; gap: 10px; align-items: flex-end; }
.tl-col { display: flex; flex-direction: column; align-items: center; gap: 3px; }
.stack { display: flex; flex-direction: column-reverse; gap: 2px; min-height: 12px; justify-content: flex-start; }
.stack i { display: block; width: 26px; height: 6px; border-radius: 2px; background: var(--bg-active); border: 1px solid var(--border-strong); }
.tl-col.cur .stack i { background: rgba(245, 158, 11, 0.35); border-color: var(--accent-amber); }
.stack em { font-style: normal; font-size: 11px; color: var(--text-faint); }
.tl-lab { font-size: 11px; color: var(--text-secondary); }
.tl-col.past .tl-lab { color: var(--text-faint); }
.line { display: flex; align-items: center; gap: 8px; font-size: 12.5px; color: var(--text-primary); margin: 4px 0 6px; flex-wrap: wrap; }
.kv { display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 2px 10px; margin: 0; font-size: 12.5px; }
.kv dt { color: var(--text-muted); }
.kv dd { margin: 0; color: var(--text-primary); }
b { font-variant-numeric: tabular-nums; }
.pocket { margin: 8px 0 0; padding: 5px 8px; border-radius: 6px; background: var(--bg-inset); font-size: 12px; color: var(--text-secondary); border-left: 2px solid var(--accent-amber); }
.half:last-child .pocket { border-left-color: #a78bfa; }
.small { margin: 6px 0 0; font-size: 11.5px; color: var(--text-muted); }
.b-top { display: flex; gap: 16px; align-items: flex-end; margin-bottom: 6px; }
.b-num { display: flex; flex-direction: column; line-height: 1.1; }
.b-n { font-family: var(--font-brand); font-size: 30px; font-weight: 600; color: #c4b5fd; }
.b-l { font-size: 11px; color: var(--text-muted); }
.side { margin-left: auto; font-size: 12px; padding: 2px 8px; border-radius: 4px; background: var(--bg-raised); color: var(--text-secondary); }
.side.flipped { background: rgba(124, 58, 237, 0.2); color: #ddd6fe; }
.dist { margin: 6px 0 0; }
.dist figcaption { font-size: 12px; color: var(--text-secondary); margin-bottom: 4px; }
.dist .base { stroke: var(--border-strong); stroke-width: 1; }
.dist .bar { fill: #9085e9; }
.dist .col.hov .bar, .dist .col:focus .bar { fill: #b3abf0; }
.dist .col:focus { outline: none; }
.dist .hit { fill: transparent; }
.dist .cap { fill: var(--text-primary); font-size: 11px; font-weight: 600; }
.dist .xl { fill: var(--text-secondary); font-size: 11px; }
.chains { margin-top: 8px; }
.chains-h { font-size: 12px; color: var(--text-secondary); margin-bottom: 4px; }
.chain { display: flex; flex-wrap: wrap; align-items: center; gap: 6px; font-size: 12px; color: var(--text-primary); padding: 2px 0; }
.stop { display: inline-flex; gap: 3px; align-items: center; flex-wrap: wrap; color: var(--text-muted); }
.rule { font-style: italic; }
</style>
