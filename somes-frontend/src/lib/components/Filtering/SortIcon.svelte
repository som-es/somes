<script lang="ts">
	interface Props {
		sortOrder: 'relevance' | 'Desc' | 'Asc';
	}
	let { sortOrder }: Props = $props();
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	class="size-[18px]"
	viewBox="0 0 24 24"
	fill="none"
	stroke="currentColor"
	stroke-width="2.5"
	stroke-linecap="round"
	stroke-linejoin="round"
>
	<polyline points="8 9 12 5 16 9" class:opacity-25={sortOrder === 'Desc'} />
	<polyline points="8 15 12 19 16 15" class:opacity-25={sortOrder === 'Asc'} />
</svg>
