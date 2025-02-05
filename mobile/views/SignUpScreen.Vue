<template>
  <AuthenticationLayout>
    <SignUpScreenHeader />
    <SignUpScreenForm />
    <RouterLink :to="LOGIN_ROUTE" class="text-app block text-sm font-bold mt-3">
      Already have and account? Login
    </RouterLink>
  </AuthenticationLayout>
</template>

<script lang="ts" setup>
import AuthenticationLayout from '@mobile/components/layouts/AuthenticationLayout.vue'
import SignUpScreenForm from '@mobile/components/signUp/SignUpScreenForm.vue'
import SignUpScreenHeader from '@mobile/components/signUp/SignUpScreenHeader.vue'
import { LOGIN_ROUTE } from '@mobile/router/routeNames'
</script>

<style></style>
