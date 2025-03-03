import {
  HomeIcon,
  Cog6ToothIcon,
  BellIcon,
  UserIcon,
  SparklesIcon,
} from '@heroicons/vue/24/outline'
import {
  HomeIcon as SolidHomeIcon,
  Cog6ToothIcon as SolidCog6Icon,
  BellIcon as SolidBellIcon,
  UserIcon as SolidUserIcon,
  SparklesIcon as SolidSparklesIcon,
} from '@heroicons/vue/24/solid'
import { DASHBOARD_HOME, DASHBOARD_NOTIFICATION_ROUTE } from '@mobile/router/routeNames'

export const bottomNavigationRoutes = [
  {
    path: DASHBOARD_HOME,
    label: 'Home',
    defaultIcon: HomeIcon,
    activeIcon: SolidHomeIcon,
  },
  {
    path: DASHBOARD_NOTIFICATION_ROUTE,
    label: 'notification',
    defaultIcon: BellIcon,
    activeIcon: SolidBellIcon,
  },
  {
    path: 'favorites',
    label: 'favorites',
    defaultIcon: SparklesIcon,
    activeIcon: SolidSparklesIcon,
  },
  {
    path: 'profile',
    label: 'profile',
    defaultIcon: UserIcon,
    activeIcon: SolidUserIcon,
  },
  {
    path: 'settings',
    label: 'settings',
    defaultIcon: Cog6ToothIcon,
    activeIcon: SolidCog6Icon,
  },
]

export const mergeRoute = (base: string, path: string): string => {
  return `${base}/${path}`
}
