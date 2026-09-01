<!-- Collapsible card on the user page for managing the personal MCP access token -->
<script lang="ts">
	import { isHasError } from '$lib/api/api';
	import { createMcpToken, hasMcpToken, revokeMcpToken } from '$lib/api/authed';
	import SButton from '$lib/components/UI/SButton.svelte';
	import { t } from '$lib/i18n';
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import arrowDown from '$lib/assets/misc_icons/down-arrow.svg?raw';

	let expanded = $state(false);
	let hasToken = $state<boolean | null>(null);
	let freshToken = $state<string | null>(null);
	let copied = $state(false);
	let busy = $state(false);
	let error = $state(false);

	onMount(async () => {
		const res = await hasMcpToken();
		if (!isHasError(res)) {
			hasToken = res.has_token;
		}
	});

	async function generate() {
		busy = true;
		error = false;
		copied = false;
		const res = await createMcpToken();
		busy = false;
		if (isHasError(res)) {
			error = true;
			return;
		}
		freshToken = res.access_token;
		hasToken = true;
	}

	async function revoke() {
		busy = true;
		error = false;
		const res = await revokeMcpToken();
		busy = false;
		if (isHasError(res)) {
			error = true;
			return;
		}
		hasToken = false;
		freshToken = null;
		copied = false;
	}

	async function copyToken() {
		if (!freshToken) return;
		try {
			await navigator.clipboard.writeText(freshToken);
			copied = true;
			setTimeout(() => (copied = false), 2000);
		} catch (e) {
			console.error('Clipboard write failed:', e);
		}
	}
</script>

<div class="w-full rounded-xl bg-primary-300 p-4 dark:bg-primary-500">
	<button
		class="flex w-full cursor-pointer flex-col gap-1 text-left"
		onclick={() => (expanded = !expanded)}
	>
		<div class="flex w-full flex-wrap items-center justify-between gap-3">
			<h2 class="text-xl font-bold text-gray-900 dark:text-gray-50">{t('user.mcp.title')}</h2>
			<div class="flex items-center gap-3">
				{#if hasToken !== null}
					<span
						class="rounded-full px-2.5 py-0.5 text-sm font-semibold {hasToken
							? 'bg-secondary-500 text-white'
							: 'bg-gray-300 text-gray-800'}"
					>
						{hasToken ? t('user.mcp.statusActive') : t('user.mcp.statusNone')}
					</span>
				{/if}
				<span class="w-4 transition-transform {expanded ? 'rotate-180' : ''}">
					{@html arrowDown}
				</span>
			</div>
		</div>
		<span class="text-sm text-gray-600 dark:text-gray-300">{t('user.mcp.description')}</span>
	</button>

	{#if expanded}
		<div transition:slide={{ duration: 200 }}>
			{#if freshToken}
				<div
					class="mt-3 rounded-lg border border-yellow-300 bg-yellow-50 p-3 dark:border-yellow-500 dark:bg-yellow-900/20"
				>
					<p class="text-sm font-semibold text-yellow-800 dark:text-yellow-200">
						{t('user.mcp.tokenOnceWarning')}
					</p>
					<div class="mt-2 flex flex-wrap items-center gap-2">
						<code
							class="rounded-md bg-white px-3 py-2 font-mono text-sm break-all dark:bg-gray-800"
						>
							{freshToken}
						</code>
						<SButton class="bg-secondary-500 text-white hover:bg-secondary-600" onclick={copyToken}>
							{copied ? t('user.mcp.copied') : t('user.mcp.copy')}
						</SButton>
					</div>
				</div>
			{/if}

			<div class="mt-3 flex flex-wrap items-center gap-2">
				{#if hasToken}
					<SButton
						class="bg-primary-400 text-black hover:bg-primary-500"
						disabled={busy}
						onclick={generate}
					>
						{t('user.mcp.regenerate')}
					</SButton>
					<SButton class="bg-red-500 text-white hover:bg-red-600" disabled={busy} onclick={revoke}>
						{t('user.mcp.revoke')}
					</SButton>
				{:else if hasToken === false}
					<SButton
						class="bg-secondary-500 text-white hover:bg-secondary-600"
						disabled={busy}
						onclick={generate}
					>
						{t('user.mcp.generate')}
					</SButton>
				{/if}
			</div>
			{#if hasToken}
				<p class="mt-2 text-sm text-gray-600 dark:text-gray-300">{t('user.mcp.regenerateHint')}</p>
			{/if}
			{#if error}
				<p class="mt-2 text-sm text-red-500">{t('user.mcp.error')}</p>
			{/if}
		</div>
	{/if}
</div>
