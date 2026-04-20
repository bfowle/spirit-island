<script setup lang="ts">
import { computed } from 'vue'
import { getPaletteForSpirit, type DiscStyle } from '../spiritColors'

/**
 * A single presence disc in the chosen color + style. Reused on the board,
 * in the map view, and in the presence-track bowls.
 */

const props = withDefaults(
  defineProps<{
    slug: string
    color?: string
    style?: DiscStyle
    size?: number | string
    title?: string
  }>(),
  { size: 14, style: 'glass' },
)

const palette = computed(() => getPaletteForSpirit(props.slug, props.color))
const dim = computed(() => (typeof props.size === 'number' ? `${props.size}px` : props.size))
const cssVars = computed(() => ({
  '--disc-primary': palette.value.primary,
  '--disc-highlight': palette.value.highlight,
  '--disc-shadow': palette.value.shadow,
  width: dim.value,
  height: dim.value,
}))
</script>

<template>
  <span class="p-disc" :class="`style-${style}`" :style="cssVars" :title="title || slug"></span>
</template>

<style scoped>
.p-disc {
  display: inline-block;
  border-radius: 50%;
  flex-shrink: 0;
  position: relative;
}

.p-disc.style-glass {
  background:
    radial-gradient(circle at 35% 25%, rgba(255, 255, 255, 0.45) 0%, transparent 40%),
    radial-gradient(circle at 50% 50%, color-mix(in srgb, var(--disc-primary) 78%, transparent), color-mix(in srgb, var(--disc-shadow) 80%, transparent));
  border: 1px solid color-mix(in srgb, var(--disc-primary) 60%, #000);
  box-shadow:
    0 1px 2px rgba(0, 0, 0, 0.55),
    inset 0 1px 2px rgba(255, 255, 255, 0.35),
    inset 0 -1px 2px rgba(0, 0, 0, 0.35);
}

.p-disc.style-wood {
  background:
    repeating-linear-gradient(
      90deg,
      color-mix(in srgb, var(--disc-primary) 72%, #6b4b2a) 0,
      color-mix(in srgb, var(--disc-primary) 72%, #6b4b2a) 1px,
      color-mix(in srgb, var(--disc-primary) 60%, #3a2818) 2px,
      color-mix(in srgb, var(--disc-primary) 72%, #6b4b2a) 4px
    ),
    radial-gradient(circle at 35% 30%, color-mix(in srgb, var(--disc-highlight) 65%, #a07050), color-mix(in srgb, var(--disc-shadow) 80%, #3a2010));
  border: 1px solid color-mix(in srgb, var(--disc-shadow) 70%, #2a1808);
  box-shadow:
    0 1px 2px rgba(0, 0, 0, 0.55),
    inset 0 1px 1px rgba(255, 255, 255, 0.15),
    inset 0 -1px 2px rgba(0, 0, 0, 0.4);
}

.p-disc.style-solid {
  background: radial-gradient(circle at 40% 35%, var(--disc-highlight), var(--disc-primary) 60%, var(--disc-shadow));
  border: 1px solid var(--disc-shadow);
  box-shadow:
    0 1px 2px rgba(0, 0, 0, 0.55),
    inset 0 1px 1px rgba(255, 255, 255, 0.15),
    inset 0 -1px 2px rgba(0, 0, 0, 0.4);
}
</style>
