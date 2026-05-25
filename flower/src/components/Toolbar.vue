<script setup lang="ts">
defineEmits<{
  'file-new': [];
  'file-open': [];
  'file-save': [];
  'file-save-as': [];
  'set-mode': [mode: 'edit' | 'split' | 'preview'];
}>();

defineProps<{ showPreview: boolean; splitView: boolean }>();
</script>

<template>
  <div class="toolbar">
    <div class="toolbar__left">
      <button class="toolbar__icon-btn" title="New (Ctrl+N)" @click="$emit('file-new')">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
          <line x1="12" y1="8" x2="12" y2="16" />
          <line x1="8" y1="12" x2="16" y2="12" />
        </svg>
      </button>
      <button class="toolbar__icon-btn" title="Open (Ctrl+O)" @click="$emit('file-open')">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
          <polyline points="14 2 14 8 20 8" />
        </svg>
      </button>
      <button class="toolbar__icon-btn" title="Save (Ctrl+S)" @click="$emit('file-save')">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
          <polyline points="17 21 17 13 7 13 7 21" />
          <polyline points="7 3 7 8 15 8" />
        </svg>
      </button>
      <button class="toolbar__icon-btn" title="Save As (Ctrl+Shift+S)" @click="$emit('file-save-as')">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
          <polyline points="17 21 17 13 7 13 7 21" />
          <polyline points="7 3 7 8 15 8" />
          <line x1="17" y1="3" x2="17" y2="8" />
          <line x1="14" y1="5" x2="20" y2="5" />
        </svg>
      </button>
    </div>
    <div class="toolbar__spacer"></div>
    <div class="toolbar__right">
      <button
        class="toolbar__mode-btn"
        :class="{ 'is-active': !showPreview && !splitView }"
        @click="$emit('set-mode', 'edit')"
      >
        编辑
      </button>
      <button
        class="toolbar__mode-btn"
        :class="{ 'is-active': splitView }"
        @click="$emit('set-mode', 'split')"
      >
        分栏
      </button>
      <button
        class="toolbar__mode-btn"
        :class="{ 'is-active': showPreview && !splitView }"
        @click="$emit('set-mode', 'preview')"
      >
        预览
      </button>
    </div>
  </div>
</template>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  height: 36px;
  padding: 0 12px;
  background: var(--toolbar-bg, #f8f8f8);
  border-bottom: 1px solid var(--border-color, #e0e0e0);
  flex-shrink: 0;
  gap: 4px;
}
.toolbar__left {
  display: flex;
  gap: 2px;
}
.toolbar__spacer {
  flex: 1;
}
.toolbar__right {
  display: flex;
  gap: 2px;
  background: var(--toolbar-bg, #2d2d2d);
  border-radius: 6px;
  padding: 2px;
}
.toolbar__icon-btn {
  width: 28px;
  height: 28px;
  padding: 0;
  border: 1px solid transparent;
  border-radius: 4px;
  background: transparent;
  color: var(--text-muted, #888);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.15s, background 0.15s;
}
.toolbar__icon-btn:hover {
  background: var(--btn-hover, #3c3c3c);
  color: var(--text-color, #d4d4d4);
}
.toolbar__icon-btn svg {
  width: 16px;
  height: 16px;
}
.toolbar__mode-btn {
  padding: 4px 14px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-muted, #888);
  font-size: 13px;
  cursor: pointer;
  transition: color 0.15s, background 0.15s;
}
.toolbar__mode-btn:hover {
  color: var(--text-color, #d4d4d4);
}
.toolbar__mode-btn.is-active {
  background: var(--btn-hover, #3c3c3c);
  color: var(--text-color, #d4d4d4);
}
</style>
