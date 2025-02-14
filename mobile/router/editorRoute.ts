import type { RouteRecordRaw } from 'vue-router'
import { EDITOR_BASE_ROUTE, EDITOR_NEW_ENTRY_ROUTE } from './routeNames'
import EditorLayout from '@mobile/views/editor/EditorLayout.vue'
import NewBookmarkScreen from '@mobile/views/editor/NewBookmarkScreen.vue'

export default [
  {
    path: EDITOR_BASE_ROUTE,
    component: EditorLayout,
    children: [{ path: EDITOR_NEW_ENTRY_ROUTE, component: NewBookmarkScreen }],
  },
] as RouteRecordRaw[]
