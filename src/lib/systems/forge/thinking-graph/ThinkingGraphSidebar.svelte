<script lang="ts">
    import { Check, Circle, Loader } from '@lucide/svelte';
    import StatusIcon from '#lib/components/common/StatusIcon.svelte';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { graphStack } from '#lib/core/runes/graph.svelte';
    import { sessionsStack } from '#lib/core/runes/sessions.svelte';

    const session = $derived(sessionsStack.focused);
    const plan = $derived(session?.plan ?? []);
    const planDone = $derived(plan.filter((item) => item.status === 'completed').length);

    /** Un resumen por turno para saltar rápidamente a cualquier momento de la sesión. */
    const turns = $derived.by(() => {
        const nodes = graphStack.nodes(session?.id ?? null);
        const byTurn = new Map<number, { turn: number; prompt: string; steps: number }>();

        for (const node of nodes) {
            const entry = byTurn.get(node.turn) ?? { turn: node.turn, prompt: '', steps: 0 };
            if (node.kind === 'prompt') entry.prompt = node.label;
            if (node.kind === 'tool') entry.steps++;
            byTurn.set(node.turn, entry);
        }
        return [...byTurn.values()].reverse();
    });
</script>

<div class="flex h-full flex-col">
    <div class="panel-title">{t('graph.sessions')}</div>
    <div class="max-h-[28%] shrink-0 overflow-y-auto px-1.5 pb-2">
        {#each sessionsStack.sessions as item (item.id)}
            <button
                class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left transition-colors {item.id ===
                session?.id
                    ? 'bg-active'
                    : 'hover:bg-hover'}"
                onclick={() => sessionsStack.focus(item.id)}
            >
                <StatusIcon status={item.status} size={12} />
                <span class="truncate">{item.name}</span>
            </button>
        {/each}
    </div>

    {#if plan.length > 0}
        <div class="panel-title border-t border-border-subtle">
            <span>{t('plan.title')}</span>
            <span class="font-mono normal-case">{planDone}/{plan.length}</span>
        </div>
        <div class="mx-3 mb-2 h-1 overflow-hidden rounded-full bg-hover">
            <div
                class="h-full bg-ok transition-[width]"
                style="width:{(planDone / plan.length) * 100}%"
            ></div>
        </div>
        <ul class="max-h-[30%] shrink-0 space-y-px overflow-y-auto px-1.5 pb-2">
            {#each plan as item, index (index)}
                <li
                    class="flex items-start gap-2 rounded-md px-2 py-1 {item.status === 'in_progress'
                        ? 'bg-accent-soft'
                        : ''}"
                >
                    {#if item.status === 'completed'}
                        <Check size={13} class="mt-0.5 shrink-0 text-ok" />
                    {:else if item.status === 'in_progress'}
                        <Loader size={13} class="anim-spin mt-0.5 shrink-0 text-st-working" />
                    {:else}
                        <Circle size={13} class="mt-0.5 shrink-0 text-faint" />
                    {/if}
                    <span
                        class="text-meta {item.status === 'completed'
                            ? 'text-faint line-through'
                            : item.status === 'in_progress'
                              ? 'text-text'
                              : 'text-muted'}"
                    >
                        {item.content}
                    </span>
                </li>
            {/each}
        </ul>
    {/if}

    <div class="panel-title border-t border-border-subtle">{t('graph.turns')}</div>
    <div class="min-h-0 flex-1 overflow-y-auto px-1.5 pb-2">
        {#each turns as turn (turn.turn)}
            <button
                class="row-hover flex w-full items-start gap-2 rounded-md px-2 py-1.5 text-left"
                onclick={() => (graphStack.jumpTurn = turn.turn)}
            >
                <span class="mt-px font-mono text-meta font-semibold text-accent">T{turn.turn}</span>
                <span class="min-w-0 flex-1">
                    <span class="line-clamp-2">{turn.prompt}</span>
                    <span class="font-mono text-meta text-faint">{turn.steps} {t('unit.steps')}</span>
                </span>
            </button>
        {:else}
            <p class="px-3 py-6 text-center text-meta text-faint">{t('graph.empty')}</p>
        {/each}
    </div>
</div>
