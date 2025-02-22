<template>
  <EditorLayout>
    <template #header>
      <AppHeader class="flex items-center gap-x-3 justify-start">
        <ChevronLeftIcon class="size-5" @click="router.back" />
        <PageHeadingText text="new bookmark"></PageHeadingText>
      </AppHeader>
    </template>
    <template #content>
      <input
        type="text"
        class="text-xl placeholder:text-xl font-medium mt-4 placeholder:text-gray-400 border-none outline-none"
        placeholder="Bookmark title"
        autofocus
      />
      <EditorContent :editor="editor" class="-mt-3" />

      <div v-if="editor">
        <button
          :disabled="!editor.can().chain().focus().toggleBold().run()"
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          :class="{ 'ring-app-secondary-900 ring-2': editor.isActive('bold') }"
          @click="editor.chain().focus().toggleBold().run()"
          v-text="'bold'"
        />
        <button
          :disabled="!editor.can().chain().focus().toggleItalic().run()"
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          :class="{ 'ring-app-secondary-900 ring-2': editor.isActive('italic') }"
          @click="editor.chain().focus().toggleItalic().run()"
          v-text="'italic'"
        />
        <button
          :disabled="!editor.can().chain().focus().toggleStrike().run()"
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          :class="{ 'ring-app-secondary-900 ring-2': editor.isActive('strike') }"
          @click="editor.chain().focus().toggleStrike().run()"
          v-text="'strike'"
        />
        <button
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          :class="{ 'ring-app-secondary-900 ring-2': editor.isActive('paragraph') }"
          @click="editor.chain().focus().setParagraph().run()"
          v-text="'paragraph'"
        />
        <button
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          :class="{ 'ring-app-secondary-900 ring-2': editor.isActive('heading', { level: 1 }) }"
          @click="editor.chain().focus().toggleHeading({ level: 1 }).run()"
          v-text="'h1'"
        />
        <button
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          :class="{ 'ring-app-secondary-900 ring-2': editor.isActive('heading', { level: 2 }) }"
          @click="editor.chain().focus().toggleHeading({ level: 2 }).run()"
          v-text="'h2'"
        />
        <button
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          :class="{ 'ring-app-secondary-900 ring-2': editor.isActive('heading', { level: 3 }) }"
          @click="editor.chain().focus().toggleHeading({ level: 3 }).run()"
          v-text="'h3'"
        />
        <button
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          :class="{ 'ring-app-secondary-900 ring-2': editor.isActive('bulletList') }"
          @click="editor.chain().focus().toggleBulletList().run()"
          v-text="'bullet list'"
        />
        <button
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          :class="{ 'ring-app-secondary-900 ring-2': editor.isActive('orderedList') }"
          @click="editor.chain().focus().toggleOrderedList().run()"
          v-text="'ordered list'"
        />
        <button
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          :class="{ 'ring-app-secondary-900 ring-2': editor.isActive('blockquote') }"
          @click="editor.chain().focus().toggleBlockquote().run()"
          v-text="'blockquote'"
        />
        <button
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          @click="editor.chain().focus().setHorizontalRule().run()"
          v-text="'horizontal rule'"
        />
        <button
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          @click="editor.chain().focus().setHardBreak().run()"
          v-text="'hard break'"
        />
        <button
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          :disabled="!editor.can().chain().focus().undo().run()"
          @click="editor.chain().focus().undo().run()"
        >
          <ArrowUturnLeftIcon class="h-4 w-4 flex-shrink-0" aria-hidden="true" />
        </button>
        <button
          class="m-1 inline-flex items-center rounded-md bg-app-secondary-50 px-2 py-1 text-xs font-medium text-app-secondary-700 ring-1 ring-inset ring-app-secondary-700/10"
          :disabled="!editor.can().chain().focus().redo().run()"
          @click="editor.chain().focus().redo().run()"
        >
          <ArrowUturnRightIcon class="h-4 w-4 flex-shrink-0" aria-hidden="true" />
        </button>
      </div>
    </template>
  </EditorLayout>
</template>

<script lang="ts" setup>
import { ChevronLeftIcon, ArrowUturnLeftIcon, ArrowUturnRightIcon } from '@heroicons/vue/24/outline'
import AppHeader from '@mobile/components/header/AppHeader.vue'
import EditorLayout from '@mobile/components/layouts/EditorLayout.vue'
import router from '@mobile/router'
import Placeholder from '@tiptap/extension-placeholder'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'

const editor = useEditor({
  content: "<p>I'm running Tiptap with Vue.js. 🎉</p>",
  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: 'Start typing here',
    }),
  ],
  editorProps: {
    attributes: {
      class: 'w-full prose my-6 mx-auto focus:outline-none',
    },
  },
})
</script>

<style></style>
