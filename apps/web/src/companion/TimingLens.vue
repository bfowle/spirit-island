<script setup lang="ts">
import { computed } from 'vue'
import type { LandOutlook } from './types'
import Panel from './Panel.vue'
import LandChip from './LandChip.vue'
import Icon from '../components/Icon.vue'
import { prettyText } from './format'

const props = defineProps<{ lands: LandOutlook[] }>()

// Fixed land order (as the backend lists them); never sorted by value.
// A line that reads the same for Fast and Slow is shown once, across both columns.
const rows = computed(() =>
  props.lands
    .filter((l) => l.fast_window.length || l.slow_window.length)
    .map((l) => {
      const both = l.fast_window.filter((t) => l.slow_window.includes(t))
      return {
        id: l.id,
        fast: l.fast_window.filter((t) => !both.includes(t)),
        slow: l.slow_window.filter((t) => !both.includes(t)),
        both,
      }
    }),
)
</script>

<template>
  <Panel title="Fast or Slow" question="When does an action in each land change something?" anchor="panel-timing">
    <div class="grid head">
      <span></span>
      <div class="col-h">
        <Icon name="speed-fast" :size="16" decorative />
        <div><b>Fast</b>: before the Invader Phase. <span class="hint">Solves this turn.</span></div>
      </div>
      <div class="col-h">
        <Icon name="speed-slow" :size="16" decorative />
        <div><b>Slow</b>: after it, shapes next turn. <span class="hint">Sets up the next Invader Phase.</span></div>
      </div>
    </div>
    <div v-for="r in rows" :key="r.id" class="grid row">
      <div class="land"><LandChip :id="r.id" /></div>
      <ul class="win fast">
        <li v-for="(t, i) in r.fast" :key="i">{{ prettyText(t) }}</li>
        <li v-if="!r.fast.length && !r.both.length" class="none">nothing changes</li>
      </ul>
      <ul class="win slow">
        <li v-for="(t, i) in r.slow" :key="i">{{ prettyText(t) }}</li>
        <li v-if="!r.slow.length && !r.both.length" class="none">nothing changes</li>
      </ul>
      <p v-for="(t, i) in r.both" :key="`b${i}`" class="both"><span class="both-l">Either speed</span> {{ prettyText(t) }}</p>
    </div>
    <p v-if="!rows.length" class="empty">No land has a timing difference right now.</p>
  </Panel>
</template>

<style scoped>
.grid { display: grid; grid-template-columns: 52px minmax(0, 1fr) minmax(0, 1fr); column-gap: 12px; }
.head { padding-bottom: 6px; border-bottom: 1px solid var(--border-subtle); }
.col-h { display: flex; gap: 6px; align-items: flex-start; font-size: 12.5px; color: var(--text-primary); }
.hint { color: var(--text-muted); }
.row { padding: 5px 0; border-bottom: 1px solid color-mix(in srgb, var(--border-subtle) 60%, transparent); row-gap: 3px; }
.row:last-child { border-bottom: 0; }
.land { grid-row: span 3; }
.win { margin: 0; padding: 0 0 0 9px; list-style: none; font-size: 12px; line-height: 1.4; color: var(--text-primary); border-left: 2px solid; }
.win:empty { border-left-color: transparent !important; }
.win.fast { border-color: rgba(239, 68, 68, 0.55); }
.win.slow { border-color: rgba(59, 130, 246, 0.6); }
.win li + li { margin-top: 2px; }
.both { grid-column: 2 / 4; margin: 0; padding-left: 9px; font-size: 12px; line-height: 1.4; color: var(--text-secondary); border-left: 2px solid var(--border-strong); }
.both-l { color: var(--text-muted); margin-right: 4px; }
.none { color: var(--text-faint); }
.empty { margin: 8px 0 0; font-size: 12.5px; color: var(--text-muted); }
</style>
