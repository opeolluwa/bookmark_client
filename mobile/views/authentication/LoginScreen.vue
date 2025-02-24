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

import LoginScreenForm from '@mobile/components/login/LoginScreenForm.vue'
import LoginScreenHeader from '@mobile/components/login/LoginScreenHeader.vue'
import {
  DASHBOARD_BASE_ROUTE
} from '@mobile/router/routeNames'

import router from '@mobile/router'
import { useInstallationStatus } from '@shared/stores/installationStatus'
import { storeToRefs } from 'pinia'
import { onMounted } from 'vue'

onMounted(() => {
  const installationStatusStore = useInstallationStatus()
  const { appIsInstalled } = storeToRefs(installationStatusStore)
  if (appIsInstalled.value == false) {
    router.push('/walkthrough')
  }
})
</script>

<style></style>
