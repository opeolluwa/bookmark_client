import { defineStore } from 'pinia'
import { APP_INSTALLATION_STATUS } from './storeNames'
import { ref } from 'vue'

export enum InstallationStatusEnum {
  Installed = 1,
  NotInstalled = 0,
}

export const useInstallationStatus = defineStore(APP_INSTALLATION_STATUS, () => {
  const castToBoolean = (value: number | string | null) => Boolean(Number(value))
  const appIsInstalled = ref(castToBoolean(localStorage.getItem('bookmark.appIsInstalled')))

  function setInstallationStatus(status: InstallationStatusEnum) {
    localStorage.setItem('bookmark.appIsInstalled', status.toString())
  }

  return { appIsInstalled, setInstallationStatus }
})
