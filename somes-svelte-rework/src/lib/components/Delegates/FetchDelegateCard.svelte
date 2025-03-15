<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, DelegateFavo, DelegateQA, Mandate } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import SButton from '../UI/SButton.svelte';
	import { currentDelegateStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import star from '$lib/assets/misc_icons/star.svg?raw';
	import starFilled from '$lib/assets/misc_icons/starFilled.svg?raw';
	import { onMount } from 'svelte';
	import { cachedDelegateFavos } from '$lib/caching/favos';
	import { addDelegateFavo, removeDelegateFavo } from '$lib/api/authed';
	import { delegate_by_id, errorToNull } from '$lib/api/api';
	import DelegateCard from './DelegateCard.svelte';

	export let delegateId: number;
	export let onlyTop: boolean = false;
	export let showQA: boolean = false;
	export let showAI: boolean = true;
	export let questions: DelegateQA[] = [];
	export let showMoreDetailsBtn = false;
	export let showImg = true;
	export let title: string | null = null;

	let delegate: Delegate | null = null;

	onMount(async () => {
		delegate = errorToNull(await delegate_by_id(delegateId))
	});

</script>

{#if delegate}
	<DelegateCard delegate={delegate}> {onlyTop} {showQA} {showAI} {questions} {showMoreDetailsBtn} {showImg} {title}</DelegateCard>
{/if}