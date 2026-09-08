<script lang="ts">
	import { isHasError } from '$lib/api/api';
	import { addMoodValue, getOwnMoodValue } from '$lib/api/authed';
	import { jwtStore, loginDrawerOpenStore } from '$lib/caching/stores/stores.svelte';
	import { mood_by_path } from '$lib/components/Proposals/api';
	import { t } from '$lib/i18n/i18n.svelte';
	import type { MoodBarometer as MoodBarometerData } from '$lib/types';

	interface Props {
		gp: string;
		inr: string | number;
	}

	let { gp, inr }: Props = $props();

	let barometer: MoodBarometerData | null = $state(null);
	let ownMood: number | null = $state(null);
	let userMood = $state(0);
	let submitting = $state(false);
	let loginRequired = $state(false);
	let errorMsg: string | null = $state(null);

	$effect(() => {
		errorMsg = null;
		mood_by_path(gp, inr).then((result) => {
			barometer = isHasError(result) ? null : result;
		});
	});

	$effect(() => {
		if (jwtStore.value == null) {
			ownMood = null;
			return;
		}
		getOwnMoodValue(gp, inr).then((result) => {
			ownMood = isHasError(result) ? null : result;
		});
	});

	const communityMood: number | null = $derived.by(() => {
		if (barometer == null) return null;
		if (barometer.pre_aggregated_user_mood !== null) return barometer.pre_aggregated_user_mood;
		if (barometer.user_moods.length > 0) {
			return barometer.user_moods.reduce((a, b) => a + b, 0) / barometer.user_moods.length;
		}
		return null;
	});

	const voteCount = $derived.by(() => barometer?.user_moods.length ?? 0);

	const showResults = $derived.by(() => jwtStore.value != null && ownMood != null && voteCount > 0);

	const buckets = [
		{ label: 'mood.veryNegative', color: '#991b1b' },
		{ label: 'mood.negative', color: '#d97862' },
		{ label: 'mood.neutral', color: '#414b5a' },
		{ label: 'mood.positive', color: '#57a874' },
		{ label: 'mood.veryPositive', color: '#14532d' }
	] as const;

	function bucketIndex(mood: number): number {
		return Math.min(buckets.length - 1, Math.floor(((mood + 1) / 2) * buckets.length));
	}

	const bucketCounts: number[] = $derived.by(() => {
		const counts = buckets.map(() => 0);
		for (const mood of barometer?.user_moods ?? []) {
			counts[bucketIndex(mood)]++;
		}
		return counts;
	});

	const maxBucketCount = $derived.by(() => Math.max(...bucketCounts, 1));

	function toPercent(mood: number): number {
		return ((mood + 1) / 2) * 100;
	}

	function moodLabel(mood: number): string {
		return t(buckets[bucketIndex(mood)].label);
	}

	async function submit() {
		if (jwtStore.value == null) {
			loginRequired = true;
			return;
		}
		submitting = true;
		errorMsg = null;
		const result = await addMoodValue(gp, inr, userMood);
		submitting = false;
		if (isHasError(result)) {
			if (result.error_type === 'AuthError') {
				loginRequired = true;
				loginDrawerOpenStore.value = true;
			} else {
				errorMsg = t('mood.error');
			}
		} else {
			barometer = result;
			ownMood = userMood;
		}
	}
</script>

<div class="rounded-xl bg-primary-300 px-6 py-5 dark:bg-primary-500">
	<span class="text-base font-bold md:text-lg">{t('mood.heading')}</span>

	{#if showResults}
		<p class="mt-0.5 text-sm text-gray-600 dark:text-gray-300">n={voteCount}</p>
		<!-- Distribution of opinions -->
		<div class="mt-4">
			<div class="flex h-28 items-end gap-[2px]">
				{#each buckets as bucket, i}
					<div
						class="flex h-full flex-1 flex-col items-center justify-end"
						title="{t(bucket.label)}: {bucketCounts[i]}"
					>
						<span class="mb-0.5 text-xs text-gray-700 dark:text-gray-200">
							{bucketCounts[i]}
						</span>
						<div
							class="w-full max-w-14 rounded-t-[4px]"
							style="background-color: {bucket.color}; height: {bucketCounts[i] === 0
								? '2px'
								: `${Math.max(4, (bucketCounts[i] / maxBucketCount) * 80)}%`}"
						></div>
					</div>
				{/each}
			</div>
			<!-- X axis with the mood as a dot -->
			<div class="relative border-t border-gray-500/50">
				{#if communityMood != null}
					<div
						class="mood-marker bg-secondary-500"
						style="left: {toPercent(communityMood)}%"
						title={t('mood.communityMood')}
					></div>
				{/if}
			</div>
			<div class="flex gap-[2px] pt-1.5">
				{#each buckets as bucket}
					<span
						class="flex-1 text-center text-[0.65rem] leading-tight text-gray-600 dark:text-gray-300"
					>
						{t(bucket.label)}
					</span>
				{/each}
			</div>
			{#if communityMood != null}
				<div class="mt-3 flex items-center gap-2 text-sm">
					<span class="inline-block h-2 w-2 rounded-full bg-secondary-500"></span>
					<span>{t('mood.communityMood')}: <b>{moodLabel(communityMood)}</b></span>
				</div>
			{/if}
		</div>
	{:else if loginRequired && jwtStore.value == null}
		<!-- Shown in place of the chart after sharing without being logged in -->
		<p class="mt-0.5 text-sm text-gray-600 dark:text-gray-300">{t('mood.loginForChart')}</p>
		<button
			onclick={() => (loginDrawerOpenStore.value = true)}
			class="mt-4 rounded-lg bg-primary-500 px-4 py-2 text-sm font-medium text-white dark:bg-primary-300 dark:text-gray-900"
		>
			{t('login.title.signIn')}
		</button>
	{:else}
		<p class="mt-0.5 text-sm text-gray-600 dark:text-gray-300">{t('mood.unlockHint')}</p>

		<!-- Own vote -->
		<div class="mt-5 flex flex-wrap items-center gap-x-4 gap-y-3">
			<span class="shrink-0 text-sm">{t('mood.stronglyAgainst')}</span>
			<input
				type="range"
				min="-1"
				max="1"
				step="0.05"
				bind:value={userMood}
				aria-label={t('mood.heading')}
				class="mood-slider min-w-40 flex-1 touch-manipulation"
			/>
			<span class="shrink-0 text-sm">{t('mood.stronglyFor')}</span>
			<button
				onclick={submit}
				disabled={submitting}
				class="shrink-0 rounded-lg bg-primary-500 px-4 py-2 text-sm font-medium text-white disabled:cursor-not-allowed disabled:opacity-40 dark:bg-primary-300 dark:text-gray-900"
			>
				{t('mood.submit')}
			</button>
		</div>
		{#if errorMsg}
			<p class="mt-2 text-xs text-red-800 dark:text-red-300">{errorMsg}</p>
		{/if}
	{/if}
</div>

<style>
	.mood-marker {
		position: absolute;
		top: 0;
		width: 0.9rem;
		height: 0.9rem;
		border-radius: 9999px;
		border: 2px solid white;
		transform: translate(-50%, -50%);
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
	}

	.mood-slider {
		appearance: none;
		height: 1.75rem;
		background: transparent;
		cursor: pointer;
	}

	.mood-slider::-webkit-slider-runnable-track {
		height: 2px;
		border-radius: 9999px;
		background: light-dark(rgb(107 114 128 / 0.6), rgb(229 231 235 / 0.6));
	}

	.mood-slider::-webkit-slider-thumb {
		appearance: none;
		margin-top: -7px;
		height: 16px;
		width: 16px;
		border-radius: 9999px;
		background: light-dark(var(--color-primary-500), var(--color-primary-300));
	}

	.mood-slider::-moz-range-track {
		height: 2px;
		border-radius: 9999px;
		background: light-dark(rgb(107 114 128 / 0.6), rgb(229 231 235 / 0.6));
	}

	.mood-slider::-moz-range-thumb {
		border: none;
		height: 16px;
		width: 16px;
		border-radius: 9999px;
		background: light-dark(var(--color-primary-500), var(--color-primary-300));
	}
</style>
