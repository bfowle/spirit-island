<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import type { GameState } from '../types'
import ExpansionBadge from './ExpansionBadge.vue'

interface BoardSummary {
  file: string
  board_id: string
  expansion: string
  variants: { key: string; name: string }[]
}

interface AspectOption {
  key: string
  name: string
  expansion?: string
  complexity_change?: string
}

interface RegistryResponse {
  spirits: { spirits: { slug: string; name: string; expansion: string; complexity?: string }[] }
  adversaries: { adversaries: { slug: string; name: string; expansion: string; difficulty_levels: number[] }[] }
  scenarios: { scenarios: { slug: string; name: string; expansion: string }[] }
  boards: BoardSummary[]
  aspects?: Record<string, AspectOption[]>
}

const emit = defineEmits<{ 'game-started': [state: GameState]; close: [] }>()
defineProps<{ show: boolean }>()

const registry = ref<RegistryResponse | null>(null)
const error = ref<string | null>(null)

const adversary = ref<string>('england')
const level = ref<number>(3)
const scenario = ref<string>('')  // empty = no scenario
const selectedSpirits = ref<string[]>(['shadows-flicker-like-flame'])
const selectedBoards = ref<string[]>(['A'])
const expansions = ref<string[]>(['base'])
const archiveCurrent = ref<boolean>(true)
const boardVariant = ref<'balanced' | 'thematic'>('balanced')
/** Per-spirit aspect selection. Keyed by spirit slug; empty string = base. */
const selectedAspects = ref<Record<string, string>>({})

function aspectsFor(slug: string): AspectOption[] {
  return registry.value?.aspects?.[slug] ?? []
}

// Display order for expansions across any spirit/board grouping.
// Horizons sits at the bottom because it's the beginner/onboarding set —
// serious drafters usually scroll past it, so keeping it out of the
// top-of-list real estate is more helpful.
const EXPANSION_ORDER = [
  'base',
  'branch-and-claw',
  'jagged-earth',
  'nature-incarnate',
  'promo-1',
  'promo-2',
  'horizons',
  'horizons-of-spirit-island',
  'hosi',
] as const

function expansionRank(slug: string | null | undefined): number {
  if (!slug) return EXPANSION_ORDER.length
  const normalized = slug.toLowerCase().replace(/_/g, '-')
  const idx = EXPANSION_ORDER.indexOf(normalized as (typeof EXPANSION_ORDER)[number])
  return idx === -1 ? EXPANSION_ORDER.length - 1 : idx
}

interface SpiritGroup {
  expansion: string
  spirits: { slug: string; name: string; expansion: string; complexity?: string }[]
}

const spiritGroups = computed<SpiritGroup[]>(() => {
  const all = registry.value?.spirits.spirits ?? []
  const byExp = new Map<string, SpiritGroup>()
  for (const s of all) {
    const exp = s.expansion || 'base'
    if (!byExp.has(exp)) byExp.set(exp, { expansion: exp, spirits: [] })
    byExp.get(exp)!.spirits.push(s)
  }
  // Sort spirits alphabetically inside each group, and groups by EXPANSION_ORDER.
  for (const g of byExp.values()) {
    g.spirits.sort((a, b) => a.name.localeCompare(b.name))
  }
  return [...byExp.values()].sort((a, b) => expansionRank(a.expansion) - expansionRank(b.expansion))
})

onMounted(async () => {
  try {
    const res = await fetch('/api/registry')
    if (!res.ok) throw new Error(`GET /api/registry → ${res.status}`)
    registry.value = await res.json()
  } catch (e) {
    error.value = (e as Error).message
  }
})

interface LevelOption {
  level: number
  rating: number
  label: string
}

const availableLevels = computed((): LevelOption[] => {
  const adv = registry.value?.adversaries.adversaries.find(a => a.slug === adversary.value)
  const ratings = adv?.difficulty_levels ?? []
  const out: LevelOption[] = [{ level: 0, rating: 0, label: 'L0 (base, difficulty 0)' }]
  for (let i = 0; i < Math.min(ratings.length, 6); i++) {
    out.push({
      level: i + 1,
      rating: ratings[i],
      label: `L${i + 1} (difficulty ${ratings[i]})`,
    })
  }
  return out
})

const availableBoards = computed((): BoardSummary[] => {
  if (!registry.value) return []
  return [...registry.value.boards].sort((a, b) => a.board_id.localeCompare(b.board_id))
})

const variantAvailableForAllSelected = computed(() => {
  if (!registry.value) return { balanced: true, thematic: true }
  const hasVariant = (key: string) =>
    selectedBoards.value.every(bid => {
      const b = registry.value!.boards.find(x => x.board_id === bid)
      return !b || b.variants.some(v => v.key === key)
    })
  return { balanced: hasVariant('balanced'), thematic: hasVariant('thematic') }
})

function toggleSpirit(slug: string) {
  const i = selectedSpirits.value.indexOf(slug)
  if (i >= 0) selectedSpirits.value.splice(i, 1)
  else selectedSpirits.value.push(slug)
}

function toggleBoard(b: string) {
  const i = selectedBoards.value.indexOf(b)
  if (i >= 0) selectedBoards.value.splice(i, 1)
  else selectedBoards.value.push(b)
}

function toggleExpansion(e: string) {
  const i = expansions.value.indexOf(e)
  if (i >= 0) expansions.value.splice(i, 1)
  else expansions.value.push(e)
}

async function startGame() {
  error.value = null
  try {
    if (archiveCurrent.value) {
      try {
        await fetch('/api/saved-games/archive', { method: 'POST' })
      } catch (e) {
        console.warn('archive current game failed', e)
      }
    }
    // Drop aspects for spirits that aren't selected, or whose value is empty.
    const aspectsPayload: Record<string, string> = {}
    for (const slug of selectedSpirits.value) {
      const a = selectedAspects.value[slug]
      if (a) aspectsPayload[slug] = a
    }
    const res = await fetch('/api/new-game', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        board_variant: boardVariant.value,
        adversary: adversary.value || null,
        level: level.value,
        scenario: scenario.value || null,
        spirits: selectedSpirits.value,
        boards: selectedBoards.value,
        expansions_active: expansions.value,
        aspects: aspectsPayload,
      }),
    })
    if (!res.ok) {
      const txt = await res.text()
      throw new Error(`POST /api/new-game → ${res.status}: ${txt}`)
    }
    const state = await res.json()
    emit('game-started', state)
    emit('close')
  } catch (e) {
    error.value = (e as Error).message
  }
}
</script>

<template>
  <div v-if="show" class="aegis-overlay" @click.self="emit('close')">
    <div class="aegis-dialog wizard">
      <div class="aegis-dialog-head">
        <h2 class="aegis-dialog-title">New Game Setup</h2>
        <span class="head-spacer" />
        <button class="ghost size-icon" @click="emit('close')" aria-label="close">×</button>
      </div>

      <div class="aegis-dialog-body">
        <div v-if="error" class="error">{{ error }}</div>
        <div v-else-if="!registry" class="aegis-empty">
          <div class="aegis-empty-title">Loading registry…</div>
        </div>
        <div v-else class="form">
          <section class="grid-2">
            <label class="field">
              <span class="aegis-section-hdr">Adversary</span>
              <select v-model="adversary" class="aegis-input">
                <option value="">(none)</option>
                <option v-for="a in registry.adversaries.adversaries" :key="a.slug" :value="a.slug">
                  {{ a.name }}
                </option>
              </select>
            </label>
            <label class="field">
              <span class="aegis-section-hdr">Level</span>
              <select v-model.number="level" class="aegis-input">
                <option v-for="l in availableLevels" :key="l.level" :value="l.level">{{ l.label }}</option>
              </select>
            </label>
          </section>

          <section>
            <label class="field">
              <span class="aegis-section-hdr">Scenario</span>
              <select v-model="scenario" class="aegis-input">
                <option value="">(none)</option>
                <option v-for="s in registry.scenarios.scenarios" :key="s.slug" :value="s.slug">
                  {{ s.name }}
                </option>
              </select>
            </label>
          </section>

          <hr class="aegis-separator" />

          <section>
            <div class="aegis-section-hdr">Spirits</div>
            <div v-for="g in spiritGroups" :key="g.expansion" class="spirit-group">
              <div class="spirit-group-hdr">
                <ExpansionBadge :slug="g.expansion" :size="16" />
                <span class="spirit-group-name">{{
                  g.expansion === 'base' ? 'Base Game' :
                  g.expansion === 'branch-and-claw' ? 'Branch & Claw' :
                  g.expansion === 'jagged-earth' ? 'Jagged Earth' :
                  g.expansion === 'nature-incarnate' ? 'Nature Incarnate' :
                  g.expansion === 'horizons-of-spirit-island' || g.expansion === 'hosi' || g.expansion === 'horizons' ? 'Horizons of Spirit Island' :
                  g.expansion === 'promo-1' ? 'Promo Pack 1' :
                  g.expansion === 'promo-2' ? 'Promo Pack 2' :
                  g.expansion
                }}</span>
                <span class="spirit-group-count">{{ g.spirits.length }}</span>
              </div>
              <div class="chips">
                <label v-for="s in g.spirits" :key="s.slug" class="pick-chip">
                  <input type="checkbox" :checked="selectedSpirits.includes(s.slug)" @change="toggleSpirit(s.slug)" />
                  <span>{{ s.name }}</span>
                  <span class="muted">{{ s.complexity || '?' }}</span>
                </label>
              </div>
            </div>

            <div v-if="selectedSpirits.some(slug => aspectsFor(slug).length > 0)" class="aspect-list">
              <div class="aegis-section-hdr">Aspects (optional)</div>
              <div
                v-for="slug in selectedSpirits.filter(s => aspectsFor(s).length > 0)"
                :key="slug"
                class="aspect-row"
              >
                <span class="aspect-spirit-name">{{
                  registry.spirits.spirits.find(s => s.slug === slug)?.name || slug
                }}</span>
                <select v-model="selectedAspects[slug]" class="aegis-input">
                  <option value="">Base (no aspect)</option>
                  <option v-for="a in aspectsFor(slug)" :key="a.key" :value="a.key">
                    {{ a.name }}<span v-if="a.complexity_change"> — {{ a.complexity_change }}</span>
                  </option>
                </select>
              </div>
            </div>
          </section>

          <hr class="aegis-separator" />

          <section>
            <div class="aegis-section-hdr">Boards</div>
            <div class="chips">
              <label v-for="b in availableBoards" :key="b.board_id" class="pick-chip">
                <input type="checkbox" :checked="selectedBoards.includes(b.board_id)" @change="toggleBoard(b.board_id)" />
                <span class="board-id">{{ b.board_id }}</span>
                <ExpansionBadge :slug="b.expansion" :size="14" />
              </label>
            </div>
            <div class="variant-row">
              <span class="variant-label">Variant</span>
              <div class="variant-picker">
                <label class="variant-opt">
                  <input type="radio" v-model="boardVariant" value="balanced" :disabled="!variantAvailableForAllSelected.balanced" />
                  Balanced
                </label>
                <label class="variant-opt">
                  <input type="radio" v-model="boardVariant" value="thematic" :disabled="!variantAvailableForAllSelected.thematic" />
                  Thematic
                  <span v-if="!variantAvailableForAllSelected.thematic" class="muted">(not available for some selected boards)</span>
                </label>
              </div>
            </div>
          </section>

          <hr class="aegis-separator" />

          <section>
            <div class="aegis-section-hdr">Expansions active</div>
            <div class="chips">
              <label v-for="e in ['base', 'branch-and-claw', 'jagged-earth', 'nature-incarnate', 'promo-1', 'promo-2']" :key="e" class="pick-chip expansion-chip">
                <input type="checkbox" :checked="expansions.includes(e)" @change="toggleExpansion(e)" />
                <ExpansionBadge :slug="e" :size="20" :show-label="true" />
              </label>
            </div>
          </section>
        </div>
      </div>

      <div class="aegis-dialog-foot">
        <label class="archive-toggle" title="Save the current game to data/games/ before overwriting">
          <input type="checkbox" v-model="archiveCurrent" />
          <span>Archive current game first</span>
        </label>
        <div class="spacer" />
        <button class="outline" @click="emit('close')">Cancel</button>
        <button class="primary" :disabled="!selectedSpirits.length || !selectedBoards.length" @click="startGame">
          Start Game
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.wizard { width: min(92vw, 760px); }
.head-spacer { flex: 1; }

.error {
  background: rgba(239, 68, 68, 0.12);
  color: #fca5a5;
  border: 1px solid rgba(239, 68, 68, 0.35);
  padding: 8px 12px;
  border-radius: var(--r-sm);
  margin-bottom: 12px;
  font-size: var(--fs-sm);
}

.form { display: flex; flex-direction: column; gap: 16px; }
section { display: flex; flex-direction: column; gap: 8px; }

.grid-2 { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }

.field { display: flex; flex-direction: column; gap: 4px; }
.field .aegis-section-hdr { padding: 0 0 4px; }

.chips { display: flex; flex-wrap: wrap; gap: 6px; }

.pick-chip {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 5px 10px;
  font-size: var(--fs-xs);
  background: var(--bg-raised);
  border: 1px solid var(--aegis-border);
  border-radius: var(--r-sm);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--motion-fast);
}
.pick-chip:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
  border-color: var(--border-strong);
}
.pick-chip:has(input:checked) {
  background: color-mix(in srgb, var(--accent-blue) 12%, transparent);
  border-color: var(--accent-blue);
  color: var(--text-primary);
  box-shadow: 0 0 0 1px var(--accent-blue) inset;
}
.pick-chip input { accent-color: var(--accent-blue); }
.pick-chip .muted { color: var(--text-muted); font-size: var(--fs-xxs); }
.pick-chip .board-id { font-family: var(--font-mono); font-weight: var(--fw-semibold); }

.archive-toggle {
  display: inline-flex; align-items: center; gap: 6px;
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  cursor: pointer;
}
.spacer { flex: 1; }

.variant-row { display: flex; align-items: center; gap: 12px; margin-top: 4px; }
.variant-label {
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--text-muted);
  font-weight: var(--fw-bold);
}
.variant-picker { display: inline-flex; gap: 12px; }
.variant-opt {
  display: inline-flex; align-items: center; gap: 5px;
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  cursor: pointer;
}
.variant-opt:has(input:disabled) { color: var(--text-faint); cursor: not-allowed; }

.spirit-group { display: flex; flex-direction: column; gap: 6px; margin-bottom: 6px; }
.spirit-group-hdr {
  display: flex; align-items: center; gap: 8px;
  padding: 4px 6px;
  font-size: 10px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--text-muted);
  border-bottom: 1px solid var(--aegis-border);
}
.spirit-group-name { flex: 1; font-weight: var(--fw-semibold); color: var(--text-secondary); }
.spirit-group-count {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-muted);
  background: var(--bg-inset);
  padding: 1px 7px;
  border-radius: var(--r-full);
  border: 1px solid var(--aegis-border);
}

.aspect-list {
  margin-top: 8px;
  padding: 12px;
  background: color-mix(in srgb, var(--bg-inset) 50%, transparent);
  border: 1px solid var(--aegis-border);
  border-radius: var(--r-sm);
  display: flex; flex-direction: column; gap: 8px;
}
.aspect-row {
  display: grid;
  grid-template-columns: 200px 1fr;
  align-items: center;
  gap: 12px;
  font-size: var(--fs-sm);
}
.aspect-spirit-name {
  color: var(--text-secondary);
  font-weight: var(--fw-medium);
}
.aspect-row .aegis-input { height: 32px; font-size: var(--fs-xs); }
</style>
