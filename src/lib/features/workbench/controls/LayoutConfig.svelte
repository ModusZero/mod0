<script lang="ts">
	import { toggleUIStack } from "$lib/core/runes/toggle-ui.svelte";
	import { PanelLeft, PanelBottom, LayoutTemplate } from "lucide-svelte";

	const btnClass = "p-1.5 rounded-md transition-all duration-200 hover:bg-white/10";
</script>

<div class="flex items-center gap-1 bg-white/5 p-1 rounded-lg border border-white/5">
	<button 
		onclick={() => toggleUIStack.toggleSidebar()}
		class="{btnClass} {toggleUIStack.sidebarOpen ? 'text-accent' : 'text-text/40'}"
	>
		<PanelLeft size={16} />
	</button>
	<button 
		onclick={() => toggleUIStack.toggleTerminal()}
		class="{btnClass} {toggleUIStack.terminalOpen ? 'text-accent' : 'text-text/40'}"
	>
		<PanelBottom size={16} />
	</button>
	<button class="{btnClass} text-text/40 hover:text-text">
		<LayoutTemplate size={16} />
	</button>
</div>