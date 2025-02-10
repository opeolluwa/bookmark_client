import {
  LOGIN_ROUTE,
  SIGN_UP_ROUTE,
  FORGOTTEN_PASSWORD_ROUTE,
  CONFIRM_OTP_ROUTE,
  SET_NEW_PASSWORD_ROUTE,
} from './routeNames'
import ConfirmOtpScreen from '@mobile/views/authentication/ConfirmOtpScreen.vue'
import ForgottenPasswordScreen from '@mobile/views/authentication/ForgottenPasswordScreen.vue'
import LoginScreen from '@mobile/views/authentication/LoginScreen.vue'
import SignUpScreen from '@mobile/views/authentication/SignUpScreen.vue'
import SetNewPasswordScreen from '@mobile/views/authentication/SetNewPasswordScreen.vue'
import type { RouteRecordRaw } from 'vue-router'

export default [
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
] as RouteRecordRaw[]
