import { ActivityByWork, ActivityIDs, WorkSectionIDs } from '#lib/core/constants/work-config';
import type { ActivityID, WorkSectionID } from '#lib/core/types/work';

/** Actividad activa del IDE y la sección a la que pertenece. */
class WorkStack {
    activity = $state<ActivityID>(ActivityIDs.SESSIONS);

    section: WorkSectionID = $derived(
        (Object.keys(ActivityByWork) as WorkSectionID[]).find((section) =>
            ActivityByWork[section].includes(this.activity),
        ) ?? WorkSectionIDs.FORGE,
    );

    setActivity(activity: ActivityID) {
        this.activity = activity;
    }
}

export const workStack = new WorkStack();
