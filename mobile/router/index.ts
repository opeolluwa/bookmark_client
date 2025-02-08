import ConfirmOtpScreen from '@mobile/views/ConfirmOtpScreen.vue'
import ForgottenPasswordScreen from '@mobile/views/ForgottenPasswordScreen.vue'
import LoginScreen from '@mobile/views/LoginScreen.vue'
import SignUpScreen from '@mobile/views/SignUpScreen.vue'
import { createRouter, createWebHistory } from 'vue-router'
import {
  CONFIRM_OTP_ROUTE,
  DASHBOARD_BASE_ROUTE,
  DASHBOARD_FAVORITES_ROUTES,
  DASHBOARD_HOME,
  DASHBOARD_NOTIFICATION_ROUTE,
  DASHBOARD_PROFILE_ROUTES,
  DASHBOARD_SETTINGS_ROUTES,
  // DASHBOARD_HOME,
  FORGOTTEN_PASSWORD_ROUTE,
  LOGIN_ROUTE,
  SET_NEW_PASSWORD_ROUTE,
  SIGN_UP_ROUTE,
} from './routeNames'
import SetNewPasswordScreen from '@mobile/views/SetNewPasswordScreen.vue'
import MobileDashboardLayout from '@mobile/views/dashboard/MobileDashboardLayout.vue'
import HomeScreen from '@mobile/views/dashboard/HomeScreen.vue'
import NotificationScreen from '@mobile/views/dashboard/NotificationScreen.vue'
import SettingsScreen from '@mobile/views/dashboard/SettingsScreen.vue'
import FavoritesScreen from '@mobile/views/dashboard/FavoritesScreen.vue'
import ProfileScreen from '@mobile/views/dashboard/ProfileScreen.vue'

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
          path: DASHBOARD_HOME,
          component: HomeScreen,
        },
        {
          path: DASHBOARD_NOTIFICATION_ROUTE,
          component: NotificationScreen,
        },
        {
          path: DASHBOARD_FAVORITES_ROUTES,
          component: FavoritesScreen,
        },
        {
          path: DASHBOARD_SETTINGS_ROUTES,
          component: SettingsScreen,
        },
        {
          path: DASHBOARD_PROFILE_ROUTES,
          component: ProfileScreen,
        },
      ],
    },
  ],
})

export default router
