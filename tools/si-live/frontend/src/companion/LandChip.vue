<script setup lang="ts">
import { computed } from 'vue'
import { useCompanionCtx } from './context'
import { parseLand, terrainColor } from './format'

/** A land reference: terrain swatch + "C3". Click selects it on the map. */
const props = defineProps<{ id: string; note?: string | null; muted?: boolean }>()
const ctx = useCompanionCtx()
const info = computed(() => {
  const land = ctx?.landById.value[props.id]
  const p = parseLand(props.id)
  return { terrain: land?.terrain ?? p.terrain, short: p.short }
})
const selected = computed(() => ctx?.selected.value === props.id)
</script>

<template>
  <button
    type="button"
    class="land-chip"
    :class="{ selected, muted }"
    :title="`${info.terrain} ${info.short}${note ? ` (${note})` : ''}: show on the map`"
    @click="ctx?.selectLand(id)"
  >
    <i class="sw" :style="{ background: terrainColor(info.terrain) }" aria-hidden="true"></i>
    <span class="nm">{{ info.short }}</span>
    <span v-if="note" class="note">{{ note }}</span>
  </button>
</template>

<style scoped>
.land-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 1px 6px 1px 3px;
  border-radius: 5px;
  border: 1px solid var(--border-subtle);
  background: var(--bg-inset);
  color: var(--text-primary);
  font-size: 12px;
  font-weight: 600;
  line-height: 18px;
  cursor: pointer;
  white-space: nowrap;
}
.land-chip:hover { border-color: var(--border-strong); background: var(--bg-hover); }
.land-chip.selected { border-color: var(--accent-blue-hover); box-shadow: 0 0 0 1px var(--accent-blue-hover); }
.land-chip.muted { opacity: 0.6; }
.sw { width: 10px; height: 10px; border-radius: 2px; flex: 0 0 auto; }
.nm { font-variant-numeric: tabular-nums; }
.note { font-weight: 500; color: var(--text-secondary); font-size: 11px; }
</style>
