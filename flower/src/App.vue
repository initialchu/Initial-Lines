<script setup lang="ts">
import { reactive, ref, watch, onMounted, onUnmounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { MilkdownProvider } from '@milkdown/vue';
import TitleBar from './components/TitleBar.vue';
import Toolbar from './components/Toolbar.vue';
import EditorArea from './components/EditorArea.vue';
import SettingsModal from './components/SettingsModal.vue';
import Sidebar from './components/Sidebar.vue';
import type { Settings } from './components/SettingsModal.vue';
import type { Note } from './components/Sidebar.vue';
import type { VaultEntry } from './components/SidebarNoteTree.vue';
import { open } from '@tauri-apps/plugin-dialog';

interface FileData {
  path: string;
  content: string;
}

const SETTINGS_KEY = 'chujian-settings';
const NOTES_KEY = 'chujian-notes';

function loadSettings(): Settings {
  try {
    const raw = localStorage.getItem(SETTINGS_KEY);
    if (raw) return JSON.parse(raw);
  } catch { /* ignore corrupt data */ }
  return { background: '', backgroundOpacity: 100, backgroundBlur: 0, backgroundSize: 'cover', backgroundPositionX: 50, backgroundPositionY: 50, sidebarBackground: '', sidebarBackgroundOpacity: 100, sidebarBackgroundBlur: 0, sidebarBackgroundSize: 'cover', sidebarBackgroundPositionX: 50, sidebarBackgroundPositionY: 50, defaultSavePath: '', vaultPath: '' };
}

function loadNotes(): Note[] {
  try {
    const raw = localStorage.getItem(NOTES_KEY);
    if (raw) return JSON.parse(raw);
  } catch { /* ignore corrupt data */ }
  return [];
}

function saveNotes(notes: Note[]) {
  localStorage.setItem(NOTES_KEY, JSON.stringify(notes));
}

const vaultTree = ref<VaultEntry[] | null>(null);

async function loadVaultTree(dir: string) {
  try {
    const tree = await invoke<VaultEntry[]>('list_vault_tree', { dir });
    vaultTree.value = tree;
    // Keep active note in cache so auto-save continues working after tree refresh
    const active = notes.value.find(n => n.id === activeNoteId.value);
    notes.value = active ? [active] : [];
  } catch (e) {
    console.error('Failed to list vault tree:', e);
  }
}

async function loadNoteContent(note: Note) {
  if (!note.path) return;
  try {
    const content = await invoke<string>('read_note', { path: note.path });
    note.content = content;
    currentContent.value = content;
    const firstLine = content.split('\n')[0].trim();
    note.title = firstLine.replace(/^#+\s*/, '').slice(0, 50) || note.title;
    note.wordCount = countWords(content);
  } catch (e) {
    console.error('Failed to read note:', e);
  }
}

function debouncedSync() {
  if (autoSaveTimer.value) clearTimeout(autoSaveTimer.value);
  autoSaveTimer.value = setTimeout(async () => {
    const noteId = activeNoteId.value;
    if (!noteId) return;
    // Look up current state at fire time, not capture time
    const note = notes.value.find(n => n.id === noteId);
    if (!note?.path) return;
    const content = currentContent.value;

    // 1. Save content to current path
    try {
      await invoke('save_file', { path: note.path, content });
    } catch (e) {
      console.error('Auto-save failed:', e);
      return;
    }

    // 2. Check if heading ≠ filename → rename
    const firstLine = content.split('\n')[0].trim();
    if (!firstLine.startsWith('# ')) return;
    const headingTitle = firstLine.slice(2).trim();
    const fileStem = note.path.replace(/\\/g, '/').split('/').pop()?.replace(/\.md$/, '') || '';
    if (!headingTitle || headingTitle === fileStem) return;

    try {
      const result = await invoke<{ path: string; name: string }>('rename_note', { oldPath: note.path, newName: headingTitle });
      note.path = result.path;
      note.title = result.name;
      note.id = result.path;
      if (activeNoteId.value === noteId) {
        activeNoteId.value = result.path;
      }
      if (vaultPath.value) await loadVaultTree(vaultPath.value);
    } catch (e) {
      console.error('Auto-rename failed:', e);
    }
  }, 500);
}

function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).slice(2, 8);
}

function countWords(text: string): number {
  // Count CJK characters and words separately
  const cjk = (text.match(/[一-鿿]/g) || []).length;
  const others = text.trim().split(/\s+/).filter(w => w.length > 0 && !/[一-鿿]/.test(w)).length;
  return cjk + others;
}

const settings = reactive<Settings>(loadSettings());
const showSettings = ref(false);

watch(
  () => ({ ...settings }),
  (v) => localStorage.setItem(SETTINGS_KEY, JSON.stringify(v)),
);

const notes = ref<Note[]>(loadNotes());
const activeNoteId = ref<string | null>(null);
const currentFilePath = ref<string | null>(null);
const currentContent = ref('');
const showPreview = ref(false);
const splitView = ref(false);
const autoSaveTimer = ref<ReturnType<typeof setTimeout> | null>(null);
const vaultPath = computed(() => settings.vaultPath);
const pendingDeleteId = ref<string | null>(null);

const activeNote = computed(() => notes.value.find((n) => n.id === activeNoteId.value));
const pendingDeleteNote = computed(() => notes.value.find((n) => n.id === pendingDeleteId.value));

watch(notes, (v) => {
  if (!vaultPath.value) saveNotes(v);
}, { deep: true });

watch(vaultPath, async (v) => {
  activeNoteId.value = null;
  currentContent.value = '';
  currentFilePath.value = null;
  if (v) {
    await loadVaultTree(v);
  } else {
    notes.value = loadNotes();
  }
}, { immediate: true });

async function handleNewNote(subdir?: string) {
  if (vaultPath.value) {
    const args: Record<string, unknown> = { dir: vaultPath.value };
    if (subdir) args.subdir = subdir;
    const file = await invoke<{ path: string; name: string; updated_at: number }>('create_note', args);
    const note: Note = {
      id: file.path,
      title: file.name,
      content: '',
      updatedAt: new Date(file.updated_at * 1000).toISOString(),
      wordCount: 0,
      path: file.path,
    };
    notes.value.unshift(note);
    activeNoteId.value = note.id;
    currentContent.value = '';
    currentFilePath.value = null;
    await loadVaultTree(vaultPath.value);
  } else {
    const note: Note = {
      id: generateId(),
      title: '',
      content: '',
      updatedAt: new Date().toISOString(),
      wordCount: 0,
    };
    notes.value.unshift(note);
    activeNoteId.value = note.id;
    currentContent.value = '';
    currentFilePath.value = null;
  }
}

async function handleSelectNote(id: string) {
  activeNoteId.value = id;
  let note = notes.value.find((n) => n.id === id);
  if (!note) {
    // Note not yet loaded — create entry and load content
    note = {
      id,
      title: id.split(/[\\/]/).pop()?.replace(/\.md$/, '') || '',
      content: '',
      updatedAt: new Date().toISOString(),
      wordCount: 0,
      path: id,
    };
    notes.value.unshift(note);
  }
  if (note.path) {
    await loadNoteContent(note);
  } else {
    currentContent.value = note.content;
  }
  currentFilePath.value = note.path || null;
}

function handleContentUpdate(content: string) {
  currentContent.value = content;
  if (activeNoteId.value) {
    const note = notes.value.find((n) => n.id === activeNoteId.value);
    if (note) {
      note.content = content;
      note.updatedAt = new Date().toISOString();
      const firstLine = content.split('\n')[0].trim();
      note.title = firstLine.replace(/^#+\s*/, '').slice(0, 50) || '无标题';
      note.wordCount = countWords(content);
      if (note.path) {
        debouncedSync();
      }
    }
  }
}

function handleDeleteNote(id: string) {
  pendingDeleteId.value = id;
}

async function confirmDelete() {
  const id = pendingDeleteId.value;
  if (!id) return;
  const note = notes.value.find((n) => n.id === id);
  if (note?.path) {
    try {
      await invoke('delete_note', { path: note.path });
    } catch (e) {
      console.error('Delete failed:', e);
    }
  }
  notes.value = notes.value.filter((n) => n.id !== id);
  if (activeNoteId.value === id) {
    activeNoteId.value = null;
    currentContent.value = '';
    currentFilePath.value = null;
  }
  pendingDeleteId.value = null;
  if (vaultPath.value) await loadVaultTree(vaultPath.value);
}

function cancelDelete() {
  pendingDeleteId.value = null;
}

async function handleRenameNote(id: string, newName: string) {
  const note = notes.value.find((n) => n.id === id);
  if (!note?.path) return;
  try {
    const result = await invoke<{ path: string; name: string; updated_at: number }>('rename_note', {
      oldPath: note.path,
      newName,
    });
    note.path = result.path;
    note.title = result.name;
    note.updatedAt = new Date(result.updated_at * 1000).toISOString();
    const oldId = note.id;
    note.id = result.path;
    if (activeNoteId.value === oldId) {
      activeNoteId.value = result.path;
      // Sync editor content: update the first heading to match
      const newHeading = `# ${newName}`;
      const content = currentContent.value;
      const firstLineEnd = content.indexOf('\n');
      if (firstLineEnd === -1) {
        currentContent.value = newHeading;
      } else if (content.startsWith('# ')) {
        currentContent.value = newHeading + content.slice(firstLineEnd);
      } else {
        currentContent.value = `${newHeading}\n\n${content}`;
      }
    }
  } catch (e) {
    console.error('Rename failed:', e);
    alert(`重命名失败：${e}`);
  }
  if (vaultPath.value) await loadVaultTree(vaultPath.value);
}

async function handleNewFolder() {
  if (!vaultPath.value) return;
  const name = prompt('新建文件夹名称：');
  if (!name || !name.trim()) return;
  try {
    await invoke('create_folder', { dir: vaultPath.value, name: name.trim() });
    await loadVaultTree(vaultPath.value);
  } catch (e) {
    console.error('Create folder failed:', e);
    alert(`创建文件夹失败：${e}`);
  }
}

async function handleNewNoteInDir(dirPath: string) {
  await handleNewNote(dirPath);
}

async function handleNewFolderInDir(dirPath: string) {
  const name = prompt('新建文件夹名称：');
  if (!name || !name.trim()) return;
  try {
    await invoke('create_folder', { dir: dirPath, name: name.trim() });
    if (vaultPath.value) await loadVaultTree(vaultPath.value);
  } catch (e) {
    console.error('Create folder failed:', e);
    alert(`创建文件夹失败：${e}`);
  }
}

async function handleDeleteFolder(path: string) {
  if (!confirm('确定要删除此文件夹及其所有内容吗？此操作不可撤销。')) return;
  try {
    await invoke('delete_folder', { path });
    if (vaultPath.value) await loadVaultTree(vaultPath.value);
    // If active note was in deleted folder, clear it
    if (activeNoteId.value?.startsWith(path)) {
      activeNoteId.value = null;
      currentContent.value = '';
      currentFilePath.value = null;
    }
  } catch (e) {
    console.error('Delete folder failed:', e);
    alert(`删除文件夹失败：${e}`);
  }
}

async function handleRenameFolder(path: string, newName: string) {
  try {
    await invoke('rename_folder', { oldPath: path, newName });
    if (vaultPath.value) await loadVaultTree(vaultPath.value);
  } catch (e) {
    console.error('Rename folder failed:', e);
    alert(`重命名文件夹失败：${e}`);
  }
}

async function handleImportNote(subdir?: string) {
  if (!vaultPath.value) return;
  const selected = await open({
    multiple: true,
    filters: [{ name: 'Markdown', extensions: ['md', 'txt', 'markdown'] }],
  });
  if (!selected) return;
  const files = Array.isArray(selected) ? selected : [selected];
  for (const file of files) {
    if (typeof file !== 'string') continue;
    try {
      await invoke('import_note', { vaultDir: vaultPath.value, sourcePath: file, subdir: subdir ?? null });
    } catch (e) {
      console.error('Import failed:', e);
    }
  }
  await loadVaultTree(vaultPath.value);
}

function handleFileNew() {
  currentFilePath.value = null;
  currentContent.value = '';
}

async function handleFileOpen() {
  try {
    const result = await invoke<FileData>('open_file');
    currentFilePath.value = result.path;
    currentContent.value = result.content;
  } catch (e) {
    if (e !== 'cancelled') console.error(e);
  }
}

async function handleFileSave() {
  if (!currentFilePath.value) {
    return handleFileSaveAs();
  }
  try {
    await invoke('save_file', {
      path: currentFilePath.value,
      content: currentContent.value,
    });
  } catch (e) {
    console.error('Save failed:', e);
  }
}

async function handleFileSaveAs() {
  try {
    const newPath = await invoke<string>('save_file_as', {
      content: currentContent.value,
    });
    currentFilePath.value = newPath;
  } catch (e) {
    if (e !== 'cancelled') console.error(e);
  }
}

function handleSetMode(mode: 'edit' | 'split' | 'preview') {
  switch (mode) {
    case 'edit':
      showPreview.value = false;
      splitView.value = false;
      break;
    case 'split':
      splitView.value = true;
      showPreview.value = false;
      break;
    case 'preview':
      showPreview.value = true;
      splitView.value = false;
      break;
  }
}

function handleSettingsSave(s: Settings) {
  settings.background = s.background;
  settings.backgroundOpacity = s.backgroundOpacity;
  settings.backgroundBlur = s.backgroundBlur;
  settings.backgroundSize = s.backgroundSize;
  settings.backgroundPositionX = s.backgroundPositionX;
  settings.backgroundPositionY = s.backgroundPositionY;
  settings.sidebarBackground = s.sidebarBackground;
  settings.sidebarBackgroundOpacity = s.sidebarBackgroundOpacity;
  settings.sidebarBackgroundBlur = s.sidebarBackgroundBlur;
  settings.sidebarBackgroundSize = s.sidebarBackgroundSize;
  settings.sidebarBackgroundPositionX = s.sidebarBackgroundPositionX;
  settings.sidebarBackgroundPositionY = s.sidebarBackgroundPositionY;
  settings.defaultSavePath = s.defaultSavePath;
  settings.vaultPath = s.vaultPath;
}

function handleKeydown(e: KeyboardEvent) {
  const mod = e.ctrlKey || e.metaKey;
  if (!mod) return;

  switch (e.key) {
    case 'n':
      e.preventDefault();
      handleFileNew();
      break;
    case 'o':
      e.preventDefault();
      handleFileOpen();
      break;
    case 's':
      e.preventDefault();
      if (e.shiftKey) {
        handleFileSaveAs();
      } else {
        handleFileSave();
      }
      break;
  }
}

onMounted(() => window.addEventListener('keydown', handleKeydown, true));
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown, true);
  if (autoSaveTimer.value) clearTimeout(autoSaveTimer.value);
});
</script>

<template>
  <div class="app-layout">
    <MilkdownProvider>
      <Sidebar
        :notes="notes"
        :active-id="activeNoteId"
        :vault-path="vaultPath"
        :vault-tree="vaultTree"
        :background="settings.sidebarBackground"
        :background-opacity="settings.sidebarBackgroundOpacity"
        :background-blur="settings.sidebarBackgroundBlur"
        :background-size="settings.sidebarBackgroundSize"
        :background-position-x="settings.sidebarBackgroundPositionX"
        :background-position-y="settings.sidebarBackgroundPositionY"
        @new-note="handleNewNote()"
        @new-folder="handleNewFolder"
        @new-note-in-dir="handleNewNoteInDir"
        @new-folder-in-dir="handleNewFolderInDir"
        @select-note="handleSelectNote"
        @delete-note="handleDeleteNote"
        @delete-folder="handleDeleteFolder"
        @import-note="handleImportNote()"
        @rename-note="handleRenameNote"
        @rename-folder="handleRenameFolder"
      />
      <div class="main-area">
        <TitleBar
          :file-path="currentFilePath"
          :note-title="activeNote?.title"
          :note-date="activeNote ? new Date(activeNote.updatedAt).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' }) : undefined"
          :note-words="activeNote?.wordCount"
          :saved="true"
          @open-settings="showSettings = true"
        />
        <Toolbar
          :show-preview="showPreview"
          :split-view="splitView"
          @file-new="handleFileNew"
          @file-open="handleFileOpen"
          @file-save="handleFileSave"
          @file-save-as="handleFileSaveAs"
          @set-mode="handleSetMode"
        />
        <EditorArea
          :content="currentContent"
          :show-preview="showPreview"
          :split-view="splitView"
          :background="settings.background"
          :background-opacity="settings.backgroundOpacity"
          :background-blur="settings.backgroundBlur"
          :background-size="settings.backgroundSize"
          :background-position-x="settings.backgroundPositionX"
          :background-position-y="settings.backgroundPositionY"
          @update:content="handleContentUpdate"
        />
      </div>
    </MilkdownProvider>
    <SettingsModal
      v-model="showSettings"
      :settings="settings"
      @save="handleSettingsSave"
    />

    <div v-if="pendingDeleteId" class="confirm-overlay" @click.self="cancelDelete">
      <div class="confirm-card">
        <p class="confirm-text">确定要删除「{{ pendingDeleteNote?.title || '无标题' }}」吗？</p>
        <p class="confirm-hint">此操作不可撤销，文件将被永久删除。</p>
        <div class="confirm-actions">
          <button class="confirm-btn confirm-btn--cancel" @click="cancelDelete">取消</button>
          <button class="confirm-btn confirm-btn--danger" @click="confirmDelete">删除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  flex-direction: row;
  height: 100vh;
  overflow: hidden;
}

.main-area {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

.confirm-overlay {
  position: fixed;
  inset: 0;
  background: var(--modal-overlay-bg, rgba(0, 0, 0, 0.4));
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1100;
}

.confirm-card {
  width: 360px;
  background: var(--bg-color, #1e1e1e);
  border: 1px solid var(--border-color, #3c3c3c);
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}

.confirm-text {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-color, #d4d4d4);
  margin-bottom: 8px;
}

.confirm-hint {
  font-size: 12px;
  color: var(--text-muted, #888);
  margin-bottom: 20px;
}

.confirm-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.confirm-btn {
  padding: 6px 20px;
  border: 1px solid var(--border-color, #3c3c3c);
  border-radius: 4px;
  background: transparent;
  color: var(--text-color, #d4d4d4);
  font-size: 13px;
  cursor: pointer;
  transition: background 0.15s;
}

.confirm-btn:hover {
  background: var(--btn-hover, #3c3c3c);
}

.confirm-btn--danger {
  background: #d94a4a;
  border-color: #d94a4a;
  color: #fff;
}

.confirm-btn--danger:hover {
  background: #c23a3a;
}
</style>
