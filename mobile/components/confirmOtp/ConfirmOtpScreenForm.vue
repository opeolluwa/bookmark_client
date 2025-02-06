<script setup lang="ts">
import { OTPInput, REGEXP_ONLY_DIGITS } from 'vue-input-otp'
import { toast } from 'vue-sonner'
import { nextTick, onMounted, onUnmounted, ref } from 'vue'
import { balloons } from 'balloons-js'
import Slot from './OtpSlot.vue'
import { cn } from '@lib/utils'

const input = ref('')
const inputRef = ref<{ ref: HTMLInputElement } | null>(null)
const disabled = ref(false)

let t1: ReturnType<typeof setTimeout>
let t2: ReturnType<typeof setTimeout>

onMounted(() => {
  const isMobile = window.matchMedia('(max-width: 1023px)').matches
  if (!isMobile) disabled.value = true

  nextTick(() => {
    t1 = setTimeout(() => {
      disabled.value = false
    }, 1_900)
    t2 = setTimeout(
      () => {
        inputRef.value?.ref.focus()
      },
      isMobile ? 0 : 2_500,
    )
  })
})

onUnmounted(() => {
  clearTimeout(t1)
  clearTimeout(t2)
})

//TOdo: use kooks here
async function onSubmit(e?: Event | string) {
  inputRef.value?.ref.select()
  await new Promise((r) => setTimeout(r, 1_00))

  if (typeof e !== 'string') e?.preventDefault?.()

  if (input.value === '123456') {
    toast('Try guessing the right password 🤔')
    balloons()
  } else {
    toast('Try guessing the right password 🤔')
  }

  input.value = ''
  setTimeout(() => {
    inputRef.value?.ref?.blur()
  }, 20)
}
</script>
<template>
  <form :class="cn(' flex justify-between my-2 py-4 ', $attrs.class as string)" @submit="onSubmit">
    <OTPInput
      ref="inputRef"
      v-slot="{ slots, isFocused }"
      v-model="input"
      :disabled="disabled"
      :maxlength="6"
      :pattern="REGEXP_ONLY_DIGITS"
      container-class="group flex items-center"
      aria-label="showcase-otp-input"
      @complete="onSubmit"
    >
      <div class="flex">
        <Slot
          v-for="(slot, idx) in slots.slice(0, 3)"
          :key="idx"
          :is-focused="isFocused"
          :animate-idx="idx"
          v-bind="slot"
        />
      </div>

      <!-- Layout inspired by Stripe -->
      <div class="flex w-10 md:20 justify-center items-center">
        <div class="w-6 h-1.5 rounded-full bg-black/50" />
      </div>

      <div class="flex">
        <Slot
          v-for="(slot, idx) in slots.slice(3)"
          :key="idx"
          :is-focused="isFocused"
          v-bind="slot"
        />
      </div>
    </OTPInput>
  </form>
</template>
