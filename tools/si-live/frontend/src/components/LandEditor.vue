<script setup lang="ts">
import type { Land } from '../types'
import { UNIT_KEYS, type UnitKey, TERRAINS } from '../types'
import Icon from './Icon.vue'

const props = defineProps<{ modelValue: Land }>()

function bump(key: UnitKey, delta: number) {
  const current = (props.modelValue[key] as number | undefined) ?? 0
  const next = Math.max(0, current + delta)
  props.modelValue[key] = next as never
}

const UNIT_META: Record<UnitKey, { label: string; icon?: string; symbol?: string }> = {
  explorers: { label: 'Explorer', icon: 'unit-explorer' },
  towns: { label: 'Town', icon: 'unit-town' },
  cities: { label: 'City', icon: 'unit-city' },
  dahan: { label: 'Dahan', icon: 'unit-dahan' },
  blight: { label: 'Blight', icon: 'resource-blight' },
}
</script>

<template>
  <div class="editor">
    <div class="geometry">
      <label class="geo-field" title="Change terrain of this land">
        <span class="geo-label">Terrain</span>
        <select v-model="modelValue.terrain">
          <option v-for="t in TERRAINS" :key="t" :value="t">{{ t }}</option>
        </select>
      </label>
      <label class="geo-field coast-field" title="Ocean-adjacent?">
        <input type="checkbox" v-model="modelValue.coastal" />
        <span class="geo-label">Coastal</span>
      </label>
    </div>

    <div class="units">
      <div v-for="k in UNIT_KEYS" :key="k" class="unit-row" :title="UNIT_META[k].label">
        <Icon
          v-if="UNIT_META[k].icon"
          :name="UNIT_META[k].icon!"
          :label="UNIT_META[k].label"
          :size="18"
          class="unit-icon"
        />
        <span class="unit-name">{{ UNIT_META[k].label }}</span>
        <div class="stepper">
          <button class="step" @click="bump(k, -1)" aria-label="decrement">−</button>
          <span class="val">{{ (modelValue[k] as number) ?? 0 }}</span>
          <button class="step" @click="bump(k, 1)" aria-label="increment">+</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.editor {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
  margin-top: var(--sp-2);
}

.geometry {
  display: flex;
  gap: var(--sp-3);
  align-items: center;
  flex-wrap: wrap;
  padding-bottom: var(--sp-2);
  border-bottom: 1px dashed var(--border-subtle);
}

.geo-field {
  display: inline-flex;
  align-items: center;
  gap: var(--sp-1);
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  cursor: pointer;
}

.geo-label {
  text-transform: uppercase;
  letter-spacing: 0.03em;
  font-weight: var(--fw-medium);
  color: var(--text-muted);
  font-size: 0.7rem;
}

.geo-field select {
  text-transform: capitalize;
}

.coast-field { gap: var(--sp-2); }

.units {
  display: flex;
  flex-direction: column;
  gap: var(--sp-1);
}

.unit-row {
  display: grid;
  grid-template-columns: 1.5rem 1fr auto;
  align-items: center;
  gap: var(--sp-2);
  padding: 2px var(--sp-1);
  border-radius: var(--r-sm);
  transition: background var(--motion-fast);
}

.unit-row:hover { background: var(--bg-muted); }

.unit-icon {
  justify-self: center;
}

.unit-name {
  font-size: var(--fs-xs);
  color: var(--text-secondary);
}

.stepper {
  display: inline-flex;
  align-items: center;
  gap: 2px;
}

.step {
  width: 1.35rem;
  height: 1.35rem;
  padding: 0;
  line-height: 1;
  font-size: var(--fs-sm);
  border-radius: var(--r-sm);
}

.val {
  min-width: 1.2rem;
  text-align: center;
  font-family: var(--font-mono);
  font-weight: var(--fw-semibold);
  font-size: var(--fs-sm);
  color: var(--text-primary);
}
</style>
