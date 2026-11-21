<script lang="ts">
    import { FileCode, X } from '@lucide/svelte';
    import { FamilyVisual } from '#lib/core/constants/family-config';
    import { t } from '#lib/core/i18n/utils.svelte';
    import { editorStack } from '#lib/core/runes/editor.svelte';
    import { graphStack } from '#lib/core/runes/graph.svelte';
    import type { ThoughtNode } from '#lib/core/types/session';
    import { basename } from '#lib/core/utils/format';

    interface Props {
        node: ThoughtNode;
    }

    let { node }: Props = $props();

    const visual = $derived(FamilyVisual[node.family]);
    const canOpen = $derived(!!node.target && (node.family === 'read' || node.family === 'write'));
</script>

<aside class="flex w-80 shrink-0 flex-col border-l border-border-subtle bg-sidebar">
    <header class="flex h-10 shrink-0 items-center justify-between border-b border-border-subtle pr-1.5 pl-3">
        <span class="flex items-center gap-2 text-meta font-semibold tracking-wide uppercase {visual.text}">
            <visual.icon size={13} />
            {node.tool ?? t(`node.${node.kind}`)}
        </span>
        <button class="icon-btn size-6" onclick={() => (graphStack.selectedId = null)} aria-label="Cerrar"
            ><X size={13} /></button
        >
    </header>

    <div class="min-h-0 flex-1 space-y-4 overflow-y-auto p-3">
        <p class="font-medium break-words">{node.label}</p>

        {#if node.intent}
            <section>
                <h4 class="label">{t('node.intent')}</h4>
                <p class="text-muted italic">{node.intent}</p>
            </section>
        {/if}

        {#if node.detail}
            <section>
                <h4 class="label">{t('node.detail')}</h4>
                <pre
                    class="max-h-72 overflow-auto rounded-md bg-main p-2 font-mono text-meta whitespace-pre-wrap">{node.detail}</pre>
            </section>
        {/if}

        {#if node.target}
            <section>
                <h4 class="label">{t('node.target')}</h4>
                <p class="font-mono text-meta break-all text-muted">{node.target}</p>
                {#if canOpen}
                    <button class="btn mt-2" onclick={() => editorStack.open(node.target!)}>
                        <FileCode size={13} />
                        {t('node.open')}
                        {basename(node.target)}
                    </button>
                {/if}
            </section>
        {/if}

        <dl class="grid grid-cols-2 gap-2 font-mono text-meta text-muted">
            <div>
                <dt class="text-faint">{t('node.status')}</dt>
                <dd>{node.status}</dd>
            </div>
            <div>
                <dt class="text-faint">{t('node.time')}</dt>
                <dd>{new Date(node.ts).toLocaleTimeString()}</dd>
            </div>
            {#if node.durationMs !== null}<div>
                    <dt class="text-faint">{t('node.duration')}</dt>
                    <dd>{(node.durationMs / 1000).toFixed(1)}s</dd>
                </div>{/if}
        </dl>
    </div>
</aside>
