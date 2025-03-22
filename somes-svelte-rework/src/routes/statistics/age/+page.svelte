<script lang="ts">
	import { errorToNull, justPost } from "$lib/api/api";
	import Container from "$lib/components/Layout/Container.svelte";
	import DelegateBarChartControl from "$lib/components/Statistics/DelegateBarChartControl.svelte";
	import type { DelegateData } from "$lib/types";


	type DelegateAge = {
		delegate_name: string;
		delegate_party: string;
		delegate_gender: string;
		legislative_period: string;
		delegate_age: number;
		average_age: number;
	};

	let tableHeightDelegate = 0;
	let tableHeightParty= 0;
	let tableHeightGender= 0;
	let tableHeightLegis = 0;

	const setTableHeight = (numRows: number): number => {
		const minHeight = 275;
		const maxHeight = 3100;

		let height = minHeight

		if (numRows > 6) { height = 350; }
		if (numRows > 10) { height = 500; }
		if (numRows > 15) { height = 725; }
		if (numRows > 22) { height = 975; }
		if (numRows > 30) { height = 1300; }
		if (numRows > 42) { height = 1800; }
		if (numRows > 58) { height = 2425; }
		if (numRows > 80) { height = maxHeight; }

		return height;
	};

	const delegateSimpleAge = async (gp: string | null, gender: string | null, isDesc: boolean | true, normalized: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateAge[]>('age_of_delegates', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		})) ?? [];

		console.log(response)

		tableHeightDelegate = setTableHeight(response.length);

		return response.map(val => {
			return {
				name: val.delegate_name,
				party: val.delegate_party,
				data: val.delegate_age,
			};
		});
	}

	const partySimpleAge = async (gp: string | null, gender: string | null, isDesc: boolean | true, normalized: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateAge[]>('age_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		})) ?? [];

		console.log(response)

		tableHeightParty = setTableHeight(response.length);

		return response.map(val => {
			return {
				name: null,
				party: val.delegate_party,
				data: val.average_age,
			};
		});
	}

	const genderSimpleAge = async (gp: string | null, gender: string | null, isDesc: boolean | true, normalized: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateAge[]>('age_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized

		})) ?? [];


		console.log(response)

		tableHeightGender = setTableHeight(response.length);

		return response.map(val => {
			return {
				name: null,
				party: val.delegate_gender,
				data: val.average_age,
			};
		});
	}

	const legisSimpleAge = async (gp: string | null, gender: string | null, isDesc: boolean | true, normalized: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateAge[]>('age_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized

		})) ?? [];

		console.log(response)

		tableHeightLegis = setTableHeight(response.length);

		return response.map(val => {
			return {
				name: null,
				party: val.legislative_period,
				data: val.average_age,
			};
		});
	}


</script>

<Container>
	<DelegateBarChartControl height={tableHeightDelegate} id={0} delegateMakeRequest={delegateSimpleAge} title="Alter pro Abgeordneten"/>
</Container>

<Container>
	<DelegateBarChartControl height={tableHeightParty} id={1} delegateMakeRequest={partySimpleAge} title="Alter nach Parteien" />
</Container>

<Container>
	<DelegateBarChartControl height={tableHeightGender} id={2} delegateMakeRequest={genderSimpleAge} title="Alter nach Gender" />
</Container>

<Container>
	<DelegateBarChartControl height={tableHeightLegis} id={4} delegateMakeRequest={legisSimpleAge} title="Alter nach Legislaturperiode" />
</Container>