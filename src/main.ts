import '@styles/app.css'
import '@styles/tailwind.css'
import PrimeVue from 'primevue/config'
import Ripple from 'primevue/ripple'
import FormButton from '@shared/components/form/FormButton.vue'
import HeadingText from '@shared/components/typography/AppHeadingText.vue'
import PageHeadingText from '@shared/components/typography/AppPageHeadingText.vue'

import SmallText from '@shared/components/typography/AppSmallText.vue'
import AppView from '@shared/components/ui/AppView.vue'
import BackArrowLong from '@shared/components/icons/BackArrowLong.vue'

import { PresetTheme } from './theme'
import { createPinia } from 'pinia'
import { createApp } from 'vue'

import Desktop from '@desktop/App.vue'
import Mobile from '@mobile/App.vue'

import desktopRouter from '../desktop/router'
import mobileRouter from '../mobile/router'

const isMobile = true

const app = createApp(isMobile ? Mobile : Desktop)


app.use(createPinia())
app.use(isMobile ? mobileRouter : desktopRouter)
app.use(PrimeVue, {
  theme: {
    preset: PresetTheme,
  },
  ripple: true,
})

app.directive('ripple', Ripple)

app.component('HeadingText', HeadingText)
app.component('SmallText', SmallText)
app.component('BackArrowLong', BackArrowLong)
app.component('FormButton', FormButton)
app.component('PageHeadingText', PageHeadingText)
app.component('AppView', AppView)

app.mount('#app')
