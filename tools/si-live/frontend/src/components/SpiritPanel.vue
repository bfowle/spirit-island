<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import type { Spirit } from '../types'
import Icon from './Icon.vue'
import PresenceTrack from './PresenceTrack.vue'
import {
  STOCK_COLORS,
  SPIRIT_DEFAULT_COLOR,
  getPaletteForSpirit,
  type DiscStyle,
} from '../spiritColors'

const props = defineProps<{ modelValue: Spirit; slug?: string }>()

interface CardDetail {
  name?: string
  cost?: string | number
  speed?: string
  range?: string
  target?: string
  elements?: string[]
  text?: string
  card_type?: string
}

// Spirit wiki data (immutable, from data/references/wiki/<slug>.json).
const fullEnergyTrack = ref<string[]>([])
const fullCardplayTrack = ref<string[]>([])
const cardDetailsByName = ref<Record<string, CardDetail>>({})
const wikiError = ref<string | null>(null)
const presenceExpanded = ref<boolean>(false)

async function loadSpiritMeta(slug: string | undefined) {
  if (!slug) return
  try {
    const res = await fetch(`/api/spirit/${encodeURIComponent(slug)}`)
    if (!res.ok) throw new Error(`${res.status}`)
    const data = await res.json()
    fullEnergyTrack.value = data.presence_energy_track || []
    fullCardplayTrack.value = data.presence_cardplay_track || []
    // Collect card lookups from both unique + suggested detail arrays
    const lookup: Record<string, CardDetail> = {}
    for (const c of data.unique_card_details || []) {
      if (c.name) lookup[c.name] = c
    }
    for (const c of data.suggested_card_details || []) {
      if (c.name) lookup[c.name] = c
    }
    cardDetailsByName.value = lookup
  } catch (e) {
    wikiError.value = `spirit metadata lookup failed: ${(e as Error).message}`
  }
}

onMounted(() => loadSpiritMeta(props.slug))
watch(() => props.slug, loadSpiritMeta)

function updateEnergyCovered(next: string[]) {
  props.modelValue.presence_on_track_energy = next
}
function updateCardplayCovered(next: string[]) {
  props.modelValue.presence_on_track_cardplay = next
}

// --- Elements + pile computations ------------------------------------------
const elements = computed(() => {
  const e = props.modelValue.elements_this_turn ?? {}
  return Object.entries(e).filter(([, n]) => (n as number) > 0)
})
const handCount = computed(() => props.modelValue.hand?.length ?? 0)
const discardCount = computed(() => props.modelValue.discard?.length ?? 0)
const playedCount = computed(() => props.modelValue.played_this_turn?.length ?? 0)
const forgottenCount = computed(() => props.modelValue.forgotten?.length ?? 0)

// Placed counts = full track length minus what's still covered
const placedEnergy = computed(
  () => Math.max(0, fullEnergyTrack.value.length - (props.modelValue.presence_on_track_energy?.length ?? 0)),
)
const placedCardplay = computed(
  () => Math.max(0, fullCardplayTrack.value.length - (props.modelValue.presence_on_track_cardplay?.length ?? 0)),
)

function parseTokenValue(token: string | undefined): string {
  if (!token) return '?'
  const m = token.match(/^[a-z]+(\d+)/i)
  return m ? m[1] : token
}

// Income next turn = value of the rightmost uncovered slot. Track convention:
// covered tokens are the RIGHT side of the array (everything except the
// left-most `placed` slots). So the most-recently-uncovered is at
// fullTrack[placed - 1] (when placed > 0); otherwise the first-uncovered
// leftmost (index 0).
const nextEnergyIncome = computed(() => {
  const full = fullEnergyTrack.value
  if (!full.length) return props.modelValue.energy ?? 0
  const placed = placedEnergy.value
  const idx = Math.max(0, placed - 1)
  return parseTokenValue(full[idx])
})
const nextCardplayIncome = computed(() => {
  const full = fullCardplayTrack.value
  if (!full.length) return props.modelValue.card_plays ?? 1
  const placed = placedCardplay.value
  const idx = Math.max(0, placed - 1)
  return parseTokenValue(full[idx])
})

function moveCard(card: string, from: keyof Spirit, to: keyof Spirit) {
  const src = (props.modelValue[from] as string[] | undefined) ?? []
  const dst = (props.modelValue[to] as string[] | undefined) ?? []
  const idx = src.indexOf(card)
  if (idx < 0) return
  src.splice(idx, 1)
  dst.push(card)
  ;(props.modelValue[from] as unknown) = src
  ;(props.modelValue[to] as unknown) = dst
}

const ELEMENT_ICONS: Record<string, string> = {
  sun: 'element-sun', moon: 'element-moon', fire: 'element-fire', air: 'element-air',
  water: 'element-water', earth: 'element-earth', plant: 'element-plant', animal: 'element-animal',
}

// --- Disc color + style ----------------------------------------------------
const discColor = computed({
  get: () =>
    (props.modelValue as unknown as { disc_color?: string }).disc_color ||
    SPIRIT_DEFAULT_COLOR[props.slug || ''] ||
    STOCK_COLORS.indigo,
  set: (v: string) => { (props.modelValue as unknown as { disc_color?: string }).disc_color = v },
})
const discStyle = computed({
  get: () => (props.modelValue as unknown as { disc_style?: DiscStyle }).disc_style || 'glass',
  set: (v: DiscStyle) => { (props.modelValue as unknown as { disc_style?: DiscStyle }).disc_style = v },
})
const palette = computed(() => getPaletteForSpirit(props.slug || '', discColor.value))
const styleOptions: { value: DiscStyle; label: string }[] = [
  { value: 'glass', label: 'Glass' },
  { value: 'wood', label: 'Wood' },
  { value: 'solid', label: 'Solid' },
]
const stockColorEntries = Object.entries(STOCK_COLORS)
function resetToDefault() {
  discColor.value = SPIRIT_DEFAULT_COLOR[props.slug || ''] || STOCK_COLORS.indigo
}

function cardInfo(name: string): CardDetail | undefined {
  return cardDetailsByName.value[name]
}
</script>

<template>
  <div class="panel">
    <div class="top">
      <div class="resources">
        <label class="res">
          <span class="res-label">Energy</span>
          <input type="number" v-model.number="modelValue.energy" />
        </label>
        <label class="res">
          <span class="res-label">Card Plays</span>
          <input type="number" min="0" v-model.number="modelValue.card_plays" />
        </label>
      </div>

      <div class="elements">
        <span class="section-label">Elements this turn</span>
        <div class="element-chips">
          <span v-for="[el, n] in elements" :key="el" class="element-chip" :title="el">
            <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="14" decorative />
            <span class="el-name">{{ el }}</span>
            <span class="el-count">×{{ n }}</span>
          </span>
          <span v-if="!elements.length" class="muted">no elements tallied</span>
        </div>
      </div>
    </div>

    <!-- Presence tracks: numeric summary always visible; the bowl visualization
         is collapsible via the toggle button. -->
    <div class="presence-block">
      <div class="presence-hdr">
        <div class="presence-stats">
          <div class="presence-stat">
            <span class="stat-label">Energy track</span>
            <div class="stat-detail">
              <span class="stat-placed">
                <strong>{{ placedEnergy }}</strong> / {{ fullEnergyTrack.length || '?' }} placed
              </span>
              <span class="stat-income">
                Income next turn: <strong>{{ nextEnergyIncome }}E</strong>
              </span>
            </div>
          </div>
          <div class="presence-stat">
            <span class="stat-label">Card Play track</span>
            <div class="stat-detail">
              <span class="stat-placed">
                <strong>{{ placedCardplay }}</strong> / {{ fullCardplayTrack.length || '?' }} placed
              </span>
              <span class="stat-income">
                Card plays next turn: <strong>{{ nextCardplayIncome }}</strong>
              </span>
            </div>
          </div>
        </div>
        <button class="bowl-toggle" @click="presenceExpanded = !presenceExpanded">
          {{ presenceExpanded ? 'Hide bowls' : 'Show bowls' }}
        </button>
      </div>

      <div v-if="presenceExpanded" class="tracks">
        <div class="disc-controls">
          <span class="disc-label">Disc color</span>
          <div class="swatches">
            <button
              v-for="[name, hex] in stockColorEntries"
              :key="name"
              class="swatch"
              :class="{ active: discColor === hex }"
              :style="{ '--s': hex }"
              :title="name"
              @click="discColor = hex"
            />
            <label class="swatch-custom" title="custom color">
              <input type="color" v-model="discColor" />
            </label>
            <button class="reset-color" @click="resetToDefault" title="Reset to canonical color">↺</button>
          </div>
          <div class="style-picker">
            <button
              v-for="opt in styleOptions"
              :key="opt.value"
              class="style-btn"
              :class="{ active: discStyle === opt.value }"
              @click="discStyle = opt.value"
            >{{ opt.label }}</button>
          </div>
        </div>

        <PresenceTrack
          v-if="fullEnergyTrack.length"
          label="Energy"
          :full-track="fullEnergyTrack"
          :covered-tokens="modelValue.presence_on_track_energy ?? []"
          :palette="palette"
          :disc-style="discStyle"
          @update:covered-tokens="updateEnergyCovered"
        />
        <PresenceTrack
          v-if="fullCardplayTrack.length"
          label="Card Plays"
          :full-track="fullCardplayTrack"
          :covered-tokens="modelValue.presence_on_track_cardplay ?? []"
          :palette="palette"
          :disc-style="discStyle"
          @update:covered-tokens="updateCardplayCovered"
        />
        <div v-if="!fullEnergyTrack.length && !wikiError" class="track-loading">Loading presence tracks…</div>
        <div v-if="wikiError" class="track-error">{{ wikiError }}</div>
      </div>
    </div>

    <!-- Card piles with rich detail -->
    <div class="piles">
      <div class="pile">
        <div class="pile-hdr">
          <span class="pile-name">Hand</span>
          <span class="pile-count">{{ handCount }}</span>
        </div>
        <ul>
          <li v-for="c in modelValue.hand ?? []" :key="c" class="card-row">
            <div class="card-main">
              <div class="card-title">{{ c }}</div>
              <div v-if="cardInfo(c)" class="card-meta">
                <span v-if="cardInfo(c)?.cost !== undefined" class="meta-chip cost" :title="'Energy cost'">{{ cardInfo(c)?.cost }}E</span>
                <span v-if="cardInfo(c)?.speed" class="meta-chip speed">
                  <Icon v-if="cardInfo(c)?.speed?.toLowerCase() === 'fast'" name="speed-fast" :size="12" decorative />
                  <Icon v-else-if="cardInfo(c)?.speed?.toLowerCase() === 'slow'" name="speed-slow" :size="12" decorative />
                  {{ cardInfo(c)?.speed }}
                </span>
                <span v-if="cardInfo(c)?.range" class="meta-chip" :title="'Range'">R{{ cardInfo(c)?.range }}</span>
                <span
                  v-for="el in cardInfo(c)?.elements ?? []"
                  :key="el"
                  class="meta-chip elem"
                >
                  <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="12" decorative />
                </span>
              </div>
              <div v-if="cardInfo(c)?.text" class="card-text">{{ cardInfo(c)?.text }}</div>
            </div>
            <div class="card-actions">
              <button class="ghost" @click="moveCard(c, 'hand', 'played_this_turn')">Play</button>
              <button class="ghost" @click="moveCard(c, 'hand', 'discard')">Discard</button>
            </div>
          </li>
          <li v-if="!(modelValue.hand ?? []).length" class="empty">hand is empty</li>
        </ul>
      </div>

      <div class="pile">
        <div class="pile-hdr">
          <span class="pile-name">Played this turn</span>
          <span class="pile-count">{{ playedCount }}</span>
        </div>
        <ul>
          <li v-for="c in modelValue.played_this_turn ?? []" :key="c" class="card-row slim">
            <div class="card-main">
              <div class="card-title">{{ c }}</div>
              <div v-if="cardInfo(c)" class="card-meta">
                <span v-if="cardInfo(c)?.cost !== undefined" class="meta-chip cost">{{ cardInfo(c)?.cost }}E</span>
                <span v-if="cardInfo(c)?.speed" class="meta-chip speed">
                  <Icon v-if="cardInfo(c)?.speed?.toLowerCase() === 'fast'" name="speed-fast" :size="12" decorative />
                  <Icon v-else-if="cardInfo(c)?.speed?.toLowerCase() === 'slow'" name="speed-slow" :size="12" decorative />
                  {{ cardInfo(c)?.speed }}
                </span>
                <span
                  v-for="el in cardInfo(c)?.elements ?? []"
                  :key="el"
                  class="meta-chip elem"
                >
                  <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="12" decorative />
                </span>
              </div>
            </div>
            <div class="card-actions">
              <button class="ghost" @click="moveCard(c, 'played_this_turn', 'discard')">→ Discard</button>
              <button class="ghost" @click="moveCard(c, 'played_this_turn', 'hand')">↩ Hand</button>
            </div>
          </li>
          <li v-if="!(modelValue.played_this_turn ?? []).length" class="empty">no plays yet this turn</li>
        </ul>
      </div>

      <div class="pile">
        <div class="pile-hdr">
          <span class="pile-name">Discard</span>
          <span class="pile-count">{{ discardCount }}</span>
        </div>
        <ul>
          <li v-for="c in modelValue.discard ?? []" :key="c" class="card-row slim">
            <div class="card-main">
              <div class="card-title">{{ c }}</div>
            </div>
            <div class="card-actions">
              <button class="ghost" @click="moveCard(c, 'discard', 'hand')">↩ Hand</button>
            </div>
          </li>
          <li v-if="!(modelValue.discard ?? []).length" class="empty">discard empty</li>
        </ul>
      </div>

      <div v-if="forgottenCount > 0" class="pile forgotten">
        <div class="pile-hdr">
          <span class="pile-name">Forgotten</span>
          <span class="pile-count">{{ forgottenCount }}</span>
        </div>
        <ul>
          <li v-for="c in modelValue.forgotten ?? []" :key="c" class="empty">{{ c }}</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<style scoped>
.panel {
  display: flex; flex-direction: column; gap: var(--sp-3);
  background: var(--bg-surface); border: 1px solid var(--border-subtle);
  border-radius: var(--r-lg); padding: var(--sp-4);
  box-shadow: var(--shadow-sm);
}

.top { display: grid; grid-template-columns: auto 1fr; gap: var(--sp-4); align-items: start; }
.resources { display: inline-flex; gap: var(--sp-3); }
.res { display: flex; flex-direction: column; gap: var(--sp-1); }
.res-label, .section-label {
  font-size: 0.68rem; text-transform: uppercase; letter-spacing: 0.06em;
  color: var(--text-muted); font-weight: var(--fw-medium);
}

.elements { display: flex; flex-direction: column; gap: var(--sp-1); }
.element-chips { display: flex; flex-wrap: wrap; gap: var(--sp-1); align-items: center; min-height: 1.75rem; }
.element-chip {
  display: inline-flex; align-items: center; gap: 4px;
  padding: 2px var(--sp-2);
  background: var(--bg-muted); border: 1px solid var(--border-subtle);
  border-radius: var(--r-full); font-size: var(--fs-xs);
}
.el-name { text-transform: capitalize; color: var(--text-secondary); }
.el-count { font-family: var(--font-mono); font-weight: var(--fw-semibold); color: var(--text-primary); }

/* Presence block — numeric summary always visible; bowl visuals toggle */
.presence-block {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-3);
}
.presence-hdr {
  display: flex;
  justify-content: space-between;
  align-items: start;
  gap: var(--sp-3);
}
.presence-stats {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sp-3) var(--sp-5);
  flex: 1;
}
.presence-stat { display: flex; flex-direction: column; gap: 2px; }
.stat-label {
  font-size: var(--fs-xxs);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
}
.stat-detail {
  display: flex;
  gap: var(--sp-3);
  flex-wrap: wrap;
  font-size: var(--fs-sm);
  color: var(--text-secondary);
}
.stat-placed strong { color: var(--text-primary); font-family: var(--font-mono); }
.stat-income strong { color: var(--accent-amber); font-family: var(--font-mono); }
.bowl-toggle {
  font-size: var(--fs-xs);
  padding: var(--sp-1) var(--sp-3);
  white-space: nowrap;
}

.tracks {
  display: flex; flex-direction: column; gap: var(--sp-3);
  padding: var(--sp-3) 0 0;
  margin-top: var(--sp-3);
  border-top: 1px solid var(--border-subtle);
}

.track-loading, .track-error { font-size: var(--fs-xs); color: var(--text-muted); font-style: italic; }
.track-error { color: var(--status-danger); font-style: normal; }

.disc-controls {
  display: flex; gap: var(--sp-3); align-items: center; flex-wrap: wrap;
  padding-bottom: var(--sp-2); border-bottom: 1px dashed var(--border-subtle);
}
.disc-label {
  font-size: 0.68rem; text-transform: uppercase; letter-spacing: 0.06em;
  color: var(--text-muted); font-weight: var(--fw-medium);
}
.swatches { display: inline-flex; gap: 3px; align-items: center; flex-wrap: wrap; }
.swatch {
  width: 1.15rem; height: 1.15rem; padding: 0;
  border-radius: var(--r-full); border: 2px solid transparent;
  background: var(--s); cursor: pointer;
  transition: transform var(--motion-fast), border-color var(--motion-fast);
  flex-shrink: 0;
}
.swatch:hover { transform: scale(1.15); }
.swatch.active { border-color: var(--text-primary); box-shadow: 0 0 0 1px var(--bg-canvas); }
.swatch-custom {
  position: relative; width: 1.15rem; height: 1.15rem;
  border-radius: var(--r-full); border: 1px dashed var(--border-strong);
  cursor: pointer; overflow: hidden; display: inline-flex;
  align-items: center; justify-content: center; font-size: 0.65rem; color: var(--text-muted);
}
.swatch-custom::before { content: '+'; font-size: 0.85rem; line-height: 1; }
.swatch-custom input[type="color"] {
  position: absolute; inset: 0; opacity: 0; cursor: pointer; padding: 0; border: 0;
}
.reset-color {
  padding: 0 var(--sp-1); font-size: var(--fs-xs);
  background: transparent; border: none; color: var(--text-muted); cursor: pointer;
}
.reset-color:hover { color: var(--text-primary); background: transparent; border: none; }
.style-picker {
  display: inline-flex; gap: 2px; margin-left: auto;
  padding: 2px; background: var(--bg-canvas); border-radius: var(--r-sm);
}
.style-btn {
  font-size: var(--fs-xs); padding: 2px var(--sp-2);
  background: transparent; border: none; color: var(--text-secondary);
  border-radius: var(--r-sm); cursor: pointer;
}
.style-btn:hover { background: var(--bg-hover); }
.style-btn.active {
  background: var(--bg-surface); color: var(--text-primary);
  border: 1px solid var(--border-default);
}

/* Card piles */
.piles {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: var(--sp-3);
}
.pile { display: flex; flex-direction: column; gap: var(--sp-1); }
.pile.forgotten { opacity: 0.6; }
.pile-hdr {
  display: flex; align-items: baseline; justify-content: space-between;
  padding-bottom: var(--sp-1); border-bottom: 1px solid var(--border-subtle);
}
.pile-name { font-size: var(--fs-sm); font-weight: var(--fw-semibold); color: var(--text-primary); }
.pile-count { font-family: var(--font-mono); font-size: var(--fs-xs); color: var(--text-muted); }

.pile ul { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: var(--sp-1); }

.card-row {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: var(--sp-2);
  padding: var(--sp-2);
  border-radius: var(--r-md);
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  align-items: start;
  transition: border-color var(--motion-fast), box-shadow var(--motion-fast);
}
.card-row:hover { border-color: var(--accent-border); box-shadow: var(--shadow-sm); }
.card-row.slim { padding: var(--sp-1) var(--sp-2); }

.card-main { display: flex; flex-direction: column; gap: 3px; min-width: 0; }
.card-title {
  font-size: var(--fs-sm); font-weight: var(--fw-semibold);
  color: var(--text-primary);
}
.card-meta {
  display: flex; flex-wrap: wrap; gap: 3px;
  font-size: 0.68rem;
}
.meta-chip {
  display: inline-flex; align-items: center; gap: 2px;
  padding: 1px 5px;
  background: var(--bg-surface); border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  color: var(--text-secondary);
}
.meta-chip.cost { font-family: var(--font-mono); color: var(--text-primary); font-weight: var(--fw-semibold); }
.meta-chip.speed { text-transform: capitalize; }
.meta-chip.elem { padding: 1px 3px; }
.card-text {
  font-size: var(--fs-xs); color: var(--text-secondary);
  line-height: 1.35; margin-top: 2px;
}

.card-actions { display: inline-flex; flex-direction: column; gap: 2px; align-self: center; }
.card-actions button {
  font-size: 0.68rem; padding: 2px var(--sp-1); white-space: nowrap;
}

.empty { color: var(--text-faint); font-style: italic; text-align: center; padding: var(--sp-2); font-size: var(--fs-xs); }
.muted { color: var(--text-muted); font-style: italic; font-size: var(--fs-xs); }
</style>
