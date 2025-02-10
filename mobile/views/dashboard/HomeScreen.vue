<template>
  <DashboardLayout>
    <template #header>
      <div class="flex justify-between items-center w-full h-max-[2rem] my-0 py-0">
        <div class="flex items-center gap-x-3">
          <Bars3Icon class="size-6" />
          <PageHeadingText text="Bookmarks" />
        </div>
        <a href="/dashboard/search">
          <MagnifyingGlassIcon class="size-6" />
        </a>
      </div>
    </template>

    <template #content> hey content </template>
  </DashboardLayout>
</template>

<script lang="ts" setup>
import { MagnifyingGlassIcon, Bars3Icon } from '@heroicons/vue/24/solid'
import DashboardLayout from '@mobile/components/layouts/DashboardLayout.vue'
</script>

<style></style>
