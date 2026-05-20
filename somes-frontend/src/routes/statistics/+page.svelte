<script lang="ts">
	import AbsencesStatistics from './absences/+page.svelte';
	import ActivityStatistics from './activity/+page.svelte';
	import AgeStatistics from './age/+page.svelte';
	import CallToOrdersStatistics from './call_to_orders/+page.svelte';
	import ComplexityStatistics from './complexity/+page.svelte';
	import OrientationStatistics from './orientation/+page.svelte';
	import SpeechTimeStatistics from './speech_time/+page.svelte';
	import TotalSpeechesStatistics from './total_speeches/+page.svelte';

	const sections = [
		{ id: 'speech-time', label: 'Redezeit', component: SpeechTimeStatistics },
		{ id: 'total-speeches', label: 'Gehaltene Reden', component: TotalSpeechesStatistics },
		{ id: 'absences', label: 'Abwesenheiten', component: AbsencesStatistics },
		{ id: 'activity', label: 'Aktivität', component: ActivityStatistics },
		{ id: 'call-to-orders', label: 'Ordnungsrufe', component: CallToOrdersStatistics },
		{ id: 'complexity', label: 'Sprachkomplexität', component: ComplexityStatistics },
		{ id: 'age', label: 'Alter', component: AgeStatistics },
		{ id: 'orientation', label: 'Politische Positionen', component: OrientationStatistics }
	];

	function scrollToSection(index: number) {
		const section = sections[index];
		if (!section) return;

		document.getElementById(section.id)?.scrollIntoView({
			behavior: 'smooth',
			block: 'start'
		});
		window.history.replaceState(null, '', `#${section.id}`);
	}
</script>

<svelte:head>
	<title>Statistiken - Parlamentsinformationssystem</title>
	<meta name="description" content="Parlamentarische Statistiken als scrollbare Gesamtübersicht" />
</svelte:head>

<div class="statistics-snap-page">
	{#each sections as section, index}
		<section id={section.id} class="statistics-snap-section">
			<div class="statistics-section-inner">
				<div class="statistics-section-nav">
					{#if index > 0}
						<button
							type="button"
							class="statistics-section-button"
							aria-label="Zur vorherigen Statistik: {sections[index - 1].label}"
							title="Zur vorherigen Statistik"
							onclick={() => scrollToSection(index - 1)}
						>
							<span aria-hidden="true">↑</span>
						</button>
					{/if}
				</div>
				<svelte:component this={section.component} />
				<div class="statistics-section-nav">
					{#if index < sections.length - 1}
						<button
							type="button"
							class="statistics-section-button"
							aria-label="Zur nächsten Statistik: {sections[index + 1].label}"
							title="Zur nächsten Statistik"
							onclick={() => scrollToSection(index + 1)}
						>
							<span aria-hidden="true">↓</span>
						</button>
					{/if}
				</div>
			</div>
		</section>
	{/each}
</div>

<style>
	:global(html) {
		scroll-behavior: smooth;
		scroll-snap-type: y mandatory;
		scroll-padding-top: 0;
	}

	.statistics-snap-page {
		scroll-snap-type: y mandatory;
	}

	.statistics-snap-section {
		position: relative;
		min-height: 100svh;
		scroll-snap-align: start;
		scroll-snap-stop: always;
		display: grid;
		align-items: start;
		padding-block: 0.5rem 1rem;
	}

	.statistics-section-inner {
		display: grid;
		grid-template-rows: 1.25rem minmax(0, 1fr) 2.75rem;
		min-height: calc(100svh - 1.5rem);
	}

	.statistics-section-nav {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	:global(.statistics-snap-section [class*='pb-12']) {
		padding-bottom: 0;
	}

	:global(.statistics-snap-section [class*='mb-6']) {
		margin-bottom: 0.75rem;
	}

	.statistics-section-button {
		display: grid;
		height: 1.75rem;
		width: 1.75rem;
		place-items: center;
		border-radius: 9999px;
		border: 1px solid rgb(209 213 219);
		background: rgb(255 255 255 / 0.9);
		color: rgb(31 41 55);
		font-size: 1rem;
		font-weight: 700;
		box-shadow: 0 4px 14px rgb(15 23 42 / 0.1);
		backdrop-filter: blur(8px);
		transition:
			background-color 150ms ease,
			transform 150ms ease,
			box-shadow 150ms ease;
	}

	.statistics-section-button:hover {
		background: rgb(195 205 217);
		transform: scale(1.06);
		box-shadow: 0 10px 24px rgb(15 23 42 / 0.16);
	}

	:global(.dark) .statistics-section-button {
		border-color: rgb(55 65 81);
		background: rgb(22 24 37 / 0.88);
		color: rgb(249 250 251);
	}

	:global(.dark) .statistics-section-button:hover {
		background: rgb(78 97 121);
	}
</style>
