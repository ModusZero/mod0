<script lang="ts">
	import { type Snippet } from 'svelte';
	import Modal from './low-level/Modal.svelte';

	let { title, variant = 'info', onclose, onconfirm, children, footer }: {
		title: string,
		variant?: 'info' | 'danger' | 'warning',
		onclose: () => void,
		onconfirm?: () => void,
		children: Snippet,
		footer?: Snippet
	} = $props();

	const colors = {
		info: 'text-accent',
		danger: 'text-red-500',
		warning: 'text-yellow-500'
	};
</script>

<Modal {onclose}>
	<div class="w-100 bg-main border border-border-subtle rounded-2xl overflow-hidden flex flex-col">
		<header class="p-4 border-b border-border-subtle bg-sidebar/50 flex items-center justify-between">
			<h3 class="text-xs font-bold uppercase tracking-widest {colors[variant]}">{title}</h3>
		</header>
		
		<div class="p-6 text-sm text-text/80 leading-relaxed">
			{@render children()}
		</div>

		<footer class="p-4 bg-sidebar/30 flex justify-end gap-3 border-t border-border-subtle">
			<button onclick={onclose} class="px-4 py-2 text-xs hover:bg-white/5 rounded-full transition-colors">
				Cancelar
			</button>
			{#if footer}
				{@render footer()}
			{:else}
				<button 
					onclick={() => { onconfirm?.(); onclose(); }}
					class="px-5 py-2 {variant === 'danger' ? 'bg-red-600' : 'bg-text text-main'} rounded-full text-xs font-bold hover:opacity-90"
				>
					Confirmar
				</button>
			{/if}
		</footer>
	</div>
</Modal>