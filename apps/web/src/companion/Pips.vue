<script setup lang="ts">
import { computed } from 'vue'

/**
 * Natural-frequency pips: `hit` filled of `total`. Above `max` pips the
 * row becomes a proportional bar so it never wraps.
 */
const props = withDefaults(
  defineProps<{ hit: number; total: number; tone?: 'explore' | 'fear' | 'blight' | 'danger' | 'neutral' | 'build'; max?: number; size?: number }>(),
  { tone: 'neutral', max: 12, size: 7 },
)
const asBar = computed(() => props.total > props.max)
const cells = computed(() => Array.from({ length: Math.max(0, props.total) }, (_, i) => i < props.hit))
const width = computed(() => (props.total > 0 ? Math.min(100, (props.hit / props.total) * 100) : 0))
</script>

<template>
  <span class="pips" :class="tone" :title="`${hit} of ${total}`" role="img" :aria-label="`${hit} of ${total}`">
    <template v-if="!asBar">
      <i v-for="(on, i) in cells" :key="i" :class="{ on }" :style="{ width: `${size}px`, height: `${size}px` }"></i>
    </template>
    <span v-else class="bar"><span :style="{ width: `${width}%` }"></span></span>
  </span>
</template>

<style scoped>
.pips { display: inline-flex; align-items: center; gap: 2px; vertical-align: middle; }
.pips i { display: block; border-radius: 2px; background: var(--bg-active); }
.pips .bar { display: inline-block; width: 64px; height: 6px; border-radius: 3px; background: var(--bg-active); overflow: hidden; }
.pips .bar > span { display: block; height: 100%; border-radius: 3px; }
.neutral i.on, .neutral .bar > span { background: var(--text-secondary); }
.explore i.on, .explore .bar > span { background: var(--accent-teal); }
.build i.on, .build .bar > span { background: #fb923c; }
.fear i.on, .fear .bar > span { background: var(--accent-amber); }
.blight i.on, .blight .bar > span { background: #a78bfa; }
.danger i.on, .danger .bar > span { background: var(--accent-red); }
</style>
