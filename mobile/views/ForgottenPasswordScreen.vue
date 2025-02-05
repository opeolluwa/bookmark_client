<template>
  <AuthenticationLayout>
    <ForgottenPasswordScreenHeader />
    <ForgottenPasswordScreenForm />
    <RouterLink :to="LOGIN_ROUTE" href="/auth/login" class="text-app block text-sm font-bold mt-3">
      Remember your password? Login!
    </RouterLink>
  </AuthenticationLayout>
</template>

<script lang="ts" setup>
import { LOGIN_ROUTE } from '@mobile/router/routeNames'

import AuthenticationLayout from '@mobile/components/layouts/AuthenticationLayout.vue'
import ForgottenPasswordScreenHeader from '@mobile/components/forgottenPassword/ForgottenPasswordScreenHeader.vue'
import ForgottenPasswordScreenForm from '@mobile/components/forgottenPassword/ForgottenPasswordScreenForm.vue';
</script>

<style></style>
