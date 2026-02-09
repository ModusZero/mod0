<script lang="ts">
	let { count, variant = 'accent' }: { 
		count: number | string, 
		variant?: 'accent' | 'danger' | 'ghost' 
	} = $props();

	const themes = {
		accent: 'bg-accent/20 text-accent border-accent/30',
		danger: 'bg-red-500/20 text-red-400 border-red-500/30',
		ghost: 'bg-white/5 text-text/50 border-white/10'
	};
</script>

<span class="px-1.5 py-0.5 rounded-full border text-[9px] font-bold tabular-nums {themes[variant]}">
	{count}
</span>