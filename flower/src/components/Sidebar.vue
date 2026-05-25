<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import SidebarNoteTree, { type VaultEntry } from './SidebarNoteTree.vue';

export interface Note {
  id: string;
  title: string;
  content: string;
  updatedAt: string;
  wordCount: number;
  path?: string;
}

const props = defineProps<{
  notes: Note[];
  activeId: string | null;
  vaultPath?: string;
  vaultTree?: VaultEntry[] | null;
  background?: string;
  backgroundOpacity?: number;
  backgroundBlur?: number;
  backgroundSize?: string;
  backgroundPositionX?: number;
  backgroundPositionY?: number;
}>();

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

const emit = defineEmits<{
  'select-note': [id: string];
  'new-note': [];
  'new-folder': [];
  'new-note-in-dir': [dirPath: string];
  'new-folder-in-dir': [dirPath: string];
  'delete-note': [id: string];
  'delete-folder': [path: string];
  'import-note': [];
  'rename-note': [id: string, newName: string];
  'rename-folder': [path: string, newName: string];
}>()

const searchQuery = ref('');

// Context menu state
interface CtxEntry { x: number; y: number; entry: VaultEntry }
const contextMenu = ref<CtxEntry | null>(null);
const renamePopup = ref<{ x: number; y: number; entry: VaultEntry } | null>(null);
const renameValue = ref('');

// Flatten tree for search
function flattenTree(entries: VaultEntry[]): VaultEntry[] {
  const result: VaultEntry[] = [];
  for (const entry of entries) {
    if (entry.entry_type === 'file') {
      result.push(entry);
    }
    if (entry.children.length > 0) {
      result.push(...flattenTree(entry.children));
    }
  }
  return result;
}

const hasSearch = computed(() => searchQuery.value.trim().length > 0);

const flatFileNodes = computed(() => {
  if (!props.vaultTree) return [];
  return flattenTree(props.vaultTree);
});

const filteredNotes = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return [];
  return props.notes.filter(
    (n: Note) =>
      n.title.toLowerCase().includes(q) ||
      n.content.toLowerCase().includes(q),
  );
});

const filteredTreeFiles = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return [];
  return flatFileNodes.value.filter(
    (e: VaultEntry) =>
      e.name.toLowerCase().includes(q) &&
      !props.notes.some(n => n.id === e.path)
  );
});

// --- Context menu handlers ---

function handleContextEntry(e: MouseEvent, entry: VaultEntry) {
  contextMenu.value = { x: e.clientX, y: e.clientY, entry };
}

function closeContextMenu() {
  contextMenu.value = null;
}

function handleCtxRename() {
  if (!contextMenu.value) return;
  const { x, y, entry } = contextMenu.value;
  renamePopup.value = { x, y, entry };
  renameValue.value = entry.name;
  contextMenu.value = null;
}

function confirmRenamePopup() {
  if (!renamePopup.value) return;
  const trimmed = renameValue.value.trim();
  if (trimmed && trimmed !== renamePopup.value.entry.name) {
    if (renamePopup.value.entry.entry_type === 'file') {
      emit('rename-note', renamePopup.value.entry.path, trimmed);
    } else {
      emit('rename-folder', renamePopup.value.entry.path, trimmed);
    }
  }
  renamePopup.value = null;
}

function cancelRenamePopup() {
  renamePopup.value = null;
}

function handleRenamePopupKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    e.preventDefault();
    confirmRenamePopup();
  } else if (e.key === 'Escape') {
    e.preventDefault();
    cancelRenamePopup();
  }
}

function handleCtxDelete() {
  if (!contextMenu.value) return;
  const entry = contextMenu.value.entry;
  if (entry.entry_type === 'file') {
    emit('delete-note', entry.path);
  } else {
    emit('delete-folder', entry.path);
  }
  contextMenu.value = null;
}

function handleCtxNewNoteInDir() {
  if (!contextMenu.value) return;
  emit('new-note-in-dir', contextMenu.value.entry.path);
  contextMenu.value = null;
}

function handleCtxNewFolderInDir() {
  if (!contextMenu.value) return;
  emit('new-folder-in-dir', contextMenu.value.entry.path);
  contextMenu.value = null;
}

// --- Tree note click — load content from disk ---

function handleSelectNote(path: string) {
  emit('select-note', path);
}

// --- Delete click from tree file ---

function handleTreeDeleteNote(path: string) {
  emit('delete-note', path);
}

function handleDeleteClick(e: MouseEvent, id: string) {
  e.stopPropagation();
  emit('delete-note', id);
}

function formatDateTime(dateStr: string): string {
  const d = new Date(dateStr);
  return d.toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  });
}

function onGlobalClick() {
  closeContextMenu();
}

onMounted(() => {
  document.addEventListener('click', onGlobalClick);
});

onUnmounted(() => {
  document.removeEventListener('click', onGlobalClick);
});
</script>

<template>
  <aside class="sidebar" :class="{ 'has-bg': !!background }" :style="bgStyle" @contextmenu.prevent>
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
      <button class="sidebar__new-btn" title="新建笔记" @click="emit('new-note')">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <line x1="12" y1="5" x2="12" y2="19" />
          <line x1="5" y1="12" x2="19" y2="12" />
        </svg>
        <span>新建笔记</span>
      </button>
      <button class="sidebar__new-btn" title="新建文件夹" @click="emit('new-folder')">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
          <line x1="12" y1="11" x2="12" y2="17" />
          <line x1="9" y1="14" x2="15" y2="14" />
        </svg>
        <span>新建文件夹</span>
      </button>
      <button class="sidebar__import-btn" title="导入 Markdown" @click="emit('import-note')">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
          <polyline points="17 8 12 3 7 8" />
          <line x1="12" y1="3" x2="12" y2="15" />
        </svg>
        <span>导入 Markdown</span>
      </button>
    </div>

    <div class="sidebar__list">
      <!-- No vault -->
      <div v-if="!vaultPath" class="sidebar__empty">
        <p>请先在设置中选择笔记库路径</p>
      </div>

      <!-- Vault + No search: tree view -->
      <template v-else-if="!hasSearch">
        <div v-if="vaultTree && vaultTree.length === 0" class="sidebar__empty">
          <p>暂无笔记</p>
        </div>
        <SidebarNoteTree
          v-else-if="vaultTree"
          :entries="vaultTree"
          :active-id="activeId"
          @select-note="handleSelectNote"
          @delete-note="handleTreeDeleteNote"
          @rename-note="(path, name) => emit('rename-note', path, name)"
          @context-entry="handleContextEntry"
        />
      </template>

      <!-- Search: flat list -->
      <template v-else>
        <div v-if="filteredNotes.length === 0 && filteredTreeFiles.length === 0" class="sidebar__empty">
          <p>没有匹配的笔记</p>
        </div>
        <!-- Flat notes with content -->
        <div
          v-for="note in filteredNotes"
          :key="note.id"
          class="sidebar__item"
          :class="{ 'is-active': note.id === activeId }"
          @click="emit('select-note', note.id)"
          @contextmenu="handleContextEntry($event, { name: note.title, path: note.id, entry_type: 'file', children: [], updated_at: 0 })"
        >
          <div class="sidebar__item-header">
            <div class="sidebar__item-title">{{ note.title || '无标题' }}</div>
            <button
              class="sidebar__item-delete"
              title="删除笔记"
              @click="handleDeleteClick($event, note.id)"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="6" x2="6" y2="18" />
                <line x1="6" y1="6" x2="18" y2="18" />
              </svg>
            </button>
          </div>
          <div class="sidebar__item-meta">
            <span class="sidebar__item-preview">{{ note.content.slice(0, 60).replace(/\n/g, ' ') || ' ' }}</span>
          </div>
          <div class="sidebar__item-footer">
            <span class="sidebar__item-date">{{ formatDateTime(note.updatedAt) }}</span>
            <span class="sidebar__item-count">{{ note.wordCount }} 字</span>
          </div>
        </div>
        <!-- Tree-only files in search (no content loaded) -->
        <div
          v-for="entry in filteredTreeFiles"
          :key="entry.path"
          class="sidebar__item"
          :class="{ 'is-active': entry.path === activeId }"
          @click="emit('select-note', entry.path)"
          @contextmenu="handleContextEntry($event, entry)"
        >
          <div class="sidebar__item-header">
            <div class="sidebar__item-title">{{ entry.name || '无标题' }}</div>
          </div>
          <div class="sidebar__item-footer">
            <span class="sidebar__item-date" v-if="entry.updated_at">{{ formatDateTime(new Date(entry.updated_at * 1000).toISOString()) }}</span>
          </div>
        </div>
      </template>
    </div>

    <!-- Context menu (teleported to body) -->
    <Teleport to="body">
      <div
        v-if="contextMenu"
        class="sidebar__context-menu"
        :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
        @click.stop
      >
        <template v-if="contextMenu.entry.entry_type === 'file'">
          <button class="sidebar__context-item" @click="handleCtxRename">重命名</button>
          <button class="sidebar__context-item sidebar__context-item--danger" @click="handleCtxDelete">删除</button>
        </template>
        <template v-else>
          <button class="sidebar__context-item" @click="handleCtxNewNoteInDir">在此新建笔记</button>
          <button class="sidebar__context-item" @click="handleCtxNewFolderInDir">新建文件夹</button>
          <button class="sidebar__context-item" @click="handleCtxRename">重命名</button>
          <button class="sidebar__context-item sidebar__context-item--danger" @click="handleCtxDelete">删除文件夹</button>
        </template>
      </div>
    </Teleport>

    <!-- Rename popup (teleported to body) -->
    <Teleport to="body">
      <div
        v-if="renamePopup"
        class="sidebar__rename-popup"
        :style="{ left: renamePopup.x + 'px', top: renamePopup.y + 'px' }"
        @click.stop
      >
        <input
          v-model="renameValue"
          class="sidebar__rename-popup-input"
          @keydown="handleRenamePopupKeydown"
        />
        <div class="sidebar__rename-popup-actions">
          <button class="sidebar__rename-popup-btn" @click="confirmRenamePopup">确定</button>
          <button class="sidebar__rename-popup-btn sidebar__rename-popup-btn--cancel" @click="cancelRenamePopup">取消</button>
        </div>
      </div>
    </Teleport>
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

.sidebar.has-bg {
  position: relative;
}

.sidebar.has-bg::before {
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

.sidebar.has-bg > * {
  position: relative;
  z-index: 1;
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
  background: var(--sidebar-search-bg, #282828);
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
  background: var(--sidebar-item-hover, rgba(255,255,255,0.05));
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
  background: var(--sidebar-item-hover, rgba(255,255,255,0.05));
}

.sidebar__item.is-active {
  background: var(--sidebar-item-active, rgba(255,255,255,0.08));
}

.sidebar__item-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 4px;
}

.sidebar__item-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-color, #d4d4d4);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
  min-width: 0;
}

.sidebar__item-delete {
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

.sidebar__item:hover .sidebar__item-delete {
  opacity: 1;
}

.sidebar__item-delete:hover {
  color: #e05555;
  background: rgba(224, 85, 85, 0.15);
}

.sidebar__item-delete svg {
  width: 14px;
  height: 14px;
}

.sidebar__empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  padding: 32px 16px;
  text-align: center;
}

.sidebar__empty p {
  font-size: 13px;
  color: var(--text-muted, #666);
  line-height: 1.6;
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
