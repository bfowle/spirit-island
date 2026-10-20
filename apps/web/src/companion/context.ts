import { inject, type ComputedRef, type InjectionKey, type Ref } from 'vue'
import type { PublicLand } from './types'

/** Cross-panel context: the selected land and a land lookup. */
export interface CompanionCtx {
  selected: Ref<string | null>
  selectLand: (id: string | null) => void
  landById: ComputedRef<Record<string, PublicLand>>
  spiritTitles: ComputedRef<Record<string, string>>
}

export const COMPANION_CTX: InjectionKey<CompanionCtx> = Symbol('companion-ctx')

export function useCompanionCtx(): CompanionCtx | null {
  return inject(COMPANION_CTX, null)
}
