import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Word } from '../types'

export const useWordsStore = defineStore('words', () => {
  const words = ref<Word[]>([])

  const mapWord = (w: any): Word => ({
    ...w,
    percentage: w.total > 0 ? (w.correct / w.total) * 100 : 0
  })

  async function fetchWords() {
    const res: any[] = await invoke('get_words')
    words.value = res.map(mapWord)
  }

  async function addWord(word: string, translate: string) {
    const raw: any = await invoke('add_word', { word, translate })
    const newWord = mapWord(raw)
    words.value.unshift(newWord) // add to top
    return newWord
  }

  async function updateWord(id: number, word: string, translate: string) {
    const raw: any = await invoke('update_word', { id, word, translate })
    const updated = mapWord(raw)
    const index = words.value.findIndex(w => w.id === id)
    if (index !== -1) {
      words.value[index] = updated
    }
    return updated
  }

  async function deleteWord(id: number) {
    const success: boolean = await invoke('delete_word', { id })
    if (success) {
      words.value = words.value.filter(w => w.id !== id)
    }
  }

  async function recordAnswer(id: number, known: boolean) {
    const raw: any = await invoke('record_answer', { id, known })
    const updated = mapWord(raw)
    const index = words.value.findIndex(w => w.id === id)
    if (index !== -1) {
      words.value[index] = updated
    }
    return updated
  }

  return { words, fetchWords, addWord, updateWord, deleteWord, recordAnswer }
})
