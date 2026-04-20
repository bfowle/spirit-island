<script setup lang="ts">
import { computed } from 'vue'

// Vite glob import — all icon files in src/assets/icons/ get bundled.
// Keys look like '/src/assets/icons/element-moon.png'; values are resolved URLs.
const iconModules = import.meta.glob('../assets/icons/*.{svg,png}', {
  eager: true,
  query: '?url',
  import: 'default',
})

const iconMap: Record<string, string> = {}
for (const [path, url] of Object.entries(iconModules)) {
  const match = path.match(/\/([^/]+)\.(svg|png)$/)
  if (match) iconMap[match[1]] = url as string
}

const props = withDefaults(
  defineProps<{
    /**
     * Icon slug. Examples:
     *   element: "element-moon", "element-fire", …
     *   unit:    "unit-explorer", "unit-town", "unit-city", "unit-dahan"
     *   resource:"resource-fear", "resource-blight", "resource-sacred-site"
     *   speed:   "speed-fast", "speed-slow"
     *   terrain: "terrain-mountain", "terrain-wetland", "terrain-jungle", "terrain-sands"
     */
    name: string
    /** Pixel size (short side). Default 16. */
    size?: number | string
    /** Optional override alt/title text. Defaults to humanized name. */
    label?: string
    /** Hide from screen readers when decorative. */
    decorative?: boolean
  }>(),
  { size: 16, decorative: false },
)

const src = computed(() => iconMap[props.name] || '')
const dim = computed(() => (typeof props.size === 'number' ? `${props.size}px` : props.size))
const altText = computed(() => {
  if (props.decorative) return ''
  if (props.label) return props.label
  return props.name.replace(/^[a-z]+-/, '').replace(/-/g, ' ')
})
</script>

<script lang="ts">
// extra block so we can compute an SVG class; keeps the component tidy
</script>

<template>
  <img
    v-if="src"
    :src="src"
    :alt="altText"
    :title="altText || undefined"
    :aria-hidden="decorative || undefined"
    :width="dim"
    :height="dim"
    class="si-icon"
    :class="{ 'si-icon-mono': src.endsWith('.svg') }"
  />
  <span v-else class="si-icon-missing" :title="`missing icon: ${name}`">?</span>
</template>

<style scoped>
.si-icon {
  display: inline-block;
  vertical-align: middle;
  object-fit: contain;
  user-select: none;
  flex-shrink: 0;
}
/* Black silhouette SVGs from the Wiki need to flip polarity in dark mode so
   they stay visible. PNG element icons (colored) skip this. */
.si-icon-mono {
  filter: var(--icon-svg-filter);
}
.si-icon-missing {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 1em;
  height: 1em;
  background: var(--bg-muted);
  color: var(--text-muted);
  border-radius: var(--r-sm);
  font-size: 0.75em;
}
</style>
