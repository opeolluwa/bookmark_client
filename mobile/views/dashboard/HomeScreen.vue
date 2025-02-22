<template>
  <DashboardLayout>
    <template #header>
      <AppHeader class="h-max-[2rem] my-0 py-0">
        <div class="flex items-center gap-x-3">
          <Bars3Icon class="size-6 my-0 py-0" @click="openDrawer = true" />
          <PageHeadingText text="Bookmarks" />
        </div>
        <a href="/dashboard/search">
          <MagnifyingGlassIcon class="size-6" />
        </a>
      </AppHeader>

      <Drawer v-model:visible="openDrawer">
        <template #header>
          <div class="flex items-center gap-2">
            <Avatar image="/images/avatar/amyelsner.png" shape="circle" />
            <span class="font-bold">Amy Elsner</span>
          </div>
        </template>

        <template #footer>
          <div class="border-t-gray-300/50 border-t-[1.75px] mt-3">
            <button class="btn text-red-500/70 border-none rounded flex gap-x-2 bg-transparent">
              <ArrowLeftEndOnRectangleIcon class="size-5" />
              Logout
            </button>
          </div>
        </template>
      </Drawer>
    </template>

    <template #content>
      hey content

      <RouterLink :to="EDITOR_BASE_ROUTE" class="fab mb-3">
        <PlusIcon />
      </RouterLink>
    </template>
  </DashboardLayout>
</template>

<script lang="ts" setup>
import { MagnifyingGlassIcon, Bars3Icon } from '@heroicons/vue/24/solid'
import DashboardLayout from '@mobile/components/layouts/DashboardLayout.vue'

import Drawer from 'primevue/drawer'
import { ref } from 'vue'
import { ArrowLeftEndOnRectangleIcon, PlusIcon } from '@heroicons/vue/24/outline'
import { EDITOR_BASE_ROUTE } from '@mobile/router/routeNames'
import AppHeader from '@mobile/components/header/AppHeader.vue'

const openDrawer = ref(false)
</script>

<style></style>
