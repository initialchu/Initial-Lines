<script setup lang="ts">
import { ref } from 'vue';
import CodeEditor from './CodeEditor.vue';

defineProps<{ content: string; showPreview: boolean; splitView: boolean; background?: string }>();
defineEmits<{ 'update:content': [value: string] }>();

const previewHtml = ref('');
</script>

<template>
  <div
    class="editor-area"
    :class="{ 'is-split': splitView }"
    :style="background ? { background } : undefined"
  >
    <template v-if="splitView">
      <div class="editor-pane">
        <CodeEditor
          :content="content"
          :show-preview="false"
          @update:content="$emit('update:content', $event)"
          @preview-html="previewHtml = $event"
        />
      </div>
      <div class="editor-divider"></div>
      <div class="preview-pane markdown-body" v-html="previewHtml"></div>
    </template>
    <template v-else>
      <CodeEditor
        :content="content"
        :show-preview="showPreview"
        @update:content="$emit('update:content', $event)"
      />
    </template>
  </div>
</template>

<style scoped>
.editor-area {
  flex: 1;
  overflow: hidden;
}

.editor-area.is-split {
  display: flex;
  flex-direction: row;
}

.editor-pane,
.preview-pane {
  flex: 1;
  overflow: auto;
  min-width: 0;
}

.editor-divider {
  width: 1px;
  flex-shrink: 0;
  background: var(--border-color);
}

.preview-pane {
  padding: 24px 32px;
  max-width: 720px;
  margin: 0 auto;
  font-family: var(--font-editor, 'Iowan Old Style', 'Noto Serif SC', serif);
  font-size: 16px;
  line-height: 1.8;
  color: var(--text-color);
}
</style>
