import { onMounted, onUnmounted, ref, shallowRef } from 'vue'
import type { CompanionHealth, CompanionState, ReplayCommand } from './types'

/**
 * Read-only client for the Steam companion backend.
 *
 * - Initial `GET /api/companion/state`, then `GET /api/companion/stream`
 *   (SSE, event `state`, full CompanionState per change).
 * - Reconnects with backoff when the stream drops; re-fetches state on reconnect.
 * - Drops a state whose `seq` is not newer than the one shown, with three
 *   exceptions the backend's seq semantics force:
 *     1. the first state after a (re)connect is always taken (a restarted
 *        backend starts its seq over);
 *     2. replay mode: a seek rebuilds the timeline, so seq can go down, and
 *        play/pause re-publishes the same seq with new replay status;
 *     3. same seq with a newer `source.updated_at_ms` (status-only change,
 *        e.g. the stale flag) replaces the shown state.
 * - Polls `GET /api/companion/health` every 15 s.
 *
 * The only write this module can make is `POST /api/companion/replay`
 * (replay transport). It never touches `/api/state`.
 */

export type Connection = 'connecting' | 'open' | 'reconnecting'

export function acceptState(current: CompanionState | null, next: CompanionState, afterConnect: boolean): boolean {
  if (!current || afterConnect) return true
  if (next.source?.mode === 'replay') return true
  if (next.seq > current.seq) return true
  if (next.seq === current.seq) return (next.source?.updated_at_ms ?? 0) > (current.source?.updated_at_ms ?? 0)
  return false
}

export function useCompanionStream() {
  const state = shallowRef<CompanionState | null>(null)
  const health = shallowRef<CompanionHealth | null>(null)
  const connection = ref<Connection>('connecting')
  const lastError = ref<string | null>(null)
  const lastMessageAt = ref<number>(0)

  let es: EventSource | null = null
  let retry = 0
  let reconnectTimer: number | null = null
  let healthTimer: number | null = null
  let freshConnect = true
  let disposed = false

  function take(next: CompanionState) {
    if (!acceptState(state.value, next, freshConnect)) return
    freshConnect = false
    state.value = next
    lastMessageAt.value = Date.now()
  }

  async function fetchState() {
    try {
      const res = await fetch('/api/companion/state', { cache: 'no-store' })
      if (!res.ok) throw new Error(`state: HTTP ${res.status}`)
      take((await res.json()) as CompanionState)
      lastError.value = null
    } catch (e) {
      lastError.value = (e as Error).message
    }
  }

  async function fetchHealth() {
    try {
      const res = await fetch('/api/companion/health', { cache: 'no-store' })
      if (!res.ok) throw new Error(`health: HTTP ${res.status}`)
      health.value = (await res.json()) as CompanionHealth
    } catch (e) {
      health.value = null
      lastError.value = (e as Error).message
    }
  }

  function connect() {
    if (disposed) return
    freshConnect = true
    connection.value = retry === 0 ? 'connecting' : 'reconnecting'
    void fetchState()
    es = new EventSource('/api/companion/stream')
    es.onopen = () => {
      connection.value = 'open'
      retry = 0
      lastError.value = null
    }
    es.addEventListener('state', (ev) => {
      try {
        take(JSON.parse((ev as MessageEvent<string>).data) as CompanionState)
      } catch (e) {
        lastError.value = `stream parse: ${(e as Error).message}`
      }
    })
    es.onerror = () => {
      es?.close()
      es = null
      if (disposed) return
      connection.value = 'reconnecting'
      retry += 1
      const delay = Math.min(15000, 500 * 2 ** Math.min(retry, 5))
      reconnectTimer = window.setTimeout(connect, delay)
    }
  }

  async function replay(cmd: ReplayCommand): Promise<string | null> {
    try {
      const res = await fetch('/api/companion/replay', {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify(cmd),
      })
      if (res.status === 204) return null
      return `${res.status} ${await res.text()}`
    } catch (e) {
      return (e as Error).message
    }
  }

  onMounted(() => {
    connect()
    void fetchHealth()
    healthTimer = window.setInterval(fetchHealth, 15000)
  })

  onUnmounted(() => {
    disposed = true
    es?.close()
    if (reconnectTimer !== null) window.clearTimeout(reconnectTimer)
    if (healthTimer !== null) window.clearInterval(healthTimer)
  })

  return { state, health, connection, lastError, lastMessageAt, replay, refreshHealth: fetchHealth }
}
