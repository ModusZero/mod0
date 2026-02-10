<script lang="ts">
    import { open } from '@tauri-apps/plugin-dialog';
    
    // Runes de estado global
    import { toggleUIStack } from "$lib/core/runes/toggle-ui.svelte";
    import { workStack } from "$lib/core/runes/work-modes.svelte";
    import { fileStack } from "$lib/core/runes/filesystem.svelte";
    
    // Componentes de UI
    import FloatingPanel from "$lib/components/common/low-level/FloatingPanel.svelte";
    import Menu from "$lib/components/common/Menu.svelte";
    import { Menu as MenuIcon, FolderOpen, FileCode, Save } from "lucide-svelte";

    let anchor = $state<HTMLElement | null>(null);
    let isOpen = $state(false);

    /**
     * Abre el diálogo nativo para carpetas y delega al fileStack
     */
    async function handleOpenFolder() {
        const selected = await open({
            directory: true,
            multiple: false,
            title: 'Seleccionar Carpeta del Proyecto'
        });

        if (selected && typeof selected === 'string') {
            await fileStack.openFolder(selected);
            // El workStack se encarga de abrir el sidebar y poner la actividad FILES
            workStack.setActivity('FILES'); 
            isOpen = false;
        }
    }

    /**
     * Abre el diálogo nativo para archivos y delega al fileStack
     */
    async function handleOpenFile() {
        const selected = await open({
            directory: false,
            multiple: false,
            title: 'Abrir Archivo'
        });

        if (selected && typeof selected === 'string') {
            const fileName = selected.split(/[\\/]/).pop() || selected;
            // Usamos la lógica ya definida en nuestro Rune
            await fileStack.openFile({
                name: fileName,
                path: selected,
                is_dir: false
            });
            workStack.setActivity('FILES');
            isOpen = false;
        }
    }

    const menuItems = [
        { 
            label: "Archivo", 
            children: [
                { 
                    label: "Abrir Archivo...", 
                    icon: FileCode, 
                    shortcut: "Ctrl+O", 
                    action: handleOpenFile 
                },
                { 
                    label: "Abrir Carpeta...", 
                    icon: FolderOpen, 
                    shortcut: "Ctrl+Shift+O", 
                    action: handleOpenFolder 
                },
                { 
                    label: "Guardar", 
                    icon: Save, 
                    shortcut: "Ctrl+S", 
                    action: () => fileStack.saveCurrentFile() 
                }
            ]
        },
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
        class="p-1.5 hover:bg-white/10 rounded-md transition-colors {isOpen ? 'text-text bg-white/10' : 'text-text/60 hover:text-text'}"
        aria-label="Abrir Menú Principal"
    >
        <MenuIcon size={18} />
    </button>

    {#if isOpen}
        <FloatingPanel anchorEl={anchor} onclose={() => isOpen = false} offset={10}>
            <Menu items={menuItems} onclose={() => isOpen = false} />
        </FloatingPanel>
    {/if}
</div>