<template>
  <div class="card">
    <input
      type="text"
      class="text-2xl placeholder:text-2xl font-bold placeholder:text-gray-400 border-none outline-none"
      placeholder="Bookmark title"
      autofocus
    />
    <textarea
      name=""
      placeholder="description"
      class="mt-4  font-bold placeholder:text-gray-400 border-none outline-none"
      id=""
    ></textarea>
  </div>
</template>

<script setup lang="ts">
import Editor from 'primevue/editor'
import { ref } from 'vue'

const value = ref('')
</script>
