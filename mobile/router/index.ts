import { createRouter, createWebHistory } from 'vue-router'

import HomeScreen from '@mobile/views/dashboard/HomeScreen.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: HomeScreen,
    },
  ],
})

export default router
