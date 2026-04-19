<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import type { GameState } from '../types'

interface RegistryResponse {
  spirits: { spirits: { slug: string; name: string; expansion: string; complexity?: string }[] }
  adversaries: { adversaries: { slug: string; name: string; expansion: string; difficulty_levels: number[] }[] }
  scenarios: { scenarios: { slug: string; name: string; expansion: string }[] }
  boards: { file: string; data: { board_id: string; expansion: string } }[]
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

onMounted(async () => {
  try {
    const res = await fetch('/api/registry')
    if (!res.ok) throw new Error(`GET /api/registry → ${res.status}`)
    registry.value = await res.json()
  } catch (e) {
    error.value = (e as Error).message
  }
})

const availableLevels = computed(() => {
  if (!registry.value) return [0, 1, 2, 3, 4, 5, 6]
  const adv = registry.value.adversaries.adversaries.find(a => a.slug === adversary.value)
  return adv ? [0, ...adv.difficulty_levels] : [0, 1, 2, 3, 4, 5, 6]
})

const availableBoards = computed(() => {
  if (!registry.value) return []
  return registry.value.boards.map(b => b.data.board_id).sort()
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
    const res = await fetch('/api/new-game', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        adversary: adversary.value || null,
        level: level.value,
        scenario: scenario.value || null,
        spirits: selectedSpirits.value,
        boards: selectedBoards.value,
        expansions_active: expansions.value,
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
  <div v-if="show" class="modal-bg" @click.self="emit('close')">
    <div class="modal">
      <header>
        <h2>New Game Setup</h2>
        <button class="close" @click="emit('close')" aria-label="close">×</button>
      </header>

      <div v-if="error" class="error">{{ error }}</div>
      <div v-else-if="!registry" class="loading">Loading…</div>
      <div v-else class="form">
        <section>
          <label>Adversary
            <select v-model="adversary">
              <option value="">(none)</option>
              <option v-for="a in registry.adversaries.adversaries" :key="a.slug" :value="a.slug">
                {{ a.name }}
              </option>
            </select>
          </label>
          <label>Level
            <select v-model.number="level">
              <option v-for="l in availableLevels" :key="l" :value="l">L{{ l }}</option>
            </select>
          </label>
        </section>

        <section>
          <label>Scenario
            <select v-model="scenario">
              <option value="">(none)</option>
              <option v-for="s in registry.scenarios.scenarios" :key="s.slug" :value="s.slug">
                {{ s.name }}
              </option>
            </select>
          </label>
        </section>

        <section>
          <div class="hdr">Spirits</div>
          <div class="chips">
            <label v-for="s in registry.spirits.spirits" :key="s.slug" class="chip">
              <input type="checkbox" :checked="selectedSpirits.includes(s.slug)" @change="toggleSpirit(s.slug)" />
              {{ s.name }}
              <span class="muted">({{ s.complexity || '?' }})</span>
            </label>
          </div>
        </section>

        <section>
          <div class="hdr">Boards</div>
          <div class="chips">
            <label v-for="b in availableBoards" :key="b" class="chip">
              <input type="checkbox" :checked="selectedBoards.includes(b)" @change="toggleBoard(b)" />
              {{ b }}
            </label>
          </div>
        </section>

        <section>
          <div class="hdr">Expansions active</div>
          <div class="chips">
            <label v-for="e in ['base', 'branch-and-claw', 'jagged-earth', 'nature-incarnate', 'promo-2']" :key="e" class="chip">
              <input type="checkbox" :checked="expansions.includes(e)" @change="toggleExpansion(e)" />
              {{ e }}
            </label>
          </div>
        </section>

        <footer>
          <button class="primary" :disabled="!selectedSpirits.length || !selectedBoards.length" @click="startGame">
            Start Game
          </button>
          <button @click="emit('close')">Cancel</button>
        </footer>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-bg { position: fixed; inset: 0; background: rgba(0,0,0,0.65); display: flex; align-items: center; justify-content: center; z-index: 100; }
.modal { background: #1a1a1e; border: 1px solid #444; border-radius: 8px; padding: 1rem; width: min(90vw, 720px); max-height: 90vh; overflow-y: auto; }
header { display: flex; justify-content: space-between; align-items: center; margin-bottom: .75rem; border-bottom: 1px solid #333; padding-bottom: .5rem; }
h2 { margin: 0; font-size: 1.1rem; }
.close { background: transparent; border: none; color: #ccc; font-size: 1.5rem; cursor: pointer; }
.close:hover { color: #fff; }
.error { background: #522; color: #fcc; padding: .5rem; border-radius: 4px; margin-bottom: .5rem; }
.loading { text-align: center; color: #aaa; padding: 2rem; }
section { margin-bottom: .75rem; }
.hdr { font-size: .85rem; color: #ccc; margin-bottom: .35rem; text-transform: uppercase; letter-spacing: 0.5px; }
label { display: inline-flex; align-items: center; gap: .35rem; font-size: .9rem; margin-right: 1rem; }
select { padding: .15rem .3rem; }
.chips { display: flex; flex-wrap: wrap; gap: .4rem; }
.chip { font-size: .85rem; background: #2a2a30; padding: .25rem .5rem; border-radius: 4px; border: 1px solid #444; cursor: pointer; margin: 0; }
.chip:has(input:checked) { background: #3a3a48; border-color: #888; }
.chip .muted { color: #888; font-size: .75rem; }
footer { display: flex; gap: .5rem; justify-content: flex-end; margin-top: 1rem; border-top: 1px solid #333; padding-top: .75rem; }
button { background: #2a2a30; border: 1px solid #444; color: #eee; padding: .35rem .75rem; border-radius: 4px; cursor: pointer; font-size: .9rem; }
button:hover:not(:disabled) { background: #383840; }
button.primary { background: #3a5a3a; border-color: #5a8a5a; }
button.primary:hover:not(:disabled) { background: #4a6a4a; }
button:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
