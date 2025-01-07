<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import TestChart from '$lib/components/Statistics/TestChart.svelte';
	import { onMount } from 'svelte';
	import { justPost } from '$lib/api';
	// import { Chart } from "frappe-charts"; C:\Schule4neu\DIPLO\somes\somes-svelte-rework\src\lib\api.ts

	// TypeScript Typen
	type DelegateSpeechTime = {
		delegate_name: string;
		delegate_party: string;
		total_speech_time: number;
	};

	let speechTimeData: DelegateSpeechTime[] = [];
	let error: string | null = null;

	// Funktion, um Daten von der API zu laden
	async function fetchSpeechTimeData() {
		const response = await justPost<DelegateSpeechTime[]>('speechtime_per_delegate', {
			legis_period: "XXVII",
			party: null,
			gender: null,
			is_desc: true});

		if ('error' in response) {
			// Fehlerbehandlung
			error = response.error;
			return;
		}

		// Erfolgreiche Antwort
		speechTimeData = response;
		error = null;
	}

	// Daten abrufen, wenn die Komponente geladen wird
	onMount(fetchSpeechTimeData);
</script>

<Container>
	<div class="entry bg-primary-200 dark:bg-primary-400 gap-3 flex flex-wrap">
		<div class="flex-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
			<h1 class="font-bold text-3xl">Statistiken</h1>
		</div>

		<div
			class="rounded-xl min-w-40 min-h-40 w-[20rem] h-[20rem] bg-primary-300 dark:bg-primary-500 flex items-center justify-center"
		>
			Sprachkomplexität
		</div>
		<div
			class="rounded-xl min-w-40 min-h-40 w-[20rem] h-[20rem] bg-primary-300 dark:bg-primary-500 flex items-center justify-center"
		>
			Aktivitätsscoring
		</div>
		<div
			class="rounded-xl min-w-40 min-h-40 w-[20rem] h-[20rem] bg-primary-300 dark:bg-primary-500 flex items-center justify-center"
		>
			Redezeit
		</div>
		<div
			class="rounded-xl min-w-40 min-h-40 w-[20rem] h-[20rem] bg-primary-300 dark:bg-primary-500 flex items-center justify-center"
		>
			Ordnungsrufe
		</div>
	</div>
</Container>

<div>
	{#if error}
		<div class="error">Fehler: {error}</div>
	{:else if speechTimeData.length === 0}
		<div>Loading...</div>
	{:else}
		<table>
			<thead>
			<tr>
				<th>Name</th>
				<th>Partei</th>
				<th>Gesamtredezeit (Sekunden)</th>
			</tr>
			</thead>
			<tbody>
			{#each speechTimeData as item}
				<tr>
					<td>{item.delegate_name}</td>
					<td>{item.delegate_party}</td>
					<td>{item.total_speech_time}</td>
				</tr>
			{/each}
			</tbody>
		</table>
	{/if}
</div>


<div>
	<TestChart dataNoParty={[7, 9, 5, 1, 2]}> </TestChart>
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}

	.flex-item {
		flex-basis: 100%;
	}

	.square-item {
		flex-basis: 50%;
	}
</style>
