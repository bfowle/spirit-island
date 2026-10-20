<script setup lang="ts">
import { computed } from 'vue'
import type { PublicInvaderCard } from './types'
import { invaderLabel, roman, terrainColor } from './format'

/**
 * A small Invader card. Face: one band per terrain it matches, stage
 * numeral at the foot. Back (`back`): the stage printed on the back only.
 */
const props = withDefaults(
  defineProps<{ card?: PublicInvaderCard | null; stage?: number | null; back?: boolean; size?: 'xs' | 'sm' | 'md'; dim?: boolean; label?: boolean }>(),
  { card: null, stage: null, back: false, size: 'sm', dim: false, label: false },
)

const stageNum = computed(() => props.card?.stage ?? props.stage ?? null)
const bands = computed(() => (props.card?.terrains ?? []).map((t) => terrainColor(t)))
const title = computed(() =>
  props.back || !props.card
    ? `Face-down Stage ${roman(stageNum.value)} card`
    : `${invaderLabel(props.card)} (Stage ${roman(stageNum.value)}${props.card.escalation ? ', Escalation' : ''})`,
)
</script>

<template>
  <span class="inv" :class="[size, { back: back || !card, dim }]" :title="title">
    <span class="tile" aria-hidden="true">
      <template v-if="!back && card">
        <i v-for="(c, i) in bands" :key="i" class="band" :style="{ background: c }"></i>
      </template>
      <b class="stage">{{ roman(stageNum) }}</b>
      <svg v-if="!back && card?.escalation" class="esc" viewBox="0 0 10 10" aria-hidden="true"><path d="M2 9V1.5h5.5L6.3 3.6 7.5 5.7H3" /></svg>
    </span>
    <span v-if="label && card && !back" class="lbl">{{ invaderLabel(card) }}<small v-if="card.escalation" class="esc-l">Escalation</small></span>
    <span class="sr">{{ title }}</span>
  </span>
</template>

<style scoped>
.inv { display: inline-flex; align-items: center; gap: 6px; vertical-align: middle; }
.tile {
  position: relative;
  display: inline-flex;
  flex-direction: column;
  overflow: hidden;
  border-radius: 4px;
  border: 1px solid rgba(255, 255, 255, 0.16);
  background: var(--bg-raised);
  flex: 0 0 auto;
}
.xs .tile { width: 18px; height: 25px; border-radius: 3px; }
.sm .tile { width: 30px; height: 42px; }
.md .tile { width: 44px; height: 62px; border-radius: 6px; }
.band { flex: 1 1 0; display: block; }
.stage {
  position: absolute;
  left: 0; right: 0; bottom: 2px;
  text-align: center;
  font-family: var(--font-brand);
  font-weight: 600;
  color: #fff;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.7);
  line-height: 1;
}
.xs .stage { font-size: 8px; bottom: 1px; }
.sm .stage { font-size: 11px; }
.md .stage { font-size: 14px; bottom: 4px; }
.back .tile {
  background:
    repeating-linear-gradient(135deg, rgba(255, 255, 255, 0.04) 0 4px, transparent 4px 8px),
    linear-gradient(160deg, hsl(222 35% 22%), hsl(222 40% 14%));
  border-color: var(--border-strong);
}
.back .stage { top: 50%; bottom: auto; transform: translateY(-50%); color: var(--text-secondary); text-shadow: none; }
.back.md .stage { font-size: 18px; }
.back.sm .stage { font-size: 13px; }
.dim { opacity: 0.38; }
.esc { position: absolute; top: 1px; right: 1px; width: 40%; max-width: 12px; height: auto; fill: #fff; stroke: #0b1220; stroke-width: 1; paint-order: stroke; }
.xs .esc { max-width: 7px; }
.lbl { font-size: 12px; color: var(--text-primary); white-space: nowrap; display: inline-flex; flex-direction: column; line-height: 1.25; }
.esc-l { font-size: 10.5px; color: var(--text-muted); }
.sr { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0 0 0 0); white-space: nowrap; }
</style>
