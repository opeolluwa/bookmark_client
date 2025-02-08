<template>
  <RouterLink :to="String(path)" href="page_route" class="flex flex-col items-center p-0 m-0  rounded-lg hover:text-app btn-animated">
    <HomeIcon  class="size-5"/>
    <span class="text-[12px] font-medium capitalize">{{ label }}</span>
  </RouterLink>
</template>

<script lang="ts" setup>
import { HomeIcon } from '@heroicons/vue/24/outline';
import { defineProps } from 'vue'
defineProps({
  label: String,
  path: String,
})
</script>

<style></style>
