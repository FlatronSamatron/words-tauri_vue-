export interface Word {
  id: number
  word: string
  translate: string
  correct: number
  total: number
  created_at: string
  percentage: number
}

export interface Settings {
  interval_minutes: number
  direction: 'native_to_foreign' | 'foreign_to_native'
}
