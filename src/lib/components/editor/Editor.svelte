<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { EditorView, basicSetup } from 'codemirror';
    import { Compartment } from '@codemirror/state';
    import { javascript } from '@codemirror/lang-javascript';
    import { oneDark } from '@codemirror/theme-one-dark';
    import { editorStack } from '$lib/runes/editor.svelte';
    import { configStack } from '$lib/runes/config.svelte';

    let editorElement: HTMLDivElement;
    let view: EditorView;
    
    const themeConfig = new Compartment();

    onMount(() => {
        view = new EditorView({
            doc: editorStack.activeFile?.content || "",
            extensions: [
                basicSetup,
                themeConfig.of(oneDark), // Iniciar con tema oscuro
                javascript(),
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) {
                        editorStack.updateContent(update.state.doc.toString());
                    }
                })
            ],
            parent: editorElement
        });
    });

    // Reaccionar al cambio de archivo activo (Efecto de Svelte 5)
    $effect(() => {
        const content = editorStack.activeFile?.content;
        if (view && content !== undefined && content !== view.state.doc.toString()) {
            view.dispatch({
                changes: { from: 0, to: view.state.doc.length, insert: content }
            });
        }
    });

    onDestroy(() => {
        if (view) view.destroy();
    });
</script>

<div class="h-full w-full bg-[#1e1e1e]" bind:this={editorElement}></div>

<style>
    :global(.cm-editor) {
        height: 100%;
        outline: none !important;
    }
    :global(.cm-scroller) {
        font-family: 'Fira Code', 'JetBrains Mono', monospace;
    }
</style>