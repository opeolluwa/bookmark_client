<template>
  <form class="flex flex-col gap-y-4 mt-6">
    <div class="form-input">
      <label for="first_name">First name</label>
      <input
        type="text"
        placeholder="type your first name"
      />
    </div>

    <div class="form-input">
      <label for="last_name">last name</label>
      <input
        type="text"
        placeholder="type your last name"
      />
    </div>
    <div class="form-input">
      <label for="email">Email</label>
      <input
        type="email"
        placeholder="type your email"
      />
    </div>
    <div class="form-input">
      <label for="password">Password</label>
      <input
        type="password"
        placeholder="type your password"
      />
    </div>

   <FormButton text="continue"/>
  </form>
 
</template>

<script lang="ts" setup></script>

<style></style>
