import '@/styles/app.css'
import Aura from '@primevue/themes/aura'
import PrimeVue from 'primevue/config'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import Mobile from '@mobile/App.vue'
import Desktop from '@desktop/App.vue'

const isMobile = true

const app = createApp(isMobile ? Mobile : Desktop)

import mobileRouter from '../mobile/router'
import desktopRouter from '../desktop/router'

app.use(createPinia())
app.use(isMobile ? mobileRouter : desktopRouter)
// app.use(PrimeVue, {
//   theme: {
//     preset: Aura,
//     options: {
//       cssLayer: {
//         // name: 'primevue',
//         // order: 'app-styles, primevue, another-css-library',
//       },
//     },
//   },
// })

app.mount('#app')
