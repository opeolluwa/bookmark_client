import { createRouter, createWebHistory } from 'vue-router'

import authenticationRoutes from './authenticationRoutes'
import dashboardRoutes from './dashboardRoutes'
import walkThroughRoutes from './walkThroughRoutes'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [...authenticationRoutes, ...walkThroughRoutes, ...dashboardRoutes],
})

export default router
