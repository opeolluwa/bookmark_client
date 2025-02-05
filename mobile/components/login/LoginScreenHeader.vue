<template>
  <div v-if="cachedUserDataExist" class="mb-12 flex justify-between items-center">
    <a href="/auth/sign-up" class="block size-6">
      <ArrowLongLeftIcon />
    </a>
    <button class="font-medium text-sm text-gray-600">Not {{ userInformation.firstName }}</button>
  </div>

  <div class="mb-12 flex justify-between items-center">
    <a href="/auth/sign-up" class="block size-6">
      <ArrowLongLeftIcon />
    </a>
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
import HeadingText from '@shared/components/typography/HeadingText.vue'
import SmallText from '@shared/components/typography/SmallText.vue'

import { ArrowLongLeftIcon } from '@heroicons/vue/24/solid'

import { useCachedUserStore } from '@shared/stores/cachedUser'
import { storeToRefs } from 'pinia'

const cachedUserData = useCachedUserStore()
const { cachedUserDataExist, userInformation } = storeToRefs(cachedUserData)
</script>

<style></style>
