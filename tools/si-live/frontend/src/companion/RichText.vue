<script setup lang="ts">
import { computed } from 'vue'
import Icon from '../components/Icon.vue'
import { ELEMENTS, elementIcon, pieceIcon, prettyText } from './format'

/**
 * Rule and card text from the engine/register: `{Town}`-style tokens become
 * icons (pieces, blight, fear, elements) or words, `{BR}` a line break, and
 * `<i>…</i>` italics. Everything else is plain text (never v-html).
 */
const props = defineProps<{ text: string }>()

type Seg = { t: 'text'; v: string; i: boolean } | { t: 'icon'; name: string; label: string } | { t: 'br' }

const segs = computed<Seg[]>(() => {
  const out: Seg[] = []
  let italic = false
  const re = /(<i>|<\/i>|\{[A-Za-z ]+\}|<br\s*\/?>)/gi
  for (const part of props.text.split(re)) {
    if (!part) continue
    const low = part.toLowerCase()
    if (low === '<i>') { italic = true; continue }
    if (low === '</i>') { italic = false; continue }
    if (/^<br/.test(low) || part === '{BR}') { out.push({ t: 'br' }); continue }
    const tok = /^\{([A-Za-z ]+)\}$/.exec(part)
    if (tok) {
      const name = tok[1]
      const icon = pieceIcon(name) ?? (ELEMENTS.includes(name) ? elementIcon(name) : null)
      if (icon) out.push({ t: 'icon', name: icon, label: name })
      else out.push({ t: 'text', v: name, i: italic })
      continue
    }
    out.push({ t: 'text', v: prettyText(part.replace(/<\/?[a-z]+[^>]*>/gi, '')), i: italic })
  }
  return out
})
</script>

<template>
  <span class="rt"><template v-for="(s, k) in segs" :key="k"><br v-if="s.t === 'br'" /><Icon v-else-if="s.t === 'icon'" :name="s.name" :size="13" :label="s.label" class="rt-ico" /><em v-else-if="s.i">{{ s.v }}</em><template v-else>{{ s.v }}</template></template></span>
</template>

<style scoped>
.rt-ico { margin: 0 1px; vertical-align: -2px; }
em { color: var(--text-secondary); }
</style>
