<script lang="ts">
    import { Bell, Eye } from '@lucide/svelte';
    import { StatusStyle } from '#lib/core/constants/work-config';
    import type { SessionStatus } from '#lib/core/types/session';

    interface Props {
        status: SessionStatus;
        size?: number;
    }

    let { status, size = 14 }: Props = $props();
</script>

<!-- Cada estado tiene color y forma distintos para no depender solo del color. -->
<span class="inline-flex shrink-0 items-center justify-center" style="width:{size}px;height:{size}px">
    {#if status === 'working' || status === 'starting'}
        <span
            class="anim-spin block rounded-full border-2 border-current/25 border-t-current {StatusStyle[
                status
            ].text}"
            style="width:{size - 2}px;height:{size - 2}px"
        ></span>
    {:else if status === 'waiting'}
        <Bell {size} class="anim-pulse {StatusStyle.waiting.text}" />
    {:else if status === 'review'}
        <Eye {size} class={StatusStyle.review.text} />
    {:else if status === 'idle'}
        <span
            class="block rounded-full border-2 border-st-idle"
            style="width:{size - 4}px;height:{size - 4}px"
        ></span>
    {:else}
        <span class="block rounded-[2px] bg-st-exited" style="width:{size - 6}px;height:{size - 6}px"></span>
    {/if}
</span>
