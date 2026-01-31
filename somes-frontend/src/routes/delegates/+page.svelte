<script lang="ts">
	import { run } from 'svelte/legacy';

	import type { AutocompleteOption } from '$lib/components/Autocompletion/types';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import Autocomplete from '$lib/components/Autocompletion/Autocomplete.svelte';
	import type {
		Delegate,
		GeneralDelegateInfo,
		GeneralGovOfficialInfo,
		LegisPeriod,
		SpeechesWithMaxPage,
		HasError,
		Party
	} from '$lib/types';
	import { onMount, untrack } from 'svelte';
	import {
		delegate_by_id,
		errorToNull,
		general_delegate_info,
		general_gov_official_info,
		speeches_by_delegate_per_page,
		toActualDateString
	} from '$lib/api/api';
	import {
		currentDelegateFilterStore,
		currentDelegateStore,
		hasGoBackStore
	} from '$lib/stores/stores';
	import SButton from '$lib/components/UI/SButton.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import {
		convertDelegatesToAutocompleteOptions,
		delegateFilterOptions
	} from '$lib/components/Autocompletion/filtering';
	import LegisButtons from '$lib/components/Filtering/LegisButtons.svelte';
	import AllBadges from '$lib/components/VoteResults/SimpleYesNo/AllBadges.svelte';
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
	import { goto, pushState, replaceState } from '$app/navigation';
	import AutocompleteWithPopover from '$lib/components/Autocompletion/AutocompleteWithPopover.svelte';
	import type { PageProps } from './$types';
	import downArrowIcon from '$lib/assets/misc_icons/down-arrow.svg?raw';
	import { partyColors } from '$lib/partyColor';
	import { groupPartyDelegates } from '$lib/parliaments/defaultParliament';
	import searchIcon from '$lib/assets/misc_icons/search-glass.svg?raw';
	import { Popover, Select } from 'bits-ui';
	import upDownArrowIcon from '$lib/assets/misc_icons/up-down-arrow.svg?raw';
	import checkmark_small from '$lib/assets/misc_icons/checkmark_small.svg?raw';

	let { data }: PageProps = $props();

	let delegates: Delegate[] = $derived(data.delegates ?? []);
	let partiesPerGp: Record<string, Party[]> = $derived(data.partiesPerGp ?? {});

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

	// Christoph Rework
	const sliderSteps = [25, 50, 75, 365];
	let isLegisPeriodFilterOpen = $state(false);
	let isSearchPopupOpen = $state(false);
	let searchInput = $state('');
	let selectedParty = $state<Party | undefined>(undefined);
	let selectedSearchPeriod = $state<string | undefined>(undefined);


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

	// Caches the sorted delegates
	let sortedDelegates = $derived(sortDelegates(delegates));

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
		// console.log(supplyDate);

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

	function getMandatePeriods(delegate: Delegate, periods: LegisPeriod[]): string {
		if (!delegate.mandates || !delegate.active_mandates || delegate.mandates.length === 0) {
			return 'unbekannt';
		}

		// Sort mandates by start date
		const sortedMandates = [...delegate.mandates, ...delegate.active_mandates].sort(
			(a, b) => new Date(a.start_date || '').getTime() - new Date(b.start_date || '').getTime()
		);

		// Find the period for a given date
		const findPeriod = (date: string): string | null => {
			const d = new Date(date);
			for (let i = 0; i < periods.length; i++) {
				const periodStart = new Date(periods[i].start_date);
				const periodEnd = periods[i + 1] ? new Date(periods[i + 1].start_date) : new Date();

				if (d >= periodStart && d < periodEnd) {
					return periods[i].gp;
				}
			}
			return null;
		};

		// Get first and last period
		const firstPeriod = findPeriod(sortedMandates[0].start_date || '');
		const lastMandate = sortedMandates.reduce((latest, current) => {
			if (!current.end_date) return current; // Active mandate always wins
			if (!latest.end_date) return latest;
			return new Date(current.end_date) > new Date(latest.end_date) ? current : latest;
		});
		const lastPeriod = lastMandate.end_date ? findPeriod(lastMandate.end_date) : 'dato';

		if (!firstPeriod) return `unbekannt - ${lastPeriod}`;

		return `${firstPeriod} - ${lastPeriod}`;
	}

	function sortDelegates(delegates: Delegate[]): Delegate[] {
		const filter = maybeCurrentDelegateFilter;

		// Filter for Party
		let filteredDelegates = delegates;
		if (selectedParty !== undefined) {
			filteredDelegates = filteredDelegates.filter((d) => d.party === selectedParty?.name);
		}

		// Filter by Legislative Period
		if (selectedSearchPeriod !== undefined) {
			const sortedPeriods = [...periods].sort(
				(a, b) => new Date(a.start_date).getTime() - new Date(b.start_date).getTime()
			);
			const periodIndex = sortedPeriods.findIndex((p) => p.gp === selectedSearchPeriod);

			if (periodIndex !== -1) {
				const periodStart = new Date(sortedPeriods[periodIndex].start_date);

				const nextPeriod = sortedPeriods[periodIndex + 1];
				const periodEnd = nextPeriod ? new Date(nextPeriod.start_date) : new Date();

				filteredDelegates = filteredDelegates.filter((delegate) => {
					const allMandates = [
						...(delegate.mandates ?? []),
						...(delegate.active_mandates ?? [])
					];

					return allMandates.some((mandate) => {
						const mandateStart = new Date(mandate.start_date ?? '1900-01-01');
						const mandateEnd = mandate.end_date ? new Date(mandate.end_date) : new Date();

						return mandateStart < periodEnd && mandateEnd > periodStart;
					});
				});
			}
		}

		// If there's a search value, prioritize matching delegates
		if (filter.search_value && filter.search_value.trim()) {
			const searchLower = filter.search_value.toLowerCase().trim();

			return [...filteredDelegates].sort((a, b) => {
				const aName = a.name.toLowerCase();
				const bName = b.name.toLowerCase();

				// Exact matches first
				const aExactMatch = aName === searchLower;
				const bExactMatch = bName === searchLower;
				if (aExactMatch && !bExactMatch) return -1;
				if (!aExactMatch && bExactMatch) return 1;

				// Starts with search term
				const aStartsWith = aName.startsWith(searchLower);
				const bStartsWith = bName.startsWith(searchLower);
				if (aStartsWith && !bStartsWith) return -1;
				if (!aStartsWith && bStartsWith) return 1;

				// Contains search term
				const aContains = aName.includes(searchLower);
				const bContains = bName.includes(searchLower);
				if (aContains && !bContains) return -1;
				if (!aContains && bContains) return 1;

				// Alphabetical as fallback
				return aName.localeCompare(bName);
			});
		}

		// Default: sort alphabetically by name
		return [...filteredDelegates].sort((a, b) => a.name.localeCompare(b.name));
	}

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
	<h1 class="px-1 pt-2 text-3xl font-bold sm:p-0 sm:text-4xl">Abgeordnete des Nationalrats</h1>
	<span class="mb-2 ml-1 block text-base text-gray-800 sm:mt-1 sm:ml-0">
		Aktualisiert am: Unknown
	</span>

	<!-- Search PopUp -->
	<div class="relative" bind:this={searchWrapper} onfocusout={handleFocusOut}>
		<!-- Search Input -->
		<div class="mt-12 flex h-10 flex-grow rounded-xl border-[2px] border-gray-400">
			<div class="flex h-9 w-10 items-center justify-center text-gray-600">
				{@html searchIcon}
			</div>
			<input
				type="search"
				class="block w-full bg-transparent py-2 placeholder:text-gray-600 focus:outline-none"
				placeholder="Suche..."
				bind:value={inputValue}
				oninput={(e) => {
					maybeCurrentDelegateFilter.search_value = e.currentTarget.value;
					currentDelegateFilterStore.value = maybeCurrentDelegateFilter;
				}}
				onfocus={() => (isSearchPopupOpen = true)}
			/>
		</div>

		<!-- PopUp -->
		{#if isSearchPopupOpen}
			<div
				class="absolute top-full right-0 left-0 z-100 mt-2 w-150 rounded-xl border border-gray-300 bg-surface-50 px-5 pt-4 pb-5 shadow-lg md:px-6"
				data-popup="popupSearch"
				role="button"
				tabindex="0"
				onmousedown={(e) => e.preventDefault()}
			>
				<div>
					<!-- Filters -->
					<div class="mr-4">
						<span class="text-base font-semibold text-gray-800">Filter</span>
						<div class="flex gap-3">
						<!-- Period Filter -->
						<div class="flex flex-col gap-2 mt-1">
							<Select.Root
								type="single"
								value={selectedSearchPeriod}
								onValueChange={(v) => (selectedSearchPeriod = v)}
								items={periods.map((p) => ({ value: p.gp, label: p.gp })).reverse()}
								allowDeselect={true}
							>
								<Select.Trigger
									class="inline-flex h-10 w-[200px] items-center justify-between rounded-xl bg-secondary-500 px-[11px] text-white transition-colors placeholder:text-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2"
								>
									<div class="flex items-center gap-2">
										<span class="truncate">{selectedSearchPeriod || 'Alle Perioden'}</span>
									</div>
									{@html upDownArrowIcon}
								</Select.Trigger>
								<Select.Portal>
									<Select.Content
										class="z-500 max-h-60 w-[200px] min-w-[var(--bits-select-anchor-width)] overflow-hidden rounded-xl border border-gray-200 bg-white shadow-lg"
										sideOffset={8}
									>
										<Select.Viewport class="p-1">
											{#each [...periods].reverse() as period}
												<Select.Item
													class="flex h-10 w-full cursor-pointer select-none items-center rounded-lg py-3 pl-3 pr-1.5 text-sm capitalize outline-none transition-all duration-75 data-[highlighted]:bg-gray-100"
													value={period.gp}
													label={period.gp}
												>
													{#snippet children({ selected })}
														<div class="flex items-center gap-2">
															{period.gp}
														</div>
														{#if selected}
															<div class="ml-auto h-4" style="stroke:black">
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
						<div class="flex flex-col gap-2 mt-1">
							<Select.Root
								type="single"
								value={selectedParty?.code}
								onValueChange={(v) => (selectedParty = uniqueParties.find((p) => p.code === v) || undefined)}
								items={uniqueParties.map((p) => ({ value: p.code, label: p.name }))}
								allowDeselect={true}
							>
								<Select.Trigger
									class="inline-flex h-10 w-[200px] items-center justify-between rounded-xl px-[11px] bg-secondary-500 text-white transition-colors placeholder:text-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2"
								>
									<div class="flex items-center gap-2">
										{#if selectedParty}
											<div
												class="h-3 w-3 rounded-full"
												style="background-color: {selectedParty.color};"
											></div>
										{/if}
										<span class="truncate">{selectedParty?.name ||'Alle Parteien'}</span>
									</div>
									{@html upDownArrowIcon}
								</Select.Trigger>
								<Select.Portal>
									<Select.Content
										class="z-500 max-h-60 w-[200px] min-w-[var(--bits-select-anchor-width)] overflow-hidden rounded-xl border border-gray-200 bg-white shadow-lg"
										sideOffset={8}
									>
										<Select.Viewport class="p-1">
											{#each uniqueParties as party}
												<Select.Item
													class="flex h-10 w-full cursor-pointer select-none items-center rounded-lg py-3 pl-3 pr-1.5 text-sm capitalize outline-none transition-all duration-75 data-[highlighted]:bg-gray-100"
													value={party.code}
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
													<div
														class="ml-auto h-4"
														style="stroke:black"
													>
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
						</div>
					</div>

					<!-- Search Results -->
					<div class="mt-3">
						<span class="text-base font-semibold text-gray-800">Suchergebnisse</span>
						<div class="max-h-96 overflow-y-auto mt-1">
							{#each sortedDelegates as d}
								<!-- svelte-ignore a11y_click_events_have_key_events -->
								<!-- svelte-ignore a11y_no_static_element_interactions -->
								<div
									class="mb-3 flex w-full justify-between rounded-xl bg-primary-300 p-2 transition-colors hover:cursor-pointer hover:bg-primary-400"
									onclick={() => {
										const url = new URL(window.location.href);
										url.searchParams.set('delegate', d.id.toString());
										goto(url.toString(), { noScroll: true });
										isSearchPopupOpen = false;
										maybeCurrentDelegateFilter.search_value = d.name;
										currentDelegateFilterStore.value = maybeCurrentDelegateFilter;
									}}
								>
									<div class="flex">
										<img class="h-14 rounded-full" src={d.image_url} alt={d.name} />
										<div class="ml-3 mt-1">
											<h4 class="text-lg/5 font-semibold text-gray-900">{d.name}</h4>
											<div class="flex items-center gap-2">
												<div
													class="h-2 w-2 rounded-full"
													style="background-color: {partyColors.get(d.party) ?? '#ccc'};"
												></div>
												<span class="text-sm font-medium text-gray-800">{d.party}</span>
											</div>
										</div>
									</div>
									<div class="text-sm font-medium text-gray-800">
										{getMandatePeriods(d, periods)}
									</div>
								</div>
							{/each}
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
						<div class="flex items-center gap-1 rounded-xl bg-primary-600 p-2 px-3 text-white">
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
								class="relative w-auto max-w-[96vw] top-1 rounded-xl border border-gray-300 bg-surface-50 px-6 pt-4 pb-5 shadow-lg"
								data-popup="popupLegisPeriod"
							>
								<div class="mt-4 first:mt-0">
									<span class="text-base font-semibold text-gray-800">Legislaturperiode</span>
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
				<div class="mb-1 flex min-w-full justify-between px-1 text-base text-gray-800">
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
		<div class="flex min-w-full justify-between rounded-xl bg-primary-300 dark:bg-primary-200">
			<div class="w-3/4">
				<div class="relative px-8 py-5">
					{#if delegates && delegates.length > 0}
						<div class="absolute top-5 left-8 z-10">
							{#each [...groupPartyDelegates(structuredClone(delegates))].sort((a, b) => b[1].length - a[1].length) as [party, partyDelegates]}
								<div class="flex items-center gap-2">
									<div
										class="h-3 w-3 rounded-full"
										style="background-color: {partyColors.get(party) ?? '#ccc'};"
									></div>
									<span class="text-lg font-medium text-gray-800"
										>{party} ({partyDelegates.length})</span
									>
								</div>
							{/each}
						</div>
					{/if}
					{#if supplyDate && finishedMounting}
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
			<div class="w-[450px] p-4">
				{#if delegate}
					<DelegateCard {delegate} questions={generalDelegateInfo?.delegate_qa ?? []} showQA />
				{/if}
			</div>
		</div>

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
		background: #6881a1;
		cursor: pointer;
		border: none;
	}

	.range-slider::-moz-range-thumb {
		height: 20px;
		width: 20px;
		border-radius: 50%;
		background: #6881a1;
		cursor: pointer;
		border: none;
	}

	.range-slider::-webkit-slider-runnable-track {
		background: linear-gradient(
			to right,
			#6881a1 0%,
			#6881a1 var(--progress),
			#e5e7eb var(--progress),
			#e5e7eb 100%
		);
		height: 8px;
		border-radius: 4px;
	}

	.range-slider::-moz-range-track {
		background: #e5e7eb;
		height: 8px;
		border-radius: 4px;
		border: none;
	}

	.range-slider::-moz-range-progress {
		background: #6881a1;
		height: 8px;
		border-radius: 4px;
	}
</style>
