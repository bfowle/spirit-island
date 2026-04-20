<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import type { Spirit } from '../types'
import Icon from './Icon.vue'
import PresenceTrack from './PresenceTrack.vue'
import { fetchDeck } from '../api'
import {
  STOCK_COLORS,
  SPIRIT_DEFAULT_COLOR,
  getPaletteForSpirit,
  type DiscStyle,
} from '../spiritColors'

const props = defineProps<{
  modelValue: Spirit
  slug?: string
  round?: number
}>()
const emit = defineEmits<{
  'log-event': [event: string, details: Record<string, unknown>]
}>()

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

interface InnateThreshold {
  /** Element counts required at this tier, e.g. { moon: "2", fire: "1" }. */
  [element: string]: string | undefined
  effect?: string
}

interface InnatePower {
  name: string
  speed?: string
  range?: string
  target?: string
  option?: string
  thresholds: InnateThreshold[]
}

// Spirit wiki data (immutable, from data/references/wiki/<slug>.json).
const fullEnergyTrack = ref<string[]>([])
const fullCardplayTrack = ref<string[]>([])
const cardDetailsByName = ref<Record<string, CardDetail>>({})
const innates = ref<InnatePower[]>([])
const wikiError = ref<string | null>(null)
const presenceExpanded = ref<boolean>(false)

// Global decks — loaded once, shared across all SpiritPanel instances so that
// drafted Minor/Major/Unique cards render with full metadata (cost/speed/
// elements/text) the same as spirit-uniques.
const globalDeck = ref<Record<string, CardDetail>>({})

async function loadGlobalDecks() {
  const decks: Array<'minor' | 'major' | 'unique'> = ['minor', 'major', 'unique']
  const all = await Promise.allSettled(decks.map(d => fetchDeck<CardDetail & { name: string }>(d)))
  const merged: Record<string, CardDetail> = {}
  for (const r of all) {
    if (r.status === 'fulfilled') {
      for (const c of r.value) {
        if (c.name) merged[c.name] = c
      }
    }
  }
  globalDeck.value = merged
}

async function loadSpiritMeta(slug: string | undefined) {
  if (!slug) return
  try {
    const res = await fetch(`/api/spirit/${encodeURIComponent(slug)}`)
    if (!res.ok) throw new Error(`${res.status}`)
    const data = await res.json()
    fullEnergyTrack.value = data.presence_energy_track || []
    fullCardplayTrack.value = data.presence_cardplay_track || []
    innates.value = data.innates || []
    // Spirit-local card lookup (uniques + suggested)
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

onMounted(async () => {
  await Promise.all([loadSpiritMeta(props.slug), loadGlobalDecks()])
})
watch(() => props.slug, loadSpiritMeta)

function updateEnergyCovered(next: string[]) {
  props.modelValue.presence_on_track_energy = next
}
function updateCardplayCovered(next: string[]) {
  props.modelValue.presence_on_track_cardplay = next
}

// --- Pile computations -----------------------------------------------------
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

  // Auto-tally elements when a card enters `played_this_turn`; reverse when it leaves.
  const info = cardInfo(card)
  const elementsArr = info?.elements ?? []
  const bumps = (dir: 1 | -1) => {
    const et = (props.modelValue.elements_this_turn ?? {}) as Record<string, number>
    for (const el of elementsArr) {
      const key = el.toLowerCase()
      const curr = et[key] ?? 0
      const next = Math.max(0, curr + dir)
      et[key] = next
    }
    props.modelValue.elements_this_turn = et as Spirit['elements_this_turn']
  }
  if (to === 'played_this_turn' && from !== 'played_this_turn') bumps(1)
  if (from === 'played_this_turn' && to !== 'played_this_turn') bumps(-1)
}

/** Reclaim — move all cards from discard back to hand. The Spirit Island
 *  term for returning cards from the discard pile into the ready hand. */
function reclaimAll() {
  const d = props.modelValue.discard ?? []
  if (!d.length) return
  const h = props.modelValue.hand ?? []
  props.modelValue.hand = [...h, ...d]
  props.modelValue.discard = []
}

const ELEMENT_ICONS: Record<string, string> = {
  sun: 'element-sun', moon: 'element-moon', fire: 'element-fire', air: 'element-air',
  water: 'element-water', earth: 'element-earth', plant: 'element-plant', animal: 'element-animal',
}

const ALL_ELEMENTS = ['sun', 'moon', 'fire', 'air', 'water', 'earth', 'plant', 'animal'] as const
type ElementKey = typeof ALL_ELEMENTS[number]

/** Adjust a single element count by ±N. Used for manually adding elements
 *  from effects that don't come from card plays — Elemental Boon, innate
 *  effects that grant elements, events, or aspect bonuses. */
function bumpElement(el: ElementKey, delta: number) {
  const et = { ...(props.modelValue.elements_this_turn ?? {}) } as Record<string, number>
  const next = Math.max(0, (et[el] ?? 0) + delta)
  if (next === 0) delete et[el]
  else et[el] = next
  props.modelValue.elements_this_turn = et as Spirit['elements_this_turn']
}

function clearAllElements() {
  if (!confirm('Clear all element counts for this turn?')) return
  props.modelValue.elements_this_turn = {}
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
  return cardDetailsByName.value[name] ?? globalDeck.value[name]
}

/** Parse a card's `threshold` text into element requirements + effect description.
 *  Handles formats like "2 Plant — 2 Plant: You may do both." or
 *  "3 Fire + 2 Air: Deal 4 Damage". Returns null for cards without a threshold. */
interface ThresholdInfo {
  reqs: Record<string, number>
  effect: string
  met: boolean
  missing: Record<string, number>
}
function thresholdInfo(name: string): ThresholdInfo | null {
  const card = cardInfo(name)
  const raw = (card as { threshold?: string } | undefined)?.threshold
  if (!raw) return null
  if (raw.toLowerCase().includes('no threshold')) return null

  // Threshold text comes in two shapes from the Wiki:
  //   "2 Plant — 2 Plant: You may do both."                    (single element)
  //   "3 Sun 2 Water 3 Plant — 3 Sun, 2 Water, 3 Plant: ..."   (multi element;
  //      header uses spaces, effect prefix uses commas)
  // We match ALL `<digit>+ <word>+` pairs via global regex so both work.
  const parts = raw.split(/\s*[—–]\s*/)
  const header = parts[0].trim()
  const rest = parts.slice(1).join(' — ').trim()

  const reqs: Record<string, number> = {}
  const reqRe = /(\d+)\s+([A-Za-z]+)/g
  let m: RegExpExecArray | null
  while ((m = reqRe.exec(header)) !== null) {
    reqs[m[2].toLowerCase()] = parseInt(m[1], 10)
  }
  if (!Object.keys(reqs).length) return null

  // Strip the duplicate requirement prefix from the effect text. The prefix is
  // of the form "N Element" joined by commas/spaces, optionally followed by a colon.
  const effect = rest
    .replace(/^(?:\d+\s+[A-Za-z]+[,\s]*)+:?\s*/, '')
    .trim()

  const pool = (props.modelValue.elements_this_turn ?? {}) as Record<string, number>
  const missing: Record<string, number> = {}
  let met = true
  for (const [el, need] of Object.entries(reqs)) {
    const have = pool[el] ?? 0
    if (have < need) {
      met = false
      missing[el] = need - have
    }
  }
  return { reqs, effect: effect || rest, met, missing }
}

// --- Innate tier tracking --------------------------------------------------
// For each innate, compute which tier the spirit's current element pool
// satisfies. Tier `i` is satisfied when all element thresholds in
// `thresholds[i]` are met by `elements_this_turn`. Returns the highest
// satisfied tier (0-indexed) or -1 if none.
function highestTierSatisfied(innate: InnatePower): number {
  const pool = (props.modelValue.elements_this_turn ?? {}) as Record<string, number>
  let highest = -1
  for (let i = 0; i < innate.thresholds.length; i++) {
    const t = innate.thresholds[i]
    const ok = Object.entries(t).every(([k, v]) => {
      if (k === 'effect' || v === undefined) return true
      const need = parseInt(String(v), 10) || 0
      return (pool[k] ?? 0) >= need
    })
    if (ok) highest = i
    else break   // thresholds are cumulative; a miss means higher tiers also miss
  }
  return highest
}

function tierElementNeeds(tier: InnateThreshold): [string, string][] {
  return Object.entries(tier)
    .filter(([k, v]) => k !== 'effect' && v !== undefined)
    .map(([k, v]) => [k, String(v)])
}

function logInnateFired(innateName: string, tier: number) {
  emit('log-event', 'innate_fired', {
    spirit: props.slug,
    innate: innateName,
    tier: tier + 1,
  })
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
        <div class="elements-hdr">
          <span class="section-label">Elements this turn</span>
          <button class="ghost tiny" @click="clearAllElements" title="Reset all element counts (end of turn handled automatically)">Clear</button>
        </div>
        <!-- Live tally of card-play + manual elements. Each element shows a
             stepper so the player can add/remove from non-card sources
             (Elemental Boon, innate grants, events). -->
        <div class="element-steppers">
          <div
            v-for="el in ALL_ELEMENTS"
            :key="el"
            class="el-stepper"
            :class="[`el-${el}`, { active: ((modelValue.elements_this_turn ?? {})[el] ?? 0) > 0 }]"
          >
            <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="16" decorative />
            <span class="el-count-big">{{ (modelValue.elements_this_turn ?? {})[el] ?? 0 }}</span>
            <div class="el-buttons">
              <button class="step" @click="bumpElement(el, -1)" aria-label="decrement">−</button>
              <button class="step" @click="bumpElement(el, 1)" aria-label="increment">+</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Innate powers: each tier listed with its threshold; highest achieved
         tier highlighted based on current elements_this_turn. "Mark fired"
         logs the tier to the state log for retrospective + rules-check. -->
    <div v-if="innates.length" class="innates-block">
      <div class="innates-hdr">
        <span class="section-label">Innate Powers</span>
        <span class="hdr-hint">Tiers highlight when current elements meet the threshold</span>
      </div>
      <div v-for="(innate, ii) in innates" :key="ii" class="innate-card" :class="innate.speed">
        <div class="innate-name">
          <Icon
            v-if="innate.speed"
            :name="innate.speed.toLowerCase() === 'fast' ? 'speed-fast' : 'speed-slow'"
            :size="12"
            decorative
          />
          <span>{{ innate.name }}</span>
        </div>
        <div class="innate-tiers">
          <div
            v-for="(tier, ti) in innate.thresholds"
            :key="ti"
            class="tier-row"
            :class="{
              achieved: ti <= highestTierSatisfied(innate),
              highest: ti === highestTierSatisfied(innate),
            }"
          >
            <span class="tier-num mono">L{{ ti + 1 }}</span>
            <span class="tier-elements">
              <span v-for="[el, n] in tierElementNeeds(tier)" :key="el" class="tier-elem">
                <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="11" decorative />
                <span class="tier-elem-count">{{ n }}</span>
              </span>
            </span>
            <span class="tier-effect">{{ tier.effect }}</span>
            <button
              v-if="ti <= highestTierSatisfied(innate)"
              class="ghost tiny fire-btn"
              @click="logInnateFired(innate.name, ti)"
              :title="`Log '${innate.name}' firing at L${ti + 1} in round ${round ?? '?'}`"
            >⚡ fired</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Presence tracks: bowls always visible. Only the disc color/style
         customization panel is behind a toggle. -->
    <div class="presence-block">
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

      <!-- Only the disc-style customization panel is collapsible -->
      <details class="disc-custom" :open="presenceExpanded" @toggle="presenceExpanded = ($event.target as HTMLDetailsElement).open">
        <summary>
          <span>Disc appearance</span>
          <span class="custom-hint">{{ presenceExpanded ? 'click to hide' : 'change color / style' }}</span>
        </summary>
        <div class="disc-controls">
          <span class="disc-label">Color</span>
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
            <button class="reset-color" @click="resetToDefault" title="Reset to canonical">↺</button>
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
      </details>
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
              <div v-if="thresholdInfo(c)" class="threshold-row" :class="{ met: thresholdInfo(c)?.met }">
                <span class="thresh-label">THR</span>
                <span class="thresh-reqs">
                  <span
                    v-for="[el, n] in Object.entries(thresholdInfo(c)!.reqs)"
                    :key="el"
                    class="thresh-elem"
                    :class="{ need: (thresholdInfo(c)?.missing[el] ?? 0) > 0 }"
                  >
                    <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="11" decorative />
                    <span class="thresh-count">{{ n }}</span>
                  </span>
                </span>
                <span class="thresh-effect">{{ thresholdInfo(c)?.effect }}</span>
                <span v-if="thresholdInfo(c)?.met" class="thresh-badge met-badge">✓ MET</span>
              </div>
            </div>
            <div class="card-actions">
              <button class="ghost" @click="moveCard(c, 'hand', 'played_this_turn')" title="Play this card">Play</button>
              <button class="ghost" @click="moveCard(c, 'hand', 'discard')" title="Discard">Discard</button>
              <button
                class="ghost forget"
                @click="moveCard(c, 'hand', 'forgotten')"
                title="Forget (remove from deck — usually paid as a Major Power cost)"
              >🗑 Forget</button>
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
              <div v-if="thresholdInfo(c)" class="threshold-row" :class="{ met: thresholdInfo(c)?.met }">
                <span class="thresh-label">THR</span>
                <span class="thresh-reqs">
                  <span
                    v-for="[el, n] in Object.entries(thresholdInfo(c)!.reqs)"
                    :key="el"
                    class="thresh-elem"
                    :class="{ need: (thresholdInfo(c)?.missing[el] ?? 0) > 0 }"
                  >
                    <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="11" decorative />
                    <span class="thresh-count">{{ n }}</span>
                  </span>
                </span>
                <span class="thresh-effect">{{ thresholdInfo(c)?.effect }}</span>
                <span v-if="thresholdInfo(c)?.met" class="thresh-badge met-badge">✓ MET</span>
              </div>
            </div>
            <div class="card-actions">
              <button class="ghost" @click="moveCard(c, 'played_this_turn', 'discard')" title="Send to discard">→ Discard</button>
              <button class="ghost" @click="moveCard(c, 'played_this_turn', 'hand')" title="Unplay (return to hand)">↩ Unplay</button>
              <button class="ghost forget" @click="moveCard(c, 'played_this_turn', 'forgotten')" title="Forget">🗑</button>
            </div>
          </li>
          <li v-if="!(modelValue.played_this_turn ?? []).length" class="empty">no plays yet this turn</li>
        </ul>
      </div>

      <div class="pile">
        <div class="pile-hdr">
          <span class="pile-name">Discard</span>
          <span class="pile-count">{{ discardCount }}</span>
          <button
            v-if="discardCount > 0"
            class="ghost tiny reclaim-all"
            @click="reclaimAll"
            :title="`Reclaim all ${discardCount} card(s) from discard back to hand`"
          >↩ Reclaim all</button>
        </div>
        <ul>
          <li v-for="c in modelValue.discard ?? []" :key="c" class="card-row slim">
            <div class="card-main">
              <div class="card-title">{{ c }}</div>
            </div>
            <div class="card-actions">
              <button class="ghost" @click="moveCard(c, 'discard', 'hand')" title="Return this card to hand">↩ Reclaim</button>
              <button class="ghost forget" @click="moveCard(c, 'discard', 'forgotten')" title="Forget">🗑</button>
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
          <li v-for="c in modelValue.forgotten ?? []" :key="c" class="card-row slim forgotten-row">
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
              <button
                class="ghost tiny"
                @click="moveCard(c, 'forgotten', 'discard')"
                title="Un-forget — return to discard (for mistakes)"
              >↩ Un-forget</button>
            </div>
          </li>
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

.elements { display: flex; flex-direction: column; gap: var(--sp-2); }
.elements-hdr { display: flex; align-items: center; justify-content: space-between; gap: var(--sp-2); }
.element-chips { display: flex; flex-wrap: wrap; gap: var(--sp-1); align-items: center; min-height: 1.75rem; }
.element-chip {
  display: inline-flex; align-items: center; gap: 4px;
  padding: 2px var(--sp-2);
  background: var(--bg-muted); border: 1px solid var(--border-subtle);
  border-radius: var(--r-full); font-size: var(--fs-xs);
}
.el-name { text-transform: capitalize; color: var(--text-secondary); }
.el-count { font-family: var(--font-mono); font-weight: var(--fw-semibold); color: var(--text-primary); }

.element-steppers {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(4rem, 1fr));
  gap: var(--sp-1);
}
.el-stepper {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  padding: 4px;
  background: var(--bg-canvas);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  transition: all var(--motion-fast);
  opacity: 0.55;
}
.el-stepper.active {
  opacity: 1;
  background: var(--bg-muted);
  border-color: rgba(123, 184, 245, 0.35);
}
.el-count-big {
  font-family: var(--font-mono);
  font-size: var(--fs-lg);
  font-weight: var(--fw-bold);
  color: var(--text-primary);
  line-height: 1;
  margin: 2px 0;
}
.el-stepper:not(.active) .el-count-big { color: var(--text-muted); }
.el-buttons { display: inline-flex; gap: 2px; }
.el-buttons .step {
  width: 1.1rem; height: 1.1rem;
  padding: 0;
  font-size: 0.72rem;
  line-height: 1;
  border-radius: var(--r-sm);
}
.muted { color: var(--text-muted); font-style: italic; font-size: var(--fs-xs); }

.innates-block {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-3);
  display: flex; flex-direction: column; gap: var(--sp-2);
}
.innates-hdr {
  display: flex; align-items: baseline; justify-content: space-between; flex-wrap: wrap; gap: var(--sp-2);
}
.hdr-hint { font-size: 0.68rem; color: var(--text-muted); font-style: italic; }

.innate-card {
  background: var(--bg-canvas);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  padding: var(--sp-2);
  display: flex; flex-direction: column; gap: 4px;
}
.innate-card.fast { border-left: 3px solid var(--accent-red); }
.innate-card.slow { border-left: 3px solid var(--accent-blue); }

.innate-name {
  display: inline-flex; align-items: center; gap: var(--sp-1);
  font-family: var(--font-mono); font-weight: var(--fw-semibold);
  font-size: var(--fs-sm); color: var(--text-white);
  text-transform: uppercase; letter-spacing: 0.04em;
}

.innate-tiers { display: flex; flex-direction: column; gap: 2px; }
.tier-row {
  display: grid;
  grid-template-columns: 2rem auto 1fr auto;
  gap: var(--sp-2);
  padding: 3px var(--sp-1);
  font-size: var(--fs-xs);
  align-items: center;
  border-radius: var(--r-sm);
  transition: background var(--motion-fast);
}
.tier-row.achieved { background: rgba(82, 183, 136, 0.1); }
.tier-row.highest  {
  background: rgba(82, 183, 136, 0.2);
  box-shadow: inset 0 0 0 1px rgba(82, 183, 136, 0.4);
}
.tier-num { color: var(--accent-amber); font-weight: var(--fw-bold); }
.tier-elements { display: inline-flex; gap: 3px; flex-wrap: wrap; }
.tier-elem {
  display: inline-flex; align-items: center; gap: 2px;
  padding: 1px 4px;
  background: var(--bg-muted);
  border-radius: var(--r-sm);
}
.tier-elem-count {
  font-family: var(--font-mono); font-size: 0.66rem; font-weight: var(--fw-bold);
  color: var(--text-primary);
}
.tier-effect { color: var(--text-secondary); line-height: 1.35; }
.fire-btn { padding: 2px var(--sp-1); color: var(--accent-amber); white-space: nowrap; }
.fire-btn:hover { color: var(--status-success); }

/* Presence block — stats + tracks + bowls all visible by default. Only the
   disc customization (color swatches + style picker) sits inside a collapsible. */
.presence-block {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-3);
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
}
.presence-stats {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sp-3) var(--sp-5);
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

/* Collapsible disc-custom panel */
.disc-custom {
  margin-top: var(--sp-1);
  padding-top: var(--sp-2);
  border-top: 1px dashed var(--border-subtle);
}
.disc-custom > summary {
  list-style: none;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  padding: 2px 0;
}
.disc-custom > summary::-webkit-details-marker { display: none; }
.disc-custom > summary::before {
  content: '▸';
  margin-right: var(--sp-2);
  color: var(--text-muted);
  transition: transform var(--motion-fast);
}
.disc-custom[open] > summary::before { transform: rotate(90deg); }
.disc-custom > summary > span:first-of-type { flex: 1; font-weight: var(--fw-medium); color: var(--text-primary); }
.custom-hint { font-size: var(--fs-xxs); color: var(--text-muted); font-style: italic; }
.disc-custom .disc-controls { margin-top: var(--sp-2); }

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

.card-actions .ghost.forget {
  color: var(--text-muted);
  opacity: 0.7;
  transition: all var(--motion-fast);
}
.card-actions .ghost.forget:hover {
  color: var(--status-danger);
  opacity: 1;
}
.forgotten-row {
  opacity: 0.65;
  background: rgba(80, 60, 80, 0.08);
}
.forgotten-row .card-title { text-decoration: line-through; color: var(--text-muted); }

.threshold-row {
  display: grid;
  grid-template-columns: auto auto 1fr auto;
  gap: var(--sp-2);
  align-items: center;
  padding: 4px var(--sp-2);
  margin-top: 4px;
  font-size: 0.72rem;
  background: var(--bg-inset);
  border: 1px dashed rgba(255, 255, 255, 0.08);
  border-radius: var(--r-sm);
  color: var(--text-secondary);
  transition: all var(--motion-fast);
}
.threshold-row.met {
  background: rgba(82, 183, 136, 0.15);
  border-color: rgba(82, 183, 136, 0.5);
  border-style: solid;
  color: var(--text-primary);
}
.thresh-label {
  font-family: var(--font-mono);
  font-size: 0.6rem;
  font-weight: var(--fw-bold);
  letter-spacing: 0.08em;
  color: var(--text-muted);
}
.threshold-row.met .thresh-label { color: var(--status-success); }
.thresh-reqs { display: inline-flex; gap: 4px; }
.thresh-elem {
  display: inline-flex; align-items: center; gap: 2px;
  padding: 1px 4px;
  background: var(--bg-canvas);
  border-radius: var(--r-sm);
  font-family: var(--font-mono);
  font-size: 0.66rem;
  font-weight: var(--fw-semibold);
}
.thresh-elem.need { opacity: 0.55; }
.threshold-row.met .thresh-elem { background: rgba(82, 183, 136, 0.2); }
.thresh-count { color: var(--text-primary); }
.thresh-effect {
  font-style: italic;
  color: var(--text-secondary);
  line-height: 1.3;
}
.threshold-row.met .thresh-effect { color: var(--text-primary); font-style: normal; }
.thresh-badge {
  font-family: var(--font-mono);
  font-size: 0.62rem;
  font-weight: var(--fw-bold);
  letter-spacing: 0.08em;
  padding: 1px 6px;
  border-radius: var(--r-sm);
}
.met-badge {
  background: var(--status-success);
  color: var(--bg-canvas);
}
</style>
