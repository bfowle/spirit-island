<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import type { Pools } from '../types'
import { fetchDeck } from '../api'
import Icon from './Icon.vue'

interface BlightCardData {
  name: string
  expansion?: string
  cardtype?: string
  blight_per_player?: string
  text?: string
}

const props = defineProps<{ modelValue: Pools; playerCount?: number }>()
defineEmits<{ 'update:modelValue': [value: Pools] }>()

const blightCards = ref<BlightCardData[]>([])
onMounted(async () => {
  try {
    blightCards.value = await fetchDeck<BlightCardData>('blight')
  } catch { /* best-effort */ }
})

const blightQ = ref('')
const blightSuggestions = computed(() => {
  const q = blightQ.value.trim().toLowerCase()
  if (!q) return blightCards.value.slice(0, 8)
  return blightCards.value.filter(c => c.name.toLowerCase().includes(q)).slice(0, 10)
})

const selectedCard = computed<BlightCardData | null>(() => {
  if (!props.modelValue.blight_card) return null
  return blightCards.value.find(c => c.name === props.modelValue.blight_card) ?? null
})

function pickBlightCard(card: BlightCardData) {
  props.modelValue.blight_card = card.name
  // Auto-set blight cap based on card's blight_per_player × player count
  const per = parseInt(card.blight_per_player ?? '2', 10) || 2
  const players = props.playerCount ?? 1
  props.modelValue.blight_cap = per * players
  blightQ.value = ''
}

function clearBlightCard() {
  props.modelValue.blight_card = null
  blightQ.value = ''
}

function toggleFlipped() {
  props.modelValue.island_blighted = !props.modelValue.island_blighted
}
</script>

<template>
  <section class="pools">
    <div class="pool fear">
      <div class="pool-hdr">
        <Icon name="resource-fear" :size="16" decorative />
        <span class="pool-name">Fear</span>
        <span class="terror">Terror {{ modelValue.terror_level }}</span>
      </div>
      <div class="pool-values">
        <input type="number" min="0" v-model.number="modelValue.fear_current" />
        <span class="sep">/</span>
        <input type="number" min="0" v-model.number="modelValue.fear_threshold" />
      </div>
    </div>

    <div class="pool blight" :class="{ flipped: modelValue.island_blighted }">
      <div class="pool-hdr">
        <Icon name="resource-blight" :size="16" decorative />
        <span class="pool-name">Blight</span>
        <span v-if="modelValue.island_blighted" class="flipped-tag">🌑 BLIGHTED ISLAND</span>
      </div>
      <div class="pool-values">
        <input type="number" min="0" v-model.number="modelValue.blight_current" />
        <span class="sep">/</span>
        <input type="number" min="0" v-model.number="modelValue.blight_cap" />
      </div>

      <!-- Blight card selection -->
      <div class="blight-card-section">
        <div v-if="!selectedCard" class="card-picker">
          <span class="field-label">Blight card:</span>
          <div class="autocomplete">
            <input
              type="text"
              v-model="blightQ"
              placeholder="pick the drawn blight card…"
            />
            <div v-if="blightQ && blightSuggestions.length" class="suggestions">
              <button
                v-for="c in blightSuggestions"
                :key="c.name"
                type="button"
                class="suggestion"
                @click="pickBlightCard(c)"
              >
                <span class="sug-name">{{ c.name }}</span>
                <span class="sug-meta">
                  <span class="sug-type" :class="c.cardtype?.toLowerCase().replace(/\s+/g, '-')">
                    {{ c.cardtype }}
                  </span>
                  <span class="sug-per">×{{ c.blight_per_player }}/p</span>
                </span>
              </button>
            </div>
          </div>
        </div>
        <div v-else class="card-detail" :class="modelValue.island_blighted ? 'active-blighted' : 'active-healthy'">
          <div class="card-hdr">
            <div class="card-title-block">
              <span class="card-title">{{ selectedCard.name }}</span>
              <span v-if="selectedCard.expansion" class="card-exp">{{ selectedCard.expansion }}</span>
              <span class="card-type-tag" :class="selectedCard.cardtype?.toLowerCase().replace(/\s+/g, '-')">
                {{ selectedCard.cardtype }}
              </span>
            </div>
            <div class="card-actions">
              <button
                class="primary"
                @click="toggleFlipped"
                :title="modelValue.island_blighted
                  ? 'Flip back to the Healthy Island side'
                  : 'Flip the island to the Blighted side (triggers the card effect)'"
              >
                {{ modelValue.island_blighted ? '↩ Flip to Healthy' : '🌑 Flip to Blighted' }}
              </button>
              <button class="ghost tiny" @click="clearBlightCard" title="Change the selected Blight card">Change</button>
            </div>
          </div>
          <div class="card-text">
            <span class="text-side-label">
              {{ modelValue.island_blighted ? 'Blighted-side effect:' : 'Flip trigger (on blight depletion):' }}
            </span>
            {{ selectedCard.text }}
          </div>
          <div class="card-math">
            Cap: {{ selectedCard.blight_per_player }} per player × {{ playerCount ?? 1 }} = <strong>{{ modelValue.blight_cap }}</strong>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.pools {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: var(--sp-3);
}

.pool {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-lg);
  padding: var(--sp-3);
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
  transition: background var(--motion-base);
}
.pool.blight.flipped {
  background: linear-gradient(180deg, rgba(199, 125, 255, 0.05) 0%, var(--bg-surface) 100%);
  border-color: rgba(199, 125, 255, 0.4);
}

.pool-hdr {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  font-size: var(--fs-sm);
}

.pool-name {
  font-weight: var(--fw-semibold);
  color: var(--text-primary);
}

.terror {
  margin-left: auto;
  font-size: var(--fs-xs);
  color: var(--pool-fear);
  padding: 1px var(--sp-2);
  background: var(--accent-soft);
  border-radius: var(--r-full);
  font-weight: var(--fw-medium);
}

.flipped-tag {
  margin-left: auto;
  font-size: 0.68rem;
  color: var(--accent-purple);
  padding: 1px var(--sp-2);
  background: rgba(199, 125, 255, 0.15);
  border-radius: var(--r-full);
  font-weight: var(--fw-bold);
  letter-spacing: 0.08em;
  font-family: var(--font-mono);
}

.pool-values {
  display: inline-flex;
  align-items: center;
  gap: var(--sp-2);
}
.sep { color: var(--text-muted); font-family: var(--font-mono); }

.blight-card-section {
  margin-top: var(--sp-1);
  padding-top: var(--sp-2);
  border-top: 1px dashed var(--border-subtle);
}

.card-picker {
  display: flex; align-items: center; gap: var(--sp-2);
}
.field-label {
  font-size: 0.68rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  white-space: nowrap;
}

.autocomplete { position: relative; flex: 1; }
.autocomplete input { width: 100%; }
.suggestions {
  position: absolute; top: calc(100% + 4px); left: 0; right: 0;
  z-index: 25;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  box-shadow: 0 4px 14px rgba(0, 0, 0, 0.4);
  max-height: 18rem; overflow-y: auto;
}
.suggestion {
  width: 100%;
  display: flex; justify-content: space-between; gap: var(--sp-2);
  padding: var(--sp-1) var(--sp-2);
  text-align: left;
  border: none; background: transparent; color: inherit;
  cursor: pointer;
  font-size: var(--fs-xs);
}
.suggestion:hover { background: var(--bg-muted); }
.sug-name { font-weight: var(--fw-medium); }
.sug-meta { display: inline-flex; gap: 4px; align-items: baseline; }
.sug-type, .card-type-tag {
  font-family: var(--font-mono);
  font-size: 0.62rem;
  padding: 0 4px;
  border-radius: var(--r-sm);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  font-weight: var(--fw-bold);
}
.sug-type.blighted, .card-type-tag.blighted {
  background: rgba(199, 125, 255, 0.2); color: var(--accent-purple);
}
.sug-type.still-healthy, .sug-type.still-healthy-island,
.card-type-tag.still-healthy, .card-type-tag.still-healthy-island {
  background: rgba(82, 183, 136, 0.2); color: var(--status-success);
}
.sug-per { color: var(--text-muted); font-size: 0.68rem; }

.card-detail {
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: var(--sp-2) var(--sp-3);
  display: flex; flex-direction: column; gap: var(--sp-2);
  transition: all var(--motion-base);
}
.card-detail.active-blighted {
  border-color: rgba(199, 125, 255, 0.5);
  background: rgba(199, 125, 255, 0.08);
}

.card-hdr {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: var(--sp-2);
  align-items: start;
}
.card-title-block { display: flex; flex-wrap: wrap; gap: var(--sp-2); align-items: baseline; }
.card-title { font-weight: var(--fw-semibold); color: var(--text-white); }
.card-exp { font-size: var(--fs-xs); color: var(--text-muted); font-style: italic; }
.card-actions { display: inline-flex; gap: 4px; align-items: center; }

.card-text {
  font-size: var(--fs-xs);
  line-height: 1.5;
  color: var(--text-secondary);
}
.text-side-label {
  display: block;
  font-size: 0.62rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  font-weight: var(--fw-bold);
  margin-bottom: 2px;
}

.card-math {
  font-size: var(--fs-xs);
  color: var(--text-muted);
  font-family: var(--font-mono);
}
.card-math strong { color: var(--text-primary); }

.tiny { padding: 2px var(--sp-1); font-size: var(--fs-xs); }
</style>
