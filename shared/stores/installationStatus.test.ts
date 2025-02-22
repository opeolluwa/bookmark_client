import { describe, expect, test } from '@jest/globals'

import { InstallationStatusEnum } from './installationStatus'

describe('Should confirm the enum variants are properly casted to boolean ', () => {
  test('Should return string ', () => {
    expect(typeof InstallationStatusEnum.Installed.toString).toBe('string')
  })
})
