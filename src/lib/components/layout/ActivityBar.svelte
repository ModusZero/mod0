<script lang="ts">
    import { fade, fly } from 'svelte/transition';
    import { cubicInOut } from 'svelte/easing';
    import { uiStack } from "$lib/runes/ui.svelte";
    import { ActivityVisual } from "$lib/constants/ui";

    let activities = $derived(uiStack.currentActivities);
</script>

<aside class="w-14 flex flex-col items-center py-6 bg-sidebar border-r border-black/8 dark:border-white/5 z-40 relative select-none">
    <div class="mb-10 w-8 h-8 rounded-xl bg-accent/10 flex items-center justify-center border border-accent/20">
        <div class="w-3 h-3 rounded-full bg-accent shadow-[0_0_10px_var(--accent)]"></div>
    </div>

    <nav class="flex-1 flex flex-col gap-4 w-full items-center">
        {#each activities as activityId, i (activityId)}
            {@const activity = ActivityVisual[activityId]}
            {#if activity}
                {@const isActive = uiStack.activeActivity === activityId}
                
                <div in:fly={{ y: 8, duration: 400, delay: i * 40, easing: cubicInOut }}>
                    <button 
                        onclick={() => uiStack.setActivity(activityId)}
                        class="group relative flex items-center justify-center w-12 h-12 transition-all duration-300
                        {isActive ? 'text-accent' : 'text-black/40 dark:text-white/30 hover:text-black dark:hover:text-white'}"
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

                        <span class="absolute left-16 bg-white dark:bg-[#18181b] text-black dark:text-white border border-black/10 dark:border-white/10 px-3 py-1.5 rounded-lg text-[10px] font-bold uppercase tracking-widest opacity-0 group-hover:opacity-100 transition-all -translate-x-2 group-hover:translate-x-0 shadow-xl pointer-events-none z-50">
                            {activity.label}
                        </span>
                    </button>
                </div>
            {/if}
        {/each}
    </nav>
</aside>