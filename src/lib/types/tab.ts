import type { ActivityID } from "./work";

export interface Tab {
    id: string;
    type: 'code' | 'other' | ActivityID;
    label: string;
    icon: any;
    color: string;
    content?: string; // Para código
}