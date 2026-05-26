# 初笺

轻量级桌面 Markdown 编辑器，基于 Tauri v2 + Vue 3 + Milkdown 构建。

## 特性

- **WYSIWYG 编辑** — 基于 Milkdown 的所见即所得 Markdown 编辑，支持 CommonMark + GFM
- **笔记库（Vault）** — 指定本地文件夹作为笔记库，树形浏览子目录与笔记
- **首行即标题** — 每篇笔记第一行强制为一级标题，与文件名双向同步
- **三种视图** — 编辑 / 分栏预览 / 纯预览，一键切换
- **自定义背景** — 编辑区和侧边栏支持图片/颜色背景，可调透明度、模糊和位置
- **自动保存** — 500ms 防抖自动保存，重命名时同步更新文件内标题
- **导入导出** — 支持导入外部 `.md` 文件到笔记库
- **系统托盘** — 关闭窗口后驻留托盘（规划中）

## 技术栈

| 层 | 技术 |
|---|---|
| 桌面框架 | Tauri v2 |
| 前端 | Vue 3 + TypeScript |
| 编辑器 | Milkdown v7 |
| 样式 | CSS 变量 + 亮/暗主题 |
| 后端 | Rust |

## 开发

```bash
# 安装依赖
npm install

# 启动开发环境（Vite 前端 + Tauri 桌面窗口）
npm run tauri dev

# 仅启动前端（浏览器开发，部分功能受限）
npm run dev

# 类型检查 + 构建前端
npm run build

# 构建生产安装包
npm run tauri build
```

## 项目结构

```
src/
├── App.vue                    # 主布局 + 状态管理 + 快捷键
├── main.ts                    # 入口
├── components/
│   ├── Sidebar.vue            # 侧边栏：搜索、新建、树形浏览、右键菜单
│   ├── SidebarNoteTree.vue    # 递归树组件（文件夹折叠 + 笔记项）
│   ├── TitleBar.vue           # 标题栏（文件名、字数、保存状态）
│   ├── Toolbar.vue            # 工具栏（新建/打开/保存 + 模式切换）
│   ├── EditorArea.vue         # 编辑区容器（单窗/分栏）
│   ├── CodeEditor.vue         # Milkdown WYSIWYG 编辑器
│   └── SettingsModal.vue      # 设置弹窗
├── styles/
│   └── global.css             # 全局样式 + CSS 变量 + 暗色主题
src-tauri/
├── src/
│   ├── main.rs                # Rust 入口
│   └── lib.rs                 # Tauri 命令（14 个文件操作 API）
└── capabilities/
    └── default.json           # 权限配置
```
