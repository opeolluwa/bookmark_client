import ForgottenPasswordScreen from '@mobile/views/ForgottenPasswordScreen.vue'
import LoginScreen from '@mobile/views/LoginScreen.vue'
import SignUpScreen from '@mobile/views/SignUpScreen.vue'
import { createRouter, createWebHistory } from 'vue-router'
import { FORGOTTEN_PASSWORD_ROUTE, LOGIN_ROUTE, SIGN_UP_ROUTE } from './routeNames'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: LOGIN_ROUTE,
      component: LoginScreen,
    },
    {
      path: SIGN_UP_ROUTE,
      component: SignUpScreen,
    },
    {
      path: FORGOTTEN_PASSWORD_ROUTE,
      component: ForgottenPasswordScreen,
    },
  ],
})

export default router
