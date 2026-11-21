<script lang="ts">
    import type { Component, Snippet } from 'svelte';

    interface Props {
        icon: Component<any>;
        title: string;
        hint?: string;
        children?: Snippet;
    }

    let { icon: Icon, title, hint, children }: Props = $props();
</script>

<div class="flex h-full flex-col items-center justify-center gap-3 p-8 text-center">
    <div class="rounded-2xl bg-hover p-4 text-faint"><Icon size={26} strokeWidth={1.5} /></div>
    <div class="space-y-1">
        <p class="text-ui font-medium text-text">{title}</p>
        {#if hint}<p class="max-w-sm text-meta text-muted">{hint}</p>{/if}
    </div>
    {@render children?.()}
</div>
