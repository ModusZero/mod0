<script lang="ts">
    import { FileText } from '@lucide/svelte';
    import { tick } from 'svelte';
    import Segmented from '#lib/components/common/Segmented.svelte';
    import { buildCommands } from '#lib/core/registry/commands';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { editorStack } from '#lib/core/runes/editor.svelte';
    import { filesStack } from '#lib/core/runes/files.svelte';
    import { toastStack } from '#lib/core/runes/toast-runes.svelte';
    import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
    import type { PaletteItem } from '#lib/core/types/menu';
    import { basename, fuzzyScore, joinPath } from '#lib/core/utils/format';

    const MAX_RESULTS = 40;

    let query = $state('');
    let selected = $state(0);
    let input = $state<HTMLInputElement>();
    let commands = $state<PaletteItem[]>([]);
    let files = $state<string[]>([]);

    const mode = $derived(toggleUIStack.paletteMode);

    // Al abrirse: reconstruye los comandos, carga el índice de archivos y enfoca el campo.
    $effect(() => {
        if (!toggleUIStack.paletteOpen) return;
        query = '';
        selected = 0;
        commands = buildCommands();
        void filesStack
            .ensureIndex()
            .then((index) => (files = index))
            .catch(toastStack.error);
        void tick().then(() => input?.focus());
    });

    const results = $derived.by((): PaletteItem[] => {
        if (mode === 'files') {
            const root = filesStack.rootPath;
            if (!root) return [];

            return files
                .map((path) => ({ path, score: fuzzyScore(query, path) }))
                .filter((entry): entry is { path: string; score: number } => entry.score !== null)
                .sort((a, b) => b.score - a.score || a.path.length - b.path.length)
                .slice(0, MAX_RESULTS)
                .map(({ path }) => ({
                    id: path,
                    label: basename(path),
                    hint: path,
                    icon: FileText,
                    run: () => void editorStack.open(joinPath(root, path)),
                }));
        }

        return commands
            .map((item) => ({ item, score: fuzzyScore(query, `${item.label} ${item.keywords ?? ''}`) }))
            .filter((entry): entry is { item: PaletteItem; score: number } => entry.score !== null)
            .sort((a, b) => b.score - a.score)
            .slice(0, MAX_RESULTS)
            .map(({ item }) => item);
    });

    $effect(() => {
        void results;
        selected = 0;
    });

    function close() {
        toggleUIStack.paletteOpen = false;
    }

    function run(item: PaletteItem | undefined) {
        if (!item) return;
        close();
        void Promise.resolve(item.run()).catch(toastStack.error);
    }

    function onKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape') close();
        else if (event.key === 'ArrowDown') selected = Math.min(results.length - 1, selected + 1);
        else if (event.key === 'ArrowUp') selected = Math.max(0, selected - 1);
        else if (event.key === 'Enter') run(results[selected]);
        else return;
        event.preventDefault();

        void tick().then(() =>
            document.querySelector('[data-selected="true"]')?.scrollIntoView({ block: 'nearest' }),
        );
    }
</script>

{#if toggleUIStack.paletteOpen}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
        class="fixed inset-0 z-50 flex items-start justify-center bg-black/45 pt-[14vh]"
        onpointerdown={(event) => event.target === event.currentTarget && close()}
    >
        <div
            class="flex max-h-[64vh] w-[38rem] max-w-[94vw] flex-col rounded-xl border border-border-subtle bg-raised shadow-2xl"
        >
            <div class="flex items-center gap-3 border-b border-border-subtle p-2.5">
                <input
                    bind:this={input}
                    bind:value={query}
                    onkeydown={onKeydown}
                    class="h-8 min-w-0 flex-1 bg-transparent px-1.5 text-[0.9375rem] outline-none placeholder:text-faint"
                    placeholder={mode === 'files' ? t('palette.filesPlaceholder') : t('palette.placeholder')}
                    spellcheck="false"
                    autocomplete="off"
                />
                <Segmented
                    options={[
                        { value: 'commands', label: t('palette.commands') },
                        { value: 'files', label: t('palette.files') },
                    ]}
                    value={mode}
                    onchange={(value) => {
                        toggleUIStack.paletteMode = value;
                        void tick().then(() => input?.focus());
                    }}
                />
            </div>

            <div class="min-h-0 flex-1 overflow-y-auto p-1.5">
                {#each results as item, index (item.id)}
                    <button
                        data-selected={index === selected}
                        class="flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-left {index ===
                        selected
                            ? 'bg-accent-soft text-text'
                            : 'text-muted hover:bg-hover'}"
                        onpointermove={() => (selected = index)}
                        onclick={() => run(item)}
                    >
                        {#if item.icon}<item.icon size={14} class="shrink-0" />{:else}<span class="w-3.5"
                            ></span>{/if}
                        <span class="min-w-0 flex-1 truncate {index === selected ? 'text-text' : ''}"
                            >{item.label}</span
                        >
                        {#if item.hint}
                            <span class="max-w-[16rem] truncate font-mono text-meta text-faint"
                                >{item.hint}</span
                            >
                        {/if}
                    </button>
                {:else}
                    <p class="px-3 py-8 text-center text-meta text-faint">{t('palette.empty')}</p>
                {/each}
            </div>
        </div>
    </div>
{/if}
