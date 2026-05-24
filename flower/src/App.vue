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
  return { background: '', defaultSavePath: '' };
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

const activeNote = computed(() => notes.value.find((n) => n.id === activeNoteId.value));

watch(notes, (v) => saveNotes(v), { deep: true });

function handleNewNote() {
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

function handleSelectNote(id: string) {
  activeNoteId.value = id;
  const note = notes.value.find((n) => n.id === id);
  if (note) {
    currentContent.value = note.content;
  }
  currentFilePath.value = null;
}

function handleContentUpdate(content: string) {
  currentContent.value = content;
  if (activeNoteId.value) {
    const note = notes.value.find((n) => n.id === activeNoteId.value);
    if (note) {
      note.content = content;
      note.updatedAt = new Date().toISOString();
      // Extract title from first line
      const firstLine = content.split('\n')[0].trim();
      note.title = firstLine.replace(/^#+\s*/, '').slice(0, 50) || '无标题';
      note.wordCount = countWords(content);
    }
  }
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

function handleTogglePreview() {
  showPreview.value = !showPreview.value;
}

function handleToggleSplit() {
  splitView.value = !splitView.value;
}

function handleSettingsSave(s: Settings) {
  settings.background = s.background;
  settings.defaultSavePath = s.defaultSavePath;
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
onUnmounted(() => window.removeEventListener('keydown', handleKeydown, true));
</script>

<template>
  <div class="app-layout">
    <MilkdownProvider>
      <Sidebar
        :notes="notes"
        :active-id="activeNoteId"
        @new-note="handleNewNote"
        @select-note="handleSelectNote"
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
          @toggle-preview="handleTogglePreview"
          @toggle-split="handleToggleSplit"
        />
        <EditorArea
          :content="currentContent"
          :show-preview="showPreview"
          :split-view="splitView"
          :background="settings.background"
          @update:content="handleContentUpdate"
        />
      </div>
    </MilkdownProvider>
    <SettingsModal
      v-model="showSettings"
      :settings="settings"
      @save="handleSettingsSave"
    />
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
</style>
