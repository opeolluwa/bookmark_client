<template>
  <button @click="router.back" href="/auth/login" class="mb-12 flex justify-between items-center">
    <BackArrowLong />
  </button>

  <div class="mb-6">
    <HeadingText class="font-bold">Let&apos;s get you back in</HeadingText>
    <SmallText class="leading-1 text-gray-400">
      Provide the email associated with your account and we'd send more instruction
    </SmallText>
  </div>
</template>

<script lang="ts" setup>
import router from '@mobile/router'
</script>

<style></style>
