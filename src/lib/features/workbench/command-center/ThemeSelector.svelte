<script lang="ts">
    import { fly } from 'svelte/transition';
    import { Sun, Moon, Monitor, ChevronLeft } from "lucide-svelte";

    let { onSelect, onBack } = $props<{ onSelect: (id: string) => void, onBack: () => void }>();

    const themes = [
        { id: 'light', label: 'Claro', icon: Sun },
        { id: 'dark', label: 'Oscuro', icon: Moon },
        { id: 'system', label: 'Sistema', icon: Monitor }
    ];
</script>

<div class="w-full">
    <button onclick={onBack} class="w-full flex items-center gap-3 px-4 py-2 hover:bg-white/5 text-[10px] opacity-50 hover:opacity-100 transition-opacity border-b border-white/5 mb-1">
        <ChevronLeft size={12} /> Volver a comandos
    </button>
    {#each themes as theme, i}
        <button 
            in:fly={{ y: 10, duration: 300, delay: i * 30 }}
            onclick={() => onSelect(theme.id)}
            class="w-full flex items-center gap-3 px-4 py-2.5 hover:bg-accent/10 transition-colors group"
        >
            <theme.icon size={14} class="opacity-50 group-hover:text-accent" />
            <span class="text-[11px] text-text/70 group-hover:text-text">{theme.label}</span>
        </button>
    {/each}
</div>