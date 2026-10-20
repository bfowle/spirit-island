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
 *
 * Per-spirit impact assessment cross-references data/spirit-terrain-affinity.json
 * to indicate whether each concentration event helps or hurts the spirit.
 * Example: Shadows-flicker-like-flame has a positive concentration factor → a
 * Jungle double-up in T3/T4 is net-favorable. Mud-of-the-Swamp, in contrast,
 * has negative concentration → the same event is net-unfavorable.
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
  score: number      // > 0 = good for spirit; < 0 = bad
  concentration: number
  rationale: string
}

/**
 * Compute a per-spirit impact score for a given turn's exposure. Considers:
 *   - terrain_affinity averaged across touched terrains
 *   - concentration modifier multiplied by (doubled + 2*tripled) terrain count
 * Scores capped to roughly [-1, 1] for display.
 */
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
  if (s < -0.3) return '−−'
  if (s < -0.1) return '−'
  return '·'
}
function scoreClass(s: number): string {
  if (s > 0.1) return 'pos'
  if (s < -0.1) return 'neg'
  return 'neu'
}

function terrainChipClass(t: string): string {
  const slug = t.toLowerCase().replace(/\s+/g, '-')
  return `terrain-chip t-${slug}`
}

const hasDoubles = computed(() => timeline.value.some(t => t.doubled.length + t.tripled.length > 0))

function humanSlug(slug: string): string {
  return slug.split('-').map(p => p.charAt(0).toUpperCase() + p.slice(1)).join(' ')
}
</script>

<template>
  <div class="terrain-timeline card">
    <div class="hdr">
      <h3>Terrain Exposure Timeline</h3>
      <span class="subtle">
        next {{ TURNS_TO_PROJECT }} turns · flags terrain doublings that bend win-probability modelling
      </span>
    </div>

    <div v-if="affinityError" class="banner error">Affinity data unavailable: {{ affinityError }}</div>

    <div v-if="!hasDoubles" class="empty">
      No terrain doublings in the next {{ TURNS_TO_PROJECT }} turns. Straight progression expected.
    </div>

    <div class="timeline">
      <div
        v-for="t in timeline"
        :key="t.turn"
        class="turn-row"
        :class="{ 'has-double': t.doubled.length, 'has-triple': t.tripled.length }"
      >
        <div class="turn-num mono">T{{ t.turn }}</div>

        <div class="phases">
          <div class="phase ravage">
            <span class="phase-label">Rav</span>
            <span v-if="t.ravaged.length" class="terrain-chips">
              <span v-for="terr in t.ravaged" :key="terr" :class="terrainChipClass(terr)">{{ labelFor(terr) }}</span>
            </span>
            <span v-else class="empty-phase">—</span>
          </div>
          <div class="phase build">
            <span class="phase-label">Bld</span>
            <span v-if="t.built.length" class="terrain-chips">
              <span v-for="terr in t.built" :key="terr" :class="terrainChipClass(terr)">{{ labelFor(terr) }}</span>
            </span>
            <span v-else class="empty-phase">—</span>
          </div>
          <div class="phase explore">
            <span class="phase-label">Exp</span>
            <span v-if="t.explored.length" class="terrain-chips">
              <span v-for="terr in t.explored" :key="terr" :class="terrainChipClass(terr)">{{ labelFor(terr) }}</span>
            </span>
            <span v-else class="empty-phase">—</span>
          </div>
        </div>

        <div class="flags">
          <span v-for="terr in t.tripled" :key="`trip-${terr}`" class="double-flag triple">
            TRIPLE {{ labelFor(terr) }}
          </span>
          <span v-for="terr in t.doubled" :key="`dbl-${terr}`" class="double-flag">
            DOUBLE {{ labelFor(terr) }}
          </span>
        </div>

        <div class="spirit-impact" v-if="affinityMap">
          <span
            v-for="sp in impactForTurn(t)"
            :key="sp.slug"
            class="impact-pill"
            :class="scoreClass(sp.score)"
            :title="`${humanSlug(sp.slug)}: score ${sp.score.toFixed(2)}\n${sp.rationale}`"
          >
            {{ humanSlug(sp.slug).split(' ')[0] }} {{ scoreLabel(sp.score) }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.terrain-timeline { display: flex; flex-direction: column; gap: var(--sp-3); }

.hdr { display: flex; align-items: baseline; gap: var(--sp-3); flex-wrap: wrap; }
.hdr h3 { color: var(--text-white); margin: 0; }
.hdr .subtle { font-size: var(--fs-xs); color: var(--text-muted); font-style: italic; }

.empty {
  padding: var(--sp-2) var(--sp-3);
  font-size: var(--fs-xs);
  color: var(--text-muted);
  font-style: italic;
}
.banner.error {
  padding: var(--sp-2) var(--sp-3);
  font-size: var(--fs-xs);
  color: var(--status-danger);
  background: rgba(184, 113, 106, 0.1);
  border-radius: var(--r-sm);
}

.timeline { display: flex; flex-direction: column; gap: 4px; }

.turn-row {
  display: grid;
  grid-template-columns: 2.5rem 1fr auto;
  gap: var(--sp-2);
  align-items: center;
  padding: var(--sp-2) var(--sp-3);
  background: var(--bg-inset);
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  font-size: var(--fs-sm);
}
.turn-row.has-double { border-left: 3px solid var(--accent-amber); }
.turn-row.has-triple { border-left: 3px solid var(--status-danger); }

.turn-num {
  color: var(--accent-amber);
  font-weight: var(--fw-bold);
  font-family: var(--font-mono);
}

.phases {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--sp-1);
}
@media (max-width: 680px) { .phases { grid-template-columns: 1fr; } }

.phase {
  display: grid;
  grid-template-columns: 2rem 1fr;
  gap: var(--sp-1);
  align-items: center;
  font-size: var(--fs-xs);
}
.phase-label {
  font-family: var(--font-mono);
  font-size: 0.68rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-muted);
  font-weight: var(--fw-medium);
}
.phase.ravage .phase-label  { color: var(--accent-red); }
.phase.build .phase-label   { color: var(--accent-amber); }
.phase.explore .phase-label { color: var(--accent-blue); }

.terrain-chips { display: inline-flex; gap: 4px; flex-wrap: wrap; }
.empty-phase { color: var(--text-muted); }

.terrain-chip {
  display: inline-block;
  padding: 1px 6px;
  font-size: 0.68rem;
  border-radius: var(--r-sm);
  font-weight: var(--fw-medium);
  border: 1px solid var(--border-subtle);
}
.terrain-chip.t-mountain { background: rgba(184, 134, 11, 0.12); color: #d4a444; }
.terrain-chip.t-jungle   { background: rgba(82, 183, 136, 0.15); color: #52b788; }
.terrain-chip.t-sands    { background: rgba(233, 196, 106, 0.15); color: #e9c46a; }
.terrain-chip.t-wetland  { background: rgba(123, 184, 245, 0.15); color: #7bb8f5; }
.terrain-chip.t-coastal-lands { background: rgba(0, 184, 212, 0.15); color: #00b8d4; }

.flags { display: inline-flex; gap: 4px; flex-wrap: wrap; grid-column: 2 / 3; }
.double-flag {
  padding: 1px 6px;
  font-size: 0.62rem;
  letter-spacing: 0.08em;
  font-family: var(--font-mono);
  font-weight: var(--fw-bold);
  border-radius: var(--r-sm);
  background: rgba(233, 196, 106, 0.15);
  color: var(--accent-amber);
  border: 1px solid rgba(233, 196, 106, 0.4);
}
.double-flag.triple {
  background: rgba(220, 47, 2, 0.2);
  color: var(--accent-red);
  border-color: rgba(220, 47, 2, 0.5);
}

.spirit-impact {
  grid-column: 3 / 4;
  display: inline-flex; flex-wrap: wrap; gap: 4px;
  justify-content: flex-end;
}
.impact-pill {
  display: inline-flex;
  padding: 1px 6px;
  font-size: 0.66rem;
  font-family: var(--font-mono);
  border-radius: var(--r-full);
  border: 1px solid var(--border-subtle);
}
.impact-pill.pos { color: var(--status-success); background: rgba(82, 183, 136, 0.1); border-color: rgba(82, 183, 136, 0.3); }
.impact-pill.neg { color: var(--status-danger); background: rgba(220, 47, 2, 0.12); border-color: rgba(220, 47, 2, 0.4); }
.impact-pill.neu { color: var(--text-muted); }
</style>
