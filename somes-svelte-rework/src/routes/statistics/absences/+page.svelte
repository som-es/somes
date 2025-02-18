<script lang="ts">
	import { errorToNull, justPost } from "$lib/api/api";
	import Container from "$lib/components/Layout/Container.svelte";
	import DelegateBarChartControl from "$lib/components/Statistics/DelegateBarChartControl.svelte";
	import type { DelegateData } from "$lib/types";


	type DelegateAbcenses = {
		delegate_name: string;
		delegate_gender: string;
		delegate_party: string;
		abcenses: number;
	};

	let tableHeightDelegate = 0;
	let tableHeightParty= 0;

	const calculateTableHeight = (numRows: number): number => {
		const maxHeight = 3100;
		const maxRows = 100;

		const height = (numRows / maxRows) * maxHeight;

		return Math.min(height, maxHeight);
	};

	const delegateSimpleAbcenses = async (gp: string | null, gender: string | null, isDesc: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateAbcenses[]>('absences_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		})) ?? [];

		console.log(response)

		tableHeightDelegate = calculateTableHeight(response.length);

		console.log(tableHeightDelegate)

		return response.map(val => {
			return {
				name: val.delegate_name,
				gender: null,
				party: val.delegate_party,
				data: val.abcenses
			};
		});
	}

	const partySimpleAbcenses = async (gp: string | null, gender: string | null, isDesc: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateAbcenses[]>('absences_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		})) ?? [];

		// tableheight berechnung ist die gefehrlich, da response hier 14 (Zahl der Parteien) zurückgibt
		console.log(response)

		tableHeightParty = calculateTableHeight(response.length);

		console.log(tableHeightParty)

		return response.map(val => {
			return {
				name: null,
				gender: null,
				party: val.delegate_party,
				data: val.abcenses
			};
		});
	}


</script>

<Container>
	<DelegateBarChartControl height={tableHeightDelegate} id={0} delegateMakeRequest={delegateSimpleAbcenses} title="Komplexität pro Abgeordneten (in Schulklasse)"/>
</Container>

<Container>
	<DelegateBarChartControl height={tableHeightParty} id={1} delegateMakeRequest={partySimpleAbcenses} title="Komplexität nach Parteien (in Schulklasse)" />
</Container>
