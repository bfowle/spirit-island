<script setup lang="ts">
import { computed } from 'vue'
import type { InvaderPhaseForecast } from './types'
import Panel from './Panel.vue'
import LandChip from './LandChip.vue'
import InvaderCardTile from './InvaderCardTile.vue'
import Icon from '../components/Icon.vue'
import { invaderLabel, parseBlocked, pieceIcon, prettyText, roman } from './format'

const props = defineProps<{ phase: InvaderPhaseForecast }>()

const ex = computed(() => props.phase.explore)
const heading = computed(() => {
  const e = ex.value
  if (e.revealed) return 'The Explore card is face up'
  if (e.deck_empty) return 'The Invader deck is empty'
  if (e.next_stage != null) return `If the next card is …`
  return 'No Explore card'
})
const anyBlocked = computed(() => ex.value.candidates.some((c) => c.blocked.length > 0))
</script>

<template>
  <Panel title="Build and Explore" question="Where do new Invaders arrive?" anchor="panel-explore">
    <template v-if="phase.high_immigration_cards?.length">
      <h3 class="sub">High Immigration, before the Ravage <span class="cnt">{{ phase.high_immigration_cards.map(invaderLabel).join(', ') }}</span></h3>
      <table v-if="phase.high_immigration.length" class="tbl builds">
        <tbody>
          <tr v-for="b in phase.high_immigration" :key="b.land" :class="{ done: b.done }">
            <td class="c-land"><LandChip :id="b.land" /></td>
            <td class="c-add">
              <Icon v-if="pieceIcon(b.adds)" :name="pieceIcon(b.adds)!" :size="14" decorative />
              +{{ b.adds }}
            </td>
            <td class="c-note">
              <span v-if="b.done" class="muted">built</span>
              <span v-for="(n, i) in b.notes" :key="i" class="note">{{ prettyText(n) }}</span>
            </td>
          </tr>
        </tbody>
      </table>
      <p v-else class="empty">No land builds.</p>
    </template>

    <h3 class="sub">Build <span class="cnt">{{ phase.builds.length }}</span></h3>
    <table v-if="phase.builds.length" class="tbl builds">
      <tbody>
        <tr v-for="b in phase.builds" :key="b.land" :class="{ done: b.done }">
          <td class="c-land"><LandChip :id="b.land" /></td>
          <td class="c-add">
            <Icon v-if="pieceIcon(b.adds)" :name="pieceIcon(b.adds)!" :size="14" decorative />
            +{{ b.adds }}
          </td>
          <td class="c-note">
            <span v-if="b.done" class="muted">built</span>
            <span v-for="(n, i) in b.notes" :key="i" class="note">{{ prettyText(n) }}</span>
          </td>
        </tr>
      </tbody>
    </table>
    <p v-else class="empty">No land matches the Build card<template v-if="phase.build_cards.length"> and has Invaders</template>.</p>

    <template v-if="phase.escalation">
      <h3 class="sub">
        Escalation: {{ phase.escalation.title }}
        <span class="cnt">{{ phase.escalation.chance[0] }} of {{ phase.escalation.chance[1] }} possible cards</span>
      </h3>
      <p class="lead">{{ prettyText(phase.escalation.text) }} Before the Explore.</p>
      <table v-if="phase.escalation.builds.length" class="tbl builds">
        <tbody>
          <tr v-for="b in phase.escalation.builds" :key="b.land">
            <td class="c-land"><LandChip :id="b.land" /></td>
            <td class="c-add">
              <Icon v-if="pieceIcon(b.adds)" :name="pieceIcon(b.adds)!" :size="14" decorative />
              +{{ b.adds }}
            </td>
            <td class="c-note"><span v-for="(n, i) in b.notes" :key="i" class="note">{{ prettyText(n) }}</span></td>
          </tr>
        </tbody>
      </table>
    </template>

    <h3 class="sub">
      Explore
      <span v-if="ex.next_stage != null && !ex.revealed" class="cnt">next card is Stage {{ roman(ex.next_stage) }}</span>
    </h3>
    <p class="lead">{{ heading }}</p>
    <table v-if="ex.candidates.length" class="tbl cands">
      <thead>
        <tr>
          <th>Card</th>
          <th>Explores</th>
          <th v-if="anyBlocked">Matches but won't explore</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="c in ex.candidates" :key="c.card.id">
          <td class="c-card"><InvaderCardTile :card="c.card" size="xs" /> <span>{{ invaderLabel(c.card) }}</span></td>
          <td>
            <span class="lands">
              <LandChip v-for="l in c.lands" :key="l" :id="l" />
              <span v-if="!c.lands.length" class="muted">no land</span>
            </span>
          </td>
          <td v-if="anyBlocked">
            <span class="lands">
              <LandChip
                v-for="b in c.blocked"
                :key="b"
                :id="parseBlocked(b).land"
                :note="parseBlocked(b).reason ?? 'no source'"
                muted
              />
            </span>
          </td>
        </tr>
      </tbody>
    </table>
    <p class="pocket">{{ prettyText(ex.pocket_rule) }}</p>
  </Panel>
</template>

<style scoped>
.sub { font-family: var(--font-brand); font-size: 14px; font-weight: 500; color: var(--text-white); margin: 2px 0 6px; display: flex; gap: 8px; align-items: baseline; }
.sub + .tbl, .sub + .empty { margin-top: 0; }
.sub:not(:first-child) { margin-top: 12px; }
.cnt { font-family: var(--font-sans); font-size: 12px; color: var(--text-muted); font-weight: 500; }
.tbl { width: 100%; border-collapse: collapse; font-size: 12.5px; }
.tbl th { text-align: left; font-weight: 500; font-size: 11px; color: var(--text-muted); padding: 2px 6px 4px 0; border-bottom: 1px solid var(--border-subtle); }
.tbl td { padding: 4px 6px 4px 0; vertical-align: middle; border-bottom: 1px solid color-mix(in srgb, var(--border-subtle) 60%, transparent); }
.tbl tr:last-child td { border-bottom: 0; }
.builds .c-land { width: 1%; white-space: nowrap; }
.builds .c-add { width: 1%; white-space: nowrap; font-weight: 600; }
.builds tr.done { opacity: 0.5; }
.c-note { color: var(--text-secondary); }
.note { color: var(--accent-amber); }
.c-card { white-space: nowrap; width: 1%; }
.c-card span { margin-left: 4px; }
.lands { display: flex; flex-wrap: wrap; gap: 3px; }
.lead { margin: 0 0 4px; font-size: 12.5px; color: var(--text-secondary); }
.pocket { margin: 8px 0 0; padding: 6px 8px; border-radius: 6px; background: var(--bg-inset); font-size: 12px; color: var(--text-secondary); border-left: 2px solid var(--accent-teal); }
.muted { color: var(--text-muted); }
.empty { margin: 0; font-size: 12.5px; color: var(--text-muted); }
</style>
