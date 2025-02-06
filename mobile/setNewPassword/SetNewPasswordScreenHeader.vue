<template>
  <button @click="router.back" href="/auth/login" class="mb-12 flex justify-between items-center">
    <BackArrowLong />
  </button>

  <div class="mb-6">
    <HeadingText class="font-bold">Set new password</HeadingText>
    <SmallText class="leading-1 text-gray-400"> Set new password </SmallText>
  </div>
</template>

<script lang="ts" setup>
import router from '@mobile/router'
</script>

<style></style>
