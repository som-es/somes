<script lang="ts">
	import ShareBadges from './ShareBadges.svelte';
	import ShareFrame from './ShareFrame.svelte';
	import type { ShareCounts, ShareIcons, ShareImage, ShareParty } from './types';

	interface Props {
		icons: ShareIcons;
		heading: string;
		title: string;
		summary: string;
		result: 'accepted' | 'rejected' | null;
		pills: string[];
		parties: ShareParty[];
		counts: ShareCounts;
		seats: ShareImage | null;
	}

	let { icons, heading, title, summary, result, pills, parties, counts, seats }: Props = $props();

	let countItems = $derived(
		[
			{ icon: icons.check, value: counts.inFavor },
			{ icon: icons.cross, value: counts.against },
			{ icon: icons.minus, value: counts.abstention }
		].filter((item, i) => i < 2 || item.value > 0)
	);
</script>

<ShareFrame logo={icons.logo} {heading}>
	<div style="display: flex; flex-direction: column; flex-grow: 1; flex-basis: 0; min-width: 0;">
		<div style="display: flex; align-items: flex-start; gap: 18px;">
			<div
				style="display: block; flex-grow: 1; flex-basis: 0; min-width: 0; font-size: 38px; font-weight: 700; line-height: 1.2; line-clamp: 3;"
			>
				{title}
			</div>
			{#if result}
				<img
					src={result === 'accepted' ? icons.check : icons.cross}
					width="56"
					height="56"
					alt=""
				/>
			{/if}
		</div>

		{#if summary}
			<div
				style="display: block; margin-top: 12px; font-size: 22px; line-height: 1.4; color: rgb(31 41 55); line-clamp: {seats
					? 2
					: 5};"
			>
				{summary}
			</div>
		{/if}

		<div style="display: flex; flex-grow: 1;"></div>

		{#if parties.length > 0}
			<div style="display: flex; flex-wrap: wrap; align-items: center; gap: 4px 18px;">
				{#each parties as party (party.name)}
					<div style="display: flex; align-items: center; gap: 6px; font-size: 23px;">
						{#if party.donut}
							<img src={party.donut} width="24" height="24" alt="" />
							<div style="display: flex;">{party.name}</div>
						{:else}
							<div style="display: flex;">{party.name}</div>
							<img src={party.inFavor ? icons.check : icons.cross} width="26" height="26" alt="" />
						{/if}
					</div>
				{/each}
			</div>
		{/if}

		<ShareBadges texts={pills} />
	</div>

	{#if seats}
		<div
			style="display: flex; flex-direction: column; align-items: center; justify-content: center; width: 320px; flex-shrink: 0;"
		>
			<img src={seats.src} width={seats.width} height={seats.height} alt="" />
			<div style="display: flex; align-items: center; gap: 22px; margin-top: 14px;">
				{#each countItems as item (item.icon)}
					<div
						style="display: flex; align-items: center; gap: 4px; font-size: 30px; font-weight: 700;"
					>
						<img src={item.icon} width="34" height="34" alt="" />
						<div style="display: flex;">{item.value}</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</ShareFrame>
