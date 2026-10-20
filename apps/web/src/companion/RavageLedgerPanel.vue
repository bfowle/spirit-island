<script setup lang="ts">
import { computed } from 'vue'
import type { BlightStats, InvaderPhaseForecast, PublicStatusEffect, StatusParam } from './types'
import Panel from './Panel.vue'
import LedgerEntry from './LedgerEntry.vue'
import LandChip from './LandChip.vue'
import { plural, splitCamel } from './format'

const props = defineProps<{
  phase: InvaderPhaseForecast
  blight: BlightStats
  effects: PublicStatusEffect[]
}>()

const open = computed(() => props.phase.ravages.filter((r) => !r.done).length)

function paramText(p: Record<string, StatusParam> | undefined): string {
  if (!p) return ''
  return Object.entries(p)
    .map(([k, v]) => `${splitCamel(k)}: ${Array.isArray(v) ? v.join(', ') : String(v)}`)
    .join('\n')
}
</script>

<template>
  <Panel title="Ravage ledger" question="What does each Ravage do if nothing changes?" anchor="panel-ravage">
    <template #meta><span>{{ plural(phase.ravages.length, 'land') }}</span></template>

    <div class="summary">
      <span>
        If nothing changes: at least <b>{{ blight.adds_this_phase_min }}</b> blight this phase
        <template v-if="blight.ravage_lands_adding.length">
          from <LandChip v-for="l in blight.ravage_lands_adding" :key="l" :id="l" />
        </template>
      </span>
      <span class="sep">→</span>
      <span class="outcome">{{ blight.outcome_this_phase }}</span>
      <span class="muted">({{ blight.remaining }} on the card)</span>
    </div>

    <div v-if="effects.length" class="effects">
      <span class="eff-label">Active effects</span>
      <span v-for="(e, i) in effects" :key="i" class="eff" :title="paramText(e.params) || undefined">
        {{ splitCamel(e.kind) }}<template v-if="e.source"> from {{ e.source }}</template>
      </span>
    </div>

    <LedgerEntry v-for="r in phase.ravages" :key="r.land" :land="r.land" :ledger="r.ledger" :done="r.done" />
    <p v-if="!phase.ravages.length" class="empty">
      No land matches the Ravage card<template v-if="phase.ravage_cards.length"> and has Invaders</template>.
    </p>
    <p v-else-if="open === 0" class="empty">Every Ravage this phase has resolved.</p>
  </Panel>
</template>

<style scoped>
.summary { display: flex; flex-wrap: wrap; align-items: center; gap: 6px; font-size: 12.5px; color: var(--text-primary); padding-bottom: 8px; border-bottom: 1px solid var(--border-subtle); }
.summary b { font-variant-numeric: tabular-nums; }
.sep { color: var(--text-muted); }
.outcome { font-weight: 600; }
.muted { color: var(--text-muted); }
.effects { display: flex; flex-wrap: wrap; gap: 4px 6px; align-items: center; padding: 8px 0; border-bottom: 1px solid var(--border-subtle); font-size: 12px; }
.eff-label { color: var(--text-muted); margin-right: 2px; }
.eff { padding: 1px 7px; border-radius: 4px; background: rgba(124, 58, 237, 0.16); border: 1px solid rgba(124, 58, 237, 0.4); color: #ddd6fe; cursor: default; }
.empty { margin: 10px 0 0; font-size: 12.5px; color: var(--text-muted); }
</style>
