import { defineStore } from 'pinia'
import { CACHES_USER_STORE } from './storeNames'
import type { UserInformationInterface } from './types/interfaces/user'

export const useCachedUserStore = defineStore(CACHES_USER_STORE, {
  state: () => {
    return { userInformation: {} as UserInformationInterface, cachedUserDataExist: false }
  },
  actions: {
    cacheUserInformation(userInformation: UserInformationInterface) {
      this.userInformation = userInformation
    },
  },
})
  