<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  filePath: string | null;
  noteTitle?: string;
  noteDate?: string;
  noteWords?: number;
  saved?: boolean;
}>();

defineEmits<{ 'open-settings': [] }>();

const displayTitle = computed(() => {
  if (props.filePath) return props.filePath.split(/[\\/]/).pop() || 'Untitled';
  return props.noteTitle || 'Untitled';
});
</script>

<template>
  <div class="title-bar" data-tauri-drag-region>
    <div class="title-bar__left">
      <h1 class="title-bar__title">{{ displayTitle }}</h1>
      <div class="title-bar__meta" v-if="noteDate">
        <span class="title-bar__date">{{ noteDate }}</span>
        <span class="title-bar__dot" v-if="noteWords !== undefined">·</span>
        <span class="title-bar__words" v-if="noteWords !== undefined">{{ noteWords }} 字</span>
        <span class="title-bar__dot">·</span>
        <span class="title-bar__saved" :class="{ 'is-saved': saved }">
          {{ saved ? '已保存' : '未保存' }}
        </span>
      </div>
    </div>
    <button class="title-bar__settings-btn" title="设置" @click="$emit('open-settings')">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="3" />
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9 1.65 1.65 0 0 0 4.27 7.18l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
      </svg>
    </button>
  </div>
</template>

<style scoped>
.title-bar {
  display: flex;
  align-items: center;
  height: 36px;
  padding: 0 16px;
  background: var(--title-bar-bg, #f0f0f0);
  border-bottom: 1px solid var(--border-color, #e0e0e0);
  user-select: none;
  flex-shrink: 0;
}
.title-bar__left {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
  min-width: 0;
}
.title-bar__title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-color, #d4d4d4);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1;
}
.title-bar__meta {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--text-muted, #888);
}
.title-bar__dot {
  color: var(--text-muted, #666);
}
.title-bar__saved {
  color: var(--text-muted, #888);
}
.title-bar__saved.is-saved {
  color: var(--accent-color, #4a90d9);
}
.title-bar__settings-btn {
  width: 28px;
  height: 28px;
  padding: 0;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-muted, #888);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.15s, background 0.15s;
  flex-shrink: 0;
}
.title-bar__settings-btn:hover {
  background: var(--btn-hover, #3c3c3c);
  color: var(--text-color, #d4d4d4);
}
.title-bar__settings-btn svg {
  width: 16px;
  height: 16px;
}
</style>
