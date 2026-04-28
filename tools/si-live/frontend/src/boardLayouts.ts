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
 * Board A Balanced — ocean runs along the LEFT edge of the Wiki PNG.
 * Positions hand-tuned from the transparent PNG at
 * `/board-images/A-balanced.png` (1500×1043). Coords are 0..100 where
 * (0, 0) = top-left of the rendered image and (100, 100) = bottom-right.
 *
 * Land placements (reading the PNG):
 *   1 — upper-left jungle
 *   2 — upper-center-left wetland (coastal; the starting-city land)
 *   3 — left-center wetland (coastal)
 *   4 — center mountain (blight-seeded)
 *   5 — lower-center sands
 *   6 — upper-center mountain strip
 *   7 — right sands
 *   8 — right jungle (starting-town)
 */
const BOARD_A_BALANCED: BoardLayout = {
  oceanEdge: 'left',
  positions: {
    '1': { x: 30, y: 22 },                      // upper-center jungle
    '2': { x: 22, y: 42, oceanEdge: 'left' },   // left-center wetland, coastal (starting city)
    '3': { x: 18, y: 72, oceanEdge: 'left' },   // lower-left wetland/sands, coastal
    '4': { x: 52, y: 50 },                      // center mountain (blight)
    '5': { x: 58, y: 82 },                      // lower sands
    '6': { x: 52, y: 20 },                      // upper mountain strip
    '7': { x: 80, y: 60 },                      // right sands
    '8': { x: 80, y: 25 },                      // right jungle (starting town)
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
