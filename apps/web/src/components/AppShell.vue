<script setup lang="ts">
import { computed, onMounted, onBeforeUnmount, ref, watch } from 'vue'
import type { GameState, Phase } from '../types'
import { computeWinProb } from '../lib/winprob'
import { fetchSpiritAffinity, type SpiritAffinityMap } from '../api'

import SidebarNav from './SidebarNav.vue'

import Board from './Board.vue'
import SpiritPanel from './SpiritPanel.vue'
import InvaderDeck from './InvaderDeck.vue'
import FearDeck from './FearDeck.vue'
import EventDeck from './EventDeck.vue'
import StatsPanel from './StatsPanel.vue'
import UiIcon from './UiIcon.vue'

type InvaderStage = 1 | 2 | 3
type InvaderTerrain = 'jungle' | 'mountain' | 'sands' | 'wetland' | 'coastal'
const TERRAINS: InvaderTerrain[] = ['jungle', 'mountain', 'sands', 'wetland', 'coastal']
import Retrospective from './Retrospective.vue'
import TerrainTimeline from './TerrainTimeline.vue'
import TurnController from './TurnController.vue'
import Pools from './Pools.vue'
import LandEditor from './LandEditor.vue'
import { getLayout } from '../boardLayouts'

/**
 * Aegis dashboard shell. Mirrors the si-mockup.html layout exactly:
 *
 *   [ SidebarNav 248px | header(matchup·phase·status·advance)    | SpiritRail 360px ]
 *   [ SidebarNav       | main (aegis-grid-bg) with dash-grid     | SpiritRail       ]
 *
 * Main dash-grid has 3 zones (board / decks / analytics) which REFLOW per
 * phase via `html[data-phase]`. Each zone is ONE aegis-card with sub-cards
 * inside (decks-wrap holds 3 deck-stations; charts-wrap holds 3 chart
 * blocks). TurnController + Pools live in a collapsible ops drawer, not
 * in the main grid — keeps the dashboard focused on state, not data entry.
 */

const props = defineProps<{ modelValue: GameState }>()
const emit = defineEmits<{
  'update:modelValue': [value: GameState]
  'log-event': [event: string, details: Record<string, unknown>]
  'new-game': []
  'saved-games': []
  'archive': []
  'export': []
  'reset-fear-pool': []
  'bump-pool': [pool: 'blight', delta: number]
}>()

const activeSection = ref<string>('overview')

// Shared win-prob for header status bar
const affinityMap = ref<SpiritAffinityMap | null>(null)
;(async () => {
  try { affinityMap.value = await fetchSpiritAffinity() } catch { /* optional */ }
})()
const wp = computed(() => computeWinProb(props.modelValue, affinityMap.value))
const winPct = computed(() => `${Math.round(wp.value.mean * 100)}%`)
const winState = computed(() => wp.value.endState)

// Game-over: once won or lost, phase advancement is locked. Imminent still
// allows progressing (you still need to survive the Invader phase to clinch).
const gameOver = computed(() => winState.value === 'won' || winState.value === 'lost')

// Section nav — "terrain" and "retro" open slide-in drawers rather than
// scroll to inline content. They're analysis panels that clutter the
// dashboard when always visible.
const drawerOpen = ref<'terrain' | 'retro' | null>(null)
const sectionAnchors: Record<string, string> = {
  overview: 'zone-board',
  board:    'zone-board',
  invader:  'sec-invader',
  fear:     'sec-fear',
  events:   'sec-events',
  stats:    'sec-stats',
}
function selectSection(id: string) {
  activeSection.value = id
  if (id === 'terrain' || id === 'retro') {
    drawerOpen.value = drawerOpen.value === id ? null : id
    return
  }
  drawerOpen.value = null
  const el = document.getElementById(sectionAnchors[id])
  if (el) el.scrollIntoView({ behavior: 'smooth', block: 'start' })
}

// Phase ↔ html[data-phase] sync for CSS reflow
watch(() => props.modelValue.phase, (p: Phase) => {
  document.documentElement.dataset.phase = p
}, { immediate: true })

// Ops drawer toggle — off by default; keeps dashboard focused on state
const opsOpen = ref(false)

// Advance-phase button
const phaseOrder = ['setup', 'growth', 'fast', 'event', 'fear', 'invader', 'slow', 'timepasses'] as const
const nextPhaseLabel = computed(() => {
  const curr = phaseOrder.indexOf(props.modelValue.phase as typeof phaseOrder[number])
  if (curr < 0 || curr >= phaseOrder.length - 1) return '→ Round ' + (props.modelValue.round + 1)
  const next = phaseOrder[curr + 1]
  return `→ ${next.charAt(0).toUpperCase() + next.slice(1)}`
})
function advancePhase() {
  if (gameOver.value) return
  const curr = phaseOrder.indexOf(props.modelValue.phase as typeof phaseOrder[number])
  if (curr < 0) return
  if (curr >= phaseOrder.length - 1) {
    emit('update:modelValue', { ...props.modelValue, round: props.modelValue.round + 1, phase: 'growth' })
  } else {
    emit('update:modelValue', { ...props.modelValue, phase: phaseOrder[curr + 1] })
  }
}

const boardIds = computed(() => Object.keys(props.modelValue.board_state ?? {}))
const activeBoardId = ref<string>('')
watch(boardIds, (ids) => {
  if (!ids.includes(activeBoardId.value) && ids.length > 0) activeBoardId.value = ids[0]
}, { immediate: true })

const activeBoard = computed(() => props.modelValue.board_state[activeBoardId.value])
const boardVariant = computed(() => activeBoard.value?.variant || 'balanced')
const boardBgUrl = computed(() =>
  activeBoardId.value ? `/board-images/${activeBoardId.value}-${boardVariant.value}.png` : '',
)

// Land positions come from the shared `boardLayouts` (hand-eyeballed from the
// Wiki board images). Each entry is { x, y } in 0..100 percent coords
// relative to the rendered board image. Falls back to a generic grid when a
// board/variant's layout isn't defined.
const boardLayout = computed(() => getLayout(activeBoardId.value, boardVariant.value))
function coordFor(id: string): { x: number; y: number } {
  const pos = boardLayout.value?.positions[id]
  if (pos) return { x: pos.x, y: pos.y }
  // Fallback: rough grid so markers at least land on the canvas
  const n = parseInt(id, 10) || 1
  return { x: ((n - 1) % 4) * 22 + 20, y: Math.floor((n - 1) / 4) * 35 + 25 }
}

// Selected land for the inline land popover (click a marker to open)
const selectedLandId = ref<string | null>(null)
function selectLand(id: string) {
  selectedLandId.value = selectedLandId.value === id ? null : id
}

// Right-panel mode: toggleable between spirits + decks. Map on the left,
// analytics rail on the far right.
const rightPanelMode = ref<'spirits' | 'decks'>('spirits')

// When on a decks-focused phase, auto-flip the right panel to decks
watch(() => props.modelValue.phase, (p: Phase) => {
  if (p === 'event' || p === 'fear' || p === 'invader') rightPanelMode.value = 'decks'
  else if (p === 'growth' || p === 'fast' || p === 'slow') rightPanelMode.value = 'spirits'
})

// Active spirit (for compact spirit view with multi-spirit tabs)
const spiritSlugs = computed(() => Object.keys(props.modelValue.spirits ?? {}))
const activeSpiritSlug = ref<string>('')
watch(spiritSlugs, (slugs) => {
  if (!slugs.includes(activeSpiritSlug.value) && slugs.length > 0) {
    activeSpiritSlug.value = slugs[0]
  }
}, { immediate: true })

// Zone detail drawers — open the full Vue component for detailed editing
// of decks. Board does NOT use a drawer; it toggles in-place between the
// PNG map view and the full grid view (Board.vue).
const detailDrawer = ref<'invader' | 'fear' | 'event' | 'spirit' | null>(null)

// Board view mode: 'map' (PNG backdrop + land tiles) or 'grid' (Board.vue's
// full grid layout filling the same space). Swapped via the top-right button.
const boardView = ref<'map' | 'grid'>('map')

// Global Escape key — closes any open drawer/popover (top-most first)
function onKey(e: KeyboardEvent) {
  if (e.key !== 'Escape') return
  if (selectedLandId.value) { selectedLandId.value = null; return }
  if (detailDrawer.value) { detailDrawer.value = null; return }
  if (drawerOpen.value) { drawerOpen.value = null; return }
  if (opsOpen.value) { opsOpen.value = false; return }
}
onMounted(() => document.addEventListener('keydown', onKey))
onBeforeUnmount(() => document.removeEventListener('keydown', onKey))

// Terrain class for a land marker's border/tint
function terrainClass(terrain?: string): string {
  if (!terrain) return ''
  return `terrain-${terrain.toLowerCase()}`
}

// Presence disc summary per land
function presenceOn(landId: string): Array<{ slug: string; count: number; color: string }> {
  const out: Array<{ slug: string; count: number; color: string }> = []
  for (const [slug, spirit] of Object.entries(props.modelValue.spirits ?? {})) {
    const key = `${activeBoardId.value}.${landId}`
    const count = spirit.presence_on_board?.[key] ?? 0
    if (count > 0) out.push({ slug, count, color: spirit.disc_color ?? '#7bb8f5' })
  }
  return out
}

// ─── Interactive compact deck helpers ─────────────────────────────────────

function stageLabel(n?: number): string {
  return n === 1 ? 'I' : n === 2 ? 'II' : n === 3 ? 'III' : '?'
}

function cycleStage(slot: 'ravage' | 'build' | 'explore') {
  const deck = props.modelValue.invader_deck
  if (!deck || !deck[slot]) return
  const curr = (deck[slot]!.stage ?? 1) as InvaderStage
  const next: InvaderStage = curr === 1 ? 2 : curr === 2 ? 3 : 1
  emit('update:modelValue', {
    ...props.modelValue,
    invader_deck: { ...deck, [slot]: { ...deck[slot]!, stage: next } },
  })
}

function cycleTerrain(slot: 'ravage' | 'build' | 'explore') {
  const deck = props.modelValue.invader_deck
  if (!deck || !deck[slot]) return
  const curr = deck[slot]!.terrain as InvaderTerrain | undefined
  const idx = curr ? TERRAINS.indexOf(curr) : -1
  const next = TERRAINS[(idx + 1) % TERRAINS.length]
  emit('update:modelValue', {
    ...props.modelValue,
    invader_deck: { ...deck, [slot]: { ...deck[slot]!, terrain: next } },
  })
}

// Fear slot derivation — tier counts + states (unseen/earned/resolved)
interface FearSlot { id: number; tier: 1 | 2 | 3; state: 'unseen' | 'earned' | 'resolved' }
const fearSlots = computed<FearSlot[]>(() => {
  const fd = props.modelValue.fear_deck
  if (!fd) return []
  const third = Math.ceil(fd.deck_size / 3)
  const tiers = fd.tier_counts ?? [third, third, Math.max(0, fd.deck_size - 2 * third)]
  const slots: FearSlot[] = []
  for (let i = 0; i < tiers[0]; i++) slots.push({ id: slots.length, tier: 1, state: 'unseen' })
  for (let i = 0; i < tiers[1]; i++) slots.push({ id: slots.length, tier: 2, state: 'unseen' })
  for (let i = 0; i < tiers[2]; i++) slots.push({ id: slots.length, tier: 3, state: 'unseen' })
  let cursor = 0
  for (let i = 0; i < fd.resolved.length && cursor < slots.length; i++) slots[cursor++].state = 'resolved'
  for (let i = 0; i < fd.earned.length && cursor < slots.length; i++) slots[cursor++].state = 'earned'
  return slots
})

// Click a fear dot: promote unseen→earned; earned→resolved; resolved→unseen
function cycleFearSlot(slot: FearSlot) {
  const fd = props.modelValue.fear_deck
  if (!fd) return
  const round = props.modelValue.round
  const next = { ...fd, earned: [...fd.earned], resolved: [...fd.resolved] }
  if (slot.state === 'unseen') {
    next.earned.push({ name: '', terror_level: props.modelValue.pools.terror_level, round })
    emit('log-event', 'fear_card_earned', { terror_level: props.modelValue.pools.terror_level, round, manual: true })
  } else if (slot.state === 'earned') {
    const card = next.earned.pop()
    if (card) next.resolved.push({ ...card, round })
    emit('log-event', 'fear_card_resolved', { round })
  } else {
    // resolved → back to unseen (undo)
    next.resolved.pop()
  }
  next.unseen = Math.max(0, next.deck_size - next.resolved.length - next.earned.length)
  emit('update:modelValue', { ...props.modelValue, fear_deck: next })
}

function bumpBlight(delta: number) { emit('bump-pool', 'blight', delta) }
function bumpFear(delta: number) {
  const pools = { ...props.modelValue.pools, fear_current: Math.max(0, props.modelValue.pools.fear_current + delta) }
  emit('update:modelValue', { ...props.modelValue, pools })
}

// Compact unit-totals summary per land (for hover)
function landSummary(landId: string): string {
  const land = activeBoard.value?.lands?.[landId]
  if (!land) return landId
  const parts: string[] = []
  if (land.terrain) parts.push(land.terrain)
  if (land.explorers) parts.push(`${land.explorers} exp`)
  if (land.towns) parts.push(`${land.towns} town`)
  if (land.cities) parts.push(`${land.cities} city`)
  if (land.dahan) parts.push(`${land.dahan} dahan`)
  if (land.blight) parts.push(`${land.blight} blight`)
  return parts.join(' · ')
}

</script>

<template>
  <div class="shell">
    <SidebarNav
      :state="modelValue"
      :active-section="activeSection"
      @select-section="selectSection"
      @new-game="emit('new-game')"
      @saved-games="emit('saved-games')"
      @set-phase="(p: Phase) => { if (!gameOver) emit('update:modelValue', { ...modelValue, phase: p }) }"
      @archive="emit('archive')"
      @export="emit('export')"
      @toggle-ops="opsOpen = !opsOpen"
    />

    <header class="shell-header">
      <div class="matchup">
        <span class="matchup-dot"></span>
        <span class="mono">{{
          (modelValue.setup.adversary ?? 'solo').replace(/-/g, ' ')
        }}{{ modelValue.setup.level != null ? ' · L' + modelValue.setup.level : '' }}</span>
      </div>

      <div></div>

      <div class="header-actions">
        <div class="status-inline" :class="`state-${winState}`">
          <div class="stat-mini">
            <span class="stat-mini-label">R</span>
            <span class="stat-mini-val">{{ modelValue.round }}</span>
          </div>
          <div class="stat-mini fear">
            <span class="stat-mini-label">FEAR</span>
            <span class="stat-mini-val mono">{{ modelValue.pools.fear_current }}/{{ modelValue.pools.fear_threshold }}</span>
            <span class="terror-pill" :data-tl="modelValue.pools.terror_level">T{{ modelValue.pools.terror_level }}</span>
          </div>
          <div class="stat-mini blight">
            <span class="stat-mini-label">BLT</span>
            <span class="stat-mini-val mono">{{ modelValue.pools.blight_current }}/{{ modelValue.pools.blight_cap }}</span>
            <UiIcon v-if="modelValue.pools.island_blighted" name="moon" :size="12" class="blighted-tag" label="Blighted island" />
          </div>
          <div class="winprob">
            <span class="winprob-value aegis-gradient">{{ winPct }}</span>
            <span v-if="winState === 'won'" class="aegis-badge ok"><UiIcon name="trophy" :size="12" decorative /> WIN</span>
            <span v-else-if="winState === 'lost'" class="aegis-badge danger"><UiIcon name="skull" :size="12" decorative /> LOSS</span>
            <span v-else-if="winState === 'imminent'" class="aegis-badge imminent"><UiIcon name="target" :size="12" decorative /> CLOSE</span>
          </div>
        </div>
        <button
          class="primary advance-btn"
          @click="advancePhase"
          :disabled="gameOver"
          :title="gameOver ? `Game ended (${winState}) — phase locked` : `Advance from ${modelValue.phase}`"
        >
          <template v-if="gameOver && winState === 'won'"><UiIcon name="trophy" :size="14" decorative /> WON</template>
          <template v-else-if="gameOver && winState === 'lost'"><UiIcon name="skull" :size="14" decorative /> LOST</template>
          <template v-else>{{ nextPhaseLabel }}</template>
        </button>
      </div>
    </header>

    <main class="shell-main aegis-grid-bg">
      <!-- Optional ops drawer: log + pool editors. Hidden by default. -->
      <transition name="drawer">
        <div v-if="opsOpen" class="ops-drawer aegis-card">
          <div class="drawer-col">
            <div class="drawer-col-title">Turn Log</div>
            <TurnController v-model="(modelValue as any)" />
          </div>
          <div class="drawer-col">
            <div class="drawer-col-title">Pools &amp; Blight Card</div>
            <Pools
              v-model="modelValue.pools"
              :player-count="Object.keys(modelValue.spirits ?? {}).length"
            />
          </div>
        </div>
      </transition>

      <div class="map-only-grid">

        <!-- BOARD zone — PNG backdrop + overlay land markers; click opens detail -->
        <section id="zone-board" class="dash-zone zone-board aegis-card">
          <div class="aegis-card-head">
            <span class="aegis-card-title">Board</span>
            <div v-if="boardIds.length > 1" class="board-tabs">
              <button
                v-for="bid in boardIds"
                :key="bid"
                type="button"
                class="board-tab"
                :class="{ active: activeBoardId === bid }"
                @click="activeBoardId = bid"
              >{{ bid }}</button>
            </div>
            <span v-else class="aegis-card-meta">{{ activeBoardId }} · {{ boardVariant }}</span>
            <span class="aegis-card-meta" style="margin-left:auto;">
              <div class="board-view-toggle">
                <button
                  class="bv-btn"
                  :class="{ active: boardView === 'map' }"
                  @click="boardView = 'map'"
                  title="Positioned PNG map view"
                >Map</button>
                <button
                  class="bv-btn"
                  :class="{ active: boardView === 'grid' }"
                  @click="boardView = 'grid'"
                  title="Detailed land grid editor"
                >Grid</button>
              </div>
            </span>
          </div>
          <div class="card-body-fill pad-none board-canvas">
            <!-- GRID VIEW: full Board.vue fills the same space as the map -->
            <Board
              v-if="boardView === 'grid' && activeBoardId && modelValue.board_state[activeBoardId]"
              v-model="modelValue.board_state[activeBoardId]"
              :board-id="activeBoardId"
              :spirits="modelValue.spirits"
              @bump-pool="(pool, delta) => emit('bump-pool', pool, delta)"
            />

            <!-- MAP VIEW: PNG backdrop + overlay land tiles (default) -->
            <div v-else-if="activeBoardId" class="map-wrap">
              <!-- Image + markers share one container so markers position
                   relative to the rendered image, not the wrap. Click outside
                   any marker closes the land popover. -->
              <div class="board-stage" @click="selectedLandId = null">
                <img class="board-bg" :src="boardBgUrl" :alt="`Board ${activeBoardId}`" />
                <button
                  v-for="(_land, landId) in (activeBoard?.lands ?? {})"
                  :key="landId"
                  type="button"
                  class="land-tile"
                  :class="[terrainClass(_land.terrain), { active: selectedLandId === landId, coastal: _land.coastal }]"
                  :style="{ left: coordFor(landId).x + '%', top: coordFor(landId).y + '%' }"
                  :title="landSummary(landId)"
                  @click.stop="selectLand(landId)"
                >
                  <div class="tile-head">
                    <span class="tile-num">{{ landId }}</span>
                    <span v-if="_land.coastal" class="tile-coast" title="Coastal">~</span>
                    <span v-if="_land.blight" class="tile-blight" :title="`${_land.blight} blight`">⬤{{ _land.blight }}</span>
                  </div>

                  <div v-if="(_land.explorers || _land.towns || _land.cities || _land.dahan)" class="tile-units">
                    <span v-if="_land.explorers" class="unit u-exp" title="Explorers">
                      <svg class="u-ic" viewBox="0 0 20 20"><circle cx="10" cy="10" r="6" fill="currentColor"/></svg>{{ _land.explorers }}
                    </span>
                    <span v-if="_land.towns" class="unit u-town" title="Towns">
                      <svg class="u-ic" viewBox="0 0 20 20"><rect x="3" y="5" width="14" height="12" fill="currentColor"/></svg>{{ _land.towns }}
                    </span>
                    <span v-if="_land.cities" class="unit u-city" title="Cities">
                      <svg class="u-ic" viewBox="0 0 20 20"><polygon points="10,2 17,18 3,18" fill="currentColor"/></svg>{{ _land.cities }}
                    </span>
                    <span v-if="_land.dahan" class="unit u-dahan" title="Dahan">
                      <svg class="u-ic" viewBox="0 0 20 20"><path d="M5 5 L15 5 L10 15 Z" fill="currentColor"/></svg>{{ _land.dahan }}
                    </span>
                  </div>

                  <div v-if="presenceOn(landId).length" class="tile-presence">
                    <span
                      v-for="info in presenceOn(landId)"
                      :key="info.slug"
                      class="presence-disc"
                      :style="{ background: info.color }"
                      :title="`${info.slug}: ${info.count}`"
                    >{{ info.count > 1 ? info.count : '●' }}</span>
                  </div>

                  <div v-if="(_land.tokens?.length ?? 0) > 0" class="tile-tokens">
                    <span
                      v-for="(tok, i) in Array.from(new Set(_land.tokens ?? []))"
                      :key="i"
                      class="tok-chip"
                      :class="'tok-' + tok.toLowerCase().replace(/\s+/g, '-')"
                      :title="tok"
                    >{{ tok.charAt(0).toUpperCase() }}{{ (_land.tokens ?? []).filter((t: string) => t === tok).length > 1 ? (_land.tokens ?? []).filter((t: string) => t === tok).length : '' }}</span>
                  </div>
                </button>

                <!-- Land popover — anchored to the clicked marker -->
                <div
                  v-if="selectedLandId && activeBoard?.lands?.[selectedLandId]"
                  class="land-popover"
                  :style="{
                    left: coordFor(selectedLandId).x + '%',
                    top: coordFor(selectedLandId).y + '%',
                  }"
                  @click.stop
                >
                  <div class="popover-head">
                    <span class="popover-title">Land {{ selectedLandId }}</span>
                    <span class="popover-terrain">{{ activeBoard.lands[selectedLandId].terrain }}</span>
                    <button class="icon-btn" @click="selectedLandId = null" title="Close">×</button>
                  </div>
                  <div class="popover-body">
                    <LandEditor
                      v-model="activeBoard.lands[selectedLandId]"
                      :land-id="selectedLandId"
                      :board-id="activeBoardId"
                      :spirits="modelValue.spirits"
                      @bump-pool="(pool, delta) => emit('bump-pool', pool, delta)"
                    />
                  </div>
                </div>
              </div>
            </div>
            <div v-else class="empty-state">No board configured</div>
          </div>
        </section>

      </div>
    </main>

    <!-- ────────── Right panel: toggleable Spirits ↔ Decks ────────── -->
    <aside class="right-panel">
      <div class="right-panel-tabs">
        <button
          type="button"
          :class="{ active: rightPanelMode === 'spirits' }"
          @click="rightPanelMode = 'spirits'"
        ><UiIcon name="user" :size="14" decorative /> Spirits</button>
        <button
          type="button"
          :class="{ active: rightPanelMode === 'decks' }"
          @click="rightPanelMode = 'decks'"
        ><UiIcon name="scroll" :size="14" decorative /> Decks</button>
      </div>

      <!-- Spirits view — full SpiritPanel so all interactions (cards, presence,
           elements, energy, card-plays, growth, played) stay wired.
           Tabs are always visible (even for 1 spirit) so the active spirit's
           identity + any selected aspect are never ambiguous. -->
      <div v-if="rightPanelMode === 'spirits'" class="rp-body">
        <div v-if="spiritSlugs.length" class="spirit-tabs-row">
          <button
            v-for="slug in spiritSlugs"
            :key="slug"
            type="button"
            class="spirit-tab"
            :class="{ active: activeSpiritSlug === slug, solo: spiritSlugs.length === 1 }"
            @click="activeSpiritSlug = slug"
            :title="slug + (modelValue.spirits[slug]?.aspect_name ? ` — ${modelValue.spirits[slug].aspect_name}` : '')"
          >
            <span class="spirit-tab-name">{{ slug.split('-')[0].charAt(0).toUpperCase() + slug.split('-')[0].slice(1) }}</span>
            <span v-if="modelValue.spirits[slug]?.aspect_name" class="spirit-tab-aspect">{{ modelValue.spirits[slug].aspect_name }}</span>
          </button>
        </div>

        <SpiritPanel
          v-if="activeSpiritSlug && modelValue.spirits[activeSpiritSlug]"
          v-model="modelValue.spirits[activeSpiritSlug]"
          :slug="activeSpiritSlug"
          :round="modelValue.round"
          @log-event="(event, details) => emit('log-event', event, details)"
        />
        <div v-else class="sc-empty" style="padding:24px">Start a new game to see your spirit here.</div>
      </div>

      <!-- Decks view — styled compact panels from the Aegis mockup. Every
           element is directly clickable; the "Detail ↗" chip on each header
           opens the full component in a drawer for heavy edits. -->
      <div v-else class="rp-body decks-stack">

        <!-- Event deck -->
        <div id="sec-events" class="deck-block" :class="{ primary: modelValue.phase === 'event' }">
          <div class="deck-block-head">
            <span class="deck-block-title"><UiIcon name="scroll" :size="14" decorative /> Event</span>
            <span v-if="modelValue.phase === 'event'" class="aegis-badge info">ACTIVE</span>
            <span class="deck-block-meta">{{ modelValue.event_deck?.resolved.length ?? 0 }} resolved</span>
            <button class="pop-btn micro" @click="detailDrawer = 'event'" title="Open full Event deck">↗</button>
          </div>
          <div class="deck-block-body">
            <div class="event-preview-slot">
              <span class="slot-label">Previewed · resolves next turn</span>
              <input
                type="text"
                class="slot-input"
                :value="modelValue.event_deck?.previewed[0]?.name ?? ''"
                placeholder="(face-down)"
                @change="(e) => {
                  const deck = modelValue.event_deck
                  if (!deck) return
                  const val = (e.target as HTMLInputElement).value
                  const previewed = deck.previewed.length
                    ? [{ ...deck.previewed[0], name: val }]
                    : [{ name: val, previewed_on_turn: modelValue.round }]
                  emit('update:modelValue', { ...modelValue, event_deck: { ...deck, previewed } })
                }"
              />
            </div>
            <div v-if="modelValue.event_deck?.resolved.length" class="event-resolved-hint">
              Last resolved: <strong>{{ modelValue.event_deck.resolved[modelValue.event_deck.resolved.length - 1]?.name || '—' }}</strong>
            </div>
          </div>
        </div>

        <!-- Fear deck -->
        <div id="sec-fear" class="deck-block" :class="{ primary: modelValue.phase === 'fear' }">
          <div class="deck-block-head">
            <span class="deck-block-title"><UiIcon name="ghost" :size="14" decorative /> Fear</span>
            <span v-if="modelValue.phase === 'fear'" class="aegis-badge info">ACTIVE</span>
            <span class="deck-block-meta">
              {{ modelValue.fear_deck?.resolved.length ?? 0 }}r · {{ modelValue.fear_deck?.earned.length ?? 0 }}e / {{ modelValue.fear_deck?.deck_size ?? 9 }}
            </span>
            <button class="pop-btn micro" @click="detailDrawer = 'fear'" title="Open full Fear deck">↗</button>
          </div>
          <div class="deck-block-body">
            <div class="fear-slots">
              <button
                v-for="s in fearSlots"
                :key="s.id"
                type="button"
                class="fear-dot"
                :class="[`t${s.tier}`, s.state]"
                :title="`Tier ${s.tier} · ${s.state} — click to cycle`"
                @click="cycleFearSlot(s)"
              >
                <UiIcon v-if="s.state === 'resolved'" name="check" :size="10" decorative />
                <span v-else-if="s.state === 'earned'" class="fear-earned-dot"></span>
              </button>
            </div>
            <div class="fear-controls">
              <div class="pool-bumper">
                <span class="pb-label">Pool</span>
                <button class="pb-btn" @click="bumpFear(-1)" :disabled="modelValue.pools.fear_current <= 0">−</button>
                <span class="pb-val mono">{{ modelValue.pools.fear_current }}/{{ modelValue.pools.fear_threshold }}</span>
                <button class="pb-btn" @click="bumpFear(1)">+</button>
              </div>
              <span class="terror-pill" :data-tl="modelValue.pools.terror_level">T{{ modelValue.pools.terror_level }}</span>
            </div>
          </div>
        </div>

        <!-- Invader deck -->
        <div id="sec-invader" class="deck-block" :class="{ primary: modelValue.phase === 'invader' }">
          <div class="deck-block-head">
            <span class="deck-block-title"><UiIcon name="swords" :size="14" decorative /> Invader</span>
            <span v-if="modelValue.phase === 'invader'" class="aegis-badge info">ACTIVE</span>
            <span class="deck-block-meta">{{ modelValue.invader_deck?.notation ?? '' }}</span>
            <button class="pop-btn micro" @click="detailDrawer = 'invader'" title="Open full Invader deck">↗</button>
          </div>
          <div class="deck-block-body">
            <div class="slot-row">
              <div class="slot ravage">
                <span class="slot-label">Ravage</span>
                <div class="slot-controls">
                  <button class="slot-btn stage" @click="cycleStage('ravage')" :title="`Stage ${stageLabel(modelValue.invader_deck?.ravage?.stage)} — click to cycle`">
                    {{ stageLabel(modelValue.invader_deck?.ravage?.stage) }}
                  </button>
                  <button class="slot-btn terrain" @click="cycleTerrain('ravage')" :title="`${modelValue.invader_deck?.ravage?.terrain ?? 'set'} — click to cycle`">
                    {{ modelValue.invader_deck?.ravage?.terrain ?? '—' }}
                  </button>
                </div>
              </div>
              <div class="slot build">
                <span class="slot-label">Build</span>
                <div class="slot-controls">
                  <button class="slot-btn stage" @click="cycleStage('build')">
                    {{ stageLabel(modelValue.invader_deck?.build?.stage) }}
                  </button>
                  <button class="slot-btn terrain" @click="cycleTerrain('build')">
                    {{ modelValue.invader_deck?.build?.terrain ?? '—' }}
                  </button>
                </div>
              </div>
              <div class="slot explore">
                <span class="slot-label">Explore</span>
                <div class="slot-controls">
                  <button class="slot-btn stage" @click="cycleStage('explore')">
                    {{ stageLabel(modelValue.invader_deck?.explore?.stage) }}
                  </button>
                  <button class="slot-btn terrain" @click="cycleTerrain('explore')">
                    {{ modelValue.invader_deck?.explore?.terrain ?? '—' }}
                  </button>
                </div>
              </div>
            </div>
            <div v-if="(modelValue.invader_deck?.upcoming ?? []).length" class="upcoming-row">
              <span class="upcoming-label">Upcoming:</span>
              <span
                v-for="(c, i) in modelValue.invader_deck?.upcoming ?? []"
                :key="i"
                class="stage-chip"
                :class="`s${c.stage}`"
                :title="`Stage ${stageLabel(c.stage)}`"
              >{{ stageLabel(c.stage) }}</span>
            </div>
            <div class="pool-bumper blight-bumper">
              <span class="pb-label">Blight</span>
              <button class="pb-btn" @click="bumpBlight(-1)" :disabled="modelValue.pools.blight_current <= 0">−</button>
              <span class="pb-val mono">{{ modelValue.pools.blight_current }}/{{ modelValue.pools.blight_cap }}</span>
              <button class="pb-btn" @click="bumpBlight(1)">+</button>
              <span v-if="modelValue.pools.island_blighted" class="blighted-tag"><UiIcon name="moon" :size="11" decorative /> flipped</span>
            </div>
          </div>
        </div>

      </div>
    </aside>

    <!-- ────────── Analytics rail — far right, stats only ────────── -->
    <aside class="analytics-rail aegis-card">
      <div class="aegis-card-head">
        <span class="aegis-card-title">Analytics</span>
        <span v-if="winState !== 'in-progress'" class="aegis-badge" :class="winState === 'won' ? 'ok' : winState === 'lost' ? 'danger' : 'imminent'">
          {{ winState.toUpperCase() }}
        </span>
      </div>
      <div class="ar-actions">
        <button class="pop-btn" title="Terrain Timeline" @click="drawerOpen = drawerOpen === 'terrain' ? null : 'terrain'"><UiIcon name="chart" :size="14" decorative /> Terrain</button>
        <button class="pop-btn" title="Retrospective" @click="drawerOpen = drawerOpen === 'retro' ? null : 'retro'"><UiIcon name="search" :size="14" decorative /> Retro</button>
      </div>
      <div id="sec-stats" class="card-body-fill">
        <StatsPanel :state="modelValue" />
      </div>
    </aside>

    <!-- Click-away backdrop for drawers — closes on outside click -->
    <transition name="fade">
      <div
        v-if="drawerOpen || detailDrawer"
        class="drawer-backdrop"
        @click="drawerOpen = null; detailDrawer = null"
      ></div>
    </transition>

    <!-- Analysis drawers (Terrain / Retro) — slide in from bottom over the main area -->
    <transition name="popdrawer">
      <div v-if="drawerOpen" class="pop-drawer aegis-card">
        <div class="aegis-card-head">
          <span class="aegis-card-title">
            <template v-if="drawerOpen === 'terrain'"><UiIcon name="chart" :size="14" decorative /> Terrain Timeline</template>
            <template v-else><UiIcon name="search" :size="14" decorative /> Retrospective</template>
          </span>
          <button class="icon-btn" style="margin-left:auto;" @click="drawerOpen = null" title="Close">×</button>
        </div>
        <div class="pop-drawer-body">
          <TerrainTimeline v-if="drawerOpen === 'terrain'" :state="modelValue" />
          <Retrospective  v-else-if="drawerOpen === 'retro'" :state="modelValue" />
        </div>
      </div>
    </transition>

    <!-- Zone detail drawers — full underlying Vue component for detailed edits.
         Overlays the main area; spirit rail + sidebar stay visible. -->
    <transition name="popdrawer">
      <div v-if="detailDrawer" class="pop-drawer aegis-card detail-drawer">
        <div class="aegis-card-head">
          <span class="aegis-card-title">
            <template v-if="detailDrawer === 'invader'"><UiIcon name="swords" :size="14" decorative /> Invader Deck — full</template>
            <template v-else-if="detailDrawer === 'fear'"><UiIcon name="ghost" :size="14" decorative /> Fear Deck — full</template>
            <template v-else-if="detailDrawer === 'event'"><UiIcon name="scroll" :size="14" decorative /> Event Deck — full</template>
            <template v-else>Spirit — full panel</template>
          </span>
          <button class="icon-btn" style="margin-left:auto;" @click="detailDrawer = null" title="Close">×</button>
        </div>
        <div class="pop-drawer-body">
          <InvaderDeck
            v-if="detailDrawer === 'invader'"
            v-model="modelValue.invader_deck"
            :adversary="modelValue.setup.adversary"
            :level="modelValue.setup.level"
          />
          <FearDeck
            v-else-if="detailDrawer === 'fear'"
            v-model="modelValue.fear_deck"
            :round="modelValue.round"
            :terror-level="modelValue.pools.terror_level"
            :fear-threshold="modelValue.pools.fear_threshold"
            @log-event="(event, details) => emit('log-event', event, details)"
            @reset-fear-pool="emit('reset-fear-pool')"
          />
          <EventDeck
            v-else-if="detailDrawer === 'event'"
            v-model="modelValue.event_deck"
            :round="modelValue.round"
          />
        </div>
      </div>
    </transition>
  </div>
</template>

<style scoped>
/* ──────────────────────── Four-column shell ────────────────────────
 *   [ nav 240 | map (flex) | right-panel 400 | analytics 240 ]
 * The right-panel hosts EITHER the spirits view OR the decks view (user
 * toggles). Analytics is a slim rail on the far right.
 */
.shell {
  display: grid;
  grid-template-columns: 260px minmax(0, 1fr) 520px 340px;
  grid-template-rows: 56px minmax(0, 1fr);
  grid-template-areas:
    "nav header header header"
    "nav main   right  analytics";
  height: 100vh;
  min-height: 0;
  overflow: hidden;
}

.shell-header {
  grid-area: header;
  border-bottom: 1px solid var(--border-subtle);
  background: color-mix(in srgb, var(--bg-surface) 92%, transparent);
  backdrop-filter: blur(8px);
  display: grid;
  grid-template-columns: auto 1fr auto;
  align-items: center;
  gap: 14px;
  padding: 0 20px;
  min-height: 0;
}

.shell-main {
  grid-area: main;
  padding: 16px 20px 20px;
  display: flex;
  flex-direction: column;
  gap: 14px;
  min-height: 0;
  overflow: hidden;
}

/* ──────────────────────── Header ──────────────────────── */
.matchup {
  display: inline-flex; align-items: center; gap: 8px;
  padding: 5px 10px;
  border-radius: var(--r-full);
  background: color-mix(in srgb, var(--accent-purple) 16%, transparent);
  border: 1px solid color-mix(in srgb, var(--accent-purple) 32%, transparent);
  font-size: 11px;
  color: #ddd4f9;
  text-transform: capitalize;
  white-space: nowrap;
}
.matchup-dot { width: 6px; height: 6px; border-radius: 50%; background: var(--accent-purple); box-shadow: 0 0 8px var(--accent-purple); }


/* Inline phase stepper — mirrors the mockup. No focus-hint line, no extra chrome. */
.phase-stepper {
  display: inline-flex;
  align-items: center;
  min-width: 0;
  gap: 0;
  overflow-x: auto;
  justify-content: center;
}
.phase-node {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 4px 10px;
  border-radius: var(--r-full);
  color: var(--text-muted);
  border: 1px solid transparent;
  background: transparent;
  cursor: pointer;
  font-size: 11px;
  font-weight: var(--fw-medium);
  white-space: nowrap;
  transition: all 150ms ease;
}
.phase-node:hover { color: var(--text-primary); background: color-mix(in srgb, var(--accent-blue) 8%, transparent); }
.phase-node .pnum { font-size: 10px; color: var(--text-muted); }
.phase-node.done { color: var(--text-secondary); opacity: 0.72; }
.phase-node.done .pnum { color: var(--accent-green); }
.phase-node.active {
  background: var(--accent-grad);
  color: white;
  border-color: transparent;
  box-shadow: var(--shadow-glow);
  opacity: 1;
}
.phase-node.active .pnum { color: rgba(255, 255, 255, 0.78); }
.phase-sep {
  display: inline-block;
  width: 8px; height: 1px;
  background: var(--border-subtle);
}
.phase-sep.done { background: color-mix(in srgb, var(--accent-green) 45%, transparent); }

.header-actions { display: inline-flex; align-items: center; gap: 8px; }

.status-inline {
  display: inline-flex; align-items: center; gap: 12px;
  padding: 4px 12px;
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
}
.stat-mini { display: inline-flex; align-items: baseline; gap: 4px; }
.stat-mini-label { font-size: 9px; text-transform: uppercase; letter-spacing: .1em; color: var(--text-muted); font-weight: 600; }
.stat-mini-val { font-size: 13px; font-weight: var(--fw-bold); color: var(--text-primary); }
.stat-mini.fear .stat-mini-val { color: var(--pool-fear); }
.stat-mini.blight .stat-mini-val { color: var(--pool-blight); }

.terror-pill {
  font-family: var(--font-mono);
  font-size: 10px; font-weight: 700;
  padding: 1px 6px; border-radius: var(--r-sm);
  background: var(--bg-canvas);
  margin-left: 2px;
}
.terror-pill[data-tl="1"] { color: var(--accent-blue); }
.terror-pill[data-tl="2"] { color: var(--accent-amber); }
.terror-pill[data-tl="3"] { color: var(--accent-red); }
.blighted-tag { font-size: 12px; margin-left: 2px; }

.winprob { display: inline-flex; align-items: center; gap: 6px; padding-left: 10px; border-left: 1px solid var(--border-subtle); }
.winprob-value { font-size: 16px; font-weight: var(--fw-bold); font-family: var(--font-mono); }

.advance-btn {
  padding: 8px 14px;
  font-size: 12px;
  font-weight: var(--fw-bold);
  text-transform: uppercase;
  letter-spacing: 0.04em;
  white-space: nowrap;
}
.advance-btn:disabled {
  background: var(--bg-inset);
  color: var(--text-muted);
  box-shadow: none;
  cursor: not-allowed;
  border: 1px solid var(--border-subtle);
  opacity: 0.85;
}

.icon-btn {
  width: 32px; height: 32px;
  border-radius: var(--r-sm);
  display: inline-grid; place-items: center;
  background: transparent; border: 1px solid transparent;
  color: var(--text-secondary); cursor: pointer;
  transition: all 150ms ease;
  padding: 0; font-size: 14px;
}
.icon-btn:hover, .icon-btn.active {
  background: var(--bg-elevated);
  color: var(--text-primary);
  border-color: var(--border-subtle);
}
.icon-btn.active {
  background: var(--accent-soft);
  color: var(--accent-blue);
  border-color: var(--accent-border);
}

/* ──────────────────────── Ops drawer ──────────────────────── */
.ops-drawer {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 14px;
  padding: 14px;
  max-height: 45vh;
  overflow: auto;
  flex: 0 0 auto;
}
.drawer-col { min-width: 0; }
.drawer-col-title {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--text-muted);
  font-weight: var(--fw-semibold);
  margin-bottom: 6px;
}
.drawer-enter-active, .drawer-leave-active {
  transition: max-height var(--motion-base), opacity var(--motion-base);
}
.drawer-enter-from, .drawer-leave-to { max-height: 0; opacity: 0; }

/* ──────────────────────── Dashboard grid ──────────────────────── */
.dash-grid {
  flex: 1 1 auto;
  min-height: 0;
  display: grid;
  gap: 12px;
  /* Default — same as Fast/Slow: board big + decks/analytics stacked right */
  grid-template-columns: minmax(0, 3fr) minmax(0, 2fr);
  grid-template-rows: minmax(0, 3fr) minmax(0, 2fr);
  grid-template-areas:
    "board    decks"
    "board    analytics";
}
.zone-board     { grid-area: board; min-width: 0; min-height: 0; }
.zone-decks     { grid-area: decks; min-width: 0; min-height: 0; }
.zone-analytics { grid-area: analytics; min-width: 0; min-height: 0; }

/* All aegis-cards in the grid get flex-column so their body fills remaining */
.dash-grid .aegis-card {
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.card-body-fill {
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
  padding: 14px 16px;
}
.card-body-fill.pad-none { padding: 0; }

/* Map/Grid view toggle in the board card-head — not a drawer, just swaps
 * what renders inside .board-canvas. */
.board-view-toggle {
  display: inline-flex;
  gap: 2px;
  padding: 2px;
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
}
.bv-btn {
  padding: 3px 12px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 11px;
  font-weight: var(--fw-semibold);
  cursor: pointer;
  border-radius: calc(var(--r-sm) - 2px);
  transition: all 150ms ease;
}
.bv-btn:hover { color: var(--text-primary); }
.bv-btn.active {
  background: var(--accent-grad);
  color: white;
  box-shadow: var(--shadow-glow);
}

/* Board tabs in board-zone card-head */
.board-tabs { display: inline-flex; gap: 2px; margin-left: 8px; }
.board-tab {
  min-width: 28px; height: 22px; padding: 0 8px;
  border-radius: var(--r-sm);
  border: 1px solid var(--border-subtle);
  background: var(--bg-inset);
  color: var(--text-secondary);
  font-size: 11px; font-weight: var(--fw-semibold);
  cursor: pointer;
  transition: all 150ms ease;
}
.board-tab.active { background: var(--accent-grad); color: white; border-color: transparent; box-shadow: var(--shadow-glow); }

/* Decks wrap — 3 sub-stations stacked vertically with compact headers */
.decks-wrap {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 14px;
}
.deck-station {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  display: flex; flex-direction: column;
  overflow: hidden;
  min-height: 0;
  flex: 1 1 0;
}
.deck-station.primary {
  border-color: var(--accent-blue);
  box-shadow: 0 0 0 1px rgba(37, 99, 235, 0.22);
}
.deck-head {
  padding: 10px 14px;
  border-bottom: 1px solid var(--border-subtle);
  display: flex; align-items: center; gap: 10px;
  flex: 0 0 auto;
  font-size: 11px;
}
.deck-title { font-weight: var(--fw-bold); text-transform: uppercase; letter-spacing: 0.06em; color: var(--text-primary); }
.deck-count { margin-left: auto; font-size: 10px; color: var(--text-muted); }
.deck-body {
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
  padding: 12px 14px;
}

/* ────────────────── Board zone — PNG backdrop + markers ──────────────────
 * Three nested containers:
 *   .board-canvas → outer zone-fill
 *   .board-wrap   → centers the stage
 *   .board-stage  → inline-block sized to the image's rendered dimensions;
 *                   markers position absolutely relative to THIS, so their
 *                   %-coords align with the image (not the full card).
 */
/* Board container — plain block with scroll so either rendering mode can
 * claim the full width. The map view does its own image centering via
 * .map-wrap; the grid view (Board.vue's .board-wrap) stretches naturally. */
.board-canvas {
  width: 100%; height: 100%;
  min-height: 0;
  padding: 14px;
  overflow: auto;
}
/* When Board.vue is rendered inline for the grid view, force its own
 * .board-wrap root to full width so its grid lays out across the column. */
.board-canvas :deep(.board-wrap) { width: 100%; min-width: 0; }

.map-wrap {
  position: relative;
  width: 100%; height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: radial-gradient(ellipse at top, rgba(37, 99, 235, 0.06), transparent 60%);
}
.board-stage {
  position: relative;
  display: inline-block;
  max-width: 100%;
  max-height: 100%;
  line-height: 0;  /* collapse whitespace between img + markers */
}
.board-bg {
  display: block;
  max-width: 100%;
  max-height: 100%;
  width: auto; height: auto;
  object-fit: contain;
  filter: drop-shadow(0 8px 24px rgba(0, 0, 0, 0.5));
}
/* ───── Land tiles — each is a small card centered on its land position with
 *       number, unit icons, presence discs, blight, tokens.
 */
.land-tile {
  position: absolute;
  transform: translate(-50%, -50%);
  min-width: 68px; max-width: 120px;
  padding: 4px 6px;
  border-radius: var(--r-sm);
  display: flex; flex-direction: column; gap: 3px;
  background: color-mix(in srgb, var(--bg-canvas) 78%, transparent);
  border: 2px solid var(--accent-blue);
  color: var(--text-primary);
  font-family: var(--font-sans);
  font-size: 11px;
  box-shadow: var(--shadow-glow), 0 2px 6px rgba(0, 0, 0, 0.45);
  cursor: pointer;
  backdrop-filter: blur(4px);
  transition: transform 150ms ease, border-color 150ms ease, box-shadow 150ms ease;
  text-align: left;
  line-height: 1.2;
}
.land-tile:hover { transform: translate(-50%, -50%) scale(1.06); border-color: var(--accent-purple); box-shadow: var(--shadow-glow-violet); }
.land-tile.active {
  transform: translate(-50%, -50%) scale(1.1);
  border-color: var(--accent-purple);
  box-shadow: var(--shadow-glow-violet);
}
.land-tile.terrain-mountain { border-color: var(--terrain-mountain); }
.land-tile.terrain-jungle   { border-color: var(--terrain-jungle); }
.land-tile.terrain-sands    { border-color: var(--terrain-sands); }
.land-tile.terrain-wetland  { border-color: var(--terrain-wetland); }
.land-tile.coastal::after {
  content: "";
  position: absolute; inset: -3px;
  border-radius: var(--r-md);
  border: 1px dashed rgba(79, 123, 158, 0.55);
  pointer-events: none;
}

/* Row 1 — land number + coastal mark + blight badge */
.tile-head {
  display: flex; align-items: center; gap: 4px;
  font-weight: var(--fw-bold);
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--text-white);
  line-height: 1;
}
.tile-num { flex: 0 0 auto; }
.tile-coast { color: var(--terrain-ocean); font-size: 10px; }
.tile-blight {
  margin-left: auto;
  font-size: 9px;
  padding: 1px 4px;
  border-radius: var(--r-full);
  background: rgba(124, 58, 237, 0.22);
  color: var(--accent-purple);
  border: 1px solid rgba(124, 58, 237, 0.45);
}

/* Row 2 — unit counts. Icons are tiny inline SVGs so they render without
 * needing external assets. Colors cue which unit type it is.
 */
.tile-units { display: flex; flex-wrap: wrap; gap: 3px; }
.unit {
  display: inline-flex; align-items: center; gap: 2px;
  padding: 1px 4px;
  border-radius: var(--r-sm);
  background: var(--bg-inset);
  font-family: var(--font-mono);
  font-size: 10px;
  font-weight: 700;
}
.u-ic { width: 9px; height: 9px; flex: 0 0 auto; }
.u-exp   { color: #d4d4d8; }   /* light neutral */
.u-town  { color: #fbbf24; }   /* amber */
.u-city  { color: #ef4444; }   /* red */
.u-dahan { color: #10b981; }   /* green */

/* Row 3 — presence discs, one per spirit on this land */
.tile-presence { display: flex; flex-wrap: wrap; gap: 2px; }
.presence-disc {
  min-width: 14px; height: 14px;
  padding: 0 3px;
  border-radius: var(--r-full);
  display: grid; place-items: center;
  font-size: 9px;
  font-weight: 700;
  color: white;
  border: 1.5px solid var(--bg-canvas);
  box-shadow: 0 0 4px rgba(0, 0, 0, 0.5);
}

/* Row 4 — token chips (beasts/disease/wilds/etc.) */
.tile-tokens { display: flex; flex-wrap: wrap; gap: 2px; }
.tok-chip {
  min-width: 14px; height: 14px;
  padding: 0 3px;
  border-radius: var(--r-sm);
  display: grid; place-items: center;
  font-size: 9px;
  font-weight: 700;
  color: white;
  background: var(--text-muted);
  border: 1px solid var(--border-subtle);
}
.tok-beast    { background: #6b4423; }
.tok-disease  { background: #7c3aed; }
.tok-strife   { background: #dc2626; }
.tok-wilds    { background: #16a34a; }
.tok-badlands { background: #a16207; }
.tok-vitality { background: #10b981; }
.tok-defend   { background: #2563eb; }
.tok-isolate  { background: #a855f7; }

/* Popover for land edit — anchored below the marker, stays on top of board */
.land-popover {
  position: absolute;
  transform: translate(-50%, 20px);
  z-index: 10;
  width: 280px;
  max-height: 360px;
  overflow: auto;
  background: var(--bg-surface);
  border: 1px solid var(--accent-blue);
  border-radius: var(--r-md);
  box-shadow: 0 12px 24px rgba(0, 0, 0, 0.5), var(--shadow-glow);
  line-height: 1.4;
}
.popover-head {
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-subtle);
  display: flex; align-items: center; gap: 8px;
  background: linear-gradient(90deg, var(--accent-soft), transparent);
}
.popover-title { font-weight: var(--fw-bold); color: var(--text-white); font-family: var(--font-mono); }
.popover-terrain {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--accent-blue);
  padding: 2px 6px;
  background: var(--bg-inset);
  border-radius: var(--r-sm);
}
.popover-body { padding: 14px 16px; }

.empty-state {
  padding: 24px;
  text-align: center;
  color: var(--text-muted);
  font-style: italic;
}

/* ────────────────── Event / Fear / Invader compact bodies ────────────────── */
.event-preview-slot {
  padding: 6px 10px;
  background: var(--bg-canvas);
  border: 1px solid var(--border-subtle);
  border-left: 3px solid var(--accent-purple);
  border-radius: var(--r-sm);
  display: flex; flex-direction: column; gap: 2px;
}
.event-resolved-hint {
  margin-top: 6px;
  font-size: 11px;
  color: var(--text-secondary);
}

.slot-row { display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px; }
.slot {
  padding: 6px 8px;
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  background: var(--bg-canvas);
  display: flex; flex-direction: column; gap: 1px;
  min-height: 44px;
}
.slot.ravage { border-left: 3px solid var(--accent-red); }
.slot.build  { border-left: 3px solid var(--accent-amber); }
.slot.explore { border-left: 3px solid var(--accent-blue); }
.slot-label {
  font-size: 9px; text-transform: uppercase; letter-spacing: 0.06em;
  color: var(--text-muted); font-weight: 600;
}
.slot-name { font-size: 12px; color: var(--text-primary); }
.slot-stage { font-family: var(--font-mono); font-weight: 700; font-size: 12px; color: var(--text-primary); }
.slot-terrain { font-size: 10px; color: var(--text-secondary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }

.upcoming { display: flex; flex-wrap: wrap; gap: 3px; margin-top: 6px; }
.stage-chip {
  width: 22px; height: 22px;
  border-radius: 50%;
  display: grid; place-items: center;
  font-size: 10px; font-weight: 700;
  font-family: var(--font-mono);
}
.stage-chip.s1 { background: rgba(59, 130, 246, 0.25); color: #93c5fd; }
.stage-chip.s2 { background: rgba(245, 158, 11, 0.25); color: #fcd34d; }
.stage-chip.s3 { background: rgba(239, 68, 68, 0.25); color: #fca5a5; }

.fear-slots {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 4px;
}
.fear-dot {
  aspect-ratio: 1;
  border-radius: 50%;
  border: 1.5px solid var(--border-subtle);
  display: grid; place-items: center;
  font-size: 10px;
  font-weight: 700;
  font-family: var(--font-mono);
  color: var(--text-muted);
  background: var(--bg-canvas);
}
.fear-dot.t1 { border-color: rgba(59, 130, 246, 0.5); color: #93c5fd; }
.fear-dot.t2 { border-color: rgba(245, 158, 11, 0.5); color: #fcd34d; }
.fear-dot.t3 { border-color: rgba(239, 68, 68, 0.5); color: #fca5a5; }
.fear-dot.resolved { background: rgba(16, 185, 129, 0.15); border-color: var(--accent-green); color: var(--accent-green); }
.fear-dot.earned   { background: rgba(245, 158, 11, 0.25); border-color: var(--accent-amber); color: var(--accent-amber); }
.fear-earned-dot {
  display: block;
  width: 6px; height: 6px;
  border-radius: 50%;
  background: currentColor;
}
.fear-sub {
  margin-top: 6px;
  font-size: 11px;
  color: var(--text-secondary);
}
.fear-sub strong.t1 { color: var(--accent-blue); }
.fear-sub strong.t2 { color: var(--accent-amber); }
.fear-sub strong.t3 { color: var(--accent-red); }

.pop-btn.micro {
  padding: 2px 6px;
  font-size: 10px;
  margin-left: 0;
}

.detail-drawer {
  /* wider than analysis drawers — full component UIs live in here */
  max-height: 80vh;
}

/* Pop-out drawer for Terrain / Retrospective — overlays the bottom of main */
.pop-drawer {
  position: fixed;
  left: 248px;
  right: 360px;
  bottom: 0;
  max-height: 70vh;
  z-index: 30;
  box-shadow: 0 -12px 24px rgba(0, 0, 0, 0.4);
  border-radius: var(--r-lg) var(--r-lg) 0 0;
  border-bottom: 0;
  display: flex;
  flex-direction: column;
}
.pop-drawer-body {
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
  padding: 12px 16px;
}
.popdrawer-enter-active, .popdrawer-leave-active {
  transition: transform var(--motion-base), opacity var(--motion-base);
}
.popdrawer-enter-from, .popdrawer-leave-to { transform: translateY(100%); opacity: 0; }

/* Backdrop dim layer behind drawers — click closes */
.drawer-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(3, 7, 18, 0.45);
  backdrop-filter: blur(2px);
  z-index: 25;
  cursor: pointer;
}
.fade-enter-active, .fade-leave-active { transition: opacity var(--motion-base); }
.fade-enter-from, .fade-leave-to { opacity: 0; }

.pop-btn {
  display: inline-flex; align-items: center; gap: 5px;
  font-size: 11px;
  font-weight: var(--fw-medium);
  padding: 5px 10px;
  border-radius: var(--r-sm);
  background: color-mix(in srgb, var(--bg-raised) 50%, transparent);
  color: var(--text-secondary);
  border: 1px solid var(--aegis-border);
  cursor: pointer;
  margin-left: 4px;
  transition: all 150ms ease;
}
.pop-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
  border-color: var(--accent-blue);
}

/* Map-only-grid — board fills the whole main zone now */
.map-only-grid {
  flex: 1 1 auto;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
.dash-zone.zone-board { flex: 1 1 auto; min-height: 0; display: flex; flex-direction: column; overflow: hidden; }

/* ──────────────────────── Right panel — toggleable Spirits/Decks ──────────────────────── */
.right-panel {
  grid-area: right;
  border-left: 1px solid var(--border-subtle);
  background: var(--bg-surface);
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}
.right-panel-tabs {
  flex: 0 0 auto;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4px;
  padding: 10px 12px;
  border-bottom: 1px solid var(--border-subtle);
  background: color-mix(in srgb, var(--bg-inset) 60%, transparent);
}
.right-panel-tabs button {
  display: inline-flex; align-items: center; justify-content: center; gap: 6px;
  padding: 9px 0;
  border: 1px solid transparent;
  background: transparent;
  color: var(--text-secondary);
  font-size: 11px;
  font-weight: var(--fw-semibold);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  cursor: pointer;
  border-radius: var(--r-sm);
  transition: all 150ms ease;
}
.right-panel-tabs button:hover {
  color: var(--text-primary);
  background: color-mix(in srgb, var(--accent-blue) 10%, transparent);
}
.right-panel-tabs button.active {
  background: var(--accent-grad);
  color: white;
  border-color: transparent;
  box-shadow: var(--shadow-glow);
}
.rp-body {
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.rp-body:not(.decks-stack) > :deep(*) { flex: 1 1 auto; min-width: 0; }
.rp-body.decks-stack { padding: 12px; gap: 14px; }

.deck-block {
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  background: var(--bg-inset);
  overflow: hidden;
}
.deck-block.primary {
  border-color: var(--accent-blue);
  box-shadow: 0 0 0 1px rgba(37, 99, 235, 0.22);
}
.deck-block-head {
  padding: 6px 10px;
  display: flex; align-items: center; gap: 8px;
  border-bottom: 1px solid var(--border-subtle);
  background: color-mix(in srgb, var(--bg-surface) 40%, transparent);
}
.deck-block-title {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-weight: var(--fw-bold);
  color: var(--text-white);
}
.deck-block-meta {
  margin-left: auto;
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-muted);
}
.deck-block-body { padding: 10px 12px; display: flex; flex-direction: column; gap: 10px; }

/* Event preview */
.event-preview-slot {
  padding: 8px 10px;
  background: var(--bg-canvas);
  border: 1px solid var(--border-subtle);
  border-left: 3px solid var(--accent-purple);
  border-radius: var(--r-sm);
  display: flex; flex-direction: column; gap: 4px;
}
.slot-input {
  background: transparent;
  border: none;
  color: var(--text-white);
  font-size: 13px;
  font-weight: var(--fw-semibold);
  padding: 2px 0;
  width: 100%;
  outline: none;
}
.slot-input::placeholder { color: var(--text-muted); font-style: italic; font-weight: var(--fw-regular); }
.slot-input:focus { border-bottom: 1px solid var(--accent-blue); }
.event-resolved-hint { font-size: 11px; color: var(--text-secondary); }
.event-resolved-hint strong { color: var(--text-white); font-weight: var(--fw-semibold); }

/* Fear slots as clickable dots */
.fear-slots { display: grid; grid-template-columns: repeat(9, 1fr); gap: 4px; }
.fear-dot {
  aspect-ratio: 1;
  border-radius: 50%;
  border: 1.5px solid var(--border-subtle);
  display: grid; place-items: center;
  font-size: 10px;
  font-weight: 700;
  font-family: var(--font-mono);
  color: var(--text-muted);
  background: var(--bg-canvas);
  cursor: pointer;
  transition: all 150ms ease;
  padding: 0;
}
.fear-dot:hover { transform: scale(1.08); }
.fear-dot.t1 { border-color: rgba(59, 130, 246, 0.5); color: #93c5fd; }
.fear-dot.t2 { border-color: rgba(245, 158, 11, 0.5); color: #fcd34d; }
.fear-dot.t3 { border-color: rgba(239, 68, 68, 0.5); color: #fca5a5; }
.fear-dot.resolved { background: rgba(16, 185, 129, 0.2); border-color: var(--accent-green); color: var(--accent-green); }
.fear-dot.earned { background: rgba(245, 158, 11, 0.28); border-color: var(--accent-amber); color: var(--accent-amber); }

.fear-controls { display: flex; align-items: center; justify-content: space-between; gap: 8px; }

/* Invader slot row — stage + terrain as cycle buttons */
.slot-row { display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px; }
.slot {
  padding: 6px 8px;
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  background: var(--bg-canvas);
  display: flex; flex-direction: column; gap: 4px;
}
.slot.ravage  { border-left: 3px solid var(--accent-red); }
.slot.build   { border-left: 3px solid var(--accent-amber); }
.slot.explore { border-left: 3px solid var(--accent-blue); }
.slot-label {
  font-size: 9px; text-transform: uppercase; letter-spacing: 0.08em;
  color: var(--text-muted); font-weight: 600;
}
.slot-controls { display: flex; gap: 4px; }
.slot-btn {
  flex: 1 1 auto;
  padding: 4px 6px;
  border-radius: var(--r-sm);
  border: 1px solid var(--border-subtle);
  background: var(--bg-inset);
  color: var(--text-primary);
  font-family: var(--font-mono);
  font-size: 11px;
  font-weight: 700;
  cursor: pointer;
  transition: all 120ms ease;
  text-transform: capitalize;
}
.slot-btn:hover { border-color: var(--accent-blue); background: var(--bg-elevated); }
.slot-btn.stage { max-width: 38px; flex: 0 0 38px; text-align: center; }
.slot-btn.terrain { text-align: left; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

.upcoming-row { display: flex; align-items: center; gap: 6px; flex-wrap: wrap; }
.upcoming-label { font-size: 10px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.08em; }
.stage-chip {
  width: 22px; height: 22px;
  border-radius: 50%;
  display: grid; place-items: center;
  font-size: 10px; font-weight: 700;
  font-family: var(--font-mono);
}
.stage-chip.s1 { background: rgba(59, 130, 246, 0.25); color: #93c5fd; }
.stage-chip.s2 { background: rgba(245, 158, 11, 0.25); color: #fcd34d; }
.stage-chip.s3 { background: rgba(239, 68, 68, 0.25); color: #fca5a5; }

/* Pool bumpers — shared by fear and blight */
.pool-bumper {
  display: inline-flex; align-items: center; gap: 6px;
  padding: 4px 8px;
  background: var(--bg-canvas);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
}
.blight-bumper { align-self: flex-start; }
.pb-label {
  font-size: 9px; text-transform: uppercase; letter-spacing: 0.08em;
  color: var(--text-muted); font-weight: 600;
}
.pb-btn {
  width: 22px; height: 22px;
  border-radius: var(--r-sm);
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  color: var(--text-primary);
  font-size: 13px; font-weight: 700;
  cursor: pointer;
  display: grid; place-items: center;
  padding: 0;
}
.pb-btn:hover:not(:disabled) { border-color: var(--accent-blue); color: var(--accent-blue); }
.pb-btn:disabled { opacity: 0.3; cursor: not-allowed; }
.pb-val { font-size: 12px; font-weight: 700; color: var(--text-white); min-width: 44px; text-align: center; }

/* Spirit tabs row — always shown so the active spirit is unambiguous,
   even in a 1-spirit game. Styled as a true tab bar with a sub-header
   label, background container, and clear visual separation from the
   SpiritPanel content below. */
.spirit-tabs-row {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 10px;
  margin: 0 0 10px;
  background: color-mix(in srgb, var(--accent-blue) 5%, var(--bg-inset));
  border: 1px solid var(--aegis-border);
  border-radius: var(--r-md);
  flex-wrap: wrap;
}
.spirit-tabs-row::before {
  content: 'Spirit';
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  font-weight: var(--fw-bold);
  color: var(--text-muted);
  flex: 0 0 auto;
  padding-right: 8px;
  border-right: 1px solid var(--aegis-border);
  margin-right: 2px;
}
.spirit-tab {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: var(--r-sm);
  border: 1px solid var(--border-subtle);
  background: var(--bg-surface);
  color: var(--text-secondary);
  font-size: 12px;
  font-weight: var(--fw-semibold);
  cursor: pointer;
  transition: all 150ms ease;
}
.spirit-tab:hover:not(.active):not(.solo) {
  border-color: var(--accent-blue);
  color: var(--text-primary);
}
.spirit-tab-name { font-size: 12px; }
.spirit-tab-aspect {
  font-size: 9px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  padding: 2px 6px;
  border-radius: 3px;
  background: color-mix(in srgb, var(--accent-purple, #7c3aed) 22%, transparent);
  color: color-mix(in srgb, var(--accent-purple, #7c3aed) 75%, white);
  border: 1px solid color-mix(in srgb, var(--accent-purple, #7c3aed) 40%, transparent);
}
.spirit-tab.solo {
  /* 1-spirit: still clickable but we visually de-emphasize the clickability
     to make this feel like an identity badge rather than a switcher. */
  cursor: default;
  background: var(--accent-grad);
  color: white;
  border-color: transparent;
  box-shadow: var(--shadow-glow);
}
.spirit-tab.active:not(.solo) {
  background: var(--accent-grad);
  color: white;
  border-color: transparent;
  box-shadow: var(--shadow-glow);
}
.spirit-tab.active .spirit-tab-aspect,
.spirit-tab.solo .spirit-tab-aspect {
  background: rgba(255, 255, 255, 0.22);
  color: white;
  border-color: rgba(255, 255, 255, 0.3);
}

/* ──────────────────────── Compact spirit view ──────────────────────── */
.spirit-compact { display: flex; flex-direction: column; gap: 10px; min-width: 0; }
.sc-ident { display: flex; align-items: baseline; gap: 8px; min-width: 0; }
.sc-name {
  font-family: var(--font-display, 'Oswald'), var(--font-sans);
  font-size: 20px;
  font-weight: 700;
  letter-spacing: 0.04em;
  color: var(--text-white);
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.sc-sub {
  font-size: 10px; color: var(--text-muted);
  text-transform: uppercase; letter-spacing: 0.1em;
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}

.sc-resources {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 6px;
}
.sc-res {
  padding: 6px 4px;
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  text-align: center;
}
.sc-res-label {
  font-size: 9px; text-transform: uppercase; letter-spacing: 0.08em;
  color: var(--text-muted); font-weight: 600;
  margin-bottom: 2px;
}
.sc-res-val {
  font-family: var(--font-mono);
  font-size: 18px;
  font-weight: 700;
  color: var(--text-white);
  line-height: 1;
}

.sc-section-title {
  font-size: 10px; text-transform: uppercase; letter-spacing: 0.1em;
  color: var(--text-muted); font-weight: var(--fw-semibold);
  margin-top: 4px;
}

.sc-elements {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 4px;
}
.sc-el {
  padding: 4px;
  border: 1px solid var(--border-subtle);
  background: var(--bg-inset);
  border-radius: var(--r-sm);
  display: flex; justify-content: space-between; align-items: center;
  font-size: 10px;
  color: var(--text-muted);
  text-transform: capitalize;
}
.sc-el.has {
  color: var(--text-white);
  border-color: var(--accent-blue);
  background: color-mix(in srgb, var(--accent-blue) 12%, var(--bg-inset));
}
.sc-el-count {
  font-family: var(--font-mono);
  font-weight: 700;
}

.sc-hand {
  display: flex;
  flex-direction: column;
  gap: 2px;
  max-height: 180px;
  overflow: auto;
  padding: 4px;
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
}
.sc-card-row {
  padding: 4px 6px;
  border-radius: var(--r-sm);
  font-size: 11px;
  color: var(--text-primary);
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.sc-card-row:hover { background: var(--bg-elevated); }
.sc-empty { padding: 8px; text-align: center; color: var(--text-muted); font-style: italic; font-size: 11px; }

.sc-piles {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 6px;
}
.sc-pile {
  padding: 4px 6px;
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-sm);
  display: flex; justify-content: space-between; align-items: baseline;
  font-size: 10px;
}
.sc-p-label { color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.06em; }
.sc-p-count { font-family: var(--font-mono); font-weight: 700; color: var(--text-primary); }

/* ──────────────────────── Analytics rail (far right) ──────────────────────── */
.analytics-rail {
  grid-area: analytics;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
  border-radius: 0;
  border-left: 1px solid var(--border-subtle);
  border-top: none; border-right: none; border-bottom: none;
  background: var(--bg-surface);
}
.ar-actions {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 12px;
  border-bottom: 1px solid var(--border-subtle);
  flex: 0 0 auto;
}
.ar-actions .pop-btn { margin-left: 0; text-align: left; }

/* ──────────────────────── Responsive ──────────────────────── */
@media (max-width: 1600px) {
  .shell { grid-template-columns: 240px minmax(0, 1fr) 500px 300px; }
}
@media (max-width: 1440px) {
  .shell { grid-template-columns: 240px minmax(0, 1fr) 480px 260px; }
}
@media (max-width: 1280px) {
  /* At this width, drop the analytics rail — nav stays full width */
  .shell {
    grid-template-columns: 240px minmax(0, 1fr) 420px;
    grid-template-areas:
      "nav header header"
      "nav main right";
  }
  .analytics-rail { display: none; }
}
@media (max-width: 1000px) {
  .shell { grid-template-columns: 220px minmax(0, 1fr) 400px; }
}
@media (max-width: 880px) {
  .shell {
    grid-template-columns: 1fr;
    grid-template-rows: 56px auto 1fr auto;
    grid-template-areas:
      "header"
      "nav"
      "main"
      "right";
  }
}

/* Pop-drawer position — covers main+right, leaving nav + analytics visible */
.pop-drawer {
  left: 260px;
  right: 340px;
}
@media (max-width: 1600px) { .pop-drawer { left: 240px; right: 300px; } }
@media (max-width: 1440px) { .pop-drawer { left: 240px; right: 260px; } }
@media (max-width: 1280px) { .pop-drawer { left: 240px; right: 0; } }
@media (max-width: 1600px) { .pop-drawer { left: 240px; right: 300px; } }
@media (max-width: 1440px) { .pop-drawer { left: 240px; right: 260px; } }
@media (max-width: 1280px) { .pop-drawer { left: 240px; right: 0; } }
@media (max-width: 1000px) { .pop-drawer { left: 220px; right: 0; } }
@media (max-width: 880px)  { .pop-drawer { left: 0;     right: 0; } }
</style>
