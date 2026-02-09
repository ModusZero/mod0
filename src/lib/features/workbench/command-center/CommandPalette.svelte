<script lang="ts">
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import FloatingPanel from "$lib/components/common/low-level/FloatingPanel.svelte";

	let { anchorEl, onclose }: { anchorEl: HTMLElement | null, onclose: () => void } = $props();

	// Placeholder de comandos
	const commands = [
		{ icon: 
            "📄", 
            label: "Go to File...", 
            shortcut: "Ctrl P", func: () => {} 
        },
        { icon: 
            "🔍", 
            label: "Search...", 
            shortcut: "Ctrl Shift F", func: () => {} 
        },
		{ icon: 
            "⚙️", 
            label: "Settings...", 
            shortcut: "Ctrl ,", func: () =>{} 
        },
	];
</script>

<FloatingPanel {anchorEl} {onclose} offset={12}>
	<div 
		in:fly={{ y: -10, duration: 250, easing: cubicOut }}
		class="w-125 bg-sidebar/90 backdrop-blur-2xl border border-white/10 rounded-xl shadow-2xl overflow-hidden py-2"
	>
		{#each commands as cmd}
			<button 
				class="w-full flex items-center gap-3 px-4 py-2.5 hover:bg-accent/20 transition-all group"
				onclick={() => {
                    cmd.func();
                    onclose();
                }}
			>
				<span class="text-sm">{cmd.icon}</span>
				<span class="flex-1 text-left text-[12px] text-text/70 group-hover:text-text">{cmd.label}</span>
				<span class="text-[10px] font-mono opacity-20 group-hover:opacity-50">{cmd.shortcut}</span>
			</button>
		{/each}
	</div>
</FloatingPanel>