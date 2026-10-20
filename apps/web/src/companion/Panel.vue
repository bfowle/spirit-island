<script setup lang="ts">
/** Panel frame: a title, the question the panel answers, and a body. */
defineProps<{ title: string; question?: string; anchor?: string; flush?: boolean; fill?: boolean }>()
</script>

<template>
  <section class="cpanel" :class="{ fill }" :id="anchor" :aria-label="title">
    <header class="cpanel-head">
      <h2 class="cpanel-title">{{ title }}</h2>
      <p v-if="question" class="cpanel-q">{{ question }}</p>
      <div class="cpanel-meta"><slot name="meta" /></div>
    </header>
    <div class="cpanel-body" :class="{ flush }"><slot /></div>
  </section>
</template>

<style scoped>
.cpanel {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: 10px;
  min-width: 0;
  display: flex;
  flex-direction: column;
}
.cpanel-head {
  display: flex;
  align-items: baseline;
  gap: 10px;
  padding: 10px 14px 8px;
  border-bottom: 1px solid var(--border-subtle);
  flex-wrap: wrap;
}
.cpanel-title {
  font-family: var(--font-brand);
  font-size: 17px;
  font-weight: 500;
  letter-spacing: 0.01em;
  color: var(--text-white);
  margin: 0;
}
.cpanel-q {
  margin: 0;
  font-size: 12px;
  color: var(--text-muted);
  flex: 1 1 auto;
  min-width: 0;
}
.cpanel-meta { margin-left: auto; display: flex; gap: 6px; align-items: center; font-size: 12px; color: var(--text-secondary); }
.cpanel-meta:empty { display: none; }
.cpanel-body { padding: 10px 14px 12px; min-width: 0; flex: 1 1 auto; }
.cpanel-body.flush { padding: 0; }
/* fill: the panel takes the height its parent gives it and scrolls inside (wide layout only). */
@media (min-width: 1600px) {
  .cpanel.fill { min-height: 0; }
  .cpanel.fill .cpanel-body { flex: 1 1 0; min-height: 0; overflow-y: auto; }
}
</style>
