<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import type { Absence, Delegate, LegisPeriod, PlenarySession } from '$lib/types';
	import { Dialog, Popover } from 'bits-ui';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import { onMount } from 'svelte';
	import { activePlenarySessionsForDelegate } from '$lib/activePlenarySessions';
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import { cachedPlenarySessions } from '$lib/caching/plenarySessions';
	import { formatDate } from '$lib/date';

	interface Props {
		absences: Absence[];
		title?: string;
		showDetails?: boolean;
		delegate: Delegate;
	}

	let {
		absences = [],
		title = t('absences.title'),
		showDetails = true,
		delegate
	}: Props = $props();
	let absentOrNotPlenarySessionPerGp: {
		gp: LegisPeriod;
		sessions: { absencesDuringSession?: Absence[]; session: PlenarySession }[];
	}[] = $state([]);

	onMount(async () => {
		const allPlenarySessions = await cachedPlenarySessions();
		const legisPeriods = await cachedAllLegisPeriods();
		if (allPlenarySessions && legisPeriods) {
			const activePlenarySessionsPerGp = activePlenarySessionsForDelegate(
				delegate.mandates ?? [],
				allPlenarySessions
			);
			for (const [gp, plenarySessions] of Object.entries(activePlenarySessionsPerGp)) {
				const sessions = plenarySessions.map((plenarySession) => {
					const absencesDuringSession: Absence[] = absences.filter(
						(absence) => absence.plenary_session_id == plenarySession.id
					);
					return { absencesDuringSession, session: plenarySession };
				});
				const legisPeriod = legisPeriods.find((period) => period.gp == gp);
				if (legisPeriod) absentOrNotPlenarySessionPerGp.push({ gp: legisPeriod, sessions });
			}
		}
		absentOrNotPlenarySessionPerGp.sort(
			(a, b) => new Date(b.gp.start_date).getTime() - new Date(a.gp.start_date).getTime()
		);
	});

	let totalAbsences = $derived(absences.length);

	let openSessionId: number | null = $state(null);
</script>

<div class="card p-8">
	<div class="flex items-center justify-between">
		<span></span>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>
	<!-- Card Container with soft steel-blue background and deep rounding -->
	<div class="mt-3 rounded-3xl bg-[#c8d4df] p-8 shadow-sm select-none">
		<!-- Card Title -->
		<h2 class="mb-6 text-3xl font-semibold text-slate-800/90">{t('absences.modalTitle')}</h2>

		<!-- GP Sections -->
		<div class="space-y-6">
			{#each absentOrNotPlenarySessionPerGp as entry (entry.gp)}
				<div class="space-y-3">
					<!-- GP Label (Centered, light serif/sans tracking) -->
					<div class="text-center text-xl font-normal tracking-widest text-slate-700/80">
						{entry.gp.gp}
					</div>

					<!-- 10-Column Grid for Sessions -->
					<div class="flex flex-wrap gap-x-2 gap-y-1.5">
						{#each entry.sessions as item, i (item.session.id)}
							{@const isAbsent = (item.absencesDuringSession?.length ?? 0) !== 0}
							<Popover.Root
								open={openSessionId === item.session.id}
								onOpenChange={(isOpen) => {
									if (isOpen) openSessionId = item.session.id;
									else if (openSessionId === item.session.id) openSessionId = null;
								}}
							>
								<Popover.Trigger openOnHover openDelay={0}>
									<svelte:element
										this={"div"}
										class="flex h-8 w-8 items-center justify-center rounded-md text-xs font-normal transition-all duration-150
									{isAbsent ? 'text-slate/70 bg-tertiary-400/80 text-black' : 'bg-primary-400 text-white'}"
									>
										{i + 1}
									</svelte:element>
								</Popover.Trigger>
								<Popover.Portal>
									<Popover.Content
										side="top"
										class="text-primary-950 z-70 max-w-sm rounded-xl border border-primary-200 bg-primary-50 p-4 text-sm shadow-xl focus:outline-none data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
									>
										<div class="flex w-64 flex-col gap-3">
											<!-- Popover Header -->
											<div
												class="flex items-center justify-between border-b border-primary-200 pb-2"
											>
												<span class="font-bold text-primary-900">
													{i + 1}. Sitzung
												</span>
												<span
													class="text-xs font-semibold tracking-wider text-primary-500 uppercase"
												>
													{item.session.legislative_period}
												</span>
											</div>

											<!-- Session Title & Description -->
											<div class="space-y-1">
												{#if item.session.description}
													<p class="text-[11px] leading-normal text-primary-600">
														{item.session.title}
													</p>
												{/if}
											</div>

											<!-- Dynamic Status Card (Present vs. Absent) -->
											{#if isAbsent}
												{#each item.absencesDuringSession ?? [] as absence}
													<div
														class="text-tertiary-950 rounded-lg border border-primary-300/50 bg-primary-200/50 p-3 text-xs"
													>
														<div class="mb-1.5 flex items-center justify-between">
															<span
																class="badge bg-tertiary-400/80 px-2 py-0.75 text-[10px] font-bold tracking-wider text-black"
															>
																{t('absences.absent')}
															</span>
															{#if absence.date}
																<span class="text-[10px] font-medium text-primary-600">
																	{formatDate(absence.date)}
																</span>
															{/if}
														</div>
														<!-- Document & Source Links -->
														{#if absence.source_url}
															<div
																class="flex flex-col gap-1.5 border-t border-primary-200/60 pt-2 text-[11px]"
															>
																<a
																	href={absence.source_url}
																	target="_blank"
																	rel="noopener noreferrer"
																	class="flex items-center gap-1 font-semibold text-tertiary-700 underline transition-colors hover:text-tertiary-900"
																>
																	{t('absences.sourceLink')}
																</a>
															</div>
														{/if}
													</div>
												{/each}
											{:else}
												<div
													class="text-primary-950 rounded-lg border border-primary-300/50 bg-primary-200/50 p-3 text-xs"
												>
													<div class="flex items-center justify-between">
														<span
															class="badge bg-primary-400 px-2 py-0.75 text-[10px] font-bold tracking-wider text-white"
														>
															{t('absences.present')}
														</span>
														{#if item.session.created_at}
															<span class="text-[10px] font-medium text-primary-600">
																{formatDate(item.session.raw_data_created_at)}
															</span>
														{/if}
													</div>
												</div>
											{/if}
										</div>
									</Popover.Content>
								</Popover.Portal>
							</Popover.Root>
						{/each}
					</div>
				</div>
			{/each}
		</div>
		<!-- Bottom Absence Counter -->
		<div class="mt-6 pt-2 text-xl font-normal text-slate-700/90">
			{totalAbsences}
			{totalAbsences === 1 ? t('absences.countSingular') : t('absences.countPlural')}
		</div>
	</div>
</div>
