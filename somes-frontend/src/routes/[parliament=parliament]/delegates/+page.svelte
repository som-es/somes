<script lang="ts">
	import type { AutocompleteOption } from '$lib/components/Autocompletion/types';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import type {
		Delegate,
		GeneralDelegateInfo,
		GeneralGovOfficialInfo,
		LegisPeriod,
		SpeechesWithMaxPage,
		Party,
		InterjectionsWithMaxPage
	} from '$lib/types';
	import { onMount, untrack } from 'svelte';
	import {
		delegate_by_id,
		errorToNull,
		general_delegate_info,
		general_gov_official_info,
		speeches_by_delegate_per_page,
		toActualDateString,
		delegates_search_persons,
		isHasError,
		interjections_made_by_delegate_per_page,
		interjections_received_by_delegate_per_page
	} from '$lib/api/api';
	import { getLocale, t, type Locale } from '$lib/i18n/i18n.svelte';
	import {
		aiViewEnabledStore,
		currentDelegateFilterStore,
		currentDelegateStore
	} from '$lib/stores/stores';
	import Container from '$lib/components/Layout/Container.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import {
		convertDelegatesToAutocompleteOptions,
		delegateFilterOptions
	} from '$lib/components/Autocompletion/filtering';
	import { dashDateToDotDate } from '$lib/date';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import GovProposalPreview from '$lib/components/Proposals/GovProposalPreviewAtDelegate.svelte';
	import SpeechesPreview from '$lib/components/Delegates/Speeches/SpeechesPreview.svelte';
	import SquarePoliticalSpectrum from '$lib/components/Delegates/Spectrum/SquarePoliticalSpectrum.svelte';
	import AbsencesPreview from '$lib/components/Delegates/Absences/AbsencesPreview.svelte';
	import NamedVotePreview from '$lib/components/Delegates/NamedVote/NamedVotePreview.svelte';
	import TopicsChart from '$lib/components/Delegates/Interests/TopicsChart.svelte';
	import LeftRightChart from '$lib/components/Delegates/Spectrum/Stance/LeftRightChart.svelte';
	import PoliticalStanceTitleBar from '$lib/components/Delegates/Spectrum/PoliticalStanceTitleBar.svelte';
	import DecreePreview from '$lib/components/Delegates/Decrees/DecreePreview.svelte';
	import IssuedProposalPreview from '$lib/components/Delegates/IssuedProposal/IssuedProposalPreview.svelte';
	import MandatesPreview from '$lib/components/Delegates/Mandates/MandatesPreview.svelte';
	import { goto } from '$app/navigation';
	import type { PageProps } from './$types';
	import downArrowIcon from '$lib/assets/misc_icons/down-arrow.svg?raw';
	import searchIcon from '$lib/assets/misc_icons/search-glass.svg?raw';
	import { groupPartyDelegates } from '$lib/parliaments/defaultParliament';
	import { Popover } from 'bits-ui';
	import MultiSelectFilter from '$lib/components/Filtering/MultiSelectFilter.svelte';
	import { countryName } from '$lib/countries';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import { getMandateLatestPeriod, getMandatePeriods } from './searchDelegates';
	import GenericFilters from '$lib/components/Filtering/GenericFilters.svelte';
	import { type GenericFilterGroup } from '$lib/components/Filtering/types';
	import DelegateListItem from '$lib/components/Delegates/DelegateListItem.svelte';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import InterjectionsPreview from '$lib/components/Delegates/Interjections/InterjectionsPreview.svelte';
	import MobileParliamentModal from '$lib/components/Parliaments/MobileParliamentModal.svelte';
	import { defaultGp } from '$lib/api/parliament';
	import { createFilterGroup } from '$lib/components/Filtering/filterGroup.svelte';

	let { data }: PageProps = $props();

	let delegates: Delegate[] = $derived(data.delegates ?? []);
	let partiesPerGp: Record<string, Party[]> = $derived(data.partiesPerGp ?? {});

	const partyColors = $derived(data.partyColors);

	// Christoph Rework
	const sliderSteps = [25, 50, 75, 365];
	let isLegisPeriodFilterOpen = $state(false);
	let isSearchPopupOpen = $state(false);
	let searchInput = $state('');

	let selectedPartiesNames = $state<string[]>([]);
	let selectedParties = $state<Party[]>([]);
	let selectedCountries = $state<string[]>([]);

	// countryfilter for EU
	let uniqueCountries = $derived.by(() => {
		if (data.parliament !== 'eu') return [];
		const codes = new Set<string>();
		delegates.forEach((d) => {
			if (d.constituency?.trim()) codes.add(d.constituency);
		});
		return Array.from(codes)
			.map((code) => ({ code, name: countryName(code) }))
			.sort((a, b) => a.name.localeCompare(b.name, 'de'));
	});

	let selectedSearchPeriod = $derived<string[]>([data.gp || defaultGp()]);
	let timeout: any;

	let searchResults: Delegate[] = $derived(data.delegates ?? []);
	let isLoadingSearch = $state(false);

	let genericFilters: [
		GenericFilterGroup<boolean>,
		GenericFilterGroup<boolean>,
		GenericFilterGroup<boolean>
	] = $state([
		createFilterGroup<boolean>({
			title: () => t('delegates.mandateType'),
			hidden: () => false,
			options: () => [
				{ title: t('delegates.any'), value: undefined },
				{ title: t('delegates.government'), value: true },
				{ title: t('delegates.nationalCouncil'), value: false }
			]
		}),
		createFilterGroup<boolean>({
			title: () => t('delegates.activeMandate'),
			hidden: () => false,
			options: () => [
				{ title: t('delegates.any'), value: undefined },
				{ title: t('delegates.yes'), value: true },
				{ title: t('delegates.no'), value: false }
			]
		}),
		createFilterGroup<boolean>({
			title: () => t('delegates.considerPrevParty'),
			hidden: () => false,
			initialValue: true,
			options: () => [
				{ title: t('delegates.yes'), value: true },
				{ title: t('delegates.no'), value: false }
			]
		})
	]);

	// Filter Elements to Keep the PopUp open
	let searchWrapper: HTMLDivElement | undefined = $state();
	function handleFocusOut(e: FocusEvent) {
		const relatedTarget = e.relatedTarget as Node | null;
		if (relatedTarget) {
			if (searchWrapper?.contains(relatedTarget)) return;
			if ((relatedTarget as Element).closest('.search-filter-portal')) return;
		}
		isSearchPopupOpen = false;
	}

	// Search PopUp, refetch delegates if filters present
	$effect(() => {
		clearTimeout(timeout);

		const sv = searchInput;
		const searchPeriods = selectedSearchPeriod;
		const searchParties = selectedParties;
		const searchCountries = selectedCountries;
		const onlyGov = genericFilters[0].activeValue;
		const hasActiveMandate = genericFilters[1].activeValue;
		const mindPreviousPartyMembership = genericFilters[2].activeValue;

		// debounce fetch
		timeout = setTimeout(async () => {
			isLoadingSearch = true;
			if (!sv && !searchPeriods && !searchParties) {
				searchResults = data.delegates ?? [];
				isLoadingSearch = false;
				return;
			}

			const res = await delegates_search_persons(
				1,
				50,
				sv || null,
				searchPeriods,
				searchParties.map((party) => party.name),
				onlyGov,
				mindPreviousPartyMembership,
				hasActiveMandate,
				searchCountries
			);

			if (!isHasError(res)) {
				searchResults = res.delegates;
			}
			isLoadingSearch = false;
		}, 400);
	});

	// Christoph Rework end

	function selectFittingDelegate(delegates: Delegate[]): Delegate | null {
		if (delegates.length === 0) {
			return null;
		}
		let tempDelegate = null;
		const maybeStoredDelegate = currentDelegateStore.value;
		if (maybeStoredDelegate) {
			tempDelegate = maybeStoredDelegate;
			const foundDel = delegates.find((del) => del.id === maybeStoredDelegate.id);
			if (foundDel) {
				tempDelegate = foundDel;
			} else {
				tempDelegate = delegates[Math.floor(Math.random() * delegates.length)];
			}
		} else {
			tempDelegate = delegates[Math.floor(Math.random() * delegates.length)];
		}
		return tempDelegate;
	}

	let syncDelegates: Delegate[] = $derived(delegates);

	let delegate: Delegate | null = $derived.by(() => {
		if (syncDelegates.length == 0) {
			return null;
		}
		if (data.delegate !== null) {
			// @ts-ignore
			const found = syncDelegates.find((d) => d.id === data.delegate.id);
			if (found) {
				return found;
			}
		}
		const delegate = untrack(() => {
			return selectFittingDelegate($state.snapshot(syncDelegates));
		});

		return delegate;
	});

	let periods: LegisPeriod[] = $derived(data.cachedPeriods ?? []);

	let speechesPage0: SpeechesWithMaxPage | null = $state(null);
	let interjectionsMadePage0: InterjectionsWithMaxPage | null = $state(null);
	let interjectionsReceivedPage0: InterjectionsWithMaxPage | null = $state(null);
	let generalDelegateInfo: GeneralDelegateInfo | null = $state(null);
	let generalGovOfficialInfo: GeneralGovOfficialInfo | null = $state(null);
	let maxDayOffset = $state(365 * 5);

	let latestPeriod = $derived(data.cachedPeriods[data.cachedPeriods.length - 1].gp ?? defaultGp());
	let selectedPeriod = $derived(data.gp ?? latestPeriod);
	let prevSelectedPeriod = $derived(data.gp ?? latestPeriod);

	let prevSelectedDelegateId = $state(0);
	let prevLocale: Locale | null = $state(null);

	let finishedMounting = $state(false);

	let periodBounds = $derived.by(() => {
		const firstIdx = periods.findIndex((p) => p.gp == selectedPeriod);
		if (firstIdx === -1) return null;

		const periodStart = new Date(periods[firstIdx].start_date);
		const nextStart = periods[firstIdx + 1]?.start_date;
		const periodEnd = new Date(nextStart ?? new Date());
		periodEnd.setDate(periodEnd.getDate() - 1);

		const maxOffset = Math.floor(
			(periodEnd.getTime() - periodStart.getTime()) / (1000 * 60 * 60 * 24)
		);
		return {
			periodStart,
			maxOffset,
			renderStart: periods[firstIdx].start_date,
			renderEnd: nextStart
		};
	});
	function calcDayOffset(): number {
		if (!periodBounds) return maxDayOffset;
		const paramDate = data.date;
		if (!paramDate) return periodBounds.maxOffset;
		const diffMs = Math.abs(new Date(paramDate).getTime() - periodBounds.periodStart.getTime());
		return Math.floor(diffMs / (1000 * 60 * 60 * 24));
	}
	let renderStartDate: Date | null = $derived(periodBounds?.renderStart ?? null);
	let renderEndDate: Date | null = $derived(periodBounds?.renderEnd ?? null);

	let dayOffset = $derived(calcDayOffset());

	let supplyDate: Date | null = $derived.by(() => {
		if (!periodBounds) return new Date(data.date ?? new Date());
		const d = new Date(periodBounds.periodStart);
		d.setDate(d.getDate() + dayOffset);
		return d;
	});
	let inputValue = $derived(maybeCurrentDelegateFilter.search_value ?? '');

	let activeTab = $state<'analysis' | 'activities' | 'gov'>('analysis');

	// Watcher to reset tab if the delegate doesn't have gov info but tab is gov
	$effect(() => {
		if (delegate && activeTab === 'gov') {
			if (
				delegate.council !== 'gov' &&
				!(
					generalGovOfficialInfo?.gov_proposals && generalGovOfficialInfo.gov_proposals.length > 0
				) &&
				!(generalGovOfficialInfo?.decrees && generalGovOfficialInfo.decrees.length > 0)
			) {
				activeTab = 'analysis';
			}
		}
	});

	let maybeCurrentDelegateFilter = $derived(
		currentDelegateFilterStore.value ?? {
			day_offset: maxDayOffset,
			search_value: '',
			legis_period: data.gp ?? defaultGp(),
			supply_date: data.date
		}
	);

	let uniqueParties = $derived.by(() => {
		if (false) {
			return partiesPerGp[selectedPeriod].sort((a, b) => {
				return b.fraction - a.fraction;
			});
		} else {
			const parties: Party[] = [];
			const namedParties = new Set();
			const keys = Object.keys(partiesPerGp).sort().reverse();
			keys.forEach((key) => {
				partiesPerGp[key].forEach((party) => {
					if (!namedParties.has(party.code)) {
						namedParties.add(party.code);
						parties.push(party);
					}
				});
			});
			return parties.sort((a, b) => {
				return b.fraction - a.fraction;
			});
		}
	});

	let autocompleteOptions: AutocompleteOption<string>[] = $derived(
		convertDelegatesToAutocompleteOptions(delegates)
	);

	function delegateFilter(): AutocompleteOption<string>[] {
		let _options = [...autocompleteOptions];
		let _inputValue = `${String(inputValue).toLowerCase().trim()} `;
		return delegateFilterOptions(_options, _inputValue);
	}

	function onDelegateSelection(event: AutocompleteOption<string>): void {
		// @ts-ignore
		delegate = event.meta;
		inputValue = event.label;
		searchInput = 'Dere';
	}

	onMount(async () => {
		const url = new URL(window.location.href);
		const paramDelegateId = url.searchParams.get('delegate');
		updateDelsToDisplay();
		if (paramDelegateId) {
			// setting here currentDelegateStore instead of `delegate` var directly
			// this is important for a single reason: delegates without seat by default (if the backend seat history is too short)
			// wouldn't be selectable by the DataParliament component -> however, there is a reactive update happening,
			// when `delegates` is updated (therefore the client-side seat position generation was complete) and `delegate` is null
			currentDelegateStore.value = errorToNull(await delegate_by_id(+paramDelegateId));
		}

		finishedMounting = true;
	});

	const updateDelsToDisplay = async () => {
		if (!periods || periods.length == 0) {
			return;
		}

		const firstIdx = periods.findIndex((x) => x.gp == selectedPeriod);
		if (firstIdx == -1) return;
		// delegate = null;
		// const endDate = new Date(periods[firstIdx + 1].start_date);
		renderStartDate = periods[firstIdx].start_date;
		renderEndDate = periods[firstIdx + 1]?.start_date;
		const startDate = new Date(renderStartDate);

		const endDate = new Date(renderEndDate ? renderEndDate : new Date());
		if (renderEndDate) {
			const stopEarlier = data.parliament === 'eu' && selectedPeriod === '8' ? 4 : 1;
			endDate.setDate(endDate.getDate() - stopEarlier);
		}

		const diffTime = Math.abs(endDate.getTime() - startDate.getTime());
		maxDayOffset = Math.floor(diffTime / (1000 * 60 * 60 * 24));
		if (prevSelectedPeriod !== selectedPeriod) {
			dayOffset = maxDayOffset;
		}
		startDate.setDate(startDate.getDate() + dayOffset - 1);
		supplyDate = startDate;

		maybeCurrentDelegateFilter.supply_date = startDate.toISOString().split('T')[0];
		maybeCurrentDelegateFilter.day_offset = dayOffset;
		currentDelegateFilterStore.value = maybeCurrentDelegateFilter;
		const url = new URL(window.location.href);
		const previousDate = url.searchParams.get('date');
		const previousPeriod = url.searchParams.get('gp');

		startDate.setDate(startDate.getDate());
		if (previousDate === toActualDateString(supplyDate) && previousPeriod === selectedPeriod) {
			return;
		}

		url.searchParams.set('date', toActualDateString(supplyDate));
		url.searchParams.set('gp', selectedPeriod);
		goto(url.toString(), { noScroll: true, replaceState: true });
	};

	const onLettingGoOfDaySlider = () => {
		renderEndDate = null;
		renderStartDate = null;
		updateDelsToDisplay();
		if (finishedMounting) prevSelectedPeriod = selectedPeriod;
	};

	const updateStoredPeriod = () => {
		maybeCurrentDelegateFilter.legis_period = selectedPeriod;
		currentDelegateFilterStore.value = maybeCurrentDelegateFilter;
	};

	function updateDelegateIdInUrl(delegate: Delegate) {
		const url = new URL(window.location.href);
		const newId = delegate.id.toString();

		if (url.searchParams.get('delegate') === newId) return;
		url.searchParams.set('delegate', delegate.id.toString());
		goto(url.toString(), { noScroll: true, replaceState: true });
		currentDelegateStore.value = delegate;
	}

	$effect(() => {
		void delegate;
		const locale = getLocale();
		// if ($navigating) return;
		untrack(() => {
			if (delegate) {
				updateDelegateIdInUrl(delegate);
			}

			if (delegate && (prevSelectedDelegateId != delegate.id || locale !== prevLocale)) {
				const newFilter = { ...maybeCurrentDelegateFilter };
				newFilter.search_value = delegate.name;
				currentDelegateFilterStore.value = newFilter;

				generalDelegateInfo = null;
				general_delegate_info(delegate.id, locale).then((res) => {
					generalDelegateInfo = errorToNull(res);
					if (generalDelegateInfo) {
						generalDelegateInfo.interests.sort((a, b) => b.self_share - a.self_share);
						generalDelegateInfo.detailed_interests.sort((a, b) => b.self_share - a.self_share);
					}
				});

				generalGovOfficialInfo = null;
				general_gov_official_info(delegate.id).then((res) => {
					generalGovOfficialInfo = errorToNull(res);
				});

				speechesPage0 = null;
				speeches_by_delegate_per_page(delegate.id, 0).then((res) => {
					speechesPage0 = errorToNull(res);
				});
				interjectionsMadePage0 = null;
				interjections_made_by_delegate_per_page(delegate.id, 0).then((res) => {
					interjectionsMadePage0 = errorToNull(res);
				});
				interjectionsReceivedPage0 = null;
				interjections_received_by_delegate_per_page(delegate.id, 0).then((res) => {
					interjectionsReceivedPage0 = errorToNull(res);
				});

				prevSelectedDelegateId = delegate.id;
				prevLocale = locale;
			}
		});
	});

	const title = $derived(
		data.parliament == 'at' ? t('delegates.title.at') : t('delegates.title.eu')
	);
</script>

<svelte:head>
	<title>{title}</title>
	<meta name="description" content={t('delegates.meta.description')} />
</svelte:head>

<Container>
	<h1 class="px-1 pt-2 text-3xl font-bold sm:p-0 sm:text-4xl">{title}</h1>
	<span class="mb-2 ml-1 block text-base text-gray-800 sm:mt-1 sm:ml-0 dark:text-gray-300">
		{t('delegates.updated')}
	</span>

	<!------------------>
	<!-- Search PopUp -->
	<!------------------>
	<div>
		{#snippet searchContent()}
			<div>
				<!-- Filters -->
				<div>
					<span class="text-base font-semibold text-gray-800 dark:text-gray-200"
						>{t('delegates.filter')}</span
					>
					<div class="mt-2 flex h-10 w-full gap-2 md:mt-1 md:w-auto">
						<!-- Period Filter -->
						<div
							class="flex h-full grow touch-manipulation items-center justify-center gap-1 md:grow-0"
						>
							<MultiSelectFilter
								items={periods.map((p) => ({ value: p.gp, label: p.gp })).reverse()}
								bind:value={selectedSearchPeriod}
								allLabel="Alle Perioden"
							/>
						</div>
						<!-- Parteien Filter -->
						<div
							class="flex h-full grow touch-manipulation items-center justify-center gap-1 md:grow-0"
						>
							<MultiSelectFilter
								items={uniqueParties.map((p) => ({ value: p.name, label: p.name, color: p.color }))}
								bind:value={selectedPartiesNames}
								allLabel="Alle Parteien"
								onValueChange={(value) => {
									selectedParties = uniqueParties.filter((party) => value.includes(party.name));
								}}
							>
								{#snippet itemLabel(party)}
									<div
										class="h-3 w-3 shrink-0 rounded-full"
										style="background-color: {party.color};"
									></div>
									<span class="truncate">{party.label}</span>
								{/snippet}
							</MultiSelectFilter>
						</div>
						{#if uniqueCountries.length > 0}
							<div
								class="flex h-full grow touch-manipulation items-center justify-center gap-1 md:grow-0"
							>
								<MultiSelectFilter
									items={uniqueCountries.map((c) => ({ value: c.code, label: c.name }))}
									bind:value={selectedCountries}
									allLabel="Alle Länder"
								/>
							</div>
						{/if}
						<div
							class="flex h-full grow touch-manipulation items-center justify-center gap-1 md:grow-0"
						>
							<GenericFilters bind:genericFilters />
						</div>
					</div>
				</div>

				<!-- Search Results -->
				<div class="mt-3">
					<span class="text-base font-semibold text-gray-800 dark:text-gray-200"
						>{t('delegates.searchResults')}</span
					>
					<div class="mt-1 max-h-96 overflow-y-auto">
						{#if isLoadingSearch}
							<div class="flex justify-center p-4">
								<span class="text-gray-500">{t('delegates.loading')}</span>
							</div>
						{:else}
							{#each searchResults as d (d.id)}
								{@const nrMandates = getMandatePeriods($state.snapshot(d), periods, false)}
								{@const govMandates = getMandatePeriods($state.snapshot(d), periods, true)}

								<DelegateListItem
									delegate={d}
									size="md"
									class="mb-3 w-full bg-primary-300"
									onclick={() => {
										const { date, gp } = getMandateLatestPeriod(d, periods);

										const period = periods.find((p) => p.gp === gp);
										let newDayOffset = 0;
										if (period) {
											const startDate = new Date(period.start_date);
											const diffTime = Math.abs(date.getTime() - startDate.getTime());
											newDayOffset = Math.floor(diffTime / (1000 * 60 * 60 * 24));
										}

										dayOffset = newDayOffset;
										prevSelectedPeriod = gp;

										const url = new URL(window.location.href);
										url.searchParams.set('delegate', d.id.toString());
										url.searchParams.set('gp', gp);
										url.searchParams.set('date', toActualDateString(date));

										currentDelegateStore.value = d;

										const newFilter = { ...maybeCurrentDelegateFilter };
										newFilter.search_value = d.name;
										newFilter.legis_period = gp;
										newFilter.day_offset = newDayOffset;
										currentDelegateFilterStore.value = newFilter;

										goto(url.toString(), { noScroll: true });
										isSearchPopupOpen = false;
									}}
								>
									<!-- disable mandate infor on mobile -->
									<div class="hidden flex-col flex-wrap items-end gap-1 sm:flex">
										{#if govMandates !== '' && govMandates !== 'unbekannt'}
											<div class="text-sm font-medium text-gray-800 dark:text-gray-200">
												{govMandates}
												<span class="font-light text-gray-700 dark:text-gray-300">
													({t('delegates.government')})
												</span>
											</div>
										{/if}

										{#if nrMandates !== '' && nrMandates !== 'unbekannt'}
											<div class="text-sm font-medium text-gray-800 dark:text-gray-200">
												{nrMandates}

												{#if data.parliament === 'at'}
													<span class="font-light text-gray-700 dark:text-gray-300">
														({t('delegates.nationalCouncil')})
													</span>
												{/if}
											</div>
										{/if}
									</div>
								</DelegateListItem>
							{/each}
						{/if}
					</div>
				</div>
			</div>
		{/snippet}

		<div class="relative mt-7" bind:this={searchWrapper} onfocusout={handleFocusOut}>
			<!-- Search Input (desktop) -->
			<div class="hidden lg:block">
				<SearchBar
					oninput={(e) => {
						maybeCurrentDelegateFilter.search_value = e.currentTarget.value;
						searchInput = e.currentTarget.value;
						currentDelegateFilterStore.value = maybeCurrentDelegateFilter;
					}}
					onfocus={() => (isSearchPopupOpen = true)}
					bind:searchValue={inputValue}
				/>
			</div>

			<!-- Dummy search Input (mobile) -->
			<div class="flex gap-2 lg:hidden">
				<button
					class="flex h-10 min-w-0 flex-1 touch-manipulation items-center rounded-xl border-[2px] border-gray-400 text-left"
					onclick={() => {
						inputValue = '';
						searchInput = '';
						maybeCurrentDelegateFilter.search_value = '';
						currentDelegateFilterStore.value = maybeCurrentDelegateFilter;
						isSearchPopupOpen = true;
					}}
				>
					<div
						class="flex h-9 w-10 shrink-0 items-center justify-center text-gray-600 dark:text-gray-300"
					>
						{@html searchIcon}
					</div>
					<span class="truncate">
						{inputValue || t('delegates.search') + '...'}
					</span>
				</button>

				<MobileParliamentModal
					{delegates}
					bind:delegate
					bind:syncDelegates
					allSeats={data.cachedSeats}
					{selectedPeriod}
					{supplyDate}
					hasSeatInfo={data.hasSeatInfo}
				/>
			</div>

			<!-- PopUp -->
			{#if isSearchPopupOpen}
				<div
					class="absolute top-full right-0 left-0 z-100 mt-2 hidden w-[98%] rounded-xl border border-gray-300 bg-surface-50 px-5 pt-4 pb-5 shadow-lg max-md:mx-auto md:w-140 md:px-6 lg:block dark:bg-surface-600"
					data-popup="popupSearch"
					role="button"
					tabindex="0"
					onmousedown={(e) => e.preventDefault()}
				>
					{@render searchContent()}
				</div>

				<!-- Mobile -->
				<div
					class="fixed top-0 left-0 z-50 flex h-[100dvh] w-full items-start justify-center bg-black/50 p-2 backdrop-blur-sm lg:hidden"
					onfocusout={(e) => e.stopPropagation()}
				>
					<div class="w-full max-w-md rounded-2xl bg-primary-100 p-4 shadow-xl dark:bg-primary-600">
						<div class="mb-1 flex items-center justify-between">
							<h3 class="text-lg font-semibold">{t('delegates.search')}</h3>
							<ModalCloseButton class="p-1" onclick={() => (isSearchPopupOpen = false)} />
						</div>
						<div class="mb-2">
							<SearchBar
								oninput={(e) => {
									maybeCurrentDelegateFilter.search_value = e.currentTarget.value;
									searchInput = e.currentTarget.value;
									currentDelegateFilterStore.value = maybeCurrentDelegateFilter;
								}}
								bind:searchValue={inputValue}
								placeholder={t('delegates.searchDelegates')}
								autofocus={true}
							/>
						</div>

						{@render searchContent()}
					</div>
				</div>
			{/if}
		</div>
	</div>

	<!-- Normal Page proceed -->
	<div class="mt-5 flex flex-wrap gap-3">
		<!-- <div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
			<LegisButtons bind:periods bind:selectedPeriod showAllButton={false}></LegisButtons>
		</div> -->

		<div class="hidden w-full rounded-xl bg-primary-300 p-3 lg:flex dark:bg-primary-500">
			<!-- LegisPeriod Filter -->
			<div class="mx-3 flex items-center">
				<Popover.Root bind:open={isLegisPeriodFilterOpen}>
					<Popover.Trigger>
						<div
							class="flex items-center gap-1 rounded-xl bg-primary-600 p-2 px-3 text-white dark:bg-surface-500"
						>
							<h4>{selectedPeriod}</h4>
							<div
								class="block w-4 text-white transition-transform duration-200"
								class:rotate-180={isLegisPeriodFilterOpen}
							>
								{@html downArrowIcon}
							</div>
						</div>
					</Popover.Trigger>
					<!-- LegisPeriod Filter PopUp -->
					<Popover.Portal>
						<Popover.Content class="z-[1000]" sideOffset={5} align="start" side="bottom">
							<div
								class="relative top-1 w-auto max-w-[96vw] rounded-xl border border-gray-300 bg-surface-50 px-6 pt-4 pb-5 shadow-lg dark:bg-surface-600"
								data-popup="popupLegisPeriod"
							>
								<div class="mt-4 first:mt-0">
									<span class="text-base font-semibold text-gray-800 dark:text-gray-200"
										>{t('delegates.legislaturePeriod')}</span
									>
									<div class="flex w-60 flex-wrap gap-1 text-sm">
										{#each [...periods].reverse() as period}
											<button
												class="close-explicitly cursor-pointer rounded-lg border {selectedPeriod ===
												period.gp
													? 'bg-primary-300 dark:bg-primary-400'
													: ''} border-primary-300 px-2 py-1 text-sm"
												onclick={() => {
													selectedPeriod = period.gp;
													updateStoredPeriod();
													updateDelsToDisplay();
												}}
											>
												<span class="text-nowrap">{period.gp}</span>
											</button>
										{/each}
									</div>
								</div>
								<Popover.Arrow class="rounded-sm fill-current stroke-gray-300 text-gray-300" />
							</div>
						</Popover.Content>
					</Popover.Portal>
				</Popover.Root>
			</div>

			<!------------------------------------->
			<!-- Timeline Slider  (only Desktop) -->
			<!------------------------------------->
			<!-- Slider -->
			<div class="flex-1">
				<div class="mt-1 flex min-w-full justify-between px-1 text-base text-gray-800">
					<div>
						{renderStartDate == null ? '' : dashDateToDotDate(renderStartDate.toString())} ({t(
							'delegates.timeline.start'
						)})
					</div>
					<div>
						{renderEndDate == null
							? dashDateToDotDate(new Date().toISOString().split('T')[0])
							: dashDateToDotDate(renderEndDate.toString())}
						({t('delegates.timeline.end')})
					</div>
				</div>
				<input
					class="range-slider h-2 w-full cursor-pointer appearance-none rounded-lg bg-primary-200/80"
					bind:value={dayOffset}
					onchange={onLettingGoOfDaySlider}
					type="range"
					min="2"
					max={maxDayOffset + 2}
					step={1}
					list="steplist"
				/>
				<div class="flex w-full">
					{#each sliderSteps as step}
						<div
							class="relative h-2 w-[1px] bg-white"
							style="left: calc({((step - 2) / maxDayOffset) * 100}% + {10 -
								(step / maxDayOffset) * 24}px)"
						></div>
					{/each}
				</div>
				<datalist id="steplist">
					{#each sliderSteps as step}
						<option>{step}</option>
					{/each}
				</datalist>
			</div>
		</div>

		<!-------------------------------------------->
		<!-- Parliament and Delegat  (only Desktop) -->
		<!-------------------------------------------->
		{#if delegates && delegates.length > 0 && supplyDate}
			<div
				class="relative hidden min-w-full flex-wrap justify-between rounded-xl bg-primary-300 px-3 py-5 lg:flex lg:flex-nowrap dark:bg-primary-200"
			>
				<div class="mb-4 hidden w-full pl-4 max-lg:block">
					<div class="grid items-center">
						{#each [...groupPartyDelegates(structuredClone(delegates))].sort((a, b) => b[1].length - a[1].length) as [party, partyDelegates]}
							<div
								class="h-2.5 w-2.5 rounded-full"
								style="background-color: {partyColors.get(party) ?? '#ccc'};"
							></div>
							<span class="text-base font-medium text-gray-800">{party}</span>
							<span class="text-right text-base font-medium text-gray-800"
								>({partyDelegates.length})</span
							>
						{/each}
					</div>
				</div>

				<div
					class="absolute top-5 left-8 z-10 grid grid-cols-[min-content_auto_min-content] items-center gap-x-2 gap-y-0 max-lg:hidden"
				>
					{#each [...groupPartyDelegates(structuredClone(delegates))].sort((a, b) => b[1].length - a[1].length) as [party, partyDelegates]}
						<div
							class="h-2.5 w-2.5 rounded-full"
							style="background-color: {partyColors.get(party) ?? '#ccc'};"
						></div>
						<span class="text-base font-medium text-gray-800">{party}</span>
						<span class="text-right text-base font-medium text-gray-800"
							>({partyDelegates.length})</span
						>
					{/each}
				</div>

				<div class="flex w-full items-center justify-center">
					<div class={data.parliament == 'at' ? 'w-2/3' : 'w-4/5'}>
						{#if supplyDate}
							<VoteParliament2
								againstOpacity={1}
								voteResult={null}
								bind:delegate
								bind:syncDelegates
								{delegates}
								allSeats={data.cachedSeats}
								gp={selectedPeriod}
								{supplyDate}
								orderingFactor={-1}
								showGovs={true}
								overrideDelegates
								noSeats={!data.hasSeatInfo}
								useOffset={data.hasSeatInfo}
								parliament={data.parliament}
								partyColoring={partyColors}
							/>
						{/if}
					</div>
				</div>

				<div class="min-h-130 w-100">
					{#if delegate}
						<DelegateCard
							{delegate}
							questions={generalDelegateInfo?.delegate_qa ?? []}
							showQA
							partyColors={data.partyColors}
							parliament={data.parliament}
						/>
					{/if}
				</div>
			</div>
		{/if}
		<!----------------------------------------->
		<!-- Delegat Card  (only Mobile / Table) -->
		<!----------------------------------------->
		<div class="flex w-full justify-center rounded-xl bg-primary-300 p-3 lg:hidden">
			<div class="w-full sm:w-100">
				{#if delegate}
					<DelegateCard
						{delegate}
						questions={generalDelegateInfo?.delegate_qa ?? []}
						showQA
						partyColors={data.partyColors}
						parliament={data.parliament}
					/>
				{/if}
			</div>
		</div>

		{#if data.parliament == 'at'}
			<!-- Navigation Tabs -->
			<div class="mt-6 mb-2 flex w-full gap-1 rounded-xl bg-primary-300 p-1 dark:bg-surface-600">
				<button
					class="flex-1 rounded-lg px-4 py-2.5 text-sm font-medium {activeTab === 'analysis'
						? 'bg-primary-600 text-white'
						: 'text-gray-700 hover:bg-primary-400 dark:text-gray-300 dark:hover:bg-primary-500'}"
					onclick={() => (activeTab = 'analysis')}
				>
					{t('delegates.overview')}
				</button>
				<button
					class="flex-1 rounded-lg px-4 py-2.5 text-sm font-medium {activeTab === 'activities'
						? 'bg-primary-600 text-white'
						: 'text-gray-700 hover:bg-primary-400 dark:text-gray-400 dark:hover:bg-primary-500'}"
					onclick={() => (activeTab = 'activities')}
				>
					{t('delegates.activities')}
				</button>
				{#if delegate?.council === 'gov' || delegate?.mandates?.find((mandate) => {
						return mandate.is_gov_official;
					}) !== undefined}
					<button
						class="flex-1 rounded-lg px-4 py-2.5 text-sm font-medium {activeTab === 'gov'
							? 'bg-primary-600 text-white'
							: 'text-gray-700 hover:bg-primary-400 dark:text-gray-400 dark:hover:bg-primary-500'}"
						onclick={() => (activeTab = 'gov')}
					>
						{t('delegates.gov')}
					</button>
				{/if}
			</div>

			<!-- Tab Content -->
			{#if activeTab === 'gov'}
				{#if generalGovOfficialInfo?.gov_proposals && generalGovOfficialInfo.gov_proposals.length > 0 && delegate}
					<div class="title-item w-full rounded-xl bg-primary-300 p-5 dark:bg-primary-500">
						<GovProposalPreview govProposals={generalGovOfficialInfo.gov_proposals} {delegate} />
					</div>
				{:else if generalGovOfficialInfo?.gov_proposals == null && delegate && delegate.council == 'gov'}
					<ExpandablePlaceholder />
					<ExpandablePlaceholder />
				{/if}

				{#if generalGovOfficialInfo?.decrees && generalGovOfficialInfo.decrees.length > 0 && delegate}
					<div class="title-item w-full rounded-xl bg-primary-300 p-5 dark:bg-primary-500">
						<DecreePreview decrees={generalGovOfficialInfo.decrees} {delegate} />
					</div>
				{:else if (generalGovOfficialInfo?.decrees == null && delegate && delegate.council == 'gov') || !delegate}
					<ExpandablePlaceholder />
					<ExpandablePlaceholder />
				{/if}
			{:else if activeTab === 'analysis'}
				{#if delegate && generalDelegateInfo?.political_position && aiViewEnabledStore.value}
					<div class="title-item rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
						<PoliticalStanceTitleBar
							stanceTopicInfluences={generalDelegateInfo.stance_topic_influences}
							usefulInfoCount={generalDelegateInfo.political_position.total_score.count}
						/>
					</div>
				{/if}
				<div class="flex w-full flex-col gap-3 lg:flex-row">
					{#if delegate && generalDelegateInfo?.political_position && aiViewEnabledStore.value}
						<SquarePoliticalSpectrum
							{delegate}
							politicalPosition={generalDelegateInfo.political_position.total_score}
						/>
					{:else if !generalDelegateInfo}
						<ExpandablePlaceholder class={'my-3'} />
					{/if}

					{#if delegate && generalDelegateInfo?.political_position?.scores_by_topic?.length && generalDelegateInfo.political_position.scores_by_topic.length > 0 && aiViewEnabledStore.value}
						<div class="lg:flex-1">
							<LeftRightChart
								stances={generalDelegateInfo.political_position.scores_by_topic}
								interests={generalDelegateInfo.interests}
							/>
						</div>
					{:else if !generalDelegateInfo}
						<ExpandablePlaceholder class={'my-3'} />
					{/if}
				</div>

				<!-- Meist behandelte Themen & Abwesenheiten -->
				<div
					class="flex w-full flex-col gap-3 {!generalDelegateInfo ||
					generalDelegateInfo.interests?.length > 0
						? 'lg:flex-row'
						: ''}"
				>
					<!-- Meist behandelte Themen  -->
					{#if !generalDelegateInfo || generalDelegateInfo.interests?.length > 0}
						<div class="flex w-full flex-col gap-4 lg:w-2/3">
							{#if generalDelegateInfo?.interests && generalDelegateInfo?.detailed_interests}
								<span class="w-full max-sm:hidden">
									<TopicsChart
										detailedInterests={generalDelegateInfo.detailed_interests}
										interests={generalDelegateInfo.interests}
									/>
								</span>
								<span class="w-full sm:hidden">
									<TopicsChart
										detailedInterests={generalDelegateInfo.detailed_interests}
										interests={generalDelegateInfo.interests.slice(0, 8)}
									/>
								</span>
							{/if}
						</div>
					{/if}

					<!-- Abwesenheiten -->
					<div
						class="flex w-full flex-col gap-4 {!generalDelegateInfo ||
						generalDelegateInfo.interests?.length > 0
							? 'lg:w-1/3'
							: ''}"
					>
						{#if delegate && generalDelegateInfo?.absences}
							<AbsencesPreview
								{delegate}
								absences={generalDelegateInfo.absences}
								parliament={data.parliament}
							/>
						{/if}
					</div>
				</div>

				<!-- Mandateninformation -->
				{#if delegate?.mandates}
					<div class="title-item w-full rounded-xl bg-primary-300 p-5 dark:bg-primary-500">
						<MandatesPreview mandates={delegate.mandates} {periods} gender={delegate.gender} />
					</div>
				{:else if !delegate}
					<ExpandablePlaceholder />
					<ExpandablePlaceholder />
				{/if}
			{:else if activeTab === 'activities'}
				<!-- Letzte Reden -->
				{#if speechesPage0 && delegate && speechesPage0.speeches.length > 0}
					<div class="title-item w-full rounded-xl bg-primary-300 p-5 dark:bg-primary-500">
						<SpeechesPreview
							speeches={speechesPage0.speeches}
							totalCount={speechesPage0.entry_count}
							delegateId={delegate.id}
							maxPage={speechesPage0.max_page}
						/>
					</div>
				{:else if speechesPage0 == null && delegate && delegate.council == 'gov'}
					<ExpandablePlaceholder />
					<ExpandablePlaceholder />
				{/if}

				<!-- Letzte namentliche Abstimmungen -->
				{#if generalDelegateInfo?.named_votes && generalDelegateInfo?.named_votes.length > 0}
					<div class="title-item w-full rounded-xl bg-primary-300 p-5 dark:bg-primary-500">
						<NamedVotePreview namedVotes={generalDelegateInfo.named_votes} />
					</div>
				{:else if generalDelegateInfo?.absences == null || !delegate}
					<ExpandablePlaceholder />
					<ExpandablePlaceholder />
				{/if}

				<!-- Letzte eingebrachte Anträge -->
				{#if generalDelegateInfo?.issued_proposals && generalDelegateInfo.issued_proposals.length > 0}
					<div class="title-item w-full rounded-xl bg-primary-300 p-5 dark:bg-primary-500">
						<IssuedProposalPreview issuedProposals={generalDelegateInfo.issued_proposals} />
					</div>
				{:else if generalDelegateInfo?.issued_proposals == null || !delegate}
					<ExpandablePlaceholder />
					<ExpandablePlaceholder />
				{/if}

				<div class="flex w-full flex-col gap-3 lg:flex-row">
					<div
						class="flex w-full flex-col gap-4 {!generalDelegateInfo ||
						generalDelegateInfo.interests?.length > 0
							? 'lg:w-1/3'
							: ''}"
					>
						{#if delegate && generalDelegateInfo?.received_call_to_orders}
							<AbsencesPreview
								title={t('delegate.orderCalls.title')}
								explanation={t('delegate.orderCalls.explanation')}
								lastEntriesText={t('delegate.orderCalls.lastEntries')}
								noEntriesText={t('delegate.orderCalls.noEntries')}
								{delegate}
								showTotal
								showDetails={false}
								parliament={data.parliament}
								absences={generalDelegateInfo.received_call_to_orders.map((cto) => ({
									date: cto.date,
									gp: cto.gp,
									inr: cto.inr,
									plenary_session_id: cto.plenary_session_id,
									missed_legis_init_ids: [],
									source_url: null
								}))}
							/>
						{/if}
					</div>
					<div
						class="flex w-full flex-col gap-4 {!generalDelegateInfo ||
						generalDelegateInfo.interests?.length > 0
							? 'lg:w-2/3'
							: ''}"
					>
						{#if delegate && interjectionsMadePage0 && interjectionsReceivedPage0}
							<InterjectionsPreview
								issuerDelegate={delegate}
								receivedInterjectionsPage0={interjectionsReceivedPage0}
								issuedInterjectionsPage0={interjectionsMadePage0}
							/>
						{:else if interjectionsMadePage0 == null && delegate && delegate.council == 'gov'}
							<ExpandablePlaceholder />
							<ExpandablePlaceholder />
						{/if}
					</div>
					<!-- {#if generalDelegateInfo.} -->
				</div>
			{/if}
		{:else if data.parliament == 'eu'}
			<!-- Letzte Reden -->
			{#if speechesPage0 && delegate && speechesPage0.speeches.length > 0}
				<div class="title-item w-full rounded-xl bg-primary-300 p-5 dark:bg-primary-500">
					<SpeechesPreview
						speeches={speechesPage0.speeches}
						totalCount={speechesPage0.entry_count}
						delegateId={delegate.id}
						maxPage={speechesPage0.max_page}
					/>
				</div>
			{:else if speechesPage0 == null && delegate && delegate.council == 'gov'}
				<ExpandablePlaceholder />
				<ExpandablePlaceholder />
			{/if}

			<!-- Letzte namentliche Abstimmungen -->
			{#if generalDelegateInfo?.named_votes && generalDelegateInfo?.named_votes.length > 0}
				<div class="title-item w-full rounded-xl bg-primary-300 p-5 dark:bg-primary-500">
					<NamedVotePreview namedVotes={generalDelegateInfo.named_votes} />
				</div>
			{:else if generalDelegateInfo?.absences == null || !delegate}
				<ExpandablePlaceholder />
				<ExpandablePlaceholder />
			{/if}
			{#if delegate && generalDelegateInfo?.political_position && aiViewEnabledStore.value}
				<div class="title-item rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
					<PoliticalStanceTitleBar
						stanceTopicInfluences={generalDelegateInfo.stance_topic_influences}
						usefulInfoCount={generalDelegateInfo.political_position.total_score.count}
					/>
				</div>
			{/if}
			<div class="flex w-full flex-col gap-3 lg:flex-row">
				{#if delegate && generalDelegateInfo?.political_position && aiViewEnabledStore.value}
					<SquarePoliticalSpectrum
						{delegate}
						politicalPosition={generalDelegateInfo.political_position.total_score}
					/>
				{:else if !generalDelegateInfo}
					<ExpandablePlaceholder class={'my-3'} />
				{/if}

				{#if delegate && generalDelegateInfo?.political_position?.scores_by_topic?.length && generalDelegateInfo.political_position.scores_by_topic.length > 0 && aiViewEnabledStore.value}
					<div class="lg:flex-1">
						<LeftRightChart
							stances={generalDelegateInfo.political_position.scores_by_topic}
							interests={generalDelegateInfo.interests}
						/>
					</div>
				{:else if !generalDelegateInfo}
					<ExpandablePlaceholder class={'my-3'} />
				{/if}
			</div>

			<!-- Meist behandelte Themen & Abwesenheiten -->
			<div
				class="flex w-full flex-col gap-3 {!generalDelegateInfo ||
				generalDelegateInfo.interests?.length > 0
					? 'lg:flex-row'
					: ''}"
			>
				<!-- Meist behandelte Themen  -->
				{#if !generalDelegateInfo || generalDelegateInfo.interests?.length > 0}
					<div class="flex w-full flex-col gap-4 lg:w-2/3">
						{#if generalDelegateInfo?.interests && generalDelegateInfo?.detailed_interests}
							<span class="w-full max-sm:hidden">
								<TopicsChart
									detailedInterests={generalDelegateInfo.detailed_interests}
									interests={generalDelegateInfo.interests}
								/>
							</span>
							<span class="w-full sm:hidden">
								<TopicsChart
									detailedInterests={generalDelegateInfo.detailed_interests}
									interests={generalDelegateInfo.interests.slice(0, 8)}
								/>
							</span>
						{/if}
					</div>
				{/if}

				<!-- Abwesenheiten -->
				<div
					class="flex w-full flex-col gap-4 {!generalDelegateInfo ||
					generalDelegateInfo.interests?.length > 0
						? 'lg:w-1/3'
						: ''}"
				>
					{#if delegate && generalDelegateInfo?.absences}
						<AbsencesPreview
							{delegate}
							absences={generalDelegateInfo.absences}
							parliament={data.parliament}
						/>
					{/if}
				</div>
			</div>

			<!-- Mandateninformation -->
			{#if delegate?.mandates}
				<div class="title-item w-full rounded-xl bg-primary-300 p-5 dark:bg-primary-500">
					<MandatesPreview mandates={delegate.mandates} {periods} gender={delegate.gender} />
				</div>
			{:else if !delegate}
				<ExpandablePlaceholder />
				<ExpandablePlaceholder />
			{/if}
		{/if}

		<!--  -->

		<!-- <div class="flex gap-2 w-full">
		<ExpandablePlaceholder class={'my-3 w-full min-w-full'} />
		</div> -->

		<!-- {#if generalDelegateInfo}
			<ReactiveRadarChart title="hi" chartData={[
				{ label: "namentliche Abstimmungen", data: generalDelegateInfo.named_votes.length, color: "" },
				{ label: "Abwesenheiten", data: generalDelegateInfo.absences.length, color: "" },
				{ label: "Reden", data: speechesPage0?.entry_count ?? 0, color: "" },
				{ label: "Abwesenheiten", data: generalDelegateInfo.absences.length, color: "" },
				{ label: "Abwesenheiten", data: generalDelegateInfo.absences.length, color: "" },
				{ label: "Abwesenheiten", data: generalDelegateInfo.absences.length, color: "" },
			]} />
		{:else if generalDelegateInfo == null || !delegate}
			<ExpandablePlaceholder />
			<ExpandablePlaceholder />
		{/if} -->

		<!-- <div class="activity-item bg-primary-300">
                    Activity
                </div> -->
		<!-- {/if} -->
	</div>
</Container>

<!-- </div> -->

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}

	.grid-container {
		box-sizing: border-box;
		display: grid;
		min-width: 0;
		min-height: 0;
		grid-template-columns: 3fr 1fr;
		grid-template-rows: 2fr auto;
		grid-template-areas:
			'p d'
			'i .';
		/* "i i i a"; */
		padding: 10px;
	}

	.grid-container > div {
		padding: 20px 0;
	}

	@media (min-width: 768px) {
		.delegate-item {
			grid-area: d;
			flex-basis: 33%;
		}
	}

	.title-item {
		flex-basis: 100%;
	}

	:global(.interests-item) {
		grid-area: i;
		/* overflow: hidden; */
		/* min-width: 0; */
	}

	.activity-item {
		grid-area: activity;
	}

	.grid-tile-content {
		display: flex;
		justify-content: center;
		flex-direction: column;
		align-items: center;
	}

	/* Custom CSS for the slider */
	.range-slider::-webkit-slider-thumb {
		appearance: none;
		height: 20px;
		width: 20px;
		border-radius: 50%;
		background: var(--color-primary-500);
		cursor: pointer;
		border: none;
	}
	.range-slider::-moz-range-thumb {
		height: 20px;
		width: 20px;
		border-radius: 50%;
		background: var(--color-primary-500);
		cursor: pointer;
		border: none;
	}

	.range-slider::-webkit-slider-runnable-track {
		background: linear-gradient(
			to right,
			var(--color-primary-500) 0%,
			var(--color-primary-500) var(--progress),
			#e5e7eb var(--progress),
			#e5e7eb 100%
		);
	}

	.range-slider::-moz-range-track {
		background: #e5e7eb;
		height: 8px;
		border-radius: 4px;
		border: none;
	}

	.range-slider::-moz-range-progress {
		background: var(--color-primary-500);
		height: 8px;
		border-radius: 4px;
	}

	:global(.dark) .range-slider::-webkit-slider-thumb {
		background: var(--color-primary-400);
	}

	:global(.dark) .range-slider::-moz-range-thumb {
		background: var(--color-primary-400);
	}

	:global(.dark) .range-slider::-webkit-slider-runnable-track {
		background: linear-gradient(
			to right,
			var(--color-primary-400) 0%,
			var(--color-primary-400) var(--progress),
			#374151 var(--progress),
			#374151 100%
		);
	}

	:global(.dark) .range-slider::-moz-range-track {
		background: #374151;
	}

	:global(.dark) .range-slider::-moz-range-progress {
		background: var(--color-primary-400);
	}
</style>
