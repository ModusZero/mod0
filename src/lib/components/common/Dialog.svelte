<script lang="ts">
    import { X } from '@lucide/svelte';
    import type { Snippet } from 'svelte';

    interface Props {
        open: boolean;
        title: string;
        width?: string;
        onclose: () => void;
        children: Snippet;
        footer?: Snippet;
    }

    let { open, title, width = '30rem', onclose, children, footer }: Props = $props();

    function onKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape') {
            event.stopPropagation();
            onclose();
        }
    }
</script>

{#if open}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
        class="fixed inset-0 z-50 flex items-start justify-center bg-black/45 pt-[12vh]"
        onkeydown={onKeydown}
        onpointerdown={(event) => event.target === event.currentTarget && onclose()}
    >
        <div
            role="dialog"
            aria-modal="true"
            aria-label={title}
            class="flex max-h-[76vh] flex-col rounded-xl border border-border-subtle bg-raised shadow-2xl"
            style="width:min({width}, 94vw)"
        >
            <header
                class="flex h-11 shrink-0 items-center justify-between border-b border-border-subtle pr-2 pl-4"
            >
                <h2 class="text-ui font-semibold">{title}</h2>
                <button class="icon-btn" onclick={onclose} aria-label="Cerrar"><X size={15} /></button>
            </header>
            <div class="min-h-0 flex-1 overflow-y-auto p-4">{@render children()}</div>
            {#if footer}
                <footer class="flex shrink-0 justify-end gap-2 border-t border-border-subtle px-4 py-3">
                    {@render footer()}
                </footer>
            {/if}
        </div>
    </div>
{/if}
