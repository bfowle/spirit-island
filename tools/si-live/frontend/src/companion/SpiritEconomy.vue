<script setup lang="ts">
import { computed } from 'vue'
import type { PublicPowerCard, PublicSpirit, SpiritStats } from './types'
import Panel from './Panel.vue'
import Icon from '../components/Icon.vue'
import { ELEMENTS, elementIcon, pct, prettyText } from './format'

const props = defineProps<{ stats: SpiritStats; spirit: PublicSpirit | undefined; multi: boolean }>()
const S = computed(() => props.stats)

function cardsFor(speed: 'Fast' | 'Slow', names: string[]): (PublicPowerCard | { title: string; missing: true })[] {
  const hand = props.spirit?.hand ?? []
  const fromHand = hand.filter((c) => c.speed === speed)
  if (fromHand.length || !names.length) return fromHand
  return names.map((title) => ({ title, missing: true as const }))
}
const fast = computed(() => cardsFor('Fast', S.value.hand_fast))
const slow = computed(() => cardsFor('Slow', S.value.hand_slow))
const inPlay = computed(() => props.spirit?.play_area ?? [])

function isCard(c: PublicPowerCard | { title: string; missing: true }): c is PublicPowerCard {
  return !('missing' in c)
}
function thresholdList(t: Record<string, number>): [string, number][] {
  return ELEMENTS.filter((e) => (t[e] ?? 0) > 0).map((e) => [e, t[e]])
}
function missingText(m: Record<string, number>): string {
  return Object.entries(m).map(([e, n]) => `${n} ${e}`).join(', ')
}
</script>

<template>
  <Panel :title="multi ? S.title : 'Spirit'" :question="multi ? undefined : `${S.title}: what can you afford and reach this turn?`" anchor="panel-spirit">
    <div class="tiles">
      <div class="tile">
        <span class="t-l">Energy</span>
        <span class="t-v">{{ S.energy }}</span>
        <span class="t-s">
          <template v-if="S.energy_per_turn != null">+{{ S.energy_per_turn }} per turn</template>
          <template v-if="S.next_energy != null">; next presence off the track shows {{ S.next_energy }}</template>
        </span>
      </div>
      <div class="tile">
        <span class="t-l">Card plays left</span>
        <span class="t-v">{{ S.card_plays }}</span>
        <span class="t-s">
          <template v-if="S.plays_per_turn != null">{{ S.plays_per_turn }} per turn</template>
          <template v-if="S.next_plays != null">; next shows {{ S.next_plays }}</template>
        </span>
      </div>
      <div v-if="spirit" class="tile">
        <span class="t-l">Presence</span>
        <span class="t-v">{{ spirit.presence_on_island }}</span>
        <span class="t-s">on the island; {{ spirit.presence_destroyed }} destroyed</span>
      </div>
    </div>

    <div class="elements" aria-label="Elements this turn">
      <span v-for="e in ELEMENTS" :key="e" class="el" :class="{ zero: !(S.elements[e] ?? 0) }" :title="`${e}: ${S.elements[e] ?? 0}`">
        <Icon :name="elementIcon(e)" :size="20" :label="e" />
        <b>{{ S.elements[e] ?? 0 }}</b>
      </span>
    </div>

    <div class="hand">
      <div v-for="col in [{ k: 'Fast', cards: fast, icon: 'speed-fast' }, { k: 'Slow', cards: slow, icon: 'speed-slow' }]" :key="col.k" class="hand-col">
        <div class="hc-h"><Icon :name="col.icon" :size="14" decorative /> {{ col.k }} in hand <span class="cnt">{{ col.cards.length }}</span></div>
        <ul>
          <li v-for="c in col.cards" :key="c.title" class="hcard" :title="isCard(c) ? c.body.map(prettyText).join('\n') : ''">
            <span class="cost" v-if="isCard(c)" :title="`Costs ${c.cost} Energy`">{{ c.cost }}</span>
            <span class="ct">{{ c.title }}</span>
            <span v-if="isCard(c)" class="els">
              <Icon v-for="(e, i) in c.elements" :key="i" :name="elementIcon(e)" :size="13" :label="e" />
            </span>
            <span v-if="isCard(c)" class="kind">{{ c.kind }}</span>
          </li>
          <li v-if="!col.cards.length" class="none">none</li>
        </ul>
      </div>
    </div>
    <p v-if="inPlay.length" class="inplay"><span class="lbl">In play</span> {{ inPlay.map((c) => c.title).join(', ') }}</p>

    <div v-for="inn in S.innates" :key="inn.title" class="innate">
      <div class="in-h">
        <Icon :name="inn.speed === 'Fast' ? 'speed-fast' : 'speed-slow'" :size="14" :label="inn.speed" />
        <b>{{ inn.title }}</b>
        <span class="muted">innate</span>
      </div>
      <div v-for="(lv, i) in inn.levels" :key="i" class="lvl" :class="{ met: lv.met_now }">
        <span class="thr">
          <span v-for="[e, n] in thresholdList(lv.threshold)" :key="e" class="th"><b>{{ n }}</b><Icon :name="elementIcon(e)" :size="13" :label="e" /></span>
        </span>
        <span class="st">
          <template v-if="lv.met_now"><span class="ok">met now</span></template>
          <template v-else>
            <span class="miss">missing {{ missingText(lv.missing) }}</span>
            <template v-if="lv.reachable_with && lv.reachable_with.length">; reachable by playing {{ lv.reachable_with.join(' + ') }}</template>
            <template v-else>; not reachable with this hand, Energy and plays</template>
          </template>
        </span>
        <span class="body">{{ prettyText(lv.body) }}</span>
      </div>
    </div>

    <h3 class="sub">Power draw odds</h3>
    <p class="hint">Unseen cards with the element, of the unseen pool, and the chance at least one shows in a draw of 4.</p>
    <table class="odds">
      <thead><tr><th>Element</th><th>Minor</th><th>Major</th></tr></thead>
      <tbody>
        <tr v-for="d in S.draw_odds" :key="d.element">
          <th scope="row"><Icon :name="elementIcon(d.element)" :size="14" decorative /> {{ d.element }}</th>
          <td><span class="f">{{ d.minor[0] }} of {{ d.minor[1] }}</span> <span class="arrow">→</span> <b>{{ pct(d.minor[2]) }}</b></td>
          <td><span class="f">{{ d.major[0] }} of {{ d.major[1] }}</span> <span class="arrow">→</span> <b>{{ pct(d.major[2]) }}</b></td>
        </tr>
      </tbody>
    </table>
  </Panel>
</template>

<style scoped>
.tiles { display: flex; gap: 8px; flex-wrap: wrap; }
.tile { flex: 1 1 120px; display: flex; flex-direction: column; padding: 6px 10px; border-radius: 8px; background: var(--bg-inset); border: 1px solid var(--border-subtle); min-width: 0; }
.t-l { font-size: 11px; color: var(--text-muted); }
.t-v { font-size: 26px; font-weight: 600; line-height: 1.1; color: var(--text-white); }
.t-s { font-size: 11px; color: var(--text-secondary); }
.elements { display: flex; gap: 10px; flex-wrap: wrap; margin: 10px 0 4px; }
.el { display: inline-flex; align-items: center; gap: 3px; font-size: 13px; }
.el.zero { opacity: 0.3; }
.hand { display: grid; grid-template-columns: minmax(0, 1fr); gap: 8px; margin-top: 8px; }
.hc-h { display: flex; align-items: center; gap: 5px; font-size: 12px; color: var(--text-secondary); margin-bottom: 4px; }
.cnt { color: var(--text-muted); }
.hand ul { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 3px; }
.hcard { display: flex; align-items: center; gap: 6px; padding: 3px 6px; border-radius: 6px; background: var(--bg-inset); font-size: 12px; min-width: 0; }
.cost { display: inline-grid; place-items: center; min-width: 18px; height: 18px; border-radius: 50%; background: rgba(251, 191, 36, 0.18); color: #fde68a; font-weight: 700; font-size: 11px; }
.ct { flex: 1 1 auto; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; color: var(--text-primary); }
.els { display: inline-flex; gap: 1px; }
.kind { font-size: 10px; color: var(--text-muted); }
.none { font-size: 12px; color: var(--text-faint); }
.inplay { margin: 6px 0 0; font-size: 12px; color: var(--text-secondary); }
.lbl { color: var(--text-muted); margin-right: 4px; }
.innate { margin-top: 10px; }
.in-h { display: flex; align-items: center; gap: 6px; font-size: 13px; color: var(--text-white); margin-bottom: 4px; }
.muted { color: var(--text-muted); font-size: 11px; }
.lvl { display: grid; grid-template-columns: 96px minmax(0, 1fr); gap: 1px 8px; padding: 4px 0; border-top: 1px solid color-mix(in srgb, var(--border-subtle) 60%, transparent); font-size: 12px; }
.thr { display: flex; gap: 6px; align-items: center; grid-row: span 2; }
.th { display: inline-flex; align-items: center; gap: 1px; }
.st { color: var(--text-secondary); }
.ok { color: #6ee7b7; font-weight: 600; }
.miss { color: var(--text-primary); }
.body { color: var(--text-muted); }
.sub { font-family: var(--font-brand); font-weight: 500; font-size: 14px; color: var(--text-white); margin: 12px 0 2px; }
.hint { margin: 0 0 4px; font-size: 11.5px; color: var(--text-muted); }
.odds { width: 100%; border-collapse: collapse; font-size: 12px; font-variant-numeric: tabular-nums; }
.odds th { text-align: left; font-weight: 500; color: var(--text-muted); font-size: 11px; padding: 2px 6px 3px 0; border-bottom: 1px solid var(--border-subtle); }
.odds tbody th { color: var(--text-secondary); font-size: 12px; border-bottom: 0; white-space: nowrap; }
.odds td { padding: 2px 6px 2px 0; white-space: nowrap; }
.f { color: var(--text-secondary); }
.arrow { color: var(--text-faint); }
</style>
