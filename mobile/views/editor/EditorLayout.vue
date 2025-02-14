<template>
  <div class="px-4 pt-10 relative w-full h-[95vh] overflow-y-scroll bg-gray-50/80">
    <header
      class="flex h-[20px] bg-white fixed top-0 left-0 mb-5 right-0 w-full border-b px-4 justify-between items-center text-gray-600 dark:bg-gray-900 dark:border-none z-[1000] border-gray-500/20"
    >
      <button
        @click="router.back"
        href="/auth/login"
        class="mb-12 flex justify-between items-center"
      >
        <BackArrowLong />
      </button>
    </header>

    <RouterView />
  </div>
</template>

<script lang="ts" setup>
import { RouterView } from 'vue-router'
import router from '@mobile/router'
</script>

<style></style>
