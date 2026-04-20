<script setup lang="ts">
import type { Pools } from '../types'
import Icon from './Icon.vue'

defineProps<{ modelValue: Pools }>()
defineEmits<{ 'update:modelValue': [value: Pools] }>()
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

    <div class="pool blight">
      <div class="pool-hdr">
        <Icon name="resource-blight" :size="16" decorative />
        <span class="pool-name">Blight</span>
        <label class="island-state">
          <input type="checkbox" v-model="modelValue.island_blighted" />
          Island blighted
        </label>
      </div>
      <div class="pool-values">
        <input type="number" min="0" v-model.number="modelValue.blight_current" />
        <span class="sep">/</span>
        <input type="number" min="0" v-model.number="modelValue.blight_cap" />
      </div>
    </div>
  </section>
</template>

<style scoped>
.pools {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
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

.island-state {
  margin-left: auto;
  font-size: var(--fs-xs);
  color: var(--text-secondary);
  display: inline-flex;
  align-items: center;
  gap: var(--sp-1);
  cursor: pointer;
}

.pool-values {
  display: inline-flex;
  align-items: center;
  gap: var(--sp-2);
}

.sep {
  color: var(--text-muted);
  font-family: var(--font-mono);
}
</style>
