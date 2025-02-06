<template>
  <form class="flex flex-col gap-y-4 mt-6">
    <div class="form-input">
      <label for="password">New password</label>
      <input value="password" type="password" placeholder="type your password" />
    </div>
    <div class="form-input">
      <label for="password">Confirm password</label>
      <input value="password" type="password" placeholder="type your password" />
    </div>

    <FormButton text="continue" />
  </form>
</template>

<script lang="ts" setup></script>

<style></style>
