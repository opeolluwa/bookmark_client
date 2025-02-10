<template>
  <AppView class="flex flex-col relative h-[90vh] overflow-hidden">
    <div style="height: 40vh">
      <img src="@assets/illustrations/feature.png" alt="welcome" width="100%" height="auto" />
    </div>
    <div class="text-center">
      <h1 class="text-3xl font-black">Offline first</h1>
      <p class="text-[18px] text-gray-400 mt-2">
        Access all your important bookmarks without internet
      </p>
    </div>

    <div class="flex justify-between absolute w-full bottom-0 mt-6 left-0 right-0">
      <PreviousScreenButton />
      <NextScreenButton :path="nextRoute" class="btn px-4 btn-sm bg-app text-white">
        Next
      </NextScreenButton>
    </div>
  </AppView>
</template>

<script lang="ts" setup>
import { mergeRoute } from '@mobile/components/partials/bottomNavigation'
import { WALK_THROUGH_BASE_ROUTE, WALK_THROUGH_GET_STARTED_ROUTE } from '@mobile/router/routeNames'
import { ref } from 'vue'

import PreviousScreenButton from '@mobile/components/walkThrough/PreviousScreenButton.vue'
import NextScreenButton from '@mobile/components/walkThrough/NextScreenButton.vue'

const nextRoute = ref(mergeRoute(WALK_THROUGH_BASE_ROUTE, WALK_THROUGH_GET_STARTED_ROUTE))
</script>

<style></style>
