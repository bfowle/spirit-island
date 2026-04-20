<script setup lang="ts">
import { computed } from 'vue'

/**
 * Renders a row of presence bowls with discs. Each bowl shows a printed
 * income value (e.g., "0 Energy" or "1 Card Play"). Covered bowls (disc
 * still present) render a disc icon; uncovered bowls show the value.
 *
 * Data model: `fullTrack` is the immutable ordered list of slot tokens
 * from the spirit's Wiki JSON (e.g., ["energy0", "energy1", "energy3", …]).
 * `coveredTokens` is the subset currently in state — which slots still have
 * their disc. By convention, discs uncover left-to-right.
 *
 * v-model emits the updated `coveredTokens` array when the user toggles a disc.
 */

const props = defineProps<{
  fullTrack: string[]          // all slot tokens, in panel order
  coveredTokens: string[]      // tokens still covered (discs still present)
  label: string                // "Energy" or "Card Plays"
  /** Optional token → decorator override (e.g., element suffix).  */
  decorate?: (token: string) => string | undefined
}>()

const emit = defineEmits<{ 'update:coveredTokens': [value: string[]] }>()

interface Slot {
  token: string
  index: number
  covered: boolean
  value: string
  rawSuffix: string
}

function parseToken(token: string): { value: string; suffix: string } {
  // energy0, energy1, energy3, card1, card2, card3, … possibly with suffixes
  // like energy3animal (NI spirits).
  const m = token.match(/^([a-z]+)(\d+)(.*)$/i)
  if (!m) return { value: token, suffix: '' }
  const [, , digits, suffix] = m
  return { value: digits, suffix }
}

const slots = computed((): Slot[] => {
  // Mark slots as covered/uncovered. Strategy: the coveredTokens array
  // always contains the LEFTMOST remaining slots in fullTrack. So we
  // count how many in fullTrack are uncovered by comparing lengths.
  const uncoveredCount = Math.max(0, props.fullTrack.length - props.coveredTokens.length)
  return props.fullTrack.map((token, i) => {
    const { value, suffix } = parseToken(token)
    return {
      token,
      index: i,
      covered: i >= uncoveredCount,
      value,
      rawSuffix: suffix,
    }
  })
})

function toggle(slot: Slot) {
  // Toggle disc: if currently covered, remove it (place presence); if
  // uncovered, restore (undo a placement). Only the ROW can make a sensible
  // next-adjacent move — we only allow toggling the boundary slot.
  const uncoveredCount = props.fullTrack.length - props.coveredTokens.length
  const nextToUncoverIndex = uncoveredCount
  const lastUncoveredIndex = uncoveredCount - 1

  let newCovered: string[]
  if (slot.index === nextToUncoverIndex) {
    // removing the leftmost covered disc (standard play action)
    newCovered = props.coveredTokens.slice(1)
  } else if (slot.index === lastUncoveredIndex) {
    // restoring the rightmost uncovered slot (undo)
    newCovered = [props.fullTrack[lastUncoveredIndex], ...props.coveredTokens]
  } else {
    return // non-boundary clicks are no-ops
  }
  emit('update:coveredTokens', newCovered)
}

function canToggle(slot: Slot): boolean {
  const uncoveredCount = props.fullTrack.length - props.coveredTokens.length
  return slot.index === uncoveredCount || slot.index === uncoveredCount - 1
}

const placedCount = computed(() => props.fullTrack.length - props.coveredTokens.length)
</script>

<template>
  <div class="presence-track">
    <div class="track-hdr">
      <span class="track-label">{{ label }} Track</span>
      <span class="placed">{{ placedCount }} / {{ fullTrack.length }} placed</span>
    </div>
    <div class="bowls">
      <button
        v-for="slot in slots"
        :key="slot.token"
        class="bowl"
        :class="{ covered: slot.covered, uncovered: !slot.covered, clickable: canToggle(slot) }"
        :disabled="!canToggle(slot)"
        :title="slot.covered ? `Click to place this presence on the board (uncovers: ${slot.value})` : `Click to return this presence to the track`"
        @click="toggle(slot)"
      >
        <span class="bowl-value">{{ slot.value }}</span>
        <span v-if="slot.rawSuffix" class="bowl-suffix">{{ slot.rawSuffix }}</span>
        <span v-if="slot.covered" class="disc" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.presence-track {
  display: flex;
  flex-direction: column;
  gap: var(--sp-1);
}

.track-hdr {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  font-size: var(--fs-xs);
}

.track-label {
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-size: 0.68rem;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
}

.placed {
  font-family: var(--font-mono);
  font-size: var(--fs-xs);
  color: var(--text-secondary);
}

.bowls {
  display: flex;
  gap: var(--sp-1);
  padding: var(--sp-1);
  background: var(--bg-canvas);
  border-radius: var(--r-md);
  border: 1px solid var(--border-subtle);
  overflow-x: auto;
}

.bowl {
  position: relative;
  width: 2.5rem;
  height: 2.5rem;
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-full);
  padding: 0;
  font-family: var(--font-mono);
  font-size: var(--fs-md);
  font-weight: var(--fw-bold);
  color: var(--text-primary);
  transition: all var(--motion-fast);
  cursor: default;
}

.bowl.uncovered {
  background: var(--bg-muted);
  color: var(--pool-fear);
  border-color: var(--accent-border);
  box-shadow: inset 0 2px 6px rgba(0, 0, 0, 0.3);
}

.bowl.covered .bowl-value {
  opacity: 0.15;
}

.bowl.clickable {
  cursor: pointer;
}
.bowl.clickable:hover {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent-soft);
  transform: translateY(-1px);
}

.bowl:disabled {
  cursor: default;
  opacity: 0.6;
}
.bowl:disabled.covered .bowl-value {
  opacity: 0.12;
}

.bowl-value {
  position: relative;
  z-index: 1;
  transition: opacity var(--motion-fast);
}

.bowl-suffix {
  position: absolute;
  bottom: 2px;
  right: 2px;
  font-size: 0.55rem;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  z-index: 2;
  font-weight: var(--fw-medium);
}

/* The disc: a circular token that sits on top of the covered bowl */
.disc {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 1.65rem;
  height: 1.65rem;
  border-radius: var(--r-full);
  background: radial-gradient(circle at 35% 30%, #4a4f5a 0%, #2a2e36 60%, #1a1d23 100%);
  box-shadow:
    0 2px 4px rgba(0, 0, 0, 0.5),
    inset 0 1px 2px rgba(255, 255, 255, 0.08),
    inset 0 -2px 3px rgba(0, 0, 0, 0.4);
  z-index: 2;
  pointer-events: none;
}

.bowl.clickable.covered:hover .disc {
  background: radial-gradient(circle at 35% 30%, #d97757 0%, #9c5542 60%, #6a3a2d 100%);
}
</style>
