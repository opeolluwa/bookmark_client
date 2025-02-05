<template>
  <div>
    <template v-if="href == ''">
      <p class="leading-5  text-inherit">
        <slot />
      </p>
    </template>

    <template v-else>
      <a :href="href" class="leading-5 text-sm text-inherit"> <slot /> </a>
    </template>
  </div>
</template>

<script lang="ts" setup>
import { defineProps } from 'vue'
defineProps({
  href: String,
})
</script>

<style></style>
