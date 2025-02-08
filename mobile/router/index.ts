import ConfirmOtpScreen from '@mobile/views/ConfirmOtpScreen.vue'
import ForgottenPasswordScreen from '@mobile/views/ForgottenPasswordScreen.vue'
import LoginScreen from '@mobile/views/LoginScreen.vue'
import SignUpScreen from '@mobile/views/SignUpScreen.vue'
import { createRouter, createWebHistory } from 'vue-router'
import {
  CONFIRM_OTP_ROUTE,
  DASHBOARD_BASE_ROUTE,
  DASHBOARD_HOME,
  FORGOTTEN_PASSWORD_ROUTE,
  LOGIN_ROUTE,
  SET_NEW_PASSWORD_ROUTE,
  SIGN_UP_ROUTE,
} from './routeNames'
import SetNewPasswordScreen from '@mobile/views/SetNewPasswordScreen.vue'
import MobileDashboardLayout from '@mobile/views/dashboard/MobileDashboardLayout.vue'
import HomeScreen from '@mobile/views/dashboard/HomeScreen.vue'

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
    {
      path: CONFIRM_OTP_ROUTE,
      component: ConfirmOtpScreen,
    },
    {
      path: SET_NEW_PASSWORD_ROUTE,
      component: SetNewPasswordScreen,
    },
    {
      component: MobileDashboardLayout,
      path: DASHBOARD_BASE_ROUTE,
      children: [
        {
          path: "",
          component: HomeScreen,
        },
      ],
    },
  ],
})

export default router
