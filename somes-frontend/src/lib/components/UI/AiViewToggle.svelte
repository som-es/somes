<script lang="ts">
	import { aiViewEnabledStore } from '$lib/stores/stores';

	function toggleAiView() {
		aiViewEnabledStore.value = !aiViewEnabledStore.value;
	}
</script>

<button
	data-button-root="true"
	role="switch"
	tabindex="0"
	aria-label="AI View Toggle"
	aria-checked={aiViewEnabledStore.value}
	title={aiViewEnabledStore.value ? 'KI-Ansichten deaktivieren' : 'KI-Ansichten aktivieren'}
	onclick={toggleAiView}
	class="rounded-input focus-visible:ring-foreground focus-visible:ring-offset-background relative inline-flex h-10 w-10 cursor-pointer items-center justify-center rounded-xl px-2 transition-colors hover:bg-primary-600 focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-hidden"
>
	<div class="absolute inline-flex h-full w-full items-center justify-center">
		{#if aiViewEnabledStore.value}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="1em"
				height="1em"
				viewBox="0 0 256 256"
				class="size-7"
				aria-hidden="true"
			>
				<text
					x="50%"
					y="50%"
					fill="white"
					font-family="sans-serif"
					font-weight="900"
					font-size="170"
					dominant-baseline="central"
					text-anchor="middle">AI</text
				>
			</svg>
		{:else}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="1em"
				height="1em"
				viewBox="0 0 256 256"
				class="size-7"
				aria-hidden="true"
			>
				<g opacity="0.5">
					<text
						x="50%"
						y="50%"
						fill="white"
						font-family="sans-serif"
						font-weight="900"
						font-size="170"
						dominant-baseline="central"
						text-anchor="middle">AI</text
					>
				</g>
				<line
					x1="50"
					y1="60"
					x2="206"
					y2="196"
					stroke="white"
					stroke-width="24"
					stroke-linecap="round"
				/>
				<line
					x1="50"
					y1="60"
					x2="206"
					y2="196"
					stroke="currentColor"
					stroke-width="8"
					class="text-primary-600"
					stroke-linecap="round"
				/>
			</svg>
		{/if}
	</div>
</button>
