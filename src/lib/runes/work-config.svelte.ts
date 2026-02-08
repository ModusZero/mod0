import { 
    WorkSectionIDs, 
    ActivityByWork 
} from '$lib/constants/ui-config';
import type { 
    WorkSectionID, 
    ActivityID 
} from '$lib/types/work';
import { toggleUIStack } from './toggle-ui.svelte';

/**
 * UIStack centraliza el estado de navegación global.
 * Utiliza el patrón de "Registry" para evitar condicionales innecesarios.
 */
class WorkStack {
    /** * @type {WorkSectionID} Sección maestra actual (Blueprint, Forge, etc.)
     */
    mode: WorkSectionID = $state<WorkSectionID>(WorkSectionIDs.BLUEPRINT);
    
    /** * @type {ActivityID} Identificador de la actividad activa en el Sidebar
     */
    activeActivity: ActivityID = $state<ActivityID>(ActivityByWork[WorkSectionIDs.BLUEPRINT][0]);

    /** * @type {ActivityID[]} Lista de actividades disponibles para la sección actual
     */
    currentActivities: ActivityID[] = $state<ActivityID[]>(ActivityByWork[WorkSectionIDs.BLUEPRINT]);

    /**
     * Transiciona el IDE a un nuevo modo de trabajo.
     * Reinicia automáticamente la actividad a la primera disponible del nuevo modo.
     * * @param {WorkSectionID} newMode - El identificador de la sección deseada.
     */
    setMode(newMode: WorkSectionID) {
        if (this.mode === newMode) return;

        this.mode = newMode;
        this.currentActivities = ActivityByWork[newMode];

        if (this.currentActivities && this.currentActivities.length > 0) {
            this.activeActivity = this.currentActivities[0];
        }

    }

    /**
     * Cambia la actividad activa del sidebar y asegura que sea visible.
     * * @param {ActivityID} activity - ID de la actividad a mostrar.
     */
    setActivity(activity: ActivityID) {
        this.activeActivity = activity;
        toggleUIStack.sidebarOpen = true;
    }
}

/**
 * Instancia única del estado de UI para toda la aplicación.
 */
export const workStack = new WorkStack();