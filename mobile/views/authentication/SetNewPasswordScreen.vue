<template>
  <AuthenticationLayout>
    <SetNewPasswordScreenHeader />
    <SetNewPasswordScreenForm />
  </AuthenticationLayout>
</template>

<script lang="ts" setup>
import AuthenticationLayout from '@mobile/components/layouts/AuthenticationLayout.vue'

import SetNewPasswordScreenForm from '@mobile/setNewPassword/SetNewPasswordScreenForm.vue'
import SetNewPasswordScreenHeader from '@mobile/setNewPassword/SetNewPasswordScreenHeader.vue'
</script>

<style></style>
