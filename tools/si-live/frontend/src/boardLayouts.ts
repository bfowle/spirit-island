/**
 * Approximate XY positions (on a 0..100 square canvas) for each land per
 * board + variant. Used by Board.vue's Map view to render lands where they
 * physically sit on the panel instead of a uniform grid.
 *
 * Origin: top-left (0, 0); bottom-right (100, 100). Ocean strip runs along
 * one edge — lands 1-based, ocean = "0".
 *
 * These are hand-eyeballed from Wiki board images (close enough for click-
 * edit navigation; not pixel-perfect). Falls back to grid rendering when
 * a variant's positions aren't defined.
 */

export interface LandPos {
  x: number
  y: number
  /** Ocean edge if applicable: 'top' | 'bottom' | 'left' | 'right'. */
  oceanEdge?: 'top' | 'bottom' | 'left' | 'right'
}

export interface BoardLayout {
  /** Which side of the board is the ocean on? */
  oceanEdge: 'top' | 'bottom' | 'left' | 'right'
  positions: Record<string, LandPos>
}

/**
 * Board A Balanced. Boards are tall rectangles with ocean along the top
 * (for Base A/B — ocean on top/left vs bottom/right depending). Approximate
 * coords based on the Wiki board image.
 */
const BOARD_A_BALANCED: BoardLayout = {
  oceanEdge: 'top',
  positions: {
    '1': { x: 22, y: 35, oceanEdge: 'top' },   // mountain, coastal
    '2': { x: 52, y: 22, oceanEdge: 'top' },   // wetland, coastal (city + dahan)
    '3': { x: 78, y: 30, oceanEdge: 'top' },   // jungle, coastal (2 dahan)
    '4': { x: 48, y: 48 },                     // sands, inland (blight)
    '5': { x: 22, y: 60 },                     // wetland, inland
    '6': { x: 32, y: 82 },                     // mountain, inland (dahan)
    '7': { x: 60, y: 78 },                     // sands, inland (2 dahan)
    '8': { x: 82, y: 62 },                     // jungle, inland (town)
  },
}

const BOARD_A_THEMATIC: BoardLayout = {
  oceanEdge: 'top',
  positions: {
    '1': { x: 18, y: 28, oceanEdge: 'top' },
    '2': { x: 40, y: 20, oceanEdge: 'top' },
    '3': { x: 60, y: 32, oceanEdge: 'top' },
    '4': { x: 82, y: 30, oceanEdge: 'top' },
    '5': { x: 42, y: 46 },
    '6': { x: 20, y: 64 },
    '7': { x: 50, y: 72 },
    '8': { x: 80, y: 58 },
    '9': { x: 72, y: 86 },
  },
}

/**
 * Board B — ocean on top. Approximate positions based on Wiki image.
 */
const BOARD_B_BALANCED: BoardLayout = {
  oceanEdge: 'top',
  positions: {
    '1': { x: 20, y: 32, oceanEdge: 'top' },
    '2': { x: 50, y: 22, oceanEdge: 'top' },
    '3': { x: 80, y: 34, oceanEdge: 'top' },
    '4': { x: 36, y: 52 },
    '5': { x: 66, y: 50 },
    '6': { x: 22, y: 72 },
    '7': { x: 50, y: 82 },
    '8': { x: 78, y: 70 },
  },
}

const BOARD_C_BALANCED: BoardLayout = BOARD_B_BALANCED
const BOARD_D_BALANCED: BoardLayout = BOARD_B_BALANCED

export const BOARD_LAYOUTS: Record<string, Record<string, BoardLayout>> = {
  A: { balanced: BOARD_A_BALANCED, thematic: BOARD_A_THEMATIC },
  B: { balanced: BOARD_B_BALANCED },
  C: { balanced: BOARD_C_BALANCED },
  D: { balanced: BOARD_D_BALANCED },
}

export function getLayout(boardId: string, variant: string | undefined): BoardLayout | null {
  const byId = BOARD_LAYOUTS[boardId]
  if (!byId) return null
  const v = variant && byId[variant]
  return v || byId.balanced || null
}
