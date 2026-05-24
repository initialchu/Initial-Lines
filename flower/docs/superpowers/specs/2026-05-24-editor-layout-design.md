# 编辑器三栏布局设计（雾感粉蓝、方案 2）

概述
- 目标：为现有前端页面实现“雾感粉蓝”主题的三栏编辑器布局，采用固定列宽并支持折叠侧栏/预览（方案 2），兼顾窄屏响应式行为。
- 范围：仅涉及前端 UI、样式与交互，不包含后端变更或数据模型调整。
- 成果：可复用的 `MainLayout` 组件、`SidebarNotes`、`EditorArea`、`MarkdownPreview`、折叠控制与主题变量。

设计目标与约束
- 桌面优先：三栏布局（左：笔记列表 240px；中：编辑区 1fr；右：预览 420px）。
- 不提供列拖拽；提供折叠/展开功能（左/右可单独折叠）。
- 主题：雾感粉蓝（深色面板 + 柔和粉蓝/粉色雾光）
- 可访问性：折叠控件可通过键盘激活，语义化 DOM，满足基本对比度与焦点指示。
- 实现应尊重现有项目框架（Vue 3 + Vite + CodeMirror + markdown-it）。

组件分解
- `MainLayout` (新)：负责 Grid 布局、折叠状态管理、响应式断点。Props/State: `leftCollapsed`, `rightCollapsed`。
- `SidebarNotes` (现有或改造)：左侧笔记列表，height: 100%。支持折叠/展开动画。
- `EditorArea` (`src/components/EditorArea.vue`)：封装 CodeMirror，暴露 `value`、`onChange`、`onScroll` 事件。
- `MarkdownPreview` (`src/components/MarkdownPreview.vue`)：渲染 HTML（`markdown-it` + `highlight.js`），支持接收 `html` 与 `syncScrollPosition`。
- `TitleBar` / `Toolbar`：现有组件延伸，增加折叠按钮与主题切换入口。

交互细节
- 折叠/展开：左/右各有按钮（图标 + aria-label），Fold 动画 180ms ease。折叠状态保存在 `localStorage`：`flower.ui.leftCollapsed`, `flower.ui.rightCollapsed`。
- 折叠快捷键：`Ctrl+B` 折叠/展开侧边栏，`Ctrl+Shift+P` 折叠/展开预览（可配置）。
- 编辑-预览同步（非滚动同步，仅实时渲染）：编辑器变更触发即时渲染到 `MarkdownPreview`（节流 150ms）。后续可选启用滚动同步作为增强。

视觉与主题变量（主要色 token）
- 全局 CSS 变量（放在 `src/styles/global.css` 或 `:root`）

```
:root{
  --bg: #071022; /* 深蓝灰 背景 */
  --panel-bg: rgba(255,255,255,0.02); /* 面板半透明 */
  --mist-pink: #F7E7F0; /* 雾感粉 */
  --mist-blue-start: #CDE7F9;
  --mist-blue-end: #EADFF7;
  --muted: #98A3B3;
  --text: #E6EEF6;
  --accent: #F6A6D9; /* 用于高亮与按钮 */
  --sidebar-w: 240px;
  --preview-w: 420px;
  --radius: 12px;
}
```

样式要点
- 布局使用 CSS Grid：`grid-template-columns: var(--sidebar-w) 1fr var(--preview-w);`。当折叠某列时，把对应列宽设为 `0` 并隐藏 overflow。
- 使用 `backdrop-filter: blur(6px)` 与轻微噪点（背景图或 CSS 生成）构建雾感效果。
- 卡片/面板使用柔和内阴影和发光：`box-shadow: 0 6px 30px rgba(173,160,220,0.06)`。
- 文本与代码块使用高对比度配色，代码高亮使用 `highlight.js` 的暗色主题并微调使之与粉色色调匹配。

关键 CSS 片段（供实现时复制）

```
.main-grid{
  display: grid;
  grid-template-columns: var(--sidebar-w) 1fr var(--preview-w);
  gap: 12px;
  height: calc(100vh - var(--titlebar-height));
}
.pane{
  background: linear-gradient(180deg, rgba(255,255,255,0.01), rgba(255,255,255,0.02));
  border-radius: var(--radius);
  overflow: hidden;
  display:flex;
  flex-direction:column;
}
.collapsed{ grid-column: 1 / span 0; width:0; padding:0; visibility:hidden; }
.fold-button{ background:transparent; border-radius:8px; padding:6px; }
.fold-button:focus{ outline: 2px solid rgba(246,166,217,0.4); }
```

实现步骤（按优先级）
1. 在 `src/components/` 新建 `MainLayout.vue`，实现 Grid 布局与折叠状态管理，导出 `leftCollapsed`、`rightCollapsed` 状态。将 `App.vue` 调整为使用 `MainLayout`。
2. 调整 `SidebarNotes` 与 `MarkdownPreview` 使其高度 100% 并支持 `.collapsed` 类。
3. 在 `EditorArea` 中确保 CodeMirror 高度自适应并暴露 `onScroll`/`onChange` 事件。
4. 将主题变量加入 `src/styles/global.css` 并替换局部硬编码颜色。
5. 添加折叠控件到 `TitleBar` 或 `Toolbar`，实现 `localStorage` 保存与快捷键。
6. 测试主要断点（>=1200px 桌面，768–1199 平板，<768 手机），验证折叠/切换与键盘可达性。

文件修改清单（建议）
- `src/App.vue` → 包装 `MainLayout`
- 新文件：`src/components/MainLayout.vue`
- `src/components/SidebarNotes.vue` → 支持折叠类
- `src/components/EditorArea.vue` → 调整高度/事件
- `src/components/MarkdownPreview.vue` → 调整高度/样式
- `src/styles/global.css` → 添加主题 token
- 可选：`src/components/TitleBar.vue` / `Toolbar.vue` → 添加折叠按钮

测试与验收标准
- 桌面打开时看到三栏，左 240px，右 420px，中间自适应。
- 点击折叠按钮左侧收起，右侧收起并且中间扩展为 1fr。
- 折叠状态在刷新后保持（localStorage）。
- 编辑时右侧能实时渲染 Markdown（变更节流后低延迟）。
- 深色雾感主题视觉与截图风格接近，按钮和焦点对比满足可访问性。

风险与备选项
- 如果对性能有顾虑（大文档实时渲染），可在后续加“延迟渲染”选项（如切换到“仅保存渲染”或增加 300ms 节流）。
- 若后续需要列宽可调，可基于此框架加入分割条组件或引入 `split.js`。

后续步骤（我建议）
1. 若你通过此设计，我将把此文件提交（已完成）。
2. 接着我会生成实现计划（写入 TODO 并开始实现 `MainLayout.vue`）。

---
作者：AI 设计助手
路径：`docs/superpowers/specs/2026-05-24-editor-layout-design.md`
日期：2026-05-24
