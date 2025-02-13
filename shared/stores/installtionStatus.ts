import { defineStore } from 'pinia'
import { APP_INSTALLATION_STATUS } from './storeNames'

export const useInstallationStatus = defineStore(APP_INSTALLATION_STATUS, {
  state: () => {
    const appIsInstalled = localStorage.getItem('bookmark.isInstalled')
    return { appIsInstalled }
  },
  actions: {
    setInstallationStatus() {
      localStorage.setItem('bookmark.appIsInstalled', 'true')
    },
  },
})
