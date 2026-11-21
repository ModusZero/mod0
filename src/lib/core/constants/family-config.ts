import { Bot, Brain, Globe, ListTodo, Pencil, Search, Terminal, Eye, Zap } from '@lucide/svelte';
import type { Component } from 'svelte';
import type { ActionFamily } from '#lib/core/types/session';

/** Icono, color y clave de i18n de cada familia de acción de la IA. */
export const FamilyVisual: Record<ActionFamily, { icon: Component<any>; text: string; label: string }> = {
    read: { icon: Eye, text: 'text-f-read', label: 'family.read' },
    write: { icon: Pencil, text: 'text-f-write', label: 'family.write' },
    exec: { icon: Terminal, text: 'text-f-exec', label: 'family.exec' },
    search: { icon: Search, text: 'text-f-search', label: 'family.search' },
    web: { icon: Globe, text: 'text-f-web', label: 'family.web' },
    agent: { icon: Bot, text: 'text-f-agent', label: 'family.agent' },
    plan: { icon: ListTodo, text: 'text-f-plan', label: 'family.plan' },
    think: { icon: Brain, text: 'text-f-think', label: 'family.think' },
    other: { icon: Zap, text: 'text-f-other', label: 'family.other' },
};
