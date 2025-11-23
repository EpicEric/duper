<template>
  <div class="duper-editor">
    <div class="editor-container">
      <div class="editor-tabs">
        <div class="active">Duper</div>
      </div>
      <div class="editor" ref="duperMonaco"></div>

      <div class="code-editor">
        <div class="editor-tabs">
          <button :class="{ active: activeTab === 'json' }" @click="switchTab('json')">
            JSON
          </button>
          <button :class="{ active: activeTab === 'yaml' }" @click="switchTab('yaml')">
            YAML
          </button>
          <button :class="{ active: activeTab === 'toml' }" @click="switchTab('toml')">
            TOML
          </button>
        </div>
        <div class="editor" ref="otherMonaco"></div>
      </div>
    </div>

    <div v-if="error" class="error-message">
      {{ error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import jsonGrammar from "@shikijs/langs/json";
import tomlGrammar from "@shikijs/langs/toml";
import yamlGrammar from "@shikijs/langs/yaml";
import { shikiToMonaco } from "@shikijs/monaco";
import githubDarkTheme from "@shikijs/themes/github-dark";
import githubLightTheme from "@shikijs/themes/github-light";
import { createHighlighterCore } from "shiki/core";
import { createOnigurumaEngine } from "shiki/engine/oniguruma";
import shikiWasm from "shiki/wasm";
import { useData } from "vitepress";
import { onMounted, onUnmounted, ref, watch } from "vue";
import { convertDuper } from "@/pkg/duper_website";

const props = defineProps<{
  initial?: string;
}>();

const activeTab = ref<"json" | "yaml" | "toml">("json");
const error = ref("");
const { isDark } = useData();

const duperMonaco = ref<HTMLDivElement>();
const otherMonaco = ref<HTMLDivElement>();
let duperEditor: any = null;
let otherEditor: any = null;

onMounted(async () => {
  const monaco = await import("monaco-editor/esm/vs/editor/editor.api");

  const highlighter = await createHighlighterCore({
    themes: [githubDarkTheme, githubLightTheme],
    langs: [
      jsonGrammar,
      yamlGrammar,
      tomlGrammar,
      import.meta.env.DUPER_GRAMMAR,
      import.meta.env.EBNF_GRAMMAR,
    ],
    engine: createOnigurumaEngine(shikiWasm),
  });

  monaco.languages.register({ id: "json" });
  monaco.languages.register({ id: "yaml" });
  monaco.languages.register({ id: "toml" });
  monaco.languages.register({ id: "duper" });

  shikiToMonaco(highlighter, monaco);

  const editorOptions: any = {
    scrollBeyondLastLine: false,
    automaticLayout: true,
    minimap: { enabled: false },
    theme: isDark.value ? "github-dark" : "github-light",
  };

  duperEditor = monaco.editor.create(duperMonaco.value!, {
    ...editorOptions,
    value: props.initial || "",
    language: "duper",
  });
  duperEditor.getModel().onDidChangeContent(handleDuperInput);

  otherEditor = monaco.editor.create(otherMonaco.value!, {
    ...(editorOptions as any),
    value: props.initial ? convertDuper(props.initial, "json") : "",
    language: "json",
    readOnly: true,
  });
});

onUnmounted(() => {
  duperEditor?.dispose();
  otherEditor?.dispose();
});

function handleDuperInput() {
  try {
    error.value = "";
    const text = duperEditor?.getValue() || "";
    otherEditor?.setValue(convertDuper(text, activeTab.value));
  } catch (err) {
    error.value = err instanceof Error ? err.message : String(err);
  }
}

function switchTab(tab: "json" | "yaml" | "toml") {
  activeTab.value = tab;
  otherEditor?.getModel().setLanguage(tab);
  handleDuperInput();
}

watch(isDark, (newIsDark) => {
  duperEditor?.updateOptions({
    theme: newIsDark ? "github-dark" : "github-light",
  });
  otherEditor?.updateOptions({
    theme: newIsDark ? "github-dark" : "github-light",
  });
});
</script>

<style scoped>
.duper-editor {
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  overflow: hidden;
  margin: 1rem 0;
}

.editor-tabs {
  display: flex;
  background: var(--vp-c-bg-soft);
  border-bottom: 1px solid var(--vp-c-divider);
  z-index: 30;
}

.editor-tabs * {
  padding: 0.75rem 1rem;
  background: none;
  border: none;
  cursor: pointer;
  border-bottom: 2px solid transparent;
  color: var(--vp-c-text-2);
  font-size: 0.875rem;
  transition: all 0.2s ease;
}

.editor-tabs *:hover {
  color: var(--vp-c-text-1);
}

.editor-tabs .active {
  border-bottom-color: var(--vp-c-brand);
  color: var(--vp-c-brand);
}

.editor {
  min-height: 22rem;
}

.error-message {
  padding: 0.75rem 1rem;
  background: var(--vp-c-red-soft);
  color: var(--vp-c-red);
  font-size: 0.875rem;
  border-top: 1px solid var(--vp-c-divider);
}
</style>
