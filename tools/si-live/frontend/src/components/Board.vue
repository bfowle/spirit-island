<script setup lang="ts">
import { ref } from 'vue'
import type { Board, Land } from '../types'
import LandEditor from './LandEditor.vue'

const props = defineProps<{ modelValue: Board; boardId: string }>()
defineEmits<{ 'update:modelValue': [value: Board] }>()

const saveStatus = ref<string>('')

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
    saveStatus.value = 'saved ✓'
    setTimeout(() => { saveStatus.value = '' }, 3000)
  } catch (e) {
    saveStatus.value = `error: ${(e as Error).message}`
  }
}
</script>

<template>
  <div class="board-wrap">
    <div class="board-tools">
      <div class="legend">
        <span><strong>E</strong>xplorer · <strong>T</strong>own · <strong>C</strong>ity · <strong>D</strong>ahan · <strong>⌧</strong> Blight</span>
      </div>
      <button class="save-geo" @click="saveGeometry" :title="'Persist corrected terrain/coastal back to data/boards/*.json so future New Games use your corrections'">
        Save Board Geometry
      </button>
      <span class="save-status" :class="{ error: saveStatus.startsWith('error') }">{{ saveStatus }}</span>
    </div>
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
  </div>
</template>

<style scoped>
.board-wrap { }
.board-tools { display: flex; gap: 1rem; align-items: center; margin-bottom: .5rem; flex-wrap: wrap; }
.legend { font-size: .75rem; color: #999; }
.legend strong { color: #fca; }
.save-geo { background: #2a3a2a; border: 1px solid #5a7a5a; color: #eee; padding: .2rem .6rem; border-radius: 4px; cursor: pointer; font-size: .8rem; }
.save-geo:hover { background: #3a4a3a; }
.save-status { font-size: .8rem; color: #8a8; font-style: italic; }
.save-status.error { color: #faa; font-style: normal; }
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: .5rem; }
.land { border: 1px solid #444; border-radius: 6px; padding: .5rem; background: #1a1a1e; }
.land.mountain { border-left: 4px solid #a67; }
.land.wetland  { border-left: 4px solid #4a8; }
.land.jungle   { border-left: 4px solid #6a4; }
.land.sands    { border-left: 4px solid #ca6; }
.hdr { display: flex; justify-content: space-between; font-size: .85rem; color: #ccc; }
.terrain { text-transform: capitalize; color: #999; }
.summary { font-family: monospace; margin: .25rem 0; color: #fca; }
</style>
