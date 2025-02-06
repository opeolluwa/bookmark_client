<template>
  <button @click="router.back" href="/auth/login" class="mb-12 flex justify-between items-center">
    <BackArrowLong />
  </button>

  <div class="mb-6">
    <HeadingText class="font-bold">Confirm OTP</HeadingText>
    <SmallText class="leading-1 text-gray-400">
      Confirm the one time password (OTP) that was sent to your email
    </SmallText>
  </div>
</template>

<script lang="ts" setup>
import router from '@mobile/router'
</script>

<style></style>
