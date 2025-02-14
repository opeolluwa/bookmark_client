<template>
  <div class="flex items-center justify-center mb-6">
    <InputOtp v-model="value" filled integerOnly :length="6">
      <template #default="{ attrs, events }">
        <input type="text" v-bind="attrs" v-on="events" class="custom-otp-input" autofocus />
      </template>
    </InputOtp>
  </div>
  <span class="text-app block text-sm font-bold mt-3"> Didn&apos;t get an OTP? </span>
</template>

<script setup lang="ts">
import { ref } from 'vue'

import InputOtp from 'primevue/inputotp'

const value = ref('')
</script>

<style scoped>
@reference "@styles/app.css";

.custom-otp-input {
  width: 40px;
  font-size: 32px;
  border: 0 none;
  appearance: none;
  text-align: center;
  transition: all 0.2s;
  background: transparent;
  @apply border-b-gray-400 border-b-[1.75px];
}

.custom-otp-input:focus {
  outline: 0 none;
  @apply border-b-app;
}
</style>
