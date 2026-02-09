<script lang="ts">
	import { toggleUIStack } from "$lib/core/runes/toggle-ui.svelte";
	import FloatingPanel from "$lib/components/common/low-level/FloatingPanel.svelte";
	import Menu from "$lib/components/common/Menu.svelte";
	import { Menu as MenuIcon } from "lucide-svelte";

	let anchor = $state<HTMLElement | null>(null);
	let isOpen = $state(false);

	const menuItems = [
		{ label: "Archivo", action: () => console.log("Archivo") },
		{ label: "Editar", action: () => console.log("Editar") },
		{ label: "Ver", action: () => console.log("Ver") },
		{ 
			label: "Terminal", 
			shortcut: "Ctrl+`",
			action: () => toggleUIStack.toggleTerminal() 
		},
		{ 
			label: "Configuración", 
			shortcut: "Ctrl+,",
			action: () => toggleUIStack.toggleSettings() 
		}
	];
</script>

<div class="relative">
	<button 
		bind:this={anchor}
		onclick={() => isOpen = !isOpen}
		class="p-1.5 hover:bg-white/10 rounded-md transition-colors text-text/60 hover:text-text"
	>
		<MenuIcon size={18} />
	</button>

	{#if isOpen}
		<FloatingPanel anchorEl={anchor} onclose={() => isOpen = false} offset={10}>
			<Menu items={menuItems} onclose={() => isOpen = false} />
		</FloatingPanel>
	{/if}
</div>