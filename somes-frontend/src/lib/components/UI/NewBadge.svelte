<script lang="ts">
	import { isEntryNew } from '$lib/date';
	import { t } from '$lib/i18n/i18n.svelte';

	interface Props {
		date: string;
	}
	const { date }: Props = $props();
</script>

{#if isEntryNew(new Date(date))}
	<span class="badge bg-secondary-200 text-black">{t('ui.new')}</span>
{/if}
