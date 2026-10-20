<script setup lang="ts">
import { computed } from 'vue'
import type { RavageLedger } from './types'
import LandChip from './LandChip.vue'
import Icon from '../components/Icon.vue'
import { useCompanionCtx } from './context'
import { pieceIcon, prettyText } from './format'

/** One land's Ravage, as a ledger the player can recompute at the table. */
const props = defineProps<{ land: string; ledger: RavageLedger; done?: boolean; hideLand?: boolean }>()
const ctx = useCompanionCtx()

const L = computed(() => props.ledger)
const dahanLeft = computed(() => Math.max(0, L.value.dahan_before - L.value.dahan_destroyed))
const presence = computed(() =>
  Object.entries(L.value.presence_at_risk).filter(([, n]) => n > 0).map(([id, n]) => ({ name: ctx?.spiritTitles.value[id] ?? id, n })),
)

const counterTail = computed(() =>
  L.value.counterattack_clears
    ? ': clears every Invader here'
    : L.value.counterattack_can_destroy.length
      ? ': can destroy'
      : ': not enough to destroy an Invader',
)
const counterDestroy = computed(() => (L.value.counterattack_clears ? [] : L.value.counterattack_can_destroy))

// Damage blocks vs the land's health threshold.
const slots = computed(() => Math.min(24, Math.max(L.value.damage_after_defend, L.value.land_health, 1)))
const blockW = 9
const blockGap = 2
const barW = computed(() => slots.value * (blockW + blockGap))
const tickX = computed(() => Math.min(slots.value, L.value.land_health) * (blockW + blockGap) - blockGap / 2)
</script>

<template>
  <article class="ledger" :class="{ done }">
    <header class="lhead">
      <LandChip v-if="!hideLand" :id="land" />
      <span v-if="done" class="resolved">resolved</span>
      <span class="outcome" :class="{ blight: L.adds_blight }">
        <template v-if="L.adds_blight">+1 blight{{ L.cascades ? ', cascades' : '' }}</template>
        <template v-else>no blight</template>
      </span>
    </header>

    <div v-if="L.skip_available" class="skip" role="note">
      <b>Skip available:</b> {{ prettyText(L.skip_available) }} can cancel this Ravage.
    </div>

    <dl class="rows">
      <dt>Damage</dt>
      <dd class="terms">
        <template v-for="([kind, dmg], i) in L.damage_terms" :key="kind">
          <span v-if="i > 0" class="op">+</span>
          <span class="term">
            <Icon v-if="pieceIcon(kind)" :name="pieceIcon(kind)!" :size="13" decorative />
            {{ kind }} <b>{{ dmg }}</b>
          </span>
        </template>
        <span v-if="!L.damage_terms.length" class="muted">no Invader deals damage</span>
        <span class="op">=</span><b>{{ L.invader_damage }}</b>
      </dd>

      <dt>Defend</dt>
      <dd>
        <span class="op">−</span><b>{{ L.defend }}</b>
        <span class="op">=</span><b>{{ L.damage_after_defend }}</b> damage to the land
      </dd>

      <dt>Land</dt>
      <dd class="vs">
        <svg :width="barW" height="14" class="bar" role="img" :aria-label="`${L.damage_after_defend} damage against land health ${L.land_health}`">
          <rect
            v-for="i in slots"
            :key="i"
            :x="(i - 1) * (blockW + blockGap)"
            y="2"
            :width="blockW"
            height="10"
            rx="2"
            :class="i <= L.damage_after_defend ? (L.adds_blight ? 'hit blight' : 'hit') : 'gap'"
          />
          <line :x1="tickX" :x2="tickX" y1="0" y2="14" class="tick" />
        </svg>
        <span><b>{{ L.damage_after_defend }}</b> vs health <b>{{ L.land_health }}</b></span>
      </dd>

      <template v-if="L.dahan_before > 0">
        <dt>Dahan</dt>
        <dd>
          <Icon name="unit-dahan" :size="13" decorative />
          {{ L.dahan_before }} Dahan take the same damage: <b>{{ L.dahan_destroyed }}</b> destroyed, {{ dahanLeft }} left
        </dd>
        <dt>Counter</dt>
        <dd v-if="dahanLeft > 0">
          <span>{{ dahanLeft }} Dahan deal <b>{{ L.counterattack_damage }}</b> damage{{ counterTail }}</span>
          <span v-for="(k, i) in counterDestroy" :key="i" class="term">
            <Icon v-if="pieceIcon(k)" :name="pieceIcon(k)!" :size="13" decorative />{{ k }}
          </span>
          <span v-if="L.counterattack_fear > 0">(+{{ L.counterattack_fear }} fear)</span>
        </dd>
        <dd v-else class="muted">no Dahan left to counterattack</dd>
      </template>

      <template v-if="presence.length">
        <dt>Presence</dt>
        <dd>
          <span v-for="p in presence" :key="p.name">{{ p.name }}: {{ p.n }} at risk</span>
        </dd>
      </template>

      <template v-if="L.applied.length">
        <dt>Applied</dt>
        <dd class="list">
          <span v-for="(a, i) in L.applied" :key="i" class="applied">{{ prettyText(a) }}</span>
        </dd>
      </template>

      <template v-if="L.unmodeled.length">
        <dt>Not modeled</dt>
        <dd class="list">
          <span v-for="(u, i) in L.unmodeled" :key="i" class="unmodeled" title="Present here but not applied by the ledger: check its effect by hand">{{ prettyText(u) }}</span>
        </dd>
      </template>
    </dl>
  </article>
</template>

<style scoped>
.ledger { padding: 8px 0 10px; border-bottom: 1px solid var(--border-subtle); }
.ledger:last-child { border-bottom: 0; }
.ledger.done { opacity: 0.5; }
.lhead { display: flex; align-items: center; gap: 8px; margin-bottom: 6px; }
.resolved { font-size: 11px; color: var(--text-muted); }
.outcome { margin-left: auto; font-size: 12px; font-weight: 600; color: var(--text-secondary); padding: 0 7px; border-radius: 4px; background: var(--bg-raised); }
.outcome.blight { color: #fecaca; background: var(--danger-soft); }
.skip {
  margin: 0 0 6px;
  padding: 5px 8px;
  border-radius: 6px;
  border: 1px solid rgba(16, 185, 129, 0.45);
  background: var(--ok-soft);
  color: #a7f3d0;
  font-size: 12px;
}
.rows { display: grid; grid-template-columns: 78px minmax(0, 1fr); gap: 3px 8px; margin: 0; font-size: 12.5px; }
dt { color: var(--text-muted); font-size: 12px; }
dd { margin: 0; color: var(--text-primary); display: flex; flex-wrap: wrap; align-items: center; gap: 4px; }
dd b { font-variant-numeric: tabular-nums; }
.term { display: inline-flex; align-items: center; gap: 3px; }
.op { color: var(--text-muted); }
.muted { color: var(--text-muted); }
.vs { gap: 8px; }
.bar rect.gap { fill: none; stroke: var(--border-strong); stroke-width: 1; }
.bar rect.hit { fill: var(--text-secondary); }
.bar rect.hit.blight { fill: var(--accent-red); }
.bar .tick { stroke: var(--text-white); stroke-width: 1.5; }
.list { flex-direction: column; align-items: flex-start; gap: 2px; }
.applied::before { content: '✓ '; color: var(--accent-green); }
.unmodeled { color: var(--accent-amber); }
.unmodeled::before { content: 'not modeled: '; color: var(--text-muted); }
</style>
