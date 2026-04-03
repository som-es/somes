<script lang="ts">
	interface Props {
		dateFrom: string;
		dateTo: string;
	}

	let { dateFrom = $bindable(), dateTo = $bindable() }: Props = $props();
</script>

<div class="mt-1 flex flex-col gap-3">
	<div class="flex gap-2 w-full justify-between">
		<div class="flex-1">
			<label for="date-from" class="block w-full text-sm font-semibold text-gray-600 dark:text-gray-50">Von</label>
			<input
				id="date-from"
				type="date"
				class="mt-1 w-full appearance-none rounded-lg border border-primary-300 dark:border-primary-400 bg-transparent px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary-400"
				bind:value={dateFrom}
			/>
		</div>
		<div class="flex-1">
			<label for="date-to" class="block w-full text-sm font-semibold text-gray-600 dark:text-gray-50">Bis</label>
			<input
				id="date-to"
				type="date"
				class="mt-1 w-full appearance-none rounded-lg border border-primary-300 dark:border-primary-400 bg-transparent px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary-400"
				bind:value={dateTo}
			/>
		</div>
	</div>
	{#if dateFrom || dateTo}
		<button
			class="cursor-pointer rounded-lg border border-primary-300 dark:border-primary-400 px-2 py-1 text-xs hover:bg-primary-300 dark:hover:bg-primary-400 md:text-sm"
			onclick={() => { dateFrom = ''; dateTo = ''; }}
		>
			Zurücksetzen
		</button>
	{/if}
</div>
