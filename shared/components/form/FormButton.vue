<template>
  <button
    type="submit"
    class="btn btn-lg w-full disabled:bg-100 rounded-lg py-4 bg-app-600 text-white font-medium form-btn"
  >
    {{ text }}
  </button>
</template>

<script lang="ts" setup>
import { defineProps } from 'vue'

defineProps({
  text: String,
})
</script>

<style></style>
