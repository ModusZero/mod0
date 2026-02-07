import type { AppMode, ActivityType } from '$lib/types/editor';

/**
 * Definición de la correspondencia entre Modos y sus Actividades disponibles.
 */
const MODE_ACTIVITIES: Record<AppMode, ActivityType[]> = {
    blueprint: ['kanban', 'erd', 'flow'],
    forge: ['sandbox', 'agents', 'explorer'],
    pulse: ['graph', 'explorer', 'skills', 'git']
};

class UIStack {
    /** @type {AppMode} Modo maestro actual */
    mode = $state<AppMode>('blueprint');
    
    /** @type {ActivityType} Actividad sidebar activa */
    activeActivity = $state<ActivityType>('kanban');
    
    /** @type {boolean} Control de visibilidad del sidebar */
    sidebarOpen = $state(true);

    /**
     * Obtiene las actividades disponibles para el modo actual.
     * @returns {ActivityType[]}
     */
    get currentActivities(): ActivityType[] {
        return MODE_ACTIVITIES[this.mode];
    }

    /**
     * Cambia el modo maestro y reinicia la actividad a la primera disponible del modo.
     * @param {AppMode} newMode 
     */
    setMode(newMode: AppMode) {
        this.mode = newMode;
        this.activeActivity = MODE_ACTIVITIES[newMode][0];
    }

    /**
     * Cambia la actividad del sidebar.
     * @param {ActivityType} activity 
     */
    setActivity(activity: ActivityType) {
        this.activeActivity = activity;
        this.sidebarOpen = true;
    }
}

export const uiStack = new UIStack();