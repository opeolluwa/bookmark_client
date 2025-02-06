<template>
  <form class="flex flex-col gap-y-4 mt-6">
    <template v-if="!cachedUserDataExist">
      <div class="form-input">
        <label for="email">Email</label>
        <input value="email" type="email" placeholder="type your email" />
      </div>
      <div class="form-input">
        <label for="password">Password</label>
        <input value="password" type="password" placeholder="type your password" />
      </div>
    </template>

    <template v-else>
      <div class="form-input">
        <label for="password">Password</label>
        <input value="password" type="password" placeholder="type your password" />
      </div>
    </template>
    <FormButton text="continue" />
  </form>
</template>

<script lang="ts" setup>
import { useCachedUserStore } from '@shared/stores/cachedUser'
import { storeToRefs } from 'pinia'

const cachedUserData = useCachedUserStore()
const { cachedUserDataExist } = storeToRefs(cachedUserData)
</script>

<style></style>
