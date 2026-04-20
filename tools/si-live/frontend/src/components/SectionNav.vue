<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'

/**
 * Floating vertical section nav on the right edge. Pinned to viewport;
 * auto-highlights the currently-visible section via IntersectionObserver.
 * Clicking any item scrolls that section into view. Collapsible for when
 * the map is being dragged / needs full width.
 */

interface NavItem {
  id: string
  label: string
  icon: string
}

const props = defineProps<{ items: NavItem[] }>()

const activeId = ref<string>('')
const collapsed = ref<boolean>(false)

let observer: IntersectionObserver | null = null

onMounted(() => {
  if (typeof IntersectionObserver === 'undefined') return
  observer = new IntersectionObserver(
    (entries) => {
      // Pick the topmost intersecting section as active
      const visible = entries
        .filter(e => e.isIntersecting)
        .sort((a, b) => a.boundingClientRect.top - b.boundingClientRect.top)
      if (visible.length) {
        const id = visible[0].target.id
        if (id) activeId.value = id
      }
    },
    { rootMargin: '-80px 0px -60% 0px', threshold: 0 },
  )
  for (const item of props.items) {
    const el = document.getElementById(item.id)
    if (el) observer.observe(el)
  }
})

onUnmounted(() => {
  if (observer) observer.disconnect()
})

function scrollTo(id: string) {
  const el = document.getElementById(id)
  if (!el) return
  el.scrollIntoView({ behavior: 'smooth', block: 'start' })
  activeId.value = id
}
</script>

<template>
  <nav class="section-nav" :class="{ collapsed }" aria-label="Section navigation">
    <button
      class="collapse-btn"
      @click="collapsed = !collapsed"
      :title="collapsed ? 'Show nav' : 'Collapse nav'"
    >{{ collapsed ? '◀' : '▶' }}</button>
    <ul v-if="!collapsed">
      <li v-for="item in items" :key="item.id">
        <button
          type="button"
          class="nav-item"
          :class="{ active: activeId === item.id }"
          @click="scrollTo(item.id)"
          :title="`Jump to ${item.label}`"
        >
          <span class="nav-icon">{{ item.icon }}</span>
          <span class="nav-label">{{ item.label }}</span>
        </button>
      </li>
    </ul>
  </nav>
</template>

<style scoped>
.section-nav {
  position: fixed;
  top: 50%;
  right: var(--sp-2);
  transform: translateY(-50%);
  z-index: 50;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-2);
  box-shadow: 0 4px 18px rgba(0, 0, 0, 0.35);
  display: flex; flex-direction: column; gap: 4px;
  max-height: 85vh;
  overflow-y: auto;
}
.section-nav.collapsed {
  padding: var(--sp-1);
}

.collapse-btn {
  align-self: flex-end;
  padding: 2px 4px;
  font-size: 0.68rem;
  border: none;
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
}
.collapse-btn:hover { color: var(--text-primary); }

ul {
  list-style: none; margin: 0; padding: 0;
  display: flex; flex-direction: column; gap: 2px;
}

.nav-item {
  display: grid;
  grid-template-columns: 1.2rem 1fr;
  gap: var(--sp-2);
  align-items: center;
  padding: 4px var(--sp-2);
  border: 1px solid transparent;
  background: transparent;
  color: var(--text-secondary);
  font-size: 0.72rem;
  cursor: pointer;
  border-radius: var(--r-sm);
  text-align: left;
  width: 100%;
  transition: all var(--motion-fast);
}
.nav-item:hover {
  background: var(--bg-muted);
  color: var(--text-primary);
}
.nav-item.active {
  background: color-mix(in srgb, var(--accent-blue) 15%, transparent);
  color: var(--text-white);
  border-color: var(--accent-blue);
  font-weight: var(--fw-semibold);
}
.nav-icon { font-size: 0.85rem; text-align: center; }
.nav-label { white-space: nowrap; }

@media (max-width: 1100px) {
  .section-nav { display: none; }
}
</style>
