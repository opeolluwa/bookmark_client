<template>
  <RouterLink :to="String(path)" class="btn rounded-md px-4 btn-sm bg-app text-white"> 
    Next </RouterLink>
</template>

<script lang="ts" setup>
import { defineProps } from 'vue'
defineProps({
  path: String,
})
</script>

<style></style>
