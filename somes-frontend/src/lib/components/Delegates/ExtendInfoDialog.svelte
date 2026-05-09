<script lang="ts">
	import { Dialog } from 'bits-ui';
	import type { Snippet } from 'svelte';

	interface Props {
		title?: string;
		children?: Snippet;
	}

	let { title = 'Details', children }: Props = $props();
</script>

<Dialog.Root>
	<Dialog.Trigger>
		<div class="mr-auto btn bg-secondary-500 text-white">
			{title}
		</div>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Overlay
			class="fixed inset-0 z-70 bg-black/80 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
		/>
		<Dialog.Content
			class="
            fixed
            top-[50%] left-[50%] z-70
            h-[90vh] w-7xl max-w-[90%]
            translate-x-[-50%] translate-y-[-50%] overflow-hidden overflow-y-scroll rounded-lg bg-primary-100 shadow-lg outline-hidden data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 2xl:max-w-7xl dark:bg-gray-800"
		>
			{@render children?.()}
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
