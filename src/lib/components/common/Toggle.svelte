<script lang="ts">
    interface Props {
        checked: boolean;
        label: string;
        hint?: string;
        onchange: (checked: boolean) => void;
    }

    let { checked, label, hint, onchange }: Props = $props();
</script>

<label class="flex cursor-default items-start justify-between gap-4 py-1.5">
    <span class="min-w-0">
        <span class="block">{label}</span>
        {#if hint}<span class="block text-meta text-muted">{hint}</span>{/if}
    </span>
    <input
        type="checkbox"
        class="peer sr-only"
        {checked}
        onchange={(event) => onchange(event.currentTarget.checked)}
    />
    <span
        class="relative mt-0.5 h-5 w-9 shrink-0 rounded-full transition-colors peer-focus-visible:outline-2 peer-focus-visible:outline-accent {checked
            ? 'bg-accent'
            : 'bg-active'}"
    >
        <span
            class="absolute top-0.5 left-0.5 size-4 rounded-full bg-white shadow transition-transform {checked
                ? 'translate-x-4'
                : ''}"
        ></span>
    </span>
</label>
