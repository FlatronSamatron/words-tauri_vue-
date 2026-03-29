<template>
  <div class="h-full w-full flex flex-col p-6 space-y-6 overflow-y-auto">
    <div class="flex justify-between items-end mb-2">
      <div>
        <h1 class="text-3xl font-extrabold text-gray-900 tracking-tight">Words Library</h1>
        <p class="text-sm text-gray-500 mt-1">Manage and track your vocabulary</p>
      </div>
      <div class="text-sm font-medium bg-white px-3 py-1.5 rounded-lg shadow-sm border border-gray-100 text-gray-600">
        Filtered words: <span class="font-bold text-gray-900 ml-1">{{ filteredWords.length }}</span>
      </div>
    </div>

    <!-- Group Filter Chips -->
    <div class="flex flex-wrap gap-2 mb-2">
      <button 
        @click="filterGroupId = 'all'"
        class="px-4 py-1.5 rounded-full text-sm font-medium transition-all duration-200"
        :class="filterGroupId === 'all' ? 'bg-indigo-600 text-white shadow-md' : 'bg-white text-gray-600 border border-gray-200 hover:border-indigo-300'"
      >
        All
      </button>
      <button 
        v-for="group in groupsStore.groups" 
        :key="group.id"
        @click="filterGroupId = group.id.toString()"
        class="px-4 py-1.5 rounded-full text-sm font-medium transition-all duration-200"
        :class="filterGroupId === group.id.toString() ? 'bg-indigo-600 text-white shadow-md' : 'bg-white text-gray-600 border border-gray-200 hover:border-indigo-300'"
      >
        {{ group.name }}
      </button>
    </div>
    
    <div class="flex flex-col lg:flex-row gap-6 h-full min-h-0 flex-1">
      <!-- 40% Left: Form -->
      <div class="lg:w-2/5 xl:w-1/3 shrink-0 flex flex-col gap-6">
        <WordForm />
        
        <!-- Quick Stats Widget -->
        <div class="bg-gradient-to-br from-indigo-500 to-purple-600 rounded-xl p-6 text-white shadow-sm" v-if="filteredWords.length > 0">
          <h3 class="font-medium text-indigo-100 mb-4">Quick Stats (Filtered)</h3>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <p class="text-3xl font-bold">{{ masterCount }}</p>
              <p class="text-xs text-indigo-200 mt-1">Mastered (≥80%)</p>
            </div>
            <div>
              <p class="text-3xl font-bold">{{ weakCount }}</p>
              <p class="text-xs text-indigo-200 mt-1">Needs review</p>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 60% Right: Table -->
      <div class="lg:flex-1 h-full min-h-[500px]">
        <WordsTable :words="filteredWords" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, watch } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { useWordsStore } from '../stores/words'
import { useGroupsStore } from '../stores/groups'
import { useSettingsStore } from '../stores/settings'
import WordForm from '../components/WordForm.vue'
import WordsTable from '../components/WordsTable.vue'

const wordsStore = useWordsStore()
const groupsStore = useGroupsStore()
const settingsStore = useSettingsStore()

const filterGroupId = ref<string>('all')

const filteredWords = computed(() => {
  if (filterGroupId.value === 'all') return wordsStore.words
  const id = parseInt(filterGroupId.value)
  return wordsStore.words.filter(w => w.group_id === id)
})

const masterCount = computed(() => {
  return filteredWords.value.filter(w => w.total > 0 && w.percentage >= 80).length
})

const weakCount = computed(() => {
  return filteredWords.value.filter(w => w.total === 0 || w.percentage < 50).length
})

let unlisten: (() => void) | null = null

onMounted(async () => {
  await wordsStore.fetchWords()
  await groupsStore.fetchGroups()
  await settingsStore.fetchSettings()
  
  // Set initial filter from settings
  filterGroupId.value = settingsStore.settings.active_group_id
  
  // Listen for answers recorded in the game window and refresh data
  unlisten = await listen('answer-recorded', () => {
    wordsStore.fetchWords()
  })
})

// Sync filter with settings if it changes globally
watch(() => settingsStore.settings.active_group_id, (newVal) => {
  filterGroupId.value = newVal
})

onUnmounted(() => {
  if (unlisten) unlisten()
})
</script>
