<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import Topics from '$lib/components/Topics/Topics.svelte';
	import type { PageProps } from './$types';

	type TopicInfluence = { topic: string; influence: number };
	type StrongReferenceAnswer = {
		id: number;
		question_id: number;
		answer: string;
		stance_llm: string;
		is_strong_reference?: boolean | null;
		model_used?: string | null;
		created_at: string;
		full_stance?: any;
	};
	type OrientationQuestion = {
		id: number;
		question: string;
		is_left?: boolean | null;
		is_liberal?: boolean | null;
		is_part_of: string[];
		strong_reference_answers: StrongReferenceAnswer[];
		topics: string[];
		topics_influence: TopicInfluence[];
		detailed_topics: string[];
		detailed_topics_influence: TopicInfluence[];
	};

	let { data }: PageProps = $props();
	const allQuestions: OrientationQuestion[] = data.questions ?? [];

	let step = $state<'start' | 'quiz' | 'result'>('start');
	let quizType = $state<'short' | 'long' | null>(null);
	let strongRefMode = $state(true);

	let filteredQuestions = $derived(() => {
		if (!quizType) return [];
		return allQuestions.filter((q) => q.is_part_of.includes(quizType));
	});

	let currentIndex = $state(0);
	let answers = $state<Record<number, number | null>>({});

	function startQuiz(type: 'short' | 'long') {
		quizType = type;
		currentIndex = 0;
		answers = {};
		step = 'quiz';
	}

	function currentQuestion(): OrientationQuestion | null {
		const qs = filteredQuestions();
		return qs[currentIndex] ?? null;
	}

	function next() {
		const qs = filteredQuestions();
		if (currentIndex < qs.length - 1) {
			currentIndex++;
		} else {
			step = 'result';
		}
	}

	function prev() {
		if (currentIndex > 0) currentIndex--;
	}

	function toggleStrongRef() {
		strongRefMode = !strongRefMode;
	}

	function setAnswer(val: number) {
		const q = currentQuestion();
		if (!q) return;
		answers[q.id] = val;
	}

	const answeredCount = $derived(() => {
		const qs = filteredQuestions();
		return qs.filter((q) => answers[q.id] !== undefined && answers[q.id] !== null).length;
	});
</script>

<svelte:head>
	<title>Politische Orientierung</title>
	<meta name="description" content="Politische Orientierung Fragebogen" />
</svelte:head>

<Container>
	{#if step === 'start'}
		<h2 class="mt-6 h2">Politische Orientierung</h2>
		<p class="mt-3">Wähle die Länge des Fragebogens.</p>

		<div class="mt-4 card p-4 text-sm text-gray-700 dark:text-gray-300">
			<p class="mb-2 font-semibold">Hinweis</p>
			<p>
				<span class="font-bold"
					>Dieses Quiz ist rein als Orientierungshilfe gedacht und nicht zur Entscheidung bei Wahlen
					geeignet.
				</span>
				Es zeigt auch, was heute technisch stark automatisiert möglich ist. Der Vergleich mit anderen
				Abgeordneten am Ende des Quiz funktioniert mit dem politischen Analyseprofil der Abgeordneten,
				welches ebenfalls automatisiert generiert wird, auffindbar unter der somes-Abgeordnetenseite.
				Deine Daten bzw. Eingaben werden lediglich lokal am Gerät verarbeitet und nicht gespeichert.
			</p>
			<p class="mt-2">
				Die Pro/Contra-Positionen von "Hubert" wurden KI-generiert. Die Fragen basieren teilweise
				auf Anträgen aus dem Nationalrat der aktuellen Legislaturperiode. Dazu zählen z.B.
				Regierungsvorlagen sowie Entschließungsanträge bzw. sonstige Anträge der Opposition. Die
				Fragen wurden zu den jeweiligen Anträgen generiert und danach händisch selektiert. Dadurch
				entsteht Bias, u. a. durch den Auswahlprozess und die dominierenden Themen der Opposition
				wie Migration bzw. Klima.
				<span class="font-bold">KI-generierte (sowie menschliche) Inhalte enthalten Fehler.</span>
			</p>
		</div>

		<div class="mt-6 grid gap-4 sm:grid-cols-2">
			<button class="btn card border border-black p-6 text-left" onclick={() => startQuiz('short')}>
				<h3 class="h3">Kurz</h3>
				<p class="mt-2 text-sm">Weniger Fragen, schneller Überblick.</p>
			</button>
			<button class="btn card border border-black p-6 text-left" onclick={() => startQuiz('long')}>
				<h3 class="h3">Lang</h3>
				<p class="mt-2 text-sm">Umfassendere Abdeckung der Themen.</p>
			</button>
		</div>
	{:else if step === 'quiz'}
		{@const q = currentQuestion()}
		{@const qs = filteredQuestions()}
		{#if q}
			<h2 class="h3">Frage {currentIndex + 1} von {qs.length}</h2>

			<div class="mt-4 card p-4">
				<div class="flex flex-col gap-8">
					<div class="flex flex-col gap-6 lg:flex-row lg:gap-10">
						<div class="min-w-0 flex-1">
							<p class="mb-3 text-xl font-medium">{q.question}</p>

							{#if q.is_part_of.length}
								<div class="mt-1 flex flex-wrap gap-1">
									{#each q.is_part_of as tag}
										{@const label = tag === 'short' ? 'Kurz' : tag === 'long' ? 'Lang' : tag}
										<span class="badge bg-primary-600 text-white dark:bg-primary-800">{label}</span>
									{/each}
								</div>
							{/if}
						</div>

						{#if q.topics.length}
							<div class="hidden w-full shrink-0 lg:block lg:w-72">
								<h4 class="mb-3 text-sm font-semibold">Themen</h4>
								<Topics topics={q.topics.map((t) => ({ topic: t }))} />
							</div>
						{/if}
					</div>

					<div class="w-full">
						{#if strongRefMode}
							{#if q.strong_reference_answers.length >= 2}
								{@const pro = q.strong_reference_answers[0]}
								{@const contra = q.strong_reference_answers[1]}
								<label class="mb-2 block text-sm">Stellungnahme</label>
								<input
									type="range"
									min="0"
									max="100"
									value={answers[q.id] ?? 50}
									oninput={(e) => setAnswer(parseInt((e.target as HTMLInputElement).value))}
									class="h-10 w-full touch-manipulation"
									style="width:100%"
								/>
								<div class="mt-2 flex justify-between gap-4">
									<span class="max-w-[45%] text-xs">{pro.answer}</span>
									<span class="max-w-[45%] text-right text-xs">{contra.answer}</span>
								</div>
							{:else}
								<p class="text-sm">Keine starken Referenzen vorhanden.</p>
							{/if}
						{:else}
							<div class="mt-4 grid grid-cols-2 gap-2 sm:grid-cols-5">
								{#each ['Stark dagegen', 'Dagegen', 'Neutral', 'Dafür', 'Stark dafür'] as label, i}
									<button
										class="btn bg-primary-400 {answers[q.id] === i * 25
											? 'bg-secondary-500 text-white'
											: ''}"
										onclick={() => setAnswer(i * 25)}
									>
										{label}
									</button>
								{/each}
							</div>
						{/if}
					</div>

					{#if q.topics.length}
						<div class="block lg:hidden">
							<h4 class="mb-3 text-sm font-semibold">Themen</h4>
							<Topics topics={q.topics.map((t) => ({ topic: t }))} />
						</div>
					{/if}

					<div class="flex justify-end gap-3">
						<button
							class="btn"
							onclick={() => {
								step = 'start';
								quizType = null;
							}}>Neu starten</button
						>
						<button class="btn" onclick={prev} disabled={currentIndex === 0}>← Zurück</button>
						<button class="btn" onclick={next}>Weiter →</button>
					</div>

					<div class="flex justify-start">
						<button class="btn" onclick={toggleStrongRef}>
							{strongRefMode ? 'Zu 5-Punkte Skala' : 'Zu Slider'}
						</button>
					</div>
				</div>
			</div>
		{/if}
	{:else if step === 'result'}
		<h2 class="mt-6 h2">Ergebnis</h2>
		<p class="mt-3">Du hast {answeredCount()} Fragen beantwortet.</p>
		<div class="mt-6 space-y-4">
			{#each filteredQuestions() as q}
				<div class="card p-4">
					<p class="font-medium">{q.question}</p>
					<p class="mt-1 text-sm">Antwort: {answers[q.id] ?? 'keine'}</p>
				</div>
			{/each}
		</div>
		<button
			class="mt-6 btn"
			onclick={() => {
				step = 'start';
				quizType = null;
			}}>← Neu starten</button
		>
	{/if}
</Container>
