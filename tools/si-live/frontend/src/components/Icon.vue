<script setup lang="ts">
import { computed } from 'vue'

// Vite glob import — all icon files in src/assets/icons/ get bundled.
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
    name: string
    size?: number | string
    label?: string
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

// True for black-silhouette SVGs from the Wiki (units + the Fear/Blight/
// SacredSite resources). These need CSS polarity-flip on dark bg. Everything
// else — color PNGs + the color-keyed SVGs (FastColor, SlowColor) — keeps
// its native colors (Fast stays red, Slow stays blue).
const isMono = computed(() =>
  props.name.startsWith('unit-') ||
  props.name === 'resource-fear' ||
  props.name === 'resource-blight' ||
  props.name === 'resource-sacred-site',
)
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
    :class="{ 'si-icon-mono': isMono }"
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
/* Only monochrome silhouettes get polarity-flipped in dark mode. FastColor
   and SlowColor SVGs keep their canonical Fast=red / Slow=blue hues. */
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
