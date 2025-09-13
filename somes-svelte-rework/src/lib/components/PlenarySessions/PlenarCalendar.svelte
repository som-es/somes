<script lang="ts">
	import { errorToNull } from '$lib/api/api';
	import Calendar from '../Calendar/Calendar.svelte';
	import type { Day } from '../Calendar/types';
	import { plenar_dates, type PlenarDate } from './api';

	const date = new Date();
	let year = date.getFullYear();
	let month = date.getMonth();

	var daysInThisMonth = new Date(year, month + 1, 0).getDate();

	const months = [
		'Jänner',
		'Februar',
		'März',
		'April',
		'Mai',
		'Juni',
		'Juli',
		'August',
		'September',
		'Oktober',
		'November',
		'Dezember'
	];

	let days: Day[] = [];
	let plenarDates: PlenarDate[] = [];

	// const updatePlenarDays = () => {

	// };

	const addDays = (plenarDates: PlenarDate[]) => {
		days = [];
		for (let i = 0; i < daysInThisMonth; i++) {
			let testDate = new Date(year, month, i + 1);
			if (testDate.getDay() > 0 && testDate.getDay() < 6) {
				const maybeDate =
					plenarDates.find((date) => {
						const newDate = new Date(date.date);
						return (
							newDate.getFullYear() == year &&
							newDate.getMonth() == month &&
							newDate.getDate() == i + 1
						);
					}) ?? null;
				const item =
					maybeDate == null
						? null
						: { title: 'Plenarsitzung', classes: 'badge text-xs bg-tertiary-500 text-black' };
				days.push({ name: (i + 1).toString(), enabled: true, date: testDate, item });
			}
		}

		// days[17].item = { title: "Plenarsitzung", classes: "badge text-xs bg-tertiary-500"}
		// days[18].item = { title: "Plenarsitzung", classes: "badge text-xs bg-tertiary-500"}
	};

	$: {
		month;
		year;
		plenar_dates(`${year}-${month}-${date.getDate()}`).then((dates) => {
			const newDates = errorToNull(dates);
			console.log(newDates);
			if (newDates !== null) {
				addDays(newDates);
			}
		});
	}
</script>

{#if days}
	<Calendar
		bind:month
		bind:year
		title={`${months[month % 12]} ${year}`}
		{days}
		headers={['Mo', 'Di', 'Mi', 'Do', 'Fr']}
	/>
{/if}
