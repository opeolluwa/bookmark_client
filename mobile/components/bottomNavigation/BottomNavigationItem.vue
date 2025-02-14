<template>
  <RouterLink
    :to="mergeRoute(DASHBOARD_BASE_ROUTE, String(path))"
    class="flex flex-col items-center p-0 m-0 rounded-lg hover:text-app"
    v-ripple style=" --p-ripple-background: rgba(19, 104, 240, .15)"
  >
    <slot></slot>
    <span class="text-[12px] font-medium capitalize dock-label hover:text-app">{{ label }} </span>
  </RouterLink>
</template>

<script lang="ts" setup>
import { defineProps, ref } from 'vue'
import { mergeRoute } from './bottomNavigation'
import { DASHBOARD_BASE_ROUTE } from '@mobile/router/routeNames'
import { usePrimeVue } from 'primevue/config'

const PrimeVue = usePrimeVue()
PrimeVue.config.ripple = true

const props = defineProps({
  label: String,
  path: String,
  //   isActiveRoute: Boolean,
})

import { useRoute } from 'vue-router'
const router = useRoute()
const currentRoute = router.path
const isActiveRoute = ref(currentRoute == mergeRoute(DASHBOARD_BASE_ROUTE, String(props.path)))
</script>

<style></style>
