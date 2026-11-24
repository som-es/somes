<script lang="ts">
	import ShareBadges from './ShareBadges.svelte';
	import ShareFrame from './ShareFrame.svelte';
	import SharePerson from './SharePerson.svelte';
	import type { ShareIcons, SharePersonData } from './types';

	interface Props {
		icons: ShareIcons;
		heading: string;
		title: string;
		subtitle: string;
		summaryHeading: string;
		summary: string;
		badges: string[];
		person: SharePersonData | null;
	}

	let { icons, heading, title, subtitle, summaryHeading, summary, badges, person }: Props =
		$props();
</script>

<ShareFrame logo={icons.logo} {heading}>
	<div style="display: flex; flex-direction: column; flex-grow: 1; flex-basis: 0; min-width: 0;">
		<div
			style="display: block; font-size: 38px; font-weight: 700; line-height: 1.2; line-clamp: 3;"
		>
			{title}
		</div>
		<div style="display: flex; margin-top: 6px; font-size: 20px; color: rgb(31 41 55);">
			{subtitle}
		</div>

		{#if summary}
			<div style="display: flex; margin-top: 18px; font-size: 24px; font-weight: 700;">
				{summaryHeading}
			</div>
			<div
				style="display: block; margin-top: 4px; font-size: 21px; line-height: 1.4; color: rgb(31 41 55); line-clamp: 3;"
			>
				{summary}
			</div>
		{/if}

		<div style="display: flex; flex-grow: 1;"></div>

		<ShareBadges texts={badges} wrapText />
	</div>

	{#if person}
		<div style="display: flex; flex-direction: column; width: 290px; flex-shrink: 0;">
			<SharePerson {person} photoSize={140} />
		</div>
	{/if}
</ShareFrame>
