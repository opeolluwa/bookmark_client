import {
  DASHBOARD_BASE_ROUTE,
  DASHBOARD_HOME,
  DASHBOARD_NOTIFICATION_ROUTE,
  DASHBOARD_FAVORITES_ROUTES,
  DASHBOARD_SETTINGS_ROUTES,
  DASHBOARD_PROFILE_ROUTES,
} from './routeNames'

import MobileDashboardLayout from '@mobile/views/dashboard/DashboardLayout.vue'
import HomeScreen from '@mobile/views/dashboard/HomeScreen.vue'
import NotificationScreen from '@mobile/views/dashboard/NotificationScreen.vue'
import SettingsScreen from '@mobile/views/dashboard/SettingsScreen.vue'
import FavoritesScreen from '@mobile/views/dashboard/FavoritesScreen.vue'
import ProfileScreen from '@mobile/views/dashboard/ProfileScreen.vue'
import type { RouteRecordRaw } from 'vue-router'
export default [
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
] as RouteRecordRaw[]
