<template>
  <div class="h-screen w-screen bg-gray-900 text-white flex flex-col overflow-hidden relative font-sans select-none">
    
    <!-- Thin drag region header — ONLY this area is draggable -->
    <div class="h-10 w-full flex-shrink-0 flex items-center justify-end px-3" data-tauri-drag-region>
      <button @click="closeWindow" class="relative z-10 text-gray-400 hover:text-white transition-colors focus:outline-none p-1 bg-gray-800/50 hover:bg-gray-700/80 rounded-full backdrop-blur-sm">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Main Content (NO drag region here) -->
    <main class="flex-1 flex flex-col items-center justify-center p-6 w-full">
      
      <div v-if="gameStore.currentWord" class="w-full max-w-sm flex flex-col items-center">
        <!-- Word display with transition -->
        <transition name="slide-fade" mode="out-in">
          <div :key="gameStore.currentWord.id + '-' + gameStore.renderKey" class="text-center w-full mb-8">
            <h2 class="text-3xl font-extrabold tracking-tight break-words" :class="displayWord.length > 15 ? 'text-2xl' : 'text-4xl'">
              {{ displayWord }}
            </h2>
            <div class="mt-2 text-xs text-gray-400 font-mono tracking-widest opacity-60 flex flex-col gap-1">
              <div>#{{ gameStore.currentWord.id }} • {{ gameStore.currentWord.percentage.toFixed(0) }}% Mastery</div>
              <div v-if="currentGroupName" class="text-[10px] uppercase text-indigo-400/70 tracking-[0.2em]">Group: {{ currentGroupName }}</div>
            </div>
          </div>
        </transition>

        <!-- Action Buttons -->
        <div class="flex gap-4 w-full justify-center">
          <button @click="handleDontKnow" class="flex-1 py-3 px-4 rounded-xl bg-rose-500/10 text-rose-400 border border-rose-500/20 hover:bg-rose-500 hover:text-white transition-all duration-200 font-semibold text-sm flex items-center justify-center gap-2 group focus:outline-none focus:ring-2 focus:ring-rose-500 focus:ring-offset-2 focus:ring-offset-gray-900 select-none">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 group-hover:-translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
            Don't Know <span class="text-[10px] opacity-50 ml-1 hidden sm:inline">(←)</span>
          </button>
          
          <button @click="handleKnow" class="flex-1 py-3 px-4 rounded-xl bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 hover:bg-emerald-500 hover:text-white transition-all duration-200 font-semibold text-sm flex items-center justify-center gap-2 group focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2 focus:ring-offset-gray-900 select-none">
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
        <h3 class="text-white font-bold mb-2">
          {{ wordsStore.words.length === 0 ? 'Dictionary is empty' : 'No words in group' }}
        </h3>
        <p class="text-sm text-gray-400">
          {{ wordsStore.words.length === 0 
              ? 'Add words in the main window to start learning.' 
              : 'Add words to this group or change active group in settings.' }}
        </p>
        <button @click="closeWindow" class="mt-6 px-4 py-2 bg-gray-800 hover:bg-gray-700 text-white text-sm rounded-lg transition-colors border border-gray-700">
          Close
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
import { useGroupsStore } from '../stores/groups'

const gameStore = useGameStore()
const settingsStore = useSettingsStore()
const wordsStore = useWordsStore()
const groupsStore = useGroupsStore()

// Computed property to show native or foreign word based on settings
const displayWord = computed(() => {
  if (!gameStore.currentWord) return ''
  return settingsStore.settings.direction === 'native_to_foreign' 
    ? gameStore.currentWord.translate 
    : gameStore.currentWord.word
})

const currentGroupName = computed(() => {
  if (settingsStore.settings.active_group_id === 'all') return 'All Groups'
  const groupId = parseInt(settingsStore.settings.active_group_id)
  const group = groupsStore.groups.find(g => g.id === groupId)
  return group ? group.name : ''
})

const handleKnow = async () => {
  try {
    await gameStore.answer(true)
  } catch (e) {
    console.error('answer(true) error:', e)
  }
}

const handleDontKnow = async () => {
  try {
    await gameStore.answer(false)
  } catch (e) {
    console.error('answer(false) error:', e)
  }
}

const closeWindow = async () => {
  await invoke('close_game_window')
}

const handleKeydown = async (e: KeyboardEvent) => {
  if (e.key === 'ArrowLeft') await handleDontKnow()
  if (e.key === 'ArrowRight') await handleKnow()
  if (e.key === 'Escape') await closeWindow()
}

const handleFocus = async () => {
  // Refetch settings and words each time the window is shown
  await settingsStore.fetchSettings()
  await wordsStore.fetchWords()
  await groupsStore.fetchGroups()
  gameStore.nextWord()
}

onMounted(async () => {
  window.addEventListener('keydown', handleKeydown)
  window.addEventListener('focus', handleFocus)

  // Load data
  await settingsStore.fetchSettings()
  await wordsStore.fetchWords()
  await groupsStore.fetchGroups()
  
  // Initialize game
  gameStore.nextWord()
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
  window.removeEventListener('focus', handleFocus)
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
