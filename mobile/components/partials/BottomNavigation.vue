<template>
  <ul class="dock flex">
    <li v-for="(item, index) in bottomNavigationRoutes" :key="index">
      <BottomNavigationItem :label="item.label" :path="item.path">
        <span class="size-5">
          <component :is="item.defaultIcon" />
        </span>
      </BottomNavigationItem>
    </li>
  </ul>
</template>

<script lang="ts" setup>
import { defineProps } from 'vue'
import BottomNavigationItem from './BottomNavigationItem.vue'
import { bottomNavigationRoutes } from './bottomNavigation'
defineProps({
  isActive: Boolean,
  path: String,
  label: String,
})
</script>

<style></style>
