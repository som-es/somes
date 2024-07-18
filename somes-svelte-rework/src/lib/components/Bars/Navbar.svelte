<script lang="ts">
	import { AppBar, LightSwitch } from '@skeletonlabs/skeleton';
	import somesTextIcon from '$lib/assets/somes_with_text.svg?raw';
</script>

<AppBar slotTrail="!space-x-2">
	<svelte:fragment slot="lead">
		<a class="mx-4 fill-current stroke-current w-32" href="/">
			{@html somesTextIcon}
		</a>
	</svelte:fragment>
	<svelte:fragment slot="trail">
		<LightSwitch />
	</svelte:fragment>
</AppBar>
