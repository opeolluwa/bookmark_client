import { definePreset } from '@primevue/themes'
import Aura from '@primevue/themes/aura'

export const PresetTheme = definePreset(Aura, {
  semantic: {
    primary: {
      DEFAULT: '#1368F0',
      50: '#C0D6FB',
      100: '#ACCAFA',
      200: '#86B2F7',
      300: '#6099F5',
      400: '#3981F2',
      500: '#1368F0',
      600: '#0C51BF',
      700: '#093A8A',
      800: '#052455',
      900: '#020E21',
      950: '#000306',
    },
  },
})
