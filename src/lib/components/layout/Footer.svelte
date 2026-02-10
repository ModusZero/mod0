<script lang="ts">
    import { settingsStack } from "$lib/features/settings/settings-runes.svelte";
    import { workStack } from "$lib/core/runes/work-modes.svelte";
</script>

<footer class="h-7 border-t border-border-subtle bg-sidebar flex items-center px-3 justify-between text-[10px]">
    <div class="flex gap-4 items-center text-text/60">
        <div class="flex items-center gap-1.5">
            <div class="w-2 h-2 rounded-full bg-green-500 shadow-[0_0_5px_rgba(34,197,94,0.5)]"></div>
            <span class="font-bold uppercase tracking-tighter">System Online</span>
        </div>
    </div>
    
    <div class="flex gap-4 items-center">
        <span class="bg-accent/10 text-accent px-2 py-0.5 rounded font-bold uppercase tracking-tighter">
            {workStack.mode}
        </span>
        <span class="text-text/40 uppercase font-bold">{settingsStack.current.theme} mode</span>
    </div>
</footer>