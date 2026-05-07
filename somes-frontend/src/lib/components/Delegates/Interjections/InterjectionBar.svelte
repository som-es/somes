<script lang="ts">
	import { delegate_by_id, errorToNull } from '$lib/api/api';
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, Interjection } from '$lib/types';

	interface Props {
		interjection: Interjection;
		ty: 'issued' | 'received';
	}

	let { interjection, ty }: Props = $props();

	let delegate = $state<Delegate | null>(null);
	let loading = $state(true);

	$effect(() => {
		loading = true;
		delegate_by_id(
			ty === 'issued' ? interjection.speaker_delegate_id : interjection.interjector_delegate_id
		).then((res) => {
			delegate = errorToNull(res);
			loading = false;
		});
	});
</script>

{#if !loading && delegate}
	<div
		class="flex w-full items-center gap-4 rounded-lg bg-primary-400 p-3 shadow-md dark:bg-primary-600"
	>
		<div
			class="flex min-w-28 flex-col items-center justify-center border-r border-primary-500/50 pr-4"
		>
			<span
				class="text-primary-950 text-[10px] font-bold tracking-widest uppercase dark:text-primary-100"
			>
				{ty === 'issued' ? 'Zwischenruf an' : 'Zwischenruf von'}
			</span>
			<div class="mt-1 flex items-center gap-2">
				<div
					class="h-2 w-2 rounded-full ring-1 ring-white/20"
					style="background-color: {partyToColor(delegate.party)}"
				></div>
				<span class="text-sm font-bold">
					{delegate.name}
				</span>
			</div>
		</div>

		<div class="flex-1 overflow-hidden">
			{#if interjection.interjection_text}
				<p class="text-sm leading-relaxed italic">
					&ldquo;{interjection.interjection_text}&rdquo;
				</p>
			{:else}
				<p class="text-sm font-medium text-tertiary-200">
					Kein Text zu diesem Zwischenruf verfügbar.
				</p>
			{/if}
		</div>
	</div>
{/if}
