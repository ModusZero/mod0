<script lang="ts">
    import { fade } from 'svelte/transition';
    let { onclick }: { onclick?: () => void } = $props();
</script>

<button
    type="button"
    aria-label="Cerrar"
    onclick={onclick}
    class="fixed inset-0 z-998 bg-black/50 backdrop-blur-sm cursor-default w-screen h-screen border-none outline-none"
    transition:fade={{ duration: 150 }}
></button>