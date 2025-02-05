import { createRouter, createWebHistory } from 'vue-router'
import LoginScreen from '@mobile/views/LoginScreen.vue'
// import { loadLayoutMiddleware } from './injectLayout'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: LoginScreen,
    },
  ],
})

// router.beforeEach(loadLayoutMiddleware)
export default router
