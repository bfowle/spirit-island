<script setup lang="ts">
import { computed, ref } from 'vue'
import type { Board, Land } from '../types'
import LandEditor from './LandEditor.vue'
import Icon from './Icon.vue'
import { getLayout } from '../boardLayouts'

const props = defineProps<{ modelValue: Board; boardId: string }>()
defineEmits<{ 'update:modelValue': [value: Board] }>()

type ViewMode = 'grid' | 'map'
const viewMode = ref<ViewMode>('grid')
const expandedLand = ref<string | null>(null)

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

const layout = computed(() => getLayout(props.boardId, props.modelValue.variant))
const canMap = computed(() => layout.value !== null)

function toggleLand(id: string) {
  expandedLand.value = expandedLand.value === id ? null : id
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

      <div class="board-meta">
        <span v-if="modelValue.variant_name" class="variant-tag">{{ modelValue.variant_name }}</span>
        <div class="view-toggle">
          <button
            class="view-btn"
            :class="{ active: viewMode === 'grid' }"
            @click="viewMode = 'grid'"
          >Grid</button>
          <button
            class="view-btn"
            :class="{ active: viewMode === 'map' }"
            :disabled="!canMap"
            :title="canMap ? 'Show lands positioned like the physical board' : 'Map layout not defined for this board/variant'"
            @click="viewMode = 'map'"
          >Map</button>
        </div>
      </div>
    </div>

    <!-- GRID VIEW -->
    <div v-if="viewMode === 'grid'" class="grid">
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

    <!-- MAP VIEW -->
    <div v-else class="map-view">
      <div class="map-canvas" :class="[`ocean-${layout?.oceanEdge || 'top'}`]">
        <div class="map-ocean" :title="'Ocean'"></div>
        <button
          v-for="id in sortedKeys(modelValue)"
          :key="id"
          class="map-land"
          :class="[
            `terrain-${modelValue.lands[id].terrain}`,
            { coastal: modelValue.lands[id].coastal, expanded: expandedLand === id }
          ]"
          :style="{
            left: (layout?.positions[id]?.x ?? 50) + '%',
            top: (layout?.positions[id]?.y ?? 50) + '%'
          }"
          :title="`#${id} — ${modelValue.lands[id].terrain}${modelValue.lands[id].coastal ? ' · coast' : ''}`"
          @click="toggleLand(id)"
        >
          <span class="map-id">{{ id }}</span>
          <div class="map-chips">
            <span
              v-for="chip in summaryChips(modelValue.lands[id])"
              :key="chip.label"
              class="map-chip"
              :title="`${chip.count} ${chip.label}`"
            >
              <Icon :name="chip.icon" :size="10" decorative />{{ chip.count }}
            </span>
          </div>
        </button>
      </div>

      <!-- Inline editor opens beneath the map when a land is clicked -->
      <div v-if="expandedLand" class="map-editor-panel">
        <div class="editor-hdr">
          <strong>#{{ expandedLand }}</strong>
          <span class="editor-terrain">
            <Icon :name="`terrain-${modelValue.lands[expandedLand].terrain}`" :size="14" decorative />
            {{ modelValue.lands[expandedLand].terrain }}
            <span v-if="modelValue.lands[expandedLand].coastal" class="coastal-tag">coast</span>
          </span>
          <button class="ghost" @click="expandedLand = null">Close</button>
        </div>
        <LandEditor v-model="modelValue.lands[expandedLand]" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.board-wrap { display: flex; flex-direction: column; gap: var(--sp-3); }

.board-tools {
  display: flex; justify-content: space-between; align-items: center;
  flex-wrap: wrap; gap: var(--sp-3);
}

.legend {
  display: flex; gap: var(--sp-2); align-items: center;
  font-size: var(--fs-xs); color: var(--text-secondary);
}
.legend-label {
  text-transform: uppercase; letter-spacing: 0.05em;
  font-size: 0.7rem; color: var(--text-muted);
}
.legend-items { display: inline-flex; gap: var(--sp-3); align-items: center; flex-wrap: wrap; }

.board-meta { display: inline-flex; align-items: center; gap: var(--sp-2); }

.variant-tag {
  font-size: var(--fs-xs); color: var(--text-secondary);
  padding: 2px var(--sp-2);
  background: var(--bg-muted); border: 1px solid var(--border-subtle);
  border-radius: var(--r-full);
  text-transform: capitalize; font-weight: var(--fw-medium);
}

.view-toggle {
  display: inline-flex; gap: 2px; padding: 2px;
  background: var(--bg-muted); border-radius: var(--r-sm);
}
.view-btn {
  font-size: var(--fs-xs); padding: 2px var(--sp-2);
  background: transparent; border: 1px solid transparent;
  color: var(--text-secondary); border-radius: var(--r-sm);
}
.view-btn:hover { background: var(--bg-hover); }
.view-btn.active {
  background: var(--bg-surface); color: var(--text-primary);
  border-color: var(--border-default);
}
.view-btn:disabled { opacity: 0.4; cursor: not-allowed; }

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
  gap: var(--sp-3);
}

.land {
  position: relative; background: var(--bg-surface);
  border: 1px solid var(--border-subtle); border-radius: var(--r-lg);
  padding: var(--sp-3); box-shadow: var(--shadow-sm);
  transition: box-shadow var(--motion-fast);
}
.land:hover { box-shadow: var(--shadow-md); }
.land::before {
  content: ''; position: absolute; top: 0; left: 0; bottom: 0;
  width: 4px; border-radius: var(--r-lg) 0 0 var(--r-lg);
  background: var(--terrain-color, var(--border-default));
}

.land.terrain-mountain { --terrain-color: var(--terrain-mountain); }
.land.terrain-wetland  { --terrain-color: var(--terrain-wetland); }
.land.terrain-jungle   { --terrain-color: var(--terrain-jungle); }
.land.terrain-sands    { --terrain-color: var(--terrain-sands); }
.land.terrain-ocean    { --terrain-color: var(--terrain-ocean); }

.land-hdr { display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--sp-2); }
.land-id { font-weight: var(--fw-bold); font-size: var(--fs-md); color: var(--text-primary); font-family: var(--font-mono); }
.land-terrain { display: inline-flex; align-items: center; gap: var(--sp-1); font-size: var(--fs-xs); color: var(--text-secondary); text-transform: capitalize; }
.terrain-label { color: var(--text-secondary); }

.coastal-tag {
  color: var(--terrain-ocean); font-size: 0.68rem;
  text-transform: uppercase; letter-spacing: 0.05em;
  padding: 1px var(--sp-1); background: rgba(79, 114, 145, 0.15);
  border-radius: var(--r-sm);
}

.summary { display: flex; gap: var(--sp-1); flex-wrap: wrap; min-height: 1.6rem; padding: var(--sp-1) 0; }
.sum-chip { display: inline-flex; align-items: center; gap: 3px; background: var(--bg-muted); border: 1px solid var(--border-subtle); border-radius: var(--r-full); padding: 1px var(--sp-2); font-size: var(--fs-xs); }
.sum-count { font-family: var(--font-mono); font-weight: var(--fw-semibold); color: var(--text-primary); }
.sum-empty { color: var(--text-faint); font-style: italic; font-size: var(--fs-xs); }

/* ───────── Map view ───────── */

.map-view { display: flex; flex-direction: column; gap: var(--sp-3); }

.map-canvas {
  position: relative;
  width: 100%;
  aspect-ratio: 1 / 1.05;
  background: linear-gradient(180deg, var(--bg-surface), var(--bg-muted));
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-lg);
  box-shadow: var(--shadow-sm);
  overflow: hidden;
  max-width: 560px;
  margin: 0 auto;
}

/* Ocean strip along one edge */
.map-ocean {
  position: absolute;
  background: linear-gradient(to bottom, rgba(79, 114, 145, 0.18), rgba(79, 114, 145, 0));
  pointer-events: none;
}
.map-canvas.ocean-top .map-ocean    { top: 0; left: 0; right: 0; height: 15%; }
.map-canvas.ocean-bottom .map-ocean { bottom: 0; left: 0; right: 0; height: 15%; background: linear-gradient(to top, rgba(79, 114, 145, 0.18), rgba(79, 114, 145, 0)); }
.map-canvas.ocean-left .map-ocean   { top: 0; bottom: 0; left: 0; width: 15%; background: linear-gradient(to right, rgba(79, 114, 145, 0.18), rgba(79, 114, 145, 0)); }
.map-canvas.ocean-right .map-ocean  { top: 0; bottom: 0; right: 0; width: 15%; background: linear-gradient(to left, rgba(79, 114, 145, 0.18), rgba(79, 114, 145, 0)); }

.map-land {
  position: absolute;
  transform: translate(-50%, -50%);
  width: 4.5rem;
  min-height: 4.5rem;
  padding: var(--sp-1) var(--sp-2);
  background: var(--bg-surface);
  border: 2px solid var(--terrain-color, var(--border-default));
  border-radius: var(--r-md);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  box-shadow: var(--shadow-sm);
  cursor: pointer;
  transition: transform var(--motion-fast), box-shadow var(--motion-fast);
}
.map-land:hover { transform: translate(-50%, calc(-50% - 2px)); box-shadow: var(--shadow-md); }
.map-land.expanded { box-shadow: 0 0 0 3px var(--accent-soft), var(--shadow-md); }

.map-land.terrain-mountain { --terrain-color: var(--terrain-mountain); }
.map-land.terrain-wetland  { --terrain-color: var(--terrain-wetland); }
.map-land.terrain-jungle   { --terrain-color: var(--terrain-jungle); }
.map-land.terrain-sands    { --terrain-color: var(--terrain-sands); }

.map-id {
  font-family: var(--font-mono); font-weight: var(--fw-bold);
  font-size: var(--fs-md); color: var(--text-primary);
  line-height: 1;
}

.map-chips { display: flex; flex-wrap: wrap; gap: 2px; justify-content: center; }

.map-chip {
  display: inline-flex; align-items: center; gap: 1px;
  font-size: 0.58rem; padding: 0 3px;
  background: var(--bg-muted); border-radius: var(--r-sm);
  color: var(--text-primary);
  font-family: var(--font-mono);
  font-weight: var(--fw-semibold);
}

.map-editor-panel {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-lg);
  padding: var(--sp-3);
  box-shadow: var(--shadow-sm);
}
.editor-hdr {
  display: flex; align-items: center; gap: var(--sp-2);
  margin-bottom: var(--sp-2); padding-bottom: var(--sp-2);
  border-bottom: 1px solid var(--border-subtle);
}
.editor-hdr strong { font-family: var(--font-mono); font-size: var(--fs-md); }
.editor-terrain { display: inline-flex; align-items: center; gap: var(--sp-1); font-size: var(--fs-xs); color: var(--text-secondary); text-transform: capitalize; flex: 1; }
</style>
