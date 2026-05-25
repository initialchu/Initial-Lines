<script setup lang="ts">
import { ref } from 'vue';

export interface VaultEntry {
  name: string;
  path: string;
  entry_type: string;
  children: VaultEntry[];
  updated_at: number;
}

withDefaults(defineProps<{
  entries: VaultEntry[];
  activeId: string | null;
  depth?: number;
}>(), { depth: 0 });

const emit = defineEmits<{
  'select-note': [path: string];
  'delete-note': [path: string];
  'rename-note': [path: string, newName: string];
  'context-entry': [event: MouseEvent, entry: VaultEntry];
}>();

const expandedDirs = ref<Record<string, boolean>>({});

function toggleDir(path: string) {
  expandedDirs.value[path] = !expandedDirs.value[path];
}

function handleContextMenu(e: MouseEvent, entry: VaultEntry) {
  e.preventDefault();
  e.stopPropagation();
  emit('context-entry', e, entry);
}

function handleDeleteClick(e: MouseEvent, path: string) {
  e.stopPropagation();
  emit('delete-note', path);
}

function formatTime(secs: number): string {
  if (!secs) return '';
  const d = new Date(secs * 1000);
  return d.toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  });
}
</script>

<template>
  <template v-for="entry in entries" :key="entry.path">
    <!-- Directory node -->
    <div
      v-if="entry.entry_type === 'dir'"
      class="tree-dir"
      :style="{ paddingLeft: depth * 16 + 'px' }"
      @contextmenu="handleContextMenu($event, entry)"
    >
      <div class="tree-dir__header" @click="toggleDir(entry.path)">
        <svg
          class="tree-dir__arrow"
          :class="{ 'is-open': expandedDirs[entry.path] }"
          viewBox="0 0 24 24"
          fill="currentColor"
        >
          <path d="M8 5l8 7-8 7z" />
        </svg>
        <svg class="tree-dir__icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
        </svg>
        <span class="tree-dir__name">{{ entry.name }}</span>
      </div>
      <div v-if="expandedDirs[entry.path]" class="tree-dir__children">
        <SidebarNoteTree
          :entries="entry.children"
          :active-id="activeId"
          :depth="depth + 1"
          @select-note="emit('select-note', $event)"
          @delete-note="emit('delete-note', $event)"
          @rename-note="(path, name) => emit('rename-note', path, name)"
          @context-entry="(e, entry) => emit('context-entry', e, entry)"
        />
      </div>
    </div>

    <!-- File node -->
    <div
      v-else
      class="tree-file"
      :class="{ 'is-active': entry.path === activeId }"
      :style="{ paddingLeft: depth * 16 + 12 + 'px' }"
      @click="emit('select-note', entry.path)"
      @contextmenu="handleContextMenu($event, entry)"
    >
      <div class="tree-file__header">
        <div class="tree-file__title">{{ entry.name || '无标题' }}</div>
        <button
          class="tree-file__delete"
          title="删除笔记"
          @click="handleDeleteClick($event, entry.path)"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </div>
      <div class="tree-file__meta">
        <span class="tree-file__date" v-if="entry.updated_at">{{ formatTime(entry.updated_at) }}</span>
      </div>
    </div>
  </template>
</template>

<style scoped>
.tree-dir {
  margin-bottom: 1px;
}

.tree-dir__header {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 8px;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
}

.tree-dir__header:hover {
  background: var(--sidebar-item-hover, rgba(255,255,255,0.05));
}

.tree-dir__arrow {
  width: 14px;
  height: 14px;
  color: var(--text-muted, #666);
  flex-shrink: 0;
  transition: transform 0.15s;
}

.tree-dir__arrow.is-open {
  transform: rotate(90deg);
}

.tree-dir__icon {
  width: 15px;
  height: 15px;
  color: var(--accent-color, #4a90d9);
  flex-shrink: 0;
}

.tree-dir__name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-color, #d4d4d4);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tree-file {
  padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.15s;
  margin-bottom: 1px;
}

.tree-file:hover {
  background: var(--sidebar-item-hover, rgba(255,255,255,0.05));
}

.tree-file.is-active {
  background: var(--sidebar-item-active, rgba(255,255,255,0.08));
}

.tree-file__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 2px;
}

.tree-file__title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-color, #d4d4d4);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
  min-width: 0;
}

.tree-file.is-active .tree-file__title {
  color: var(--accent-color, #4a90d9);
}

.tree-file__delete {
  width: 22px;
  height: 22px;
  padding: 0;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-muted, #666);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  opacity: 0;
  transition: opacity 0.15s, color 0.15s;
}

.tree-file:hover .tree-file__delete {
  opacity: 1;
}

.tree-file__delete:hover {
  color: #e05555;
  background: rgba(224, 85, 85, 0.15);
}

.tree-file__delete svg {
  width: 14px;
  height: 14px;
}

.tree-file__meta {
  display: flex;
  align-items: center;
  font-size: 11px;
  color: var(--text-muted, #666);
  padding-left: 0;
}
</style>
