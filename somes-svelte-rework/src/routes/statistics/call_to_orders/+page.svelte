<script lang="ts">
	import { errorToNull, justPost } from "$lib/api/api";
	import Container from "$lib/components/Layout/Container.svelte";
	import DelegateBarChartControl from "$lib/components/Statistics/DelegateBarChartControl.svelte";
	import type { DelegateData } from "$lib/types";


	type DelegateCallsToOrder = {
		delegate_name: string;
		delegate_party: string;
		delegate_gender: string;
		age_group: string;
		legislative_period: string;
		total_order_calls: number;
		normalized_calls_to_order: number;
	};

	let tableHeightDelegate = 0;
	let tableHeightParty= 0;
	let tableHeightGender= 0;
	let tableHeightAge = 0;
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

	const delegateSimpleCallsToOrder = async (gp: string | null, gender: string | null, isDesc: boolean | true, normalized: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateCallsToOrder[]>('call_to_orders_by_delegate', {
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
				data: normalized ? val.normalized_calls_to_order : val.total_order_calls,
			};
		});
	}

	const partySimpleCallsToOrder = async (gp: string | null, gender: string | null, isDesc: boolean | true, normalized: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateCallsToOrder[]>('call_to_orders_per_party', {
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
				data: normalized ? val.normalized_calls_to_order : val.total_order_calls,
			};
		});
	}

	const genderSimpleCallsToOrder = async (gp: string | null, gender: string | null, isDesc: boolean | true, normalized: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateCallsToOrder[]>('call_to_orders_per_gender', {
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
				data: normalized ? val.normalized_calls_to_order : val.total_order_calls,
			};
		});
	}

	const ageSimpleCallsToOrder = async (gp: string | null, gender: string | null, isDesc: boolean | true, normalized: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateCallsToOrder[]>('call_to_orders_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized

		})) ?? [];


		console.log(response)

		tableHeightAge = setTableHeight(response.length);

		return response.map(val => {
			return {
				name: null,
				party: val.age_group,
				data: normalized ? val.normalized_calls_to_order : val.total_order_calls,
			};
		});
	}

	const legisSimpleCallsToOrder = async (gp: string | null, gender: string | null, isDesc: boolean | true, normalized: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateCallsToOrder[]>('call_to_orders_per_legis', {
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
				data: normalized ? val.normalized_calls_to_order : val.total_order_calls,
			};
		});
	}


</script>

<Container>
	<DelegateBarChartControl height={tableHeightDelegate} id={0} delegateMakeRequest={delegateSimpleCallsToOrder} title="Ordnungsrufe pro Abgeordneten"/>
</Container>

<Container>
	<DelegateBarChartControl height={tableHeightParty} id={1} delegateMakeRequest={partySimpleCallsToOrder} title="Ordnungsrufe nach Parteien" />
</Container>

<Container>
	<DelegateBarChartControl height={tableHeightGender} id={2} delegateMakeRequest={genderSimpleCallsToOrder} title="Ordnungsrufe nach Gender" />
</Container>

<Container>
	<DelegateBarChartControl height={tableHeightAge} id={3} delegateMakeRequest={ageSimpleCallsToOrder} title="Ordnungsrufe nach Alter" />
</Container>

<Container>
	<DelegateBarChartControl height={tableHeightLegis} id={4} delegateMakeRequest={legisSimpleCallsToOrder} title="Ordnungsrufe nach Legislaturperiode" />
</Container>
