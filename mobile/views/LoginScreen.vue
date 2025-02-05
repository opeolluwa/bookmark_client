<template>
  <AuthenticationLayout class="relative" style="height: calc(100vh - 100px)" id="editor">
    <LoginScreenHeader />
    <LoginScreenForm />
    <RouterLink :to="FORGOTTEN_PASSWORD_ROUTE" class="text-app block text-sm font-bold mt-3">
      Forgotten password?
    </RouterLink>
  </AuthenticationLayout>
</template>

<script lang="ts" setup>
import AuthenticationLayout from '@mobile/components/layouts/AuthenticationLayout.vue'

import LoginScreenHeader from '@mobile/components/login/LoginScreenHeader.vue'
import { FORGOTTEN_PASSWORD_ROUTE } from '@mobile/router/routeNames'
import LoginScreenForm from '@mobile/components/login/LoginScreenForm.vue'
</script>

<style></style>
