<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		logo: string;
		heading: string;
		children: Snippet;
	}

	let { logo, heading, children }: Props = $props();
</script>

<div
	style="display: flex; width: 100%; height: 100%; background: rgb(222 223 226); font-family: 'Noto Sans'; color: rgb(17 24 39);"
>
	<div
		style="display: flex; flex-direction: column; align-items: center; width: 112px; flex-shrink: 0; padding-top: 44px; background: rgb(36 40 62);"
	>
		<img src={logo} width="56" height="72" alt="" />
	</div>

	<div
		style="display: flex; flex-direction: column; flex-grow: 1; min-width: 0; padding: 40px 48px 36px;"
	>
		<div style="display: flex; justify-content: space-between; align-items: center;">
			<div style="display: flex; font-size: 26px; color: rgb(55 65 81);">{heading}</div>
			<div style="display: flex; font-size: 24px; font-weight: 700; color: rgb(36 40 62);">
				somes.at
			</div>
		</div>

		<div
			style="display: flex; flex-grow: 1; gap: 32px; margin-top: 18px; padding: 34px 38px; border-radius: 20px; background: rgb(195 205 217); box-shadow: 0 4px 14px rgba(36, 40, 62, 0.18);"
		>
			{@render children()}
		</div>
	</div>
</div>
