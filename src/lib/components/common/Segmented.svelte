<script lang="ts" generics="T extends string">
    interface Props {
        options: Array<{ value: T; label: string; badge?: number }>;
        value: T;
        onchange: (value: T) => void;
    }

    let { options, value, onchange }: Props = $props();
</script>

<div class="inline-flex rounded-md border border-border-subtle bg-main p-0.5">
    {#each options as option (option.value)}
        <button
            class="flex h-6 items-center gap-1.5 rounded px-2.5 text-meta transition-colors {value ===
            option.value
                ? 'bg-raised text-text shadow-sm'
                : 'text-muted hover:text-text'}"
            onclick={() => onchange(option.value)}
        >
            {option.label}
            {#if option.badge}<span class="font-mono text-faint">{option.badge}</span>{/if}
        </button>
    {/each}
</div>
