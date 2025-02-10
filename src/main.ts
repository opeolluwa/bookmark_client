import '@styles/app.css'
import '@styles/tailwind.css'

import { createPinia } from 'pinia'
import { createApp } from 'vue'

import Desktop from '@desktop/App.vue'
import Mobile from '@mobile/App.vue'

const isMobile = true

const app = createApp(isMobile ? Mobile : Desktop)

import BackArrowLong from '@shared/components/icons/BackArrowLong.vue'
import HeadingText from '@shared/components/typography/AppHeadingText.vue'
import SmallText from '@shared/components/typography/AppSmallText.vue'
import FormButton from '@shared/components/form/FormButton.vue'
import PageHeadingText from '@shared/components/typography/AppPageHeadingText.vue'
import AppView from '@shared/components/ui/AppView.vue'

import desktopRouter from '../desktop/router'
import mobileRouter from '../mobile/router'

app.use(createPinia())
app.use(isMobile ? mobileRouter : desktopRouter)

app.component('HeadingText', HeadingText)
app.component('SmallText', SmallText)
app.component('BackArrowLong', BackArrowLong)
app.component('FormButton', FormButton)
app.component('PageHeadingText', PageHeadingText)
app.component('AppView', AppView)

app.mount('#app')
