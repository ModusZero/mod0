<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { EditorView, basicSetup } from 'codemirror';
    import { javascript } from '@codemirror/lang-javascript';
    import { oneDark } from '@codemirror/theme-one-dark';
    import { Compartment } from '@codemirror/state';
    import { settingsStack } from '$lib/features/settings/settings-runes.svelte';
    import { editorStack } from './editor-runes.svelte';

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
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) {
                        editorStack.updateContent(update.state.doc.toString());
                    }
                }),
                EditorView.theme({
                    "&": { height: "100%", fontSize: "13px" },
                    ".cm-scroller": { fontFamily: "'JetBrains Mono', monospace" }
                })
            ],
            parent: editorElement
        });
    });

    /** Sincroniza cambios externos (ej. cambio de pestaña) hacia el editor */
    $effect(() => {
        if (view && content !== view.state.doc.toString()) {
            view.dispatch({
                changes: { from: 0, to: view.state.doc.length, insert: content }
            });
        }
    });

    /** Reconfigura tema */
    $effect(() => {
        if (view) {
            view.dispatch({ 
                effects: themeConf.reconfigure(settingsStack.current.theme === 'dark' ? oneDark : []) 
            });
        }
    });

    /** Manejo de atajos de teclado global (Fuera del efecto del tema) */
    $effect(() => {
        const handleSave = (e: KeyboardEvent) => {
            // Verificamos Ctrl+S o Cmd+S (para Mac)
            if ((e.ctrlKey || e.metaKey) && e.key === 's') {
                e.preventDefault();
                e.stopPropagation();
                e.stopImmediatePropagation(); // Detiene a otros manejadores
                
                console.log("Guardado quirúrgico activado...");
                editorStack.saveFileChanges();
            }
        };

        window.addEventListener('keydown', handleSave, true); // Usamos "capture phase" (true)
        return () => window.removeEventListener('keydown', handleSave, true);
    });

    onDestroy(() => view?.destroy());
</script>

<div class="h-full w-full overflow-hidden" bind:this={editorElement}></div>