<script setup lang="ts">
import { reactive, watch } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

export interface Settings {
  background: string;
  backgroundOpacity: number;
  backgroundBlur: number;
  backgroundSize: string;
  backgroundPositionX: number;
  backgroundPositionY: number;
  sidebarBackground: string;
  sidebarBackgroundOpacity: number;
  sidebarBackgroundBlur: number;
  sidebarBackgroundSize: string;
  sidebarBackgroundPositionX: number;
  sidebarBackgroundPositionY: number;
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
      form.backgroundOpacity = props.settings.backgroundOpacity;
      form.backgroundBlur = props.settings.backgroundBlur;
      form.backgroundSize = props.settings.backgroundSize;
      form.backgroundPositionX = props.settings.backgroundPositionX;
      form.backgroundPositionY = props.settings.backgroundPositionY;
      form.sidebarBackground = props.settings.sidebarBackground;
      form.sidebarBackgroundOpacity = props.settings.sidebarBackgroundOpacity;
      form.sidebarBackgroundBlur = props.settings.sidebarBackgroundBlur;
      form.sidebarBackgroundSize = props.settings.sidebarBackgroundSize;
      form.sidebarBackgroundPositionX = props.settings.sidebarBackgroundPositionX;
      form.sidebarBackgroundPositionY = props.settings.sidebarBackgroundPositionY;
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

async function handlePickSidebarImage() {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp', 'svg'] }],
  });
  if (selected && typeof selected === 'string') {
    const dataUrl = await invoke<string>('read_image_base64', { path: selected });
    form.sidebarBackground = `url(${dataUrl}) center/cover no-repeat`;
  }
}

async function handlePickEditorImage() {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp', 'svg'] }],
  });
  if (selected && typeof selected === 'string') {
    const dataUrl = await invoke<string>('read_image_base64', { path: selected });
    form.background = `url(${dataUrl}) center/cover no-repeat`;
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
          <span class="settings-label">编辑区背景</span>
          <div class="settings-bg-row">
            <input
              v-model="form.background"
              class="settings-input"
              placeholder="CSS 颜色、渐变或图片 URL..."
            />
            <input
              type="color"
              class="settings-color-picker"
              title="选择颜色"
              @input="form.background = ($event.target as HTMLInputElement).value"
            />
            <button class="settings-browse-btn" title="选择图片" @click="handlePickEditorImage">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                <circle cx="8.5" cy="8.5" r="1.5" />
                <polyline points="21 15 16 10 5 21" />
              </svg>
            </button>
          </div>
          <div class="settings-bg-ctrls">
            <label class="settings-ctrl">
              <span>透明度</span>
              <input type="range" min="0" max="100" v-model.number="form.backgroundOpacity" />
              <span class="settings-ctrl-val">{{ form.backgroundOpacity }}%</span>
            </label>
            <label class="settings-ctrl">
              <span>模糊</span>
              <input type="range" min="0" max="50" v-model.number="form.backgroundBlur" />
              <span class="settings-ctrl-val">{{ form.backgroundBlur }}px</span>
            </label>
            <label class="settings-ctrl">
              <span>尺寸</span>
              <select v-model="form.backgroundSize" class="settings-select">
                <option value="cover">Cover</option>
                <option value="contain">Contain</option>
                <option value="auto">Auto</option>
                <option value="100% auto">100%</option>
              </select>
            </label>
          </div>
          <div class="settings-bg-ctrls">
            <label class="settings-ctrl">
              <span>水平</span>
              <input type="range" min="0" max="100" v-model.number="form.backgroundPositionX" />
              <span class="settings-ctrl-val">{{ form.backgroundPositionX }}%</span>
            </label>
            <label class="settings-ctrl">
              <span>垂直</span>
              <input type="range" min="0" max="100" v-model.number="form.backgroundPositionY" />
              <span class="settings-ctrl-val">{{ form.backgroundPositionY }}%</span>
            </label>
          </div>
        </label>

        <label class="settings-field">
          <span class="settings-label">侧边栏背景</span>
          <div class="settings-bg-row">
            <input
              v-model="form.sidebarBackground"
              class="settings-input"
              placeholder="CSS 颜色、渐变或图片 URL..."
            />
            <input
              type="color"
              class="settings-color-picker"
              title="选择颜色"
              @input="form.sidebarBackground = ($event.target as HTMLInputElement).value"
            />
            <button class="settings-browse-btn" title="选择图片" @click="handlePickSidebarImage">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
                <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                <circle cx="8.5" cy="8.5" r="1.5" />
                <polyline points="21 15 16 10 5 21" />
              </svg>
            </button>
          </div>
          <div class="settings-bg-ctrls">
            <label class="settings-ctrl">
              <span>透明度</span>
              <input type="range" min="0" max="100" v-model.number="form.sidebarBackgroundOpacity" />
              <span class="settings-ctrl-val">{{ form.sidebarBackgroundOpacity }}%</span>
            </label>
            <label class="settings-ctrl">
              <span>模糊</span>
              <input type="range" min="0" max="50" v-model.number="form.sidebarBackgroundBlur" />
              <span class="settings-ctrl-val">{{ form.sidebarBackgroundBlur }}px</span>
            </label>
            <label class="settings-ctrl">
              <span>尺寸</span>
              <select v-model="form.sidebarBackgroundSize" class="settings-select">
                <option value="cover">Cover</option>
                <option value="contain">Contain</option>
                <option value="auto">Auto</option>
                <option value="100% auto">100%</option>
              </select>
            </label>
          </div>
          <div class="settings-bg-ctrls">
            <label class="settings-ctrl">
              <span>水平</span>
              <input type="range" min="0" max="100" v-model.number="form.sidebarBackgroundPositionX" />
              <span class="settings-ctrl-val">{{ form.sidebarBackgroundPositionX }}%</span>
            </label>
            <label class="settings-ctrl">
              <span>垂直</span>
              <input type="range" min="0" max="100" v-model.number="form.sidebarBackgroundPositionY" />
              <span class="settings-ctrl-val">{{ form.sidebarBackgroundPositionY }}%</span>
            </label>
          </div>
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

.settings-bg-row {
  display: flex;
  gap: 8px;
}

.settings-bg-row .settings-input {
  flex: 1;
}

.settings-color-picker {
  width: 36px;
  height: 36px;
  padding: 2px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  background: transparent;
  flex-shrink: 0;
}

.settings-color-picker::-webkit-color-swatch-wrapper {
  padding: 0;
}

.settings-color-picker::-webkit-color-swatch {
  border: none;
  border-radius: 2px;
}

.settings-bg-ctrls {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
}

.settings-ctrl {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--text-muted, #888);
  cursor: default;
}

.settings-ctrl input[type="range"] {
  width: 80px;
  height: 4px;
  accent-color: var(--accent-color, #4a90d9);
  cursor: pointer;
}

.settings-ctrl-val {
  min-width: 32px;
  text-align: right;
  color: var(--text-color, #d4d4d4);
  font-size: 11px;
}

.settings-select {
  padding: 2px 6px;
  border: 1px solid var(--border-color, #3c3c3c);
  border-radius: 4px;
  background: var(--bg-color, #1e1e1e);
  color: var(--text-color, #d4d4d4);
  font-size: 12px;
  outline: none;
  cursor: pointer;
}

.settings-select:focus {
  border-color: var(--accent-color, #4a90d9);
}
</style>
