<script lang="ts">
    import { fade, fly } from 'svelte/transition';
    import { cubicInOut } from 'svelte/easing';
    import { workStack } from "$lib/core/runes/work-runes.svelte";
    import { ActivityVisual } from "$lib/core/constants/work-config";
    import { toggleUIStack } from '$lib/core/runes/toggle-ui.svelte';
    import { type DisplayNode } from '$lib/core/types/work';

    let activities = $derived(workStack.currentActivities);
</script>

<aside class="w-14 flex flex-col items-center bg-sidebar border-r border-border-subtle z-40 relative select-none">
    <nav class="flex-1 flex flex-col gap-4 w-full items-center">
        {#each activities as activityId, i (activityId)}
            {@const activity: DisplayNode = ActivityVisual[activityId]}
            {#if activity}
                {@const isActive: boolean = workStack.activeActivity === activityId}
                <div in:fly={{ y: 8, duration: 400, delay: i * 40, easing: cubicInOut }}>
                    <button 
                        onclick={() => {
                            if (isActive) {
                                toggleUIStack.toggleSidebar();
                            } else {
                                workStack.setActivity(activityId)
                            }
                            }}
                        aria-label="Actividad: {activity.label}"
                        class="group relative flex items-center justify-center w-12 h-12 transition-all duration-300
                        {isActive ? 'text-accent' : 'text-text/40 hover:text-text'}"
                    >
                        {#if isActive}
                            <div 
                                in:fade={{ duration: 200 }}
                                class="absolute left-0 w-1 h-6 bg-accent rounded-r-full shadow-[2px_0_10px_var(--accent)]"
                            ></div>
                        {/if}

                        <div class="relative z-10 p-2 rounded-xl group-hover:bg-accent/5 transition-colors">
                            <activity.icon size={20} strokeWidth={isActive ? 2.5 : 1.8} />
                        </div>

                        <span class="absolute left-16 bg-sidebar text-text border border-border-subtle px-3 py-1.5 rounded-lg text-[10px] font-bold uppercase tracking-widest opacity-0 group-hover:opacity-100 transition-all -translate-x-2 group-hover:translate-x-0 shadow-xl pointer-events-none z-50">
                            {activity.label}
                        </span>
                    </button>
                </div>
            {/if}
        {/each}
    </nav>
</aside>