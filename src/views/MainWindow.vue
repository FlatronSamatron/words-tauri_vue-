<template>
  <div class="h-screen w-screen bg-gray-50 flex flex-col">
    <!-- Very simple layout until Settings tabs are added in Block 4 -->
    <main class="flex-1 overflow-hidden">
      <WordsView />
    </main>
  </div>
</template>

<script setup lang="ts">
import WordsView from './WordsView.vue'
</script>
