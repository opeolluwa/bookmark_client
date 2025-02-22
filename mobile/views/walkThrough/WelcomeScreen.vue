<template>
  <AppView class="flex flex-col relative h-[90vh] overflow-hidden">
    <div style="height: 40vh">
      <img
        width="100%"
        height="auto"
        src="@assets/illustrations/welcome.png"
        alt="welcome"
        block="true"
      />
    </div>
    <div class="text-center">
      <h1 class="text-3xl font-black">Awesome bookmarks</h1>
      <p class="text-[18px] text-gray-400 mt-2">
        Craft, curate and share all important bookmarks on a single platform
      </p>
    </div>

    <div class="flex justify-end absolute w-full bottom-2 left-0 right-0">
      <RouterLink
        :to="mergeRoute(WALK_THROUGH_BASE_ROUTE, WALK_THROUGH_FEATURES_ROUTE)"
        class="btn px-4 btn-sm bg-app text-white"
      >
        Next
      </RouterLink>
    </div>
  </AppView>
</template>

<script lang="ts" setup>
import { mergeRoute } from '@mobile/components/bottomNavigation/bottomNavigation'
import { WALK_THROUGH_BASE_ROUTE, WALK_THROUGH_FEATURES_ROUTE } from '@mobile/router/routeNames'
import { RouterLink } from 'vue-router'
</script>

<style></style>
