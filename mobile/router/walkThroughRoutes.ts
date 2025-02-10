import WalkThroughLayout from '@mobile/views/walkThrough/WalkThroughLayout.vue'
import {
  WALK_THROUGH_BASE_ROUTE,
  WALK_THROUGH_FEATURES_ROUTE,
  WALK_THROUGH_GET_STARTED_ROUTE,
  WALK_THROUGH_WELCOME_ROUTE,
} from './routeNames'
import WelcomeScreen from '@mobile/views/walkThrough/WelcomeScreen.vue'
import FeaturesScreen from '@mobile/views/walkThrough/FeaturesScreen.vue'
import GetStartedScreen from '@mobile/views/walkThrough/GetStartedScreen.vue'
import type { RouteRecordRaw } from 'vue-router'

export default [
  {
    path: WALK_THROUGH_BASE_ROUTE,
    component: WalkThroughLayout,
    children: [
      {
        path: WALK_THROUGH_WELCOME_ROUTE,
        component: WelcomeScreen,
      },
      {
        path: WALK_THROUGH_FEATURES_ROUTE,
        component: FeaturesScreen,
      },
      {
        path: WALK_THROUGH_GET_STARTED_ROUTE,
        component: GetStartedScreen,
      },
    ],
  },
] as RouteRecordRaw[]
