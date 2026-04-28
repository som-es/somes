<script lang="ts">
	import { onMount } from 'svelte';
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { DelegateData } from '$lib/types';
	import { Select } from 'bits-ui';

	// Type definitions for all statistics
	type DelegateAge = {
		delegate_name: string;
		delegate_party: string;
		delegate_gender: string;
		legislative_period: string;
		delegate_age: number;
		average_age: number;
	};

	type DelegateAbsences = {
		delegate_name: string;
		delegate_party: string;
		delegate_gender: string;
		age_group: string;
		legislative_period: string;
		total_absences: number;
		normalized_absences: number;
	};

	type DelegateActivity = {
		delegate_name: string;
		delegate_party: string;
		delegate_gender: string;
		age_group: string;
		legislative_period: string;
		activity_score: number;
		normalized_activity: number;
	};

	type DelegateCallsToOrder = {
		delegate_name: string;
		delegate_party: string;
		delegate_gender: string;
		age_group: string;
		legislative_period: string;
		total_order_calls: number;
		normalized_calls_to_order: number;
	};

	type DelegateComplexity = {
		delegate_name: string;
		delegate_party: string;
		delegate_gender: string;
		age_group: string;
		legislative_period: string;
		avg_complexity: number;
	};

	type DelegateSpeechTime = {
		delegate_name: string;
		delegate_party: string;
		delegate_gender: string;
		age_group: string;
		legislative_period: string;
		total_speech_time: number;
		normalized_speech_time: number;
	};

	type DelegateSpeeches = {
		delegate_name: string;
		delegate_party: string;
		delegate_gender: string;
		age_group: string;
		legislative_period: string;
		total_speeches: number;
		normalized_speeches: number;
	};

	// Category selection states
	let selectedAgeCategory = 'delegate';
	let selectedAbsencesCategory = 'delegate';
	let selectedActivityCategory = 'delegate';
	let selectedCallsCategory = 'delegate';
	let selectedComplexityCategory = 'delegate';
	let selectedSpeechTimeCategory = 'delegate';
	let selectedSpeechesCategory = 'delegate';

	const categoryOptions = [
		{ value: 'delegate', label: 'Pro Abgeordneten', icon: '👤' },
		{ value: 'party', label: 'Nach Parteien', icon: '🏛️' },
		{ value: 'gender', label: 'Nach Gender', icon: '⚧️' },
		{ value: 'age', label: 'Nach Alter', icon: '📅' },
		{ value: 'legis', label: 'Nach Legislaturperiode', icon: '🏛️' }
	];

	// Age statistics functions
	const delegateSimpleAge = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateAge[]>('age_of_delegates', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: val.delegate_name, party: val.delegate_party, data: val.delegate_age }));
	};

	const partySimpleAge = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateAge[]>('age_per_party', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_party, data: val.average_age }));
	};

	const genderSimpleAge = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateAge[]>('age_per_gender', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_gender, data: val.average_age }));
	};

	const legisSimpleAge = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateAge[]>('age_per_legis', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.legislative_period, data: val.average_age }));
	};

	// Absences statistics functions
	const delegateSimpleAbsences = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateAbsences[]>('absences_per_delegate', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: val.delegate_name, party: val.delegate_party, data: normalized ? val.normalized_absences : val.total_absences }));
	};

	const partySimpleAbsences = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateAbsences[]>('absences_per_party', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_party, data: normalized ? val.normalized_absences : val.total_absences }));
	};

	const genderSimpleAbsences = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateAbsences[]>('absences_per_gender', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_gender, data: normalized ? val.normalized_absences : val.total_absences }));
	};

	const ageSimpleAbsences = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateAbsences[]>('absences_per_age', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.age_group, data: normalized ? val.normalized_absences : val.total_absences }));
	};

	const legisSimpleAbsences = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateAbsences[]>('absences_per_legis', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.legislative_period, data: normalized ? val.normalized_absences : val.total_absences }));
	};

	// Activity statistics functions
	const delegateSimpleActivity = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateActivity[]>('activity_per_delegate', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: val.delegate_name, party: val.delegate_party, data: normalized ? val.normalized_activity : val.activity_score }));
	};

	const partySimpleActivity = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateActivity[]>('activity_per_party', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_party, data: normalized ? val.normalized_activity : val.activity_score }));
	};

	const genderSimpleActivity = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateActivity[]>('activity_per_gender', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_gender, data: normalized ? val.normalized_activity : val.activity_score }));
	};

	const ageSimpleActivity = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateActivity[]>('activity_per_age', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.age_group, data: normalized ? val.normalized_activity : val.activity_score }));
	};

	const legisSimpleActivity = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateActivity[]>('activity_per_legis', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.legislative_period, data: normalized ? val.normalized_activity : val.activity_score }));
	};

	// Call to Orders statistics functions
	const delegateSimpleCallsToOrder = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateCallsToOrder[]>('call_to_orders_by_delegate', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: val.delegate_name, party: val.delegate_party, data: normalized ? val.normalized_calls_to_order : val.total_order_calls }));
	};

	const partySimpleCallsToOrder = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateCallsToOrder[]>('call_to_orders_per_party', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_party, data: normalized ? val.normalized_calls_to_order : val.total_order_calls }));
	};

	const genderSimpleCallsToOrder = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateCallsToOrder[]>('call_to_orders_per_gender', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_gender, data: normalized ? val.normalized_calls_to_order : val.total_order_calls }));
	};

	const ageSimpleCallsToOrder = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateCallsToOrder[]>('call_to_orders_per_age', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.age_group, data: normalized ? val.normalized_calls_to_order : val.total_order_calls }));
	};

	const legisSimpleCallsToOrder = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateCallsToOrder[]>('call_to_orders_per_legis', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.legislative_period, data: normalized ? val.normalized_calls_to_order : val.total_order_calls }));
	};

	// Speech Complexity statistics functions
	const delegateSimpleComplexity = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateComplexity[]>('complexity_per_delegate', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: val.delegate_name, party: val.delegate_party, data: val.avg_complexity }));
	};

	const partySimpleComplexity = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateComplexity[]>('complexity_per_party', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_party, data: val.avg_complexity }));
	};

	const genderSimpleComplexity = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateComplexity[]>('complexity_per_gender', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_gender, data: val.avg_complexity }));
	};

	const ageSimpleComplexity = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateComplexity[]>('complexity_at_age', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.age_group, data: val.avg_complexity }));
	};

	const legisSimpleComplexity = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateComplexity[]>('complexity_per_legis', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.legislative_period, data: val.avg_complexity }));
	};

	// Speech Time statistics functions
	const delegateSimpleSpeechTime = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_delegate', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: val.delegate_name, party: val.delegate_party, data: normalized ? val.normalized_speech_time : val.total_speech_time }));
	};

	const partySimpleSpeechTime = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_party', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_party, data: normalized ? val.normalized_speech_time : val.total_speech_time }));
	};

	const genderSimpleSpeechTime = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_gender', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_gender, data: normalized ? val.normalized_speech_time : val.total_speech_time }));
	};

	const ageSimpleSpeechTime = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_age', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.age_group, data: normalized ? val.normalized_speech_time : val.total_speech_time }));
	};

	const legisSimpleSpeechTime = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeechTime[]>('speechtime_per_legis', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.legislative_period, data: normalized ? val.normalized_speech_time : val.total_speech_time }));
	};

	// Total Speeches statistics functions
	const delegateSimpleSpeeches = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_delegate', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: val.delegate_name, party: val.delegate_party, data: normalized ? val.normalized_speeches : val.total_speeches }));
	};

	const partySimpleSpeeches = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_party', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_party, data: normalized ? val.normalized_speeches : val.total_speeches }));
	};

	const genderSimpleSpeeches = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_gender', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.delegate_gender, data: normalized ? val.normalized_speeches : val.total_speeches }));
	};

	const ageSimpleSpeeches = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_age', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.age_group, data: normalized ? val.normalized_speeches : val.total_speeches }));
	};

	const legisSimpleSpeeches = async (gp: string | null, gender: string | null, isDesc: boolean, normalized: boolean): Promise<DelegateData[]> => {
		const response = await justPostStatistics<DelegateSpeeches[]>('total_speeches_per_legis', { legis_period: gp, party: null, gender, is_desc: isDesc, normalized: normalized });
		if ('error' in response) return [];
		return response.map(val => ({ name: null, party: val.legislative_period, data: normalized ? val.normalized_speeches : val.total_speeches }));
	};

	// Reactive functions and titles
	$: ageFunction = (() => {
		switch (selectedAgeCategory) {
			case 'delegate': return delegateSimpleAge;
			case 'party': return partySimpleAge;
			case 'gender': return genderSimpleAge;
			case 'age': return async () => [];
			case 'legis': return legisSimpleAge;
			default: return delegateSimpleAge;
		}
	})();

	$: ageTitle = (() => {
		switch (selectedAgeCategory) {
			case 'delegate': return 'Alter pro Abgeordneten';
			case 'party': return 'Alter nach Parteien';
			case 'gender': return 'Alter nach Gender';
			case 'age': return 'Alter nach Alter';
			case 'legis': return 'Alter nach Legislaturperiode';
			default: return 'Alter pro Abgeordneten';
		}
	})();

	$: absencesFunction = (() => {
		switch (selectedAbsencesCategory) {
			case 'delegate': return delegateSimpleAbsences;
			case 'party': return partySimpleAbsences;
			case 'gender': return genderSimpleAbsences;
			case 'age': return ageSimpleAbsences;
			case 'legis': return legisSimpleAbsences;
			default: return delegateSimpleAbsences;
		}
	})();

	$: absencesTitle = (() => {
		switch (selectedAbsencesCategory) {
			case 'delegate': return 'Abwesenheiten pro Abgeordneten';
			case 'party': return 'Abwesenheiten nach Parteien';
			case 'gender': return 'Abwesenheiten nach Gender';
			case 'age': return 'Abwesenheiten nach Alter';
			case 'legis': return 'Abwesenheiten nach Legislaturperiode';
			default: return 'Abwesenheiten pro Abgeordneten';
		}
	})();

	$: activityFunction = (() => {
		switch (selectedActivityCategory) {
			case 'delegate': return delegateSimpleActivity;
			case 'party': return partySimpleActivity;
			case 'gender': return genderSimpleActivity;
			case 'age': return ageSimpleActivity;
			case 'legis': return legisSimpleActivity;
			default: return delegateSimpleActivity;
		}
	})();

	$: activityTitle = (() => {
		switch (selectedActivityCategory) {
			case 'delegate': return 'Aktivitätsscoring pro Abgeordneten';
			case 'party': return 'Aktivitätsscoring nach Parteien';
			case 'gender': return 'Aktivitätsscoring nach Gender';
			case 'age': return 'Aktivitätsscoring nach Alter';
			case 'legis': return 'Aktivitätsscoring nach Legislaturperiode';
			default: return 'Aktivitätsscoring pro Abgeordneten';
		}
	})();

	$: callsFunction = (() => {
		switch (selectedCallsCategory) {
			case 'delegate': return delegateSimpleCallsToOrder;
			case 'party': return partySimpleCallsToOrder;
			case 'gender': return genderSimpleCallsToOrder;
			case 'age': return ageSimpleCallsToOrder;
			case 'legis': return legisSimpleCallsToOrder;
			default: return delegateSimpleCallsToOrder;
		}
	})();

	$: callsTitle = (() => {
		switch (selectedCallsCategory) {
			case 'delegate': return 'Ordnungsrufe pro Abgeordneten';
			case 'party': return 'Ordnungsrufe nach Parteien';
			case 'gender': return 'Ordnungsrufe nach Gender';
			case 'age': return 'Ordnungsrufe nach Alter';
			case 'legis': return 'Ordnungsrufe nach Legislaturperiode';
			default: return 'Ordnungsrufe pro Abgeordneten';
		}
	})();

	$: complexityFunction = (() => {
		switch (selectedComplexityCategory) {
			case 'delegate': return delegateSimpleComplexity;
			case 'party': return partySimpleComplexity;
			case 'gender': return genderSimpleComplexity;
			case 'age': return ageSimpleComplexity;
			case 'legis': return legisSimpleComplexity;
			default: return delegateSimpleComplexity;
		}
	})();

	$: complexityTitle = (() => {
		switch (selectedComplexityCategory) {
			case 'delegate': return 'Sprachkomplexität pro Abgeordneten';
			case 'party': return 'Sprachkomplexität nach Parteien';
			case 'gender': return 'Sprachkomplexität nach Gender';
			case 'age': return 'Sprachkomplexität nach Alter';
			case 'legis': return 'Sprachkomplexität nach Legislaturperiode';
			default: return 'Sprachkomplexität pro Abgeordneten';
		}
	})();

	$: speechTimeFunction = (() => {
		switch (selectedSpeechTimeCategory) {
			case 'delegate': return delegateSimpleSpeechTime;
			case 'party': return partySimpleSpeechTime;
			case 'gender': return genderSimpleSpeechTime;
			case 'age': return ageSimpleSpeechTime;
			case 'legis': return legisSimpleSpeechTime;
			default: return delegateSimpleSpeechTime;
		}
	})();

	$: speechTimeTitle = (() => {
		switch (selectedSpeechTimeCategory) {
			case 'delegate': return 'Redezeit pro Abgeordneten (in Minuten)';
			case 'party': return 'Redezeit nach Parteien (in Minuten)';
			case 'gender': return 'Redezeit nach Gender (in Minuten)';
			case 'age': return 'Redezeit nach Alter (in Minuten)';
			case 'legis': return 'Redezeit nach Legislaturperiode (in Minuten)';
			default: return 'Redezeit pro Abgeordneten (in Minuten)';
		}
	})();

	$: speechesFunction = (() => {
		switch (selectedSpeechesCategory) {
			case 'delegate': return delegateSimpleSpeeches;
			case 'party': return partySimpleSpeeches;
			case 'gender': return genderSimpleSpeeches;
			case 'age': return ageSimpleSpeeches;
			case 'legis': return legisSimpleSpeeches;
			default: return delegateSimpleSpeeches;
		}
	})();

	$: speechesTitle = (() => {
		switch (selectedSpeechesCategory) {
			case 'delegate': return 'Reden pro Abgeordneten';
			case 'party': return 'Reden nach Parteien';
			case 'gender': return 'Reden nach Gender';
			case 'age': return 'Reden nach Alter';
			case 'legis': return 'Reden nach Legislaturperiode';
			default: return 'Reden pro Abgeordneten';
		}
	})();
</script>

<svelte:head>
    <title>Statistiken</title>
    <meta name="description" content="Statistiken über den Nationalrat und deren Abgeorndete" />
</svelte:head>

<div class="container mx-auto px-4 py-8">
	<h1 class="text-3xl font-bold mb-8">Statistiken</h1>

	<!-- Age Statistics Section -->
	<div class="bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-blue-900/20 dark:to-indigo-900/20 rounded-2xl p-8 shadow-lg border border-blue-200/50 dark:border-blue-800/50 mb-8">
		<div class="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-4 mb-6">
			<div class="flex items-center gap-3">
				<div class="w-12 h-12 bg-blue-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
					👥
				</div>
				<div>
					<h2 class="text-2xl font-bold text-slate-800 dark:text-slate-200">Altersstatistiken</h2>
					<p class="text-sm text-slate-600 dark:text-slate-400">Analyse der Altersverteilung im Parlament</p>
				</div>
			</div>
			<!-- Kategorie-Auswahl kompakt rechts oben -->
			<div class="flex flex-col items-end gap-2">
				<div class="flex items-center gap-2">
					<div class="w-1 h-4 bg-blue-500 rounded-full"></div>
					<h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">Kategorie auswählen</h3>
				</div>
				<div class="bg-white dark:bg-slate-800 rounded-xl p-1 shadow-md w-full lg:w-72">
					<Select.Root
						type="single"
						bind:value={selectedAgeCategory}
						items={categoryOptions}
					>
						<Select.Trigger class="w-full h-10 bg-white dark:bg-slate-800 border-0 rounded-lg px-3 hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors">
							<span class="flex items-center gap-2 text-slate-700 dark:text-slate-300">
								<span class="text-base">{categoryOptions.find(opt => opt.value === selectedAgeCategory)?.icon || '📊'}</span>
								<span class="text-sm">{categoryOptions.find(opt => opt.value === selectedAgeCategory)?.label || 'Kategorie auswählen'}</span>
							</span>
						</Select.Trigger>
					<Select.Portal>
						<Select.Content class="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl shadow-xl">
							<Select.Viewport>
								{#each categoryOptions as option}
									<Select.Item value={option.value} label={option.label} class="px-4 py-3 hover:bg-slate-100 dark:hover:bg-slate-700 cursor-pointer flex items-center gap-3">
										<span class="text-lg">{option.icon}</span>
										<div>
											<div class="font-medium">{option.label}</div>
										</div>
									</Select.Item>
								{/each}
							</Select.Viewport>
						</Select.Content>
					</Select.Portal>
				</Select.Root>
			</div>
			</div>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={ageFunction}
		/>
	</div>

	<!-- Absences Statistics Section -->
	<div class="bg-gradient-to-br from-amber-50 to-orange-100 dark:from-amber-900/20 dark:to-orange-900/20 rounded-2xl p-8 shadow-lg border border-amber-200/50 dark:border-amber-800/50 mb-8">
		<div class="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-4 mb-6">
			<div class="flex items-center gap-3">
				<div class="w-12 h-12 bg-amber-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
					📋
				</div>
				<div>
					<h2 class="text-2xl font-bold text-slate-800 dark:text-slate-200">Abwesenheitsstatistiken</h2>
					<p class="text-sm text-slate-600 dark:text-slate-400">Analyse der Abwesenheiten und Anwesenheiten</p>
				</div>
			</div>
			<!-- Kategorie-Auswahl kompakt rechts oben -->
			<div class="flex flex-col items-end gap-2">
				<div class="flex items-center gap-2">
					<div class="w-1 h-4 bg-amber-500 rounded-full"></div>
					<h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">Kategorie auswählen</h3>
				</div>
				<div class="bg-white dark:bg-slate-800 rounded-xl p-1 shadow-md w-full lg:w-72">
					<Select.Root
						type="single"
						bind:value={selectedAbsencesCategory}
						items={categoryOptions}
					>
						<Select.Trigger class="w-full h-10 bg-white dark:bg-slate-800 border-0 rounded-lg px-3 hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors">
							<span class="flex items-center gap-2 text-slate-700 dark:text-slate-300">
								<span class="text-base">{categoryOptions.find(opt => opt.value === selectedAbsencesCategory)?.icon || '📊'}</span>
								<span class="text-sm">{categoryOptions.find(opt => opt.value === selectedAbsencesCategory)?.label || 'Kategorie auswählen'}</span>
							</span>
						</Select.Trigger>
					<Select.Portal>
						<Select.Content class="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl shadow-xl">
							<Select.Viewport>
								{#each categoryOptions as option}
									<Select.Item value={option.value} label={option.label} class="px-4 py-3 hover:bg-slate-100 dark:hover:bg-slate-700 cursor-pointer flex items-center gap-3">
										<span class="text-lg">{option.icon}</span>
										<div>
											<div class="font-medium">{option.label}</div>
										</div>
									</Select.Item>
								{/each}
							</Select.Viewport>
						</Select.Content>
					</Select.Portal>
				</Select.Root>
			</div>
			</div>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={absencesFunction}
		/>
	</div>

	<!-- Activity Statistics Section -->
	<div class="bg-gradient-to-br from-green-50 to-emerald-100 dark:from-green-900/20 dark:to-emerald-900/20 rounded-2xl p-8 shadow-lg border border-green-200/50 dark:border-green-800/50 mb-8">
		<div class="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-4 mb-6">
			<div class="flex items-center gap-3">
				<div class="w-12 h-12 bg-green-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
					⚡
				</div>
				<div>
					<h2 class="text-2xl font-bold text-slate-800 dark:text-slate-200">Aktivitätsstatistiken</h2>
					<p class="text-sm text-slate-600 dark:text-slate-400">Analyse der parlamentarischen Aktivität</p>
				</div>
			</div>
			<!-- Kategorie-Auswahl kompakt rechts oben -->
			<div class="flex flex-col items-end gap-2">
				<div class="flex items-center gap-2">
					<div class="w-1 h-4 bg-green-500 rounded-full"></div>
					<h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">Kategorie auswählen</h3>
				</div>
				<div class="bg-white dark:bg-slate-800 rounded-xl p-1 shadow-md w-full lg:w-72">
					<Select.Root
						type="single"
						bind:value={selectedActivityCategory}
						items={categoryOptions}
					>
						<Select.Trigger class="w-full h-10 bg-white dark:bg-slate-800 border-0 rounded-lg px-3 hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors">
							<span class="flex items-center gap-2 text-slate-700 dark:text-slate-300">
								<span class="text-base">{categoryOptions.find(opt => opt.value === selectedActivityCategory)?.icon || '📊'}</span>
								<span class="text-sm">{categoryOptions.find(opt => opt.value === selectedActivityCategory)?.label || 'Kategorie auswählen'}</span>
							</span>
						</Select.Trigger>
					<Select.Portal>
						<Select.Content class="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl shadow-xl">
							<Select.Viewport>
								{#each categoryOptions as option}
									<Select.Item value={option.value} label={option.label} class="px-4 py-3 hover:bg-slate-100 dark:hover:bg-slate-700 cursor-pointer flex items-center gap-3">
										<span class="text-lg">{option.icon}</span>
										<div>
											<div class="font-medium">{option.label}</div>
										</div>
									</Select.Item>
								{/each}
							</Select.Viewport>
						</Select.Content>
					</Select.Portal>
				</Select.Root>
			</div>
			</div>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={activityFunction}
		/>
	</div>

	<!-- Call to Orders Statistics Section -->
	<div class="bg-gradient-to-br from-purple-50 to-violet-100 dark:from-purple-900/20 dark:to-violet-900/20 rounded-2xl p-8 shadow-lg border border-purple-200/50 dark:border-purple-800/50 mb-8">
		<div class="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-4 mb-6">
			<div class="flex items-center gap-3">
				<div class="w-12 h-12 bg-purple-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
					🔔
				</div>
				<div>
					<h2 class="text-2xl font-bold text-slate-800 dark:text-slate-200">Ordnungsrufstatistiken</h2>
					<p class="text-sm text-slate-600 dark:text-slate-400">Analyse der Ordnungsrufe im Parlament</p>
				</div>
			</div>
			<!-- Kategorie-Auswahl kompakt rechts oben -->
			<div class="flex flex-col items-end gap-2">
				<div class="flex items-center gap-2">
					<div class="w-1 h-4 bg-purple-500 rounded-full"></div>
					<h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">Kategorie auswählen</h3>
				</div>
				<div class="bg-white dark:bg-slate-800 rounded-xl p-1 shadow-md w-full lg:w-72">
					<Select.Root
						type="single"
						bind:value={selectedCallsCategory}
						items={categoryOptions}
					>
						<Select.Trigger class="w-full h-10 bg-white dark:bg-slate-800 border-0 rounded-lg px-3 hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors">
							<span class="flex items-center gap-2 text-slate-700 dark:text-slate-300">
								<span class="text-base">{categoryOptions.find(opt => opt.value === selectedCallsCategory)?.icon || '📊'}</span>
								<span class="text-sm">{categoryOptions.find(opt => opt.value === selectedCallsCategory)?.label || 'Kategorie auswählen'}</span>
							</span>
						</Select.Trigger>
					<Select.Portal>
						<Select.Content class="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl shadow-xl">
							<Select.Viewport>
								{#each categoryOptions as option}
									<Select.Item value={option.value} label={option.label} class="px-4 py-3 hover:bg-slate-100 dark:hover:bg-slate-700 cursor-pointer flex items-center gap-3">
										<span class="text-lg">{option.icon}</span>
										<div>
											<div class="font-medium">{option.label}</div>
										</div>
									</Select.Item>
								{/each}
							</Select.Viewport>
						</Select.Content>
					</Select.Portal>
				</Select.Root>
			</div>
			</div>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={callsFunction}
		/>
	</div>

	<!-- Speech Complexity Statistics Section -->
	<div class="bg-gradient-to-br from-rose-50 to-pink-100 dark:from-rose-900/20 dark:to-pink-900/20 rounded-2xl p-8 shadow-lg border border-rose-200/50 dark:border-rose-800/50 mb-8">
		<div class="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-4 mb-6">
			<div class="flex items-center gap-3">
				<div class="w-12 h-12 bg-rose-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
					🧠
				</div>
				<div>
					<h2 class="text-2xl font-bold text-slate-800 dark:text-slate-200">Redekomplexität</h2>
					<p class="text-sm text-slate-600 dark:text-slate-400">Analyse der Komplexität der Reden</p>
				</div>
			</div>
			<!-- Kategorie-Auswahl kompakt rechts oben -->
			<div class="flex flex-col items-end gap-2">
				<div class="flex items-center gap-2">
					<div class="w-1 h-4 bg-rose-500 rounded-full"></div>
					<h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">Kategorie auswählen</h3>
				</div>
				<div class="bg-white dark:bg-slate-800 rounded-xl p-1 shadow-md w-full lg:w-72">
					<Select.Root
						type="single"
						bind:value={selectedComplexityCategory}
						items={categoryOptions}
					>
						<Select.Trigger class="w-full h-10 bg-white dark:bg-slate-800 border-0 rounded-lg px-3 hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors">
							<span class="flex items-center gap-2 text-slate-700 dark:text-slate-300">
								<span class="text-base">{categoryOptions.find(opt => opt.value === selectedComplexityCategory)?.icon || '📊'}</span>
								<span class="text-sm">{categoryOptions.find(opt => opt.value === selectedComplexityCategory)?.label || 'Kategorie auswählen'}</span>
							</span>
						</Select.Trigger>
					<Select.Portal>
						<Select.Content class="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl shadow-xl">
							<Select.Viewport>
								{#each categoryOptions as option}
									<Select.Item value={option.value} label={option.label} class="px-4 py-3 hover:bg-slate-100 dark:hover:bg-slate-700 cursor-pointer flex items-center gap-3">
										<span class="text-lg">{option.icon}</span>
										<div>
											<div class="font-medium">{option.label}</div>
										</div>
									</Select.Item>
								{/each}
							</Select.Viewport>
						</Select.Content>
					</Select.Portal>
				</Select.Root>
			</div>
			</div>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={complexityFunction}
		/>
	</div>

	<!-- Speech Time Statistics Section -->
	<div class="bg-gradient-to-br from-cyan-50 to-sky-100 dark:from-cyan-900/20 dark:to-sky-900/20 rounded-2xl p-8 shadow-lg border border-cyan-200/50 dark:border-cyan-800/50 mb-8">
		<div class="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-4 mb-6">
			<div class="flex items-center gap-3">
				<div class="w-12 h-12 bg-cyan-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
					⏱️
				</div>
				<div>
					<h2 class="text-2xl font-bold text-slate-800 dark:text-slate-200">Redezeit</h2>
					<p class="text-sm text-slate-600 dark:text-slate-400">Analyse der Redezeit im Parlament</p>
				</div>
			</div>
			<!-- Kategorie-Auswahl kompakt rechts oben -->
			<div class="flex flex-col items-end gap-2">
				<div class="flex items-center gap-2">
					<div class="w-1 h-4 bg-cyan-500 rounded-full"></div>
					<h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">Kategorie auswählen</h3>
				</div>
				<div class="bg-white dark:bg-slate-800 rounded-xl p-1 shadow-md w-full lg:w-72">
					<Select.Root
						type="single"
						bind:value={selectedSpeechTimeCategory}
						items={categoryOptions}
					>
						<Select.Trigger class="w-full h-10 bg-white dark:bg-slate-800 border-0 rounded-lg px-3 hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors">
							<span class="flex items-center gap-2 text-slate-700 dark:text-slate-300">
								<span class="text-base">{categoryOptions.find(opt => opt.value === selectedSpeechTimeCategory)?.icon || '📊'}</span>
								<span class="text-sm">{categoryOptions.find(opt => opt.value === selectedSpeechTimeCategory)?.label || 'Kategorie auswählen'}</span>
							</span>
						</Select.Trigger>
						<Select.Portal>
							<Select.Content class="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl shadow-xl">
								<Select.Viewport>
									{#each categoryOptions as option}
										<Select.Item value={option.value} label={option.label} class="px-4 py-3 hover:bg-slate-100 dark:hover:bg-slate-700 cursor-pointer flex items-center gap-3">
											<span class="text-lg">{option.icon}</span>
											<div>
												<div class="font-medium">{option.label}</div>
											</div>
										</Select.Item>
									{/each}
								</Select.Viewport>
							</Select.Content>
						</Select.Portal>
					</Select.Root>
				</div>
			</div>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={speechTimeFunction}
		/>
	</div>

	<!-- Total Speeches Statistics Section -->
	<div class="bg-gradient-to-br from-emerald-50 to-teal-100 dark:from-emerald-900/20 dark:to-teal-900/20 rounded-2xl p-8 shadow-lg border border-emerald-200/50 dark:border-emerald-800/50 mb-8">
		<div class="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-4 mb-6">
			<div class="flex items-center gap-3">
				<div class="w-12 h-12 bg-emerald-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
					🎤
				</div>
				<div>
					<h2 class="text-2xl font-bold text-slate-800 dark:text-slate-200">Redenanzahl</h2>
					<p class="text-sm text-slate-600 dark:text-slate-400">Analyse der Anzahl der Reden</p>
				</div>
			</div>
			<!-- Kategorie-Auswahl kompakt rechts oben -->
			<div class="flex flex-col items-end gap-2">
				<div class="flex items-center gap-2">
					<div class="w-1 h-4 bg-emerald-500 rounded-full"></div>
					<h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">Kategorie auswählen</h3>
				</div>
				<div class="bg-white dark:bg-slate-800 rounded-xl p-1 shadow-md w-full lg:w-72">
					<Select.Root
						type="single"
						bind:value={selectedSpeechesCategory}
						items={categoryOptions}
					>
						<Select.Trigger class="w-full h-10 bg-white dark:bg-slate-800 border-0 rounded-lg px-3 hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors">
							<span class="flex items-center gap-2 text-slate-700 dark:text-slate-300">
								<span class="text-base">{categoryOptions.find(opt => opt.value === selectedSpeechesCategory)?.icon || '📊'}</span>
								<span class="text-sm">{categoryOptions.find(opt => opt.value === selectedSpeechesCategory)?.label || 'Kategorie auswählen'}</span>
							</span>
						</Select.Trigger>
					<Select.Portal>
						<Select.Content class="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl shadow-xl">
							<Select.Viewport>
								{#each categoryOptions as option}
									<Select.Item value={option.value} label={option.label} class="px-4 py-3 hover:bg-slate-100 dark:hover:bg-slate-700 cursor-pointer flex items-center gap-3">
										<span class="text-lg">{option.icon}</span>
										<div>
											<div class="font-medium">{option.label}</div>
										</div>
									</Select.Item>
								{/each}
							</Select.Viewport>
						</Select.Content>
					</Select.Portal>
				</Select.Root>
			</div>
			</div>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={speechesFunction}
		/>
	</div>
</div>
