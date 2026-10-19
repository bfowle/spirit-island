<script setup lang="ts">
import { computed } from 'vue'
import type { InvaderPhaseForecast, PublicInvaderDeck, StageCount } from './types'
import Panel from './Panel.vue'
import InvaderCardTile from './InvaderCardTile.vue'
import { invaderLabel, parseInvaderId, plural, prettyText, roman } from './format'

const props = defineProps<{
  invaders: PublicInvaderDeck
  phase: InvaderPhaseForecast
  stages: StageCount[]
}>()

const pos = computed(() => props.phase.position)
const ravageDone = computed(() => pos.value === 'build' || pos.value === 'explore')
const buildDone = computed(() => pos.value === 'explore')
const ex = computed(() => props.phase.explore)
const extra = computed(() => props.invaders.extra_slots.filter(([, cards]) => cards.length > 0))

const exploreSummary = computed(() => {
  const e = ex.value
  if (e.revealed) {
    const n = e.candidates[0]?.lands.length ?? 0
    return `Face up: explores ${plural(n, 'land')}`
  }
  if (e.deck_empty) return 'Deck empty: the spirits lose at the next Explore'
  if (e.next_stage != null) {
    const st = props.stages.find((s) => s.stage === e.next_stage)
    const unseen = st?.unseen.length ?? e.candidates.length
    return `Next card: 1 of ${unseen} unseen Stage ${roman(e.next_stage)} cards`
  }
  return 'No card'
})

const discard = computed(() => props.invaders.discard)
</script>

<template>
  <Panel title="Invader pipeline" question="Which cards act this turn, and which come next?" anchor="panel-pipeline">
    <template #meta>
      <span>Invader Phase of Turn {{ phase.turn }}</span>
      <span v-if="pos !== 'before'" class="now-tag">resolving: {{ pos }}</span>
    </template>

    <div class="pipe">
      <div class="slot discard">
        <div class="slot-head"><span class="slot-name">Discard</span><span class="slot-count">{{ discard.length }}</span></div>
        <div class="cards wrap">
          <InvaderCardTile v-for="c in discard" :key="c.id" :card="c" size="xs" />
          <span v-if="!discard.length" class="empty">none yet</span>
        </div>
      </div>

      <div class="arrow" aria-hidden="true"><span>←</span></div>

      <template v-for="[slot, cards] in extra" :key="slot">
        <div class="slot extra">
          <div class="slot-head"><span class="slot-name">{{ slot }}</span></div>
          <div class="cards"><InvaderCardTile v-for="c in cards" :key="c.id" :card="c" size="md" label /></div>
        </div>
        <div class="arrow" aria-hidden="true"><span>←</span></div>
      </template>

      <div class="slot act ravage" :class="{ done: ravageDone, now: pos === 'ravage' }">
        <div class="slot-head">
          <b class="ord" title="Resolves first in the Invader Phase">1</b>
          <span class="slot-name">Ravage</span>
          <span v-if="ravageDone" class="state">done</span>
        </div>
        <div class="cards">
          <InvaderCardTile v-for="c in phase.ravage_cards" :key="c.id" :card="c" size="md" label />
          <span v-if="!phase.ravage_cards.length" class="empty">empty</span>
        </div>
        <div class="slot-foot">{{ plural(phase.ravages.length, 'land') }} with Invaders match</div>
      </div>

      <div class="arrow" aria-hidden="true"><span>←</span></div>

      <div class="slot act build" :class="{ done: buildDone, now: pos === 'build' }">
        <div class="slot-head">
          <b class="ord" title="Resolves second">2</b>
          <span class="slot-name">Build</span>
          <span v-if="buildDone" class="state">done</span>
        </div>
        <div class="cards">
          <InvaderCardTile v-for="c in phase.build_cards" :key="c.id" :card="c" size="md" label />
          <span v-if="!phase.build_cards.length" class="empty">empty</span>
        </div>
        <div class="slot-foot">{{ plural(phase.builds.length, 'land') }} with Invaders match</div>
      </div>

      <div class="arrow" aria-hidden="true"><span>←</span></div>

      <div class="slot act explore" :class="{ now: pos === 'explore' }">
        <div class="slot-head">
          <b class="ord" title="Resolves third">3</b>
          <span class="slot-name">Explore</span>
        </div>
        <div class="cards">
          <InvaderCardTile v-if="ex.revealed" :card="ex.revealed" size="md" label />
          <template v-else-if="ex.next_stage != null">
            <InvaderCardTile back :stage="ex.next_stage" size="md" />
            <div class="cands" :title="ex.candidates.map((c) => invaderLabel(c.card)).join(', ')">
              <InvaderCardTile v-for="c in ex.candidates" :key="c.card.id" :card="c.card" size="xs" />
            </div>
          </template>
        </div>
        <div class="slot-foot">{{ exploreSummary }}</div>
      </div>

      <div class="arrow" aria-hidden="true"><span>←</span></div>

      <div class="slot deck">
        <div class="slot-head"><span class="slot-name">Deck</span><span class="slot-count">{{ invaders.upcoming_stages.length }} face down</span></div>
        <div class="backs" aria-label="Stages on the card backs, next card first">
          <span v-for="(s, i) in invaders.upcoming_stages" :key="i" class="deck-back" :class="`st${s}`">{{ roman(s) }}</span>
          <span v-if="!invaders.upcoming_stages.length" class="empty">empty</span>
        </div>
        <div class="slot-foot">next card first</div>
      </div>
    </div>

    <p class="caption">
      Each card moves one slot left per turn: Explore, then Build, then Ravage, then the discard.
      Inside one Invader Phase the slots resolve left to right: Ravage, then Build, then Explore.
    </p>

    <div class="counting">
      <div v-for="st in stages" :key="st.stage" class="stage-row">
        <span class="st-name">Stage {{ roman(st.stage) }}</span>
        <span class="st-nums">
          <b>{{ st.unseen.length }}</b> unseen
          <span class="sep">/</span>
          <b>{{ st.left_in_deck }}</b> in deck
          <span class="sep">/</span>
          {{ st.seen.length }} seen
        </span>
        <span class="st-cards">
          <InvaderCardTile v-for="c in st.unseen" :key="c.id" :card="c" size="xs" />
          <span v-if="st.seen.length" class="seen-sep" aria-hidden="true"></span>
          <InvaderCardTile v-for="id in st.seen" :key="id" :card="parseInvaderId(id)" size="xs" dim />
        </span>
      </div>
    </div>

    <ul v-if="phase.notes.length" class="notes">
      <li v-for="(n, i) in phase.notes" :key="i">{{ prettyText(n) }}</li>
    </ul>
  </Panel>
</template>

<style scoped>
.pipe {
  display: flex;
  align-items: stretch;
  gap: 0;
  overflow-x: auto;
  padding-bottom: 2px;
}
.slot {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 8px 10px;
  border-radius: 8px;
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  min-width: 0;
}
.slot.act { border-top-width: 3px; }
.slot.ravage { border-top-color: var(--accent-red); }
.slot.build { border-top-color: #fb923c; }
.slot.explore { border-top-color: var(--accent-teal); }
.slot.extra { border-top: 3px solid var(--accent-purple); }
.slot.done { opacity: 0.5; }
.slot.now { box-shadow: 0 0 0 1px var(--accent-blue-hover); }
.slot.discard { flex: 0 1 150px; }
.slot.deck { flex: 0 1 220px; }
.slot.act { flex: 1 1 0; }
.slot-head { display: flex; align-items: center; gap: 6px; }
.slot-name { font-family: var(--font-brand); font-size: 15px; color: var(--text-white); }
.slot-count { font-size: 11px; color: var(--text-muted); margin-left: auto; }
.state { margin-left: auto; font-size: 11px; color: var(--text-secondary); }
.ord {
  display: inline-grid;
  place-items: center;
  width: 18px; height: 18px;
  border-radius: 50%;
  font-size: 11px;
  font-weight: 700;
  color: var(--text-primary);
  background: var(--bg-active);
}
.cards { display: flex; align-items: center; gap: 8px; min-height: 62px; }
.cards.wrap { flex-wrap: wrap; gap: 3px; min-height: 0; }
.cands { display: flex; flex-wrap: wrap; gap: 3px; max-width: 110px; }
.slot-foot { font-size: 12px; color: var(--text-secondary); }
.empty { font-size: 12px; color: var(--text-muted); }
.arrow { display: flex; align-items: center; padding: 0 4px; color: var(--text-muted); font-size: 16px; }
.backs { display: flex; flex-wrap: wrap; gap: 3px; }
.deck-back {
  display: inline-grid;
  place-items: center;
  min-width: 22px; height: 30px;
  padding: 0 3px;
  border-radius: 3px;
  font-family: var(--font-brand);
  font-size: 11px;
  color: var(--text-secondary);
  background: linear-gradient(160deg, hsl(222 35% 22%), hsl(222 40% 14%));
  border: 1px solid var(--border-strong);
}
.deck-back:first-child { border-color: var(--accent-teal); color: var(--text-primary); }
.deck-back.st3 { background: linear-gradient(160deg, hsl(258 30% 26%), hsl(258 35% 16%)); }
.caption { margin: 8px 0 0; font-size: 12px; color: var(--text-muted); }
.counting { display: flex; flex-wrap: wrap; gap: 6px 22px; margin-top: 8px; padding-top: 8px; border-top: 1px solid var(--border-subtle); }
.stage-row { display: flex; align-items: center; gap: 8px; font-size: 12px; color: var(--text-secondary); }
.st-name { font-weight: 600; color: var(--text-primary); }
.st-nums b { color: var(--text-primary); font-variant-numeric: tabular-nums; }
.sep { color: var(--text-faint); margin: 0 2px; }
.st-cards { display: inline-flex; gap: 2px; align-items: center; }
.seen-sep { width: 1px; height: 18px; background: var(--border-strong); margin: 0 3px; }
.notes { margin: 8px 0 0; padding: 6px 10px 6px 26px; border-radius: 6px; background: var(--info-soft); font-size: 12px; color: var(--text-primary); }
.notes li + li { margin-top: 2px; }
.now-tag { color: var(--accent-blue-hover); font-weight: 600; }
</style>
