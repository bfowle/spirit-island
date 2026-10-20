<script setup lang="ts">
import { computed } from 'vue'
import type { LandOutlook, PublicBoard, PublicLand } from './types'
import Icon from '../components/Icon.vue'
import { computeLayout, type IslandLayout } from './islandLayout'
import { parseLand, terrainColor, terrainIcon } from './format'

const props = defineProps<{
  lands: PublicLand[]
  adjacency: [string, string][]
  boards: PublicBoard[]
  outlooks: LandOutlook[]
  selected: string | null
}>()
const emit = defineEmits<{ select: [id: string | null] }>()

// Layout depends only on topology: recompute when boards / lands / adjacency change.
let cacheKey = ''
let cache: IslandLayout | null = null
const layout = computed<IslandLayout>(() => {
  const key = [
    props.boards.map((b) => b.id).join(','),
    props.lands.map((l) => `${l.id}:${l.board}:${l.number}:${l.is_ocean ? 1 : 0}`).join(','),
    props.adjacency.map(([a, b]) => `${a}-${b}`).join(','),
  ].join('|')
  if (key !== cacheKey || !cache) {
    cache = computeLayout(props.lands, props.adjacency, props.boards.map((b) => b.id))
    cacheKey = key
  }
  return cache
})

const outlookById = computed(() => Object.fromEntries(props.outlooks.map((o) => [o.id, o])) as Record<string, LandOutlook>)
const neighbors = computed(() => {
  const m: Record<string, Set<string>> = {}
  for (const [a, b] of props.adjacency) {
    ;(m[a] ??= new Set()).add(b)
    ;(m[b] ??= new Set()).add(a)
  }
  return m
})
const selNeighbors = computed(() => (props.selected ? neighbors.value[props.selected] ?? new Set<string>() : new Set<string>()))
const selSources = computed(() => new Set(props.selected ? outlookById.value[props.selected]?.fed_by ?? [] : []))

interface Cell {
  land: PublicLand
  o: LandOutlook | undefined
  x: number
  y: number
  w: number
  h: number
  short: string
  presence: number
  tokens: [string, number][]
  strife: number
  chips: { cls: string; text: string; title: string }[]
  explore: [number, number] | null
}

const cells = computed<Cell[]>(() => {
  const L = layout.value
  return props.lands
    .filter((l) => L.nodes[l.id])
    .map((land) => {
      const n = L.nodes[land.id]
      const o = outlookById.value[land.id]
      const badges = new Set(o?.badges ?? [])
      const chips: Cell['chips'] = []
      if (badges.has('back-to-back')) {
        chips.push({ cls: 'rav', text: 'Ravage now + next', title: 'Ravages this turn and matches next turn’s Ravage card' })
      } else {
        if (badges.has('ravage-now')) chips.push({ cls: 'rav', text: 'Ravage', title: 'Matches this turn’s Ravage card and has Invaders' })
        if (badges.has('ravage-next')) chips.push({ cls: 'rav-next', text: 'Ravage next', title: 'Matches the Build card (next turn’s Ravage) and has Invaders after this Build' })
      }
      if (badges.has('build-now')) chips.push({ cls: 'bld', text: `Build +${o?.build_now ?? ''}`, title: 'Matches this turn’s Build card and has Invaders' })
      if (badges.has('sole-source')) chips.push({ cls: 'sole', text: 'Sole source', title: `Only Explore source for ${(o?.sole_source_for ?? []).map((x) => parseLand(x).short).join(', ')}` })
      if (badges.has('explore-dead')) chips.push({ cls: 'dead', text: 'No source', title: 'No Explore source in range after this phase’s Builds' })
      const ec = o?.explore_chance ?? null
      return {
        land,
        o,
        x: n.x - n.w / 2,
        y: n.y - n.h / 2,
        w: n.w,
        h: n.h,
        short: parseLand(land.id).short,
        presence: Object.values(land.presence).reduce((s, v) => s + v, 0),
        tokens: Object.entries(land.tokens).filter(([, v]) => v > 0),
        strife: land.pieces.reduce((s, p) => s + (p.strife > 0 ? 1 : 0), 0),
        chips,
        explore: ec && ec[1] > 0 && !land.is_ocean ? ec : null,
      }
    })
})

const edges = computed(() => {
  const L = layout.value
  return L.edges
    .filter((e) => L.nodes[e.a] && L.nodes[e.b])
    .map((e) => {
      const A = L.nodes[e.a], B = L.nodes[e.b]
      const hot = props.selected != null && (e.a === props.selected || e.b === props.selected)
      return { key: `${e.a}-${e.b}`, x1: A.x, y1: A.y, x2: B.x, y2: B.y, cross: e.cross, hot }
    })
})

function cellTitle(c: Cell): string {
  const l = c.land
  if (l.is_ocean) return `Ocean ${c.short}`
  const parts = [`${l.terrain} ${c.short}${l.coastal ? ' (coastal)' : ''}`]
  const inv = [l.explorers && `${l.explorers} Explorer`, l.towns && `${l.towns} Town`, l.cities && `${l.cities} City`].filter(Boolean)
  if (inv.length) parts.push(inv.join(', '))
  if (l.dahan) parts.push(`${l.dahan} Dahan`)
  if (l.blight) parts.push(`${l.blight} Blight`)
  if (c.explore) parts.push(`Explore: ${c.explore[0]} of ${c.explore[1]} candidate cards`)
  return parts.join('\n')
}
</script>

<template>
  <div class="island">
    <svg
      class="map"
      :viewBox="`0 0 ${layout.width} ${layout.height}`"
      role="group"
      aria-label="Island map"
      @click.self="emit('select', null)"
    >
      <defs>
        <pattern id="waves" width="18" height="10" patternUnits="userSpaceOnUse">
          <path d="M0 6 Q4.5 2 9 6 T18 6" fill="none" stroke="rgba(255,255,255,0.16)" stroke-width="1.2" />
        </pattern>
      </defs>
      <g class="hulls">
        <g v-for="h in layout.hulls" :key="h.board">
          <polygon :points="h.points.map((p) => p.join(',')).join(' ')" class="hull" />
          <text :x="h.labelX" :y="h.labelY" class="board-label" text-anchor="middle">Board {{ h.board }}</text>
        </g>
      </g>
      <g class="edges">
        <line
          v-for="e in edges"
          :key="e.key"
          :x1="e.x1" :y1="e.y1" :x2="e.x2" :y2="e.y2"
          :class="{ cross: e.cross, hot: e.hot }"
        />
      </g>
      <g class="cells">
        <foreignObject v-for="c in cells" :key="c.land.id" :x="c.x" :y="c.y" :width="c.w" :height="c.h">
          <button
            v-if="c.land.is_ocean"
            type="button"
            class="cell ocean"
            :class="{ sel: selected === c.land.id, nbr: selNeighbors.has(c.land.id), src: selSources.has(c.land.id) }"
            :title="cellTitle(c)"
            @click="emit('select', selected === c.land.id ? null : c.land.id)"
          >
            <span class="ocean-name">Ocean {{ c.short }}</span>
          </button>
          <button
            v-else
            type="button"
            class="cell"
            :class="{ sel: selected === c.land.id, nbr: selNeighbors.has(c.land.id), src: selSources.has(c.land.id) }"
            :title="cellTitle(c)"
            :aria-pressed="selected === c.land.id"
            @click="emit('select', selected === c.land.id ? null : c.land.id)"
          >
            <span class="ch" :style="{ background: terrainColor(c.land.terrain) }">
              <Icon v-if="terrainIcon(c.land.terrain)" :name="terrainIcon(c.land.terrain)!" :size="15" decorative class="t-ico" />
              <b class="lid">{{ c.short }}</b>
              <span class="tn">{{ c.land.terrain }}</span>
              <span v-if="c.land.coastal" class="coast" title="Coastal">≈</span>
            </span>
            <span class="pcs">
              <span v-if="c.land.explorers" class="pc"><Icon name="unit-explorer" :size="13" decorative />{{ c.land.explorers }}</span>
              <span v-if="c.land.towns" class="pc"><Icon name="unit-town" :size="14" decorative />{{ c.land.towns }}</span>
              <span v-if="c.land.cities" class="pc"><Icon name="unit-city" :size="15" decorative />{{ c.land.cities }}</span>
              <span v-if="c.land.dahan" class="pc dahan"><Icon name="unit-dahan" :size="13" decorative />{{ c.land.dahan }}</span>
              <span v-if="c.land.blight" class="pc blight"><Icon name="resource-blight" :size="14" decorative />{{ c.land.blight }}</span>
              <span v-if="c.presence" class="pc pres" :title="c.presence >= 2 ? 'Sacred site' : 'Presence'"><i class="disc" :class="{ ss: c.presence >= 2 }"></i>{{ c.presence }}</span>
              <span v-for="[t, v] in c.tokens" :key="t" class="tok">{{ t }}<template v-if="v > 1"> {{ v }}</template></span>
              <span v-if="c.strife" class="tok strife">Strife {{ c.strife }}</span>
              <span v-if="!c.land.explorers && !c.land.towns && !c.land.cities && !c.land.dahan && !c.land.blight && !c.presence && !c.tokens.length" class="none">empty</span>
            </span>
            <span v-if="c.chips.length" class="chips">
              <span v-for="ch in c.chips" :key="ch.text" class="chip" :class="ch.cls" :title="ch.title">{{ ch.text }}</span>
            </span>
            <span v-if="c.explore" class="exp" :title="`${c.explore[0]} of ${c.explore[1]} candidate Explore cards hit this land`">
              <span class="exp-l">Explore</span>
              <b>{{ c.explore[0] }}/{{ c.explore[1] }}</b>
              <span v-if="c.explore[1] <= 12" class="exp-pips">
                <i v-for="i in c.explore[1]" :key="i" :class="{ on: i <= c.explore[0] }"></i>
              </span>
            </span>
          </button>
        </foreignObject>
      </g>
    </svg>
  </div>
</template>

<style scoped>
.island { width: 100%; }
.map { display: block; width: 100%; height: auto; }
.hull { fill: hsl(222 38% 14%); stroke: hsl(222 38% 14%); stroke-width: 26; stroke-linejoin: round; }
.board-label { fill: var(--text-muted); font-family: var(--font-brand); font-size: 15px; letter-spacing: 0.04em; }
.edges line { stroke: hsl(215 25% 42%); stroke-width: 1.6; stroke-linecap: round; }
.edges line.cross { stroke: hsl(258 60% 70%); stroke-width: 2; }
.edges line.hot { stroke: var(--text-white); stroke-width: 3; }

.cell {
  all: unset;
  box-sizing: border-box;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  border-radius: 9px;
  background: hsl(222 40% 12%);
  border: 1px solid hsl(217 33% 24%);
  overflow: hidden;
  cursor: pointer;
  font-family: var(--font-sans);
  color: var(--text-primary);
  transition: border-color var(--motion-fast), box-shadow var(--motion-fast);
}
.cell:hover { border-color: var(--text-secondary); }
.cell:focus-visible { box-shadow: 0 0 0 3px var(--accent-blue-hover); }
.cell.nbr { border-color: var(--text-secondary); }
.cell.src { box-shadow: 0 0 0 2px var(--accent-teal); }
.cell.sel { border-color: var(--text-white); box-shadow: 0 0 0 3px var(--accent-blue-hover); }

.ch { display: flex; align-items: center; gap: 5px; padding: 3px 7px; color: #0b1220; flex: 0 0 auto; }
.t-ico { opacity: 0.85; }
.lid { font-family: var(--font-brand); font-size: 16px; font-weight: 600; line-height: 1; }
.tn { font-size: 11px; font-weight: 600; opacity: 0.8; }
.coast { margin-left: auto; font-weight: 700; font-size: 14px; opacity: 0.75; }

.pcs { display: flex; flex-wrap: wrap; align-items: center; gap: 2px 7px; padding: 5px 7px 2px; font-size: 13px; font-weight: 600; font-variant-numeric: tabular-nums; }
.pc { display: inline-flex; align-items: center; gap: 2px; }
.pc.dahan { color: #a7f3d0; }
.pc.blight { color: #ddd6fe; }
.disc { display: inline-block; width: 10px; height: 10px; border-radius: 50%; background: var(--accent-blue-hover); margin-right: 2px; border: 1.5px solid #dbeafe; }
.disc.ss { box-shadow: 0 0 0 2px rgba(96, 165, 250, 0.45); }
.tok { font-size: 10.5px; font-weight: 600; padding: 0 4px; border-radius: 3px; background: var(--bg-active); color: var(--text-secondary); }
.tok.strife { color: #fde68a; }
.none { font-size: 11px; font-weight: 500; color: var(--text-faint); }

.chips { display: flex; flex-wrap: wrap; gap: 3px; padding: 2px 6px 0; }
.chip { font-size: 10px; font-weight: 600; line-height: 15px; padding: 0 4px; border-radius: 3px; white-space: nowrap; }
.chip.rav { background: rgba(239, 68, 68, 0.22); color: #fecaca; }
.chip.rav-next { box-shadow: inset 0 0 0 1px rgba(239, 68, 68, 0.6); color: #fca5a5; }
.chip.bld { background: rgba(251, 146, 60, 0.2); color: #fed7aa; }
.chip.sole { background: rgba(124, 58, 237, 0.24); color: #ddd6fe; }
.chip.dead { background: var(--bg-active); color: var(--text-secondary); }

.exp { display: flex; align-items: center; gap: 5px; margin-top: auto; padding: 2px 7px 5px; font-size: 11px; color: var(--text-secondary); }
.exp b { color: #a5f3fc; font-variant-numeric: tabular-nums; font-size: 12px; }
.exp-pips { display: inline-flex; gap: 2px; }
.exp-pips i { width: 6px; height: 6px; border-radius: 1.5px; background: var(--bg-active); }
.exp-pips i.on { background: var(--accent-teal); }

.cell.ocean {
  align-items: center;
  justify-content: center;
  background-color: color-mix(in srgb, var(--terrain-ocean) 45%, hsl(222 40% 12%));
  background-image: repeating-linear-gradient(0deg, transparent 0 9px, rgba(255, 255, 255, 0.08) 9px 10px);
  border-color: color-mix(in srgb, var(--terrain-ocean) 70%, transparent);
}
.ocean-name { writing-mode: vertical-rl; font-family: var(--font-brand); font-size: 14px; letter-spacing: 0.06em; color: #dbeafe; }
</style>
