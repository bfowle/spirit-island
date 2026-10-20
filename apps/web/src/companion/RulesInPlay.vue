<script setup lang="ts">
import type { RuleInPlay } from './types'
import Panel from './Panel.vue'
import RichText from './RichText.vue'

defineProps<{ rules: RuleInPlay[] }>()

const STATUS: Record<string, { label: string; title: string }> = {
  agree: { label: 'engine and Wiki agree', title: 'The engine data and the Wiki text say the same thing' },
  conflict: { label: 'engine and Wiki differ', title: 'The engine data and the Wiki text disagree: check the rules register' },
  'engine-only': { label: 'engine only', title: 'Found in the engine data, not confirmed by the Wiki' },
  'wiki-only': { label: 'Wiki only', title: 'From the Wiki, not found in the engine data' },
}
</script>

<template>
  <Panel title="Rules in play" question="Adversary, escalation, loss, and spirit rules that touch the Invaders" anchor="panel-rules">
    <ul class="rules">
      <li v-for="(r, i) in rules" :key="i" class="rule">
        <div class="r-h">
          <span class="from">{{ r.from }}</span>
          <b class="title">{{ r.title }}</b>
        </div>
        <p class="text"><RichText :text="r.text" /></p>
        <div class="chips">
          <span class="chip" :class="`st-${r.status}`" :title="STATUS[r.status]?.title ?? r.status">{{ STATUS[r.status]?.label ?? r.status }}</span>
          <span v-if="r.review" class="chip rv" :title="r.review">{{ r.review.startsWith('reviewed') ? 'checked against the box' : 'checked against the rulebook/FAQ' }}</span>
          <span class="chip" :class="r.applied ? 'ap' : 'na'">{{ r.applied ? 'applied by the forecast' : 'not applied by the forecast' }}</span>
        </div>
      </li>
    </ul>
    <p v-if="!rules.length" class="empty">No adversary or spirit rules touch the Invaders here.</p>
  </Panel>
</template>

<style scoped>
.rules { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; }
.rule { padding: 7px 0; border-bottom: 1px solid color-mix(in srgb, var(--border-subtle) 70%, transparent); }
.rule:first-child { padding-top: 0; }
.rule:last-child { border-bottom: 0; padding-bottom: 0; }
.r-h { display: flex; flex-direction: column; }
.from { font-size: 11px; color: var(--text-muted); }
.title { font-size: 13px; color: var(--text-white); }
.text { margin: 3px 0 5px; font-size: 12px; line-height: 1.45; color: var(--text-primary); }
.chips { display: flex; flex-wrap: wrap; gap: 4px; }
.chip { font-size: 10.5px; font-weight: 600; padding: 0 6px; line-height: 16px; border-radius: 3px; background: var(--bg-active); color: var(--text-secondary); }
.chip.st-conflict { background: var(--warn-soft); color: var(--accent-amber); }
.chip.st-engine-only, .chip.st-wiki-only { box-shadow: inset 0 0 0 1px var(--border-strong); background: transparent; }
.chip.ap { background: var(--ok-soft); color: #6ee7b7; }
.chip.rv { background: transparent; box-shadow: inset 0 0 0 1px var(--border-strong); color: var(--text-secondary); }
.chip.na { background: transparent; box-shadow: inset 0 0 0 1px rgba(245, 158, 11, 0.5); color: #fcd34d; }
.empty { margin: 0; font-size: 12.5px; color: var(--text-muted); }
</style>
