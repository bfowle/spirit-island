<script setup lang="ts">
import { computed } from 'vue'
import type { Spirit } from '../types'

const props = defineProps<{ modelValue: Spirit }>()

const elements = computed(() => {
  const e = props.modelValue.elements_this_turn ?? {}
  return Object.entries(e).filter(([, n]) => (n as number) > 0)
})

const handCount = computed(() => props.modelValue.hand?.length ?? 0)
const discardCount = computed(() => props.modelValue.discard?.length ?? 0)
const playedCount = computed(() => props.modelValue.played_this_turn?.length ?? 0)

function moveCard(card: string, from: keyof Spirit, to: keyof Spirit) {
  const src = (props.modelValue[from] as string[] | undefined) ?? []
  const dst = (props.modelValue[to] as string[] | undefined) ?? []
  const idx = src.indexOf(card)
  if (idx < 0) return
  src.splice(idx, 1)
  dst.push(card)
  ;(props.modelValue[from] as unknown) = src
  ;(props.modelValue[to] as unknown) = dst
}
</script>

<template>
  <div class="panel">
    <div class="row">
      <label>Energy <input type="number" v-model.number="modelValue.energy" /></label>
      <label>Card Plays <input type="number" min="0" v-model.number="modelValue.card_plays" /></label>
      <div class="elements">
        <span v-for="[el, n] in elements" :key="el" class="elem" :data-el="el">{{ el }}×{{ n }}</span>
        <span v-if="!elements.length" class="muted">no elements this turn</span>
      </div>
    </div>

    <div class="row tracks">
      <div>
        <div class="track-label">Energy track (remaining):</div>
        <code>{{ (modelValue.presence_on_track_energy ?? []).join(' · ') || '—' }}</code>
      </div>
      <div>
        <div class="track-label">Card-play track (remaining):</div>
        <code>{{ (modelValue.presence_on_track_cardplay ?? []).join(' · ') || '—' }}</code>
      </div>
    </div>

    <div class="cards">
      <div class="pile">
        <h3>Hand ({{ handCount }})</h3>
        <ul>
          <li v-for="c in modelValue.hand ?? []" :key="c">
            {{ c }}
            <button @click="moveCard(c, 'hand', 'played_this_turn')">play</button>
            <button @click="moveCard(c, 'hand', 'discard')">discard</button>
          </li>
        </ul>
      </div>
      <div class="pile">
        <h3>Played ({{ playedCount }})</h3>
        <ul>
          <li v-for="c in modelValue.played_this_turn ?? []" :key="c">
            {{ c }}
            <button @click="moveCard(c, 'played_this_turn', 'discard')">→ discard</button>
            <button @click="moveCard(c, 'played_this_turn', 'hand')">↩ hand</button>
          </li>
        </ul>
      </div>
      <div class="pile">
        <h3>Discard ({{ discardCount }})</h3>
        <ul>
          <li v-for="c in modelValue.discard ?? []" :key="c">
            {{ c }}
            <button @click="moveCard(c, 'discard', 'hand')">↩ hand</button>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<style scoped>
.panel { background: #1a1a1e; border: 1px solid #333; border-radius: 6px; padding: .75rem; }
.row { display: flex; gap: 1rem; align-items: center; flex-wrap: wrap; margin-bottom: .5rem; }
.row.tracks { flex-direction: column; align-items: flex-start; gap: .25rem; }
.track-label { font-size: .8rem; color: #999; }
code { font-size: .85rem; color: #cfa; }
input[type=number] { width: 3.5rem; }
.elements { display: flex; gap: .4rem; flex-wrap: wrap; }
.elem { padding: .1rem .35rem; background: #2a2a35; border-radius: 4px; font-size: .8rem; font-family: monospace; }
.muted { color: #666; font-style: italic; font-size: .85rem; }
.cards { display: grid; grid-template-columns: 1fr 1fr 1fr; gap: .75rem; margin-top: .5rem; }
.pile h3 { font-size: .9rem; margin: 0 0 .25rem; color: #ccc; }
.pile ul { list-style: none; padding: 0; margin: 0; font-size: .85rem; }
.pile li { display: flex; justify-content: space-between; align-items: center; padding: .15rem 0; border-bottom: 1px solid #2a2a2a; gap: .25rem; }
.pile button { background: #2a2a30; border: 1px solid #444; color: #ccc; cursor: pointer; font-size: .7rem; padding: 0 .3rem; border-radius: 3px; }
.pile button:hover { background: #383840; }
</style>
