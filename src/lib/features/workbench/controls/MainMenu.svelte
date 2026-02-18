<script lang="ts">
    import { open } from '@tauri-apps/plugin-dialog';
    
    // Runes de estado global
    import { toggleUIStack } from "$lib/core/runes/toggle-ui.svelte";
    import { workStack } from "$lib/core/runes/work-modes.svelte";
    import { fileStack } from "$lib/features/filesystem/files-runes.svelte";
    
    // Componentes de UI
    import FloatingPanel from "$lib/components/common/low-level/FloatingPanel.svelte";
    import Menu from "$lib/components/common/Menu.svelte";
    import { Menu as MenuIcon, FolderOpen, FileCode, Save } from "lucide-svelte";
    import type { GenericFile } from '$lib/core/types/tab';
  import { ActivityIDs } from '$lib/core/constants/work-config';

    let anchor = $state<HTMLElement | null>(null);
    let isOpen = $state(false);

    /**
     * Abre el diálogo nativo para seleccionar una carpeta y carga el árbol de archivos.
     */
    async function handleOpenFolder() {
        const selected = await open({
            directory: true,
            multiple: false,
            title: 'Mod0 | Seleccionar Carpeta del Proyecto'
        });

        if (selected && typeof selected === 'string') {
            await fileStack.openFolder(selected);
            workStack.setActivity(ActivityIDs.FILES); 
            isOpen = false;
        }
    }

    /**
     * Abre el diálogo nativo para un archivo individual y lo inyecta en el Workbench.
     */
    async function handleOpenFile() {
        const selected = await open({
            directory: false,
            multiple: false,
            title: 'Mod0 | Abrir Archivo'
        });

        if (selected && typeof selected === 'string') {
            const fileName = selected.split(/[\\/]/).pop() || selected;
            const fileExtension = selected.split('.').pop() || 'unknown';

            // Creamos un nodo básico. 
            // El casting 'as GenericFile' es seguro aquí porque openFile 
            // se encargará de transformarlo en CodeTab o AssetTab.
            const fileNode = {
                id: selected,
                path: selected,
                name: fileName,
                label: fileName,
                is_dir: false,
                extension: fileExtension,
                icon: FileCode,
                color: 'text-text/60'
            } as GenericFile;

            await fileStack.openFile(fileNode);
            workStack.setActivity('FILES');
            isOpen = false;
        }
    }

    /** Definición de la estructura del menú principal */
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
        class="p-1.5 rounded-md transition-all duration-200 {isOpen ? 'text-accent bg-accent/10' : 'text-text/60 hover:text-text hover:bg-white/5'}"
        aria-label="Menú Principal"
    >
        <MenuIcon size={18} />
    </button>

    {#if isOpen}
        <FloatingPanel 
            anchorEl={anchor} 
            onclose={() => isOpen = false} 
            offset={10}
        >
            <Menu items={menuItems} onclose={() => isOpen = false} />
        </FloatingPanel>
    {/if}
</div>