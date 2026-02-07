import { 
    WorkSectionIDs, 
    ActivityByWork 
} from '$lib/constants/ui';
import type { 
    WorkSectionID, 
    ActivityID 
} from '$lib/types/ui';

/**
 * UIStack centraliza el estado de navegación global.
 * Utiliza el patrón de "Registry" para evitar condicionales innecesarios.
 */
class UIStack {
    /** * @type {WorkSectionID} Sección maestra actual (Blueprint, Forge, etc.)
     */
    mode: WorkSectionID = $state<WorkSectionID>(WorkSectionIDs.BLUEPRINT);
    
    /** * @type {ActivityID} Identificador de la actividad activa en el Sidebar
     */
    activeActivity: ActivityID = $state<ActivityID>(ActivityByWork[WorkSectionIDs.BLUEPRINT][0]);
    
    /** * @type {boolean} Estado de expansión del panel lateral
     */
    sidebarOpen: boolean = $state(true);

    /**
     * Propiedad derivada que devuelve las actividades válidas para el modo actual.
     * @returns {ActivityID[]}
     */
    get currentActivities(): ActivityID[] {
        return ActivityByWork[this.mode];
    }

    /**
     * Transiciona el IDE a un nuevo modo de trabajo.
     * Reinicia automáticamente la actividad a la primera disponible del nuevo modo.
     * * @param {WorkSectionID} newMode - El identificador de la sección deseada.
     */
    setMode(newMode: WorkSectionID) {
        if (this.mode === newMode) return;

        this.mode = newMode;
        
        // Selección inteligente de actividad:
        // Si el nuevo modo tiene actividades, seleccionamos la primera.
        const activities = ActivityByWork[newMode];
        if (activities && activities.length > 0) {
            this.activeActivity = activities[0];
        }
        
        this.sidebarOpen = true;
    }

    /**
     * Cambia la actividad activa del sidebar y asegura que sea visible.
     * * @param {ActivityID} activity - ID de la actividad a mostrar.
     */
    setActivity(activity: ActivityID) {
        this.activeActivity = activity;
        this.sidebarOpen = true;
    }

    /**
     * Alterna la visibilidad del sidebar para maximizar el área de trabajo.
     */
    toggleSidebar() {
        this.sidebarOpen = !this.sidebarOpen;
    }
}

/**
 * Instancia única del estado de UI para toda la aplicación.
 */
export const uiStack = new UIStack();