<script setup lang="ts">
import type { Pools } from '../types'

defineProps<{ modelValue: Pools }>()
defineEmits<{ 'update:modelValue': [value: Pools] }>()
</script>

<template>
  <section class="pools">
    <label>Fear
      <input type="number" min="0" v-model.number="modelValue.fear_current" />
      <span class="sep">/</span>
      <input type="number" min="0" v-model.number="modelValue.fear_threshold" />
    </label>
    <label>Terror L<input type="number" min="1" max="4" v-model.number="modelValue.terror_level" /></label>
    <label>Blight
      <input type="number" min="0" v-model.number="modelValue.blight_current" />
      <span class="sep">/</span>
      <input type="number" min="0" v-model.number="modelValue.blight_cap" />
    </label>
    <label class="chk">
      <input type="checkbox" v-model="modelValue.island_blighted" />
      Island Blighted
    </label>
  </section>
</template>

<style scoped>
.pools { display: flex; gap: 1rem; flex-wrap: wrap; background: #1e1e22; padding: .75rem; border-radius: 6px; }
label { display: flex; align-items: center; gap: .35rem; font-size: .9rem; }
input[type=number] { width: 3.5rem; }
.sep { color: #888; }
.chk { gap: .25rem; }
</style>
