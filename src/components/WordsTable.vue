<template>
  <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden flex flex-col h-full">
    <div class="overflow-x-auto overflow-y-auto flex-1 h-full min-h-0">
      <table class="w-full whitespace-nowrap text-left" v-if="wordsStore.words.length > 0">
        <thead class="bg-gray-50/80 backdrop-blur-sm sticky top-0 z-10 border-b border-gray-200">
          <tr>
            <th class="px-6 py-4 text-xs font-semibold text-gray-500 uppercase tracking-wider">Word</th>
            <th class="px-6 py-4 text-xs font-semibold text-gray-500 uppercase tracking-wider">Translation</th>
            <th class="px-6 py-4 text-xs font-semibold text-gray-500 uppercase tracking-wider">Correct / Total</th>
            <th class="px-6 py-4 text-xs font-semibold text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100 transition-colors" @click="toggleSort">
              <div class="flex items-center gap-1">
                %
                <span v-if="sortAsc !== null" class="text-gray-400">
                  <svg v-if="sortAsc" xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z" clip-rule="evenodd" />
                  </svg>
                  <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                  </svg>
                </span>
              </div>
            </th>
            <th class="px-6 py-4 text-right text-xs font-semibold text-gray-500 uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-100">
          <tr v-for="item in sortedWords" :key="item.id" class="hover:bg-gray-50/50 transition-colors group">
            <!-- Normal View -->
            <template v-if="editingId !== item.id">
              <td class="px-6 py-4 cursor-pointer" @click="startEdit(item)">
                <div class="text-sm font-medium text-gray-900">{{ item.word }}</div>
              </td>
              <td class="px-6 py-4 cursor-pointer" @click="startEdit(item)">
                <div class="text-sm text-gray-600">{{ item.translate }}</div>
              </td>
              <td class="px-6 py-4">
                <div class="text-sm text-gray-500 font-mono">{{ item.correct }} / {{ item.total }}</div>
              </td>
              <td class="px-6 py-4">
                <div class="inline-flex px-2 py-1 rounded-full text-xs font-semibold tracking-wide" :class="getPercentageBadgeClass(item.percentage, item.total)">
                  {{ item.percentage.toFixed(0) }}%
                </div>
              </td>
              <td class="px-6 py-4 text-right text-sm">
                <div v-if="deletingId === item.id" class="flex items-center justify-end gap-1.5">
                  <span class="text-xs text-red-500 font-medium mr-1">Delete?</span>
                  <button @click="executeDelete" class="text-white bg-red-500 hover:bg-red-600 rounded p-1 transition-colors focus:outline-none">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                  </button>
                  <button @click="cancelDelete" class="text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded p-1 transition-colors focus:outline-none">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                    </svg>
                  </button>
                </div>
                <button v-else @click="confirmDelete(item.id)" class="text-gray-400 hover:text-red-600 transition-colors opacity-0 group-hover:opacity-100 p-1.5 focus:opacity-100 focus:outline-none">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </button>
              </td>
            </template>
            
            <!-- Editing View -->
            <template v-else>
              <td class="px-4 py-2 w-[25%]">
                <input 
                  v-model="editForm.word" 
                  type="text" 
                  class="w-full px-3 py-1.5 text-sm border-2 border-indigo-200 bg-indigo-50/30 rounded-md focus:outline-none focus:border-indigo-500 focus:ring-0"
                  @keyup.enter="saveEdit"
                  @keyup.esc="cancelEdit"
                  ref="editWordInput"
                />
              </td>
              <td class="px-4 py-2 w-[35%]">
                <input 
                  v-model="editForm.translate" 
                  type="text" 
                  class="w-full px-3 py-1.5 text-sm border-2 border-indigo-200 bg-indigo-50/30 rounded-md focus:outline-none focus:border-indigo-500 focus:ring-0"
                  @keyup.enter="saveEdit"
                  @keyup.esc="cancelEdit"
                />
              </td>
              <td class="px-6 py-4">
                <div class="text-sm text-gray-500 font-mono">{{ item.correct }} / {{ item.total }}</div>
              </td>
              <td class="px-6 py-4">
                <div class="inline-flex px-2 py-1 rounded-full text-xs font-semibold tracking-wide" :class="getPercentageBadgeClass(item.percentage, item.total)">
                  {{ item.percentage.toFixed(0) }}%
                </div>
              </td>
              <td class="px-4 py-3 text-right text-sm space-x-2 whitespace-nowrap">
                <button @click="saveEdit" class="inline-flex items-center justify-center text-white bg-indigo-500 hover:bg-indigo-600 rounded p-1.5 transition-colors focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-1">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                  </svg>
                </button>
                <button @click="cancelEdit" class="inline-flex items-center justify-center text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded p-1.5 transition-colors focus:outline-none focus:ring-2 focus:ring-gray-300 focus:ring-offset-1">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                  </svg>
                </button>
              </td>
            </template>
          </tr>
        </tbody>
      </table>
      
      <!-- Empty State -->
      <div v-else class="flex flex-col items-center justify-center h-full min-h-[400px] py-16 px-4 text-center">
        <div class="mb-6 bg-indigo-50 p-6 rounded-full">
          <svg class="h-16 w-16 text-indigo-300" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
        </div>
        <h3 class="text-xl font-bold text-gray-800 mb-2 tracking-tight">Your vocabulary is empty</h3>
        <p class="text-md text-gray-500 max-w-sm mb-6">
          Add your first word using the form on the left to start learning!
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
import { useWordsStore } from '../stores/words'
import type { Word } from '../types'

const wordsStore = useWordsStore()

// State
const editingId = ref<number | null>(null)
const editForm = ref({ word: '', translate: '' })
const sortAsc = ref<boolean | null>(true)
const editWordInput = ref<HTMLInputElement[]>([])
const deletingId = ref<number | null>(null)

// Sorting logic
const sortedWords = computed(() => {
  const words = [...wordsStore.words]
  if (sortAsc.value === true) {
    return words.sort((a, b) => {
      if (a.total === 0 && b.total !== 0) return -1
      if (b.total === 0 && a.total !== 0) return 1
      return a.percentage - b.percentage
    })
  } else if (sortAsc.value === false) {
    return words.sort((a, b) => b.percentage - a.percentage)
  }
  return words
})

const toggleSort = () => {
  if (sortAsc.value === true) sortAsc.value = false
  else if (sortAsc.value === false) sortAsc.value = null
  else sortAsc.value = true
}

// Visual helpers
const getPercentageBadgeClass = (percentage: number, total: number) => {
  if (total === 0) return 'bg-gray-100 text-gray-600'
  if (percentage >= 80) return 'bg-emerald-100 text-emerald-700'
  if (percentage >= 50) return 'bg-amber-100 text-amber-700'
  return 'bg-rose-100 text-rose-700'
}

// Editing logic
const startEdit = async (item: Word) => {
  editingId.value = item.id
  editForm.value = { word: item.word, translate: item.translate }
  
  await nextTick()
  if (editWordInput.value && editWordInput.value.length > 0) {
    editWordInput.value[0].focus()
  }
}

const cancelEdit = () => {
  editingId.value = null
  editForm.value = { word: '', translate: '' }
}

const saveEdit = async () => {
  if (!editingId.value) return
  
  const word = editForm.value.word.trim()
  const translate = editForm.value.translate.trim()
  
  if (!word || !translate) return
  
  try {
    await wordsStore.updateWord(editingId.value, word, translate)
    cancelEdit()
  } catch (error) {
    console.error('Failed to update word:', error)
  }
}

// Deleting logic
const confirmDelete = (id: number) => {
  deletingId.value = id
}

const cancelDelete = () => {
  deletingId.value = null
}

const executeDelete = async () => {
  if (!deletingId.value) return
  try {
    await wordsStore.deleteWord(deletingId.value)
  } catch (error) {
    console.error('Failed to delete word:', error)
  } finally {
    deletingId.value = null
  }
}
</script>
