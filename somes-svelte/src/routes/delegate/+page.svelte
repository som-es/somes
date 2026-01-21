<script lang="ts">
	import { delegates } from "$lib/api";
	import { onMount } from "svelte";
	import type { Delegate } from "$lib/types";
	import { t } from "$lib/translations";
	import Parliament from "../../components/Parliament.svelte";

	let dels: Delegate[];

	onMount(async function () {
		const austrianDelegates = await delegates();
		// use local storage to cache the delegates
		dels = austrianDelegates.filter((delegate) => delegate.council === "nr");
	});
</script>

{#if dels}
	<Parliament {dels} seats={[20, 27, 37, 43, 48, 54]} />
{:else}
	<p class="loading">{$t("common.loading")}.</p>
{/if}

<style>
</style>
