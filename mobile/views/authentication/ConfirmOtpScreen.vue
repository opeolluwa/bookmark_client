<template>
  <AuthenticationLayout>
    <ConfirmOtpScreenHeader />
    <ConfirmOtpScreenForm />
    <RouterLink :to="LOGIN_ROUTE" class="text-app block text-sm font-bold mt-3">
      Didn&apos;t get an OPT? 
    </RouterLink>
  </AuthenticationLayout>
</template>

<script lang="ts" setup>
import ConfirmOtpScreenHeader from '@mobile/components/confirmOtp/ConfirmOtpScreenHeader.vue'
import ConfirmOtpScreenForm from '@mobile/components/confirmOtp/ConfirmOtpScreenForm.vue'
import { LOGIN_ROUTE } from '@mobile/router/routeNames'
import AuthenticationLayout from '@mobile/components/layouts/AuthenticationLayout.vue'
</script>

<style></style>
