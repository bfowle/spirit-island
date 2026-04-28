<script setup lang="ts">
/**
 * Small expansion icon badge. Pass the expansion slug (e.g. "base",
 * "jagged-earth", "branch-and-claw", "nature-incarnate",
 * "horizons-of-spirit-island", "promo-1", "promo-2") and the component
 * renders the Wiki "Main Page Button" icon + an accessible label.
 *
 * Icons are fetched once into `public/expansion-icons/` by the dev setup;
 * slugs that have no file fall back to text.
 */
import { computed } from 'vue'

const props = withDefaults(
  defineProps<{
    slug?: string | null
    size?: number
    /** If true, also render the expansion name next to the icon. */
    showLabel?: boolean
  }>(),
  { slug: null, size: 16, showLabel: false },
)

const NAME_BY_SLUG: Record<string, string> = {
  'base': 'Spirit Island (Base)',
  'branch-and-claw': 'Branch & Claw',
  'jagged-earth': 'Jagged Earth',
  'nature-incarnate': 'Nature Incarnate',
  'horizons-of-spirit-island': 'Horizons of Spirit Island',
  'hosi': 'Horizons of Spirit Island',
  'promo-1': 'Promo Pack 1',
  'promo-2': 'Promo Pack 2',
}

// Slugs that have an actual icon file in public/expansion-icons/. Base has
// no icon file because real Spirit Island cards have no corner symbol for
// base-game content — the *absence* of a symbol is the visual signal.
const SLUGS_WITH_ICON = new Set([
  'branch-and-claw',
  'jagged-earth',
  'nature-incarnate',
  'horizons-of-spirit-island',
  'promo-1',
  'promo-2',
])

const canonicalSlug = computed(() => {
  const s = (props.slug || '').toLowerCase().replace(/_/g, '-').trim()
  // Source data is inconsistent — spirits.json uses "horizons", boards use
  // "horizons-of-spirit-island", and some places shorthand "hosi". Normalize
  // them all to the board canonical form.
  if (s === 'hosi' || s === 'horizons') return 'horizons-of-spirit-island'
  return s
})

const displayName = computed(() => {
  return NAME_BY_SLUG[canonicalSlug.value] || props.slug || '—'
})

const iconSrc = computed(() => {
  if (!SLUGS_WITH_ICON.has(canonicalSlug.value)) return null
  return `/expansion-icons/${canonicalSlug.value}.png`
})

const isBase = computed(() => canonicalSlug.value === 'base')
</script>

<template>
  <span class="exp-badge" :title="displayName">
    <img
      v-if="iconSrc"
      :src="iconSrc"
      :alt="displayName"
      :width="size"
      :height="size"
      class="exp-icon"
      loading="lazy"
    />
    <span v-else-if="isBase" class="exp-base" aria-label="Base game">SI</span>
    <span v-else-if="!slug" class="exp-fallback">—</span>
    <span v-else class="exp-fallback">{{ slug }}</span>
    <span v-if="showLabel" class="exp-label">{{ displayName }}</span>
  </span>
</template>

<style scoped>
.exp-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  vertical-align: middle;
}
.exp-icon {
  display: inline-block;
  object-fit: contain;
  border-radius: 3px;
  background: rgba(255, 255, 255, 0.04);
  flex: 0 0 auto;
}
.exp-fallback {
  font-size: 0.72rem;
  color: var(--text-muted, #888);
  font-family: var(--font-mono, monospace);
  text-transform: lowercase;
}
.exp-base {
  display: inline-grid;
  place-items: center;
  width: 18px; height: 18px;
  font-size: 9px;
  font-weight: 700;
  letter-spacing: 0.04em;
  color: var(--text-muted, #aaa);
  background: rgba(255, 255, 255, 0.06);
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 4px;
  font-family: var(--font-mono, monospace);
}
.exp-label {
  font-size: 0.72rem;
  color: var(--text-muted, #888);
}
</style>
