<template>
  <div class="mb-12 flex justify-between items-center">
    <RouterLink :to="LOGIN_ROUTE">
      <BackArrowLong />
    </RouterLink>
  </div>
  <div class="mb-6">
    <HeadingText class="font-bold">Create account</HeadingText>
    <SmallText class="leading-1 mt-2 text-sm small-text"> Create and account to begin </SmallText>
  </div>
</template>

<script lang="ts" setup>
import { LOGIN_ROUTE } from '@mobile/router/routeNames'
</script>

<style></style>
