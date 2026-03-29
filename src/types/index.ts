export interface Word {
  id: number
  word: string
  translate: string
  correct: number
  total: number
  created_at: string
  percentage: number
  group_id: number
}

export interface Group {
  id: number
  name: string
  created_at: string
  word_count: number
}

export interface Settings {
  interval_minutes: number
  direction: 'native_to_foreign' | 'foreign_to_native'
  active_group_id: string // "all" or number as string
}
