<script lang="ts">
    import { Compartment, EditorState } from '@codemirror/state';
    import { LanguageDescription } from '@codemirror/language';
    import { languages } from '@codemirror/language-data';
    import { MergeView } from '@codemirror/merge';
    import { EditorView, lineNumbers } from '@codemirror/view';
    import { onDestroy, onMount } from 'svelte';
    import { basename } from '#lib/core/utils/format';
    import { editorTheme, highlightTheme } from './editor-theme';

    interface Props {
        path: string;
        original: string;
        modified: string;
    }

    let { path, original, modified }: Props = $props();

    let host: HTMLDivElement;
    let merge: MergeView | null = null;

    const languageA = new Compartment();
    const languageB = new Compartment();

    /** Editor de solo lectura con altura automática (el desplazamiento lo hace el contenedor). */
    const readonly = (language: Compartment) => [
        lineNumbers(),
        editorTheme,
        EditorView.theme({ '&': { height: 'auto' } }),
        highlightTheme,
        language.of([]),
        EditorState.readOnly.of(true),
        EditorView.editable.of(false),
    ];

    onMount(() => {
        merge = new MergeView({
            a: { doc: original, extensions: readonly(languageA) },
            b: { doc: modified, extensions: readonly(languageB) },
            parent: host,
            highlightChanges: true,
            gutter: true,
            collapseUnchanged: { margin: 3, minSize: 5 },
        });

        const description = LanguageDescription.matchFilename(languages, basename(path));
        void description?.load().then((support) => {
            merge?.a.dispatch({ effects: languageA.reconfigure(support) });
            merge?.b.dispatch({ effects: languageB.reconfigure(support) });
        });
    });

    onDestroy(() => merge?.destroy());
</script>

<div bind:this={host} class="h-full overflow-auto bg-main"></div>
