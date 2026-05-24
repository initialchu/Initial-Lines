<script setup lang="ts">
import { reactive, watch } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';

export interface Settings {
  background: string;
  defaultSavePath: string;
  vaultPath: string;
}

const props = defineProps<{
  modelValue: boolean;
  settings: Settings;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  save: [settings: Settings];
}>();

const form = reactive<Settings>({ ...props.settings });

watch(
  () => props.modelValue,
  (v) => {
    if (v) {
      form.background = props.settings.background;
      form.defaultSavePath = props.settings.defaultSavePath;
      form.vaultPath = props.settings.vaultPath;
    }
  },
);

async function handleBrowseVault() {
  const selected = await open({ directory: true, multiple: false });
  if (selected && typeof selected === 'string') {
    form.vaultPath = selected;
  }
}

async function handleBrowsePath() {
  const selected = await open({ directory: true, multiple: false });
  if (selected && typeof selected === 'string') {
    form.defaultSavePath = selected;
  }
}

function handleSave() {
  emit('save', { ...form });
  emit('update:modelValue', false);
}

function handleClose() {
  emit('update:modelValue', false);
}

function handleOverlayClick(e: MouseEvent) {
  if ((e.target as HTMLElement).classList.contains('settings-overlay')) {
    handleClose();
  }
}
</script>

<template>
  <div v-if="modelValue" class="settings-overlay" @click="handleOverlayClick">
    <div class="settings-card">
      <div class="settings-header">
        <h2>设置</h2>
        <button class="settings-close-btn" @click="handleClose">&times;</button>
      </div>

      <div class="settings-body">
        <label class="settings-field">
          <span class="settings-label">自定义背景</span>
          <input
            v-model="form.background"
            class="settings-input"
            placeholder="例如: #f5f5f5 或 linear-gradient(135deg, #667eea, #764ba2)"
          />
        </label>

        <label class="settings-field">
          <span class="settings-label">笔记库路径</span>
          <div class="settings-path-row">
            <input
              v-model="form.vaultPath"
              class="settings-input"
              placeholder="选择笔记库文件夹..."
            />
            <button class="settings-browse-btn" @click="handleBrowseVault">浏览</button>
          </div>
        </label>

        <label class="settings-field">
          <span class="settings-label">默认保存路径</span>
          <div class="settings-path-row">
            <input
              v-model="form.defaultSavePath"
              class="settings-input"
              placeholder="选择默认保存文件夹..."
            />
            <button class="settings-browse-btn" @click="handleBrowsePath">浏览</button>
          </div>
        </label>
      </div>

      <div class="settings-footer">
        <button class="settings-save-btn" @click="handleSave">确定</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-overlay {
  position: fixed;
  inset: 0;
  background: var(--modal-overlay-bg, rgba(0, 0, 0, 0.4));
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.settings-card {
  width: 480px;
  max-height: 80vh;
  background: var(--bg-color);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  display: flex;
  flex-direction: column;
}

.settings-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.settings-header h2 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-color);
}

.settings-close-btn {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-muted);
  font-size: 20px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.settings-close-btn:hover {
  background: var(--btn-hover);
  color: var(--text-color);
}

.settings-body {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.settings-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.settings-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-color);
}

.settings-input {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-color);
  color: var(--text-color);
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s;
}

.settings-input:focus {
  border-color: var(--accent-color);
}

.settings-input::placeholder {
  color: var(--text-muted);
}

.settings-path-row {
  display: flex;
  gap: 8px;
}

.settings-path-row .settings-input {
  flex: 1;
}

.settings-browse-btn {
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--btn-bg, transparent);
  color: var(--text-color);
  font-size: 13px;
  cursor: pointer;
  white-space: nowrap;
}

.settings-browse-btn:hover {
  background: var(--btn-hover);
}

.settings-footer {
  display: flex;
  justify-content: flex-end;
  padding: 12px 20px;
  border-top: 1px solid var(--border-color);
}

.settings-save-btn {
  padding: 6px 24px;
  border: none;
  border-radius: 4px;
  background: var(--accent-color);
  color: #fff;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
}

.settings-save-btn:hover {
  opacity: 0.9;
}
</style>
