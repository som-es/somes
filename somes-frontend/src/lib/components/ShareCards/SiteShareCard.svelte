<script lang="ts">
	import type { ShareImage } from './types';

	interface Props {
		logo: string;
		seats: ShareImage | null;
		headline: string;
		tagline: string;
	}

	let { logo, seats, headline, tagline }: Props = $props();
</script>

<div
	style="display: flex; position: relative; width: 100%; height: 100%; background: rgb(36 40 62); font-family: 'Noto Sans'; color: rgb(255 255 255);"
>
	{#if seats}
		<img
			src={seats.src}
			width={seats.width}
			height={seats.height}
			alt=""
			style="position: absolute; right: -60px; bottom: 70px;"
		/>
	{/if}

	<div
		style="display: flex; flex-direction: column; justify-content: center; width: 700px; height: 100%; padding: 0 0 0 80px;"
	>
		<div style="display: flex; align-items: center; gap: 28px;">
			<img src={logo} width="86" height="110" alt="" />
			<div style="display: flex; margin-top: -20px; font-size: 104px; line-height: 1;">somes</div>
		</div>
		<div
			style="display: flex; margin-top: 44px; font-size: 42px; font-weight: 700; line-height: 1.2;"
		>
			{headline}
		</div>
		<div
			style="display: flex; margin-top: 22px; font-size: 26px; line-height: 1.4; color: rgb(195 205 217);"
		>
			{tagline}
		</div>
		<div
			style="display: flex; margin-top: 40px; font-size: 26px; font-weight: 700; color: rgb(217 224 232);"
		>
			somes.at
		</div>
	</div>
</div>
