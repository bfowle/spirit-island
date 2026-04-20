/**
 * Canonical presence-disc colors per spirit.
 * Sourced from physical Spirit Island panel inks + community imagery.
 * Users can override per-spirit via the SpiritPanel color picker.
 */

export interface SpiritPalette {
  primary: string    // main disc color
  highlight: string  // 30% lighter edge/rim
  shadow: string     // 30% darker underside
}

/** Convert a hex to rough highlight/shadow tones using simple math. */
function palette(hex: string): SpiritPalette {
  const h = hex.replace('#', '')
  const r = parseInt(h.slice(0, 2), 16)
  const g = parseInt(h.slice(2, 4), 16)
  const b = parseInt(h.slice(4, 6), 16)
  const lighten = (v: number, amt = 60) => Math.min(255, v + amt)
  const darken = (v: number, amt = 40) => Math.max(0, v - amt)
  const hx = (v: number) => {
    const h = v.toString(16)
    return h.length < 2 ? '0' + h : h
  }
  return {
    primary: hex,
    highlight: `#${hx(lighten(r))}${hx(lighten(g))}${hx(lighten(b))}`,
    shadow: `#${hx(darken(r))}${hx(darken(g))}${hx(darken(b))}`,
  }
}

/** Canonical 12-color Spirit Island-ish palette — the physical discs come
 *  in roughly these hues; glass/etsy replacements echo them. */
export const STOCK_COLORS: Record<string, string> = {
  red: '#c04040',
  orange: '#d96a30',
  amber: '#d9a63c',
  yellow: '#e1cc4d',
  olive: '#8a8a3c',
  green: '#4a8a3c',
  teal: '#3a9a8a',
  cyan: '#4ab0c8',
  blue: '#3a6ab0',
  indigo: '#5a4a9a',
  purple: '#7a4a9a',
  magenta: '#b04a9a',
  pink: '#d97aa6',
  white: '#e4e4d4',
  black: '#2a2a2e',
}

/** Best-guess default color per spirit slug. If a slug isn't listed,
 *  the UI falls back to the 'indigo' stock color. */
export const SPIRIT_DEFAULT_COLOR: Record<string, string> = {
  // Base game (8)
  'lightning-swift-strike': STOCK_COLORS.yellow,
  'river-surges-in-sunlight': STOCK_COLORS.cyan,
  'shadows-flicker-like-flame': STOCK_COLORS.purple,
  'vital-strength-of-the-earth': STOCK_COLORS.orange,
  'ocean-hungry-grasp': STOCK_COLORS.blue,
  'a-spread-of-rampant-green': STOCK_COLORS.green,
  'thunderspeaker': STOCK_COLORS.white,
  'bringer-of-dreams-and-nightmares': STOCK_COLORS.magenta,

  // Branch & Claw
  'sharp-fangs-behind-the-leaves': STOCK_COLORS.red,
  'keeper-of-the-forbidden-wilds': STOCK_COLORS.olive,

  // Promo
  'heart-of-the-wildfire': STOCK_COLORS.orange,
  'serpent-slumbering-beneath-the-island': STOCK_COLORS.amber,
  'finder-of-paths-unseen': STOCK_COLORS.indigo,
  'downpour-drenches-the-world': STOCK_COLORS.teal,

  // Jagged Earth
  'many-minds-move-as-one': STOCK_COLORS.olive,
  'stone-unyielding-defiance': STOCK_COLORS.amber,
  'lure-of-the-deep-wilderness': STOCK_COLORS.indigo,
  'fractured-days-split-the-sky': STOCK_COLORS.pink,
  'starlight-seeks-its-form': STOCK_COLORS.white,
  'volcano-looming-high': STOCK_COLORS.red,
  'grinning-trickster': STOCK_COLORS.magenta,
  'shifting-memory-of-ages': STOCK_COLORS.teal,
  'shroud-of-silent-mist': STOCK_COLORS.white,
  'vengeance-burning-plague': STOCK_COLORS.olive,
  'devouring-teeth-lurk-underfoot': STOCK_COLORS.amber,
  'eyes-watch-from-the-trees': STOCK_COLORS.green,
  'fathomless-mud-of-the-swamp': STOCK_COLORS.olive,
  'rising-heat-of-stone-and-sand': STOCK_COLORS.red,
  'sun-bright-whirlwind': STOCK_COLORS.yellow,

  // Nature Incarnate
  'wounded-waters-bleeding': STOCK_COLORS.red,
  'dances-up-earthquakes': STOCK_COLORS.amber,
  'ember-eyed-behemoth': STOCK_COLORS.orange,
  'relentless-gaze-of-the-sun': STOCK_COLORS.yellow,
  'towering-roots-of-the-jungle': STOCK_COLORS.green,
  'hearth-vigil': STOCK_COLORS.amber,
  'breath-of-darkness': STOCK_COLORS.indigo,
  'wandering-voice': STOCK_COLORS.cyan,
}

export function getPaletteForSpirit(slug: string, overrideColor?: string): SpiritPalette {
  return palette(overrideColor || SPIRIT_DEFAULT_COLOR[slug] || STOCK_COLORS.indigo)
}

export type DiscStyle = 'glass' | 'wood' | 'solid'

export const DEFAULT_DISC_STYLE: DiscStyle = 'glass'
