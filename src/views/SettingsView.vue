<template>
  <div class="h-full w-full flex flex-col p-6 max-w-2xl mx-auto">
    <div class="mb-8">
      <h1 class="text-3xl font-extrabold text-gray-900 tracking-tight">Settings</h1>
      <p class="text-sm text-gray-500 mt-1">Configure your learning experience</p>
    </div>
    
    <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden flex-1">
      <div class="p-6 space-y-8">
        <!-- Interval Setting -->
        <div>
          <label class="block text-sm font-semibold text-gray-800 mb-2">
            Reminder Interval (minutes)
          </label>
          <p class="text-xs text-gray-500 mb-3">How often should the app ask you to review a word?</p>
          <div class="flex items-center">
            <input 
              v-model.number="form.interval_minutes" 
              type="number" 
              min="1" 
              max="60"
              class="w-32 px-4 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:bg-white text-gray-900 transition-colors"
              :class="{ 'border-red-400 focus:ring-red-500': !isValidInterval }"
            />
            <span class="ml-3 text-sm text-gray-600 font-medium">min</span>
          </div>
          <p v-if="!isValidInterval" class="mt-2 text-xs text-red-500">Interval must be between 1 and 60 minutes.</p>
        </div>

        <hr class="border-gray-100" />

        <!-- Direction Setting -->
        <div>
          <label class="block text-sm font-semibold text-gray-800 mb-2">
            Translation Direction
          </label>
          <p class="text-xs text-gray-500 mb-3">Which language should be shown in the learning popup?</p>
          
          <div class="space-y-3">
            <label class="flex items-center p-3 border rounded-lg cursor-pointer transition-colors"
                   :class="form.direction === 'native_to_foreign' ? 'border-indigo-500 bg-indigo-50/50' : 'border-gray-200 hover:bg-gray-50'">
              <input type="radio" 
                     v-model="form.direction" 
                     value="native_to_foreign" 
                     class="w-4 h-4 text-indigo-600 border-gray-300 focus:ring-indigo-500" />
              <div class="ml-3">
                <span class="block text-sm font-medium" :class="form.direction === 'native_to_foreign' ? 'text-indigo-900' : 'text-gray-900'">
                  Native → Foreign
                </span>
                <span class="block text-xs text-gray-500">e.g. "Яблоко" → You answer "Apple"</span>
              </div>
            </label>

            <label class="flex items-center p-3 border rounded-lg cursor-pointer transition-colors"
                   :class="form.direction === 'foreign_to_native' ? 'border-indigo-500 bg-indigo-50/50' : 'border-gray-200 hover:bg-gray-50'">
              <input type="radio" 
                     v-model="form.direction" 
                     value="foreign_to_native" 
                     class="w-4 h-4 text-indigo-600 border-gray-300 focus:ring-indigo-500" />
              <div class="ml-3">
                <span class="block text-sm font-medium" :class="form.direction === 'foreign_to_native' ? 'text-indigo-900' : 'text-gray-900'">
                  Foreign → Native
                </span>
                <span class="block text-xs text-gray-500">e.g. "Apple" → You answer "Яблоко"</span>
              </div>
            </label>
          </div>
        </div>
      </div>
      
      <!-- Footer actions -->
      <div class="px-6 py-4 bg-gray-50 border-t border-gray-100 flex items-center justify-between">
        <transition name="fade">
          <span v-if="showSuccess" class="text-sm text-emerald-600 font-medium flex items-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1.5" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
            </svg>
            Settings saved
          </span>
          <span v-else></span>
        </transition>
        
        <button 
          @click="handleSave"
          :disabled="!isValidInterval || isSaving"
          class="bg-indigo-600 text-white font-medium py-2 px-6 rounded-lg hover:bg-indigo-700 transition-colors focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed shadow-sm flex items-center"
        >
          <svg v-if="isSaving" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          {{ isSaving ? 'Saving...' : 'Save Settings' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useSettingsStore } from '../stores/settings'

const settingsStore = useSettingsStore()

const form = reactive({
  interval_minutes: 5,
  direction: 'native_to_foreign' as 'native_to_foreign' | 'foreign_to_native'
})

const isSaving = ref(false)
const showSuccess = ref(false)

const isValidInterval = computed(() => {
  return typeof form.interval_minutes === 'number' && 
         form.interval_minutes >= 1 && 
         form.interval_minutes <= 60
})

onMounted(async () => {
  await settingsStore.fetchSettings()
  form.interval_minutes = settingsStore.settings.interval_minutes
  form.direction = settingsStore.settings.direction
})

const handleSave = async () => {
  if (!isValidInterval.value) return
  
  isSaving.value = true
  
  try {
    await settingsStore.saveSettings({ ...form })
    
    // Show success indicator
    showSuccess.value = true
    setTimeout(() => {
      showSuccess.value = false
    }, 3000)
    
  } catch (error) {
    console.error('Failed to save settings:', error)
  } finally {
    isSaving.value = false
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
