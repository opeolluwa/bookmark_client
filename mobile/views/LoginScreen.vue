<template>
  <AuthenticationLayout class="relative" style="height: calc(100vh - 100px)" id="editor">
    <LoginScreenHeader />
    <LoginScreenForm />
    <a href="/dashboard" class="text-app block text-sm font-bold mt-3"> Forgotten password? </a>
  </AuthenticationLayout>
</template>

<script lang="ts" setup>
import AuthenticationLayout from '@mobile/components/layouts/AuthenticationLayout.vue'

import LoginScreenHeader from '@mobile/components/login/LoginScreenHeader.vue'
import LoginScreenForm from '@mobile/components/login/LoginScreenForm.vue'
</script>

<style></style>
