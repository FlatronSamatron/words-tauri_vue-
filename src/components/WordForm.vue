<template>
  <div class="bg-white p-6 rounded-xl shadow-sm border border-gray-100 relative">
    <transition name="fade">
      <div v-if="showSuccess" class="absolute top-4 right-4 bg-emerald-50 text-emerald-600 px-3 py-1.5 rounded-lg text-sm font-medium border border-emerald-100/50 shadow-sm flex items-center gap-1.5">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
        </svg>
        Word added
      </div>
    </transition>
    <h2 class="text-xl font-bold mb-5 text-gray-800 tracking-tight">Add New Word</h2>
    <form @submit.prevent="handleSubmit" class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1.5">Word</label>
        <input 
          ref="wordInput"
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
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1.5">Group</label>
        <BaseSelect 
          v-model="selectedGroupId"
          :options="groupsStore.groups"
          placeholder="Select Group"
        />
      </div>

      <button 
        type="submit" 
        class="w-full bg-indigo-600 text-white font-medium py-2.5 px-4 rounded-lg hover:bg-indigo-700 transition-colors focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed shadow-sm"
        :disabled="isSubmitting || !word.trim() || !translate.trim()"
      >
        <span v-if="isSubmitting">Saving...</span>
        <span v-else>Save Word</span>
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useWordsStore } from '../stores/words'
import { useGroupsStore } from '../stores/groups'
import { useSettingsStore } from '../stores/settings'
import BaseSelect from './BaseSelect.vue'

const wordsStore = useWordsStore()
const groupsStore = useGroupsStore()
const settingsStore = useSettingsStore()

const wordInput = ref<HTMLInputElement | null>(null)
const word = ref('')
const translate = ref('')
const selectedGroupId = ref<number>(1)
const showError = ref(false)
const isSubmitting = ref(false)
const showSuccess = ref(false)

onMounted(async () => {
  wordInput.value?.focus()
  await groupsStore.fetchGroups()
  
  // Set default group from settings
  const active = settingsStore.settings.active_group_id
  if (active !== 'all') {
    selectedGroupId.value = parseInt(active)
  } else {
    selectedGroupId.value = 1 // Default
  }
})

// Update selected group if active group changes in settings
watch(() => settingsStore.settings.active_group_id, (newVal) => {
  if (newVal !== 'all') {
    selectedGroupId.value = parseInt(newVal)
  }
})

const handleSubmit = async () => {
  if (!word.value.trim() || !translate.value.trim()) {
    showError.value = true
    return
  }

  showError.value = false
  isSubmitting.value = true

  try {
    await wordsStore.addWord(word.value.trim(), translate.value.trim(), selectedGroupId.value)
    word.value = ''
    translate.value = ''
    
    // Show toast for 2 seconds
    showSuccess.value = true
    setTimeout(() => {
      showSuccess.value = false
    }, 2000)
    
    // Set focus back to first input
    wordInput.value?.focus()
  } catch (error) {
    console.error('Failed to add word:', error)
  } finally {
    isSubmitting.value = false
  }
}
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
