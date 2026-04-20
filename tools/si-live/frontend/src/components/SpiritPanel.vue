<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import type { Spirit } from '../types'
import Icon from './Icon.vue'
import PresenceTrack from './PresenceTrack.vue'

const props = defineProps<{ modelValue: Spirit; slug?: string }>()

// Full track data (immutable, from Wiki). Fetched once per slug.
const fullEnergyTrack = ref<string[]>([])
const fullCardplayTrack = ref<string[]>([])
const wikiError = ref<string | null>(null)

async function loadSpiritMeta(slug: string | undefined) {
  if (!slug) return
  try {
    const res = await fetch(`/api/spirit/${encodeURIComponent(slug)}`)
    if (!res.ok) throw new Error(`${res.status}`)
    const data = await res.json()
    fullEnergyTrack.value = data.presence_energy_track || []
    fullCardplayTrack.value = data.presence_cardplay_track || []
  } catch (e) {
    wikiError.value = `spirit metadata lookup failed: ${(e as Error).message}`
  }
}

onMounted(() => loadSpiritMeta(props.slug))
watch(() => props.slug, loadSpiritMeta)

function updateEnergyCovered(next: string[]) {
  props.modelValue.presence_on_track_energy = next
}
function updateCardplayCovered(next: string[]) {
  props.modelValue.presence_on_track_cardplay = next
}

const elements = computed(() => {
  const e = props.modelValue.elements_this_turn ?? {}
  return Object.entries(e).filter(([, n]) => (n as number) > 0)
})

const handCount = computed(() => props.modelValue.hand?.length ?? 0)
const discardCount = computed(() => props.modelValue.discard?.length ?? 0)
const playedCount = computed(() => props.modelValue.played_this_turn?.length ?? 0)
const forgottenCount = computed(() => props.modelValue.forgotten?.length ?? 0)

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

const ELEMENT_ICONS: Record<string, string> = {
  sun: 'element-sun',
  moon: 'element-moon',
  fire: 'element-fire',
  air: 'element-air',
  water: 'element-water',
  earth: 'element-earth',
  plant: 'element-plant',
  animal: 'element-animal',
}
</script>

<template>
  <div class="panel">
    <div class="top">
      <div class="resources">
        <label class="res">
          <span class="res-label">Energy</span>
          <input type="number" v-model.number="modelValue.energy" />
        </label>
        <label class="res">
          <span class="res-label">Card Plays</span>
          <input type="number" min="0" v-model.number="modelValue.card_plays" />
        </label>
      </div>

      <div class="elements">
        <span class="section-label">Elements this turn</span>
        <div class="element-chips">
          <span v-for="[el, n] in elements" :key="el" class="element-chip" :title="el">
            <Icon v-if="ELEMENT_ICONS[el]" :name="ELEMENT_ICONS[el]" :size="14" decorative />
            <span class="el-name">{{ el }}</span>
            <span class="el-count">×{{ n }}</span>
          </span>
          <span v-if="!elements.length" class="muted">no elements tallied</span>
        </div>
      </div>
    </div>

    <div class="tracks">
      <PresenceTrack
        v-if="fullEnergyTrack.length"
        label="Energy"
        :full-track="fullEnergyTrack"
        :covered-tokens="modelValue.presence_on_track_energy ?? []"
        @update:covered-tokens="updateEnergyCovered"
      />
      <PresenceTrack
        v-if="fullCardplayTrack.length"
        label="Card Plays"
        :full-track="fullCardplayTrack"
        :covered-tokens="modelValue.presence_on_track_cardplay ?? []"
        @update:covered-tokens="updateCardplayCovered"
      />
      <div v-if="!fullEnergyTrack.length && !wikiError" class="track-loading">Loading presence tracks…</div>
      <div v-if="wikiError" class="track-error">{{ wikiError }}</div>
    </div>

    <div class="piles">
      <div class="pile">
        <div class="pile-hdr">
          <span class="pile-name">Hand</span>
          <span class="pile-count">{{ handCount }}</span>
        </div>
        <ul>
          <li v-for="c in modelValue.hand ?? []" :key="c">
            <span class="card-name">{{ c }}</span>
            <div class="card-actions">
              <button class="ghost" @click="moveCard(c, 'hand', 'played_this_turn')">Play</button>
              <button class="ghost" @click="moveCard(c, 'hand', 'discard')">Discard</button>
            </div>
          </li>
          <li v-if="!(modelValue.hand ?? []).length" class="empty">hand is empty</li>
        </ul>
      </div>

      <div class="pile">
        <div class="pile-hdr">
          <span class="pile-name">Played this turn</span>
          <span class="pile-count">{{ playedCount }}</span>
        </div>
        <ul>
          <li v-for="c in modelValue.played_this_turn ?? []" :key="c">
            <span class="card-name">{{ c }}</span>
            <div class="card-actions">
              <button class="ghost" @click="moveCard(c, 'played_this_turn', 'discard')">→ Discard</button>
              <button class="ghost" @click="moveCard(c, 'played_this_turn', 'hand')">↩ Hand</button>
            </div>
          </li>
          <li v-if="!(modelValue.played_this_turn ?? []).length" class="empty">no plays yet this turn</li>
        </ul>
      </div>

      <div class="pile">
        <div class="pile-hdr">
          <span class="pile-name">Discard</span>
          <span class="pile-count">{{ discardCount }}</span>
        </div>
        <ul>
          <li v-for="c in modelValue.discard ?? []" :key="c">
            <span class="card-name">{{ c }}</span>
            <div class="card-actions">
              <button class="ghost" @click="moveCard(c, 'discard', 'hand')">↩ Hand</button>
            </div>
          </li>
          <li v-if="!(modelValue.discard ?? []).length" class="empty">discard empty</li>
        </ul>
      </div>

      <div v-if="forgottenCount > 0" class="pile forgotten">
        <div class="pile-hdr">
          <span class="pile-name">Forgotten</span>
          <span class="pile-count">{{ forgottenCount }}</span>
        </div>
        <ul>
          <li v-for="c in modelValue.forgotten ?? []" :key="c" class="empty">{{ c }}</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<style scoped>
.panel {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-lg);
  padding: var(--sp-4);
}

.top {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: var(--sp-4);
  align-items: start;
}

.resources {
  display: inline-flex;
  gap: var(--sp-3);
}

.res {
  display: flex;
  flex-direction: column;
  gap: var(--sp-1);
}

.res-label, .section-label, .track-label {
  font-size: 0.68rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
}

.elements {
  display: flex;
  flex-direction: column;
  gap: var(--sp-1);
}

.element-chips {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sp-1);
  align-items: center;
  min-height: 1.75rem;
}

.element-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 2px var(--sp-2);
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-full);
  font-size: var(--fs-xs);
}

.el-name {
  text-transform: capitalize;
  color: var(--text-secondary);
}

.el-count {
  font-family: var(--font-mono);
  font-weight: var(--fw-semibold);
  color: var(--text-primary);
}

.tracks {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
  padding: var(--sp-3) 0;
  border-top: 1px solid var(--border-subtle);
  border-bottom: 1px solid var(--border-subtle);
}

.track-loading, .track-error {
  font-size: var(--fs-xs);
  color: var(--text-muted);
  font-style: italic;
}
.track-error { color: var(--status-danger); font-style: normal; }

.piles {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--sp-3);
}

.pile {
  display: flex;
  flex-direction: column;
  gap: var(--sp-1);
}

.pile.forgotten { opacity: 0.6; }

.pile-hdr {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  padding-bottom: var(--sp-1);
  border-bottom: 1px solid var(--border-subtle);
}

.pile-name {
  font-size: var(--fs-sm);
  font-weight: var(--fw-semibold);
  color: var(--text-primary);
}

.pile-count {
  font-family: var(--font-mono);
  font-size: var(--fs-xs);
  color: var(--text-muted);
}

.pile ul {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.pile li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-1);
  font-size: var(--fs-xs);
  border-radius: var(--r-sm);
  transition: background var(--motion-fast);
}

.pile li:hover:not(.empty) { background: var(--bg-muted); }

.card-name { color: var(--text-primary); flex: 1; }
.empty { color: var(--text-faint); font-style: italic; justify-content: center; }

.card-actions {
  display: inline-flex;
  gap: 2px;
}

.card-actions button {
  font-size: 0.68rem;
  padding: 1px var(--sp-1);
}

.muted { color: var(--text-muted); font-style: italic; font-size: var(--fs-xs); }
</style>
