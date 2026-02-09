<script lang="ts">
    import { type Snippet } from 'svelte';
    import Portal from './Portal.svelte';
    import Overlay from './Overlay.svelte';

    let { anchorEl, children, onclose, offset = 8 }: { 
        anchorEl: HTMLElement | null, 
        children: Snippet,
        onclose?: () => void,
        offset?: number 
    } = $props();

    let panelEl = $state<HTMLElement | null>(null);
    let coords = $state({ x: 0, y: 0 });

    $effect(() => {
        if (anchorEl && panelEl) {
            const rect = anchorEl.getBoundingClientRect();

            const pWidth = panelEl.offsetWidth;
            
            let x = rect.left + (rect.width / 2) - (pWidth / 2);
            let y = rect.bottom + offset;

            
            if (x + pWidth > window.innerWidth) x = window.innerWidth - pWidth - 12;
            if (x < 12) x = 12;

            coords = { x, y };
        }
    });

    $effect(() => {
        const handleEsc = (e: KeyboardEvent) => {
            if (e.key === 'Escape') onclose?.();
        };
        window.addEventListener('keydown', handleEsc);
        return () => window.removeEventListener('keydown', handleEsc);
    });
</script>

{#if anchorEl}
    <Portal>
        <Overlay onclick={() => onclose?.()} />
        
        <div 
            bind:this={panelEl}
            style:left="{coords.x}px"
            style:top="{coords.y}px"
            style:position="fixed"
            class="z-9999 pointer-events-auto shadow-2xl"
        >
            {@render children()}
        </div>
    </Portal>
{/if}