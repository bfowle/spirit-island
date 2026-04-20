<script setup lang="ts">
import { computed } from 'vue'
import type { DiscStyle, SpiritPalette } from '../spiritColors'

/**
 * Presence track: bowls with discs on top of covered slots.
 *
 * `fullTrack` is immutable (from Wiki JSON). `coveredTokens` is what's still
 * under a disc — convention is discs uncover left-to-right, so the FIRST
 * slots of fullTrack that aren't in coveredTokens are the revealed ones.
 *
 * The leftmost bowl of any track starts uncovered at game setup (that's
 * the spirit's starting income value — visible without removing a disc).
 * Placing presence removes the next disc leftward and reveals its bowl.
 */

const props = defineProps<{
  fullTrack: string[]
  coveredTokens: string[]
  label: string
  palette: SpiritPalette
  discStyle: DiscStyle
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
  const m = token.match(/^([a-z]+)(\d+)(.*)$/i)
  if (!m) return { value: token, suffix: '' }
  const [, , digits, suffix] = m
  return { value: digits, suffix }
}

const slots = computed((): Slot[] => {
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
  const uncoveredCount = props.fullTrack.length - props.coveredTokens.length
  const nextToUncoverIndex = uncoveredCount
  const lastUncoveredIndex = uncoveredCount - 1

  let newCovered: string[]
  if (slot.index === nextToUncoverIndex) {
    newCovered = props.coveredTokens.slice(1)
  } else if (slot.index === lastUncoveredIndex) {
    newCovered = [props.fullTrack[lastUncoveredIndex], ...props.coveredTokens]
  } else {
    return
  }
  emit('update:coveredTokens', newCovered)
}

function canToggle(slot: Slot): boolean {
  const uncoveredCount = props.fullTrack.length - props.coveredTokens.length
  return slot.index === uncoveredCount || slot.index === uncoveredCount - 1
}

const placedCount = computed(() => props.fullTrack.length - props.coveredTokens.length - 1)

const discCssVars = computed(() => ({
  '--disc-primary': props.palette.primary,
  '--disc-highlight': props.palette.highlight,
  '--disc-shadow': props.palette.shadow,
}))
</script>

<template>
  <div class="presence-track" :style="discCssVars">
    <div class="track-hdr">
      <span class="track-label">{{ label }} Track</span>
      <span class="placed">{{ Math.max(0, placedCount) }} placed / {{ fullTrack.length }} total</span>
    </div>
    <div class="bowls">
      <button
        v-for="slot in slots"
        :key="`${slot.token}-${slot.index}`"
        class="bowl"
        :class="{
          covered: slot.covered,
          uncovered: !slot.covered,
          clickable: canToggle(slot),
        }"
        :data-style="slot.covered ? discStyle : undefined"
        :disabled="!canToggle(slot)"
        :title="slot.covered ? `Place presence (uncovers ${slot.value})` : `Return presence to track`"
        @click="toggle(slot)"
      >
        <span class="bowl-value">{{ slot.value }}</span>
        <span v-if="slot.rawSuffix" class="bowl-suffix">{{ slot.rawSuffix }}</span>
        <span v-if="slot.covered" class="disc" :class="`style-${discStyle}`" />
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
  font-size: var(--text-xs);
}

.track-label {
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-size: var(--text-xs);
  color: var(--text-muted);
  font-weight: var(--weight-medium);
}

.placed {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-secondary);
}

.bowls {
  display: flex;
  gap: var(--sp-2);
  padding: var(--sp-2);
  background: linear-gradient(180deg, var(--bg-base), var(--bg-muted));
  border-radius: var(--radius-md);
  border: 1px solid var(--border-subtle);
  overflow-x: auto;
}

.bowl {
  position: relative;
  width: 2.75rem;
  height: 2.75rem;
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: radial-gradient(circle at 50% 45%, #0a0a0c 0%, #141418 60%, #1a1a1e 100%);
  border: 1px solid var(--border-default);
  border-radius: 50%;
  padding: 0;
  font-family: var(--font-mono);
  font-size: 1.05rem;
  font-weight: var(--weight-bold);
  color: var(--text-primary);
  cursor: default;
  transition: transform var(--duration-fast), box-shadow var(--duration-fast);
  /* inset shadow to make the bowl look recessed */
  box-shadow:
    inset 0 3px 5px rgba(0, 0, 0, 0.65),
    inset 0 -1px 1px rgba(255, 255, 255, 0.05);
}

.bowl.uncovered {
  background: radial-gradient(circle at 50% 45%, #202026 0%, #17171a 100%);
  color: var(--text-primary);
  border-color: var(--border-strong);
  box-shadow:
    inset 0 2px 4px rgba(0, 0, 0, 0.8),
    inset 0 -2px 2px rgba(255, 255, 255, 0.04);
}

.bowl.covered .bowl-value {
  opacity: 0;  /* fully hidden beneath the disc */
}

.bowl.clickable {
  cursor: pointer;
}
.bowl.clickable:hover {
  transform: translateY(-2px);
  box-shadow:
    inset 0 2px 4px rgba(0, 0, 0, 0.65),
    0 4px 10px rgba(0, 0, 0, 0.4),
    0 0 0 2px var(--disc-highlight);
}

.bowl:disabled {
  cursor: default;
}

.bowl-value {
  position: relative;
  z-index: 1;
  color: var(--disc-primary);
  text-shadow: 0 1px 0 rgba(0, 0, 0, 0.4);
  transition: opacity var(--duration-fast);
}

.bowl-suffix {
  position: absolute;
  bottom: 2px;
  right: 3px;
  font-size: 0.55rem;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  z-index: 2;
  font-weight: var(--weight-medium);
}

/* ============ DISC STYLES ============ */

.disc {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 1.9rem;
  height: 1.9rem;
  border-radius: 50%;
  z-index: 2;
  pointer-events: none;
  transition: all var(--duration-base);
}

/* --- Glass disc (etsy custom): translucent, with refractive highlight --- */
.disc.style-glass {
  background:
    radial-gradient(circle at 35% 25%, rgba(255, 255, 255, 0.5) 0%, transparent 40%),
    radial-gradient(circle at 50% 50%, color-mix(in srgb, var(--disc-primary) 75%, transparent) 0%, color-mix(in srgb, var(--disc-shadow) 80%, transparent) 100%);
  border: 1px solid color-mix(in srgb, var(--disc-primary) 60%, #000);
  box-shadow:
    0 2px 6px rgba(0, 0, 0, 0.55),
    inset 0 2px 4px rgba(255, 255, 255, 0.4),
    inset 0 -3px 5px rgba(0, 0, 0, 0.35),
    inset 0 0 14px color-mix(in srgb, var(--disc-primary) 40%, transparent);
  backdrop-filter: blur(1px);
}
.disc.style-glass::before {
  /* The refractive "bright spot" on top */
  content: '';
  position: absolute;
  top: 12%;
  left: 22%;
  width: 38%;
  height: 28%;
  border-radius: 50%;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.85), rgba(255, 255, 255, 0));
  filter: blur(0.5px);
}

/* --- Wooden disc: warm grain, carved look --- */
.disc.style-wood {
  background:
    repeating-linear-gradient(
      90deg,
      color-mix(in srgb, var(--disc-primary) 70%, #6b4b2a) 0px,
      color-mix(in srgb, var(--disc-primary) 70%, #6b4b2a) 1px,
      color-mix(in srgb, var(--disc-primary) 60%, #3a2818) 2px,
      color-mix(in srgb, var(--disc-primary) 70%, #6b4b2a) 4px
    ),
    radial-gradient(circle at 35% 30%, color-mix(in srgb, var(--disc-highlight) 65%, #a07050) 0%, color-mix(in srgb, var(--disc-shadow) 80%, #3a2010) 80%);
  border: 1px solid color-mix(in srgb, var(--disc-shadow) 70%, #2a1808);
  box-shadow:
    0 2px 4px rgba(0, 0, 0, 0.6),
    inset 0 1px 2px rgba(255, 255, 255, 0.15),
    inset 0 -2px 3px rgba(0, 0, 0, 0.5);
}
.disc.style-wood::before {
  /* faint central ring to simulate wood grain whorl */
  content: '';
  position: absolute;
  inset: 18%;
  border-radius: 50%;
  border: 1px solid rgba(0, 0, 0, 0.18);
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.08);
}

/* --- Solid disc: flat color, clean like a poker chip --- */
.disc.style-solid {
  background: radial-gradient(circle at 40% 35%, var(--disc-highlight), var(--disc-primary) 60%, var(--disc-shadow) 100%);
  border: 1px solid var(--disc-shadow);
  box-shadow:
    0 2px 4px rgba(0, 0, 0, 0.55),
    inset 0 1px 2px rgba(255, 255, 255, 0.2),
    inset 0 -2px 3px rgba(0, 0, 0, 0.4);
}

.bowl.clickable:hover .disc {
  transform: translate(-50%, -50%) scale(1.08);
  filter: brightness(1.15);
}
</style>
