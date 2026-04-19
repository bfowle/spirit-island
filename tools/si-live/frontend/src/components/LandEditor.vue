<script setup lang="ts">
import type { Land } from '../types'
import { UNIT_KEYS, type UnitKey } from '../types'

const props = defineProps<{ modelValue: Land }>()

function bump(key: UnitKey, delta: number) {
  const current = (props.modelValue[key] as number | undefined) ?? 0
  const next = Math.max(0, current + delta)
  props.modelValue[key] = next as never
}

const LABELS: Record<UnitKey, string> = {
  explorers: 'E',
  towns: 'T',
  cities: 'C',
  dahan: 'D',
  blight: '⌧',
}
</script>

<template>
  <div class="editor">
    <div v-for="k in UNIT_KEYS" :key="k" class="row">
      <button @click="bump(k, -1)" aria-label="decrement">−</button>
      <span class="label">{{ LABELS[k] }}</span>
      <span class="val">{{ (modelValue[k] as number) ?? 0 }}</span>
      <button @click="bump(k, 1)" aria-label="increment">+</button>
    </div>
  </div>
</template>

<style scoped>
.editor { display: grid; grid-template-columns: repeat(5, 1fr); gap: .15rem; margin-top: .35rem; }
.row { display: flex; flex-direction: column; align-items: center; font-size: .8rem; }
button { background: #2a2a30; border: 1px solid #444; color: #eee; cursor: pointer; padding: 0 .35rem; border-radius: 3px; }
button:hover { background: #383840; }
.label { color: #888; }
.val { font-family: monospace; font-weight: bold; }
</style>
