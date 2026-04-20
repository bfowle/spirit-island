<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { Board, Land, Spirit } from '../types'
import LandEditor from './LandEditor.vue'
import Icon from './Icon.vue'
import PresenceDisc from './PresenceDisc.vue'

/**
 * BoardZone — displays the physical game board as a PNG backdrop with
 * circular land markers overlaid at coordinate positions.
 *
 * The PNG images are stored at /board-images/{boardId}-{variant}.png
 * (e.g., /board-images/A-balanced.png). The coords JSON provides x/y
 * positions as percentages for each land.
 *
 * Land markers show:
 *   - Land number + terrain color tint
 *   - Unit counts (explorers, towns, cities, dahan, blight) as mini badges
 *   - Spirit presence discs stacked visibly
 *
 * Click/hover on a land opens a popover with full details and editing.
 */

const props = defineProps<{
  boards: Record<string, Board>
  spirits: Record<string, Spirit>
}>()

const emit = defineEmits<{
  'update:board': [boardId: string, board: Board]
  'bump-pool': [pool: 'blight', delta: number]
}>()

// Currently selected board tab
const activeBoard = ref<string>(Object.keys(props.boards)[0] ?? 'A')

// Watch for board changes (when game setup changes)
watch(() => Object.keys(props.boards), (keys) => {
  if (!keys.includes(activeBoard.value) && keys.length > 0) {
    activeBoard.value = keys[0]
  }
})

const currentBoard = computed(() => props.boards[activeBoard.value])

// ─────────────────────────────────────────────────────────────────────────────
// Board Coordinates
// ─────────────────────────────────────────────────────────────────────────────

// Default coordinates for standard balanced boards (A-D)
// These are approximate positions matching the physical board layout
// Values are percentages (0-100) for left and top

interface LandCoords {
  x: number
  y: number
}

interface BoardLayout {
  oceanEdge: 'top' | 'bottom' | 'left' | 'right'
  positions: Record<string, LandCoords>
}

const defaultLayouts: Record<string, BoardLayout> = {
  'A-balanced': {
    oceanEdge: 'top',
    positions: {
      '1': { x: 25, y: 18 },
      '2': { x: 55, y: 15 },
      '3': { x: 80, y: 22 },
      '4': { x: 15, y: 45 },
      '5': { x: 45, y: 42 },
      '6': { x: 70, y: 48 },
      '7': { x: 30, y: 72 },
      '8': { x: 58, y: 75 },
    },
  },
  'B-balanced': {
    oceanEdge: 'top',
    positions: {
      '1': { x: 20, y: 20 },
      '2': { x: 50, y: 18 },
      '3': { x: 78, y: 25 },
      '4': { x: 18, y: 50 },
      '5': { x: 48, y: 45 },
      '6': { x: 75, y: 52 },
      '7': { x: 28, y: 75 },
      '8': { x: 60, y: 78 },
    },
  },
  'C-balanced': {
    oceanEdge: 'top',
    positions: {
      '1': { x: 22, y: 22 },
      '2': { x: 52, y: 20 },
      '3': { x: 80, y: 28 },
      '4': { x: 20, y: 52 },
      '5': { x: 50, y: 48 },
      '6': { x: 78, y: 55 },
      '7': { x: 32, y: 78 },
      '8': { x: 62, y: 80 },
    },
  },
  'D-balanced': {
    oceanEdge: 'top',
    positions: {
      '1': { x: 24, y: 20 },
      '2': { x: 54, y: 18 },
      '3': { x: 82, y: 24 },
      '4': { x: 22, y: 48 },
      '5': { x: 52, y: 45 },
      '6': { x: 80, y: 50 },
      '7': { x: 30, y: 76 },
      '8': { x: 64, y: 78 },
    },
  },
}

function getLayout(boardId: string, variant?: string): BoardLayout {
  const key = `${boardId}-${variant || 'balanced'}`
  if (defaultLayouts[key]) return defaultLayouts[key]
  
  // Fallback: generate a grid layout
  const lands = currentBoard.value?.lands ?? {}
  const landIds = Object.keys(lands).sort((a, b) => Number(a) - Number(b))
  const positions: Record<string, LandCoords> = {}
  
  const cols = 3
  landIds.forEach((id, i) => {
    const row = Math.floor(i / cols)
    const col = i % cols
    positions[id] = {
      x: 20 + col * 30,
      y: 20 + row * 25,
    }
  })
  
  return { oceanEdge: 'top', positions }
}

const layout = computed(() => getLayout(activeBoard.value, currentBoard.value?.variant))

// ─────────────────────────────────────────────────────────────────────────────
// Board Image Path
// ─────────────────────────────────────────────────────────────────────────────

const boardImagePath = computed(() => {
  const variant = currentBoard.value?.variant || 'balanced'
  return `/board-images/${activeBoard.value}-${variant}.png`
})

// Track if the board image loads successfully
const imageLoaded = ref(false)
const imageError = ref(false)

function onImageLoad() {
  imageLoaded.value = true
  imageError.value = false
}

function onImageError() {
  imageLoaded.value = false
  imageError.value = true
}

// ─────────────────────────────────────────────────────────────────────────────
// Land Helpers
// ─────────────────────────────────────────────────────────────────────────────

function sortedLandIds(): string[] {
  const board = currentBoard.value
  if (!board) return []
  return Object.keys(board.lands).sort((a, b) => Number(a) - Number(b))
}

interface DiscInfo {
  slug: string
  count: number
  color: string | undefined
  style: 'glass' | 'wood' | 'solid'
}

function presenceOn(landId: string): DiscInfo[] {
  const out: DiscInfo[] = []
  for (const [slug, spirit] of Object.entries(props.spirits)) {
    const key = `${activeBoard.value}.${landId}`
    const n = spirit.presence_on_board?.[key] ?? 0
    if (n > 0) {
      out.push({
        slug,
        count: n,
        color: spirit.disc_color,
        style: spirit.disc_style ?? 'glass',
      })
    }
  }
  return out
}

function bumpPresence(landId: string, slug: string, delta: number) {
  const spirit = props.spirits[slug]
  if (!spirit) return
  const key = `${activeBoard.value}.${landId}`
  const current = spirit.presence_on_board?.[key] ?? 0
  const next = Math.max(0, current + delta)
  if (!spirit.presence_on_board) spirit.presence_on_board = {}
  if (next === 0) delete spirit.presence_on_board[key]
  else spirit.presence_on_board[key] = next
}

interface UnitBadge {
  type: string
  count: number
  icon: string
}

function getUnitBadges(land: Land): UnitBadge[] {
  const badges: UnitBadge[] = []
  if (land.explorers) badges.push({ type: 'explorer', count: land.explorers, icon: 'unit-explorer' })
  if (land.towns) badges.push({ type: 'town', count: land.towns, icon: 'unit-town' })
  if (land.cities) badges.push({ type: 'city', count: land.cities, icon: 'unit-city' })
  if (land.dahan) badges.push({ type: 'dahan', count: land.dahan, icon: 'unit-dahan' })
  if (land.blight) badges.push({ type: 'blight', count: land.blight, icon: 'resource-blight' })
  return badges
}

// ─────────────────────────────────────────────────────────────────────────────
// Popover State
// ─────────────────────────────────────────────────────────────────────────────

const activeLand = ref<string | null>(null)
const popoverPosition = ref<{ x: number; y: number }>({ x: 0, y: 0 })

function openLandPopover(landId: string, event: MouseEvent) {
  activeLand.value = landId
  // Position popover near click, but keep it in viewport
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
  popoverPosition.value = {
    x: Math.min(rect.left, window.innerWidth - 320),
    y: rect.bottom + 8,
  }
}

function closeLandPopover() {
  activeLand.value = null
}

function updateLand(landId: string, land: Land) {
  const board = currentBoard.value
  if (!board) return
  emit('update:board', activeBoard.value, {
    ...board,
    lands: { ...board.lands, [landId]: land },
  })
}

// Terrain color mapping
const terrainColors: Record<string, string> = {
  mountain: '#9ca3af',
  wetland: '#22d3ee',
  jungle: '#22c55e',
  sands: '#fbbf24',
  ocean: '#3b82f6',
}

function getTerrainColor(terrain: string): string {
  return terrainColors[terrain.toLowerCase()] ?? '#6b7280'
}
</script>

<template>
  <div class="board-zone">
    <!-- Board Tabs (if multiple boards) -->
    <div v-if="Object.keys(boards).length > 1" class="board-tabs">
      <button
        v-for="(board, id) in boards"
        :key="id"
        class="board-tab"
        :class="{ active: activeBoard === id }"
        @click="activeBoard = id"
      >
        <span class="tab-id">{{ id }}</span>
        <span v-if="board.variant_name" class="tab-variant">{{ board.variant_name }}</span>
      </button>
    </div>

    <!-- Board Canvas -->
    <div class="board-canvas" :class="[`ocean-${layout.oceanEdge}`]">
      <!-- PNG Backdrop -->
      <img
        v-if="!imageError"
        :src="boardImagePath"
        :alt="`Board ${activeBoard}`"
        class="board-image"
        :class="{ loaded: imageLoaded }"
        @load="onImageLoad"
        @error="onImageError"
      />
      
      <!-- Schematic fallback when no PNG loaded -->
      <div v-if="imageError || !imageLoaded" class="board-schematic">
        <!-- Terrain region backgrounds based on land positions -->
        <svg class="schematic-regions" viewBox="0 0 100 100" preserveAspectRatio="none">
          <defs>
            <linearGradient id="mountainGrad" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" style="stop-color:#6b7280;stop-opacity:0.15" />
              <stop offset="100%" style="stop-color:#9ca3af;stop-opacity:0.08" />
            </linearGradient>
            <linearGradient id="wetlandGrad" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" style="stop-color:#22d3ee;stop-opacity:0.12" />
              <stop offset="100%" style="stop-color:#06b6d4;stop-opacity:0.06" />
            </linearGradient>
            <linearGradient id="jungleGrad" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" style="stop-color:#22c55e;stop-opacity:0.12" />
              <stop offset="100%" style="stop-color:#16a34a;stop-opacity:0.06" />
            </linearGradient>
            <linearGradient id="sandsGrad" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" style="stop-color:#fbbf24;stop-opacity:0.12" />
              <stop offset="100%" style="stop-color:#f59e0b;stop-opacity:0.06" />
            </linearGradient>
          </defs>
          <!-- Draw organic terrain blobs for each land -->
          <ellipse 
            v-for="landId in sortedLandIds()" 
            :key="landId"
            :cx="layout.positions[landId]?.x ?? 50"
            :cy="layout.positions[landId]?.y ?? 50"
            rx="18"
            ry="16"
            :fill="`url(#${(currentBoard?.lands[landId]?.terrain ?? 'mountain').toLowerCase()}Grad)`"
            :style="{ transform: `rotate(${(Number(landId) * 23) % 45 - 22}deg)`, transformOrigin: `${layout.positions[landId]?.x ?? 50}% ${layout.positions[landId]?.y ?? 50}%` }"
          />
        </svg>
        <div class="schematic-label">
          <span class="board-letter">{{ activeBoard }}</span>
          <span class="board-variant">{{ currentBoard?.variant_name || 'Balanced' }}</span>
        </div>
      </div>

      <!-- Ocean indicator strip -->
      <div class="ocean-strip" :title="'Ocean'"></div>

      <!-- Land Markers -->
      <button
        v-for="landId in sortedLandIds()"
        :key="landId"
        class="land-marker"
        :class="[
          `terrain-${currentBoard?.lands[landId]?.terrain}`,
          { coastal: currentBoard?.lands[landId]?.coastal, active: activeLand === landId }
        ]"
        :style="{
          left: (layout.positions[landId]?.x ?? 50) + '%',
          top: (layout.positions[landId]?.y ?? 50) + '%',
          '--terrain-color': getTerrainColor(currentBoard?.lands[landId]?.terrain ?? 'mountain'),
        }"
        @click="openLandPopover(landId, $event)"
      >
        <span class="marker-id">{{ landId }}</span>
        
        <!-- Presence discs -->
        <div v-if="presenceOn(landId).length" class="marker-presence">
          <span
            v-for="info in presenceOn(landId)"
            :key="info.slug"
            class="presence-stack"
          >
            <PresenceDisc
              v-for="n in Math.min(info.count, 3)"
              :key="n"
              :slug="info.slug"
              :color="info.color"
              :style="info.style"
              :size="10"
            />
            <span v-if="info.count > 3" class="presence-overflow">+{{ info.count - 3 }}</span>
          </span>
        </div>

        <!-- Unit badges -->
        <div v-if="getUnitBadges(currentBoard?.lands[landId] ?? {}).length" class="marker-units">
          <span
            v-for="badge in getUnitBadges(currentBoard?.lands[landId] ?? {})"
            :key="badge.type"
            class="unit-badge"
            :class="badge.type"
            :title="`${badge.count} ${badge.type}`"
          >
            <Icon :name="badge.icon" :size="10" decorative />
            <span class="badge-count">{{ badge.count }}</span>
          </span>
        </div>

        <!-- Coastal indicator -->
        <span v-if="currentBoard?.lands[landId]?.coastal" class="coastal-dot" title="Coastal"></span>
      </button>
    </div>

    <!-- Land Popover -->
    <Teleport to="body">
      <div
        v-if="activeLand && currentBoard"
        class="land-popover"
        :style="{ left: popoverPosition.x + 'px', top: popoverPosition.y + 'px' }"
      >
        <div class="popover-backdrop" @click="closeLandPopover"></div>
        <div class="popover-content">
          <div class="popover-header">
            <div class="popover-title">
              <span
                class="terrain-dot"
                :style="{ background: getTerrainColor(currentBoard.lands[activeLand]?.terrain ?? 'mountain') }"
              ></span>
              <span class="land-number">#{{ activeLand }}</span>
              <span class="land-terrain">{{ currentBoard.lands[activeLand]?.terrain }}</span>
              <span v-if="currentBoard.lands[activeLand]?.coastal" class="coastal-tag">coast</span>
            </div>
            <button class="close-btn" @click="closeLandPopover">
              <span>&#10005;</span>
            </button>
          </div>

          <!-- Presence on this land -->
          <div v-if="presenceOn(activeLand).length" class="popover-presence">
            <div class="section-label">Presence</div>
            <div class="presence-list">
              <div
                v-for="info in presenceOn(activeLand)"
                :key="info.slug"
                class="presence-row"
              >
                <PresenceDisc
                  :slug="info.slug"
                  :color="info.color"
                  :style="info.style"
                  :size="14"
                />
                <span class="presence-spirit">{{ info.slug }}</span>
                <span class="presence-count">{{ info.count }}</span>
                <div class="presence-controls">
                  <button class="stepper" @click="bumpPresence(activeLand!, info.slug, -1)">-</button>
                  <button class="stepper" @click="bumpPresence(activeLand!, info.slug, 1)">+</button>
                </div>
              </div>
            </div>
          </div>

          <!-- Land Editor -->
          <LandEditor
            v-if="currentBoard.lands[activeLand]"
            :model-value="currentBoard.lands[activeLand]"
            :land-id="activeLand"
            :spirits="spirits"
            :board-id="activeBoard"
            @update:model-value="updateLand(activeLand!, $event)"
            @bump-presence="(slug, delta) => bumpPresence(activeLand!, slug, delta)"
            @bump-pool="(pool, delta) => $emit('bump-pool', pool, delta)"
          />
        </div>
      </div>
    </Teleport>

    <!-- Legend -->
    <div class="board-legend">
      <span class="legend-item">
        <span class="legend-dot" style="background: #9ca3af"></span>
        Mountain
      </span>
      <span class="legend-item">
        <span class="legend-dot" style="background: #22d3ee"></span>
        Wetland
      </span>
      <span class="legend-item">
        <span class="legend-dot" style="background: #22c55e"></span>
        Jungle
      </span>
      <span class="legend-item">
        <span class="legend-dot" style="background: #fbbf24"></span>
        Sands
      </span>
      <span class="legend-item coastal-legend">
        <span class="coastal-indicator"></span>
        Coastal
      </span>
    </div>
  </div>
</template>

<style scoped>
/* ═══════════════════════════════════════════════════════════════════════════ */
/* BOARD ZONE — PNG BACKDROP WITH CIRCULAR LAND MARKERS                        */
/* ═══════════════════════════════════════════════════════════════════════════ */

.board-zone {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
  height: 100%;
}

/* ─── BOARD TABS ─── */
.board-tabs {
  display: flex;
  gap: 2px;
  padding: 2px;
  background: var(--bg-muted);
  border-radius: var(--radius-md);
  width: fit-content;
}

.board-tab {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-2) var(--sp-3);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  cursor: pointer;
  font-size: var(--text-sm);
  transition: all var(--duration-base) var(--ease);
}

.board-tab:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.board-tab.active {
  background: var(--bg-surface);
  color: var(--text-primary);
  box-shadow: var(--shadow-sm);
}

.tab-id {
  font-weight: var(--weight-bold);
  font-family: var(--font-mono);
}

.tab-variant {
  font-size: var(--text-xs);
  color: var(--text-muted);
}

/* ─── BOARD CANVAS ─── */
.board-canvas {
  position: relative;
  flex: 1;
  min-height: 400px;
  max-height: 600px;
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.board-image {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  object-fit: contain;
  opacity: 0;
  transition: opacity var(--duration-base) var(--ease);
}

.board-image.loaded {
  opacity: 0.85;
}

.board-schematic {
  position: absolute;
  inset: 0;
  background: linear-gradient(
    135deg, 
    var(--bg-surface) 0%, 
    rgba(var(--color-accent-rgb, 99, 102, 241), 0.02) 50%,
    var(--bg-elevated) 100%
  );
}

.schematic-regions {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}

.schematic-label {
  position: absolute;
  bottom: var(--sp-4);
  right: var(--sp-4);
  display: flex;
  align-items: baseline;
  gap: var(--sp-2);
  padding: var(--sp-2) var(--sp-3);
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
}

.board-letter {
  font-family: var(--font-mono);
  font-size: var(--text-2xl);
  font-weight: var(--weight-bold);
  color: var(--text-primary);
}

.board-variant {
  font-size: var(--text-sm);
  color: var(--text-muted);
  text-transform: capitalize;
}

/* ─── OCEAN STRIP ─── */
.ocean-strip {
  position: absolute;
  pointer-events: none;
  z-index: 1;
}

.board-canvas.ocean-top .ocean-strip {
  top: 0;
  left: 0;
  right: 0;
  height: 12%;
  background: linear-gradient(to bottom, rgba(59, 130, 246, 0.2), transparent);
}

.board-canvas.ocean-bottom .ocean-strip {
  bottom: 0;
  left: 0;
  right: 0;
  height: 12%;
  background: linear-gradient(to top, rgba(59, 130, 246, 0.2), transparent);
}

.board-canvas.ocean-left .ocean-strip {
  top: 0;
  bottom: 0;
  left: 0;
  width: 12%;
  background: linear-gradient(to right, rgba(59, 130, 246, 0.2), transparent);
}

.board-canvas.ocean-right .ocean-strip {
  top: 0;
  bottom: 0;
  right: 0;
  width: 12%;
  background: linear-gradient(to left, rgba(59, 130, 246, 0.2), transparent);
}

/* ─── LAND MARKERS ─── */
.land-marker {
  position: absolute;
  transform: translate(-50%, -50%);
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: var(--bg-surface);
  border: 3px solid var(--terrain-color, var(--border-default));
  box-shadow: var(--shadow-md), 0 0 0 2px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  cursor: pointer;
  transition: transform var(--duration-base) var(--ease), box-shadow var(--duration-base) var(--ease);
  z-index: 5;
}

.land-marker:hover {
  transform: translate(-50%, -50%) scale(1.1);
  box-shadow: var(--shadow-lg), 0 0 0 3px var(--terrain-color);
  z-index: 10;
}

.land-marker.active {
  transform: translate(-50%, -50%) scale(1.15);
  box-shadow: 0 0 0 4px var(--color-accent), var(--shadow-lg);
  z-index: 15;
}

.marker-id {
  font-family: var(--font-mono);
  font-weight: var(--weight-bold);
  font-size: var(--text-base);
  color: var(--text-primary);
  line-height: 1;
}

/* ─── PRESENCE ON MARKER ─── */
.marker-presence {
  position: absolute;
  top: -4px;
  right: -4px;
  display: flex;
  gap: 1px;
}

.presence-stack {
  display: flex;
  align-items: center;
  gap: -2px;
}

.presence-overflow {
  font-size: 8px;
  font-family: var(--font-mono);
  color: var(--text-primary);
  background: var(--bg-surface);
  padding: 0 2px;
  border-radius: 4px;
  margin-left: 1px;
}

/* ─── UNIT BADGES ON MARKER ─── */
.marker-units {
  position: absolute;
  bottom: -6px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 2px;
}

.unit-badge {
  display: flex;
  align-items: center;
  gap: 1px;
  padding: 1px 3px;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: 8px;
  font-size: 8px;
  font-family: var(--font-mono);
  font-weight: var(--weight-bold);
}

.unit-badge.explorer { color: #ef4444; border-color: rgba(239, 68, 68, 0.3); }
.unit-badge.town { color: #f97316; border-color: rgba(249, 115, 22, 0.3); }
.unit-badge.city { color: #dc2626; border-color: rgba(220, 38, 38, 0.3); }
.unit-badge.dahan { color: #8b5cf6; border-color: rgba(139, 92, 246, 0.3); }
.unit-badge.blight { color: #a855f7; border-color: rgba(168, 85, 247, 0.3); }

.badge-count {
  line-height: 1;
}

/* ─── COASTAL INDICATOR ─── */
.coastal-dot {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 8px;
  height: 8px;
  background: var(--terrain-ocean, #3b82f6);
  border-radius: 50%;
  border: 1px solid white;
  box-shadow: var(--shadow-sm);
}

/* ─── LAND POPOVER ─── */
.land-popover {
  position: fixed;
  z-index: 100;
}

.popover-backdrop {
  position: fixed;
  inset: 0;
  background: transparent;
}

.popover-content {
  position: relative;
  width: 300px;
  max-height: 400px;
  overflow-y: auto;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-xl);
  padding: var(--sp-4);
}

.popover-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--sp-3);
  padding-bottom: var(--sp-3);
  border-bottom: 1px solid var(--border-subtle);
}

.popover-title {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}

.terrain-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.land-number {
  font-family: var(--font-mono);
  font-weight: var(--weight-bold);
  font-size: var(--text-lg);
  color: var(--text-primary);
}

.land-terrain {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  text-transform: capitalize;
}

.coastal-tag {
  font-size: var(--text-xs);
  color: #3b82f6;
  background: rgba(59, 130, 246, 0.1);
  padding: 2px var(--sp-2);
  border-radius: var(--radius-sm);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.close-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-muted);
  border: none;
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  cursor: pointer;
  font-size: var(--text-sm);
}

.close-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

/* ─── PRESENCE IN POPOVER ─── */
.popover-presence {
  margin-bottom: var(--sp-3);
}

.section-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  margin-bottom: var(--sp-2);
}

.presence-list {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.presence-row {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-2);
  background: var(--bg-muted);
  border-radius: var(--radius-md);
}

.presence-spirit {
  flex: 1;
  font-size: var(--text-sm);
  color: var(--text-primary);
}

.presence-count {
  font-family: var(--font-mono);
  font-weight: var(--weight-bold);
  font-size: var(--text-sm);
  color: var(--text-primary);
  min-width: 20px;
  text-align: center;
}

.presence-controls {
  display: flex;
  gap: 2px;
}

.stepper {
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  cursor: pointer;
  font-size: var(--text-sm);
  font-weight: var(--weight-bold);
}

.stepper:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

/* ─── LEGEND ─── */
.board-legend {
  display: flex;
  gap: var(--sp-4);
  padding: var(--sp-2) var(--sp-3);
  background: var(--bg-muted);
  border-radius: var(--radius-md);
  flex-wrap: wrap;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  font-size: var(--text-xs);
  color: var(--text-secondary);
}

.legend-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.coastal-indicator {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: #3b82f6;
  border: 2px solid white;
  box-shadow: 0 0 0 1px #3b82f6;
}
</style>
