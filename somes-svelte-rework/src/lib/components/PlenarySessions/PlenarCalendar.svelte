<script lang="ts">
	import Calendar from "../Calendar/Calendar.svelte";
	import type { Day } from "../Calendar/types";

	const date = new Date();
	let year = date.getFullYear();
	let month = date.getMonth();

	var daysInThisMonth = new Date(year, month+1, 0).getDate();

    const months = ["Jänner", "Februar", "März", "April", "Mai", "Juni", "Juli", "August", "September", "Oktober", "November", "Dezember"];

	let days: Day[] = []

	const addDays = () => {
        days = []
        for (let i = 0; i < daysInThisMonth; i++) {
            let testDate = new Date(year,month,i+1);
            console.log(testDate);
            console.log(testDate.getDay());
            if (testDate.getDay() > 0 && testDate.getDay() < 6) {
                days.push({name: (i +1).toString(), enabled: true, date: testDate, item: null})
            }
        }

        days[17].item = { title: "Plenarsitzung", classes: "badge text-xs bg-tertiary-500"}
        days[18].item = { title: "Plenarsitzung", classes: "badge text-xs bg-tertiary-500"}
    }

    $: month, year,addDays();

</script>

{#if days}
    <Calendar 
        bind:month 
        bind:year 
        title={`${months[month % 12]} ${year}`} {days} 
        headers={["Mo", "Di", "Mi", "Do", "Fr"]} 
    />
{/if}