import { createApp } from 'vue'
import { createPinia } from 'pinia'

import Mobile from '../apps/mobile/App.vue'
import Desktop from '../apps/desktop/App.vue'

const isMobile = true

const app = createApp(isMobile ? Mobile : Desktop)

import mobileRouter from '../apps/mobile/router'
import desktopRouter from '../apps/desktop/router'

app.use(createPinia())
app.use(isMobile ? mobileRouter : desktopRouter)

app.mount('#app')
