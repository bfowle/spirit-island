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
  spirits: Record<string, Spirit>
  slugs: string[]
  round?: number
}>()

const emit = defineEmits<{
  'update:spirits': [spirits: Record<string, Spirit>]
  'log-event': [event: string, details: Record<string, unknown>]
}>()

// Active spirit tab
const activeSlug = ref<string>(props.slugs[0] || '')
watch(() => props.slugs, (newSlugs) => {
  if (!newSlugs.includes(activeSlug.value) && newSlugs.length) {
    activeSlug.value = newSlugs[0]
  }
}, { immediate: true })

const activeSpirit = computed(() => props.spirits[activeSlug.value])

// Card pile tabs within the active spirit
type CardTab = 'hand' | 'played' | 'discard' | 'forgotten'
const cardTab = ref<CardTab>('hand')

// ─── SPIRIT METADATA ───
interface CardDetail {
  name?: string
  cost?: string | number
  speed?: string
  range?: string
  target?: string
  elements?: string[]
  text?: string
  card_type?: string
  threshold?: string
}

interface InnateThreshold {
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

// Per-spirit metadata keyed by slug
const spiritMeta = ref<Record<string, {
  energyTrack: string[]
  cardplayTrack: string[]
  innates: InnatePower[]
  cardDetails: Record<string, CardDetail>
}>>({})

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

async function loadSpiritMeta(slug: string) {
  if (!slug || spiritMeta.value[slug]) return
  try {
    const res = await fetch(`/api/spirit/${encodeURIComponent(slug)}`)
    if (!res.ok) throw new Error(`${res.status}`)
    const data = await res.json()
    const lookup: Record<string, CardDetail> = {}
    for (const c of data.unique_card_details || []) {
      if (c.name) lookup[c.name] = c
    }
    for (const c of data.suggested_card_details || []) {
      if (c.name) lookup[c.name] = c
    }
    spiritMeta.value[slug] = {
      energyTrack: data.presence_energy_track || [],
      cardplayTrack: data.presence_cardplay_track || [],
      innates: data.innates || [],
      cardDetails: lookup,
    }
  } catch {
    // Mock defaults for demo
    spiritMeta.value[slug] = {
      energyTrack: ['energy1', 'energy2', 'energy3', 'energy4', 'energy5', 'energy6', 'energy7'],
      cardplayTrack: ['card2', 'card3', 'card4', 'card5', 'card6'],
      innates: [],
      cardDetails: {},
    }
  }
}

onMounted(async () => {
  await loadGlobalDecks()
  await Promise.all(props.slugs.map(loadSpiritMeta))
})

watch(() => props.slugs, async (newSlugs) => {
  await Promise.all(newSlugs.map(loadSpiritMeta))
}, { deep: true })

// ─── COMPUTED FOR ACTIVE SPIRIT ───
const activeMeta = computed(() => spiritMeta.value[activeSlug.value] || {
  energyTrack: [],
  cardplayTrack: [],
  innates: [],
  cardDetails: {},
})

function cardInfo(name: string): CardDetail | undefined {
  return activeMeta.value.cardDetails[name] ?? globalDeck.value[name]
}

// ─── SPIRIT INDICATOR DATA ───
function spiritIndicator(slug: string) {
  const s = props.spirits[slug]
  if (!s) return { energy: 0, cardPlays: 0, presence: 0, hand: 0 }
  const meta = spiritMeta.value[slug]
  const totalPresence = meta
    ? (meta.energyTrack.length - (s.presence_on_track_energy?.length || 0)) +
      (meta.cardplayTrack.length - (s.presence_on_track_cardplay?.length || 0))
    : 0
  return {
    energy: s.energy ?? 0,
    cardPlays: s.card_plays ?? 1,
    presence: totalPresence,
    hand: s.hand?.length ?? 0,
  }
}

function spiritDisplayName(slug: string): string {
  return slug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ')
}

// ─── PRESENCE TRACKS ───
const palette = computed(() => getPaletteForSpirit(activeSlug.value, discColor.value))

const discColor = computed({
  get: () =>
    (activeSpirit.value as { disc_color?: string })?.disc_color ||
    SPIRIT_DEFAULT_COLOR[activeSlug.value] ||
    STOCK_COLORS.indigo,
  set: (v: string) => {
    if (activeSpirit.value) {
      (activeSpirit.value as { disc_color?: string }).disc_color = v
    }
  },
})

const discStyle = computed({
  get: () => (activeSpirit.value as { disc_style?: DiscStyle })?.disc_style || 'glass',
  set: (v: DiscStyle) => {
    if (activeSpirit.value) {
      (activeSpirit.value as { disc_style?: DiscStyle }).disc_style = v
    }
  },
})

function updateEnergyCovered(next: string[]) {
  if (activeSpirit.value) activeSpirit.value.presence_on_track_energy = next
}
function updateCardplayCovered(next: string[]) {
  if (activeSpirit.value) activeSpirit.value.presence_on_track_cardplay = next
}

// ─── RESOURCES ───
function bumpEnergy(delta: number) {
  if (activeSpirit.value) {
    activeSpirit.value.energy = Math.max(0, (activeSpirit.value.energy ?? 0) + delta)
  }
}

function bumpCardPlays(delta: number) {
  if (activeSpirit.value) {
    activeSpirit.value.card_plays = Math.max(0, (activeSpirit.value.card_plays ?? 1) + delta)
  }
}

// ─── ELEMENTS ───
const ELEMENT_ICONS: Record<string, string> = {
  sun: 'element-sun', moon: 'element-moon', fire: 'element-fire', air: 'element-air',
  water: 'element-water', earth: 'element-earth', plant: 'element-plant', animal: 'element-animal',
}
const ALL_ELEMENTS = ['sun', 'moon', 'fire', 'air', 'water', 'earth', 'plant', 'animal'] as const
type ElementKey = typeof ALL_ELEMENTS[number]

function getElementCount(el: ElementKey): number {
  return (activeSpirit.value?.elements_this_turn ?? {})[el] ?? 0
}

function bumpElement(el: ElementKey, delta: number) {
  if (!activeSpirit.value) return
  const et = { ...(activeSpirit.value.elements_this_turn ?? {}) } as Record<string, number>
  const next = Math.max(0, (et[el] ?? 0) + delta)
  if (next === 0) delete et[el]
  else et[el] = next
  activeSpirit.value.elements_this_turn = et as Spirit['elements_this_turn']
}

function clearAllElements() {
  if (activeSpirit.value) {
    activeSpirit.value.elements_this_turn = {}
  }
}

// ─── INNATE POWERS ───
function highestTierSatisfied(innate: InnatePower): number {
  const pool = (activeSpirit.value?.elements_this_turn ?? {}) as Record<string, number>
  let highest = -1
  for (let i = 0; i < innate.thresholds.length; i++) {
    const t = innate.thresholds[i]
    const ok = Object.entries(t).every(([k, v]) => {
      if (k === 'effect' || v === undefined) return true
      const need = parseInt(String(v), 10) || 0
      return (pool[k] ?? 0) >= need
    })
    if (ok) highest = i
    else break
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
    spirit: activeSlug.value,
    innate: innateName,
    tier: tier + 1,
  })
}

// ─── CARD MANAGEMENT ───
const handCards = computed(() => activeSpirit.value?.hand ?? [])
const playedCards = computed(() => activeSpirit.value?.played_this_turn ?? [])
const discardCards = computed(() => activeSpirit.value?.discard ?? [])
const forgottenCards = computed(() => activeSpirit.value?.forgotten ?? [])

function moveCard(card: string, from: keyof Spirit, to: keyof Spirit) {
  if (!activeSpirit.value) return
  const src = (activeSpirit.value[from] as string[] | undefined) ?? []
  const dst = (activeSpirit.value[to] as string[] | undefined) ?? []
  const idx = src.indexOf(card)
  if (idx < 0) return
  src.splice(idx, 1)
  dst.push(card)
  ;(activeSpirit.value[from] as unknown) = src
  ;(activeSpirit.value[to] as unknown) = dst

  // Auto-tally elements when a card enters played_this_turn
  const info = cardInfo(card)
  const elementsArr = info?.elements ?? []
  const bumps = (dir: 1 | -1) => {
    const et = (activeSpirit.value!.elements_this_turn ?? {}) as Record<string, number>
    for (const el of elementsArr) {
      const key = el.toLowerCase()
      const curr = et[key] ?? 0
      const next = Math.max(0, curr + dir)
      et[key] = next
    }
    activeSpirit.value!.elements_this_turn = et as Spirit['elements_this_turn']
  }
  if (to === 'played_this_turn' && from !== 'played_this_turn') bumps(1)
  if (from === 'played_this_turn' && to !== 'played_this_turn') bumps(-1)
}

function reclaimAll() {
  if (!activeSpirit.value) return
  const d = activeSpirit.value.discard ?? []
  if (!d.length) return
  const h = activeSpirit.value.hand ?? []
  activeSpirit.value.hand = [...h, ...d]
  activeSpirit.value.discard = []
}

// ─── THRESHOLD INFO ───
interface ThresholdInfo {
  reqs: Record<string, number>
  effect: string
  met: boolean
  missing: Record<string, number>
}

function thresholdInfo(name: string): ThresholdInfo | null {
  const card = cardInfo(name)
  const raw = (card as { threshold?: string } | undefined)?.threshold
  if (!raw || raw.toLowerCase().includes('no threshold')) return null

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

  const effect = rest.replace(/^(?:\d+\s+[A-Za-z]+[,\s]*)+:?\s*/, '').trim()

  const pool = (activeSpirit.value?.elements_this_turn ?? {}) as Record<string, number>
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

// Track income calculations
function parseTokenValue(token: string | undefined): string {
  if (!token) return '?'
  const m = token.match(/^[a-z]+(\d+)/i)
  return m ? m[1] : token
}

const placedEnergy = computed(() => {
  const full = activeMeta.value.energyTrack
  return Math.max(0, full.length - (activeSpirit.value?.presence_on_track_energy?.length ?? 0))
})

const placedCardplay = computed(() => {
  const full = activeMeta.value.cardplayTrack
  return Math.max(0, full.length - (activeSpirit.value?.presence_on_track_cardplay?.length ?? 0))
})

const nextEnergyIncome = computed(() => {
  const full = activeMeta.value.energyTrack
  if (!full.length) return activeSpirit.value?.energy ?? 0
  const idx = Math.max(0, placedEnergy.value - 1)
  return parseTokenValue(full[idx])
})

const nextCardplayIncome = computed(() => {
  const full = activeMeta.value.cardplayTrack
  if (!full.length) return activeSpirit.value?.card_plays ?? 1
  const idx = Math.max(0, placedCardplay.value - 1)
  return parseTokenValue(full[idx])
})

// Disc customization
const showDiscCustom = ref(false)
const stockColorEntries = Object.entries(STOCK_COLORS)
const styleOptions: { value: DiscStyle; label: string }[] = [
  { value: 'glass', label: 'Glass' },
  { value: 'wood', label: 'Wood' },
  { value: 'solid', label: 'Solid' },
]

function resetToDefaultColor() {
  discColor.value = SPIRIT_DEFAULT_COLOR[activeSlug.value] || STOCK_COLORS.indigo
}
</script>

<template>
  <div class="spirit-zone">
    <!-- Tab strip: one tab per spirit -->
    <div class="spirit-tabs">
      <button
        v-for="slug in slugs"
        :key="slug"
        class="spirit-tab"
        :class="{ active: slug === activeSlug }"
        @click="activeSlug = slug"
      >
        <span class="tab-name">{{ spiritDisplayName(slug) }}</span>
        <!-- Inactive: show mini indicators -->
        <div v-if="slug !== activeSlug" class="tab-chips">
          <span class="chip energy" :title="'Energy'">{{ spiritIndicator(slug).energy }}E</span>
          <span class="chip plays" :title="'Card Plays'">{{ spiritIndicator(slug).cardPlays }}C</span>
          <span class="chip presence" :title="'Presence placed'">{{ spiritIndicator(slug).presence }}P</span>
          <span class="chip hand" :title="'Hand size'">{{ spiritIndicator(slug).hand }}H</span>
        </div>
      </button>
    </div>

    <!-- Active spirit panel -->
    <div v-if="activeSpirit" class="active-panel">
      <!-- Row 1: Resources + Elements -->
      <div class="top-row">
        <!-- Energy & Card Plays steppers -->
        <div class="resource-group">
          <div class="resource">
            <span class="res-label">Energy</span>
            <div class="stepper">
              <button class="step-btn" @click="bumpEnergy(-1)" :disabled="(activeSpirit.energy ?? 0) <= 0">-</button>
              <span class="step-value">{{ activeSpirit.energy ?? 0 }}</span>
              <button class="step-btn" @click="bumpEnergy(1)">+</button>
            </div>
            <span class="res-hint">+{{ nextEnergyIncome }}/turn</span>
          </div>
          <div class="resource">
            <span class="res-label">Card Plays</span>
            <div class="stepper">
              <button class="step-btn" @click="bumpCardPlays(-1)" :disabled="(activeSpirit.card_plays ?? 1) <= 0">-</button>
              <span class="step-value">{{ activeSpirit.card_plays ?? 1 }}</span>
              <button class="step-btn" @click="bumpCardPlays(1)">+</button>
            </div>
            <span class="res-hint">{{ nextCardplayIncome }}/turn</span>
          </div>
        </div>

        <!-- Elements row -->
        <div class="elements-row">
          <div class="elements-hdr">
            <span class="sec-label">Elements</span>
            <button class="link-btn" @click="clearAllElements">Clear</button>
          </div>
          <div class="element-grid">
            <div
              v-for="el in ALL_ELEMENTS"
              :key="el"
              class="el-cell"
              :class="{ active: getElementCount(el) > 0 }"
            >
              <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="14" decorative />
              <span class="el-val">{{ getElementCount(el) }}</span>
              <div class="el-btns">
                <button @click="bumpElement(el, -1)" :disabled="getElementCount(el) <= 0">-</button>
                <button @click="bumpElement(el, 1)">+</button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Row 2: Presence Tracks -->
      <div class="presence-section">
        <div class="presence-header">
          <span class="sec-label">Presence Tracks</span>
          <div class="presence-stats">
            <span>Energy: {{ placedEnergy }}/{{ activeMeta.energyTrack.length }}</span>
            <span>Cards: {{ placedCardplay }}/{{ activeMeta.cardplayTrack.length }}</span>
          </div>
          <button class="link-btn" @click="showDiscCustom = !showDiscCustom">
            {{ showDiscCustom ? 'Hide' : 'Disc Style' }}
          </button>
        </div>
        <div v-if="showDiscCustom" class="disc-custom">
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
            <label class="swatch-custom" title="Custom color">
              <input type="color" v-model="discColor" />
            </label>
            <button class="reset-btn" @click="resetToDefaultColor" title="Reset">R</button>
          </div>
          <div class="style-btns">
            <button
              v-for="opt in styleOptions"
              :key="opt.value"
              class="style-btn"
              :class="{ active: discStyle === opt.value }"
              @click="discStyle = opt.value"
            >{{ opt.label }}</button>
          </div>
        </div>
        <div class="track-rows">
          <PresenceTrack
            v-if="activeMeta.energyTrack.length"
            label="Energy"
            :full-track="activeMeta.energyTrack"
            :covered-tokens="activeSpirit.presence_on_track_energy ?? []"
            :palette="palette"
            :disc-style="discStyle"
            @update:covered-tokens="updateEnergyCovered"
          />
          <PresenceTrack
            v-if="activeMeta.cardplayTrack.length"
            label="Card Plays"
            :full-track="activeMeta.cardplayTrack"
            :covered-tokens="activeSpirit.presence_on_track_cardplay ?? []"
            :palette="palette"
            :disc-style="discStyle"
            @update:covered-tokens="updateCardplayCovered"
          />
        </div>
      </div>

      <!-- Row 3: Innate Powers -->
      <div v-if="activeMeta.innates.length" class="innates-section">
        <div class="innates-hdr">
          <span class="sec-label">Innate Powers</span>
        </div>
        <div class="innate-grid">
          <div
            v-for="(innate, ii) in activeMeta.innates"
            :key="ii"
            class="innate-card"
            :class="[innate.speed?.toLowerCase(), { triggered: highestTierSatisfied(innate) >= 0 }]"
          >
            <div class="innate-header">
              <Icon
                v-if="innate.speed"
                :name="innate.speed.toLowerCase() === 'fast' ? 'speed-fast' : 'speed-slow'"
                :size="12"
                decorative
              />
              <span class="innate-name">{{ innate.name }}</span>
            </div>
            <div class="tiers">
              <div
                v-for="(tier, ti) in innate.thresholds"
                :key="ti"
                class="tier"
                :class="{
                  achieved: ti <= highestTierSatisfied(innate),
                  current: ti === highestTierSatisfied(innate),
                }"
              >
                <span class="tier-num">{{ ti + 1 }}</span>
                <span class="tier-elems">
                  <span v-for="[el, n] in tierElementNeeds(tier)" :key="el" class="t-el">
                    <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="10" decorative />
                    <span>{{ n }}</span>
                  </span>
                </span>
                <span class="tier-fx">{{ tier.effect }}</span>
                <button
                  v-if="ti <= highestTierSatisfied(innate)"
                  class="fire-btn"
                  @click="logInnateFired(innate.name, ti)"
                  title="Log as fired"
                >Fire</button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Row 4: Cards (tabbed) -->
      <div class="cards-section">
        <div class="card-tabs">
          <button
            :class="{ active: cardTab === 'hand' }"
            @click="cardTab = 'hand'"
          >Hand ({{ handCards.length }})</button>
          <button
            :class="{ active: cardTab === 'played' }"
            @click="cardTab = 'played'"
          >Played ({{ playedCards.length }})</button>
          <button
            :class="{ active: cardTab === 'discard' }"
            @click="cardTab = 'discard'"
          >Discard ({{ discardCards.length }})</button>
          <button
            v-if="forgottenCards.length"
            :class="{ active: cardTab === 'forgotten' }"
            @click="cardTab = 'forgotten'"
          >Forgotten ({{ forgottenCards.length }})</button>
          <button
            v-if="cardTab === 'discard' && discardCards.length"
            class="reclaim-btn"
            @click="reclaimAll"
          >Reclaim All</button>
        </div>

        <div class="card-list">
          <!-- HAND -->
          <template v-if="cardTab === 'hand'">
            <div v-if="!handCards.length" class="empty-msg">Hand is empty</div>
            <div v-for="c in handCards" :key="c" class="card-row">
              <div class="card-info">
                <div class="card-top">
                  <span v-if="cardInfo(c)?.cost !== undefined" class="badge cost">{{ cardInfo(c)?.cost }}E</span>
                  <span v-if="cardInfo(c)?.speed" class="badge speed" :class="cardInfo(c)?.speed?.toLowerCase()">
                    <Icon
                      :name="cardInfo(c)?.speed?.toLowerCase() === 'fast' ? 'speed-fast' : 'speed-slow'"
                      :size="10"
                      decorative
                    />
                    {{ cardInfo(c)?.speed }}
                  </span>
                  <span v-if="cardInfo(c)?.range" class="badge range">R{{ cardInfo(c)?.range }}</span>
                  <span class="card-name">{{ c }}</span>
                </div>
                <div v-if="cardInfo(c)?.elements?.length" class="card-elements">
                  <span v-for="el in cardInfo(c)?.elements" :key="el" class="c-el">
                    <Icon v-if="ELEMENT_ICONS[el.toLowerCase()]" :name="ELEMENT_ICONS[el.toLowerCase()]" :size="12" decorative />
                  </span>
                </div>
                <div v-if="cardInfo(c)?.text" class="card-text">{{ cardInfo(c)?.text }}</div>
                <div v-if="thresholdInfo(c)" class="card-threshold" :class="{ met: thresholdInfo(c)?.met }">
                  <span class="thr-label">THR</span>
                  <span class="thr-reqs">
                    <span
                      v-for="[el, n] in Object.entries(thresholdInfo(c)!.reqs)"
                      :key="el"
                      class="thr-el"
                      :class="{ missing: (thresholdInfo(c)?.missing[el] ?? 0) > 0 }"
                    >
                      <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="10" decorative />
                      {{ n }}
                    </span>
                  </span>
                  <span class="thr-effect">{{ thresholdInfo(c)?.effect }}</span>
                  <span v-if="thresholdInfo(c)?.met" class="met-badge">MET</span>
                </div>
              </div>
              <div class="card-actions">
                <button @click="moveCard(c, 'hand', 'played_this_turn')">Play</button>
                <button @click="moveCard(c, 'hand', 'discard')">Discard</button>
                <button class="forget" @click="moveCard(c, 'hand', 'forgotten')">Forget</button>
              </div>
            </div>
          </template>

          <!-- PLAYED -->
          <template v-if="cardTab === 'played'">
            <div v-if="!playedCards.length" class="empty-msg">No cards played this turn</div>
            <div v-for="c in playedCards" :key="c" class="card-row compact">
              <div class="card-info">
                <div class="card-top">
                  <span v-if="cardInfo(c)?.cost !== undefined" class="badge cost">{{ cardInfo(c)?.cost }}E</span>
                  <span v-if="cardInfo(c)?.speed" class="badge speed" :class="cardInfo(c)?.speed?.toLowerCase()">
                    <Icon
                      :name="cardInfo(c)?.speed?.toLowerCase() === 'fast' ? 'speed-fast' : 'speed-slow'"
                      :size="10"
                      decorative
                    />
                  </span>
                  <span class="card-name">{{ c }}</span>
                </div>
                <div v-if="thresholdInfo(c)" class="card-threshold compact" :class="{ met: thresholdInfo(c)?.met }">
                  <span class="thr-reqs">
                    <span
                      v-for="[el, n] in Object.entries(thresholdInfo(c)!.reqs)"
                      :key="el"
                      class="thr-el"
                      :class="{ missing: (thresholdInfo(c)?.missing[el] ?? 0) > 0 }"
                    >
                      <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="10" decorative />
                      {{ n }}
                    </span>
                  </span>
                  <span v-if="thresholdInfo(c)?.met" class="met-badge">MET</span>
                </div>
              </div>
              <div class="card-actions">
                <button @click="moveCard(c, 'played_this_turn', 'discard')">Discard</button>
                <button @click="moveCard(c, 'played_this_turn', 'hand')">Unplay</button>
              </div>
            </div>
          </template>

          <!-- DISCARD -->
          <template v-if="cardTab === 'discard'">
            <div v-if="!discardCards.length" class="empty-msg">Discard pile is empty</div>
            <div v-for="c in discardCards" :key="c" class="card-row compact">
              <div class="card-info">
                <span class="card-name">{{ c }}</span>
              </div>
              <div class="card-actions">
                <button @click="moveCard(c, 'discard', 'hand')">Reclaim</button>
                <button class="forget" @click="moveCard(c, 'discard', 'forgotten')">Forget</button>
              </div>
            </div>
          </template>

          <!-- FORGOTTEN -->
          <template v-if="cardTab === 'forgotten'">
            <div v-if="!forgottenCards.length" class="empty-msg">No forgotten cards</div>
            <div v-for="c in forgottenCards" :key="c" class="card-row compact muted">
              <div class="card-info">
                <span class="card-name">{{ c }}</span>
              </div>
              <div class="card-actions">
                <button @click="moveCard(c, 'forgotten', 'discard')">Un-forget</button>
              </div>
            </div>
          </template>
        </div>
      </div>
    </div>

    <div v-else class="no-spirit">
      <span>No spirits in game</span>
    </div>
  </div>
</template>

<style scoped>
/* ═══════════════════════════════════════════════════════════════════════════ */
/* SPIRIT ZONE — SCIENTIST'S DASHBOARD                                         */
/* ═══════════════════════════════════════════════════════════════════════════ */

.spirit-zone {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

/* ─── TAB STRIP ─── */
.spirit-tabs {
  display: flex;
  gap: 1px;
  background: var(--bg-muted);
  border-bottom: 1px solid var(--border-default);
  flex-shrink: 0;
}

.spirit-tab {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--sp-1);
  padding: var(--sp-3) var(--sp-4);
  background: var(--bg-elevated);
  border: none;
  border-radius: 0;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--duration-base) var(--ease);
  text-align: left;
  height: auto;
}

.spirit-tab:first-child { border-top-left-radius: var(--radius-lg); }
.spirit-tab:last-child { border-top-right-radius: var(--radius-lg); }

.spirit-tab:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.spirit-tab.active {
  background: var(--bg-surface);
  color: var(--text-primary);
  border-bottom: 2px solid var(--color-accent);
}

.tab-name {
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}

.tab-chips {
  display: flex;
  gap: var(--sp-1);
  flex-wrap: wrap;
}

.chip {
  font-family: var(--font-mono);
  font-size: 9px;
  font-weight: var(--weight-semibold);
  padding: 1px 4px;
  border-radius: var(--radius-sm);
  background: var(--bg-muted);
  color: var(--text-muted);
}

.chip.energy { background: rgba(234, 179, 8, 0.15); color: #fbbf24; }
.chip.plays { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
.chip.presence { background: rgba(168, 85, 247, 0.15); color: #c084fc; }
.chip.hand { background: rgba(34, 197, 94, 0.15); color: #4ade80; }

/* ─── ACTIVE PANEL ─── */
.active-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--sp-4);
  padding: var(--sp-4);
  overflow-y: auto;
}

.no-spirit {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
}

/* ─── TOP ROW: Resources + Elements ─── */
.top-row {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: var(--sp-5);
  align-items: start;
}

.resource-group {
  display: flex;
  gap: var(--sp-4);
}

.resource {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--sp-1);
}

.res-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
}

.stepper {
  display: flex;
  align-items: center;
  gap: 2px;
  background: var(--bg-muted);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  padding: 2px;
}

.step-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-subtle);
  border: none;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  font-size: var(--text-lg);
  font-weight: var(--weight-bold);
  cursor: pointer;
  padding: 0;
}

.step-btn:hover:not(:disabled) {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.step-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.step-value {
  min-width: 32px;
  text-align: center;
  font-family: var(--font-mono);
  font-size: var(--text-xl);
  font-weight: var(--weight-bold);
  color: var(--text-primary);
}

.res-hint {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-muted);
}

/* ─── ELEMENTS ROW ─── */
.elements-row {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.elements-hdr {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.sec-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
}

.link-btn {
  background: none;
  border: none;
  padding: 0;
  font-size: var(--text-xs);
  color: var(--color-accent);
  cursor: pointer;
  height: auto;
}

.link-btn:hover {
  color: var(--color-accent-hover);
  text-decoration: underline;
}

.element-grid {
  display: grid;
  grid-template-columns: repeat(8, 1fr);
  gap: 2px;
}

.el-cell {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  padding: var(--sp-1) 2px;
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  opacity: 0.5;
  transition: all var(--duration-base) var(--ease);
}

.el-cell.active {
  opacity: 1;
  background: var(--bg-elevated);
  border-color: var(--border-strong);
}

.el-val {
  font-family: var(--font-mono);
  font-size: var(--text-base);
  font-weight: var(--weight-bold);
  color: var(--text-primary);
  line-height: 1;
}

.el-cell:not(.active) .el-val {
  color: var(--text-faint);
}

.el-btns {
  display: flex;
  gap: 1px;
}

.el-btns button {
  width: 16px;
  height: 16px;
  padding: 0;
  font-size: 10px;
  line-height: 1;
  background: var(--bg-subtle);
  border: 1px solid var(--border-subtle);
  border-radius: 2px;
  color: var(--text-muted);
  cursor: pointer;
}

.el-btns button:hover:not(:disabled) {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.el-btns button:disabled {
  opacity: 0.3;
}

/* ─── PRESENCE SECTION ─── */
.presence-section {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
  padding: var(--sp-3);
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
}

.presence-header {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
}

.presence-stats {
  display: flex;
  gap: var(--sp-3);
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-secondary);
  margin-left: auto;
}

.disc-custom {
  display: flex;
  gap: var(--sp-3);
  align-items: center;
  padding: var(--sp-2);
  background: var(--bg-muted);
  border-radius: var(--radius-sm);
}

.swatches {
  display: flex;
  gap: 4px;
  align-items: center;
}

.swatch {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--s);
  border: 2px solid transparent;
  cursor: pointer;
  padding: 0;
}

.swatch.active {
  border-color: white;
}

.swatch-custom {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  overflow: hidden;
  cursor: pointer;
}

.swatch-custom input {
  width: 24px;
  height: 24px;
  margin: -3px;
  padding: 0;
  border: none;
  cursor: pointer;
}

.reset-btn {
  width: 18px;
  height: 18px;
  padding: 0;
  font-size: 10px;
  background: var(--bg-subtle);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  cursor: pointer;
}

.style-btns {
  display: flex;
  gap: 2px;
}

.style-btn {
  padding: 2px 8px;
  font-size: var(--text-xs);
  background: var(--bg-subtle);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  cursor: pointer;
  height: auto;
}

.style-btn.active {
  background: var(--color-accent);
  color: white;
  border-color: var(--color-accent);
}

.track-rows {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

/* ─── INNATES SECTION ─── */
.innates-section {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.innates-hdr {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.innate-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: var(--sp-2);
}

.innate-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  padding: var(--sp-3);
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.innate-card.fast {
  border-left: 3px solid var(--color-danger);
}

.innate-card.slow {
  border-left: 3px solid var(--color-accent);
}

.innate-card.triggered {
  background: var(--bg-hover);
}

.innate-header {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.innate-name {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.03em;
  color: var(--text-primary);
}

.tiers {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.tier {
  display: grid;
  grid-template-columns: 1.5rem auto 1fr auto;
  gap: var(--sp-2);
  padding: var(--sp-1) var(--sp-2);
  font-size: var(--text-xs);
  align-items: center;
  border-radius: var(--radius-sm);
  background: var(--bg-muted);
}

.tier.achieved {
  background: rgba(34, 197, 94, 0.1);
}

.tier.current {
  background: rgba(34, 197, 94, 0.2);
  box-shadow: inset 0 0 0 1px rgba(34, 197, 94, 0.4);
}

.tier-num {
  font-family: var(--font-mono);
  font-weight: var(--weight-bold);
  color: var(--color-fear);
}

.tier-elems {
  display: flex;
  gap: 3px;
  flex-wrap: wrap;
}

.t-el {
  display: flex;
  align-items: center;
  gap: 1px;
  padding: 1px 3px;
  background: var(--bg-subtle);
  border-radius: 2px;
  font-family: var(--font-mono);
  font-size: 9px;
  font-weight: var(--weight-bold);
}

.tier-fx {
  color: var(--text-secondary);
  font-size: var(--text-xs);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.fire-btn {
  padding: 2px 6px;
  font-size: 9px;
  background: var(--color-success);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  height: auto;
}

.fire-btn:hover {
  filter: brightness(1.1);
}

/* ─── CARDS SECTION ─── */
.cards-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
  min-height: 200px;
}

.card-tabs {
  display: flex;
  gap: var(--sp-1);
  padding: var(--sp-1);
  background: var(--bg-muted);
  border-radius: var(--radius-md);
  flex-shrink: 0;
}

.card-tabs button {
  padding: var(--sp-1) var(--sp-3);
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--text-muted);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  height: auto;
  transition: all var(--duration-base) var(--ease);
}

.card-tabs button:hover {
  color: var(--text-secondary);
}

.card-tabs button.active {
  background: var(--bg-surface);
  color: var(--text-primary);
  box-shadow: var(--shadow-sm);
}

.reclaim-btn {
  margin-left: auto;
  background: var(--color-accent) !important;
  color: white !important;
}

.card-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.empty-msg {
  color: var(--text-muted);
  font-size: var(--text-sm);
  padding: var(--sp-4);
  text-align: center;
}

.card-row {
  display: flex;
  justify-content: space-between;
  gap: var(--sp-3);
  padding: var(--sp-3);
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  align-items: flex-start;
}

.card-row.compact {
  padding: var(--sp-2) var(--sp-3);
  align-items: center;
}

.card-row.muted {
  opacity: 0.6;
}

.card-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--sp-1);
}

.card-top {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  flex-wrap: wrap;
}

.badge {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  padding: 2px 6px;
  font-family: var(--font-mono);
  font-size: 10px;
  font-weight: var(--weight-semibold);
  border-radius: var(--radius-sm);
  background: var(--bg-muted);
  color: var(--text-secondary);
}

.badge.cost {
  background: rgba(234, 179, 8, 0.15);
  color: #fbbf24;
}

.badge.speed.fast {
  background: rgba(239, 68, 68, 0.15);
  color: #f87171;
}

.badge.speed.slow {
  background: rgba(59, 130, 246, 0.15);
  color: #60a5fa;
}

.badge.range {
  background: rgba(168, 85, 247, 0.15);
  color: #c084fc;
}

.card-name {
  font-weight: var(--weight-medium);
  font-size: var(--text-sm);
  color: var(--text-primary);
}

.card-elements {
  display: flex;
  gap: var(--sp-1);
}

.c-el {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  background: var(--bg-muted);
  border-radius: var(--radius-sm);
}

.card-text {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  line-height: 1.4;
}

.card-threshold {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-2);
  background: var(--bg-muted);
  border-radius: var(--radius-sm);
  border-left: 2px solid var(--color-warning);
}

.card-threshold.met {
  border-left-color: var(--color-success);
  background: rgba(34, 197, 94, 0.08);
}

.card-threshold.compact {
  padding: var(--sp-1) var(--sp-2);
}

.thr-label {
  font-family: var(--font-mono);
  font-size: 9px;
  font-weight: var(--weight-bold);
  color: var(--text-muted);
}

.thr-reqs {
  display: flex;
  gap: var(--sp-1);
}

.thr-el {
  display: flex;
  align-items: center;
  gap: 2px;
  font-family: var(--font-mono);
  font-size: 10px;
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
}

.thr-el.missing {
  color: var(--color-danger);
}

.thr-effect {
  flex: 1;
  font-size: var(--text-xs);
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.met-badge {
  font-family: var(--font-mono);
  font-size: 9px;
  font-weight: var(--weight-bold);
  padding: 2px 6px;
  background: var(--color-success);
  color: white;
  border-radius: var(--radius-sm);
}

.card-actions {
  display: flex;
  gap: var(--sp-1);
  flex-shrink: 0;
}

.card-actions button {
  padding: var(--sp-1) var(--sp-2);
  font-size: var(--text-xs);
  background: var(--bg-subtle);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  cursor: pointer;
  height: auto;
}

.card-actions button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.card-actions button.forget {
  color: var(--color-danger);
}

.card-actions button.forget:hover {
  background: rgba(239, 68, 68, 0.15);
}
</style>
