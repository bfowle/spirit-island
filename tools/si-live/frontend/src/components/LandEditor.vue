<script setup lang="ts">
import { computed } from 'vue'
import type { Land, Spirit } from '../types'
import { UNIT_KEYS, type UnitKey } from '../types'
import Icon from './Icon.vue'
import PresenceDisc from './PresenceDisc.vue'

const props = withDefaults(
  defineProps<{
    modelValue: Land
    landId?: string
    boardId?: string
    spirits?: Record<string, Spirit>
  }>(),
  {},
)

const emit = defineEmits<{ 'bump-presence': [slug: string, delta: number] }>()

function bump(key: UnitKey, delta: number) {
  const current = (props.modelValue[key] as number | undefined) ?? 0
  const next = Math.max(0, current + delta)
  props.modelValue[key] = next as never
}

const UNIT_META: Record<UnitKey, { label: string; icon?: string }> = {
  explorers: { label: 'Explorer', icon: 'unit-explorer' },
  towns: { label: 'Town', icon: 'unit-town' },
  cities: { label: 'City', icon: 'unit-city' },
  dahan: { label: 'Dahan', icon: 'unit-dahan' },
  blight: { label: 'Blight', icon: 'resource-blight' },
}

const tokens = computed(() => props.modelValue.tokens ?? [])

interface SpiritPresenceRow {
  slug: string
  count: number
  color?: string
  style: 'glass' | 'wood' | 'solid'
}

const spiritPresenceRows = computed<SpiritPresenceRow[]>(() => {
  if (!props.spirits || !props.boardId || !props.landId) return []
  const key = `${props.boardId}.${props.landId}`
  return Object.entries(props.spirits).map(([slug, spirit]) => ({
    slug,
    count: spirit.presence_on_board?.[key] ?? 0,
    color: spirit.disc_color,
    style: spirit.disc_style ?? 'glass',
  }))
})
</script>

<template>
  <div class="editor">
    <div v-if="tokens.length" class="tokens-row">
      <span class="tokens-label">Setup tokens</span>
      <span v-for="t in tokens" :key="t" class="token-chip" :title="t">{{ t }}</span>
    </div>

    <div class="units">
      <div v-for="k in UNIT_KEYS" :key="k" class="unit-row" :title="UNIT_META[k].label">
        <Icon
          v-if="UNIT_META[k].icon"
          :name="UNIT_META[k].icon!"
          :label="UNIT_META[k].label"
          :size="18"
          class="unit-icon"
        />
        <span class="unit-name">{{ UNIT_META[k].label }}</span>
        <div class="stepper">
          <button class="step" @click="bump(k, -1)" aria-label="decrement">−</button>
          <span class="val">{{ (modelValue[k] as number) ?? 0 }}</span>
          <button class="step" @click="bump(k, 1)" aria-label="increment">+</button>
        </div>
      </div>
    </div>

    <div v-if="spiritPresenceRows.length" class="presence-rows">
      <div class="presence-heading">Presence</div>
      <div v-for="row in spiritPresenceRows" :key="row.slug" class="presence-row-item">
        <PresenceDisc :slug="row.slug" :color="row.color" :style="row.style" :size="14" />
        <span class="presence-spirit">{{ row.slug }}</span>
        <div class="stepper">
          <button class="step" @click="emit('bump-presence', row.slug, -1)" aria-label="decrement">−</button>
          <span class="val">{{ row.count }}</span>
          <button class="step" @click="emit('bump-presence', row.slug, 1)" aria-label="increment">+</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.editor {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
  margin-top: var(--sp-2);
}

.tokens-row {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sp-1);
  align-items: center;
  padding-bottom: var(--sp-2);
  border-bottom: 1px dashed var(--border-subtle);
}

.tokens-label {
  font-size: 0.65rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
  margin-right: var(--sp-1);
}

.token-chip {
  display: inline-flex;
  padding: 1px var(--sp-2);
  font-size: 0.68rem;
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-full);
  color: var(--text-secondary);
  text-transform: capitalize;
  font-weight: var(--fw-medium);
}

.units {
  display: flex;
  flex-direction: column;
  gap: var(--sp-1);
}

.unit-row {
  display: grid;
  grid-template-columns: 1.5rem 1fr auto;
  align-items: center;
  gap: var(--sp-2);
  padding: 2px var(--sp-1);
  border-radius: var(--r-sm);
  transition: background var(--motion-fast);
}

.unit-row:hover { background: var(--bg-muted); }

.unit-icon {
  justify-self: center;
}

.unit-name {
  font-size: var(--fs-xs);
  color: var(--text-secondary);
}

.stepper {
  display: inline-flex;
  align-items: center;
  gap: 2px;
}

.step {
  width: 1.35rem;
  height: 1.35rem;
  padding: 0;
  line-height: 1;
  font-size: var(--fs-sm);
  border-radius: var(--r-sm);
}

.val {
  min-width: 1.2rem;
  text-align: center;
  font-family: var(--font-mono);
  font-weight: var(--fw-semibold);
  font-size: var(--fs-sm);
  color: var(--text-primary);
}

.presence-rows {
  display: flex;
  flex-direction: column;
  gap: 2px;
  margin-top: var(--sp-2);
  padding-top: var(--sp-2);
  border-top: 1px dashed var(--border-subtle);
}
.presence-heading {
  font-size: 0.65rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
}
.presence-row-item {
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: var(--sp-2);
  align-items: center;
  padding: 2px var(--sp-1);
  border-radius: var(--r-sm);
  font-size: var(--fs-xs);
}
.presence-row-item:hover { background: var(--bg-hover); }
.presence-spirit {
  font-family: var(--font-mono);
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
