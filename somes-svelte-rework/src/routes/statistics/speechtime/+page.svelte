<script lang="ts">
	import { errorToNull, justPostStatistics } from '$lib/api/api';
	import Container from '$lib/components/Layout/Container.svelte';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { DelegateData } from '$lib/types';
	import type {Config} from '$lib/components/Statistics/types'

	type DelegateSpeechTime = {
		delegate_name: string;
		delegate_party: string;
		delegate_gender: string;
		age_group: string;
		legislative_period: string;
		total_speech_time: number;
		normalized_speech_time: number;
	};



	let tableHeightDelegate = 0;
	let tableHeightParty = 0;
	let tableHeightGender = 0;
	let tableHeightAge = 0;
	let tableHeightLegis = 0;

	const setTableHeight = (numRows: number): number => {
		const minHeight = 275;
		const maxHeight = 3100;

		let height = minHeight;

		if (numRows > 6) {
			height = 350;
		}
		if (numRows > 10) {
			height = 500;
		}
		if (numRows > 15) {
			height = 725;
		}
		if (numRows > 22) {
			height = 975;
		}
		if (numRows > 30) {
			height = 1300;
		}
		if (numRows > 42) {
			height = 1800;
		}
		if (numRows > 58) {
			height = 2425;
		}
		if (numRows > 80) {
			height = maxHeight;
		}

		return height;
	};

	const delegateSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean | true,
		normalized: boolean | true
	): Promise<DelegateData[]> => {
		const response =
			errorToNull(
				await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_delegate', {
					legis_period: gp,
					party: null,
					gender,
					is_desc: isDesc,
					normalized: normalized
				})
			) ?? [];

		tableHeightDelegate = setTableHeight(response.length);

		return response.map((val) => {
			return {
				name: val.delegate_name,
				party: val.delegate_party,
				data: normalized ? val.normalized_speech_time : val.total_speech_time
			};
		});
	};

	const partySimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean | true,
		normalized: boolean | true
	): Promise<DelegateData[]> => {
		const response =
			errorToNull(
				await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_party', {
					legis_period: gp,
					party: null,
					gender,
					is_desc: isDesc,
					normalized: normalized
				})
			) ?? [];

		tableHeightParty = setTableHeight(response.length);

		return response.map((val) => {
			return {
				name: null,
				party: val.delegate_party,
				data: normalized ? val.normalized_speech_time : val.total_speech_time
			};
		});
	};

	const genderSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean | true,
		normalized: boolean | true
	): Promise<DelegateData[]> => {
		const response =
			errorToNull(
				await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_gender', {
					legis_period: gp,
					party: null,
					gender,
					is_desc: isDesc,
					normalized: normalized
				})
			) ?? [];

		tableHeightGender = setTableHeight(response.length);

		return response.map((val) => {
			return {
				name: null,
				party: val.delegate_gender,
				data: normalized ? val.normalized_speech_time : val.total_speech_time
			};
		});
	};

	const ageSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean | true,
		normalized: boolean | true
	): Promise<DelegateData[]> => {
		const response =
			errorToNull(
				await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_age', {
					legis_period: gp,
					party: null,
					gender,
					is_desc: isDesc,
					normalized: normalized
				})
			) ?? [];

		tableHeightAge = setTableHeight(response.length);

		return response.map((val) => {
			return {
				name: null,
				party: val.age_group,
				data: normalized ? val.normalized_speech_time : val.total_speech_time
			};
		});
	};

	const legisSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean | true,
		normalized: boolean | true
	): Promise<DelegateData[]> => {
		const response =
			errorToNull(
				await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_legis', {
					legis_period: gp,
					party: null,
					gender,
					is_desc: isDesc,
					normalized: normalized
				})
			) ?? [];

		tableHeightLegis = setTableHeight(response.length);

		return response.map((val) => {
			return {
				name: null,
				party: val.legislative_period,
				data: normalized ? val.normalized_speech_time : val.total_speech_time
			};
		});
	};
</script>

<Container>
	<DelegateBarChartControl
		height={tableHeightDelegate}
		id={0}
		delegateMakeRequest={delegateSimpleSpeechTime}
		title="Redezeit pro Abgeordneten (in Mintuen)"
		config={{
			party_filter_info:"asdf",
			gender_filter_info:"asdf",
			normalized_filter_info:"asdf",
			desc_filter_info:"asdf"
		}}
	/>
</Container>

<Container>
	<DelegateBarChartControl
		height={tableHeightParty}
		id={1}
		delegateMakeRequest={partySimpleSpeechTime}
		title="Redezeit nach Parteien (in Minuten)"
		config={{
			party_filter_info:null,
			gender_filter_info:"ds",
			normalized_filter_info:"sdfgafd",
			desc_filter_info:"sdfsadfg"
		}}
	/>
</Container>

<Container>
	<DelegateBarChartControl
		height={tableHeightGender}
		id={2}
		delegateMakeRequest={genderSimpleSpeechTime}
		title="Redezeit nach Gender (in Minuten)"
		config={{
			party_filter_info:"",
			gender_filter_info:"",
			normalized_filter_info:"",
			desc_filter_info:""
		}}
	/>
</Container>

<Container>
	<DelegateBarChartControl
		height={tableHeightAge}
		id={3}
		delegateMakeRequest={ageSimpleSpeechTime}
		title="Redezeit nach Alter (in Minuten)"
		config={{
			party_filter_info:"",
			gender_filter_info:"",
			normalized_filter_info:"",
			desc_filter_info:""
		}}
	/>
</Container>

<Container>
	<DelegateBarChartControl
		height={tableHeightLegis}
		id={4}
		delegateMakeRequest={legisSimpleSpeechTime}
		title="Redezeit nach Legislaturperiode (in Minuten)"
		config={{
			party_filter_info:"",
			gender_filter_info:"",
			normalized_filter_info:"",
			desc_filter_info:""
		}}
	/>
</Container>
