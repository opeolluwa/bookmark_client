<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  char: string | null
  isActive: boolean
  isFocused: boolean
  animateIdx?: number
}>()

const willAnimateChar = computed(() => props.animateIdx !== undefined && props.animateIdx < 2)
const willAnimateCaret = computed(() => props.animateIdx === 2)

const computedContainerClass = computed(() => [
  'relative w-12 h-12 text-[1.5rem] flex items-center justify-center border-border border-y border-r first:border-l first:rounded-l-md last:rounded-r-md transition-all [transition-duration:300ms] outline-0 outline-gray-400',
  'group-hover:border-accent-foreground/20 group-focus-within:border-accent-foreground/20',
  { 'outline-2 outline-app-500': props.isActive },
])
</script>

<template>
  <div
    :class="[
      'relative w-12 h-12 text-[1.5rem] flex items-center justify-center border-border border-y border-r first:border-l first:rounded-l-md last:rounded-r-md transition-all [transition-duration:300ms] outline-0 outline-app-500',
      'group-hover:border-accent-foreground/20 group-focus-within:border-accent-foreground/20',
      { 'outline-3 outline-app-500': isActive },
    ]"
  >
    <div
      :class="[
        'duration-1000',
        {
          'lg:opacity-0 lg:animate-fade-in': willAnimateChar,
          'lg:[animation-delay:1.5s]': animateIdx === 0,
          'lg:[animation-delay:2s]': animateIdx === 1,
        },
      ]"
    >
      <div v-if="char">
        {{ char }}
      </div>
    </div>

    <div
      v-if="isActive && char === null"
      :class="{
        'lg:opacity-0 lg:animate-fade-in': willAnimateCaret,
      }"
    >
      <!-- Fake Caret -->
      <div
        class="absolute pointer-events-none inset-0 flex items-center justify-center animate-caret-blink [animate-delay:inherit]"
      >
        <div class="w-px h-8 md:w-0.5 md:h-16 bg-white" />
      </div>
    </div>
  </div>
</template>
