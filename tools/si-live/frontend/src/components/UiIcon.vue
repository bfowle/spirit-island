<script setup lang="ts">
/**
 * UiIcon — monochrome UI-chrome glyphs. Inline SVG, currentColor-aware, no
 * extra HTTP request per icon. Use this for app chrome (save, settings, trash,
 * check, etc.). Prefer `<Icon>` (SI wiki iconography) for game tokens —
 * elements, units, terrain, speed, fear/blight.
 *
 * Replaces the emoji set (trophy, skull, target, moon, ghost, swords, scroll,
 * save, archive, settings, search, chart, camera, trash, etc.) that used to
 * litter the app as native Unicode pictographs.
 */
import { computed } from 'vue'

const props = withDefaults(
  defineProps<{
    name: string
    size?: number | string
    /** Stroke width in user units. Default 2. */
    stroke?: number
    /** When true, adds aria-hidden and drops the accessible label. */
    decorative?: boolean
    /** Optional accessible label; defaults to the icon name. */
    label?: string
  }>(),
  { size: 16, stroke: 2, decorative: false },
)

const dim = computed(() => (typeof props.size === 'number' ? `${props.size}px` : props.size))

// Each entry is the inner markup of a 24×24 SVG. Paths use stroke="currentColor"
// so hooking into any container color Just Works. Kept lucide-style for consistency.
// Add new icons by appending to the map — no build step, no assets folder.
const PATHS: Record<string, string> = {
  // End-state badges
  trophy: `<path d="M8 21h8"/><path d="M12 17v4"/><path d="M7 4h10v5a5 5 0 0 1-10 0V4z"/><path d="M17 6h3a2 2 0 0 1 2 2v1a4 4 0 0 1-4 4"/><path d="M7 6H4a2 2 0 0 0-2 2v1a4 4 0 0 0 4 4"/>`,
  skull: `<circle cx="9" cy="12" r="1.5"/><circle cx="15" cy="12" r="1.5"/><path d="M12 17l-1 2h2l-1-2z"/><path d="M4 15a8 8 0 1 1 16 0v3a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-3z"/>`,
  target: `<circle cx="12" cy="12" r="9"/><circle cx="12" cy="12" r="5"/><circle cx="12" cy="12" r="1.5"/>`,

  // Game-piece / deck glyphs (abstract, non-emoji)
  swords: `<path d="M14.5 17.5 3 6V3h3l11.5 11.5"/><path d="M13 19l6-6"/><path d="M16 16l4 4"/><path d="M19 21l2-2"/><path d="M14.5 6.5 18 3h3v3l-3.5 3.5"/><path d="M5 14l-2 2v3h3l2-2"/>`,
  ghost: `<path d="M9 10h.01"/><path d="M15 10h.01"/><path d="M12 2a8 8 0 0 0-8 8v12l3-2 3 2 2-2 2 2 3-2 3 2V10a8 8 0 0 0-8-8z"/>`,
  scroll: `<path d="M8 21h9a3 3 0 0 0 3-3v-1a2 2 0 0 0-2-2H9"/><path d="M15 3H6a3 3 0 0 0-3 3v1a2 2 0 0 0 2 2h10"/><path d="M6 21a3 3 0 0 1-3-3V7"/><path d="M21 6V5a2 2 0 0 0-2-2h-4"/>`,
  moon: `<path d="M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z"/>`,

  // App chrome
  save: `<path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/><path d="M17 21v-8H7v8"/><path d="M7 3v5h8"/>`,
  archive: `<rect x="3" y="4" width="18" height="4" rx="1"/><path d="M5 8v11a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8"/><path d="M10 12h4"/>`,
  settings: `<circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09a1.65 1.65 0 0 0 1.51-1 1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33h.01A1.65 1.65 0 0 0 10 3.09V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51h.01a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82v.01A1.65 1.65 0 0 0 20.91 10H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>`,
  search: `<circle cx="11" cy="11" r="7"/><path d="M21 21l-4.3-4.3"/>`,
  chart: `<path d="M3 3v18h18"/><path d="M7 16l4-4 3 3 5-6"/>`,
  camera: `<path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"/><circle cx="12" cy="13" r="4"/>`,
  trash: `<path d="M3 6h18"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/><path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/><path d="M10 11v6"/><path d="M14 11v6"/>`,
  undo: `<path d="M3 7v6h6"/><path d="M21 17a9 9 0 0 0-15-6.7L3 13"/>`,
  user: `<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/>`,
  plus: `<path d="M12 5v14"/><path d="M5 12h14"/>`,
  chevronDown: `<path d="M6 9l6 6 6-6"/>`,
  download: `<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="M7 10l5 5 5-5"/><path d="M12 15V3"/>`,
  check: `<path d="M5 12l5 5L20 7"/>`,
  sparkle: `<path d="M12 3l1.9 5.6L19 10l-5.1 1.4L12 17l-1.9-5.6L5 10l5.1-1.4z"/>`,
  bolt: `<path d="M13 2 3 14h8l-1 8 10-12h-8l1-8z"/>`,
  flame: `<path d="M8.5 14.5A2.5 2.5 0 0 0 11 17a2.5 2.5 0 0 0 2.5-2.5c0-2.5-2.5-3-2.5-6 0 2.5-2.5 3-2.5 6z"/><path d="M13 2.5c2 4 5 5 5 9.5A6 6 0 0 1 6 12c0-3 1.5-4.5 3-6"/>`,
  eye: `<path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/>`,
}

const inner = computed(() => PATHS[props.name] || '')
const altText = computed(() => (props.decorative ? '' : props.label || props.name))
</script>

<template>
  <svg
    v-if="inner"
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    fill="none"
    :stroke="'currentColor'"
    :stroke-width="stroke"
    stroke-linecap="round"
    stroke-linejoin="round"
    class="ui-icon"
    :style="{ width: dim, height: dim }"
    :aria-hidden="decorative || undefined"
    :aria-label="altText || undefined"
    :role="decorative ? undefined : 'img'"
    v-html="inner"
  />
  <span v-else class="ui-icon-missing" :title="`missing ui-icon: ${name}`">?</span>
</template>

<style scoped>
.ui-icon {
  display: inline-block;
  vertical-align: middle;
  flex-shrink: 0;
  color: currentColor;
}
.ui-icon-missing {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 1em;
  height: 1em;
  font-size: 0.75em;
  color: var(--text-muted);
  background: var(--bg-muted, rgba(0,0,0,0.2));
  border-radius: var(--r-sm, 4px);
}
</style>
