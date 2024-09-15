<script lang="ts">
	import { areDeeplyEqual } from '$lib/types';

	// import ListBoxItem from "../UI/ListBoxItem.svelte";
	import { ListBoxItem } from '@skeletonlabs/skeleton';

	export let partyGroup: string[];
	export let twoTimeWeightsGroup: string[];
	export let justification: [string | null, number];
    export let color: string | null = null;

	let value = justification[1].toString();

	let twoTimesChecked: boolean = false;

	function updateGroup(checked: boolean) {
		const index = twoTimeWeightsGroup.indexOf(value);
		if (checked) {
			if (index < 0) {
				twoTimeWeightsGroup.push(value);
                twoTimeWeightsGroup = twoTimeWeightsGroup;
			}
		} else {
			if (index >= 0) {
				twoTimeWeightsGroup.splice(index, 1);
                twoTimeWeightsGroup = twoTimeWeightsGroup;
			}
		}
	}

	$: defaultBgColor = twoTimesChecked ? 'bg-primary-600' : 'bg-primary-100';
	$: updateGroup(twoTimesChecked);
	$: twoTimesChecked = twoTimeWeightsGroup.some((groupVal: unknown) =>
		areDeeplyEqual(value, groupVal)
	);
</script>

<ListBoxItem
	hover="hover:bg-secondary-200"
	class=" !rounded-[3rem] w-full min-w-full bg-primary-300 !text-black my-3"
	spacing="space-y-4"
	active="bg-secondary-500"
	bind:group={partyGroup}
	name="justification"
	{value}
	>{justification[0]}

	<svelte:fragment slot="trail">
        {#if color}
            <button
                style="color: {color}; background-color: {color}"
                class="p-3 px-4 rounded-full"
            >2x
            </button>
        {/if}
		<button
			on:click={(e) => {
				e.preventDefault();
				// e.stopPropagation();
				e.stopImmediatePropagation();
				twoTimesChecked = !twoTimesChecked;
			}}
			class="p-3 px-4 rounded-full hover:{twoTimesChecked ? '' : 'bg-primary-200'} {defaultBgColor}"
		>
			2x
		</button>
	</svelte:fragment>
</ListBoxItem>
