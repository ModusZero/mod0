<script lang="ts">
    import { CircleCheck, CircleX, Info } from '@lucide/svelte';
    import { toastStack } from '#lib/core/runes/toast-runes.svelte';
</script>

<div class="pointer-events-none fixed right-4 bottom-10 z-[60] flex w-80 flex-col gap-2">
    {#each toastStack.toasts as toast (toast.id)}
        <button
            class="pointer-events-auto flex items-start gap-2 rounded-lg border border-border-subtle bg-raised p-3 text-left text-ui shadow-lg"
            onclick={() => toastStack.dismiss(toast.id)}
        >
            {#if toast.kind === 'error'}
                <CircleX size={15} class="mt-0.5 shrink-0 text-danger" />
            {:else if toast.kind === 'success'}
                <CircleCheck size={15} class="mt-0.5 shrink-0 text-ok" />
            {:else}
                <Info size={15} class="mt-0.5 shrink-0 text-accent" />
            {/if}
            <span class="min-w-0 break-words">{toast.message}</span>
        </button>
    {/each}
</div>
