<script lang="ts">
	import { errorToNull, justPost } from "$lib/api/api";
	import Container from "$lib/components/Layout/Container.svelte";
	import DelegateBarChartControl from "$lib/components/Statistics/DelegateBarChartControl.svelte";
	import SquarePoliticalSpectrum from '$lib/components/Delegates/Spectrum/SquarePoliticalSpectrum.svelte';
	import type { DelegateData } from '$lib/types';

	type DelegateIsLeft = {
		delegate_name: string;
		delegate_party: string;
		left_score: number;
	};

	type DelegateIsLiberal = {
		delegate_name: string;
		delegate_party: string;
		liberal_score: number;
	};

	type PoliticalPosition = {
	//	delegate_party: string;
		is_left: number;
		is_not_left: number;
		is_liberal: number;
		is_not_liberal: number;
		neutral_count: number;
	}

	let tableHeightDelegate = 0;

	const calculateTableHeight = (numRows: number): number => {
		const maxHeight = 3100;
		const maxRows = 100;

		const height = (numRows / maxRows) * maxHeight;

		return Math.min(height, maxHeight);
	};

	const delegateSimpleIsLeft = async (gp: string | null, gender: string | null, isDesc: boolean | true, normalized: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateIsLeft[]>('is_left_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		})) ?? [];

		console.log(response)

		tableHeightDelegate = calculateTableHeight(response.length);

		console.log(tableHeightDelegate)

		return response.map(val => {
			return {
				name: val.delegate_name,
				party: val.delegate_party,
				data: val.left_score
			};
		});
	}

	const delegateSimpleIsLiberal = async (gp: string | null, gender: string | null, isDesc: boolean | true, normalized: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateIsLiberal[]>('is_left_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		})) ?? [];

		console.log(response)

		tableHeightDelegate = calculateTableHeight(response.length);

		console.log(tableHeightDelegate)

		return response.map(val => {
			return {
				name: val.delegate_name,
				party: val.delegate_party,
				data: val.liberal_score
			};
		});
	}

	const partySimplePoliticalPosition = async (gp: string | null, gender: string | null, isDesc: boolean | true, normalized: boolean | true): Promise<PoliticalPosition[]> => {
		const response = errorToNull(await justPost<PoliticalPosition[]>('is_left_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		})) ?? [];

		console.log(response)

		return response.map(val => {
			return {
				delegate_id: null,
				is_left: val.is_left,
				is_not_left: val.is_not_left,
				is_liberal: val.is_liberal,
				is_not_liberal: val.is_not_liberal,
				neutral_count: val.neutral_count
			};
		});
	}

</script>

<Container>
	<DelegateBarChartControl height={tableHeightDelegate} id={0} delegateMakeRequest={delegateSimpleIsLeft} title="Links Rechts Wert pro Abgeordneten"/>
</Container>

<Container>
	<DelegateBarChartControl height={tableHeightDelegate} id={1} delegateMakeRequest={delegateSimpleIsLiberal} title="Liberal Autoritär Wert pro Abgeordneten"/>
</Container>

<SquarePoliticalSpectrum {delegate}	politicalPosition={partySimplePoliticalPosition} />


