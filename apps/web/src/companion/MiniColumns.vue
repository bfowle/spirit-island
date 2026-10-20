<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'

/**
 * Small column chart, one column per turn, optionally stacked. Thin
 * columns with a 4px rounded data end, a 2px surface gap between stacked
 * segments, one hairline baseline and a top gridline labeled with the max.
 * Hover or focus a column for every series' value.
 */
export interface MiniSeries { name: string; color: string; values: number[] }
const props = withDefaults(
  defineProps<{ title: string; turns: number[]; series: MiniSeries[]; height?: number; partialLast?: boolean; signed?: boolean }>(),
  { height: 112, partialLast: false, signed: false },
)

// Width follows the container so text stays at its real size.
const W = ref(240)
const box = ref<HTMLElement | null>(null)
let ro: ResizeObserver | null = null
onMounted(() => {
  if (!box.value) return
  ro = new ResizeObserver(([e]) => { W.value = Math.max(160, Math.round(e.contentRect.width)) })
  ro.observe(box.value)
})
onUnmounted(() => ro?.disconnect())
const padL = 22
const padR = 6
const padT = 14
const padB = 18
const H = computed(() => props.height)
const plotH = computed(() => H.value - padT - padB)
const n = computed(() => props.turns.length)
const band = computed(() => (n.value ? (W.value - padL - padR) / n.value : 0))
const colW = computed(() => Math.min(24, Math.max(6, band.value * 0.55)))
const totals = computed(() => props.turns.map((_, i) => props.series.reduce((s, se) => s + Math.max(0, se.values[i] ?? 0), 0)))
const max = computed(() => Math.max(1, ...totals.value))
const y = (v: number) => padT + plotH.value - (v / max.value) * plotH.value
const base = computed(() => padT + plotH.value)

interface Seg { x: number; y: number; w: number; h: number; color: string; top: boolean }
const cols = computed(() =>
  props.turns.map((t, i) => {
    const x = padL + i * band.value + (band.value - colW.value) / 2
    let acc = 0
    const segs: Seg[] = []
    const nonzero = props.series.map((se) => Math.max(0, se.values[i] ?? 0))
    const lastIdx = nonzero.reduce((li, v, k) => (v > 0 ? k : li), -1)
    nonzero.forEach((v, k) => {
      if (v <= 0) return
      const y0 = y(acc)
      const y1 = y(acc + v)
      const gap = k === lastIdx ? 0 : 2
      segs.push({ x, y: y1 + gap, w: colW.value, h: Math.max(1, y0 - y1 - gap), color: props.series[k].color, top: k === lastIdx })
      acc += v
    })
    return { t, i, x, segs, total: totals.value[i] }
  }),
)

function segPath(s: Seg): string {
  if (!s.top) return `M${s.x},${s.y} h${s.w} v${s.h} h${-s.w} Z`
  const r = Math.min(4, s.h, s.w / 2)
  const yb = s.y + s.h
  return `M${s.x},${yb} V${s.y + r} Q${s.x},${s.y} ${s.x + r},${s.y} H${s.x + s.w - r} Q${s.x + s.w},${s.y} ${s.x + s.w},${s.y + r} V${yb} Z`
}

const hover = ref<number | null>(null)
const tipX = computed(() => (hover.value == null ? 0 : ((padL + hover.value * band.value + band.value / 2) / W.value) * 100))
</script>

<template>
  <figure class="mini">
    <figcaption class="mt">{{ title }}</figcaption>
    <div ref="box" class="plot">
      <svg :viewBox="`0 0 ${W} ${H}`" class="svg" role="img" :aria-label="`${title} by turn`">
        <line :x1="padL" :x2="W - padR" :y1="y(max)" :y2="y(max)" class="grid" />
        <text :x="padL - 4" :y="y(max) + 3" text-anchor="end" class="tick">{{ max }}</text>
        <line :x1="padL" :x2="W - padR" :y1="base" :y2="base" class="axis" />
        <text :x="padL - 4" :y="base + 3" text-anchor="end" class="tick">0</text>
        <g
          v-for="c in cols"
          :key="c.t"
          class="col"
          :class="{ partial: partialLast && c.i === n - 1, hov: hover === c.i }"
          tabindex="0"
          @pointerenter="hover = c.i"
          @pointerleave="hover = null"
          @focus="hover = c.i"
          @blur="hover = null"
        >
          <rect :x="padL + c.i * band" :y="padT" :width="band" :height="plotH + padB" class="hit" />
          <path v-for="(s, k) in c.segs" :key="k" :d="segPath(s)" :fill="s.color" />
          <text :x="c.x + colW / 2" :y="H - 4" text-anchor="middle" class="tick">{{ c.t }}</text>
        </g>
      </svg>
      <div v-if="hover != null" class="tip" :style="{ left: `${tipX}%` }" role="tooltip">
        <div class="tip-h">Turn {{ turns[hover] }}<template v-if="partialLast && hover === n - 1"> (so far)</template></div>
        <div v-for="se in series" :key="se.name" class="tip-r">
          <i v-if="series.length > 1" :style="{ background: se.color }"></i>
          <b>{{ se.values[hover] ?? 0 }}</b> <span>{{ se.name }}</span>
        </div>
      </div>
    </div>
    <div v-if="series.length > 1" class="legend">
      <span v-for="se in series" :key="se.name"><i :style="{ background: se.color }"></i>{{ se.name }}</span>
    </div>
  </figure>
</template>

<style scoped>
.mini { margin: 0; min-width: 0; }
.mt { font-size: 12px; color: var(--text-secondary); margin-bottom: 2px; }
.plot { position: relative; }
.svg { display: block; width: 100%; height: auto; overflow: visible; }
.grid { stroke: var(--border-subtle); stroke-width: 1; vector-effect: non-scaling-stroke; }
.axis { stroke: var(--border-strong); stroke-width: 1; vector-effect: non-scaling-stroke; }
.tick { fill: var(--text-muted); font-size: 10px; font-variant-numeric: tabular-nums; }
.hit { fill: transparent; }
.col:focus { outline: none; }
.col.hov .hit, .col:focus .hit { fill: rgba(148, 163, 184, 0.08); }
.col.partial path { opacity: 0.55; }
.tip {
  position: absolute;
  top: 0;
  transform: translate(-50%, -100%);
  background: var(--bg-raised);
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  padding: 4px 8px;
  font-size: 11.5px;
  white-space: nowrap;
  pointer-events: none;
  z-index: 5;
  box-shadow: var(--shadow-lg);
}
.tip-h { color: var(--text-secondary); font-size: 11px; }
.tip-r { display: flex; align-items: center; gap: 4px; color: var(--text-muted); }
.tip-r b { color: var(--text-white); font-variant-numeric: tabular-nums; }
.tip-r i { width: 10px; height: 2px; border-radius: 1px; display: inline-block; }
.legend { display: flex; gap: 10px; font-size: 11px; color: var(--text-secondary); margin-top: 2px; }
.legend i { display: inline-block; width: 9px; height: 9px; border-radius: 2px; margin-right: 4px; vertical-align: -1px; }
</style>
