<script lang="ts">
	import { delegates, parties } from "$lib/api/api";
	import { isHasError } from "$lib/api/api";
	import type { Delegate, Party } from "$lib/types";
	import { onMount } from "svelte";

	let dels: Delegate[] = [];
	let prts: Party[] = [];
	let minDate: string = "";
	let maxDate: string = new Date().toISOString().split('T')[0];

	onMount(async function () {
		const austrianDelegates = await delegates();
		if (!isHasError(austrianDelegates)) {
			dels = austrianDelegates.filter((delegate) => delegate.council === "nr");
		}

		const partiesResult = await parties();
		if (!isHasError(partiesResult)) {
			prts = partiesResult;
		}
	});
</script>

<!-- TODO: do it like the gmail web client :) -->
{#if dels && prts}
	<div class="flex flex-col gap-[1rem]">
		<div class="flex flex-row h-18 gap-[2vw]">
			<div class="flex flex-row h-[2.5rem] mt-auto gap-[0.5vw]">
				<input
					class="input rounded-lg p-2"
					type="text"
					placeholder="Suchbegriff eingeben"
				/>
				<button type="button" class="btn variant-filled">Suchen</button>
			</div>
			<label class="label ml-auto w-1/6">
				<span>Sortieren nach</span>
				<select class="select">
					<option value="1">Relevanz</option>
					<option value="2">Benutzerwertung</option>
					<option value="3">Neueste</option>
				</select>
			</label>
		</div>
		<div class="flex flex-row h-18 gap-[2vw]">
			<label class="label">
				<span>Abgeordnete*r</span>
				<select class="select">
					<option value="">Alle</option>
					{#each dels as del}
						<option value={del.id}>{del.name}</option>
					{/each}
				</select>
			</label>
			<label class="label">
				<span>Partei</span>
				<select class="select">
					<option value="">Alle</option>
					{#each prts as party}
						<option value={party.name}>{party.name}</option>
					{/each}
				</select>
			</label>
			<label class="label">
				<span>Zeitraum</span>
				<div class="flex flex-row h-10 gap-[0.5vw]">
					<input class="input rounded-lg p-2" title="Von Datum" type="date" bind:value={minDate} />
					<div class="mt-2">
						bis
					</div>
					<input class="input rounded-lg p-2" title="Bis Datum" type="date" bind:value={maxDate} />
				</div>
			</label>
		</div>
	</div>
	<hr class="opacity-100 mt-auto" />
{:else}
	<p class="loading">Laden...</p>
{/if}