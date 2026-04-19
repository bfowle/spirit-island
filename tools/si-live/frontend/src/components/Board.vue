<script setup lang="ts">
import type { Board, Land } from '../types'
import LandEditor from './LandEditor.vue'

defineProps<{ modelValue: Board }>()
defineEmits<{ 'update:modelValue': [value: Board] }>()

function sortedKeys(b: Board): string[] {
  return Object.keys(b.lands).sort((a, z) => Number(a) - Number(z))
}

function summary(l: Land): string {
  const parts: string[] = []
  if (l.explorers) parts.push(`${l.explorers}E`)
  if (l.towns) parts.push(`${l.towns}T`)
  if (l.cities) parts.push(`${l.cities}C`)
  if (l.dahan) parts.push(`${l.dahan}D`)
  if (l.blight) parts.push(`${l.blight}⌧`)
  return parts.join(' ') || '—'
}
</script>

<template>
  <div class="grid">
    <div v-for="id in sortedKeys(modelValue)" :key="id" class="land" :class="modelValue.lands[id].terrain">
      <div class="hdr">
        <strong>#{{ id }}</strong>
        <span class="terrain">{{ modelValue.lands[id].terrain }}<span v-if="modelValue.lands[id].coastal"> · coast</span></span>
      </div>
      <div class="summary">{{ summary(modelValue.lands[id]) }}</div>
      <LandEditor v-model="modelValue.lands[id]" />
    </div>
  </div>
</template>

<style scoped>
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(190px, 1fr)); gap: .5rem; }
.land { border: 1px solid #444; border-radius: 6px; padding: .5rem; background: #1a1a1e; }
.land.mountain { border-left: 4px solid #a67; }
.land.wetland  { border-left: 4px solid #4a8; }
.land.jungle   { border-left: 4px solid #6a4; }
.land.sands    { border-left: 4px solid #ca6; }
.hdr { display: flex; justify-content: space-between; font-size: .85rem; color: #ccc; }
.terrain { text-transform: capitalize; color: #999; }
.summary { font-family: monospace; margin: .25rem 0; color: #fca; }
</style>
