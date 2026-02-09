export interface MenuItem {
    label: string;
    icon?: any;
    shortcut?: string;
    action?: () => void;
    danger?: boolean;
    children?: MenuItem[];
}