<script setup lang="ts">
import { ref } from 'vue'

/**
 * Phase-aware section wrapper. When `collapsed` is true, renders just a
 * clickable strip with the title + metric chips; expands on click. When not
 * collapsed, renders the default slot as normal.
 *
 * Lets the dashboard fit on one viewport by hiding irrelevant-for-current-phase
 * sections while keeping their key metrics visible at a glance.
 */

const props = defineProps<{
  title: string
  collapsed: boolean
  /** Short metric chips to show on the strip when collapsed. */
  metric?: string
  /** Optional icon emoji or label to prefix the title. */
  icon?: string
}>()

const forceExpanded = ref(false)
function toggleExpanded() { forceExpanded.value = !forceExpanded.value }
</script>

<template>
  <div
    class="section-strip-wrap"
    :class="{ 'is-collapsed': props.collapsed && !forceExpanded }"
  >
    <div v-if="props.collapsed && !forceExpanded" class="strip" @click="toggleExpanded" :title="`Expand ${props.title}`">
      <span v-if="props.icon" class="strip-icon">{{ props.icon }}</span>
      <span class="strip-title">{{ props.title }}</span>
      <span v-if="props.metric" class="strip-metric">{{ props.metric }}</span>
      <span class="strip-chevron">▸</span>
    </div>
    <slot v-else />
    <button
      v-if="forceExpanded && props.collapsed"
      class="ghost collapse-btn"
      @click="forceExpanded = false"
      title="Collapse back to strip"
    >▾ Collapse</button>
  </div>
</template>

<style scoped>
.section-strip-wrap {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
  position: relative;
}
.section-strip-wrap.is-collapsed { }

.strip {
  display: grid;
  grid-template-columns: auto 1fr auto auto;
  gap: var(--sp-2);
  align-items: center;
  padding: var(--sp-1) var(--sp-3);
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  cursor: pointer;
  transition: all var(--motion-fast);
  font-size: var(--fs-sm);
}
.strip:hover {
  background: var(--bg-muted);
  border-color: var(--accent-blue);
}
.strip-icon { font-size: 1rem; }
.strip-title {
  font-weight: var(--fw-semibold);
  color: var(--text-primary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-size: var(--fs-xs);
}
.strip-metric {
  font-family: var(--font-mono);
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  text-align: right;
}
.strip-chevron {
  color: var(--text-muted);
  transition: transform var(--motion-fast);
  font-size: 0.9rem;
}
.strip:hover .strip-chevron { color: var(--accent-blue); transform: translateX(2px); }

.collapse-btn {
  position: absolute;
  top: var(--sp-2);
  right: var(--sp-2);
  font-size: var(--fs-xs);
  padding: 2px var(--sp-2);
  z-index: 5;
}
</style>
