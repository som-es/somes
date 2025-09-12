<script lang="ts">
	import { popup, type PopupSettings } from "@skeletonlabs/skeleton";
	import PlenarCalendar from "./PlenarCalendar.svelte";

    const plenarCalendar: PopupSettings = {
		event: 'click',
		target: 'plenarCalendar',
		placement: 'bottom',
        closeQuery: "none",
	};
</script>
<div class="mt-2 flex items-center gap-4 bg-primary-300 dark:bg-primary-500 p-3 rounded-[0.9rem] justify-between w-full">
    <div>
        <h1 class="mt-2 text-2xl sm:text-4xl font-bold">Nächste Nationalratssitzung</h1>
        <h2 class="text-xl sm:text-2xl">am 24.09.2025</h2>
    </div>
    <button class="btn sm:btn-lg variant-filled mt-1" use:popup={plenarCalendar} on:click={() => {}}
        >Sitzungskalender</button
    >

    <div class=" z-50" data-popup="plenarCalendar">
        <PlenarCalendar />
    </div>
</div>