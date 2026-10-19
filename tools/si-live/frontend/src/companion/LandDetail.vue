<script setup lang="ts">
import { computed } from 'vue'
import type { BuildForecast, CascadeChain, ExploreCandidate, LandOutlook, PublicLand, RavageForecast } from './types'
import LedgerEntry from './LedgerEntry.vue'
import LandChip from './LandChip.vue'
import InvaderCardTile from './InvaderCardTile.vue'
import Pips from './Pips.vue'
import Icon from '../components/Icon.vue'
import { useCompanionCtx } from './context'
import { invaderLabel, parseBlocked, parseLand, pieceIcon, prettyText, terrainColor, terrainIcon } from './format'

const props = defineProps<{
  land: PublicLand
  outlook: LandOutlook | undefined
  ravage: RavageForecast | undefined
  build: BuildForecast | undefined
  candidates: ExploreCandidate[]
  cascade: CascadeChain | undefined
}>()
const emit = defineEmits<{ close: [] }>()
const ctx = useCompanionCtx()

const short = computed(() => parseLand(props.land.id).short)
const hits = computed(() => props.candidates.filter((c) => c.lands.includes(props.land.id)))
const blockedBy = computed(() =>
  props.candidates
    .map((c) => ({ c, b: c.blocked.map(parseBlocked).find((x) => x.land === props.land.id) }))
    .filter((x) => x.b),
)
const access = computed(() => {
  switch (props.outlook?.explore_access) {
    case 'coastal': return 'Coastal: the Ocean is always an Explore source'
    case 'fed': return 'Inland, with an Explore source in range'
    case 'dead': return 'No Explore source in range after this phase’s Builds'
    case 'ocean': return 'Ocean'
    default: return null
  }
})
const ravNext = computed(() => {
  switch (props.outlook?.ravage_next) {
    case 'Certain': return 'Yes: matches the Build card and has Invaders after this Build'
    case 'IfExplored': return 'Only if it is explored this phase'
    default: return 'No'
  }
})
const presence = computed(() =>
  Object.entries(props.land.presence).filter(([, n]) => n > 0).map(([id, n]) => `${ctx?.spiritTitles.value[id] ?? id}: ${n}`),
)
const tokens = computed(() => Object.entries(props.land.tokens).filter(([, n]) => n > 0))
</script>

<template>
  <section class="detail" :aria-label="`${land.terrain} ${short}`">
    <header class="dh" :style="{ background: terrainColor(land.terrain) }">
      <Icon v-if="terrainIcon(land.terrain)" :name="terrainIcon(land.terrain)!" :size="18" decorative />
      <b class="dn">{{ short }}</b>
      <span class="dt">{{ land.terrain }}{{ land.coastal ? ', coastal' : '' }}</span>
      <button type="button" class="x" title="Close" @click="emit('close')">
        <svg viewBox="0 0 16 16" width="14" height="14" aria-hidden="true"><path d="M4 4l8 8M12 4l-8 8" stroke="currentColor" stroke-width="2" stroke-linecap="round" /></svg>
        <span class="sr">Close land detail</span>
      </button>
    </header>

    <div class="db">
      <div class="pieces">
        <span v-for="(p, i) in land.pieces" :key="i" class="piece" :title="`${p.kind}: health ${p.max_health}${p.damage ? `, ${p.damage} damage` : ''}${p.strife ? `, ${p.strife} Strife` : ''}`">
          <Icon v-if="pieceIcon(p.kind)" :name="pieceIcon(p.kind)!" :size="15" decorative />
          <span>{{ p.kind }}</span>
          <span class="hp">{{ p.max_health - p.damage }}/{{ p.max_health }}</span>
          <span v-if="p.strife" class="strife">Strife</span>
        </span>
        <span v-if="land.blight" class="piece"><Icon name="resource-blight" :size="15" decorative />{{ land.blight }} Blight</span>
        <span v-for="pr in presence" :key="pr" class="piece">{{ pr }} presence</span>
        <span v-for="[t, n] in tokens" :key="t" class="piece">{{ n }} {{ t }}</span>
        <span v-if="!land.pieces.length && !land.blight && !presence.length && !tokens.length" class="muted">Nothing here</span>
      </div>
      <p v-if="land.defend || land.damage_taken || land.health_modifier" class="muted small">
        <template v-if="land.defend">Defend {{ land.defend }}. </template>
        <template v-if="land.damage_taken">{{ land.damage_taken }} damage taken. </template>
        <template v-if="land.health_modifier">Land health {{ land.health_modifier > 0 ? '+' : '' }}{{ land.health_modifier }}.</template>
      </p>

      <h4>Ravage this turn</h4>
      <LedgerEntry v-if="ravage" :land="land.id" :ledger="ravage.ledger" :done="ravage.done" hide-land />
      <p v-else class="muted">No Ravage here this turn.</p>

      <h4>Build this turn</h4>
      <p v-if="build">+{{ build.adds }}<template v-if="build.done"> (built)</template><span v-for="(n, i) in build.notes" :key="i" class="warn"> {{ prettyText(n) }}</span></p>
      <p v-else class="muted">No Build here this turn.</p>

      <h4>Explore</h4>
      <p v-if="access" class="small">{{ access }}</p>
      <p v-if="outlook && outlook.explore_chance[1] > 0" class="row">
        <Pips :hit="outlook.explore_chance[0]" :total="outlook.explore_chance[1]" tone="explore" :size="8" />
        <span><b>{{ outlook.explore_chance[0] }}</b> of {{ outlook.explore_chance[1] }} candidate cards explore here</span>
      </p>
      <div v-if="hits.length" class="cards">
        <span v-for="c in hits" :key="c.card.id" class="cardref"><InvaderCardTile :card="c.card" size="xs" /> {{ invaderLabel(c.card) }}</span>
      </div>
      <p v-for="x in blockedBy" :key="x.c.card.id" class="small muted">
        {{ invaderLabel(x.c.card) }} matches but won't explore here ({{ x.b!.reason ?? 'no source in range' }}).
      </p>
      <p v-if="outlook?.fed_by.length" class="row wrap"><span class="lbl">Fed by</span> <LandChip v-for="f in outlook.fed_by" :key="f" :id="f" /></p>
      <p v-if="outlook?.sole_source_for.length" class="row wrap"><span class="lbl">Only source for</span> <LandChip v-for="f in outlook.sole_source_for" :key="f" :id="f" /></p>

      <h4>Next turn</h4>
      <p class="small"><span class="lbl">Ravage next turn</span> {{ ravNext }}</p>
      <p v-if="outlook && outlook.build_next_chance[1] > 0" class="small">
        <span class="lbl">Build next turn</span> {{ outlook.build_next_chance[0] }} of {{ outlook.build_next_chance[1] }} candidate cards
      </p>
      <p v-if="cascade" class="row wrap small">
        <span class="lbl">Cascade</span> +{{ cascade.min_extra }} more at least if blight lands here;
        <template v-if="cascade.safe_neighbors.length">stops at <LandChip v-for="n in cascade.safe_neighbors" :key="n" :id="n" /></template>
        <template v-else>no unblighted neighbor</template>
      </p>

      <template v-if="outlook && (outlook.fast_window.length || outlook.slow_window.length)">
        <h4>Fast or Slow here</h4>
        <div class="windows">
          <div>
            <div class="wh"><Icon name="speed-fast" :size="13" decorative /> Fast</div>
            <ul><li v-for="(t, i) in outlook.fast_window" :key="i">{{ prettyText(t) }}</li><li v-if="!outlook.fast_window.length" class="muted">nothing changes</li></ul>
          </div>
          <div>
            <div class="wh"><Icon name="speed-slow" :size="13" decorative /> Slow</div>
            <ul><li v-for="(t, i) in outlook.slow_window" :key="i">{{ prettyText(t) }}</li><li v-if="!outlook.slow_window.length" class="muted">nothing changes</li></ul>
          </div>
        </div>
      </template>
    </div>
  </section>
</template>

<style scoped>
.detail { background: var(--bg-surface); border: 1px solid var(--border-strong); border-radius: 10px; }
.dh { border-radius: 9px 9px 0 0; position: sticky; top: 0; z-index: 2; }
.dh { display: flex; align-items: center; gap: 8px; padding: 6px 10px; color: #0b1220; }
.dn { font-family: var(--font-brand); font-size: 22px; font-weight: 600; line-height: 1; }
.dt { font-size: 13px; font-weight: 600; }
.x { margin-left: auto; display: inline-grid; place-items: center; width: 26px; height: 26px; padding: 0; background: rgba(11, 18, 32, 0.18); border: 0; color: #0b1220; border-radius: 6px; }
.x:hover { background: rgba(11, 18, 32, 0.3); }
.db { padding: 8px 12px 12px; font-size: 12.5px; color: var(--text-primary); }
h4 { margin: 10px 0 4px; font-size: 12px; font-weight: 600; color: var(--text-secondary); }
p { margin: 2px 0; }
.pieces { display: flex; flex-wrap: wrap; gap: 4px; }
.piece { display: inline-flex; align-items: center; gap: 4px; padding: 2px 6px; border-radius: 5px; background: var(--bg-inset); font-size: 12px; }
.hp { color: var(--text-muted); font-variant-numeric: tabular-nums; }
.strife { color: #fde68a; font-size: 11px; }
.row { display: flex; align-items: center; gap: 6px; }
.row.wrap { flex-wrap: wrap; }
.cards { display: flex; flex-wrap: wrap; gap: 6px; margin: 4px 0; }
.cardref { display: inline-flex; align-items: center; gap: 4px; font-size: 12px; }
.lbl { color: var(--text-muted); }
.small { font-size: 12px; }
.muted { color: var(--text-muted); }
.warn { color: var(--accent-amber); }
.windows { display: grid; grid-template-columns: 1fr 1fr; gap: 8px; }
.wh { display: flex; align-items: center; gap: 4px; font-size: 11.5px; color: var(--text-secondary); }
.windows ul { margin: 2px 0 0; padding-left: 14px; font-size: 12px; }
.sr { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0 0 0 0); white-space: nowrap; }
</style>
