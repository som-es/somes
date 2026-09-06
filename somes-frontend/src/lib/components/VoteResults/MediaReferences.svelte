<script lang="ts" module>
	import type { ArticleLink } from '$lib/types';

	let openKey: string | null = $state(null);
	let instanceCount = 0;
</script>

<script lang="ts">
	import { Popover } from 'bits-ui';
	import { SvelteSet } from 'svelte/reactivity';
	import { t } from '$lib/i18n/i18n.svelte';
	import { formatDate } from '$lib/date';
	import derstandardIcon from '$lib/assets/external-favicons/derstandard-icon.png';
	import diepresseIcon from '$lib/assets/external-favicons/diepresse.png';
	import heuteIcon from '$lib/assets/external-favicons/heute-icon.png';
	import kurierIcon from '$lib/assets/external-favicons/kurier-icon.webp';
	import oe24Icon from '$lib/assets/external-favicons/oe24-icon.png';
	import profilIcon from '$lib/assets/external-favicons/profil-icon.png';

	interface Props {
		articleLinks: ArticleLink[];
	}

	interface ProviderInfo {
		key: string;
		label: string;
		src: string | null;
		scale: string;
	}

	interface ProviderGroup extends ProviderInfo {
		articles: ArticleLink[];
	}

	let { articleLinks }: Props = $props();

	const instanceId = instanceCount++;
	const MAX_VISIBLE = 5;

	const PROVIDERS: Record<string, { label: string; src: string; scale: string }> = {
		derstandard: { label: 'Der Standard', src: derstandardIcon, scale: 'scale-[1.15]' },
		diepresse: { label: 'Die Presse', src: diepresseIcon, scale: 'scale-[0.9]' },
		heute: { label: 'heute.at', src: heuteIcon, scale: 'scale-[1.1]' },
		kurier: { label: 'Kurier', src: kurierIcon, scale: 'scale-[0.9]' },
		oe24: { label: 'oe24.at', src: oe24Icon, scale: 'scale-[0.9]' },
		profil: { label: 'profil', src: profilIcon, scale: 'scale-[1.45]' }
	};

	let showAll = $state(false);
	let broken = $state(new SvelteSet<string>());

	function resolveProvider(provider: string): ProviderInfo {
		const key = provider.toLowerCase().replace(/[^a-z0-9]/g, '');
		const hit = Object.entries(PROVIDERS).find(([name]) => key.includes(name));
		if (hit) return { key: hit[0], label: hit[1].label, src: hit[1].src, scale: hit[1].scale };
		return { key, label: provider, src: null, scale: '' };
	}

	function hostOf(link: string): string {
		try {
			return new URL(link).hostname.replace(/^www\./, '');
		} catch {
			return link;
		}
	}

	let groups = $derived.by(() => {
		const map = new Map<string, ProviderGroup>();
		for (const link of articleLinks ?? []) {
			const info = resolveProvider(link.provider);
			const group = map.get(info.key) ?? { ...info, articles: [] };
			if (!group.articles.some((a) => a.url === link.url)) group.articles.push(link);
			map.set(info.key, group);
		}
		for (const group of map.values())
			group.articles.sort((a, b) => b.lastmod.localeCompare(a.lastmod));
		return [...map.values()].sort(
			(a, b) =>
				b.articles.length - a.articles.length ||
				a.label.localeCompare(b.label) ||
				a.key.localeCompare(b.key)
		);
	});

	let visible = $derived(showAll ? groups : groups.slice(0, MAX_VISIBLE));
	let restCount = $derived(groups.length - visible.length);

	const keyOf = (group: ProviderGroup) => `${instanceId}:${group.key}`;
</script>

{#if groups.length > 0}
	<div class="flex flex-wrap items-center gap-y-1">
		{#each visible as group (group.key)}
			<Popover.Root
				open={openKey === keyOf(group)}
				onOpenChange={(isOpen) => {
					if (isOpen) openKey = keyOf(group);
					else if (openKey === keyOf(group)) openKey = null;
				}}
			>
				<Popover.Trigger
					openOnHover
					openDelay={0}
					title={group.label}
					class="-ml-2 transition-transform first:ml-0 hover:z-10 hover:scale-110"
				>
					{#if group.src && !broken.has(group.key)}
						<span
							class="flex h-6 w-6 items-center justify-center overflow-hidden rounded-full bg-white ring-2 ring-primary-300 dark:ring-primary-500"
						>
							<img
								src={group.src}
								alt={group.label}
								onerror={() => broken.add(group.key)}
								class="h-full w-full object-contain {group.scale}"
							/>
						</span>
					{:else}
						<span
							class="flex h-6 w-6 items-center justify-center rounded-full bg-primary-600 text-[10px] font-semibold text-white uppercase ring-2 ring-primary-300 dark:ring-primary-500"
						>
							{group.label.charAt(0)}
						</span>
					{/if}
				</Popover.Trigger>
				<Popover.Content
					align="start"
					collisionPadding={8}
					class="z-50! w-72 max-w-[calc(100vw-2rem)] card bg-primary-300-700 p-3 shadow-xl"
				>
					<div class="flex items-baseline justify-between gap-2">
						<div class="font-semibold">{group.label}</div>
						<div class="shrink-0 text-xs text-gray-700 dark:text-gray-300">
							{group.articles.length}
							{group.articles.length === 1 ? t('media.article.one') : t('media.article.other')}
						</div>
					</div>
					<ul class="mt-1 flex max-h-60 flex-col gap-1 overflow-y-auto">
						{#each group.articles as article (article.url)}
							<li>
								<a
									href={article.url}
									target="_blank"
									rel="noopener noreferrer"
									class="flex items-baseline justify-between gap-2 text-sm text-gray-800 hover:underline dark:text-gray-200"
								>
									<span class="shrink-0">{formatDate(article.lastmod)}</span>
									<span class="truncate text-xs text-gray-700 dark:text-gray-300"
										>{hostOf(article.url)}</span
									>
								</a>
							</li>
						{/each}
					</ul>
					<div class="mt-2 text-xs text-gray-700 dark:text-gray-300">
						{t('media.clickForArticle')}
					</div>
				</Popover.Content>
			</Popover.Root>
		{/each}

		{#if restCount > 0}
			<button
				title={t('media.showAllArticles', { count: groups.length })}
				class="-ml-2 flex h-6 min-w-6 shrink-0 items-center justify-center rounded-full bg-primary-600 px-1 text-[10px] font-semibold text-white ring-2 ring-primary-300 transition-transform hover:z-10 hover:scale-110 dark:ring-primary-500"
				onclick={() => (showAll = true)}
			>
				+{restCount}
			</button>
		{/if}
	</div>
{/if}
