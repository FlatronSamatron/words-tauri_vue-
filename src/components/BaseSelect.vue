<template>
  <div class="relative w-full" ref="selectRef">
    <!-- Toggle Button -->
    <button
      type="button"
      @click="isOpen = !isOpen"
      class="w-full px-4 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:bg-white transition-all duration-200 flex items-center justify-between group"
      :class="{ 'ring-2 ring-indigo-500 bg-white border-transparent': isOpen }"
    >
      <span class="truncate text-sm" :class="selectedOption ? 'text-gray-900' : 'text-gray-400'">
        {{ selectedOption ? selectedOption.name : placeholder }}
      </span>
      <svg 
        class="h-4 w-4 text-gray-400 transition-transform duration-200" 
        :class="{ 'rotate-180': isOpen }"
        fill="none" 
        stroke="currentColor" 
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>

    <!-- Dropdown Menu -->
    <transition name="dropdown">
      <div 
        v-if="isOpen"
        class="absolute z-50 w-full mt-2 bg-white border border-gray-100 rounded-xl shadow-xl py-1.5 overflow-hidden max-h-60 overflow-y-auto custom-scrollbar"
      >
        <div
          v-for="option in options"
          :key="option.id"
          @click="selectOption(option)"
          class="px-4 py-2 text-sm cursor-pointer transition-colors flex items-center justify-between"
          :class="modelValue == option.id ? 'bg-indigo-50 text-indigo-700 font-medium' : 'text-gray-700 hover:bg-gray-50'"
        >
          <span>{{ option.name }}</span>
          <svg v-if="modelValue == option.id" class="h-4 w-4 text-indigo-500" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
          </svg>
        </div>
        <div v-if="options.length === 0" class="px-4 py-3 text-sm text-gray-400 text-center italic">
          No options available
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

interface Option {
  id: string | number
  name: string
}

const props = defineProps<{
  modelValue: string | number
  options: Option[]
  placeholder?: string
}>()

const emit = defineEmits(['update:modelValue'])

const isOpen = ref(false)
const selectRef = ref<HTMLElement | null>(null)

const selectedOption = computed(() => {
  return props.options.find(opt => opt.id == props.modelValue)
})

const selectOption = (option: Option) => {
  emit('update:modelValue', option.id)
  isOpen.value = false
}

// Click outside logic
const handleClickOutside = (event: MouseEvent) => {
  if (selectRef.value && !selectRef.value.contains(event.target as Node)) {
    isOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('mousedown', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('mousedown', handleClickOutside)
})
</script>

<style scoped>
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.98);
}

.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #e2e8f0;
  border-radius: 10px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: #cbd5e1;
}
</style>
