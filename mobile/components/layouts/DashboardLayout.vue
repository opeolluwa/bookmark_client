<template>
  <div class="relative h-[95vh] overflow-y-hidden w-full bg-white dark:bg-base-100">
    <main
      class="px-4 my-6 pt-16 pb-24 overflow-y-scroll dark:text-gray-400/90 bg-gray-100/80"
      style="overflow-y: scroll; height: 100%"
    >
      <RouterView />
    </main>
    <nav
      class="fixed pt-4 mb-0 bottom-0 z-50 bg-white text-gray-500/70 shadow-gray-400 w-full left-0 right-0 py-3 dark:bg-gray-900/50 border-t border-gray-500/50"
    >
      <BottomNavigation />
    </nav>
  </div>
</template>

<script lang="ts" setup>
import BottomNavigation from '../partials/BottomNavigation.vue'
</script>

<style></style>
