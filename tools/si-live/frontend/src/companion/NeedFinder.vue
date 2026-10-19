<script setup lang="ts">
import { computed } from 'vue'
import type { NeedCount, NeedFamily, SpiritNeeds } from './types'
import Panel from './Panel.vue'
import LandChip from './LandChip.vue'
import { pct, prettyText } from './format'

const props = defineProps<{ needs: SpiritNeeds[]; multi: boolean }>()

function freq(k: number, n: number): string {
  return `${k} of ${n}`
}

/** Minor and Major families side by side, in the backend's order. */
function families(n: SpiritNeeds): { name: string; minor: NeedFamily | null; major: NeedFamily | null }[] {
  const names: string[] = []
  for (const f of n.minor_families) if (!names.includes(f[0])) names.push(f[0])
  for (const f of n.major_families) if (!names.includes(f[0])) names.push(f[0])
  return names.map((name) => ({
    name,
    minor: n.minor_families.find((f) => f[0] === name) ?? null,
    major: n.major_families.find((f) => f[0] === name) ?? null,
  }))
}

const rows = computed(() =>
  props.needs.map((n) => ({ n, fams: families(n) })),
)

const levels: { key: 'effect' | 'reach' | 'fast_reach'; p: 'p_effect' | 'p_reach' | 'p_fast_reach'; label: string }[] = [
  { key: 'effect', p: 'p_effect', label: 'Could save it' },
  { key: 'reach', p: 'p_reach', label: '…and reach it' },
  { key: 'fast_reach', p: 'p_fast_reach', label: '…and are Fast' },
]
function cell(c: NeedCount, l: (typeof levels)[number]): { f: string; p: string } {
  return { f: freq(c[l.key], c.pool), p: pct(c[l.p]) }
}
</script>

<template>
  <Panel title="Need finder" question="Which unseen powers could answer a land about to take blight? Chances are for a draw of 4." anchor="panel-needs">

    <div v-for="{ n, fams } in rows" :key="n.spirit" class="spirit">
      <h3 v-if="multi" class="sp-name">{{ n.spirit }}</h3>

      <h4 class="sub">Save a land from blight</h4>
      <p v-if="!n.save_land.length" class="empty">No open Ravage adds blight right now.</p>
      <div v-for="s in n.save_land" :key="s.land" class="save">
        <div class="save-head">
          <LandChip :id="s.land" />
          <span>remove <b>{{ s.margin }}</b> damage (or stop the Ravage) to keep it unblighted</span>
        </div>
        <table class="tbl">
          <thead>
            <tr><th></th><th v-for="l in levels" :key="l.key">{{ l.label }}</th></tr>
          </thead>
          <tbody>
            <tr v-for="kind in (['minor', 'major'] as const)" :key="kind">
              <th scope="row">{{ kind === 'minor' ? 'Minor' : 'Major' }}</th>
              <td v-for="l in levels" :key="l.key">
                <span class="f">{{ cell(s[kind], l).f }}</span>
                <span class="arrow">→</span>
                <b class="p">{{ cell(s[kind], l).p }}</b>
              </td>
            </tr>
          </tbody>
        </table>
        <p v-if="s.minor.examples.length || s.major.examples.length" class="ex">
          <span class="lbl">For example</span>
          {{ [...s.minor.examples, ...s.major.examples].join(', ') }}
        </p>
        <p class="ex">
          <span class="lbl">In hand</span>
          <template v-if="s.in_hand.length">{{ s.in_hand.map(prettyText).join('; ') }}</template>
          <span v-else class="muted">no card in hand does it</span>
        </p>
        <p class="pocket">{{ prettyText(s.pocket_rule) }}</p>
      </div>

      <h4 class="sub">Effect families in the unseen pools</h4>
      <table class="tbl fam">
        <thead><tr><th>Effect</th><th>Minor</th><th>Major</th></tr></thead>
        <tbody>
          <tr v-for="f in fams" :key="f.name">
            <th scope="row">{{ f.name }}</th>
            <td>
              <template v-if="f.minor"><span class="f">{{ freq(f.minor[1], f.minor[2]) }}</span> <span class="arrow">→</span> <b class="p">{{ pct(f.minor[3]) }}</b></template>
              <span v-else class="muted">–</span>
            </td>
            <td>
              <template v-if="f.major"><span class="f">{{ freq(f.major[1], f.major[2]) }}</span> <span class="arrow">→</span> <b class="p">{{ pct(f.major[3]) }}</b></template>
              <span v-else class="muted">–</span>
            </td>
          </tr>
        </tbody>
      </table>

      <ol v-if="n.basis.length" class="basis">
        <li v-for="(b, i) in n.basis" :key="i">{{ prettyText(b) }}</li>
      </ol>
    </div>
  </Panel>
</template>

<style scoped>
.sp-name { font-size: 14px; color: var(--text-white); margin: 0 0 6px; }
.sub { font-family: var(--font-brand); font-weight: 500; font-size: 14px; color: var(--text-white); margin: 2px 0 6px; }
.sub:not(:first-child) { margin-top: 12px; }
.save { padding: 6px 0 8px; border-bottom: 1px solid var(--border-subtle); }
.save-head { display: flex; align-items: center; gap: 8px; font-size: 12.5px; color: var(--text-secondary); margin-bottom: 4px; }
.save-head b { color: var(--text-primary); }
.tbl { width: 100%; border-collapse: collapse; font-size: 12px; font-variant-numeric: tabular-nums; }
.tbl th { text-align: left; font-weight: 500; color: var(--text-muted); font-size: 11px; padding: 2px 6px 3px 0; }
.tbl thead th { border-bottom: 1px solid var(--border-subtle); }
.tbl tbody th { color: var(--text-secondary); font-size: 12px; white-space: nowrap; }
.tbl td { padding: 3px 6px 3px 0; white-space: nowrap; color: var(--text-primary); }
.fam tbody tr + tr td, .fam tbody tr + tr th { border-top: 1px solid color-mix(in srgb, var(--border-subtle) 60%, transparent); }
.f { color: var(--text-secondary); }
.arrow { color: var(--text-faint); }
.p { color: var(--text-primary); }
.ex { margin: 4px 0 0; font-size: 12px; color: var(--text-secondary); }
.lbl { color: var(--text-muted); margin-right: 4px; }
.pocket { margin: 6px 0 0; padding: 5px 8px; border-radius: 6px; background: var(--bg-inset); font-size: 12px; color: var(--text-secondary); border-left: 2px solid var(--accent-blue); }
.basis { margin: 10px 0 0; padding-left: 18px; font-size: 11px; color: var(--text-muted); }
.basis li + li { margin-top: 2px; }
.empty { margin: 0; font-size: 12.5px; color: var(--text-muted); }
.muted { color: var(--text-muted); }
</style>
