<script lang="ts">
	import { errorToNull, justPost } from "$lib/api/api";
	import Container from "$lib/components/Layout/Container.svelte";
	import DelegateBarChartControl from "$lib/components/Statistics/DelegateBarChartControl.svelte";
	import type { DelegateData } from "$lib/types";


	type DelegateSpeechTime = {
		delegate_name: string;
		delegate_gender: string;
		delegate_party: string;
		total_speech_time: number;
	};

	let tableHeightDelegate = 0;

	const calculateTableHeight = (numRows: number): number => {
		const maxHeight = 3100;
		const maxRows = 100;

		const height = (numRows / maxRows) * maxHeight;

		return Math.min(height, maxHeight);
	};

	const delegateSimpleSpeechTime = async (gp: string | null, gender: string | null, isDesc: boolean | true): Promise<DelegateData[]> => {
		const response = errorToNull(await justPost<DelegateSpeechTime[]>('call_to_orders_by_delegate', {
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
				data: val.total_speech_time
			};
		});
	}


</script>

<Container>
	<DelegateBarChartControl height={tableHeightDelegate} id={0} delegateMakeRequest={delegateSimpleSpeechTime} title="Redezeit pro Abgeordneten (in Mintuen)"/>
</Container>
