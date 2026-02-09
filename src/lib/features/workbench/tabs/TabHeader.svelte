<script lang="ts">
    import { X, Circle } from "lucide-svelte";

    let { 
        label = "main.js", 
        icon: Icon, 
        iconColor = "text-blue-400",
        active = true,
        isModified = false,
        onClose
    } = $props();
</script>

<div 
    class="h-9 flex items-center gap-2 px-3 border-r border-border-subtle transition-colors group relative min-w-30 max-w-50
    {active ? 'bg-main text-text' : 'bg-sidebar/30 text-text/40 hover:bg-sidebar/50'} cursor-default"
>
    {#if Icon}
        <Icon size={14} class={iconColor} />
    {/if}
    
    <span class="text-[11px] font-medium truncate flex-1 tracking-tight">
        {label}
    </span>

    <div class="flex items-center justify-center w-5 h-5">
        {#if isModified}
            <Circle size={8} fill="currentColor" class="text-accent group-hover:hidden" />
        {/if}
        <button 
            onclick={(e) => { e.stopPropagation(); onClose?.(); }}
            class="hidden group-hover:flex items-center justify-center hover:bg-text/10 rounded-md p-0.5 cursor-pointer"
        >
            <X size={12} />
        </button>
    </div>

    {#if active}
        <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-accent"></div>
    {/if}
</div>