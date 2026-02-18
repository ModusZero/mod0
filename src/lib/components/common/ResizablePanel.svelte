<script lang="ts">
    import type { Snippet } from 'svelte';

    interface Props {
        direction: 'horizontal' | 'vertical';
        initialSize: number;
        minSize: number;
        maxSize?: number;
        side: 'left' | 'right' | 'top' | 'bottom';
        children: Snippet;
    }

    let { direction, initialSize, minSize, maxSize, side, children }: Props = $props();

    let size = $state(0); 
    let isResizing = $state(false);
    let hasInitialized = $state(false);

    // Inicialización del tamaño
    $effect(() => {
        if (!hasInitialized) {
            size = initialSize;
            hasInitialized = true;
        }
    });

    function startResizing(e: MouseEvent) {
        // Bloqueamos la selección de texto y el comportamiento por defecto
        e.preventDefault();
        isResizing = true;
    }

    function stopResizing() {
        isResizing = false;
    }

    function onMouseMove(e: MouseEvent) {
        if (!isResizing) return;

        let newSize: number;
        if (direction === 'vertical') {
            newSize = side === 'bottom' ? window.innerHeight - e.clientY : e.clientY;
        } else {
            newSize = side === 'left' ? e.clientX : window.innerWidth - e.clientX;
        }

        // Aplicamos límites
        if (newSize >= minSize && (!maxSize || newSize <= maxSize)) {
            size = newSize;
        }
    }

    function resetSize() {
        size = initialSize;
    }
</script>

<svelte:window onmousemove={onMouseMove} onmouseup={stopResizing} />

{#if isResizing}
    <div 
        class="fixed inset-0 z-9997 select-none {direction === 'horizontal' ? 'cursor-col-resize' : 'cursor-ns-resize'}"
    ></div>
{/if}

<div 
    style="{direction === 'horizontal' ? 'width' : 'height'}: {size}px"
    class="relative flex flex-col overflow-hidden shrink-0"
>
    <button
        type="button"
        onmousedown={startResizing}
        ondblclick={resetSize}
        class="absolute z-40 transition-colors hover:bg-accent/40 active:bg-accent
        {direction === 'horizontal' ? 'w-1 top-0 bottom-0 cursor-col-resize' : 'h-1 left-0 right-0 cursor-ns-resize'}
        {side === 'left' ? 'right-0' : side === 'right' ? 'left-0' : side === 'bottom' ? 'top-0' : 'bottom-0'}"
        aria-label="Resize handle"
    ></button>

    {@render children()}
</div>