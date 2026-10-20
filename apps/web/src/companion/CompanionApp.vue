<script setup lang="ts">
import { computed, onMounted, onUnmounted, provide, ref, watch } from 'vue'
import { useCompanionStream } from './useCompanionStream'
import { COMPANION_CTX } from './context'
import type { PublicLand, ReplayCommand } from './types'
import StatusStrip from './StatusStrip.vue'
import InvaderPipeline from './InvaderPipeline.vue'
import IslandMap from './IslandMap.vue'
import LandDetail from './LandDetail.vue'
import RavageLedgerPanel from './RavageLedgerPanel.vue'
import BuildExplore from './BuildExplore.vue'
import TimingLens from './TimingLens.vue'
import FearBlight from './FearBlight.vue'
import SpiritEconomy from './SpiritEconomy.vue'
import NeedFinder from './NeedFinder.vue'
import ThinkAbout from './ThinkAbout.vue'
import RulesInPlay from './RulesInPlay.vue'
import PlayByPlay from './PlayByPlay.vue'
import HistoryMultiples from './HistoryMultiples.vue'
import ReviewPanel from './ReviewPanel.vue'
import Panel from './Panel.vue'
import Icon from '../components/Icon.vue'
import { prettyText } from './format'

/**
 * Steam companion dashboard root. Mounted by main.ts on `#/companion`
 * instead of the tabletop App.vue, so nothing here can save tabletop state.
 * Read-only: the only write is the replay transport.
 */
const { state, health, connection, lastError, replay } = useCompanionStream()

const pub = computed(() => state.value?.public ?? null)
const stats = computed(() => state.value?.stats ?? null)
const review = computed(() => state.value?.review ?? null)
const changes = computed(() => state.value?.changes ?? [])
const decisions = computed(() => state.value?.decisions ?? [])
const ended = computed(() => pub.value?.ending != null)

// Tabs: Review opens by default once the game has ended; a click overrides.
const userTab = ref<'live' | 'review' | null>(null)
watch(ended, (now, before) => { if (now && !before) userTab.value = null })
const tab = computed(() => userTab.value ?? (ended.value ? 'review' : 'live'))

// Selected land (map + chips).
const selected = ref<string | null>(null)
const landById = computed<Record<string, PublicLand>>(() => Object.fromEntries((pub.value?.lands ?? []).map((l) => [l.id, l])))
const spiritTitles = computed<Record<string, string>>(() => Object.fromEntries((pub.value?.spirits ?? []).map((s) => [s.id, s.title])))
watch(landById, (m) => { if (selected.value && !m[selected.value]) selected.value = null })
function selectLand(id: string | null) {
  selected.value = id
  if (id && tab.value !== 'live') userTab.value = 'live'
  if (id) {
    // Bring the map into view when a land chip elsewhere is clicked.
    const map = document.getElementById('panel-island')
    const r = map?.getBoundingClientRect()
    if (map && r && (r.bottom < 80 || r.top > window.innerHeight - 80)) map.scrollIntoView({ behavior: 'smooth', block: 'start' })
  }
}
provide(COMPANION_CTX, { selected, selectLand, landById, spiritTitles })

const sel = computed(() => {
  const id = selected.value
  const s = stats.value
  if (!id || !pub.value) return null
  const land = landById.value[id]
  if (!land || land.is_ocean) return null
  return {
    land,
    outlook: s?.lands.find((l) => l.id === id),
    ravage: s?.invader_phase.ravages.find((r) => r.land === id),
    build: s?.invader_phase.builds.find((b) => b.land === id),
    candidates: s?.invader_phase.explore.candidates ?? [],
    cascade: s?.blight.cascade_chains.find((c) => c.land === id),
  }
})

const replayError = ref<string | null>(null)
async function onReplay(cmd: ReplayCommand) {
  replayError.value = await replay(cmd)
}

function onHash() {
  if (!location.hash.startsWith('#/companion')) location.reload()
}
function onKey(e: KeyboardEvent) {
  if (e.key === 'Escape' && selected.value) selected.value = null
}
onMounted(() => {
  document.title = 'Spirit Island companion'
  window.addEventListener('hashchange', onHash)
  window.addEventListener('keydown', onKey)
})
onUnmounted(() => {
  window.removeEventListener('hashchange', onHash)
  window.removeEventListener('keydown', onKey)
})
</script>

<template>
  <div class="companion">
    <StatusStrip :state="state" :health="health" :connection="connection" :last-error="replayError ?? lastError" @replay="onReplay">
      <div class="aegis-tabs" role="tablist" aria-label="View">
        <button type="button" role="tab" :aria-selected="tab === 'live'" :class="{ active: tab === 'live' }" @click="userTab = 'live'">This turn</button>
        <button type="button" role="tab" :aria-selected="tab === 'review'" :class="{ active: tab === 'review' }" @click="userTab = 'review'">Review</button>
      </div>
    </StatusStrip>

    <main v-if="!pub" class="waiting">
      <p class="w-title">Waiting for game state</p>
      <p class="w-body">
        <template v-if="connection !== 'open'">Connecting to the companion backend. Start si-live with the Steam source, or check that it is running.</template>
        <template v-else>The backend has not decoded a save yet. It fills in as soon as the game writes one (a restarted replay takes a moment).</template>
      </p>
    </main>

    <main v-else-if="tab === 'live'" class="live">
      <div class="top">
        <div class="top-main">
          <InvaderPipeline v-if="stats" :invaders="pub.invaders" :phase="stats.invader_phase" :stages="stats.stages" />
          <Panel title="Island" question="Where is each piece, and which lands act this turn?" anchor="panel-island" flush>
            <template #meta><span>Select a land for its detail</span></template>
            <IslandMap
              :lands="pub.lands"
              :adjacency="pub.adjacency"
              :boards="pub.boards"
              :outlooks="stats?.lands ?? []"
              :selected="selected"
              @select="selectLand"
            />
            <div class="map-legend">
              <span><Icon name="unit-explorer" :size="12" decorative /> Explorer</span>
              <span><Icon name="unit-town" :size="13" decorative /> Town</span>
              <span><Icon name="unit-city" :size="14" decorative /> City</span>
              <span><Icon name="unit-dahan" :size="12" decorative /> Dahan</span>
              <span><Icon name="resource-blight" :size="13" decorative /> Blight</span>
              <span><i class="lg-disc"></i> Presence</span>
              <span><i class="lg-chip rav"></i> Ravage this turn</span>
              <span><i class="lg-chip rav-next"></i> Ravage next turn</span>
              <span><i class="lg-chip bld"></i> Build this turn</span>
              <span><i class="lg-pip"></i> Explore: candidate cards that hit it</span>
              <span><i class="lg-line"></i> Board join</span>
            </div>
          </Panel>
        </div>
        <aside class="top-side">
          <div class="side-inner">
            <LandDetail
              v-if="sel"
              class="side-detail"
              :land="sel.land"
              :outlook="sel.outlook"
              :ravage="sel.ravage"
              :build="sel.build"
              :candidates="sel.candidates"
              :cascade="sel.cascade"
              @close="selected = null"
            />
            <PlayByPlay :changes="changes" :decisions="decisions" fill class="side-feed" />
          </div>
        </aside>
      </div>

      <div v-if="stats" class="lower">
        <div class="stack">
          <RavageLedgerPanel :phase="stats.invader_phase" :blight="stats.blight" :effects="pub.status_effects" />
          <BuildExplore :phase="stats.invader_phase" />
          <RulesInPlay v-if="stats.rules_in_play?.length" :rules="stats.rules_in_play" />
        </div>
        <div class="stack">
          <TimingLens :lands="stats.lands" />
          <FearBlight :fear="stats.fear" :blight="stats.blight" :p-fear="pub.fear" :p-blight="pub.blight" />
          <NeedFinder v-if="stats.needs?.length" :needs="stats.needs" :multi="stats.needs.length > 1" />
        </div>
        <div class="stack">
          <ThinkAbout :prompts="stats.prompts" />
          <SpiritEconomy
            v-for="sp in stats.spirits"
            :key="sp.id"
            :stats="sp"
            :spirit="pub.spirits.find((x) => x.id === sp.id)"
            :multi="stats.spirits.length > 1"
          />
        </div>
      </div>

      <HistoryMultiples :history="state?.history ?? []" :ended="ended" class="history" />

      <footer v-if="stats?.basis.length" class="basis">
        <span class="b-h">What these numbers assume</span>
        <ul><li v-for="(b, i) in stats.basis" :key="i">{{ prettyText(b) }}</li></ul>
      </footer>
    </main>

    <main v-else class="review-wrap">
      <ReviewPanel v-if="review" :review="review" />
      <p v-else class="w-body">No review yet: it builds from the game's saves.</p>
    </main>
  </div>
</template>

<style scoped>
.companion { min-height: 100vh; background: var(--bg-canvas); color: var(--text-primary); }
.live, .review-wrap { padding: 12px 16px 24px; display: flex; flex-direction: column; gap: 12px; }
.top { display: grid; grid-template-columns: minmax(0, 3fr) minmax(340px, 1fr); gap: 12px; align-items: stretch; }
.top-main, .stack { display: flex; flex-direction: column; gap: 12px; min-width: 0; }
.top-side { position: relative; min-width: 0; min-height: 520px; }
/* The side column is as tall as the pipeline + map; its panels scroll inside. */
.side-inner { position: absolute; inset: 0; display: flex; flex-direction: column; gap: 12px; }
.side-detail { flex: 0 1 auto; max-height: 64%; overflow-y: auto; }
.side-feed { flex: 1 1 0; min-height: 160px; }
.lower { display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 2fr) minmax(0, 1fr); gap: 12px; align-items: start; }
@media (max-width: 1599px) {
  .top { grid-template-columns: minmax(0, 1fr); }
  .top-side { min-height: 0; }
  .side-inner { position: static; display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); align-items: start; }
  .side-detail { max-height: none; }
  .side-feed:first-child { grid-column: 1 / -1; }
  .lower { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  .lower > .stack:nth-child(2) { grid-row: span 2; }
}
.map-legend { display: flex; flex-wrap: wrap; gap: 4px 14px; padding: 6px 14px 10px; font-size: 11.5px; color: var(--text-secondary); border-top: 1px solid var(--border-subtle); }
.map-legend span { display: inline-flex; align-items: center; gap: 4px; }
.lg-disc { width: 10px; height: 10px; border-radius: 50%; background: var(--accent-blue-hover); border: 1.5px solid #dbeafe; }
.lg-chip { width: 16px; height: 9px; border-radius: 2px; }
.lg-chip.rav { background: rgba(239, 68, 68, 0.5); }
.lg-chip.rav-next { box-shadow: inset 0 0 0 1px rgba(239, 68, 68, 0.8); }
.lg-chip.bld { background: rgba(251, 146, 60, 0.5); }
.lg-pip { width: 7px; height: 7px; border-radius: 1.5px; background: var(--accent-teal); }
.lg-line { width: 16px; height: 2px; background: hsl(258 60% 70%); }
.basis { padding: 8px 14px; border-radius: 10px; border: 1px solid var(--border-subtle); font-size: 12px; color: var(--text-muted); }
.b-h { color: var(--text-secondary); font-weight: 600; }
.basis ul { margin: 4px 0 0; padding-left: 18px; }
.waiting { max-width: 560px; margin: 80px auto; text-align: center; }
.w-title { font-family: var(--font-brand); font-size: 22px; color: var(--text-white); margin: 0 0 6px; }
.w-body { font-size: 13.5px; color: var(--text-secondary); margin: 0; }
</style>
