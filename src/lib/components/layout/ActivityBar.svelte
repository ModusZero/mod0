<script lang="ts">
    import {
        ActivityByWork,
        ActivityVisual,
        WorkSectionIDs,
        WorkSectionVisual,
    } from '#lib/core/constants/work-config';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';
    import { workStack } from '#lib/core/runes/work-modes.svelte';
    import type { ActivityID } from '#lib/core/types/work';

    const sections = Object.values(WorkSectionIDs);

    /** Contador que se muestra sobre el icono de una actividad. */
    function badge(activity: ActivityID): number {
        if (activity === 'sessions') return sessionsStack.counts.waiting;
        if (activity === 'review') return sessionsStack.counts.review;
        return 0;
    }
</script>

<nav class="flex w-12 shrink-0 flex-col items-center gap-1 border-r border-border-subtle bg-sidebar py-2">
    {#each sections as section, index (section)}
        {#if index > 0}<div class="my-1 h-px w-5 bg-border-subtle"></div>{/if}

        {#each ActivityByWork[section] as activity (activity)}
            {@const visual = ActivityVisual[activity]}
            {@const count = badge(activity)}
            {@const active = workStack.activity === activity}
            <button
                class="relative flex size-9 items-center justify-center rounded-lg transition-colors {active
                    ? 'bg-accent-soft text-accent'
                    : 'text-muted hover:bg-hover hover:text-text'}"
                title="{t(visual.label)}{visual.shortcut ? ` (${visual.shortcut})` : ''}"
                aria-label={t(visual.label)}
                aria-current={active}
                onclick={() => workStack.setActivity(activity)}
            >
                {#if active}<span class="absolute top-2 -left-1.5 h-5 w-0.5 rounded-full bg-accent"
                    ></span>{/if}
                <visual.icon size={18} strokeWidth={1.75} />
                {#if count > 0}
                    <span
                        class="absolute top-0.5 right-0.5 flex min-w-3.5 items-center justify-center rounded-full bg-st-waiting px-1 font-mono text-[0.5625rem] font-bold text-main"
                    >
                        {count}
                    </span>
                {/if}
            </button>
        {/each}
    {/each}
</nav>
