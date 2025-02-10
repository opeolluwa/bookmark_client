<template>
  <div class="relative">
    <header
      class="flex bg-white fixed top-0 py-4  left-0 mb-5 right-0 w-full border-b px-4 justify-between items-center text-gray-600 dark:bg-gray-900 dark:border-none z-[1000] border-gray-500/20"
    >
      <slot name="header"></slot>
    </header>

    <div>
      <slot name="content"></slot>
    </div>
  </div>
</template>

<script lang="ts" setup></script>

<style></style>
