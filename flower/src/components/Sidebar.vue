<script setup lang="ts">
import { ref, computed } from 'vue';

export interface Note {
  id: string;
  title: string;
  content: string;
  updatedAt: string;
  wordCount: number;
}

const props = defineProps<{
  notes: Note[];
  activeId: string | null;
}>();

defineEmits<{
  'select-note': [id: string];
  'new-note': [];
  'delete-note': [id: string];
}>()

const searchQuery = ref('');

const filteredNotes = computed(() => {
  if (!searchQuery.value.trim()) return props.notes;
  const q = searchQuery.value.trim().toLowerCase();
  return props.notes.filter(
    (n: Note) =>
      n.title.toLowerCase().includes(q) ||
      n.content.toLowerCase().includes(q),
  );
});

function formatDateTime(dateStr: string): string {
  const d = new Date(dateStr);
  return d.toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  });
}
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar__header">
      <div class="sidebar__search">
        <svg class="sidebar__search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
        <input
          v-model="searchQuery"
          class="sidebar__search-input"
          placeholder="搜索笔记..."
        />
      </div>
      <button class="sidebar__new-btn" title="新建笔记" @click="$emit('new-note')">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <line x1="12" y1="5" x2="12" y2="19" />
          <line x1="5" y1="12" x2="19" y2="12" />
        </svg>
        <span>新建笔记</span>
      </button>
      <button class="sidebar__import-btn" title="导入 Markdown">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
          <polyline points="17 8 12 3 7 8" />
          <line x1="12" y1="3" x2="12" y2="15" />
        </svg>
        <span>导入 Markdown</span>
      </button>
    </div>

    <div class="sidebar__list">
      <div
        v-for="note in filteredNotes"
        :key="note.id"
        class="sidebar__item"
        :class="{ 'is-active': note.id === activeId }"
        @click="$emit('select-note', note.id)"
      >
        <div class="sidebar__item-title">{{ note.title || '无标题' }}</div>
        <div class="sidebar__item-meta">
          <span class="sidebar__item-preview">{{ note.content.slice(0, 60).replace(/\n/g, ' ') || ' ' }}</span>
        </div>
        <div class="sidebar__item-footer">
          <span class="sidebar__item-date">{{ formatDateTime(note.updatedAt) }}</span>
          <span class="sidebar__item-count">{{ note.wordCount }} 字</span>
        </div>
      </div>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 280px;
  min-width: 280px;
  display: flex;
  flex-direction: column;
  background: var(--sidebar-bg, #1a1a1a);
  border-right: 1px solid var(--border-color, #2a2a2a);
  user-select: none;
  overflow: hidden;
}

.sidebar__header {
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex-shrink: 0;
}

.sidebar__search {
  position: relative;
  display: flex;
  align-items: center;
}

.sidebar__search-icon {
  position: absolute;
  left: 10px;
  width: 16px;
  height: 16px;
  color: var(--text-muted, #666);
  pointer-events: none;
}

.sidebar__search-input {
  width: 100%;
  height: 36px;
  padding: 0 12px 0 32px;
  border: 1px solid var(--border-color, #2a2a2a);
  border-radius: 6px;
  background: var(--sidebar-search-bg, #252525);
  color: var(--text-color, #d4d4d4);
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s;
}

.sidebar__search-input::placeholder {
  color: var(--text-muted, #666);
}

.sidebar__search-input:focus {
  border-color: var(--accent-color, #4a90d9);
}

.sidebar__new-btn,
.sidebar__import-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-color, #d4d4d4);
  font-size: 13px;
  cursor: pointer;
  transition: background 0.15s;
  text-align: left;
}

.sidebar__new-btn:hover,
.sidebar__import-btn:hover {
  background: var(--sidebar-item-hover, #2a2a2a);
}

.sidebar__new-btn svg,
.sidebar__import-btn svg {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
  color: var(--accent-color, #4a90d9);
}

.sidebar__new-btn span {
  color: var(--accent-color, #4a90d9);
  font-weight: 500;
}

.sidebar__list {
  flex: 1;
  overflow-y: auto;
  padding: 0 8px 12px;
}

.sidebar__item {
  padding: 10px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.15s;
  margin-bottom: 2px;
}

.sidebar__item:hover {
  background: var(--sidebar-item-hover, #2a2a2a);
}

.sidebar__item.is-active {
  background: var(--sidebar-item-active, #2d3a2d);
}

.sidebar__item-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-color, #d4d4d4);
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sidebar__item.is-active .sidebar__item-title {
  color: var(--accent-color, #4a90d9);
}

.sidebar__item-meta {
  font-size: 12px;
  color: var(--text-muted, #888);
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1.4;
}

.sidebar__item-footer {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 11px;
  color: var(--text-muted, #666);
}

.sidebar__item-count {
  margin-left: auto;
}
</style>
