<script lang="ts">
	import ShareFrame from './ShareFrame.svelte';
	import type { ShareIcons, SharePersonData } from './types';

	interface Props {
		icons: ShareIcons;
		heading: string;
		person: SharePersonData;
	}

	let { icons, heading, person }: Props = $props();
</script>

<ShareFrame logo={icons.logo} {heading}>
	<div style="display: flex; flex-direction: column; align-items: center; flex-shrink: 0;">
		{#if person.photo}
			<img
				src={person.photo}
				width="250"
				height="250"
				style="border-radius: 9999px; object-fit: cover;"
				alt=""
			/>
		{:else}
			<div
				style="display: flex; align-items: center; justify-content: center; width: 250px; height: 250px; border-radius: 9999px; font-size: 90px; font-weight: 700; color: rgb(149 134 121); background: rgb(250 233 218);"
			>
				{person.name.charAt(0)}
			</div>
		{/if}
		{#if person.copyright}
			<div style="display: flex; margin-top: 6px; font-size: 14px; color: rgb(55 65 81);">
				© {person.copyright}
			</div>
		{/if}
	</div>

	<div
		style="display: flex; flex-direction: column; justify-content: center; flex-grow: 1; flex-basis: 0; min-width: 0;"
	>
		<div
			style="display: block; font-size: 46px; font-weight: 700; line-height: 1.15; line-clamp: 2;"
		>
			{person.age === null ? person.name : `${person.name} - ${person.age}`}
		</div>
		<div style="display: flex; align-items: center; gap: 12px; margin-top: 12px; font-size: 30px;">
			<div
				style="display: flex; width: 14px; height: 14px; margin-left: 6px; border-radius: 9999px; background: {person.partyColor};"
			></div>
			<div style="display: flex; color: rgb(31 41 55);">{person.party}</div>
		</div>
		{#each person.lines as line (line)}
			<div
				style="display: block; margin-top: 12px; font-size: 24px; line-height: 1.3; line-clamp: 2;"
			>
				{line}
			</div>
		{/each}
	</div>
</ShareFrame>
