<script lang="ts">
	import { aiViewEnabledStore } from "$lib/stores/stores";

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
	title={aiViewEnabledStore.value ? "KI-Ansicht deaktivieren" : "KI-Ansicht aktivieren"}
	onclick={toggleAiView}
	class="rounded-input hover:bg-primary-600 rounded-xl focus-visible:ring-foreground focus-visible:ring-offset-background relative inline-flex h-10 w-10 cursor-pointer items-center justify-center px-2 transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-hidden"
>
	<div class="absolute inline-flex h-full w-full items-center justify-center">
		{#if aiViewEnabledStore.value}
			<!-- AI/Brain icon for enabled state -->
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="1em"
				height="1em"
				fill="white"
				viewBox="0 0 256 256"
				class="size-6"
				aria-label="AI enabled"
			>
				<rect width="256" height="256" fill="none"></rect>
				<path d="M240,120a48.05,48.05,0,0,0-48-48h-4.72A63.82,63.82,0,0,0,128,16h-8A64.07,64.07,0,0,0,56.72,72H48a48.05,48.05,0,0,0-48,48,24,24,0,0,0,24,24v8a24,24,0,0,0,24,24h8a8,8,0,0,0,8-8V104a8,8,0,0,0-8-8H56.8A48.08,48.08,0,0,1,104,56h8a48.08,48.08,0,0,1,47.2,40H144a8,8,0,0,0-8,8v64a8,8,0,0,0,8,8h8a24,24,0,0,0,24-24v-8a24,24,0,0,0,24-24h8a24,24,0,0,0,24-24,48.05,48.05,0,0,0-48-48ZM32,128a8,8,0,0,1-8-8,32.06,32.06,0,0,1,32-32h8v40Zm176,24a8,8,0,0,1-8,8h-8V112h8a32.06,32.06,0,0,1,32,32A8,8,0,0,1,208,152Z"></path>
			</svg>
		{:else}
			<!-- Document/Text icon for disabled state -->
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="1em"
				height="1em"
				fill="white"
				viewBox="0 0 256 256"
				class="size-6"
				aria-label="AI disabled"
			>
				<rect width="256" height="256" fill="none"></rect>
				<path d="M216,40H72A16,16,0,0,0,56,56V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM72,56H216V200H72ZM56,216V40H40V216a16,16,0,0,0,16,16H216V216H56ZM184,96a8,8,0,0,1-8,8H112a8,8,0,0,1,0-16h64A8,8,0,0,1,184,96Zm0,32a8,8,0,0,1-8,8H112a8,8,0,0,1,0-16h64A8,8,0,0,1,184,128Zm0,32a8,8,0,0,1-8,8H112a8,8,0,0,1,0-16h64A8,8,0,0,1,184,160Z"></path>
			</svg>
		{/if}
	</div>
</button>
