import { defineStore } from 'pinia'
import { APP_INSTALLATION_STATUS } from './storeNames'

export const useInstallationStatus = defineStore(APP_INSTALLATION_STATUS, {
  state: () => {
    const appIsInstalled = localStorage.getItem('bookmark.appIsInstalled')
    return { appIsInstalled }
  },
  actions: {
    setInstallationStatus(status: InstallationStatusEnum) {
      localStorage.setItem('bookmark.appIsInstalled', status.toString()) // 0, and 1 are converted to truthy and falsy values
    },
  },
})

export enum InstallationStatusEnum {
  Installed = 1,
  NotInstalled = 0,
}
