<script setup lang="ts">
import { computed, ref } from 'vue'
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

const emit = defineEmits<{
  'bump-presence': [slug: string, delta: number]
  /** Emitted when a board-level pool needs to sync (e.g., blight added to a
   *  land should decrement the Blight Card's pool). Actual delta applied. */
  'bump-pool': [pool: 'blight', delta: number]
}>()

function bump(key: UnitKey, delta: number) {
  const current = (props.modelValue[key] as number | undefined) ?? 0
  const next = Math.max(0, current + delta)
  const actualDelta = next - current
  props.modelValue[key] = next as never
  if (key === 'blight' && actualDelta !== 0) {
    emit('bump-pool', 'blight', actualDelta)
  }
}

const UNIT_META: Record<UnitKey, { label: string; icon?: string }> = {
  explorers: { label: 'Explorer', icon: 'unit-explorer' },
  towns: { label: 'Town', icon: 'unit-town' },
  cities: { label: 'City', icon: 'unit-city' },
  dahan: { label: 'Dahan', icon: 'unit-dahan' },
  blight: { label: 'Blight', icon: 'resource-blight' },
}

const tokens = computed(() => props.modelValue.tokens ?? [])

// Canonical Spirit Island token types — setup-placed + spirit-placed + temporary.
// Dynamic counts are represented as repeated strings in the `tokens` array, so a
// land with 2 Beasts is `["beast", "beast"]`. This keeps the schema backward-
// compatible with the board-geometry JSON files that ship these as a simple list.
const CANONICAL_TOKENS = [
  // Setup tokens
  'beast', 'disease', 'wilds', 'badlands', 'strife',
  // Spirit-placed (Nature Incarnate / Jagged Earth additions)
  'vitality',
  // Temporary effects (not technically tokens but rendered here for trackability)
  'defend', 'isolate',
] as const

// Group tokens by name, preserving insertion order
interface TokenGroup { name: string; count: number }
const tokenGroups = computed<TokenGroup[]>(() => {
  const counts = new Map<string, number>()
  for (const t of tokens.value) counts.set(t, (counts.get(t) ?? 0) + 1)
  return Array.from(counts.entries()).map(([name, count]) => ({ name, count }))
})

const customTokenDraft = ref<string>('')

function addToken(name: string) {
  const n = name.trim().toLowerCase()
  if (!n) return
  if (!props.modelValue.tokens) props.modelValue.tokens = []
  props.modelValue.tokens.push(n)
}

function removeToken(name: string) {
  const arr = props.modelValue.tokens
  if (!arr) return
  const idx = arr.indexOf(name)
  if (idx < 0) return
  arr.splice(idx, 1)
}

function onSelectToken(ev: Event) {
  const el = ev.target as HTMLSelectElement
  const v = el.value
  if (v) {
    addToken(v)
    el.value = ''
  }
}

function submitCustomToken() {
  if (!customTokenDraft.value.trim()) return
  addToken(customTokenDraft.value)
  customTokenDraft.value = ''
}

function tokenClass(name: string): string {
  return `token-chip token-${name.replace(/[^a-z]/gi, '-').toLowerCase()}`
}

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
    <div class="tokens-section">
      <div class="tokens-hdr">
        <span class="tokens-label">Tokens</span>
        <div class="add-token">
          <select
            :value="''"
            @change="onSelectToken"
            class="token-select"
            title="Add a canonical token"
          >
            <option value="">+ add token…</option>
            <option v-for="t in CANONICAL_TOKENS" :key="t" :value="t">{{ t }}</option>
          </select>
          <input
            type="text"
            v-model="customTokenDraft"
            placeholder="custom…"
            class="custom-token-input"
            @keyup.enter="submitCustomToken"
          />
        </div>
      </div>
      <div v-if="tokenGroups.length" class="token-grid">
        <div v-for="g in tokenGroups" :key="g.name" class="token-row" :class="tokenClass(g.name)">
          <span class="token-name">{{ g.name }}</span>
          <div class="token-stepper">
            <button class="step" @click="removeToken(g.name)" aria-label="decrement">−</button>
            <span class="token-count">{{ g.count }}</span>
            <button class="step" @click="addToken(g.name)" aria-label="increment">+</button>
          </div>
        </div>
      </div>
      <span v-else class="no-tokens">no tokens</span>
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

.tokens-section {
  display: flex;
  flex-direction: column;
  gap: var(--sp-1);
  padding-bottom: var(--sp-2);
  border-bottom: 1px dashed var(--border-subtle);
}
.tokens-hdr {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-2);
  flex-wrap: wrap;
}

.tokens-label {
  font-size: 0.65rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
}

.add-token {
  display: inline-flex;
  gap: 4px;
  align-items: center;
}
.token-select {
  font-size: var(--fs-xs);
  padding: 2px var(--sp-1);
  text-transform: capitalize;
}
.custom-token-input {
  font-size: var(--fs-xs);
  padding: 2px var(--sp-1);
  width: 5.5rem;
}

.no-tokens {
  color: var(--text-muted);
  font-size: var(--fs-xs);
  font-style: italic;
}

.token-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.token-row {
  display: inline-flex;
  align-items: center;
  gap: var(--sp-1);
  padding: 2px 6px;
  font-size: 0.72rem;
  border-radius: var(--r-full);
  border: 1px solid var(--border-subtle);
  background: var(--bg-muted);
  color: var(--text-secondary);
  font-weight: var(--fw-medium);
  text-transform: capitalize;
}
.token-row.token-beast    { border-color: rgba(184, 134, 11, 0.4); color: #d4a444; background: rgba(184, 134, 11, 0.1); }
.token-row.token-disease  { border-color: rgba(201, 100, 100, 0.4); color: #e57373; background: rgba(201, 100, 100, 0.1); }
.token-row.token-wilds    { border-color: rgba(82, 183, 136, 0.4); color: #52b788; background: rgba(82, 183, 136, 0.1); }
.token-row.token-badlands { border-color: rgba(160, 82, 45, 0.5); color: #b97a40; background: rgba(160, 82, 45, 0.12); }
.token-row.token-strife   { border-color: rgba(199, 125, 255, 0.4); color: #c77dff; background: rgba(199, 125, 255, 0.1); }
.token-row.token-vitality { border-color: rgba(233, 196, 106, 0.4); color: #e9c46a; background: rgba(233, 196, 106, 0.1); }
.token-row.token-defend   { border-color: rgba(123, 184, 245, 0.4); color: #7bb8f5; background: rgba(123, 184, 245, 0.1); }
.token-row.token-isolate  { border-color: rgba(0, 184, 212, 0.4); color: #00b8d4; background: rgba(0, 184, 212, 0.1); }

.token-name { font-family: var(--font-mono); font-size: 0.68rem; }
.token-stepper {
  display: inline-flex;
  align-items: center;
  gap: 2px;
}
.token-count {
  min-width: 0.9rem;
  text-align: center;
  font-family: var(--font-mono);
  font-weight: var(--fw-bold);
  font-size: 0.72rem;
  color: var(--text-white);
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
