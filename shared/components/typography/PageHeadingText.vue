<template>
  <div class="font-medium leading-2 text-xl capitalize text-gray-700 dark:text-gray-500/90 block">
    {{ text }}
  </div>
</template>

<script lang="ts" setup>
import { defineProps } from 'vue'
defineProps({
  text: String,
})
</script>

<style></style>
