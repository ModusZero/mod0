<script lang="ts">
    import { fade } from 'svelte/transition';
    let { onclick }: { onclick?: () => void } = $props();
</script>

<button
    type="button"
    aria-label="Cerrar"
    onclick={onclick}
    class="fixed inset-0 z-9998 bg-black/50 backdrop-blur-sm cursor-default w-screen h-screen border-none outline-none pointer-events-auto"
    transition:fade={{ duration: 150 }}
></button>