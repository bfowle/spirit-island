/**
 * Schematic island layout from topology alone.
 *
 * The engine stores no land coordinates, so positions are computed,
 * deterministically, from three public facts: which board a land is on, its
 * land number, and the adjacency list.
 *
 * 1. Seed. Boards sit around a center with their Ocean facing outward (one
 *    board: Ocean on the left; two: side by side, Oceans on the outer edges;
 *    three or more: a ring). Inside a board, land number sets the depth:
 *    0 = Ocean at the outer edge, 1–3 coastal, 4–6 middle, 7–8 far inland.
 *    The order of lands along each row is the permutation with the fewest
 *    crossing adjacency edges (ties: shortest total edge length), and each
 *    later board is mirrored or not to shorten its cross-board edges.
 * 2. Relax. A fixed number of force iterations: springs on adjacency edges
 *    (cross-board edges pull boards together), box-collision repulsion, and a
 *    weak pull back to the seed so boards keep their orientation. No
 *    randomness, so the same game always draws the same map.
 * 3. Resolve. Remaining box overlaps are pushed apart along their smaller
 *    overlap axis, then the drawing is translated to start at a margin.
 */

export interface LayoutLand {
  id: string
  board: string
  number: number
  is_ocean: boolean
}

export interface NodeBox {
  id: string
  board: string
  number: number
  ocean: boolean
  x: number
  y: number
  w: number
  h: number
}

export interface LayoutEdge {
  a: string
  b: string
  cross: boolean
}

export interface BoardHull {
  board: string
  points: [number, number][]
  labelX: number
  labelY: number
}

export interface IslandLayout {
  nodes: Record<string, NodeBox>
  edges: LayoutEdge[]
  hulls: BoardHull[]
  width: number
  height: number
}

export const CELL_W = 158
export const CELL_H = 124
const OCEAN_THICK = 54
const OCEAN_LONG = 250
const GAP = 22
const MARGIN = 30
const ITERATIONS = 320

type Vec = { x: number; y: number }

function permutations<T>(xs: T[]): T[][] {
  if (xs.length <= 1) return [xs.slice()]
  const out: T[][] = []
  xs.forEach((x, i) => {
    const rest = xs.slice(0, i).concat(xs.slice(i + 1))
    for (const p of permutations(rest)) out.push([x, ...p])
  })
  return out
}

function segmentsCross(a: Vec, b: Vec, c: Vec, d: Vec): boolean {
  const o = (p: Vec, q: Vec, r: Vec) => Math.sign((q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x))
  const o1 = o(a, b, c), o2 = o(a, b, d), o3 = o(c, d, a), o4 = o(c, d, b)
  return o1 * o2 < 0 && o3 * o4 < 0
}

/** Depth (outward +) and row of a land number. */
function rowOf(n: number): 'ocean' | 'coast' | 'mid' | 'inner' | 'extra' {
  if (n === 0) return 'ocean'
  if (n >= 1 && n <= 3) return 'coast'
  if (n >= 4 && n <= 6) return 'mid'
  if (n >= 7 && n <= 8) return 'inner'
  return 'extra'
}

const DEPTH: Record<string, number> = { ocean: 1.62, coast: 1, mid: 0, inner: -1, extra: -1.8 }
const SLOTS: Record<string, number[]> = {
  coast: [-1, 0, 1],
  mid: [-1, 0, 1],
  inner: [-0.5, 0.5],
}

/** Local (depth, tangent) seed for each land of one board, best row order. */
function seedBoard(lands: LayoutLand[], adj: [string, string][]): Map<string, { d: number; t: number }> {
  const byRow: Record<string, LayoutLand[]> = { ocean: [], coast: [], mid: [], inner: [], extra: [] }
  for (const l of [...lands].sort((a, b) => a.number - b.number)) byRow[l.is_ocean ? 'ocean' : rowOf(l.number)].push(l)

  const ids = new Set(lands.map((l) => l.id))
  const edges = adj.filter(([a, b]) => ids.has(a) && ids.has(b))

  const fixed = new Map<string, { d: number; t: number }>()
  byRow.ocean.forEach((l, i) => fixed.set(l.id, { d: DEPTH.ocean, t: (i - (byRow.ocean.length - 1) / 2) * 2.4 }))
  byRow.coast.forEach((l, i) => fixed.set(l.id, { d: DEPTH.coast, t: SLOTS.coast[i] ?? i - 1 }))
  byRow.extra.forEach((l, i) => fixed.set(l.id, { d: DEPTH.extra, t: i - (byRow.extra.length - 1) / 2 }))

  const midPerms = permutations(byRow.mid)
  const innerPerms = permutations(byRow.inner)
  let best: Map<string, { d: number; t: number }> | null = null
  let bestScore = Infinity
  for (const mp of midPerms) {
    for (const ip of innerPerms) {
      const m = new Map(fixed)
      mp.forEach((l, i) => m.set(l.id, { d: DEPTH.mid, t: SLOTS.mid[i] ?? i - 1 }))
      ip.forEach((l, i) => m.set(l.id, { d: DEPTH.inner, t: SLOTS.inner[i] ?? i - 0.5 }))
      const pt = (id: string): Vec => {
        const s = m.get(id)!
        return { x: s.d * 1.3, y: s.t }
      }
      let crossings = 0
      let length = 0
      for (let i = 0; i < edges.length; i++) {
        const [a, b] = edges[i]
        const pa = pt(a), pb = pt(b)
        length += Math.hypot(pa.x - pb.x, pa.y - pb.y)
        for (let j = i + 1; j < edges.length; j++) {
          const [c, d] = edges[j]
          if (a === c || a === d || b === c || b === d) continue
          if (segmentsCross(pa, pb, pt(c), pt(d))) crossings++
        }
      }
      const score = crossings * 1000 + length
      if (score < bestScore - 1e-9) {
        bestScore = score
        best = m
      }
    }
  }
  return best ?? fixed
}

function boxSize(ocean: boolean, radial: Vec): { w: number; h: number } {
  if (!ocean) return { w: CELL_W, h: CELL_H }
  // The Ocean is a strip along the board's outer edge.
  return Math.abs(radial.x) >= Math.abs(radial.y) ? { w: OCEAN_THICK, h: OCEAN_LONG } : { w: OCEAN_LONG, h: OCEAN_THICK }
}

function convexHull(points: [number, number][]): [number, number][] {
  const pts = [...points].sort((a, b) => a[0] - b[0] || a[1] - b[1])
  if (pts.length < 3) return pts
  const cross = (o: [number, number], a: [number, number], b: [number, number]) =>
    (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
  const lower: [number, number][] = []
  for (const p of pts) {
    while (lower.length >= 2 && cross(lower[lower.length - 2], lower[lower.length - 1], p) <= 0) lower.pop()
    lower.push(p)
  }
  const upper: [number, number][] = []
  for (let i = pts.length - 1; i >= 0; i--) {
    const p = pts[i]
    while (upper.length >= 2 && cross(upper[upper.length - 2], upper[upper.length - 1], p) <= 0) upper.pop()
    upper.push(p)
  }
  upper.pop()
  lower.pop()
  return lower.concat(upper)
}

export function computeLayout(
  lands: LayoutLand[],
  adjacency: [string, string][],
  boardOrder: string[],
): IslandLayout {
  const boards = [...boardOrder]
  for (const l of lands) if (!boards.includes(l.board)) boards.push(l.board)
  const used = boards.filter((b) => lands.some((l) => l.board === b))
  const n = used.length
  const landIds = new Set(lands.map((l) => l.id))
  const adj = adjacency.filter(([a, b]) => landIds.has(a) && landIds.has(b))
  const boardOf = new Map(lands.map((l) => [l.id, l.board]))

  const S_R = CELL_W + GAP + 8 // radial spacing
  const S_T = CELL_H + GAP + 14 // tangential spacing

  // ---- 1. seed
  const seeds = new Map<string, Vec>()
  const radialOf = new Map<string, Vec>()
  const centers: Vec[] = []
  used.forEach((b, i) => {
    const theta = n === 1 ? Math.PI : n === 2 ? (i === 0 ? Math.PI : 0) : Math.PI + (2 * Math.PI * i) / n
    const r: Vec = { x: Math.cos(theta), y: Math.sin(theta) }
    const t: Vec = { x: -Math.sin(theta), y: Math.cos(theta) }
    const dist = n === 1 ? 0 : n === 2 ? S_R * 1.55 : (S_T * 1.9) / Math.sin(Math.PI / n)
    const c: Vec = { x: r.x * dist, y: r.y * dist }
    centers.push(c)
    const local = seedBoard(lands.filter((l) => l.board === b), adj)
    const place = (mirror: number) => {
      const out = new Map<string, Vec>()
      for (const [id, s] of local) {
        out.set(id, {
          x: c.x + r.x * s.d * S_R + t.x * s.t * S_T * mirror,
          y: c.y + r.y * s.d * S_R + t.y * s.t * S_T * mirror,
        })
      }
      return out
    }
    let mirror: number
    if (i === 0) {
      // First board: land 1 toward the top (or the left when the board is rotated).
      const ty = Math.abs(t.y) > 1e-6 ? t.y : t.x
      mirror = ty > 0 ? 1 : -1
    } else {
      const cost = (m: number) => {
        const p = place(m)
        let sum = 0
        for (const [a, bb] of adj) {
          const pa = p.get(a) ?? seeds.get(a)
          const pb = p.get(bb) ?? seeds.get(bb)
          if (!pa || !pb) continue
          if (p.has(a) !== p.has(bb)) sum += Math.hypot(pa.x - pb.x, pa.y - pb.y)
        }
        return sum
      }
      mirror = cost(1) <= cost(-1) ? 1 : -1
    }
    for (const [id, v] of place(mirror)) {
      seeds.set(id, v)
      radialOf.set(id, r)
    }
  })

  const nodes: NodeBox[] = lands.map((l) => {
    const s = seeds.get(l.id) ?? { x: 0, y: 0 }
    const size = boxSize(l.is_ocean, radialOf.get(l.id) ?? { x: -1, y: 0 })
    return { id: l.id, board: l.board, number: l.number, ocean: l.is_ocean, x: s.x, y: s.y, ...size }
  })
  const index = new Map(nodes.map((nd, i) => [nd.id, i]))

  // ---- 2. relax
  const contact = (a: NodeBox, b: NodeBox, ux: number, uy: number) => {
    // Center distance at which the two boxes touch along direction u.
    const ax = Math.abs(ux) > 1e-6 ? (a.w / 2 + b.w / 2) / Math.abs(ux) : Infinity
    const ay = Math.abs(uy) > 1e-6 ? (a.h / 2 + b.h / 2) / Math.abs(uy) : Infinity
    return Math.min(ax, ay)
  }
  for (let it = 0; it < ITERATIONS; it++) {
    const temp = 1 - it / ITERATIONS
    const fx = new Float64Array(nodes.length)
    const fy = new Float64Array(nodes.length)
    // springs
    for (const [a, b] of adj) {
      const i = index.get(a)!, j = index.get(b)!
      const A = nodes[i], B = nodes[j]
      let dx = B.x - A.x, dy = B.y - A.y
      let d = Math.hypot(dx, dy)
      if (d < 1e-6) { dx = 1; dy = 0; d = 1 }
      const ux = dx / d, uy = dy / d
      const rest = contact(A, B, ux, uy) + GAP + 6
      const cross = boardOf.get(a) !== boardOf.get(b)
      const k = (A.ocean || B.ocean ? 0.02 : cross ? 0.07 : 0.05) * (d - rest)
      fx[i] += k * ux; fy[i] += k * uy
      fx[j] -= k * ux; fy[j] -= k * uy
    }
    // collisions
    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const A = nodes[i], B = nodes[j]
        const dx = B.x - A.x, dy = B.y - A.y
        const ox = (A.w + B.w) / 2 + GAP - Math.abs(dx)
        const oy = (A.h + B.h) / 2 + GAP - Math.abs(dy)
        if (ox > 0 && oy > 0) {
          const d = Math.hypot(dx, dy) || 1
          const push = 0.25 * Math.min(ox, oy)
          const ux = d > 1e-6 ? dx / d : 1, uy = d > 1e-6 ? dy / d : 0
          fx[i] -= push * ux; fy[i] -= push * uy
          fx[j] += push * ux; fy[j] += push * uy
        }
      }
    }
    // anchor to seed
    nodes.forEach((nd, i) => {
      const s = seeds.get(nd.id)!
      const k = nd.ocean ? 0.12 : 0.025
      fx[i] += k * (s.x - nd.x)
      fy[i] += k * (s.y - nd.y)
    })
    nodes.forEach((nd, i) => {
      const step = Math.hypot(fx[i], fy[i])
      const cap = 24 * temp + 1
      const scale = step > cap ? cap / step : 1
      nd.x += fx[i] * scale * (0.4 + 0.6 * temp)
      nd.y += fy[i] * scale * (0.4 + 0.6 * temp)
    })
  }

  // ---- 3. resolve overlaps
  for (let pass = 0; pass < 200; pass++) {
    let moved = false
    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const A = nodes[i], B = nodes[j]
        const dx = B.x - A.x, dy = B.y - A.y
        const ox = (A.w + B.w) / 2 + GAP - Math.abs(dx)
        const oy = (A.h + B.h) / 2 + GAP - Math.abs(dy)
        if (ox > 0.5 && oy > 0.5) {
          moved = true
          const wa = A.ocean ? 0.2 : 0.5, wb = B.ocean ? 0.2 : 0.5
          const tot = wa + wb
          if (ox < oy) {
            const s = dx >= 0 ? 1 : -1
            A.x -= (s * ox * wa) / tot; B.x += (s * ox * wb) / tot
          } else {
            const s = dy >= 0 ? 1 : -1
            A.y -= (s * oy * wa) / tot; B.y += (s * oy * wb) / tot
          }
        }
      }
    }
    if (!moved) break
  }

  // ---- bounds + translate
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity
  for (const nd of nodes) {
    minX = Math.min(minX, nd.x - nd.w / 2); maxX = Math.max(maxX, nd.x + nd.w / 2)
    minY = Math.min(minY, nd.y - nd.h / 2); maxY = Math.max(maxY, nd.y + nd.h / 2)
  }
  if (!Number.isFinite(minX)) { minX = 0; minY = 0; maxX = 0; maxY = 0 }
  const top = MARGIN + 18 // room for board labels
  for (const nd of nodes) {
    nd.x += MARGIN - minX
    nd.y += top - minY
  }

  const hullPad = 12
  const hulls: BoardHull[] = used.map((b) => {
    const pts: [number, number][] = []
    for (const nd of nodes.filter((x) => x.board === b)) {
      const hw = nd.w / 2 + hullPad, hh = nd.h / 2 + hullPad
      pts.push([nd.x - hw, nd.y - hh], [nd.x + hw, nd.y - hh], [nd.x + hw, nd.y + hh], [nd.x - hw, nd.y + hh])
    }
    const hull = convexHull(pts)
    const landsOnly = nodes.filter((x) => x.board === b && !x.ocean)
    const cx = landsOnly.length ? landsOnly.reduce((s, x) => s + x.x, 0) / landsOnly.length : 0
    const yTop = hull.length ? Math.min(...hull.map((p) => p[1])) : 0
    return { board: b, points: hull, labelX: cx, labelY: yTop - 8 }
  })

  const edges: LayoutEdge[] = adj.map(([a, b]) => ({ a, b, cross: boardOf.get(a) !== boardOf.get(b) }))
  const out: Record<string, NodeBox> = {}
  for (const nd of nodes) out[nd.id] = nd
  return {
    nodes: out,
    edges,
    hulls,
    width: maxX - minX + 2 * MARGIN,
    height: maxY - minY + MARGIN + top,
  }
}
