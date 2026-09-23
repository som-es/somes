<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
</script>

<svelte:head>
	<title>Moderationskodex</title>
	<meta name="description" content="Moderationskodex für das Fragesystem von somes" />
</svelte:head>

<Container>
	<div
		class="mt-6 rounded-3xl border border-surface-200 bg-surface-50 p-6 shadow-sm md:p-10 dark:border-surface-700 dark:bg-surface-800"
	>
		<div class="mb-8 border-b border-surface-200 pb-6 dark:border-surface-700">
			<h1
				class="font-heading text-3xl font-bold text-primary-900 md:text-4xl dark:text-primary-100"
			>
				Moderationskodex
			</h1>
			<p class="mt-2 text-lg font-bold text-secondary-600 dark:text-secondary-400">Fragesystem</p>
		</div>

		<div class="space-y-8 leading-relaxed text-surface-700 dark:text-surface-300">
			<section>
				<p>
					Das Fragesystem von somes soll eine überparteiliche, sachliche und individuelle
					Kommunikation zwischen Bürger:innen und ihren Abgeordneten ermöglichen.
				</p>
				<p class="mt-4">
					Alle über somes eingehenden Fragen und Antworten werden von einem Moderationsteam
					gegengelesen und anschließend freigeschaltet, sofern sie nicht gegen diesen Kodex
					verstoßen. Voraussetzung für das Stellen von Fragen ist eine Anmeldung bei somes.
				</p>
				<p class="mt-4">
					Das Moderationsteam von somes behält sich vor, nach Rücksprache mit dem Vereinsvorstand
					bei wiederholtem Missbrauch die verantwortliche Person und/oder die betreffende
					Parteigliederung von der Plattform auszuschließen und bei entsprechenden Verstößen auch
					rechtliche Schritte einzuleiten.
				</p>
			</section>

			<section>
				<h2 class="mb-3 text-xl font-bold text-primary-800 dark:text-primary-200">
					Nicht freigeschaltet werden insbesondere:
				</h2>
				<ul class="list-disc space-y-1 pl-5 marker:text-secondary-500">
					<li>
						Beiträge, die Gewaltherrschaft, Rassismus, Sexismus sowie politische oder religiöse
						Verfolgung befürworten oder deren Opfer missachten oder verhöhnen.
					</li>
					<li>
						Beiträge, in denen Menschen pauschalisierend aufgrund ihrer Gruppenzugehörigkeit
						diskriminiert werden.
					</li>
					<li>
						Beiträge mit Beleidigungen, Beschimpfungen oder menschenverachtenden Formulierungen.
					</li>
					<li>Fragen zum Privatleben.</li>
					<li>Fragen, die unter eine berufliche Schweigepflicht fallen.</li>
					<li>
						Beiträge, die keine Frage oder Aufforderung zur Stellungnahme enthalten, sondern
						ausschließlich eine bloße Meinungsäußerung darstellen.
					</li>
					<li>
						Beiträge mit Tatsachenbehauptungen, Zitaten oder Statistiken (Zahlen), die nicht durch
						Quellen belegt sind.
					</li>
					<li>Massen-Mails.</li>
					<li>
						Eine unangemessen hohe Anzahl von Fragen durch einzelne Fragesteller:innen oder an
						einzelne Abgeordnete.
					</li>
					<li>Mehrere Nachfragen, in der Regel mehr als eine.</li>
					<li>
						Fragen von Abgeordnetenmitarbeiter:innen aus Partei- und Klubgeschäftsstellen sowie
						selbst gestellte Fragen.
					</li>
				</ul>
				<p class="mt-4">
					Mitglieder des Moderationsteams und des Vorstands dürfen ausdrücklich selbst Fragen
					stellen, bis ein natürlicher Fragefluss entsteht. Damit möchten und sollen sie keine
					(neuen) politischen Debatten provozieren.
				</p>
			</section>

			<section>
				<h2 class="mb-3 text-xl font-bold text-primary-800 dark:text-primary-200">
					Länge der Fragen
				</h2>
				<p>
					Um die Lesbarkeit und Akzeptanz von somes zu gewährleisten, sollen Fragen eine Länge von
					200 Zeichen nicht überschreiten. Zusätzlich stehen den Fragestellenden optional bis zu
					1.000 Zeichen zur Verfügung, um die Frage zu begründen oder Quellen und weitere
					Informationen anzugeben.
				</p>
			</section>

			<p class="text-sm text-surface-500 dark:text-surface-400">(Letzte Änderung: 23.09.2026)</p>
		</div>
	</div>
</Container>
