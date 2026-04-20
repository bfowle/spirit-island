<script setup lang="ts">
/**
 * DeckStation.vue — Compact deck widget with expandable detail.
 *
 * Shows a mini-view by default (top card + counter), expands inline when
 * the deck is primary for the current phase. Used for Invader, Fear, Event.
 */
import { computed } from 'vue'

const props = defineProps<{
  deck: 'invader' | 'fear' | 'event'
  topCard: string | null
  count: number
  secondaryLabel?: string
  expanded?: boolean
}>()

const emit = defineEmits<{
  (e: 'toggle'): void
}>()

const accentColor = computed(() => {
  switch (props.deck) {
    case 'invader': return 'var(--accent-red)'
    case 'fear': return 'var(--accent-amber)'
    case 'event': return 'var(--accent-purple)'
    default: return 'var(--accent-blue)'
  }
})

const deckLabel = computed(() => {
  switch (props.deck) {
    case 'invader': return 'Invader'
    case 'fear': return 'Fear'
    case 'event': return 'Event'
    default: return 'Deck'
  }
})
</script>

<template>
  <div
    class="deck-station"
    :class="{ expanded: expanded }"
    :style="{ '--deck-accent': accentColor }"
  >
    <button class="station-header" @click="emit('toggle')">
      <div class="header-left">
        <span class="deck-indicator"></span>
        <span class="deck-name">{{ deckLabel }}</span>
      </div>
      <div class="header-right">
        <span v-if="topCard" class="top-card">{{ topCard }}</span>
        <span class="deck-count">{{ count }}</span>
        <span class="expand-icon">{{ expanded ? '−' : '+' }}</span>
      </div>
    </button>

    <div class="station-body">
      <slot></slot>
    </div>

    <div v-if="secondaryLabel" class="station-footer">
      <span class="secondary-label">{{ secondaryLabel }}</span>
    </div>
  </div>
</template>

<style scoped>
.deck-station {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-left: 3px solid var(--deck-accent);
  border-radius: var(--r-md);
  overflow: hidden;
  transition: all var(--motion-base);
}

.station-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: var(--sp-2);
  background: transparent;
  border: none;
  cursor: pointer;
  color: var(--text-primary);
  font-family: inherit;
  transition: background var(--motion-fast);
}

.station-header:hover {
  background: var(--bg-hover);
}

.header-left {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.deck-indicator {
  width: 8px;
  height: 8px;
  border-radius: var(--r-full);
  background: var(--deck-accent);
}

.deck-name {
  font-size: var(--fs-xs);
  font-weight: var(--fw-semibold);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-secondary);
}

.header-right {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.top-card {
  font-size: var(--fs-xs);
  font-family: var(--font-mono);
  color: var(--deck-accent);
  max-width: 100px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.deck-count {
  font-size: var(--fs-xs);
  font-family: var(--font-mono);
  padding: 2px 6px;
  background: var(--bg-muted);
  border-radius: var(--r-sm);
  color: var(--text-muted);
}

.expand-icon {
  font-size: var(--fs-lg);
  font-weight: var(--fw-bold);
  color: var(--text-muted);
  width: 20px;
  text-align: center;
}

.station-body {
  max-height: 0;
  overflow: hidden;
  opacity: 0;
  transition: all var(--motion-base);
}

.deck-station.expanded .station-body {
  max-height: 400px;
  opacity: 1;
  padding: var(--sp-2);
  padding-top: 0;
}

.station-footer {
  padding: var(--sp-1) var(--sp-2);
  border-top: 1px solid var(--border-subtle);
  background: var(--bg-canvas);
}

.secondary-label {
  font-size: var(--fs-xxs);
  color: var(--text-muted);
}
</style>
