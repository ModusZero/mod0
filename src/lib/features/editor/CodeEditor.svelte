<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { EditorView, basicSetup } from 'codemirror';
    import { oneDark } from '@codemirror/theme-one-dark';
    import { Compartment } from '@codemirror/state';
    import { settingsStack } from '$lib/features/settings/settings-runes.svelte';
    import { editorStack } from './editor-runes.svelte';
    import type { CodeTab } from '$lib/core/types/tab';

    let { tab }: { tab: CodeTab } = $props();
    let editorElement: HTMLDivElement;
    let view: EditorView;
    
    const themeConf = new Compartment();
    const langConf = new Compartment();

    onMount(async () => {
        // Carga automática del lenguaje al iniciar
        const langExtension = await editorStack.getDynamicLanguage(tab.label || 'file.txt');

        view = new EditorView({
            doc: tab.content,
            extensions: [
                basicSetup,
                langConf.of(langExtension),
                themeConf.of(settingsStack.current.theme === 'dark' ? oneDark : []),
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) {
                        editorStack.updateContent(update.state.doc.toString());
                    }
                }),
                EditorView.theme({
                    "&": { height: "100%", fontSize: "14px" },
                    ".cm-scroller": { fontFamily: "'JetBrains Mono', monospace", lineHeight: "1.6" }
                })
            ],
            parent: editorElement
        });
    });

    // Cambio de archivo o lenguaje
    $effect(() => {
        if (view && tab.id) {
            const currentDoc = view.state.doc.toString();
            if (tab.content !== currentDoc) {
                view.dispatch({
                    changes: { from: 0, to: currentDoc.length, insert: tab.content }
                });
            }
            
            // Reconfiguración dinámica de lenguaje basado en el nuevo tab
            editorStack.getDynamicLanguage(tab.label).then(lang => {
                view.dispatch({
                    effects: langConf.reconfigure(lang)
                });
            });
        }
    });

    $effect(() => {
        const handleKeys = (e: KeyboardEvent) => {
            if ((e.ctrlKey || e.metaKey) && e.key === 's') {
                e.preventDefault();
                editorStack.saveFileChanges();
            }
        };
        window.addEventListener('keydown', handleKeys);
        return () => window.removeEventListener('keydown', handleKeys);
    });

    onDestroy(() => view?.destroy());
</script>

<div class="h-full w-full overflow-hidden" bind:this={editorElement}></div>