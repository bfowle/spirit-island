<script setup lang="ts">
import { computed } from 'vue'
import type { Phase } from '../types'

/**
 * Top-banner phase visualization. The Spirit Island turn progresses through
 * six phases; this component renders them as a flow of clickable steps with
 * the current one highlighted. Clicking jumps the `state.phase` value, which
 * (a) de-dupe-writes a phase_snapshot via the watcher in TurnController and
 * (b) shifts the app's per-phase focus emphasis.
 *
 * The component is purely navigational; it doesn't advance the round or touch
 * the invader deck (that's Advance Turn's job).
 */

interface PhaseMeta {
  key: Phase
  label: string
  short: string
  focus: string
  accent: string
}

const PHASES: PhaseMeta[] = [
  { key: 'setup',      label: 'Setup',       short: '1', focus: 'boards, invader deck, fear deck', accent: 'var(--accent-blue)' },
  { key: 'growth',     label: 'Growth',      short: '2', focus: 'spirit panels — pick growth options', accent: 'var(--accent-green)' },
  { key: 'fast',       label: 'Fast',        short: '3', focus: 'spirit panels — fast cards', accent: 'var(--accent-red)' },
  { key: 'event',      label: 'Event',       short: '4', focus: 'event deck — resolve previewed event', accent: 'var(--accent-purple)' },
  { key: 'fear',       label: 'Fear',        short: '5', focus: 'fear deck — resolve earned fear cards', accent: 'var(--pool-fear, var(--accent-amber))' },
  { key: 'invader',    label: 'Invader',     short: '6', focus: 'invader deck — Ravage, Build, Explore', accent: 'var(--accent-red)' },
  { key: 'slow',       label: 'Slow',        short: '7', focus: 'spirit panels — slow cards', accent: 'var(--accent-blue)' },
  { key: 'timepasses', label: 'Time Passes', short: '8', focus: 'retrospective, advance turn', accent: 'var(--accent-amber)' },
]

const props = defineProps<{ modelValue: Phase; round: number }>()
const emit = defineEmits<{ 'update:modelValue': [value: Phase] }>()

const activeIdx = computed(() => PHASES.findIndex(p => p.key === props.modelValue))
const activeMeta = computed(() => PHASES[activeIdx.value] ?? PHASES[0])

function jumpTo(phase: Phase) {
  if (phase === props.modelValue) return
  emit('update:modelValue', phase)
}

function isCompleted(i: number): boolean {
  return i < activeIdx.value
}
function isActive(i: number): boolean {
  return i === activeIdx.value
}
</script>

<template>
  <nav class="phase-stepper" :aria-label="`Round ${round} · phase ${activeMeta.label}`">
    <div class="steps">
      <template v-for="(p, i) in PHASES" :key="p.key">
        <button
          type="button"
          class="step"
          :class="{ active: isActive(i), done: isCompleted(i) }"
          :style="{ '--step-accent': p.accent }"
          @click="jumpTo(p.key)"
          :title="`${p.label} — ${p.focus}`"
        >
          <span class="step-num">{{ p.short }}</span>
          <span class="step-label">{{ p.label }}</span>
        </button>
        <span v-if="i < PHASES.length - 1" class="step-connector" :class="{ done: isCompleted(i) }"></span>
      </template>
    </div>
    <div class="focus-hint">
      <span class="focus-label">Focus:</span>
      <span class="focus-text">{{ activeMeta.focus }}</span>
      <span class="round-tag">Round {{ round }}</span>
    </div>
  </nav>
</template>

<style scoped>
.phase-stepper {
  position: sticky;
  top: 0;
  z-index: 40;
  background: linear-gradient(180deg, var(--bg-surface) 0%, rgba(26, 26, 46, 0.92) 100%);
  backdrop-filter: blur(6px);
  border-bottom: 1px solid var(--border-subtle);
  padding: var(--sp-3) var(--sp-4);
  display: flex; flex-direction: column; gap: var(--sp-2);
  margin: calc(-1 * var(--sp-5)) calc(-1 * var(--sp-4)) 0;
}

.steps {
  display: flex;
  flex-wrap: nowrap;
  align-items: center;
  gap: 0;
  overflow-x: auto;
}
@media (max-width: 820px) {
  .steps {
    flex-wrap: wrap;
    gap: 4px;
  }
  .step-connector { display: none; }
}

.step {
  display: inline-flex; align-items: center; gap: var(--sp-2);
  padding: var(--sp-2) var(--sp-3);
  border-radius: var(--r-md);
  border: 1px solid var(--border-subtle);
  background: var(--bg-inset);
  cursor: pointer;
  color: var(--text-secondary);
  font-size: var(--fs-sm);
  font-weight: var(--fw-medium);
  transition: all var(--motion-fast);
  --step-accent: var(--accent-blue);
}
.step:hover {
  background: var(--bg-muted);
  color: var(--text-primary);
}
.step.active {
  background: color-mix(in srgb, var(--step-accent) 18%, var(--bg-inset));
  color: var(--text-white);
  border-color: var(--step-accent);
  box-shadow: 0 0 0 2px color-mix(in srgb, var(--step-accent) 30%, transparent);
}
.step.done {
  opacity: 0.55;
  color: var(--text-muted);
}
.step.done .step-num {
  background: color-mix(in srgb, var(--step-accent) 35%, transparent);
  color: var(--text-white);
}

.step-num {
  display: inline-flex; align-items: center; justify-content: center;
  width: 1.35rem; height: 1.35rem;
  border-radius: 50%;
  background: var(--bg-canvas);
  font-family: var(--font-mono);
  font-size: 0.72rem;
  font-weight: var(--fw-bold);
}
.step.active .step-num {
  background: var(--step-accent);
  color: var(--bg-canvas);
}
.step-label {
  text-transform: capitalize;
  white-space: nowrap;
}

.step-connector {
  flex: 0 0 12px;
  height: 2px;
  background: var(--border-subtle);
  transition: background var(--motion-fast);
}
.step-connector.done {
  background: color-mix(in srgb, var(--accent-green) 40%, transparent);
}

.focus-hint {
  display: inline-flex; align-items: center; gap: var(--sp-2);
  font-size: var(--fs-xs);
  color: var(--text-muted);
  flex-wrap: wrap;
}
.focus-label {
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-weight: var(--fw-medium);
  color: var(--text-secondary);
}
.focus-text { font-style: italic; }
.round-tag {
  margin-left: auto;
  font-family: var(--font-mono);
  padding: 1px var(--sp-2);
  background: var(--bg-muted);
  border-radius: var(--r-sm);
  color: var(--accent-amber);
  font-weight: var(--fw-semibold);
}
</style>
