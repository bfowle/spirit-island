<script setup lang="ts">
import type { Prompt } from './types'
import Panel from './Panel.vue'
import { prettyText } from './format'

defineProps<{ prompts: Prompt[] }>()

const ANCHORS: Record<string, string> = {
  'Ravage ledger': 'panel-ravage',
  Pipeline: 'panel-pipeline',
  'Explore access': 'panel-island',
  'Explore odds': 'panel-explore',
  Fear: 'panel-fear',
  Blight: 'panel-fear',
  Spirit: 'panel-spirit',
  'Need finder': 'panel-needs',
}
function go(panel: string) {
  const id = ANCHORS[panel]
  const el = id ? document.getElementById(id) : null
  el?.scrollIntoView({ behavior: 'smooth', block: 'start' })
}
</script>

<template>
  <Panel title="Think about" question="Questions this moment raises, with the facts behind them" anchor="panel-think">
    <ol v-if="prompts.length" class="prompts">
      <li v-for="(p, i) in prompts" :key="i" class="prompt">
        <p class="q">{{ p.question }}</p>
        <ul class="facts">
          <li v-for="(f, j) in p.facts" :key="j">{{ prettyText(f) }}</li>
        </ul>
        <button type="button" class="link-btn" @click="go(p.panel)">See {{ p.panel }}</button>
      </li>
    </ol>
    <p v-else class="empty">No open questions at this step.</p>
  </Panel>
</template>

<style scoped>
.prompts { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 10px; }
.prompt { padding: 8px 10px; border-radius: 8px; background: var(--bg-inset); border-left: 2px solid var(--accent-purple); }
.q { margin: 0 0 4px; font-size: 13.5px; font-weight: 600; color: var(--text-white); line-height: 1.35; }
.facts { margin: 0; padding-left: 16px; font-size: 12px; color: var(--text-secondary); }
.facts li + li { margin-top: 2px; }
.link-btn { all: unset; cursor: pointer; margin-top: 4px; font-size: 11.5px; color: var(--accent-blue-hover); }
.link-btn:hover { text-decoration: underline; }
.link-btn:focus-visible { outline: 2px solid var(--accent-blue-hover); outline-offset: 2px; border-radius: 2px; }
.empty { margin: 0; font-size: 12.5px; color: var(--text-muted); }
</style>
