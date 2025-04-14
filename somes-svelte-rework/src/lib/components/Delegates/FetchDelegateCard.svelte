<script lang="ts">
	import type { Delegate, DelegateQA } from '$lib/types';
	import { onMount } from 'svelte';
	import { delegate_by_id, errorToNull } from '$lib/api/api';
	import DelegateCard from './DelegateCard.svelte';

	export let delegateId: number;
	export let onlyTop: boolean = false;
	export let showQA: boolean = false;
	export let showAI: boolean = true;
	export let questions: DelegateQA[] = [];
	export let showMoreDetailsBtn = false;
	export let showImg = true;
	export let showAge = true;
	export let title: string | null = null;

	let delegate: Delegate | null = null;

	onMount(async () => {
		delegate = errorToNull(await delegate_by_id(delegateId));
	});
</script>

{#if delegate}
	<DelegateCard
		{delegate}
		{onlyTop}
		{showQA}
		{showAI}
		{questions}
		{showMoreDetailsBtn}
		{showImg}
		{title}
		{showAge}
	/>
{/if}
