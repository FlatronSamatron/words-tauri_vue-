<template>
  <div 
    class="h-screen w-screen bg-transparent flex items-center justify-center overflow-hidden font-sans select-none p-1"
    data-tauri-drag-region
  >
    <!-- Card UI -->
    <!-- Card UI -->
    <div 
      class="w-full h-full bg-gray-900 text-white flex flex-col rounded-2xl border-2 border-violet-400/25 overflow-hidden relative"
      data-tauri-drag-region
    >
      <!-- Thin drag region header -->
      <div class="h-10 w-full flex items-center justify-end px-3" data-tauri-drag-region>
        <button @click="closeWindow" class="relative z-10 text-gray-400 hover:text-white transition-colors focus:outline-none p-1 bg-gray-800/50 hover:bg-gray-700/80 rounded-full backdrop-blur-sm">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Main Content -->
      <main class="flex-1 flex flex-col items-center justify-between px-6 pb-5 pt-2 w-full overflow-hidden">

        <div v-if="gameStore.currentWord" class="w-full flex flex-col items-center justify-between h-full">
          <!-- Word zone -->
          <div class="w-full flex flex-col items-center justify-center overflow-hidden">
            <transition name="slide-fade" mode="out-in">
              <div :key="gameStore.currentWord.id + '-' + gameStore.renderKey" class="text-center w-full">
                <h2 class="font-extrabold tracking-tight truncate leading-tight text-3xl">
                  {{ displayWord }}
                </h2>
              </div>
            </transition>

            <!-- Meta -->
            <div class="mt-2 w-full flex flex-col items-center text-[10px] text-gray-400 font-mono tracking-widest opacity-60 uppercase gap-0.5">
              <div>#{{ gameStore.currentWord.id }} • {{ gameStore.currentWord.percentage.toFixed(0) }}% Mastery</div>
              <div v-if="currentGroupName" class="text-indigo-400/80">Group: {{ currentGroupName }}</div>
            </div>
          </div>

          <!-- Translation reveal zone -->
          <div
            class="h-10 relative w-full flex items-center justify-center cursor-pointer"
            @click="isRevealed = !isRevealed"
          >
            <p
              class="text-base font-semibold text-gray-200 text-center transition-all duration-500 ease-out px-4 truncate w-full"
              :style="{ filter: isRevealed ? 'blur(0px)' : 'blur(7px)' }"
            >
              {{ translateWord }}
            </p>
            <Transition name="hint-fade">
              <span
                v-if="!isRevealed"
                class="absolute text-[9px] text-gray-500 uppercase tracking-widest pointer-events-none"
              >tap to reveal</span>
            </Transition>
          </div>

          <!-- Action Buttons -->
          <div class="h-11 flex gap-3 w-full">
            <button @click="handleDontKnow" class="flex-1 h-full rounded-xl bg-rose-500/10 text-rose-400 border border-rose-500/20 hover:bg-rose-500 hover:text-white transition-all duration-200 font-semibold text-xs flex items-center justify-center focus:outline-none select-none">
              Don't Know
            </button>
            <button @click="handleKnow" class="flex-1 h-full rounded-xl bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 hover:bg-emerald-500 hover:text-white transition-all duration-200 font-semibold text-xs flex items-center justify-center focus:outline-none select-none">
              Know
            </button>
          </div>
        </div>

        <!-- Empty State -->
        <div v-else class="text-center w-full max-w-sm px-4">
          <div class="mb-4 text-gray-600 inline-block bg-gray-800 rounded-full p-3">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
            </svg>
          </div>
          <h3 class="text-white font-bold mb-1 text-sm">
            {{ wordsStore.words.length === 0 ? 'Dictionary is empty' : 'No words in group' }}
          </h3>
          <p class="text-[11px] text-gray-400 leading-tight">
            {{ wordsStore.words.length === 0 
                ? 'Add words in the main window to start learning.' 
                : 'Add words to this group or change active group in settings.' }}
          </p>
          <button @click="closeWindow" class="mt-4 px-3 py-1.5 bg-gray-800 hover:bg-gray-700 text-white text-xs rounded-lg transition-colors border border-gray-700">
            Close
          </button>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useGameStore } from '../stores/game'
import { useSettingsStore } from '../stores/settings'
import { useWordsStore } from '../stores/words'
import { useGroupsStore } from '../stores/groups'

const gameStore = useGameStore()
const settingsStore = useSettingsStore()
const wordsStore = useWordsStore()
const groupsStore = useGroupsStore()

const isRevealed = ref(false)
watch(() => gameStore.renderKey, () => { isRevealed.value = false })

// Computed property to show native or foreign word based on settings
const displayWord = computed(() => {
  if (!gameStore.currentWord) return ''
  return settingsStore.settings.direction === 'native_to_foreign' 
    ? gameStore.currentWord.translate 
    : gameStore.currentWord.word
})

const translateWord = computed(() => {
  if (!gameStore.currentWord) return ''
  return settingsStore.settings.direction === 'native_to_foreign'
    ? gameStore.currentWord.word
    : gameStore.currentWord.translate
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
  if (e.key === ' ') { isRevealed.value = true; return }
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

.hint-fade-enter-active,
.hint-fade-leave-active {
  transition: opacity 0.3s ease;
}
.hint-fade-enter-from,
.hint-fade-leave-to {
  opacity: 0;
}
</style>
