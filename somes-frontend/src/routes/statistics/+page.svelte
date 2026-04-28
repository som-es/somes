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
		{ value: 'delegate', label: 'Pro Abgeordneten' },
		{ value: 'party', label: 'Nach Parteien' },
		{ value: 'gender', label: 'Nach Gender' },
		{ value: 'age', label: 'Nach Alter' },
		{ value: 'legis', label: 'Nach Legislaturperiode' }
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
	<div class="bg-card rounded-xl p-6 shadow-sm mb-8">
		<div class="flex items-center justify-between mb-6">
			<h2 class="text-2xl font-bold">👥 Altersstatistiken</h2>
			<Select.Root
				type="single"
				bind:value={selectedAgeCategory}
				items={categoryOptions}
			>
				<Select.Trigger class="w-64">
					<span>Kategorie wählen</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each categoryOptions as option}
								<Select.Item value={option.value} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={ageFunction}
			title={ageTitle}
		/>
	</div>

	<!-- Absences Statistics Section -->
	<div class="bg-card rounded-xl p-6 shadow-sm mb-8">
		<div class="flex items-center justify-between mb-6">
			<h2 class="text-2xl font-bold">📋 Abwesenheitsstatistiken</h2>
			<Select.Root
				type="single"
				bind:value={selectedAbsencesCategory}
				items={categoryOptions}
			>
				<Select.Trigger class="w-64">
					<span>Kategorie wählen</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each categoryOptions as option}
								<Select.Item value={option.value} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={absencesFunction}
			title={absencesTitle}
		/>
	</div>

	<!-- Activity Statistics Section -->
	<div class="bg-card rounded-xl p-6 shadow-sm mb-8">
		<div class="flex items-center justify-between mb-6">
			<h2 class="text-2xl font-bold">⚡ Aktivitätsstatistiken</h2>
			<Select.Root
				type="single"
				bind:value={selectedActivityCategory}
				items={categoryOptions}
			>
				<Select.Trigger class="w-64">
					<span>Kategorie wählen</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each categoryOptions as option}
								<Select.Item value={option.value} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={activityFunction}
			title={activityTitle}
		/>
	</div>

	<!-- Call to Orders Statistics Section -->
	<div class="bg-card rounded-xl p-6 shadow-sm mb-8">
		<div class="flex items-center justify-between mb-6">
			<h2 class="text-2xl font-bold">🔔 Ordnungsrufstatistiken</h2>
			<Select.Root
				type="single"
				bind:value={selectedCallsCategory}
				items={categoryOptions}
			>
				<Select.Trigger class="w-64">
					<span>Kategorie wählen</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each categoryOptions as option}
								<Select.Item value={option.value} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={callsFunction}
			title={callsTitle}
		/>
	</div>

	<!-- Speech Complexity Statistics Section -->
	<div class="bg-card rounded-xl p-6 shadow-sm mb-8">
		<div class="flex items-center justify-between mb-6">
			<h2 class="text-2xl font-bold">🧠 Sprachkomplexitätsstatistiken</h2>
			<Select.Root
				type="single"
				bind:value={selectedComplexityCategory}
				items={categoryOptions}
			>
				<Select.Trigger class="w-64">
					<span>Kategorie wählen</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each categoryOptions as option}
								<Select.Item value={option.value} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={complexityFunction}
			title={complexityTitle}
		/>
	</div>

	<!-- Speech Time Statistics Section -->
	<div class="bg-card rounded-xl p-6 shadow-sm mb-8">
		<div class="flex items-center justify-between mb-6">
			<h2 class="text-2xl font-bold">⏱️ Redezeitstatistiken</h2>
			<Select.Root
				type="single"
				bind:value={selectedSpeechTimeCategory}
				items={categoryOptions}
			>
				<Select.Trigger class="w-64">
					<span>Kategorie wählen</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each categoryOptions as option}
								<Select.Item value={option.value} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={speechTimeFunction}
			title={speechTimeTitle}
		/>
	</div>

	<!-- Total Speeches Statistics Section -->
	<div class="bg-card rounded-xl p-6 shadow-sm mb-8">
		<div class="flex items-center justify-between mb-6">
			<h2 class="text-2xl font-bold">🎤 Redenzahlstatistiken</h2>
			<Select.Root
				type="single"
				bind:value={selectedSpeechesCategory}
				items={categoryOptions}
			>
				<Select.Trigger class="w-64">
					<span>Kategorie wählen</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each categoryOptions as option}
								<Select.Item value={option.value} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>
		<DelegateBarChartControl
			height={400}
			delegateMakeRequest={speechesFunction}
			title={speechesTitle}
		/>
	</div>
</div>
