<template>
  <div v-if="cachedUserDataExist" class="mb-12 flex justify-between items-center">
    <RouterLink :to="SIGN_UP_ROUTE">
      <BackArrowLong />
    </RouterLink>
    <button class="font-medium text-sm text-gray-600">Not {{ userInformation.firstName }}</button>
  </div>

  <div class="mb-12 flex justify-between items-center">
    <RouterLink :to="SIGN_UP_ROUTE" class="block size-6">
      <ArrowLongLeftIcon />
    </RouterLink>
  </div>

  <div class="mb-6 flex flex-col" v-if="cachedUserDataExist">
    <HeadingText class="font-bold">Welcome, Adeoye</HeadingText>
    <SmallText class="leading-1 text-gray-400"> Enter your password to log in </SmallText>
  </div>

  <div v-else>
    <HeadingText class="font-bold">Log in</HeadingText>
    <SmallText class="leading-1 mt-2 text-sm small-text">
      Enter your email and password to log in
    </SmallText>
  </div>
</template>

<script lang="ts" setup>
import { useCachedUserStore } from '@shared/stores/cachedUser'
import { storeToRefs } from 'pinia'
import { SIGN_UP_ROUTE } from '@mobile/router/routeNames'
import { RouterLink } from 'vue-router'
import BackArrowLong from '@shared/components/icons/BackArrowLong.vue'

const cachedUserData = useCachedUserStore()
const { cachedUserDataExist, userInformation } = storeToRefs(cachedUserData)
</script>

<style></style>
