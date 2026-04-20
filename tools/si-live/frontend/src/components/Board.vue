<script setup lang="ts">
import { ref } from 'vue'
import type { Board, Land } from '../types'
import LandEditor from './LandEditor.vue'
import Icon from './Icon.vue'

const props = defineProps<{ modelValue: Board; boardId: string }>()
defineEmits<{ 'update:modelValue': [value: Board] }>()

const saveStatus = ref<string>('')

function sortedKeys(b: Board): string[] {
  return Object.keys(b.lands).sort((a, z) => Number(a) - Number(z))
}

interface SummaryChip {
  count: number
  icon: string
  label: string
}

function summaryChips(l: Land): SummaryChip[] {
  const chips: SummaryChip[] = []
  if (l.explorers) chips.push({ count: l.explorers, icon: 'unit-explorer', label: 'Explorer' })
  if (l.towns) chips.push({ count: l.towns, icon: 'unit-town', label: 'Town' })
  if (l.cities) chips.push({ count: l.cities, icon: 'unit-city', label: 'City' })
  if (l.dahan) chips.push({ count: l.dahan, icon: 'unit-dahan', label: 'Dahan' })
  if (l.blight) chips.push({ count: l.blight, icon: 'resource-blight', label: 'Blight' })
  return chips
}

async function saveGeometry() {
  saveStatus.value = 'saving…'
  try {
    const res = await fetch('/api/save-board-geometry', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        board_id: props.boardId,
        lands: props.modelValue.lands,
      }),
    })
    if (!res.ok) {
      const txt = await res.text()
      throw new Error(`${res.status}: ${txt}`)
    }
    saveStatus.value = 'Saved ✓'
    setTimeout(() => { saveStatus.value = '' }, 3000)
  } catch (e) {
    saveStatus.value = `Error: ${(e as Error).message}`
  }
}
</script>

<template>
  <div class="board-wrap">
    <div class="board-tools">
      <div class="legend">
        <span class="legend-label">Units</span>
        <span class="legend-items">
          <Icon name="unit-explorer" :size="14" decorative /> Explorer
          <Icon name="unit-town" :size="14" decorative /> Town
          <Icon name="unit-city" :size="14" decorative /> City
          <Icon name="unit-dahan" :size="14" decorative /> Dahan
          <Icon name="resource-blight" :size="14" decorative /> Blight
        </span>
      </div>
      <div class="actions">
        <button
          class="save-geo"
          @click="saveGeometry"
          title="Persist corrected terrain/coastal back to data/boards/*.json so future New Games use your corrections"
        >
          Save Geometry
        </button>
        <span class="save-status" :class="{ error: saveStatus.startsWith('Error') }">
          {{ saveStatus }}
        </span>
      </div>
    </div>

    <div class="grid">
      <div
        v-for="id in sortedKeys(modelValue)"
        :key="id"
        class="land"
        :class="[`terrain-${modelValue.lands[id].terrain}`, { coastal: modelValue.lands[id].coastal }]"
      >
        <div class="land-hdr">
          <span class="land-id">#{{ id }}</span>
          <div class="land-terrain">
            <Icon :name="`terrain-${modelValue.lands[id].terrain}`" :size="14" decorative />
            <span class="terrain-label">{{ modelValue.lands[id].terrain }}</span>
            <span v-if="modelValue.lands[id].coastal" class="coastal-tag">coast</span>
          </div>
        </div>

        <div class="summary">
          <span
            v-for="chip in summaryChips(modelValue.lands[id])"
            :key="chip.label"
            class="sum-chip"
            :title="`${chip.count} ${chip.label}`"
          >
            <Icon :name="chip.icon" :size="14" decorative />
            <span class="sum-count">{{ chip.count }}</span>
          </span>
          <span v-if="!summaryChips(modelValue.lands[id]).length" class="sum-empty">—</span>
        </div>

        <LandEditor v-model="modelValue.lands[id]" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.board-wrap {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
}

.board-tools {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--sp-3);
}

.legend {
  display: flex;
  gap: var(--sp-2);
  align-items: center;
  font-size: var(--fs-xs);
  color: var(--text-secondary);
}

.legend-label {
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-size: 0.7rem;
  color: var(--text-muted);
}

.legend-items {
  display: inline-flex;
  gap: var(--sp-3);
  align-items: center;
  flex-wrap: wrap;
}

.actions {
  display: inline-flex;
  align-items: center;
  gap: var(--sp-2);
}

.save-status {
  font-size: var(--fs-xs);
  color: var(--status-success);
  font-style: italic;
}
.save-status.error { color: var(--status-danger); font-style: normal; }

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
  gap: var(--sp-3);
}

.land {
  position: relative;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-lg);
  padding: var(--sp-3);
  transition: border-color var(--motion-fast), box-shadow var(--motion-fast);
}

.land::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  width: 4px;
  border-radius: var(--r-lg) 0 0 var(--r-lg);
  background: var(--terrain-color, var(--border-default));
}

.land.terrain-mountain { --terrain-color: var(--terrain-mountain); }
.land.terrain-wetland { --terrain-color: var(--terrain-wetland); }
.land.terrain-jungle { --terrain-color: var(--terrain-jungle); }
.land.terrain-sands { --terrain-color: var(--terrain-sands); }
.land.terrain-ocean { --terrain-color: var(--terrain-ocean); }

.land:hover { border-color: var(--border-default); }

.land-hdr {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--sp-2);
}

.land-id {
  font-weight: var(--fw-bold);
  font-size: var(--fs-md);
  color: var(--text-primary);
  font-family: var(--font-mono);
}

.land-terrain {
  display: inline-flex;
  align-items: center;
  gap: var(--sp-1);
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  text-transform: capitalize;
}

.terrain-label {
  color: var(--text-secondary);
}

.coastal-tag {
  color: var(--terrain-ocean);
  font-size: 0.68rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  padding: 1px var(--sp-1);
  background: rgba(90, 118, 145, 0.15);
  border-radius: var(--r-sm);
}

.summary {
  display: flex;
  gap: var(--sp-1);
  flex-wrap: wrap;
  min-height: 1.6rem;
  padding: var(--sp-1) 0;
}

.sum-chip {
  display: inline-flex;
  align-items: center;
  gap: 3px;
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-full);
  padding: 1px var(--sp-2);
  font-size: var(--fs-xs);
}

.sum-count {
  font-family: var(--font-mono);
  font-weight: var(--fw-semibold);
  color: var(--text-primary);
}

.sum-empty {
  color: var(--text-faint);
  font-style: italic;
  font-size: var(--fs-xs);
}
</style>
