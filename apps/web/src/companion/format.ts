/**
 * Display helpers for the companion dashboard. Pure functions, no state.
 */
import type { PublicInvaderCard } from './types'

const TERRAINS = ['Mountain', 'Wetland', 'Jungle', 'Sands', 'Ocean']

/** "MountainC3" → { terrain: "Mountain", board: "C", number: 3, short: "C3" }. */
export function parseLand(id: string): { terrain: string; board: string; number: number; short: string } {
  const m = /^([A-Za-z]*?)([A-Z])(\d+)$/.exec(id)
  if (!m) return { terrain: '', board: '', number: 0, short: id }
  return { terrain: m[1], board: m[2], number: Number(m[3]), short: `${m[2]}${m[3]}` }
}

export function landShort(id: string): string {
  return parseLand(id).short
}

/** CSS color for a terrain (design-token backed). */
export function terrainColor(terrain: string): string {
  switch (terrain) {
    case 'Mountain': return 'var(--terrain-mountain)'
    case 'Wetland': return 'var(--terrain-wetland)'
    case 'Jungle': return 'var(--terrain-jungle)'
    case 'Sands': return 'var(--terrain-sands)'
    case 'Ocean':
    case 'Coastal': return 'var(--terrain-ocean)'
    default: return 'var(--text-muted)'
  }
}

export function terrainIcon(terrain: string): string | null {
  const t = terrain.toLowerCase()
  return ['mountain', 'wetland', 'jungle', 'sands'].includes(t) ? `terrain-${t}` : null
}

const ROMAN = ['0', 'I', 'II', 'III', 'IV', 'V']
export function roman(n: number | null | undefined): string {
  if (n == null) return '–'
  return ROMAN[n] ?? String(n)
}

/** "MountainWetland3" → terrains ["Mountain", "Wetland"], stage 3. */
export function parseInvaderId(id: string): PublicInvaderCard {
  const m = /^(.*?)(\d)$/.exec(id)
  const body = m ? m[1] : id
  const stage = m ? Number(m[2]) : 0
  const terrains = body.match(/[A-Z][a-z]+/g) ?? [body]
  return { id, stage, terrains }
}

export function invaderLabel(card: PublicInvaderCard): string {
  return card.terrains.map(prettyTerrain).join(' + ')
}

function prettyTerrain(t: string): string {
  return t === 'SaltDeposits' ? 'Salt Deposits' : t
}

/** "FastPower" → "Fast Power", "LoseTooMuchBlight" → "Lose too much blight". */
export function splitCamel(s: string, sentence = false): string {
  const words = s.replace(/([a-z])([A-Z])/g, '$1 $2').replace(/([A-Z]+)([A-Z][a-z])/g, '$1 $2').split(' ')
  if (!sentence) return words.join(' ')
  return words.map((w, i) => (i === 0 ? w : w.toLowerCase())).join(' ')
}

export function stepLabel(step: string): string {
  if (!step) return '—'
  return splitCamel(step)
}

/**
 * Light touch on backend sentences: "MountainC3" → "Mountain C3",
 * "{Explorer}" → "Explorer". Meaning is unchanged.
 */
export function prettyText(s: string): string {
  return s
    .replace(new RegExp(`\\b(${TERRAINS.join('|')})([A-H])(\\d)\\b`, 'g'), '$1 $2$3')
    .replace(/\{([A-Za-z ]+)\}/g, '$1')
}

/** Strip simple HTML the engine leaves in rule text (<i>…</i>). */
export function plainRule(s: string): string {
  return prettyText(s.replace(/<\/?[a-z]+>/gi, ''))
}

export function elementIcon(el: string): string {
  return `element-${el.toLowerCase()}`
}

export const ELEMENTS = ['Sun', 'Moon', 'Fire', 'Air', 'Water', 'Earth', 'Plant', 'Animal']

export function pct(p: number): string {
  if (!Number.isFinite(p)) return '–'
  const v = p * 100
  if (v > 0 && v < 1) return '<1%'
  if (v < 100 && v > 99) return '>99%'
  return `${Math.round(v)}%`
}

export function plural(n: number, one: string, many = `${one}s`): string {
  return `${n} ${n === 1 ? one : many}`
}

/** Invader / piece kind → icon name. */
export function pieceIcon(kind: string): string | null {
  switch (kind) {
    case 'Explorer': return 'unit-explorer'
    case 'Town': return 'unit-town'
    case 'City': return 'unit-city'
    case 'Dahan': return 'unit-dahan'
    case 'Blight': return 'resource-blight'
    case 'Fear': return 'resource-fear'
    default: return null
  }
}

/** Short "Shadows Flicker Like Flame" → "Shadows Flicker Like Flame" unless an id. */
export function spiritName(id: string, titles: Record<string, string>): string {
  return titles[id] ?? splitCamel(id)
}

/** Ending code → plain words. */
export function endingLabel(code: string): string {
  return splitCamel(code, true)
}

/** "SandsC6 (Wilds)" → { land: "SandsC6", reason: "Wilds" }. */
export function parseBlocked(s: string): { land: string; reason: string | null } {
  const m = /^(\S+)\s*\((.+)\)$/.exec(s)
  return m ? { land: m[1], reason: m[2] } : { land: s, reason: null }
}
