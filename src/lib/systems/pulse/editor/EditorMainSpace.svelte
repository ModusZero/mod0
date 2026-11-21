<script lang="ts">
    import { FileCode, X } from '@lucide/svelte';
    import EmptyState from '#lib/components/common/EmptyState.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { editorStack } from '#lib/core/runes/editor.svelte';
    import { toggleUIStack } from '#lib/core/runes/toggle-ui.svelte';
    import CodeEditor from '#lib/features/editor/CodeEditor.svelte';

    const active = $derived(editorStack.active);
    const openPaths = $derived(editorStack.tabs.map((tab) => tab.path));
</script>

<div class="flex h-full flex-col">
    {#if editorStack.tabs.length > 0}
        <div
            class="flex h-9 shrink-0 items-end gap-px overflow-x-auto border-b border-border-subtle bg-sidebar px-1"
        >
            {#each editorStack.tabs as tab (tab.path)}
                {@const dirty = editorStack.isDirty(tab)}
                <div
                    class="group flex h-8 max-w-56 shrink-0 items-center gap-1.5 rounded-t-md border border-b-0 pr-1 pl-3 {tab.path ===
                    editorStack.activePath
                        ? 'border-border-subtle bg-main'
                        : 'border-transparent text-muted hover:bg-hover'}"
                >
                    <button
                        class="min-w-0 flex-1 truncate text-left"
                        title={tab.path}
                        onclick={() => (editorStack.activePath = tab.path)}
                    >
                        {tab.name}
                    </button>
                    <button
                        class="icon-btn size-5"
                        aria-label={t('common.close')}
                        onclick={() => editorStack.close(tab.path)}
                    >
                        {#if dirty}
                            <span class="size-2 rounded-full bg-accent group-hover:hidden"></span>
                            <X size={12} class="hidden group-hover:block" />
                        {:else}
                            <X size={12} />
                        {/if}
                    </button>
                </div>
            {/each}
        </div>
    {/if}

    <div class="min-h-0 flex-1">
        {#if active}
            <CodeEditor
                path={active.path}
                content={active.content}
                {openPaths}
                onchange={(content) => editorStack.setContent(active.path, content)}
                onsave={() => editorStack.save(active.path)}
            />
        {:else}
            <EmptyState icon={FileCode} title={t('editor.empty')} hint={t('editor.emptyHint')}>
                <button class="btn" onclick={() => toggleUIStack.openPalette('files')}>
                    {t('palette.files')} <span class="kbd">Ctrl P</span>
                </button>
            </EmptyState>
        {/if}
    </div>
</div>
