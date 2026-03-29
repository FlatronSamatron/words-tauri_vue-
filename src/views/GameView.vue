<template>
  <div class="h-screen w-screen bg-gray-900 text-white flex flex-col overflow-hidden relative font-sans select-none"
       ref="gameContainer">
    
    <!-- Header -->
    <div class="absolute top-0 left-0 right-0 p-3 flex justify-end z-10" data-tauri-drag-region>
      <button @click="closeWindow" data-tauri-drag-region="false" class="text-gray-400 hover:text-white transition-colors focus:outline-none p-1 bg-gray-800/50 hover:bg-gray-700/80 rounded-full backdrop-blur-sm">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Main Content -->
    <main class="flex-1 flex flex-col items-center justify-center p-6 w-full" data-tauri-drag-region>
      
      <div v-if="gameStore.currentWord" class="w-full max-w-sm flex flex-col items-center">
        <!-- Word display with transition -->
        <transition name="slide-fade" mode="out-in">
          <div :key="gameStore.currentWord.id + '-' + gameStore.renderKey" class="text-center w-full mb-8">
            <h2 class="text-3xl font-extrabold tracking-tight break-words" :class="displayWord.length > 15 ? 'text-2xl' : 'text-4xl'">
              {{ displayWord }}
            </h2>
            <div class="mt-2 text-xs text-gray-400 font-mono tracking-widest opacity-60">
              #{{ gameStore.currentWord.id }} • {{ gameStore.currentWord.percentage.toFixed(0) }}% Mastery
            </div>
          </div>
        </transition>

        <!-- Action Buttons -->
        <div class="flex gap-4 w-full justify-center">
          <button @click="handleDontKnow" data-tauri-drag-region="false" class="flex-1 py-3 px-4 rounded-xl bg-rose-500/10 text-rose-400 border border-rose-500/20 hover:bg-rose-500 hover:text-white transition-all duration-200 font-semibold text-sm flex items-center justify-center gap-2 group focus:outline-none focus:ring-2 focus:ring-rose-500 focus:ring-offset-2 focus:ring-offset-gray-900 select-none">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 group-hover:-translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
            Don't Know <span class="text-[10px] opacity-50 ml-1 hidden sm:inline">(←)</span>
          </button>
          
          <button @click="handleKnow" data-tauri-drag-region="false" class="flex-1 py-3 px-4 rounded-xl bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 hover:bg-emerald-500 hover:text-white transition-all duration-200 font-semibold text-sm flex items-center justify-center gap-2 group focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2 focus:ring-offset-gray-900 select-none">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 group-hover:translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            Know <span class="text-[10px] opacity-50 ml-1 hidden sm:inline">(→)</span>
          </button>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else class="text-center w-full max-w-sm px-4">
        <div class="mb-4 text-gray-600 inline-block bg-gray-800 rounded-full p-4">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
        </div>
        <h3 class="text-white font-bold mb-2">Добавьте слова</h3>
        <p class="text-sm text-gray-400">Словарь пуст. Добавьте новые слова в настройках.</p>
        <button @click="closeWindow" data-tauri-drag-region="false" class="mt-6 px-4 py-2 bg-gray-800 hover:bg-gray-700 text-white text-sm rounded-lg transition-colors border border-gray-700">
          Закрыть
        </button>
      </div>
      
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useGameStore } from '../stores/game'
import { useSettingsStore } from '../stores/settings'
import { useWordsStore } from '../stores/words'

const gameStore = useGameStore()
const settingsStore = useSettingsStore()
const wordsStore = useWordsStore()

// Computed property to show native or foreign word based on settings
const displayWord = computed(() => {
  if (!gameStore.currentWord) return ''
  return settingsStore.settings.direction === 'native_to_foreign' 
    ? gameStore.currentWord.word 
    : gameStore.currentWord.translate
})

const handleKnow = async () => {
  await gameStore.answer(true)
}

const handleDontKnow = async () => {
  await gameStore.answer(false)
}

const closeWindow = async () => {
  await invoke('close_game_window')
}

const handleKeydown = async (e: KeyboardEvent) => {
  if (e.key === 'ArrowLeft') await handleDontKnow()
  if (e.key === 'ArrowRight') await handleKnow()
  if (e.key === 'Escape') await closeWindow()
}

onMounted(async () => {
  window.addEventListener('keydown', handleKeydown)

  // Load data
  await settingsStore.fetchSettings()
  await wordsStore.fetchWords()
  
  // Initialize game
  gameStore.nextWord()
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.2s cubic-bezier(1, 0.5, 0.8, 1);
}

.slide-fade-enter-from {
  transform: translateX(20px);
  opacity: 0;
}

.slide-fade-leave-to {
  transform: translateX(-20px);
  opacity: 0;
}

/* Base styles for Tauri drag region support - we added it via inline attributes */
</style>
