import { createApp } from 'vue'
import './design-tokens.css'
import './style.css'

// Pick the root once at page load. The Steam companion (`#/companion`) is a
// separate, read-only root: the tabletop App.vue (state load + saveState
// timer) must never mount there. CompanionApp reloads the page when the hash
// leaves `#/companion`.
if (location.hash.startsWith('#/companion')) {
  import('./companion/CompanionApp.vue').then(({ default: CompanionApp }) => createApp(CompanionApp).mount('#app'))
} else {
  import('./App.vue').then(({ default: App }) => createApp(App).mount('#app'))
}
