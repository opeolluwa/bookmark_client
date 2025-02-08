<template>
  <RouterLink
    :to="mergeRoute(DASHBOARD_BASE_ROUTE, String(path))"
    class="flex flex-col items-center p-0 m-0 rounded-lg hover:text-app btn-animated"
  >
    <slot></slot>
    <span class="text-[12px] font-medium capitalize dock-label hover:text-app">{{ label }}</span>
  </RouterLink>
</template>

<script lang="ts" setup>
import { defineProps } from 'vue'
import { mergeRoute } from './bottomNavigation'
import { DASHBOARD_BASE_ROUTE } from '@mobile/router/routeNames'
defineProps({
  label: String,
  path: String,
  isActiveRoute: Boolean,
})
</script>

<style></style>
