<script setup lang="ts">
import { computed } from 'vue'
import type { GameState, Phase } from '../types'
import UiIcon from './UiIcon.vue'

/**
 * Left-anchored persistent sidebar nav — Aegis-style.
 *
 * Houses: logo, per-phase stepper (vertical), nav groups, quick actions
 * (saved / new / archive / export / ops). Phase stepper lives here because
 * vertical space > horizontal space for 8 phases and the top header was
 * too crammed.
 */

const props = defineProps<{
  state: GameState
  activeSection: string
}>()

const emit = defineEmits<{
  'select-section': [id: string]
  'new-game': []
  'saved-games': []
  'set-phase': [phase: Phase]
  'archive': []
  'export': []
  'toggle-ops': []
}>()

const phases: Phase[] = ['setup', 'growth', 'fast', 'event', 'fear', 'invader', 'slow', 'timepasses']
const currentIdx = computed(() => phases.indexOf((props.state?.phase ?? 'setup') as Phase))
const currentRound = computed(() => props.state?.round ?? 1)
function phaseLabel(p: Phase): string { return p.charAt(0).toUpperCase() + p.slice(1) }
function selectPhase(p: Phase) { emit('set-phase', p) }

interface NavItem { id: string; label: string; icon: string }
interface NavGroup { heading: string; items: NavItem[] }

const groups = computed<NavGroup[]>(() => [
  {
    heading: 'Workspace',
    items: [
      { id: 'overview', label: 'Overview', icon: 'target' },
      { id: 'board', label: 'Board', icon: 'chart' },
      { id: 'invader', label: 'Invader Deck', icon: 'swords' },
      { id: 'fear', label: 'Fear Deck', icon: 'ghost' },
      { id: 'events', label: 'Event Deck', icon: 'scroll' },
    ],
  },
  {
    heading: 'Analysis',
    items: [
      { id: 'stats', label: 'Stats', icon: 'chart' },
      { id: 'terrain', label: 'Terrain Timeline', icon: 'chart' },
      { id: 'retro', label: 'Retrospective', icon: 'search' },
    ],
  },
])

const matchupTag = computed(() => {
  const s = props.state?.setup
  if (!s) return 'New game'
  const adv = s.adversary ? s.adversary.replace(/-/g, ' ') : 'Solo'
  const lvl = s.level != null ? `L${s.level}` : ''
  return `${adv}${lvl ? ' · ' + lvl : ''}`
})

const spiritSummary = computed(() => {
  const ss = Object.keys(props.state.spirits ?? {})
  if (ss.length === 0) return 'no spirits'
  if (ss.length === 1) return ss[0].split('-')[0]
  return `${ss.length} spirits`
})
</script>

<template>
  <aside class="sidebar">
    <div class="logo">
      <div class="logo-mark">SI</div>
      <div class="logo-text-wrap">
        <div class="logo-text brand">SI-LIVE</div>
        <div class="logo-sub">Tableside companion</div>
      </div>
    </div>

    <nav class="nav">
      <!-- Phase stepper — vertical. Click to jump; round counter on the heading. -->
      <div class="nav-group phase-group">
        <div class="nav-heading">
          <span>Phase</span>
          <span class="round-pill">R{{ currentRound }}</span>
        </div>
        <button
          v-for="(p, i) in phases"
          :key="p"
          type="button"
          class="phase-item"
          :class="{ active: state?.phase === p, done: i < currentIdx }"
          @click="selectPhase(p)"
        >
          <span class="pnum mono">{{ String(i + 1).padStart(2, '0') }}</span>
          <span class="pdot"></span>
          <span class="plabel">{{ phaseLabel(p) }}</span>
        </button>
      </div>

      <div v-for="g in groups" :key="g.heading" class="nav-group">
        <div class="nav-heading">{{ g.heading }}</div>
        <button
          v-for="item in g.items"
          :key="item.id"
          type="button"
          class="nav-item"
          :class="{ active: activeSection === item.id }"
          @click="$emit('select-section', item.id)"
        >
          <UiIcon :name="item.icon" :size="14" class="nav-icon" decorative />
          <span class="nav-label">{{ item.label }}</span>
        </button>
      </div>

      <div class="nav-group">
        <div class="nav-heading">Games</div>
        <button type="button" class="nav-item" @click="$emit('saved-games')">
          <UiIcon name="save" :size="14" class="nav-icon" decorative />
          <span class="nav-label">Saved Games</span>
        </button>
        <button type="button" class="nav-item primary-nav" @click="$emit('new-game')">
          <UiIcon name="plus" :size="14" class="nav-icon" decorative />
          <span class="nav-label">New Game</span>
        </button>
        <button type="button" class="nav-item" @click="$emit('archive')">
          <UiIcon name="archive" :size="14" class="nav-icon" decorative />
          <span class="nav-label">Archive Current</span>
        </button>
        <button type="button" class="nav-item" @click="$emit('export')">
          <UiIcon name="download" :size="14" class="nav-icon" decorative />
          <span class="nav-label">Export JSON</span>
        </button>
        <button type="button" class="nav-item" @click="$emit('toggle-ops')">
          <UiIcon name="settings" :size="14" class="nav-icon" decorative />
          <span class="nav-label">Turn Log / Pools</span>
        </button>
      </div>
    </nav>

    <div class="sidebar-footer">
      <div class="avatar">{{ spiritSummary.charAt(0).toUpperCase() }}</div>
      <div class="footer-text">
        <div class="footer-line1">{{ spiritSummary }}</div>
        <div class="footer-line2">{{ matchupTag }}</div>
      </div>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  grid-area: nav;
  border-right: 1px solid var(--border-subtle);
  background: var(--bg-surface);
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
  height: 100%;
  min-width: 0;
  position: relative;
  z-index: 2;
}

.logo {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 0 18px;
  height: 56px;
  flex: 0 0 56px;
  border-bottom: 1px solid var(--border-subtle);
}
.logo-mark {
  width: 30px; height: 30px;
  border-radius: 7px;
  background: var(--accent-grad);
  display: grid; place-items: center;
  color: white; font-weight: 700; font-size: 13px;
  box-shadow: var(--shadow-glow);
}
.logo-text-wrap { display: flex; flex-direction: column; gap: 0; }
.logo-text { font-size: 16px; line-height: 1; color: var(--text-white); }
.logo-sub {
  font-size: 10px; color: var(--text-muted);
  text-transform: uppercase; letter-spacing: 0.12em;
  margin-top: 2px;
}

.nav {
  padding: 14px 10px;
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
}
.nav-group { margin-bottom: 18px; }
.nav-heading {
  font-size: 10px; text-transform: uppercase; letter-spacing: 0.12em;
  color: var(--text-muted);
  padding: 0 10px 8px;
  font-weight: var(--fw-semibold);
}
.nav-item {
  display: flex; align-items: center; gap: 10px;
  padding: 7px 10px;
  border-radius: var(--r-sm);
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 13px;
  width: 100%;
  background: transparent;
  border: 1px solid transparent;
  text-align: left;
  margin-bottom: 2px;
  transition: all 150ms ease;
}
.nav-item:hover {
  background: color-mix(in srgb, var(--accent-blue) 8%, transparent);
  color: var(--text-primary);
  border-color: transparent;
}
.nav-item.active {
  background: var(--accent-grad);
  color: white;
  box-shadow: var(--shadow-glow);
}
.nav-item.active .nav-icon { color: white; }
.nav-icon {
  width: 16px; height: 16px;
  display: inline-grid; place-items: center;
  color: var(--accent-blue);
  font-size: 12px;
}
.nav-label { flex: 1; }

.nav-item.primary-nav {
  background: color-mix(in srgb, var(--accent-blue) 15%, transparent);
  border: 1px solid color-mix(in srgb, var(--accent-blue) 35%, transparent);
  color: var(--text-primary);
  margin-top: 4px;
}
.nav-item.primary-nav:hover {
  background: var(--accent-grad);
  color: white;
  box-shadow: var(--shadow-glow);
}

/* ──────────────────────── Phase stepper ──────────────────────── */
.phase-group {
  border: 1px solid var(--border-subtle);
  border-radius: var(--r-md);
  padding: 8px 6px 10px;
  margin-bottom: 18px;
  background: color-mix(in srgb, var(--accent-blue) 5%, var(--bg-inset));
}
.phase-group .nav-heading { display: flex; justify-content: space-between; align-items: center; padding: 0 6px 6px; }
.round-pill {
  font-family: var(--font-mono);
  font-size: 10px;
  font-weight: 700;
  padding: 2px 6px;
  border-radius: var(--r-full);
  background: color-mix(in srgb, var(--accent-purple) 22%, transparent);
  color: #ddd4f9;
  letter-spacing: 0;
  text-transform: none;
}

.phase-item {
  display: flex; align-items: center; gap: 8px;
  padding: 7px 10px;
  border: 1px solid transparent;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: var(--r-sm);
  font-size: 13px;
  font-weight: var(--fw-medium);
  text-align: left;
  width: 100%;
  margin-bottom: 2px;
  transition: all 150ms ease;
  position: relative;
}
.phase-item:hover {
  color: var(--text-primary);
  background: color-mix(in srgb, var(--accent-blue) 8%, transparent);
}
.phase-item .pnum {
  font-size: 10px;
  color: var(--text-muted);
  width: 18px;
  text-align: right;
  flex: 0 0 18px;
}
.phase-item .pdot {
  width: 7px; height: 7px;
  border-radius: 50%;
  background: var(--border-subtle);
  flex: 0 0 7px;
  transition: all 150ms ease;
}
.phase-item .plabel { flex: 1 1 auto; }
.phase-item.done { color: var(--text-secondary); }
.phase-item.done .pnum { color: var(--accent-green); }
.phase-item.done .pdot { background: var(--accent-green); box-shadow: 0 0 6px var(--accent-green); }
.phase-item.active {
  background: var(--accent-grad);
  color: white;
  border-color: transparent;
  box-shadow: var(--shadow-glow);
}
.phase-item.active .pnum { color: rgba(255, 255, 255, 0.78); }
.phase-item.active .pdot { background: white; box-shadow: 0 0 6px white; }

.sidebar-footer {
  padding: 12px 14px;
  border-top: 1px solid var(--border-subtle);
  display: flex;
  align-items: center;
  gap: 10px;
  flex: 0 0 auto;
}
.avatar {
  width: 32px; height: 32px;
  border-radius: 50%;
  background: var(--accent-grad);
  display: grid; place-items: center;
  color: white; font-weight: 700; font-size: 13px;
  flex: 0 0 32px;
}
.footer-text { min-width: 0; overflow: hidden; }
.footer-line1 {
  font-size: 12px;
  color: var(--text-primary);
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
  font-family: var(--font-mono);
  text-transform: capitalize;
}
.footer-line2 {
  font-size: 10px;
  color: var(--text-muted);
  text-transform: capitalize;
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
</style>
