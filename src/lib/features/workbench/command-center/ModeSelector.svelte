<script lang="ts">
	import { scale, fade } from 'svelte/transition';
	import { cubicOut, cubicInOut } from 'svelte/easing';
	import { workStack } from "$lib/core/runes/work-runes.svelte";
	import { WorkSectionIDs, WorkSectionVisual } from "$lib/core/constants/work-config";
	import FloatingPanel from "$lib/components/common/low-level/FloatingPanel.svelte";
	import type { WorkSectionID } from "$lib/core/types/work";

	let { anchorEl, onclose }: { anchorEl: HTMLElement | null, onclose: () => void } = $props();

	const colorMap: Record<string, string> = {
		'text-blue-400': '#60a5fa',
		'text-orange-400': '#fb923c',
		'text-purple-400': '#c084fc',
		'text-gray-400': '#9ca3af'
	};

	function selectMode(id: WorkSectionID) {
		workStack.setMode(id);
		onclose();
	}
</script>

<FloatingPanel {anchorEl} {onclose} offset={24}>
    <div 
        role="menu"
        in:scale={{ duration: 400, start: 0.9, easing: cubicOut }}
        out:scale={{ duration: 250, start: 0.95, easing: cubicInOut }}
        class="flex gap-5 p-5 bg-sidebar/60 backdrop-blur-3xl rounded-[2.5rem] border border-white/10 shadow-2xl items-center will-change-transform"
        style="backface-visibility: hidden; transform-style: preserve-3d;"
    >
        {#each Object.values(WorkSectionIDs) as id, i}
            {@const visual = WorkSectionVisual[id]}
            {@const isActive = workStack.mode === id}
            {@const marbleColor = visual.color ? colorMap[visual.color] : '#ffffff'}
            
            <button
                onclick={() => selectMode(id)}
                in:fade={{ delay: i * 40, duration: 300 }}
                class="relative group w-12 h-12 rounded-full flex items-center justify-center transition-all duration-700 {isActive ? 'scale-110' : 'hover:scale-105 active:scale-95'}"
                style="--this-color: {marbleColor}"
            >
                <div class="absolute inset-0 rounded-full overflow-hidden border border-white/20 shadow-2xl bg-black/40 transition-transform duration-700 group-hover:rotate-360">
                    <div class="absolute inset-0 opacity-40 group-hover:opacity-100 blur-xl scale-150 animate-swirl animate-glint" style="background: radial-gradient(circle at 30% 30%, var(--this-color) 0%, transparent 70%);"></div>
                    
                    <div class="absolute -inset-full group-hover:animate-glint opacity-0 group-hover:opacity-100"
                        style="background: linear-gradient(45deg, transparent 45%, white 50%, transparent 55%); filter: blur(2px);">
                    </div>

                    <div class="absolute top-1 left-3 right-3 h-1/2 bg-linear-to-b from-white/30 to-transparent rounded-t-full"></div>
                    <div class="absolute inset-0 transition-colors duration-500 {isActive ? 'bg-(--this-color)/20' : 'group-hover:bg-(--this-color)/10'}"></div>
                </div>
                
                <visual.icon 
                    size={24} 
                    class="relative z-10 transition-all duration-500 group-hover:scale-110
                    {isActive ? 'text-white drop-shadow-[0_0_8px_var(--this-color)]' : 'text-white/40 group-hover:text-white'}" 
                />
            </button>
        {/each}
    </div>
</FloatingPanel>

<style>
    @keyframes swirl { 0% { transform: scale(1.5) rotate(0deg); } 100% { transform: scale(1.5) rotate(360deg); } }
    @keyframes glint {
        0% { transform: translateX(-100%) translateY(-100%); }
        100% { transform: translateX(100%) translateY(100%); }
    }
    .animate-swirl { animation: swirl 10s infinite linear; }
    .animate-glint { animation: glint 2.5s infinite ease-in-out; }
</style>