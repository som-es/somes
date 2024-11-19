<script lang="ts">
	import { errorToNull, walo_questions } from '$lib/api';
	import Container from '$lib/components/Layout/Container.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import ListBoxItemWalo from '$lib/components/Walo/ListBoxItemWalo.svelte';
	import ResultChart from '$lib/components/Walo/ResultChart.svelte';
	import ResultChartReactive from '$lib/components/Walo/ResultChartReactive.svelte';
	import { partyToColor } from '$lib/partyColor';
	import type { WaloQuestion } from '$lib/types';
	import { ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	let waloQuestions: WaloQuestion[];
	let allTickedQuestions: string[][];
	let twoTimesWeightQuestions: string[][];
	let started: boolean = false;
	let showResults: boolean = false;

	const partyIdxToString = ["SPÖ", "GRÜNE", "NEOS", "FPÖ", "ÖVP"];

	let justificationsOrderCache: [string | null, number][][];
	onMount(async () => {
		const fetchedWaloQuestions = errorToNull(await walo_questions());
		if (fetchedWaloQuestions) {
			waloQuestions = fetchedWaloQuestions;
			allTickedQuestions = Array.from({ length: waloQuestions.length }, () => []);
			twoTimesWeightQuestions = Array.from({ length: waloQuestions.length }, () => []);
			justificationsOrderCache = Array.from({ length: waloQuestions.length }, () => []);
		}
	});

	let idx = 0;

	function shuffleArray<T>(array: T[]): T[] {
		const shuffledArray = array.slice();
		for (let i = shuffledArray.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[shuffledArray[i], shuffledArray[j]] = [shuffledArray[j], shuffledArray[i]];
		}
		return shuffledArray;
	}

	let justifications: [string | null, number][] = [];

	let valueMultiple: string[] = [];
	let twoWeightGroup: string[] = [];

	const updateJustifications = (idx: number) => {
		if (justificationsOrderCache[idx].length > 0) {
			justifications = justificationsOrderCache[idx].slice();
			return;
		}

		const question = waloQuestions[idx];
		justifications = shuffleArray([
			[question.spoe_justification, 0],
			[question.gruene_justification, 1],
			[question.neos_justification, 2],
			[question.fpoe_justification, 3],
			[question.oevp_justification, 4]
		]);
		justificationsOrderCache[idx] = justifications.slice();
	};

	$: if (waloQuestions || idx) {
		updateJustifications(idx);
	}
	let partyPoints: number[] = [];
	$: if (allTickedQuestions) {
		valueMultiple = allTickedQuestions[idx];
		twoWeightGroup = twoTimesWeightQuestions[idx];
		console.log(valueMultiple);
	}

	$: if (allTickedQuestions || twoTimesWeightQuestions ) {
		partyPoints = sumPartyPoints();
	}


	const sumPartyPoints = () => {
		const partyPoints = [0, 0, 0, 0, 0];
		twoTimesWeightQuestions.forEach((twoTimesWeightQuestion) => {
			twoTimesWeightQuestion.forEach((partyIdx) => {
				partyPoints[+partyIdx] -= 1;
			});
		});
		allTickedQuestions.forEach((tickedQuestionParties, i) => {
			tickedQuestionParties.forEach((partyIdx) => {
				const add = twoTimesWeightQuestions[i].includes(partyIdx) ? 2 : 0;
				partyPoints[+partyIdx] += 1 + add;
			});
		});
		return partyPoints;
	};

	// started = true;
	// showResults = true;
</script>

<div class="p-4 max-w-[800px] w-full mx-auto">
	{#if waloQuestions && started && !showResults}
		<h1 class="font-bold text-xl sm:text-4xl">{waloQuestions[idx].question_statement}</h1>
		<h4 class="text-lg sm:text-xl mt-2">{waloQuestions[idx].erklaerbaer}</h4>
		<br>
		{#if idx == 0}
			<div>Wähle alle zutreffenden Aussagen aus. 
			Mit "2x" können Aussagen doppelt gewichtet werden, dann werden zwei Punkte für die Partei addiert, auch wenn sie nicht ausgewählt wurden, dann wird einer abgezogen.</div>	
		{/if}
		<div class="flex justify-center mt-4">
			<div class="flex flex-wrap flex-row justify-center items-center gap-1">
				<ListBox class="reasons" multiple>
					{#each justifications as justification}
						<ListBoxItemWalo
							bind:partyGroup={valueMultiple}
							bind:twoTimeWeightsGroup={twoWeightGroup}
							bind:justification
						></ListBoxItemWalo>
					{/each}
				</ListBox>
			</div>
		</div>
		<div class=" sm:hidden mt-3 mb-5">
			Frage {idx +1} von {waloQuestions.length}	
		</div>
		<div class="flex justify-between mt-5">
			{#if idx > 0}
				<SButton class="mb-5 bg-tertiary-500 text-black" on:click={() => (idx -= 1)}>
					Vorherige Frage
				</SButton>
			{:else}
				<div></div>
			{/if}

			<div class="max-sm:hidden mt-3">
				Frage {idx +1} von {waloQuestions.length}	
			</div>

			{#if idx + 1 < waloQuestions.length}
				<SButton class="mb-5 bg-tertiary-500 text-black" on:click={() => (idx += 1)}>
					Nächste Frage
				</SButton>
			{:else}
				<SButton
					class="mt-5 mb-5 bg-tertiary-500 text-black"
					on:click={() => {
						partyPoints = sumPartyPoints();
						showResults = true;
					}}>Ergebnis</SButton
				>
			{/if}
		</div>
	{:else if !started}
		<h1 class="font-bold text-7xl">Der <span class=" italic">somes</span> Wahlhelfer</h1>
		<h4 class="text-xl">
			bietet zusätzliche Unterstützung bei der Wahlentscheidung, basierend auf tatsächlichen Abstimmungen und
			Reden im Nationalrat, statt auf Versprechen und Parolen vor der Wahl.
		</h4>
		<br />
		Beim Start des Wahlhelfers können mehrere, aber auch keine Aussagen ausgewählt werden, die den eigenen
		Ansichten entsprechen. Jede Aussage, auch jene, die nicht zu einem passen, kann doppelt gewichtet
		werden. Wird keine Auswahl getroffen, bedeutet dies "überspringen" oder "auslassen". Die Aussagen
		beginnen entweder mit "Ja" oder "Nein". Das widerspiegelt das Abstimmungsverhalten im Nationalrat.
		Falls Antworten doppelt vorkommen, haben Parteien diesselbe Begründung in den Reden erwähnt.

		<br />
		<br />
		<span class="font-bold">Disclaimer:</span> Die Aussagen wurden aus den Reden im Nationalrat
		extrahiert. Deshalb stehen die Begründungen immer nur aus den Meinungen von einzelnen
		Abgeordneten aus der Fraktion. Die Aussagen und damit die Begründungen in den Reden wurden
		keinem Faktencheck unterzogen. Unterbewusster Bias sowie Fehler können nicht ausgeschlossen
		werden. Der somes Wahlhelfer bietet lediglich eine zusätzliche Orientierungshilfe basierend
		auf in der Vergangenheit getätigte Aussagen im Nationalrat. Die Themen wurden von dem somes Team ausgewählt. Nicht alle Begründungen aus den
		Reden wurden eingebaut. Die eigene Auswahl bleibt lokal am Gerät und wird nicht gespeichert. 
		<br />

		<SButton
			on:click={() => (started = true)}
			class="float-right mt-5 mb-5 bg-tertiary-500 text-black">Starten</SButton
		>
	{:else if waloQuestions && showResults && started}
		<span class="font-bold text-3xl"> Ergebnis </span>
		<!-- <ResultChart dataNoParty={[19, 9, 3, 10, 1]}></ResultChart> -->
		<ResultChartReactive dataNoParty={partyPoints}></ResultChartReactive>
		<span class="font-bold text-3xl my-4"> Aussagen der Parteien </span>
		
		<div class="my-3">
			{#each waloQuestions as waloQuestion, idx}
				<h1 class="font-bold text-lg sm:text-2xl">{waloQuestion.question_statement}</h1>
				<h4 class="text-sm sm:text-lg mt-2">{waloQuestion.erklaerbaer}</h4>
				<SButton class="bg-tertiary-400 mt-2 text-black"><a href="{waloQuestion.somes_link}" target="_blank">Abstimmung auf somes</a></SButton>
				<div class="flex justify-center mt-4">
					<div class="flex flex-wrap flex-row justify-center items-center gap-1">
						<ListBox class="reasons" multiple>
							{#each justificationsOrderCache[idx] as justification}
								<ListBoxItemWalo
									bind:partyGroup={allTickedQuestions[idx]}
									bind:twoTimeWeightsGroup={twoTimesWeightQuestions[idx]}
									bind:justification
									partyName={partyIdxToString[justification[1]]}
									color={partyToColor(partyIdxToString[justification[1]])}
								></ListBoxItemWalo>
							{/each}
						</ListBox>
					</div>
				</div>

			{/each}
		</div>
	{/if}
</div>

<style>
	:global(.reasons) {
		flex-basis: 89%;
	}
	:global(.weights) {
		flex-basis: 10%;
	}
</style>
