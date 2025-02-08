<template>
  <ul class="btm-nav flex">
    <li v-for="(item, index) in bottomNavigationRoutes" :key="index">
      <BottomNavigationItem :label="item.label" :path="item.path" />
    </li>
  </ul>
</template>

<script lang="ts" setup>
import { defineProps } from 'vue'
import BottomNavigationItem from './BottomNavigationItem.vue'
import { bottomNavigationRoutes } from './bottomNavigation';
defineProps({
  isActive: Boolean,
  path: String,
  label: String,
})
</script>

<style></style>
