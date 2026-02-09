<script lang="ts">
    import { settingsStack } from "$lib/features/settings/settings-runes.svelte";
    import lightLogo from "$lib/assets/logo-light.svg";
    import darkLogo from "$lib/assets/logo-dark.svg";
    import Omnibar from "$lib/features/workbench/command-center/Omnibar.svelte";
    import LayoutConfig from "$lib/features/workbench/controls/LayoutConfig.svelte";
    import MainMenu from "$lib/features/workbench/controls/MainMenu.svelte";
  	import GoTo from "$lib/features/workbench/tabs/GoTo.svelte";
</script>

<header class="h-11 border-b border-border-subtle flex items-center px-3 bg-sidebar/50 backdrop-blur-xl justify-between relative z-50">
	
	<div class="flex items-center gap-3 flex-1">
		<img 
			src={settingsStack.current.theme === 'dark' ? darkLogo : lightLogo} 
			alt="Logo" 
			class="h-5 w-5 opacity-80" 
		/>
        <div class="h-4 w-px bg-white/10 mx-1"></div>

        <MainMenu />
	</div>

    <GoTo />

	<div class="flex-1 flex justify-center">
		<Omnibar />
	</div>

	<div class="flex items-center gap-4 flex-1 justify-end">
		<LayoutConfig />
		<div class="flex items-center gap-2">
			<span class="text-[9px] opacity-20 font-black tracking-widest cursor-default">MOD0</span>
			<div class="w-2 h-2 rounded-full bg-accent shadow-[0_0_8px_var(--accent)] animate-pulse"></div>
            
		</div>
	</div>
</header>