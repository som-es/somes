<script lang="ts">
	import { Dialog } from 'bits-ui';
	import austriaFlagIcon from '$lib/assets/parliament_switch/austria_map_flag.svg?raw';
	import euFlagIcon from '$lib/assets/parliament_switch/EU_map_flag.svg?raw';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import { goto } from '$app/navigation';
	import { getParliament, type Parliament } from '$lib/api/parliament';
	import { persisted } from '$lib/persisted.svelte';
	import { parliamentModalOpenStore } from '$lib/caching/stores/stores.svelte';
	import { t } from '$lib/i18n/i18n.svelte';

	let parliament = $derived(getParliament());

	const parliamentModalSeen = persisted<boolean>('parliamentModalSeen', false);

	$effect(() => {
		if (!parliamentModalSeen.value) {
			parliamentModalOpenStore.value = true;
			parliamentModalSeen.value = true;
		}
	});

	const options: { id: Parliament; icon: string }[] = [
		{ id: 'at', icon: austriaFlagIcon },
		{ id: 'eu', icon: euFlagIcon }
	];

	function choose(p: Parliament) {
		parliamentModalOpenStore.value = false;
		if (p !== parliament) {
			goto(`/${p}/home`);
		}
	}
</script>

<Dialog.Root bind:open={parliamentModalOpenStore.value}>
	<Dialog.Portal>
		<Dialog.Overlay
			class="fixed inset-0 z-70 bg-black/70 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
		/>
		<Dialog.Content
			class="fixed top-[50%] left-[50%] z-70 w-[94vw] max-w-md translate-x-[-50%] translate-y-[-50%] rounded-xl bg-white p-4 text-gray-900 shadow-xl outline-hidden data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 dark:bg-surface-600 dark:text-gray-50"
		>
			<div class="mb-3 flex items-center justify-between gap-3">
				<Dialog.Title class="text-lg font-semibold">
					{t('nav.menu.parliament')}
				</Dialog.Title>
				<Dialog.Close>
					<ModalCloseButton class="p-1" />
				</Dialog.Close>
			</div>
			<Dialog.Description class="sr-only">
				{t('nav.menu.parliament')}
			</Dialog.Description>

			<div class="grid grid-cols-2 gap-3">
				{#each options as option}
					<button
						onclick={() => choose(option.id)}
						class="flex cursor-pointer flex-col items-center gap-2 rounded-xl border-2 p-4 transition-colors {parliament ===
						option.id
							? 'border-primary-600 bg-primary-600/10'
							: 'border-gray-200 hover:border-gray-300 hover:bg-gray-100 dark:border-surface-500 dark:hover:border-surface-400 dark:hover:bg-surface-500'}"
					>
						<span class="h-16 w-16 [&_svg]:h-full [&_svg]:w-full">
							{@html option.icon}
						</span>
						<span class="text-sm font-medium">
							{option.id === 'at' ? t('nav.nationalCouncil') : t('user.parliament.eu')}
						</span>
					</button>
				{/each}
			</div>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
