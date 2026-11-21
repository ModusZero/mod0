<script lang="ts">
    interface Props {
        /** `vertical` = divisor vertical (cambia el ancho); `horizontal` = divisor horizontal (cambia el alto). */
        orientation: 'vertical' | 'horizontal';
        /** Recibe el desplazamiento acumulado en píxeles desde que empezó el arrastre. */
        ondrag: (delta: number) => void;
        onstart?: () => void;
    }

    let { orientation, ondrag, onstart }: Props = $props();

    let origin = 0;
    let dragging = $state(false);

    function down(event: PointerEvent) {
        dragging = true;
        origin = orientation === 'vertical' ? event.clientX : event.clientY;
        onstart?.();
        (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
    }

    function move(event: PointerEvent) {
        if (!dragging) return;
        ondrag((orientation === 'vertical' ? event.clientX : event.clientY) - origin);
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class="group relative z-10 shrink-0 {orientation === 'vertical'
        ? 'w-0 cursor-col-resize'
        : 'h-0 cursor-row-resize'}"
    onpointerdown={down}
    onpointermove={move}
    onpointerup={() => (dragging = false)}
>
    <div
        class="absolute transition-colors group-hover:bg-accent/60 {dragging
            ? 'bg-accent/60'
            : ''} {orientation === 'vertical' ? '-left-0.75 h-full w-1.5' : '-top-0.75 h-1.5 w-full'}"
    ></div>
</div>
