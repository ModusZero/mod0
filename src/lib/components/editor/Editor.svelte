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
    const themeConf = new Compartment();

    onMount(() => {
        view = new EditorView({
            doc: editorStack.activeFile?.content || "",
            extensions: [
                basicSetup,
                themeConf.of(configStack.current.theme === 'dark' ? oneDark : []),
                javascript(),
                EditorView.updateListener.of((u) => {
                    if (u.docChanged) editorStack.updateContent(u.state.doc.toString());
                })
            ],
            parent: editorElement
        });
    });

    $effect(() => {
        if (view) {
            view.dispatch({
                effects: themeConf.reconfigure(configStack.current.theme === 'dark' ? oneDark : [])
            });
        }
    });

    onDestroy(() => view?.destroy());
</script>

<div class="h-full w-full border-t border-black/5 dark:border-white/5" bind:this={editorElement}></div>