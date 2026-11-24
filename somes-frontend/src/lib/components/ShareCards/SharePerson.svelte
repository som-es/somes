<script lang="ts">
	import type { SharePersonData } from './types';

	let { person, photoSize = 150 }: { person: SharePersonData; photoSize?: number } = $props();
</script>

<div
	style="display: flex; flex-direction: column; padding: 22px 24px; border-radius: 16px; background: rgb(217 224 232);"
>
	<div style="display: flex; flex-direction: column; align-items: center; padding-bottom: 14px;">
		{#if person.photo}
			<img
				src={person.photo}
				width={photoSize}
				height={photoSize}
				style="border-radius: 9999px; object-fit: cover;"
				alt=""
			/>
		{:else}
			<div
				style="display: flex; align-items: center; justify-content: center; width: {photoSize}px; height: {photoSize}px; border-radius: 9999px; font-size: {Math.round(
					photoSize / 3
				)}px; font-weight: 700; color: rgb(149 134 121); background: rgb(250 233 218);"
			>
				{person.name.charAt(0)}
			</div>
		{/if}
		{#if person.copyright}
			<div style="display: flex; margin-top: 4px; font-size: 13px; color: rgb(55 65 81);">
				© {person.copyright}
			</div>
		{/if}
	</div>
	<div style="display: flex; font-size: 26px; font-weight: 700; line-height: 1.25;">
		{person.age === null ? person.name : `${person.name} - ${person.age}`}
	</div>
	<div style="display: flex; align-items: center; gap: 10px; margin-top: 6px; font-size: 22px;">
		<div
			style="display: flex; width: 10px; height: 10px; margin-left: 6px; border-radius: 9999px; background: {person.partyColor};"
		></div>
		<div style="display: flex; color: rgb(31 41 55);">{person.party}</div>
	</div>
	{#each person.lines as line (line)}
		<div style="display: block; margin-top: 8px; font-size: 20px; line-height: 1.3; line-clamp: 2;">
			{line}
		</div>
	{/each}
</div>
