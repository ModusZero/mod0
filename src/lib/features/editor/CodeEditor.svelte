<script lang="ts">
    import { Compartment, EditorState } from '@codemirror/state';
    import { LanguageDescription } from '@codemirror/language';
    import { languages } from '@codemirror/language-data';
    import { EditorView, keymap } from '@codemirror/view';
    import { basicSetup } from 'codemirror';
    import { onDestroy, onMount, untrack } from 'svelte';
    import { basename } from '#lib/core/utils/format';
    import { editorTheme, highlightTheme } from './editor-theme';

    interface Props {
        path: string;
        content: string;
        /** Rutas de todas las pestañas abiertas (para descartar el estado de las cerradas). */
        openPaths: string[];
        onchange: (content: string) => void;
        onsave: () => void;
    }

    let { path, content, openPaths, onchange, onsave }: Props = $props();

    let host: HTMLDivElement;
    let view: EditorView | null = null;
    let current = '';

    // Cada pestaña conserva su propio estado (cursor, historial de deshacer).
    const states = new Map<string, EditorState>();
    const language = new Compartment();

    function create(doc: string): EditorState {
        return EditorState.create({
            doc,
            extensions: [
                basicSetup,
                editorTheme,
                highlightTheme,
                language.of([]),
                keymap.of([{ key: 'Mod-s', preventDefault: true, run: () => (onsave(), true) }]),
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) onchange(update.state.doc.toString());
                }),
            ],
        });
    }

    async function loadLanguage(target: string) {
        const description = LanguageDescription.matchFilename(languages, basename(target));
        const support = description ? await description.load().catch(() => null) : null;
        if (target === current && view) view.dispatch({ effects: language.reconfigure(support ?? []) });
    }

    onMount(() => {
        current = path;
        view = new EditorView({ state: create(content), parent: host });
        void loadLanguage(path);
    });

    onDestroy(() => view?.destroy());

    $effect(() => {
        const next = path;
        if (!view || next === current) return;

        states.set(current, view.state);
        current = next;
        view.setState(states.get(next) ?? create(untrack(() => content)));
        void loadLanguage(next);
    });

    $effect(() => {
        for (const key of states.keys()) if (!openPaths.includes(key)) states.delete(key);
    });
</script>

<div bind:this={host} class="h-full min-h-0 w-full"></div>
