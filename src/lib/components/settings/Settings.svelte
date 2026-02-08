<script lang="ts">
    import { settingsStack } from "$lib/runes/settings.svelte";
    import { toggleUIStack } from "$lib/runes/toggle-ui.svelte";
    import { scale } from "svelte/transition";
    import { Settings, Globe, Moon } from "lucide-svelte";

    function close() { toggleUIStack.settingsOpen = false; }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === 'Escape') close();
    }
</script>

{#if toggleUIStack.settingsOpen}
    <dialog 
        class="fixed inset-0 z-100 flex items-center justify-center bg-black/40 backdrop-blur-sm border-none w-full h-full cursor-default"
        onclick={close}
        onkeydown={handleKeydown}
        aria-label="Cerrar configuración"
    >
        <dialog 
            open
            class="relative w-125 bg-main border border-border-subtle rounded-3xl shadow-2xl overflow-hidden p-0 m-0 block text-text"
            onclick={e => e.stopPropagation()}
            in:scale={{ start: 0.95, duration: 200 }}
        >
            <form method="dialog" class="flex flex-col h-full">
                <header class="p-6 border-b border-border-subtle flex items-center gap-3 bg-sidebar/30">
                    <Settings size={20} class="text-accent" />
                    <h2 class="text-sm font-bold uppercase tracking-widest">Ajustes</h2>
                </header>

                <div class="p-8 space-y-8 grid grid-rows gap-6 ">
                    <fieldset class="space-y-4 border-none p-0 m-0">
                        <legend class="flex items-center gap-2 text-[11px] font-bold text-text/40 uppercase">
                            <Moon size={14} /> <span>Apariencia</span>
                        </legend>
                        <div class="grid grid-cols-2 gap-3">
                            {#each ['light', 'dark'] as t}
                                <button 
                                    type="button"
                                    onclick={() => settingsStack.update({ theme: t as any })}
                                    class="py-3 px-4 rounded-xl border transition-all text-xs font-medium cursor-pointer
                                    {settingsStack.current.theme === t ? 'border-accent bg-accent/5 text-accent' : 'border-border-subtle hover:border-text/20'}">
                                    Modo {t === 'light' ? 'Claro' : 'Oscuro'}
                                </button>
                            {/each}
                        </div>
                    </fieldset>

                    <label class="block space-y-4">
                        <span class="flex items-center gap-2 text-[11px] font-bold text-text/40 uppercase">
                            <Globe size={14} /> <span>Idioma</span>
                        </span>
                        <select 
                            value={settingsStack.current.language}
                            onchange={e => settingsStack.update({ language: e.currentTarget.value as any })}
                            class="w-full p-3 rounded-xl bg-sidebar border border-border-subtle text-xs outline-none focus:border-accent cursor-pointer"
                        >
                            <option value="es">Español</option>
                            <option value="en">English</option>
                        </select>
                    </label>
                </div>

                <footer class="p-4 bg-sidebar/30 flex justify-end">
                    <button 
                        type="button"
                        onclick={close} 
                        class="px-6 py-2 bg-text text-main rounded-full text-xs font-bold hover:opacity-90 cursor-pointer"
                    >
                        Listo
                    </button>
                </footer>
            </form>
        </dialog>
    </dialog>
{/if}