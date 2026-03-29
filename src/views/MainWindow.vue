<template>
  <div class="h-screen w-screen bg-gray-50 flex flex-col overflow-hidden">
    <!-- Header/Navigation Bar -->
    <header class="bg-white border-b border-gray-200 px-6 py-4 flex-shrink-0 z-10 shadow-sm relative">
      <div class="flex items-center justify-between max-w-7xl mx-auto w-full">
        <div class="flex items-center space-x-2">
          <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-white font-bold text-lg shadow-sm">
            L
          </div>
          <h1 class="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-indigo-600 to-purple-600 tracking-tight">Lexio</h1>
        </div>
        
        <nav class="flex space-x-1 bg-gray-100/80 p-1 rounded-lg border border-gray-200/50 backdrop-blur-sm">
          <button 
            @click="activeTab = 'words'"
            class="px-4 py-1.5 rounded-md text-sm font-medium transition-all duration-200 flex items-center gap-2"
            :class="activeTab === 'words' ? 'bg-white text-indigo-700 shadow-sm ring-1 ring-gray-900/5' : 'text-gray-500 hover:text-gray-700 hover:bg-gray-50'"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" :fill="activeTab === 'words' ? 'currentColor' : 'none'" viewBox="0 0 24 24" :stroke="activeTab === 'words' ? 'none' : 'currentColor'">
              <path v-if="activeTab === 'words'" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
              <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
            </svg>
            Words
          </button>
          <button 
            @click="activeTab = 'settings'"
            class="px-4 py-1.5 rounded-md text-sm font-medium transition-all duration-200 flex items-center gap-2"
            :class="activeTab === 'settings' ? 'bg-white text-indigo-700 shadow-sm ring-1 ring-gray-900/5' : 'text-gray-500 hover:text-gray-700 hover:bg-gray-50'"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" :fill="activeTab === 'settings' ? 'currentColor' : 'none'" viewBox="0 0 24 24" :stroke="activeTab === 'settings' ? 'none' : 'currentColor'">
              <path v-if="activeTab === 'settings'" fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
              <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path v-if="activeTab !== 'settings'" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            Settings
          </button>
        </nav>
      </div>
    </header>

    <!-- Main Content Area -->
    <main class="flex-1 overflow-hidden relative flex flex-col" style="height: calc(100vh - 73px);">
      <transition name="fade" mode="out-in">
        <keep-alive>
          <component :is="activeComponent" />
        </keep-alive>
      </transition>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import WordsView from './WordsView.vue'
import SettingsView from './SettingsView.vue'

type Tab = 'words' | 'settings'
const activeTab = ref<Tab>('words')

const activeComponent = computed(() => {
  return activeTab.value === 'words' ? WordsView : SettingsView
})
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
