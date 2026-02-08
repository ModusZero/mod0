<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { EditorView, basicSetup } from 'codemirror';
    import { javascript } from '@codemirror/lang-javascript';
    import { oneDark } from '@codemirror/theme-one-dark';
    import { Compartment } from '@codemirror/state';
    import { settingsStack } from '$lib/features/settings/settings-runes.svelte';

    let { content = "" } = $props();
    let editorElement: HTMLDivElement;
    let view: EditorView;
    const themeConf = new Compartment();

    onMount(() => {
        view = new EditorView({
            doc: content,
            extensions: [
                basicSetup,
                javascript(),
                themeConf.of(settingsStack.current.theme === 'dark' ? oneDark : []),
                EditorView.theme({
                    "&": { height: "100%", fontSize: "13px" },
                    ".cm-scroller": { fontFamily: "'JetBrains Mono', monospace" },
                    "&.cm-focused": { outline: "none" }
                })
            ],
            parent: editorElement
        });
    });

    $effect(() => {
        if (view) {
            view.dispatch({
                effects: themeConf.reconfigure(settingsStack.current.theme === 'dark' ? oneDark : [])
            });
        }
    });

    onDestroy(() => view?.destroy());
</script>

<div class="h-full w-full overflow-hidden cursor-auto" bind:this={editorElement}></div>