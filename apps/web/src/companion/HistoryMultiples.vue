<script setup lang="ts">
import { computed, ref } from 'vue'
import type { RoundSummary } from './types'
import Panel from './Panel.vue'
import MiniColumns, { type MiniSeries } from './MiniColumns.vue'

const props = defineProps<{ history: RoundSummary[]; ended: boolean }>()
const showTable = ref(false)

// Series colors: validated categorical slots (dataviz palette, dark steps).
const C1 = '#3987e5'
const C2 = '#199e70'
const C3 = '#d95926'

const turns = computed(() => props.history.map((h) => h.turn))
const fearPerTurn = computed(() => props.history.map((h, i) => h.fear_total - (i > 0 ? props.history[i - 1].fear_total : 0)))
const fear = computed<MiniSeries[]>(() => [{ name: 'fear generated', color: '#d9a13a', values: fearPerTurn.value }])
const blight = computed<MiniSeries[]>(() => [{ name: 'blight on the island', color: '#9085e9', values: props.history.map((h) => h.blight_on_island) }])
const invaders = computed<MiniSeries[]>(() => [
  { name: 'Explorers', color: C1, values: props.history.map((h) => h.explorers) },
  { name: 'Towns', color: C2, values: props.history.map((h) => h.towns) },
  { name: 'Cities', color: C3, values: props.history.map((h) => h.cities) },
])
const energy = computed<MiniSeries[]>(() => [{ name: 'Energy', color: '#64748b', values: props.history.map((h) => h.energy) }])
const partial = computed(() => !props.ended)
</script>

<template>
  <Panel title="History" question="How has each count moved, turn by turn?" anchor="panel-history">
    <template #meta>
      <button type="button" class="tgl" :aria-pressed="showTable" @click="showTable = !showTable">{{ showTable ? 'Show charts' : 'Show table' }}</button>
    </template>
    <template v-if="!history.length">
      <p class="empty">History builds as turns complete.</p>
    </template>
    <template v-else-if="!showTable">
      <div class="grid">
        <MiniColumns title="Fear generated" :turns="turns" :series="fear" :partial-last="partial" />
        <MiniColumns title="Blight on the island, end of turn" :turns="turns" :series="blight" :partial-last="partial" />
        <MiniColumns title="Invaders on the island" :turns="turns" :series="invaders" :partial-last="partial" />
        <MiniColumns title="Energy, end of turn" :turns="turns" :series="energy" :partial-last="partial" />
      </div>
      <p v-if="partial" class="note">The last column is the turn in progress.</p>
    </template>
    <table v-else class="tbl">
      <thead>
        <tr><th>Turn</th><th>Fear</th><th>Fear cards</th><th>TL</th><th>Blight</th><th>On card</th><th>Explorers</th><th>Towns</th><th>Cities</th><th>Dahan</th><th>Presence</th><th>Energy</th><th>Hand</th></tr>
      </thead>
      <tbody>
        <tr v-for="(h, i) in history" :key="h.turn">
          <td>{{ h.turn }}</td><td>{{ fearPerTurn[i] }}</td><td>{{ h.fear_cards_earned }}</td><td>{{ h.terror_level }}</td>
          <td>{{ h.blight_on_island }}</td><td>{{ h.blight_remaining }}{{ h.blight_flipped ? ' (flipped)' : '' }}</td>
          <td>{{ h.explorers }}</td><td>{{ h.towns }}</td><td>{{ h.cities }}</td><td>{{ h.dahan }}</td>
          <td>{{ h.presence_on_island }}</td><td>{{ h.energy }}</td><td>{{ h.cards_in_hand }}</td>
        </tr>
      </tbody>
    </table>
  </Panel>
</template>

<style scoped>
.grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(190px, 1fr)); gap: 14px 18px; }
.note { margin: 6px 0 0; font-size: 11px; color: var(--text-muted); }
.empty { margin: 0; font-size: 12.5px; color: var(--text-muted); }
.tgl { font-size: 11.5px; padding: 2px 8px; }
.tbl { width: 100%; border-collapse: collapse; font-size: 12px; font-variant-numeric: tabular-nums; }
.tbl th { text-align: right; font-weight: 500; color: var(--text-muted); font-size: 11px; padding: 2px 6px; border-bottom: 1px solid var(--border-subtle); }
.tbl td { text-align: right; padding: 2px 6px; color: var(--text-primary); }
.tbl th:first-child, .tbl td:first-child { text-align: left; }
</style>
