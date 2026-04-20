<script setup lang="ts">
import type { Land } from '../types'
import { UNIT_KEYS, type UnitKey, TERRAINS } from '../types'

const props = defineProps<{ modelValue: Land }>()

function bump(key: UnitKey, delta: number) {
  const current = (props.modelValue[key] as number | undefined) ?? 0
  const next = Math.max(0, current + delta)
  props.modelValue[key] = next as never
}

const LABELS: Record<UnitKey, string> = {
  explorers: 'Explorer',
  towns: 'Town',
  cities: 'City',
  dahan: 'Dahan',
  blight: 'Blight',
}

const SHORT: Record<UnitKey, string> = {
  explorers: 'E',
  towns: 'T',
  cities: 'C',
  dahan: 'D',
  blight: '⌧',
}
</script>

<template>
  <div class="editor">
    <div class="geometry">
      <label class="terrain-edit" :title="`Change terrain of this land`">
        <span class="geo-label">Terrain</span>
        <select v-model="modelValue.terrain">
          <option v-for="t in TERRAINS" :key="t" :value="t">{{ t }}</option>
        </select>
      </label>
      <label class="coast-edit" :title="`Coastal lands are ocean-adjacent`">
        <input type="checkbox" v-model="modelValue.coastal" />
        <span class="geo-label">Coastal</span>
      </label>
    </div>
    <div class="units">
      <div v-for="k in UNIT_KEYS" :key="k" class="row" :title="LABELS[k]">
        <button @click="bump(k, -1)" aria-label="decrement">−</button>
        <span class="label">{{ SHORT[k] }}<span class="full-label">{{ LABELS[k].slice(1) }}</span></span>
        <span class="val">{{ (modelValue[k] as number) ?? 0 }}</span>
        <button @click="bump(k, 1)" aria-label="increment">+</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.editor { margin-top: .35rem; }
.geometry { display: flex; gap: .5rem; align-items: center; font-size: .75rem; margin-bottom: .35rem; padding-bottom: .35rem; border-bottom: 1px dashed #333; }
.geo-label { color: #888; }
.terrain-edit, .coast-edit { display: flex; align-items: center; gap: .25rem; cursor: pointer; }
.terrain-edit select { font-size: .8rem; padding: 0 .15rem; }
.units { display: grid; grid-template-columns: repeat(5, 1fr); gap: .15rem; }
.row { display: flex; flex-direction: column; align-items: center; font-size: .8rem; }
button { background: #2a2a30; border: 1px solid #444; color: #eee; cursor: pointer; padding: 0 .35rem; border-radius: 3px; }
button:hover { background: #383840; }
.label { color: #888; display: flex; flex-direction: column; align-items: center; line-height: 1; }
.full-label { font-size: .55rem; opacity: 0.7; }
.val { font-family: monospace; font-weight: bold; }
</style>
