<template>
  <AppView class="flex flex-col relative h-[90vh] overflow-hidden">
    <div>
      <img width="100%" height="auto" src="@assets/illustrations/get-started.png" alt="welcome" />
    </div>
    <div class="text-center">
      <h1 class="text-3xl font-black">Get started!</h1>
      <p class="text-[18px] text-gray-400 mt-2">Create a free account or proceed without sign up</p>
    </div>

    <div class="flex flex-col mt-6">
      <RouterLink
        :to="mergeRoute(DASHBOARD_BASE_ROUTE, DASHBOARD_HOME)"
        class="btn text-white bg-app font-medium border-none"
      >
        Get Started
      </RouterLink>

      <RouterLink :to="SIGN_UP_ROUTE" class="muted_btn"> Create Account </RouterLink>
    </div>

    <div class="flex justify-between absolute w-full bottom-2 left-0 right-0">
      <PreviousScreenButton />
    </div>
  </AppView>
</template>

<script lang="ts" setup>
import { mergeRoute } from '@mobile/components/bottomNavigation/bottomNavigation'
import PreviousScreenButton from '@mobile/components/walkThrough/PreviousScreenButton.vue'
import { DASHBOARD_BASE_ROUTE, DASHBOARD_HOME, SIGN_UP_ROUTE } from '@mobile/router/routeNames'
import { RouterLink } from 'vue-router'
</script>

<style></style>
