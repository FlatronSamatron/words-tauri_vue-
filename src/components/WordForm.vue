<template>
  <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
    <h2 class="text-xl font-bold mb-5 text-gray-800 tracking-tight">Add New Word</h2>
    <form @submit.prevent="handleSubmit" class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1.5">Word</label>
        <input 
          v-model="word" 
          type="text" 
          class="w-full px-4 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:bg-white transition-all duration-200"
          :class="{ 'border-red-400 focus:ring-red-500': showError && !word }"
          placeholder="e.g. Apple"
        />
        <p v-if="showError && !word" class="mt-1 text-sm text-red-500">Word is required</p>
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1.5">Translation</label>
        <input 
          v-model="translate" 
          type="text" 
          class="w-full px-4 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:bg-white transition-all duration-200"
          :class="{ 'border-red-400 focus:ring-red-500': showError && !translate }"
          placeholder="e.g. Яблоко"
        />
        <p v-if="showError && !translate" class="mt-1 text-sm text-red-500">Translation is required</p>
      </div>
      <button 
        type="submit" 
        class="w-full bg-indigo-600 text-white font-medium py-2.5 px-4 rounded-lg hover:bg-indigo-700 transition-colors focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed shadow-sm"
        :disabled="isSubmitting"
      >
        <span v-if="isSubmitting">Saving...</span>
        <span v-else>Save Word</span>
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useWordsStore } from '../stores/words'

const wordsStore = useWordsStore()

const word = ref('')
const translate = ref('')
const showError = ref(false)
const isSubmitting = ref(false)

const handleSubmit = async () => {
  if (!word.value.trim() || !translate.value.trim()) {
    showError.value = true
    return
  }

  showError.value = false
  isSubmitting.value = true

  try {
    await wordsStore.addWord(word.value.trim(), translate.value.trim())
    word.value = ''
    translate.value = ''
  } catch (error) {
    console.error('Failed to add word:', error)
  } finally {
    isSubmitting.value = false
  }
}
</script>
