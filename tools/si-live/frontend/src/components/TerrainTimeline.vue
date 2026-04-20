<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import type { GameState } from '../types'
import { projectExposure, labelFor, type TurnExposure } from '../lib/terrain-exposure'
import { fetchSpiritAffinity, type SpiritAffinityMap, type SpiritAffinityEntry } from '../api'

/**
 * Terrain-exposure timeline.
 *
 * Projects the upcoming invader stack forward and flags when the same terrain
 * appears in ≥2 phases the same turn (doubling) — a concentration event the
 * modelling layer needs to treat specially.
 */

const props = defineProps<{ state: GameState }>()

const TURNS_TO_PROJECT = 5

const timeline = computed<TurnExposure[]>(() =>
  projectExposure(props.state.invader_deck, props.state.round, TURNS_TO_PROJECT),
)

// --- affinity lookup -------------------------------------------------------
const affinityMap = ref<SpiritAffinityMap | null>(null)
const affinityError = ref<string | null>(null)
onMounted(async () => {
  try {
    affinityMap.value = await fetchSpiritAffinity()
  } catch (e) {
    affinityError.value = (e as Error).message
  }
})

function affinityForSpirit(slug: string): SpiritAffinityEntry | null {
  if (!affinityMap.value) return null
  return affinityMap.value.spirits[slug] ?? affinityMap.value.default
}

interface ImpactForSpirit {
  slug: string
  score: number
  concentration: number
  rationale: string
}

function impactForTurn(exposure: TurnExposure): ImpactForSpirit[] {
  if (!affinityMap.value) return []
  const spirits = Object.keys(props.state.spirits ?? {})
  return spirits.map(slug => {
    const aff = affinityForSpirit(slug)
    if (!aff) return { slug, score: 0, concentration: 0, rationale: '' }
    const touched = new Set([...exposure.ravaged, ...exposure.built, ...exposure.explored])
    let affSum = 0
    let affCount = 0
    for (const t of touched) {
      const v = aff.terrain_affinity[t]
      if (typeof v === 'number') {
        affSum += v
        affCount++
      }
    }
    const avgAff = affCount ? affSum / affCount : 0
    const concentrationPressure = exposure.doubled.length + 2 * exposure.tripled.length
    const score = clamp(0.5 * avgAff + aff.concentration * concentrationPressure * 0.4, -1, 1)
    return {
      slug,
      score,
      concentration: aff.concentration * concentrationPressure,
      rationale: aff.rationale ?? '',
    }
  })
}

function clamp(n: number, lo: number, hi: number): number {
  return Math.min(hi, Math.max(lo, n))
}

function scoreLabel(s: number): string {
  if (s > 0.3) return '++'
  if (s > 0.1) return '+'
  if (s < -0.3) return '--'
  if (s < -0.1) return '-'
  return ''
}

function scoreClass(s: number): string {
  if (s > 0.1) return 'positive'
  if (s < -0.1) return 'negative'
  return 'neutral'
}

function terrainClass(t: string): string {
  const slug = t.toLowerCase().replace(/\s+/g, '-')
  return `terrain-${slug}`
}

const hasDoubles = computed(() => timeline.value.some(t => t.doubled.length + t.tripled.length > 0))

function humanSlug(slug: string): string {
  return slug.split('-').map(p => p.charAt(0).toUpperCase() + p.slice(1)).join(' ')
}
</script>

<template>
  <div class="terrain-timeline">
    <div class="timeline-intro">
      <span class="timeline-hint">Next {{ TURNS_TO_PROJECT }} turns - flags terrain doublings</span>
    </div>

    <div v-if="affinityError" class="error-msg">Affinity data unavailable: {{ affinityError }}</div>

    <div v-if="!hasDoubles" class="empty-msg">
      No terrain doublings in the next {{ TURNS_TO_PROJECT }} turns.
    </div>

    <div class="timeline-grid">
      <div
        v-for="t in timeline"
        :key="t.turn"
        class="turn-row"
        :class="{ 'has-double': t.doubled.length, 'has-triple': t.tripled.length }"
      >
        <div class="turn-num">T{{ t.turn }}</div>

        <div class="phases-row">
          <div class="phase ravage">
            <span class="phase-label">Rav</span>
            <span v-if="t.ravaged.length" class="terrain-chips">
              <span v-for="terr in t.ravaged" :key="terr" class="terrain-chip" :class="terrainClass(terr)">{{ labelFor(terr) }}</span>
            </span>
            <span v-else class="empty-phase">-</span>
          </div>
          <div class="phase build">
            <span class="phase-label">Bld</span>
            <span v-if="t.built.length" class="terrain-chips">
              <span v-for="terr in t.built" :key="terr" class="terrain-chip" :class="terrainClass(terr)">{{ labelFor(terr) }}</span>
            </span>
            <span v-else class="empty-phase">-</span>
          </div>
          <div class="phase explore">
            <span class="phase-label">Exp</span>
            <span v-if="t.explored.length" class="terrain-chips">
              <span v-for="terr in t.explored" :key="terr" class="terrain-chip" :class="terrainClass(terr)">{{ labelFor(terr) }}</span>
            </span>
            <span v-else class="empty-phase">-</span>
          </div>
        </div>

        <div class="flags-row">
          <span v-for="terr in t.tripled" :key="`trip-${terr}`" class="flag-badge triple">
            3x {{ labelFor(terr) }}
          </span>
          <span v-for="terr in t.doubled" :key="`dbl-${terr}`" class="flag-badge double">
            2x {{ labelFor(terr) }}
          </span>
        </div>

        <div class="impact-row" v-if="affinityMap">
          <span
            v-for="sp in impactForTurn(t)"
            :key="sp.slug"
            class="impact-badge"
            :class="scoreClass(sp.score)"
            :title="`${humanSlug(sp.slug)}: ${sp.score.toFixed(2)}\n${sp.rationale}`"
          >
            {{ humanSlug(sp.slug).split(' ')[0] }} {{ scoreLabel(sp.score) }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.terrain-timeline {
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
  padding: var(--sp-4);
}

.timeline-intro {
  display: flex;
  align-items: center;
  gap: var(--sp-3);
}

.timeline-hint {
  font-size: var(--text-xs);
  color: var(--text-muted);
  font-style: italic;
}

.empty-msg {
  padding: var(--sp-3);
  font-size: var(--text-xs);
  color: var(--text-muted);
  font-style: italic;
  text-align: center;
}

.error-msg {
  padding: var(--sp-2) var(--sp-3);
  font-size: var(--text-xs);
  color: var(--color-danger);
  background: rgba(239, 68, 68, 0.1);
  border-radius: var(--radius-sm);
}

.timeline-grid {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}

.turn-row {
  display: grid;
  grid-template-columns: 40px 1fr auto auto;
  gap: var(--sp-3);
  align-items: center;
  padding: var(--sp-2) var(--sp-3);
  background: var(--bg-muted);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
}

.turn-row.has-double {
  border-left: 3px solid var(--color-warning);
}

.turn-row.has-triple {
  border-left: 3px solid var(--color-danger);
}

.turn-num {
  font-family: var(--font-mono);
  font-weight: var(--weight-bold);
  color: var(--color-warning);
}

.phases-row {
  display: flex;
  gap: var(--sp-4);
}

.phase {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  font-size: var(--text-xs);
}

.phase-label {
  font-family: var(--font-mono);
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  font-weight: var(--weight-medium);
  width: 28px;
}

.phase.ravage .phase-label { color: var(--color-danger); }
.phase.build .phase-label { color: var(--color-warning); }
.phase.explore .phase-label { color: var(--color-accent); }

.terrain-chips {
  display: flex;
  gap: var(--sp-1);
  flex-wrap: wrap;
}

.empty-phase {
  color: var(--text-faint);
}

.terrain-chip {
  display: inline-block;
  padding: 2px var(--sp-2);
  font-size: 10px;
  border-radius: var(--radius-sm);
  font-weight: var(--weight-medium);
  border: 1px solid var(--border-subtle);
  background: var(--bg-subtle);
  color: var(--text-secondary);
}

.terrain-chip.terrain-mountain { background: rgba(184, 134, 11, 0.15); color: #d4a444; }
.terrain-chip.terrain-jungle { background: rgba(82, 183, 136, 0.15); color: #52b788; }
.terrain-chip.terrain-sands { background: rgba(233, 196, 106, 0.15); color: #e9c46a; }
.terrain-chip.terrain-wetland { background: rgba(123, 184, 245, 0.15); color: #7bb8f5; }
.terrain-chip.terrain-coastal-lands { background: rgba(0, 184, 212, 0.15); color: #00b8d4; }

.flags-row {
  display: flex;
  gap: var(--sp-1);
  flex-wrap: wrap;
}

.flag-badge {
  padding: 2px var(--sp-2);
  font-size: 10px;
  letter-spacing: 0.05em;
  font-family: var(--font-mono);
  font-weight: var(--weight-bold);
  border-radius: var(--radius-sm);
}

.flag-badge.double {
  background: rgba(234, 179, 8, 0.15);
  color: var(--color-warning);
  border: 1px solid rgba(234, 179, 8, 0.3);
}

.flag-badge.triple {
  background: rgba(239, 68, 68, 0.15);
  color: var(--color-danger);
  border: 1px solid rgba(239, 68, 68, 0.3);
}

.impact-row {
  display: flex;
  flex-wrap: wrap;
  gap: var(--sp-1);
  justify-content: flex-end;
}

.impact-badge {
  display: inline-flex;
  padding: 2px var(--sp-2);
  font-size: 10px;
  font-family: var(--font-mono);
  border-radius: 10px;
  border: 1px solid var(--border-subtle);
  background: var(--bg-subtle);
  color: var(--text-muted);
}

.impact-badge.positive {
  color: var(--color-success);
  background: rgba(34, 197, 94, 0.1);
  border-color: rgba(34, 197, 94, 0.2);
}

.impact-badge.negative {
  color: var(--color-danger);
  background: rgba(239, 68, 68, 0.1);
  border-color: rgba(239, 68, 68, 0.2);
}

@media (max-width: 700px) {
  .turn-row {
    grid-template-columns: 40px 1fr;
    grid-template-rows: auto auto auto;
  }
  
  .flags-row,
  .impact-row {
    grid-column: 2;
  }
  
  .phases-row {
    flex-direction: column;
    gap: var(--sp-1);
  }
}
</style>
