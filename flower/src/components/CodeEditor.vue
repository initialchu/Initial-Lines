<script setup lang="ts">
import { watch } from 'vue';
import { Editor, rootCtx, defaultValueCtx, editorViewCtx } from '@milkdown/kit/core';
import { commonmark } from '@milkdown/kit/preset/commonmark';
import { gfm } from '@milkdown/kit/preset/gfm';
import { history } from '@milkdown/kit/plugin/history';
import { listener, listenerCtx } from '@milkdown/kit/plugin/listener';
import { nord } from '@milkdown/theme-nord';
import { getMarkdown, replaceAll } from '@milkdown/kit/utils';
import { Milkdown, useEditor } from '@milkdown/vue';
import { Plugin } from 'prosemirror-state';
import { $prose } from '@milkdown/utils';

const props = defineProps<{ content: string; showPreview: boolean }>();
const emit = defineEmits<{
  'update:content': [value: string];
  'preview-html': [html: string];
  'title-required': [];
}>();

const nordTheme = nord as any;

let onTitleRequired = () => {};

const firstLineHeading = $prose(() =>
  new Plugin({
    props: {
      handleKeyDown(view, event) {
        if (event.key === 'Enter') {
          const firstNode = view.state.doc.firstChild;
          if (!firstNode || firstNode.type.name !== 'heading') return false;
          const { $from } = view.state.selection;
          if ($from.pos > firstNode.nodeSize) return false;
          if (firstNode.textContent.trim() === '') {
            event.preventDefault();
            onTitleRequired();
            return true;
          }
        }
        return false;
      },
    },
    appendTransaction(_transactions, _oldState, newState) {
      const { doc, schema } = newState;
      const headingType = schema.nodes.heading;
      if (!headingType) return null;

      if (doc.childCount === 0) {
        const heading = headingType.create({ level: 1 });
        return newState.tr.insert(0, heading);
      }

      const firstNode = doc.firstChild;
      if (!firstNode) return null;

      if (firstNode.type.name !== 'heading' || firstNode.attrs.level !== 1) {
        return newState.tr.setNodeMarkup(0, headingType, { level: 1 });
      }

      return null;
    },
  }),
);

const { loading, get } = useEditor((container) => {
  onTitleRequired = () => emit('title-required');
  const maker = Editor.make()
    .use(commonmark)
    .use(gfm)
    .use(history)
    .use(listener)
    .use(firstLineHeading)
    .use(nordTheme);

  maker.config((ctx: any) => {
    ctx.set(rootCtx, container);
    ctx.set(defaultValueCtx, props.content);
    ctx.get(listenerCtx).markdownUpdated((_ctx: any, markdown: string) => {
      emit('update:content', markdown);
    });
  });

  return maker;
});

function emitPreviewHtml() {
  const editor = get();
  if (!editor) return;
  editor.action((ctx: any) => {
    const view = ctx.get(editorViewCtx);
    const html = view.dom.innerHTML;
    emit('preview-html', html);
  });
}

watch(
  () => props.content,
  (newVal) => {
    if (loading.value) return;
    const editor = get();
    if (!editor) return;
    const current = editor.action(getMarkdown());
    if (newVal !== current) {
      editor.action(replaceAll(newVal));
    }
    emitPreviewHtml();
  },
);

watch(
  () => props.showPreview,
  (v) => {
    if (loading.value) return;
    const editor = get();
    if (!editor) return;
    editor.action((ctx: any) => {
      ctx.get(editorViewCtx).setProps({ editable: () => !v });
    });
  },
);

watch(loading, (v) => {
  if (!v) emitPreviewHtml();
});
</script>

<template>
  <div class="milkdown-host" :class="{ 'is-preview': showPreview }">
    <Milkdown />
  </div>
</template>

<style>
.milkdown-host {
  height: 100%;
  overflow: auto;
}

.milkdown-host .ProseMirror:focus {
  outline: none;
  box-shadow: none;
}

.milkdown-host [data-milkdown-root] {
  height: 100%;
}

.milkdown-host .editor {
  max-width: 720px;
  margin: 0 auto;
  padding: 24px 32px;
  font-family: var(--font-editor, 'Iowan Old Style', 'Noto Serif SC', serif);
  font-size: 16px;
  line-height: 1.8;
  color: var(--text-color);
}

.milkdown-host.is-preview .ProseMirror {
  cursor: default;
}
</style>
