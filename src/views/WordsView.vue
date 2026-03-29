<template>
  <div class="h-full w-full flex flex-col p-6 space-y-6 overflow-y-auto">
    <div class="flex justify-between items-end mb-2">
      <div>
        <h1 class="text-3xl font-extrabold text-gray-900 tracking-tight">Words Library</h1>
        <p class="text-sm text-gray-500 mt-1">Manage and track your vocabulary</p>
      </div>
      <div class="text-sm font-medium bg-white px-3 py-1.5 rounded-lg shadow-sm border border-gray-100 text-gray-600">
        Total words: <span class="font-bold text-gray-900 ml-1">{{ wordsStore.words.length }}</span>
      </div>
    </div>
    
    <div class="flex flex-col lg:flex-row gap-6 h-full min-h-0 flex-1">
      <!-- 40% Left: Form -->
      <div class="lg:w-2/5 xl:w-1/3 shrink-0 flex flex-col gap-6">
        <WordForm />
        
        <!-- Quick Stats Widget (optional nice-to-have) -->
        <div class="bg-gradient-to-br from-indigo-500 to-purple-600 rounded-xl p-6 text-white shadow-sm" v-if="wordsStore.words.length > 0">
          <h3 class="font-medium text-indigo-100 mb-4">Quick Stats</h3>
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
        <WordsTable />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { useWordsStore } from '../stores/words'
import WordForm from '../components/WordForm.vue'
import WordsTable from '../components/WordsTable.vue'

const wordsStore = useWordsStore()

const masterCount = computed(() => {
  return wordsStore.words.filter(w => w.total > 0 && w.percentage >= 80).length
})

const weakCount = computed(() => {
  return wordsStore.words.filter(w => w.total === 0 || w.percentage < 50).length
})

onMounted(() => {
  wordsStore.fetchWords()
})
</script>
