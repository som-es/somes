<script lang="ts">
	import { isEntryNew } from '$lib/date';

	interface Props {
		date: string;
	}
	const { date }: Props = $props();
</script>

{#if isEntryNew(new Date(date))}
	<span class="badge bg-secondary-200 text-black">Neu</span>
{/if}
