<template>
  <DashboardLayout>
    <template #header>
      <AppHeader>
        <PageHeadingText text="Settings" />
      </AppHeader>
    </template>

    <template #content> hey content </template>
  </DashboardLayout>
</template>

<script lang="ts" setup>
import AppHeader from '@mobile/components/header/AppHeader.vue'
import DashboardLayout from '@mobile/components/layouts/DashboardLayout.vue'
</script>

<style></style>
