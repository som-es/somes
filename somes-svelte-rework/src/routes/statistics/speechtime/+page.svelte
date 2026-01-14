<script lang="ts">
	import { errorToNull, justPostStatistics } from '$lib/api/api';
	import Container from '$lib/components/Layout/Container.svelte';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { DelegateData } from '$lib/types';
	import type {Config} from '$lib/components/Statistics/types'
	import { list } from 'postcss';

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
			filters: [
				{
					id: 'primary',
					name: 'Partei',
					isShown: true,
					multiple: true,
					infoText: 'Filtert die angezeigten Abgeordneten nach Partei.',
					options: [],
					default: []
				},
				{
					id: 'gender',
					name: 'Geschlecht',
					isShown: true,
					multiple: false,
					infoText: 'Filtert die angezeigten Abgeordneten nach Geschlecht.',
					options: ['all', 'f', 'm'],
					default: 'all',
					label: (val) => {
						if (val === 'all') return 'egal';
						return val === 'f' ? 'weiblich' : 'männlich';
					}
				},
				{
					id: 'normalized',
					name: 'Normalisiert',
					isShown: true,
					multiple: false,
					infoText:
						'Zeigt die durchschnittliche Redezeit pro Sitzung statt der gesamten Redezeit eines Abgeordneten.',
					options: [true, false],
					default: true,
					label: (val) => (val ? 'Ja' : 'Nein')
				},
				{
					id: 'desc',
					name: 'Sortierung',
					isShown: true,
					multiple: false,
					infoText: 'Sortiert die Ergebnisse absteigend. (höchste Werte zuerst)',
					options: [true, false],
					default: true,
					label: (val) => (val ? 'Ja' : 'Nein')
				}
			]
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
			filters: [
				{
					id: 'primary',
					name: 'Partei',
					isShown: true,
					multiple: true,
					infoText:
						'Filtert die Ergebnisse nach Partei. Es werden nur ausgewählte Parteien angezeigt.',
					options: [],
					default: []
				},
				{
					id: 'gender',
					name: 'Geschlecht',
					isShown: false,
					multiple: false,
					infoText: null,
					options: ['all', 'f', 'm'],
					default: 'all'
				},
				{
					id: 'normalized',
					name: 'Normalisiert',
					isShown: true,
					multiple: false,
					infoText:
						'Zeigt die durchschnittliche Redezeit pro Parteimitglied statt der gesamten Redezeit der Partei.',
					options: [true, false],
					default: true,
					label: (val) => (val ? 'Ja' : 'Nein')
				},
				{
					id: 'desc',
					name: 'Absteigend',
					isShown: true,
					multiple: false,
					infoText:
						'Sortiert die Ergebnisse absteigend (höchste Werte zuerst).',
					options: [true, false],
					default: true,
					label: (val) => (val ? 'Ja' : 'Nein')
				}
			]
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
			filters: [
				{
					id: 'primary',
					name: 'Geschlecht',
					isShown: true,
					multiple: true,
					infoText:
						'Filtert die Ergebnisse nach Geschlecht. Es werden nur ausgewählte Parteien angezeigt.',
					options: ['f', 'm'],
					default: []
				},
				{
					id: 'gender',
					name: 'Partei',
					isShown: false,
					multiple: false,
					infoText: null,
					options: ['all'],
					default: 'all'
				},
				{
					id: 'normalized',
					name: 'Normalisiert',
					isShown: true,
					multiple: false,
					infoText:
						'Zeigt die durchschnittliche Redezeit pro Person innerhalb der jeweiligen Geschlechtsgruppe statt der gesamten Redezeit.',
					options: [true, false],
					default: true,
					label: (val) => (val ? 'Ja' : 'Nein')
				},
				{
					id: 'desc',
					name: 'Absteigend',
					isShown: true,
					multiple: false,
					infoText:
						'Sortiert die Ergebnisse absteigend (höchste Werte zuerst).',
					options: [true, false],
					default: true,
					label: (val) => (val ? 'Ja' : 'Nein')
				}
			]
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
			filters: [
				{
					id: 'primary',
					name: 'Altersgruppe',
					isShown: true,
					multiple: true,
					infoText:
						'Filtert die Ergebnisse nach Altersgruppen. Es werden nur ausgewählte Altersgruppen angezeigt.',
					options: [],
					default: []
				},
				{
					id: 'gender',
					name: 'Geschlecht',
					isShown: false,
					multiple: false,
					infoText: null,
					options: ['all', 'f', 'm'],
					default: 'all'
				},
				{
					id: 'normalized',
					name: 'Normalisiert',
					isShown: true,
					multiple: false,
					infoText:
						'Zeigt die durchschnittliche Redezeit pro Person innerhalb der jeweiligen Altersgruppe statt der gesamten Redezeit.',
					options: [true, false],
					default: true,
					label: (val) => (val ? 'Ja' : 'Nein')
				},
				{
					id: 'desc',
					name: 'Absteigend',
					isShown: true,
					multiple: false,
					infoText:
						'Sortiert die Ergebnisse absteigend (höchste Werte zuerst).',
					options: [true, false],
					default: true,
					label: (val) => (val ? 'Ja' : 'Nein')
				}
			]
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
			filters: [
				{
					id: 'primary',
					name: 'Legislaturperiode',
					isShown: true,
					multiple: true,
					infoText:
						'Filtert die Ergebnisse nach Legislaturperioden. Es werden nur ausgewählte Legislaturperioden angezeigt.',
					options: [],
					default: []
				},
				{
					id: 'gender',
					name: 'Geschlecht',
					isShown: true,
					multiple: false,
					infoText:
						'Filtert die Ergebnisse nach Geschlecht der Abgeordneten.',
					options: ['all', 'f', 'm'],
					default: 'all'
				},
				{
					id: 'normalized',
					name: 'Normalisiert',
					isShown: true,
					multiple: false,
					infoText:
						'Zeigt die durchschnittliche Redezeit pro Sitzung in der jeweiligen Legislaturperiode statt der gesamten Redezeit.',
					options: [true, false],
					default: true,
					label: (val) => (val ? 'Ja' : 'Nein')
				},
				{
					id: 'desc',
					name: 'Absteigend',
					isShown: true,
					multiple: false,
					infoText:
						'Sortiert die Ergebnisse absteigend (höchste Werte zuerst).',
					options: [true, false],
					default: true,
					label: (val) => (val ? 'Ja' : 'Nein')
				}
			]
		}}


	/>
</Container>
