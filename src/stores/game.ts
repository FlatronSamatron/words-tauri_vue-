import { defineStore } from 'pinia'
import { ref } from 'vue'
import { emit } from '@tauri-apps/api/event'
import { useWordsStore } from './words'
import type { Word } from '../types'

export const useGameStore = defineStore('game', () => {
  const currentWord = ref<Word | null>(null)
  const wordsStore = useWordsStore()

  const renderKey = ref(0)

  function nextWord() {
    renderKey.value++
    const available = wordsStore.words
    if (available.length === 0) {
      currentWord.value = null
      return
    }

    let candidates = available
    // Prevent showing the exact same word twice in a row if there are alternatives
    if (available.length > 1 && currentWord.value) {
      candidates = available.filter(w => w.id !== currentWord.value!.id)
    }

    const weights = candidates.map(w => {
      const p = w.percentage
      if (p === 100) return 1
      if (p === 0) return 1000
      return Math.floor(1000 - (p * 9.99))
    })

    const totalWeight = weights.reduce((acc, val) => acc + val, 0)
    let random = Math.random() * totalWeight

    let cumulativeWeight = 0;
    for (let i = 0; i < weights.length; i++) {
      cumulativeWeight += weights[i];
      if (random <= cumulativeWeight) {
        currentWord.value = candidates[i]
        return
      }
    }
    currentWord.value = candidates[candidates.length - 1]
  }

  async function answer(known: boolean) {
    if (!currentWord.value) return
    
    await wordsStore.recordAnswer(currentWord.value.id, known)
    await emit('answer-recorded')
    nextWord()
  }

  return { currentWord, renderKey, nextWord, answer }
})
