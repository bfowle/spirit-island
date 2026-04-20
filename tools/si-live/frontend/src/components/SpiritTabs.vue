<script setup lang="ts">
/**
 * SpiritTabs.vue — Multi-spirit tab strip with mini-panel indicators.
 *
 * Shows a horizontal tab bar for selecting between spirits, with quick
 * stats (energy, card plays, hand count, elements) on each tab. Non-active
 * spirits can also be shown as mini-panels below.
 */
import { computed } from 'vue'
import type { Spirit } from '../types'

const props = defineProps<{
  spirits: Record<string, Spirit>
  activeSlug: string | null
  showMiniPanels?: boolean
}>()

const emit = defineEmits<{
  (e: 'update:activeSlug', slug: string): void
}>()

const spiritSlugs = computed(() => Object.keys(props.spirits))

function humanSlug(slug: string): string {
  return slug
    .split('-')
    .map(p => p.charAt(0).toUpperCase() + p.slice(1))
    .join(' ')
}

function shortName(slug: string): string {
  return humanSlug(slug).split(' ').slice(0, 2).join(' ')
}

function getMiniData(slug: string) {
  const spirit = props.spirits[slug]
  if (!spirit) return null
  return {
    slug,
    name: shortName(slug),
    energy: spirit.energy ?? 0,
    cardPlays: spirit.card_plays ?? 1,
    handCount: spirit.hand?.length ?? 0,
    discardCount: spirit.discard?.length ?? 0,
    elements: spirit.elements_this_turn ?? {},
  }
}

const ELEMENT_COLORS: Record<string, string> = {
  sun: '#d9a45e',
  moon: '#7bb8f5',
  fire: '#dc2f02',
  air: '#c77dff',
  water: '#2a9d8f',
  earth: '#b85c64',
  plant: '#6fa661',
  animal: '#e9c46a',
}
</script>

<template>
  <div class="spirit-tabs-container">
    <!-- Tab strip -->
    <div class="tabs-strip">
      <button
        v-for="slug in spiritSlugs"
        :key="slug"
        type="button"
        class="spirit-tab"
        :class="{ active: activeSlug === slug }"
        @click="emit('update:activeSlug', slug)"
      >
        <div class="tab-main">
          <span class="tab-name">{{ shortName(slug) }}</span>
          <div class="tab-stats">
            <span class="stat energy">{{ spirits[slug]?.energy ?? 0 }}E</span>
            <span class="stat cards">{{ spirits[slug]?.hand?.length ?? 0 }}H</span>
          </div>
        </div>
        <div class="tab-elements">
          <span
            v-for="(count, el) in (spirits[slug]?.elements_this_turn ?? {})"
            :key="String(el)"
            class="element-pip"
            :style="{ background: ELEMENT_COLORS[String(el)] + '33', color: ELEMENT_COLORS[String(el)] }"
          >{{ count }}</span>
        </div>
      </button>
    </div>

    <!-- Mini-panels strip for non-active spirits -->
    <div v-if="showMiniPanels && spiritSlugs.length > 1" class="minis-strip">
      <div
        v-for="slug in spiritSlugs.filter(s => s !== activeSlug)"
        :key="slug"
        class="mini-panel"
        @click="emit('update:activeSlug', slug)"
      >
        <span class="mini-name">{{ humanSlug(slug).split(' ')[0] }}</span>
        <div class="mini-stats">
          <span class="mini-stat energy">{{ spirits[slug]?.energy ?? 0 }}E</span>
          <span class="mini-stat plays">{{ spirits[slug]?.card_plays ?? 1 }}P</span>
          <span class="mini-stat hand">{{ spirits[slug]?.hand?.length ?? 0 }}H</span>
        </div>
        <div class="mini-elements">
          <span
            v-for="(count, el) in (spirits[slug]?.elements_this_turn ?? {})"
            :key="String(el)"
            class="mini-el"
            :style="{ background: ELEMENT_COLORS[String(el)] + '33', color: ELEMENT_COLORS[String(el)] }"
          >{{ count }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.spirit-tabs-container {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.tabs-strip {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}

.spirit-tab {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: var(--sp-2);
  min-width: 100px;
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  cursor: pointer;
  transition: all var(--motion-fast);
  text-align: left;
  color: var(--text-secondary);
}

.spirit-tab:hover {
  background: var(--bg-muted);
  border-color: var(--border-default);
  color: var(--text-primary);
}

.spirit-tab.active {
  background: var(--accent-soft);
  border-color: var(--accent-blue);
  color: var(--text-white);
}

.tab-main {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: var(--sp-2);
}

.tab-name {
  font-size: var(--fs-sm);
  font-weight: var(--fw-semibold);
  white-space: nowrap;
}

.tab-stats {
  display: flex;
  gap: var(--sp-1);
  font-size: var(--fs-xxs);
  font-family: var(--font-mono);
}

.stat.energy { color: var(--pool-energy); }
.stat.cards { color: var(--text-muted); }
.spirit-tab.active .stat.cards { color: var(--text-secondary); }

.tab-elements {
  display: flex;
  gap: 2px;
  flex-wrap: wrap;
}

.element-pip {
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.6rem;
  font-weight: var(--fw-bold);
  border-radius: var(--r-full);
}

/* Mini-panels strip */
.minis-strip {
  display: flex;
  gap: var(--sp-2);
  flex-wrap: wrap;
}

.mini-panel {
  flex: 1;
  min-width: 140px;
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-1) var(--sp-2);
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  cursor: pointer;
  transition: all var(--motion-fast);
}

.mini-panel:hover {
  background: var(--bg-muted);
  border-color: var(--border-default);
}

.mini-name {
  font-size: var(--fs-xs);
  font-weight: var(--fw-semibold);
  color: var(--text-secondary);
  min-width: 60px;
}

.mini-stats {
  display: flex;
  gap: var(--sp-1);
  font-size: 0.68rem;
  font-family: var(--font-mono);
}

.mini-stat {
  padding: 1px 4px;
  border-radius: var(--r-sm);
  background: var(--bg-muted);
}

.mini-stat.energy { color: var(--pool-energy); }
.mini-stat.plays { color: var(--accent-purple); }
.mini-stat.hand { color: var(--text-muted); }

.mini-elements {
  display: flex;
  gap: 2px;
  margin-left: auto;
}

.mini-el {
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.6rem;
  font-weight: var(--fw-bold);
  border-radius: var(--r-full);
}
</style>
