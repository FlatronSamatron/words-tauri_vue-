<template>
  <div class="h-full w-full flex flex-col p-6 max-w-2xl mx-auto overflow-y-auto">
    <div class="mb-8">
      <h1 class="text-3xl font-extrabold text-gray-900 tracking-tight">Settings</h1>
      <p class="text-sm text-gray-500 mt-1">Configure your learning experience</p>
    </div>
    
    <div class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden overflow-y-auto">
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

        <hr class="border-gray-100" />

        <!-- Active Group Setting -->
        <div>
          <label class="block text-sm font-semibold text-gray-800 mb-2">
            Active Study Group
          </label>
          <p class="text-xs text-gray-500 mb-3">Only words from this group will be shown in the learning popup.</p>
          <select 
            v-model="form.active_group_id"
            class="w-full px-4 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:bg-white text-gray-900 transition-colors"
          >
            <option value="all">All Groups</option>
            <option v-for="group in groupsStore.groups" :key="group.id" :value="group.id.toString()">
              {{ group.name }} ({{ group.word_count }} words)
            </option>
          </select>
        </div>

        <hr class="border-gray-100" />

        <!-- Manage Groups Section -->
        <div>
          <div class="flex items-center justify-between mb-4">
            <label class="block text-sm font-semibold text-gray-800">
              Manage Groups
            </label>
            <button 
              @click="showAddGroup = !showAddGroup"
              class="text-xs font-medium text-indigo-600 hover:text-indigo-700 flex items-center gap-1"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
              </svg>
              Create New Group
            </button>
          </div>

          <!-- Add Group Form -->
          <transition name="fade">
            <div v-if="showAddGroup" class="mb-4 p-3 bg-indigo-50 rounded-lg border border-indigo-100 flex gap-2">
              <input 
                v-model="newGroupName" 
                type="text" 
                placeholder="Group name..." 
                class="flex-1 px-3 py-1.5 text-sm rounded-md border border-indigo-200 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                @keyup.enter="handleAddGroup"
              />
              <button 
                @click="handleAddGroup"
                :disabled="!newGroupName.trim()"
                class="bg-indigo-600 text-white text-xs font-bold px-3 py-1.5 rounded-md hover:bg-indigo-700 disabled:opacity-50"
              >
                Add
              </button>
            </div>
          </transition>

          <!-- Groups List -->
          <div class="space-y-2 max-h-60 overflow-y-auto pr-1 custom-scrollbar">
            <div 
              v-for="group in groupsStore.groups" 
              :key="group.id"
              class="flex items-center justify-between p-3 bg-gray-50 rounded-lg border border-gray-100 group"
            >
              <div v-if="editingGroupId === group.id" class="flex-1 flex gap-2">
                <input 
                  v-model="editGroupName" 
                  type="text" 
                  class="flex-1 px-2 py-1 text-sm rounded border border-indigo-300 focus:outline-none focus:ring-1 focus:ring-indigo-500"
                  @keyup.enter="handleRenameGroup(group.id)"
                  @keyup.esc="editingGroupId = null"
                  ref="editGroupInput"
                />
                <button @click="handleRenameGroup(group.id)" class="text-indigo-600">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                  </svg>
                </button>
              </div>
              <div v-else class="flex-1 flex items-center justify-between">
                <div>
                  <span class="text-sm font-medium text-gray-800">{{ group.name }}</span>
                  <span class="ml-2 text-xs text-gray-400">{{ group.word_count }} words</span>
                </div>
                <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                  <button @click="startRename(group)" class="p-1 text-gray-400 hover:text-indigo-600">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
                    </svg>
                  </button>
                  <button 
                    v-if="group.id !== 1" 
                    @click="handleDeleteGroup(group.id)" 
                    class="p-1 text-gray-400 hover:text-red-600"
                    title="Delete group and its words"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                </div>
              </div>
            </div>
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
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import { useSettingsStore } from '../stores/settings'
import { useGroupsStore } from '../stores/groups'

const settingsStore = useSettingsStore()
const groupsStore = useGroupsStore()

const form = reactive({
  interval_minutes: 5,
  direction: 'native_to_foreign' as 'native_to_foreign' | 'foreign_to_native',
  active_group_id: 'all'
})

const isSaving = ref(false)
const showSuccess = ref(false)

// Group management state
const showAddGroup = ref(false)
const newGroupName = ref('')
const editingGroupId = ref<number | null>(null)
const editGroupName = ref('')
const editGroupInput = ref<HTMLInputElement | null>(null)

const isValidInterval = computed(() => {
  return typeof form.interval_minutes === 'number' && 
         form.interval_minutes >= 1 && 
         form.interval_minutes <= 60
})

onMounted(async () => {
  await settingsStore.fetchSettings()
  await groupsStore.fetchGroups()
  form.interval_minutes = settingsStore.settings.interval_minutes
  form.direction = settingsStore.settings.direction
  form.active_group_id = settingsStore.settings.active_group_id
})

const handleAddGroup = async () => {
  if (!newGroupName.value.trim()) return
  try {
    await groupsStore.addGroup(newGroupName.value.trim())
    newGroupName.value = ''
    showAddGroup.value = false
  } catch (error) {
    console.error('Failed to add group:', error)
  }
}

const startRename = async (group: any) => {
  editingGroupId.value = group.id
  editGroupName.value = group.name
  await nextTick()
  editGroupInput.value?.focus()
}

const handleRenameGroup = async (id: number) => {
  if (!editGroupName.value.trim()) return
  try {
    await groupsStore.renameGroup(id, editGroupName.value.trim())
    editingGroupId.value = null
  } catch (error) {
    console.error('Failed to rename group:', error)
  }
}

const handleDeleteGroup = async (id: number) => {
  if (id === 1) return
  if (!confirm('Are you sure? This will delete all words in this group.')) return
  try {
    const success = await groupsStore.deleteGroup(id)
    if (success && form.active_group_id === id.toString()) {
      form.active_group_id = 'all'
    }
  } catch (error) {
    console.error('Failed to delete group:', error)
  }
}

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
