<script setup lang="ts">
import { ref, computed } from 'vue';
import CodeEditor from './CodeEditor.vue';

const props = defineProps<{
  content: string;
  showPreview: boolean;
  splitView: boolean;
  background?: string;
  backgroundOpacity?: number;
  backgroundBlur?: number;
  backgroundSize?: string;
  backgroundPositionX?: number;
  backgroundPositionY?: number;
}>();
defineEmits<{
  'update:content': [value: string];
  'title-required': [];
}>();

const previewHtml = ref('');

const bgStyle = computed(() => {
  if (!props.background) return undefined;
  return {
    '--bg-image': props.background,
    '--bg-opacity': (props.backgroundOpacity ?? 100) / 100,
    '--bg-blur': `${props.backgroundBlur ?? 0}px`,
    '--bg-size': props.backgroundSize ?? 'cover',
    '--bg-pos-x': `${props.backgroundPositionX ?? 50}%`,
    '--bg-pos-y': `${props.backgroundPositionY ?? 50}%`,
  } as Record<string, string | number>;
});
</script>

<template>
  <div
    class="editor-area"
    :class="{ 'is-split': splitView, 'has-bg': !!background }"
    :style="bgStyle"
  >
    <template v-if="splitView">
      <div class="editor-pane">
        <CodeEditor
          :content="content"
          :show-preview="false"
          @update:content="$emit('update:content', $event)"
          @preview-html="previewHtml = $event"
          @title-required="$emit('title-required')"
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
        @title-required="$emit('title-required')"
      />
    </template>
  </div>
</template>

<style scoped>
.editor-area {
  flex: 1;
  overflow: hidden;
}

.editor-area.has-bg {
  position: relative;
}

.editor-area.has-bg::before {
  content: '';
  position: absolute;
  inset: 0;
  background: var(--bg-image);
  background-size: var(--bg-size, cover);
  background-position: var(--bg-pos-x, 50%) var(--bg-pos-y, 50%);
  background-repeat: no-repeat;
  opacity: var(--bg-opacity, 1);
  filter: blur(var(--bg-blur, 0px));
  z-index: 0;
  pointer-events: none;
}

.editor-area.has-bg > * {
  position: relative;
  z-index: 1;
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
