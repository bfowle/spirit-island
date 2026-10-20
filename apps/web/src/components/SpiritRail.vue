<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { GameState } from '../types'
import SpiritPanel from './SpiritPanel.vue'

/**
 * Right-rail persistent spirit panel. Always visible regardless of phase so
 * the player never hunts for it when playing cards. For multi-spirit games,
 * the header shows a tab strip; the body renders the active spirit's full
 * `SpiritPanel` internals.
 */

const props = defineProps<{ state: GameState }>()
const emit = defineEmits<{
  'log-event': [event: string, details: Record<string, unknown>]
}>()

const spiritSlugs = computed(() => Object.keys(props.state.spirits ?? {}))
const activeSlug = ref<string>(spiritSlugs.value[0] ?? '')

// Keep active selection valid when spirits change (new game, load, etc.)
watch(spiritSlugs, (slugs) => {
  if (!slugs.includes(activeSlug.value) && slugs.length > 0) {
    activeSlug.value = slugs[0]
  }
}, { immediate: true })

function humanName(slug: string): string {
  const parts = slug.split('-')
  // "shadows-flicker-like-flame" → "Shadows"
  return parts[0].charAt(0).toUpperCase() + parts[0].slice(1)
}
function subtitle(slug: string): string {
  const parts = slug.split('-').slice(1)
  if (!parts.length) return ''
  return parts.map(p => p.charAt(0).toUpperCase() + p.slice(1)).join(' ')
}
function initials(slug: string): string {
  return slug.split('-').map(p => p.charAt(0).toUpperCase()).slice(0, 2).join('')
}
</script>

<template>
  <aside class="spirit-rail">
    <div class="rail-header">
      <div v-if="activeSlug" class="rail-title">
        <span class="rail-name brand">{{ humanName(activeSlug).toUpperCase() }}</span>
        <span class="rail-sub">{{ subtitle(activeSlug) }}</span>
      </div>
      <div v-else class="rail-title rail-empty">
        <span class="rail-sub">No spirit in play</span>
      </div>

      <div v-if="spiritSlugs.length > 1" class="rail-tabs">
        <button
          v-for="slug in spiritSlugs"
          :key="slug"
          type="button"
          class="rail-tab"
          :class="{ active: activeSlug === slug }"
          :title="humanName(slug) + ' ' + subtitle(slug)"
          @click="activeSlug = slug"
        >{{ initials(slug) }}</button>
      </div>
    </div>

    <div class="rail-body">
      <SpiritPanel
        v-if="activeSlug && state.spirits[activeSlug]"
        v-model="state.spirits[activeSlug]"
        :slug="activeSlug"
        :round="state.round"
        @log-event="(event, details) => emit('log-event', event, details)"
      />
      <div v-else class="empty-state">
        Start a new game to see your spirit here.
      </div>
    </div>
  </aside>
</template>

<style scoped>
.spirit-rail {
  border-left: 1px solid var(--border-subtle);
  background: var(--bg-surface);
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
  height: 100%;
}

.rail-header {
  flex: 0 0 56px;
  height: 56px;
  padding: 0 16px;
  border-bottom: 1px solid var(--border-subtle);
  display: flex; align-items: center; gap: 12px;
}
.rail-title { display: flex; flex-direction: column; min-width: 0; }
.rail-name { font-size: 13px; letter-spacing: 0.04em; color: var(--text-white); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.rail-sub { font-size: 10px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.1em; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.rail-empty { color: var(--text-muted); font-style: italic; }

.rail-tabs { margin-left: auto; display: flex; gap: 4px; }
.rail-tab {
  width: 26px; height: 26px; border-radius: var(--r-sm);
  border: 1px solid var(--border-subtle); background: var(--bg-inset);
  color: var(--text-secondary); cursor: pointer;
  font-size: 11px; font-weight: var(--fw-semibold);
  font-family: var(--font-mono);
  transition: all 150ms ease;
  padding: 0;
}
.rail-tab.active {
  background: var(--accent-grad);
  color: white;
  border-color: transparent;
  box-shadow: var(--shadow-glow);
}
.rail-tab:hover:not(.active) { background: var(--bg-elevated); color: var(--text-primary); }

.rail-body {
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
  padding: 12px 14px;
}

.empty-state {
  padding: 24px;
  text-align: center;
  color: var(--text-muted);
  font-size: var(--fs-sm);
  font-style: italic;
}
</style>
