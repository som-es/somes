<script lang="ts">
	import type { AutocompleteOption } from '$lib/components/Autocompletion/types';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import type {
		Delegate,
		GeneralDelegateInfo,
		GeneralGovOfficialInfo,
		LegisPeriod,
		SpeechesWithMaxPage,
		Party
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
		isHasError
	} from '$lib/api/api';
	import {
		currentDelegateFilterStore,
		currentDelegateStore,
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
	import StanceTypeSwitcher from '$lib/components/Delegates/Spectrum/Stance/StanceTypeSwitcher.svelte';
	import PoliticalStanceTitleBar from '$lib/components/Delegates/Spectrum/PoliticalStanceTitleBar.svelte';
	import DecreePreview from '$lib/components/Delegates/Decrees/DecreePreview.svelte';
	import { goto, replaceState } from '$app/navigation';
	import type { PageProps } from './$types';
	import downArrowIcon from '$lib/assets/misc_icons/down-arrow.svg?raw';
	import { partyColors } from '$lib/partyColor';
	import { groupPartyDelegates } from '$lib/parliaments/defaultParliament';
	import { Popover, Select } from 'bits-ui';
	import upDownArrowIcon from '$lib/assets/misc_icons/up-down-arrow.svg?raw';
	import checkmark_small from '$lib/assets/misc_icons/checkmark_small.svg?raw';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import { getMandateLatestPeriod, getMandatePeriods } from './searchDelegates';
	import GenericFilters from '$lib/components/Filtering/GenericFilters.svelte';
	import { type GenericFilterGroup } from '$lib/components/Filtering/types';

	let { data }: PageProps = $props();

	let delegates: Delegate[] = $derived(data.delegates ?? []);
	let partiesPerGp: Record<string, Party[]> = $derived(data.partiesPerGp ?? {});

	// Christoph Rework
	const sliderSteps = [25, 50, 75, 365];
	let isLegisPeriodFilterOpen = $state(false);
	let isSearchPopupOpen = $state(false);
	let searchInput = $state('');

	let selectedPartiesNames = $state<string[]>([]);
	let selectedParties = $state<Party[]>([]);

	let selectedSearchPeriod = $state<string[]>([data.cachedPeriods?.at(data.cachedPeriods.length -1)?.gp || "XXVIII"]);
	let timeout: any;

	let searchResults: Delegate[] = $state(data.delegates ?? []);
	let isLoadingSearch = $state(false);
	
	let genericFilters: [
		GenericFilterGroup<boolean>,
		GenericFilterGroup<boolean>,
		GenericFilterGroup<boolean>,
	] = $state([
		{
			title: 'Mandatsart',
			activeValue: undefined,
			hidden: false,
			options: [
				{ title: 'egal', value: undefined },
				{ title: 'Regierung', value: true },
				{ title: 'Nationalrat', value: false }
			]
		},
		{
			title: 'Aktives Mandat',
			activeValue: undefined,
			hidden: false,
			options: [
				{ title: 'egal', value: undefined },
				{ title: 'Ja', value: true },
				{ title: 'Nein', value: false }
			]
		},
		{
			title: 'Ehemalige Parteizugehörigkeit beachten ',
			activeValue: true,
			hidden: false,
			options: [
				{ title: 'Ja', value: true },
				{ title: 'Nein', value: false }
			]
		},
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
				hasActiveMandate
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

	let syncDelegates: Delegate[] = $state([]);

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
	let generalDelegateInfo: GeneralDelegateInfo | null = $state(null);
	let generalGovOfficialInfo: GeneralGovOfficialInfo | null = $state(null);
	let maxDayOffset = $state(365 * 5);

	let renderStartDate: Date | null = $state(null);
	let renderEndDate: Date | null = $state(null);

	let finishedMounting = $state(false);
	let supplyDate: Date | null = $derived(new Date(data.date ?? new Date()));

	let prevSelectedDelegateId = $state(0);

	let maybeCurrentDelegateFilter = $derived(
		currentDelegateFilterStore.value ?? {
			day_offset: maxDayOffset,
			search_value: '',
			legis_period: data.gp ?? 'XXVIII'
		}
	);

	let inputValue = $derived(maybeCurrentDelegateFilter.search_value ?? '');
	let dayOffset = $state(maybeCurrentDelegateFilter.day_offset ?? maxDayOffset);

	let latestPeriod = $derived(data.cachedPeriods?.reverse()[0]?.gp ?? 'XXVIII');
	let selectedPeriod = $derived(maybeCurrentDelegateFilter.legis_period ?? latestPeriod);
	let prevSelectedPeriod = $state(maybeCurrentDelegateFilter.legis_period ?? latestPeriod);

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

	// run(() => {
	// 	if (inputValue) {
	// 		maybeCurrentDelegateFilter.search_value = inputValue;
	// 		currentDelegateFilterStore.value = maybeCurrentDelegateFilter;
	// 	}
	// });

	function onDelegateSelection(event: AutocompleteOption<string>): void {
		// @ts-ignore
		delegate = event.meta;
		inputValue = event.label;
		searchInput = 'Dere';
	}

	onMount(async () => {
		const url = new URL(window.location.href);
		const firstIdx = periods.findIndex((x) => x.gp == selectedPeriod);
		if (firstIdx == -1) return;
		const endDate = periods[firstIdx + 1]?.start_date;
		const newDate = new Date(endDate ? endDate : new Date());
		newDate.setDate(newDate.getDate() - 1);

		const paramDate = url.searchParams.get('date');
		if (paramDate) {
			const startDate = new Date(periods[firstIdx]?.start_date);
			const diffTime = Math.abs(new Date(paramDate).getTime() - startDate.getTime());
			dayOffset = Math.floor(diffTime / (1000 * 60 * 60 * 24));
			// this prevents that dayOffset is overwritten with max
			prevSelectedPeriod = selectedPeriod;
		}
		supplyDate = paramDate ? new Date(paramDate) : newDate;

		const paramDelegateId = url.searchParams.get('delegate');
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

		const diffTime = Math.abs(endDate.getTime() - startDate.getTime());
		maxDayOffset = Math.floor(diffTime / (1000 * 60 * 60 * 24));
		if (prevSelectedPeriod !== selectedPeriod) {
			dayOffset = maxDayOffset;
		}
		maybeCurrentDelegateFilter.day_offset = dayOffset;
		currentDelegateFilterStore.value = maybeCurrentDelegateFilter;
		startDate.setDate(startDate.getDate() + dayOffset - 1);

		supplyDate = startDate;

		const url = new URL(window.location.href);
		const previousDate = url.searchParams.get('date');
		const previousPeriod = url.searchParams.get('gp');

		startDate.setDate(startDate.getDate());
		if (previousDate === toActualDateString(supplyDate) && previousPeriod === selectedPeriod) {
			return;
		}

		url.searchParams.set('date', toActualDateString(supplyDate));
		url.searchParams.set('gp', selectedPeriod);
		goto(url.toString(), { noScroll: true });
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

	$effect(() => {
		void selectedPeriod;
		void periods;
		untrack(() => {
			renderEndDate = null;
			renderStartDate = null;

			updateStoredPeriod();
			updateDelsToDisplay();
			if (finishedMounting) prevSelectedPeriod = selectedPeriod;
		});
	});

	// let generalDelegateInfo	 = $derived.by()

	function updateDelegateIdInUrl(delegate: Delegate) {
		const url = new URL(window.location.href);
		const newId = delegate.id.toString();

		if (url.searchParams.get('delegate') === newId) return;
		url.searchParams.set('delegate', delegate.id.toString());
		replaceState(url.toString(), {});
		currentDelegateStore.value = delegate;
	}

	$effect(() => {
		void delegate;
		// if ($navigating) return;
		if (delegate) {
			updateDelegateIdInUrl(delegate);
		}

		untrack(() => {
			if (delegate && prevSelectedDelegateId != delegate.id) {
				generalDelegateInfo = null;
				general_delegate_info(delegate.id).then((res) => {
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

				prevSelectedDelegateId = delegate.id;
			}
		});
	});	

	/*run(() => {
		if (delegate && prevSelectedDelegateId != delegate.id) {
			// interests = null;

			const url = new URL(window.location.href);
			url.searchParams.set('delegate', delegate.id.toString());
			pushState(url.toString(), { replaceState: true });

		}
	});*/
</script>

<svelte:head>
	<title>Abgeordnete zum Nationalrat</title>
	<meta name="description" content="Auswahl und spezifische Informationen über Abgeordnete" />
</svelte:head>

<Container>
	<h1 class="px-1 pt-2 text-3xl font-bold sm:p-0 sm:text-4xl">Abgeordnete zum Nationalrat</h1>
	<span class="mb-2 ml-1 block text-base text-gray-800 sm:mt-1 sm:ml-0 dark:text-gray-300">
		Aktualisiert am: Unknown
	</span>

	<!-- Search PopUp -->
	<div class="relative mt-7" bind:this={searchWrapper} onfocusout={handleFocusOut}>
		<!-- Search Input -->
		<SearchBar
			oninput={(e) => {
				maybeCurrentDelegateFilter.search_value = e.currentTarget.value;
				searchInput = e.currentTarget.value;
				currentDelegateFilterStore.value = maybeCurrentDelegateFilter;
			}}
			onfocus={() => (isSearchPopupOpen = true)}
			bind:searchValue={inputValue}
		/>

		<!-- PopUp -->
		{#if isSearchPopupOpen}
			<div
				class="absolute top-full right-0 left-0 z-100 mt-2 w-[98%] rounded-xl border border-gray-300 bg-surface-50 dark:bg-surface-600 px-5 pt-4 pb-5 shadow-lg max-md:mx-auto md:w-140 md:px-6"
				data-popup="popupSearch"
				role="button"
				tabindex="0"
				onmousedown={(e) => e.preventDefault()}
			>
				<div>
					<!-- Filters -->
					<div class="mr-4">
						<span class="text-base font-semibold text-gray-800 dark:text-gray-200">Filter</span>
						<div class="flex gap-3">
							<!-- Period Filter -->
							<div class="mt-1 flex flex-col gap-2">
								<Select.Root
									type="multiple"
									bind:value={selectedSearchPeriod}
									items={periods.map((p) => ({ value: p.gp, label: p.gp })).reverse()}
									allowDeselect={true}
								>
									<Select.Trigger
										class="inline-flex h-10 items-center justify-between rounded-xl bg-secondary-500 px-[11px] text-white transition-colors placeholder:text-gray-600 focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:outline-none md:w-[200px]"
									>
										<div class="flex items-center gap-2">
											{#each selectedSearchPeriod.slice(0, 2) as period}
												<span class="truncate">{period}</span>
											{/each}
											{#if selectedSearchPeriod.length > 2}
												<span class="truncate">+{selectedSearchPeriod.length - 2} weitere</span>
											{/if}
											{#if selectedSearchPeriod.length === 0}
												<span class="truncate">Alle Perioden</span>
											{/if}
										</div>
										{@html upDownArrowIcon}
									</Select.Trigger>
									<Select.Portal>
										<Select.Content
											class="z-500 max-h-60 w-[200px] min-w-[var(--bits-select-anchor-width)] overflow-hidden rounded-xl border border-gray-200 bg-surface-100 shadow-lg dark:bg-surface-500"
											sideOffset={8}
										>
											<Select.Viewport class="p-1">
												{#each [...periods].reverse() as period}
													<Select.Item
														class="flex h-10 w-full cursor-pointer items-center rounded-lg py-3 pr-1.5 pl-3 text-sm capitalize transition-all duration-75 outline-none select-none data-highlighted:bg-gray-100 dark:data-highlighted:bg-gray-400"
														value={period.gp}
														label={period.gp}
													>
														{#snippet children({ selected })}
															<div class="flex items-center gap-2">
																{period.gp}
															</div>
															{#if selected}
																<div class="ml-auto h-4 stroke-black dark:stroke-white">
																	{@html checkmark_small}
																</div>
															{/if}
														{/snippet}
													</Select.Item>
												{/each}
											</Select.Viewport>
										</Select.Content>
									</Select.Portal>
								</Select.Root>
							</div>
							<!-- Parteien Filter -->
							<div class="mt-1 flex flex-col gap-2">
								<Select.Root
									type="multiple"
									bind:value={selectedPartiesNames}
									onValueChange={(v) => {
										selectedParties = uniqueParties.filter((party) => v.includes(party.name));
									}}
									items={uniqueParties.map((p) => ({ value: p.name, label: p.name }))}
								>
									<Select.Trigger
										class="inline-flex h-10 items-center justify-between rounded-xl bg-secondary-500 px-[11px] text-white transition-colors placeholder:text-gray-600 focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:outline-none md:w-[200px]"
									>
										<div class="flex items-center gap-2">
											{#each selectedParties.slice(0, 1) as party}
												<div
													class="h-3 w-3 rounded-full"
													style="background-color: {party.color};"
												></div>
												<span class="truncate">{party.name}</span>
											{/each}
											{#if selectedParties.length > 1}
												<span class="truncate">+{selectedParties.length - 1} weitere</span>
											{/if}
											{#if selectedParties.length === 0}
												<span class="truncate">Alle Parteien</span>
											{/if}
										</div>
										{@html upDownArrowIcon}
									</Select.Trigger>
									<Select.Portal>
										<Select.Content
											class="z-500 max-h-60 w-[200px] min-w-[var(--bits-select-anchor-width)] overflow-hidden rounded-xl border border-gray-200 bg-surface-100 shadow-lg dark:bg-surface-500"
											sideOffset={8}
										>
											<Select.Viewport class="p-1">
												{#each uniqueParties as party}
													<Select.Item
														class="flex h-10 w-full cursor-pointer items-center rounded-lg py-3 pr-1.5 pl-3 text-sm capitalize transition-all duration-75 outline-none select-none data-highlighted:bg-gray-100 dark:data-highlighted:bg-gray-400"
														value={party.name}
														label={party.name}
													>
														{#snippet children({ selected })}
															<div class="flex items-center gap-2">
																<div
																	class="h-3 w-3 rounded-full"
																	style="background-color: {party.color};"
																></div>
																{party.name}
															</div>
															{#if selected}
																<div class="ml-auto h-4 stroke-black dark:stroke-white">
																	{@html checkmark_small}
																</div>
															{/if}
														{/snippet}
													</Select.Item>
												{/each}
											</Select.Viewport>
										</Select.Content>
									</Select.Portal>
								</Select.Root>
							</div>
							<div>
								<GenericFilters bind:genericFilters />
							</div>
						</div>
					</div>

					<!-- Search Results -->
					<div class="mt-3">
						<span class="text-base font-semibold text-gray-800 dark:text-gray-200">Suchergebnisse</span>
						<div class="mt-1 max-h-96 overflow-y-auto">
							{#if isLoadingSearch}
								<div class="flex justify-center p-4">
									<span class="text-gray-500">Loading...</span>
								</div>
							{:else}
								{#each searchResults as d}
									{@const nrMandates = getMandatePeriods(d, periods, false)}
									{@const govMandates = getMandatePeriods(d, periods, true)}

									<button
										class="mb-3 flex w-full justify-between rounded-xl bg-primary-300 dark:bg-primary-500 p-2 transition-colors hover:cursor-pointer hover:bg-primary-400"
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
										<div class="flex">
											<img class="mx-1 h-14 rounded-full" src={d.image_url} alt={d.name} />
											<div class="mt-1 ml-3">
												<h4 class="text-lg/5 font-semibold text-gray-900 dark:text-gray-50">{d.name}</h4>
												<div class="mt-1 flex items-center gap-2">
													<div
														class="h-2 w-2 rounded-full"
														style="background-color: {partyColors.get(d.party) ?? '#ccc'};"
													></div>
													<span class="text-sm font-medium text-gray-800 dark:text-gray-200">{d.party}</span>
												</div>
											</div>
										</div>
										<div class="flex flex-col flex-wrap">
											{#if govMandates !== '' && govMandates !== 'unbekannt'}
												<div class="text-sm font-medium text-gray-800 dark:text-gray-200!">
													{govMandates}
													<span class="font-light text-gray-700 dark:text-gray-300!"> (Regierung) </span>
												</div>
											{/if}

											{#if nrMandates !== '' && nrMandates !== 'unbekannt'}
												<div class="text-sm font-medium text-gray-800 dark:text-gray-200">
													{nrMandates}
													<span class="font-light text-gray-700 dark:text-gray-300"> (Nationalrat) </span>
												</div>
											{/if}
										</div>
									</button>
								{/each}
							{/if}
						</div>
					</div>
				</div>
			</div>
		{/if}
	</div>

	<!-- Normal Page proceed -->
	<div class="mt-5 flex flex-wrap gap-3">
		<!-- <div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
			<LegisButtons bind:periods bind:selectedPeriod showAllButton={false}></LegisButtons>
		</div> -->

		<!--------------------->
		<!-- Timeline Slider -->
		<!--------------------->
		<div class="flex w-full rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
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
						<Popover.Content class="z-[1000]">
							<div
								class="relative top-1 w-auto max-w-[96vw] rounded-xl border border-gray-300 bg-surface-50 dark:bg-surface-600 px-6 pt-4 pb-5 shadow-lg"
								data-popup="popupLegisPeriod"
							>
								<div class="mt-4 first:mt-0">
									<span class="text-base font-semibold text-gray-800 dark:text-gray-200">Legislaturperiode</span>
									<div class="flex w-60 flex-wrap gap-1 text-sm">
										{#each [...periods].reverse() as period}
											<button
												class="close-explicitly cursor-pointer rounded-lg border border-primary-300 px-2 py-1 text-sm"
												class:bg-primary-300={selectedPeriod === period.gp}
												onclick={() => {
													selectedPeriod = period.gp;
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
			<!-- Slider -->
			<div class="flex-1">
				<div class="mt-1 flex min-w-full justify-between px-1 text-base text-gray-800">
					<div>
						Anfang - {renderStartDate == null ? '' : dashDateToDotDate(renderStartDate.toString())}
					</div>
					<div>
						Ende -
						{renderEndDate == null
							? dashDateToDotDate(new Date().toISOString().split('T')[0])
							: dashDateToDotDate(renderEndDate.toString())}
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
		<!-- 
		<div class="text-token w-full space-y-2">
			<input
				class="input w-full h-12 px-2"
				type="search"
				name="ac-demo"
				bind:value={inputValue}
				placeholder="Suchen..."
				use:popup={popupSettings}
			/>

			{#if autocompleteOptions}
				<div
					class="z-10 card w-full max-w-sm max-h-64 p-4 overflow-y-auto"
					data-popup="popupAutocomplete"
				>
					<Autocomplete
						bind:input={inputValue}
						options={autocompleteOptions}
						on:selection={onDelegateSelection}
						emptyState={'Keine Person gefunden'}
						filter={delegateFilter}
					/>
				</div>
			{/if}
		</div> -->

		{#if delegates && delegates.length > 0 && supplyDate}
			<div
				class="relative flex min-w-full flex-wrap justify-between rounded-xl bg-primary-300 px-5 pt-3 pb-3 lg:flex-nowrap dark:bg-primary-200"
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
					<div class="w-2/3">
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
							/>
						{/if}
					</div>
				</div>

				<div class="w-100">
					{#if delegate}
						<DelegateCard {delegate} questions={generalDelegateInfo?.delegate_qa ?? []} showQA />
					{/if}
				</div>
			</div>
		{/if}

		{#if generalGovOfficialInfo?.gov_proposals && generalGovOfficialInfo.gov_proposals.length > 0 && delegate}
			<div class="title-item w-full rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
				<GovProposalPreview govProposals={generalGovOfficialInfo.gov_proposals} {delegate} />
			</div>
		{:else if generalGovOfficialInfo?.gov_proposals == null && delegate && delegate.council == 'gov'}
			<ExpandablePlaceholder />
			<ExpandablePlaceholder />
		{/if}

		{#if generalGovOfficialInfo?.decrees && generalGovOfficialInfo.decrees.length > 0 && delegate}
			<div class="title-item w-full rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
				<DecreePreview decrees={generalGovOfficialInfo.decrees} {delegate} />
			</div>
		{:else if (generalGovOfficialInfo?.decrees == null && delegate && delegate.council == 'gov') || !delegate}
			<ExpandablePlaceholder />
			<ExpandablePlaceholder />
		{/if}

		{#if delegate && generalDelegateInfo?.political_position}
			<div class="title-item rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
				<PoliticalStanceTitleBar
					stanceTopicInfluences={generalDelegateInfo.stance_topic_influences}
				/>
			</div>
		{/if}
		<div class="flex gap-2 max-lg:flex-wrap">
			{#if delegate && generalDelegateInfo?.political_position}
				<SquarePoliticalSpectrum
					{delegate}
					politicalPosition={generalDelegateInfo.political_position}
				/>
			{:else if !generalDelegateInfo}
				<ExpandablePlaceholder class={'my-3'} />
			{/if}

			{#if delegate && generalDelegateInfo?.left_right_stances.length && generalDelegateInfo.left_right_stances.length > 0}
				<StanceTypeSwitcher delegateInfo={generalDelegateInfo} />
			{:else if !generalDelegateInfo}
				<ExpandablePlaceholder class={'my-3'} />
			{/if}
		</div>

		{#if generalDelegateInfo?.interests && generalDelegateInfo?.detailed_interests}
			{#if generalDelegateInfo?.interests?.length > 0}
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
				<!-- <div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3 w-full">

					<h1 class="font-bold text-2xl mb-2">Top 4 Interessen</h1>
					<InterestTiles bgColor={"bg-primary-300 dark:bg-primary-500"} squareColor={"dark:bg-primary-300 bg-primary-400"} interests={generalDelegateInfo.interests.slice(0, 4)} />
				</div> -->
			{/if}
		{:else}
			<ExpandablePlaceholder class={'my-3'} />
		{/if}

		{#if speechesPage0 && delegate && speechesPage0.speeches.length > 0}
			<div class="title-item w-full rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
				<SpeechesPreview delegateId={delegate.id} {speechesPage0} />
			</div>
		{:else if speechesPage0 == null && delegate && delegate.council == 'gov'}
			<ExpandablePlaceholder />
			<ExpandablePlaceholder />
		{/if}

		{#if generalDelegateInfo?.absences && delegate && generalDelegateInfo?.absences.length > 0}
			<div class="title-item w-full rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
				<AbsencesPreview delegateId={delegate.id} absences={generalDelegateInfo.absences} />
			</div>
		{:else if generalDelegateInfo?.absences == null || !delegate}
			<ExpandablePlaceholder />
			<ExpandablePlaceholder />
		{/if}

		{#if generalDelegateInfo?.named_votes && delegate && generalDelegateInfo?.named_votes.length > 0}
			<div class="title-item w-full rounded-xl bg-primary-300 p-3 dark:bg-primary-500">
				<NamedVotePreview delegateId={delegate.id} namedVotes={generalDelegateInfo.named_votes} />
			</div>
		{:else if generalDelegateInfo?.absences == null || !delegate}
			<ExpandablePlaceholder />
			<ExpandablePlaceholder />
		{/if}

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
