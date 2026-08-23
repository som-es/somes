<script lang="ts">
	import type { DbAiSummary } from '$lib/ai_summary_types';
	import { Popover } from 'bits-ui';
	import { t } from '$lib/i18n/i18n.svelte';

	interface Props {
		aiSummary: Pick<DbAiSummary, 'model_used' | 'version' | 'generated_at'>;
		useTitleHover?: boolean;
		aiGenText?: string;
		align?: 'start' | 'center' | 'end';
	}

	let {
		aiSummary,
		useTitleHover = false,
		aiGenText = t('aiSummary.defaultText'),
		align = 'center'
	}: Props = $props();

	let titleHover = $derived(useTitleHover ? aiGenText : '');

	let generatedAtDate = $derived(new Date(aiSummary.generated_at));
</script>

<Popover.Root>
	<Popover.Trigger openOnHover openDelay={100} class="text-3xl" title={titleHover}
		>⚠</Popover.Trigger
	>
	<Popover.Content
		{align}
		collisionPadding={8}
		class="z-50! w-72 card bg-primary-300-700 p-4 shadow-xl"
		data-popup="emphasisAi"
	>
		<div class="z-50 text-base font-bold">{aiGenText}</div>
		<div class="flex flex-col flex-wrap text-sm! font-thin!">
			<span
				>{t('aiSummary.generatedAt')} {generatedAtDate.toLocaleDateString()}
				{generatedAtDate.toLocaleTimeString()}</span
			>
			<span>{t('aiSummary.model')} {aiSummary.model_used}</span>
			<span>{t('aiSummary.version')} {aiSummary.version}</span>
		</div>
	</Popover.Content>
</Popover.Root>
