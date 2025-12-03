<script lang="ts">
	import { errorToNull, justPostStatistics } from '$lib/api/api';
	import Container from '$lib/components/Layout/Container.svelte';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { DelegateData } from '$lib/types';

	type DelegateSpeeches = {
		delegate_name: string;
		delegate_party: string;
		delegate_gender: string;
		age_group: string;
		legislative_period: string;
		total_speeches: number;
		normalized_speeches: number;
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

	const delegateSimpleSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean | true,
		normalized: boolean | true
	): Promise<DelegateData[]> => {
		const response =
			errorToNull(
				await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_delegate', {
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
				data: normalized ? val.normalized_speeches : val.total_speeches
			};
		});
	};

	const partySimpleSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean | true,
		normalized: boolean | true
	): Promise<DelegateData[]> => {
		const response =
			errorToNull(
				await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_party', {
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
				data: normalized ? val.normalized_speeches : val.total_speeches
			};
		});
	};

	const genderSimpleSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean | true,
		normalized: boolean | true
	): Promise<DelegateData[]> => {
		const response =
			errorToNull(
				await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_gender', {
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
				data: normalized ? val.normalized_speeches : val.total_speeches
			};
		});
	};

	const ageSimpleSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean | true,
		normalized: boolean | true
	): Promise<DelegateData[]> => {
		const response =
			errorToNull(
				await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_age', {
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
				data: normalized ? val.normalized_speeches : val.total_speeches
			};
		});
	};

	const legisSimpleSpeeches = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean | true,
		normalized: boolean | true
	): Promise<DelegateData[]> => {
		const response =
			errorToNull(
				await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_legis', {
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
				data: normalized ? val.normalized_speeches : val.total_speeches
			};
		});
	};
</script>

<Container>
	<DelegateBarChartControl
		height={tableHeightDelegate}
		id={0}
		delegateMakeRequest={delegateSimpleSpeeches}
		title="Reden pro Abgeordneten"
		config={{
			party_filter_info: "Filtert die Ergebnisse nach Partei. Es werden nur Abgeordnete der ausgewählten Partei angezeigt.",
			gender_filter_info: "Filtert die Ergebnisse nach dem Geschlecht der Abgeordneten.",
			normalized_filter_info: "Zeigt die durchschnittlihe Anzahl an Reden pro Sitzung statt der Gesamtzahl der Reden.",
			desc_filter_info: "Sortiert die Liste absteigend (höchste Werte zuerst), wenn aktiviert."
		}}
	/>
</Container>

<Container>
	<DelegateBarChartControl
		height={tableHeightParty}
		id={1}
		delegateMakeRequest={partySimpleSpeeches}
		title="Reden nach Parteien" 
		config={{
			party_filter_info: "Filtert die Ergebnisse nach Partei. Es nur ausgewählten Parteien angezeigt",
			gender_filter_info: "Dieser Filter hat in dieser Ansicht keine Funktion.",
			normalized_filter_info: "Zeigt die durchschnittliche Anzahl an Reden pro Parteimitglied statt der gesamten Reden der Partei.",
			desc_filter_info: "Sortiert die Ergebnisse absteigend (höchste Werte zuerst)."
		}}

	/>
</Container>

<Container>
	<DelegateBarChartControl
		height={tableHeightGender}
		id={2}
		delegateMakeRequest={genderSimpleSpeeches}
		title="Reden nach Gender"
		config={{
			party_filter_info: "Filtert die Ergebnise nach Geschlecht. Es werden nur ausgewählte Parteien angezeigt.",
			gender_filter_info: "Dieser Filter hat in dieser Ansicht keine Funktion.",
			normalized_filter_info: "Zeigt die durchschnittliche Anzahl an Reden pro Person innerhalb der Geschlechtsgruppe statt der gesamten Anzahl der Reden.",
			desc_filter_info: "Sortiert die Ergebnisse absteigend (höchste Werte zuerst)."
		}}

	/>
</Container>

<Container>
	<DelegateBarChartControl
		height={tableHeightAge}
		id={3}
		delegateMakeRequest={ageSimpleSpeeches}
		title="Reden nach Alter"
		config={{
			party_filter_info: "Filtert die Ergebnisse nach Altersgruppen. Es werden nur die ausgewählten Altersgruppen angezeigt.",
			gender_filter_info: "Dieser Filter hat in dieser Ansicht keine Funktion.",
			normalized_filter_info: "Zeigt die durchschnittliche Anzahl an Reden pro Person innerhalb der jeweiligen Altersgruppe statt der gesamten Reden dieser Altersgruppe.",
			desc_filter_info: "Sortiert die Ergebnisse absteigend (höchste Werte zuerst)."
		}}

	/>
</Container>

<Container>
	<DelegateBarChartControl
		height={tableHeightLegis}
		id={4}
		delegateMakeRequest={legisSimpleSpeeches}
		title="Reden nach Legislaturperiode"
		config={{
			party_filter_info: "Filtert die Ergebnisse nach Partei. Es werden nur die Reden der Mitglieder der ausgewählten Partei berücksichtigt.",
			gender_filter_info: "Filtert die Ergebnisse nach dem Geschlecht der Abgeordneten.",
			normalized_filter_info: "Zeigt die durchschnittliche Anzahl an Reden pro Sitzung innerhalb der jeweiligen Legislaturperiode statt der Gesamtzahl der Reden.",
			desc_filter_info: "Sortiert die Ergebnisse absteigend (höchste Werte zuerst)."
		}}

	/>
</Container>
