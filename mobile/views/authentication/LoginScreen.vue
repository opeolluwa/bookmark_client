<template>
  <AuthenticationLayout class="relative" style="height: calc(100vh - 100px)" id="editor">
    <LoginScreenHeader />
    <LoginScreenForm />
    <RouterLink :to="DASHBOARD_BASE_ROUTE" class="text-app block text-sm font-bold mt-3">
      Forgotten password?
    </RouterLink>
  </AuthenticationLayout>
</template>

<script lang="ts" setup>
import AuthenticationLayout from '@mobile/components/layouts/AuthenticationLayout.vue'

import LoginScreenHeader from '@mobile/components/login/LoginScreenHeader.vue'
import {
  CONFIRM_OTP_ROUTE,
  DASHBOARD_BASE_ROUTE,
  FORGOTTEN_PASSWORD_ROUTE,
} from '@mobile/router/routeNames'
import LoginScreenForm from '@mobile/components/login/LoginScreenForm.vue'

import router from '@mobile/router'
import { useInstallationStatus } from '@shared/stores/installationStatus'
import { onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import Button from 'primevue/button'

onMounted(() => {
  const installationStatusStore = useInstallationStatus()
  const { appIsInstalled } = storeToRefs(installationStatusStore)
  if (!appIsInstalled) {
    router.push('/walkthrough')
  }
})
</script>

<style></style>
