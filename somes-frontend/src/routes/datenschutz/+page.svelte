<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
</script>

<svelte:head>
	<title>Datenschutzerklärung</title>
	<meta name="description" content="Datenschutzerklärung" />
</svelte:head>

<Container>
	<!-- Main Content Card -->
	<div
		class="mt-6 rounded-3xl border border-surface-200 bg-surface-50 p-6 shadow-sm md:p-10 dark:border-surface-700 dark:bg-surface-800"
	>
		<!-- Header -->
		<div class="mb-8 border-b border-surface-200 pb-6 dark:border-surface-700">
			<h1
				class="font-heading text-3xl font-bold text-primary-900 md:text-4xl dark:text-primary-100"
			>
				Erklärung zur Informationspflicht
			</h1>
			<p class="mt-2 text-lg font-bold text-secondary-600 dark:text-secondary-400">
				Datenschutzerklärung
			</p>
		</div>

		<div class="space-y-8 leading-relaxed text-surface-700 dark:text-surface-300">
			<!-- Intro -->
			<section>
				<p>
					In folgender Datenschutzerklärung informieren wir Sie über die wichtigsten Aspekte der
					Datenverarbeitung im Rahmen unserer Webseite. Wir erheben und verarbeiten personenbezogene
					Daten nur auf Grundlage der gesetzlichen Bestimmungen (Datenschutzgrundverordnung,
					Telekommunikationsgesetz 2003).
				</p>
				<p class="mt-4">
					Sobald Sie als Benutzer auf unsere Webseite zugreifen oder diese besuchen wird Ihre
					IP-Adresse, Beginn sowie Beginn und Ende der Sitzung erfasst. Dies ist technisch bedingt
					und stellt somit ein berechtigtes Interesse iSv Art 6 Abs 1 lit f DSGVO.
				</p>
			</section>

			<!-- Sections -->
			<section>
				<h2 class="mb-3 text-xl font-bold text-primary-800 dark:text-primary-200">
					Kontakt mit uns
				</h2>
				<p>
					Wenn Sie uns, entweder über unser Kontaktformular auf unserer Webseite, oder per Email
					kontaktieren, dann werden die von Ihnen an uns übermittelten Daten zwecks Bearbeitung
					Ihrer Anfrage oder für den Fall von weiteren Anschlussfragen für sechs Monate bei uns
					gespeichert. Es erfolgt, ohne Ihre Einwilligung, keine Weitergabe Ihrer übermittelten
					Daten.
				</p>
			</section>

			<section>
				<h2 class="mb-3 text-xl font-bold text-primary-800 dark:text-primary-200">Newsletter</h2>
				<p>
					Sie haben die Möglichkeit, über unsere Website unseren Newsletter zu abonnieren. Hierfür
					benötigen wir Ihre E-Mail-Adresse und ihre Erklärung, dass Sie mit dem Bezug des
					Newsletters einverstanden sind.
				</p>
			</section>

			<section>
				<h2 class="mb-3 text-xl font-bold text-primary-800 dark:text-primary-200">Cookies</h2>
				Wir verwenden ausschließlich technisch notwendige Cookies, um die grundlegenden Funktionen dieser
				Website zu gewährleisten.
				<ul class="list-disc space-y-1 pl-5 marker:text-secondary-500">
					<li>
						<span class="font-bold">Dunkel-/Hellmodus Einstellungen: </span> Ein Cookie (theme) wird gespeichert,
						um Ihre bevorzugten Anzeigeeinstellungen (Dunkel-/Hellmodus) zu speichern, sofern Sie diese
						angepasst haben. Das vermeidet z.B. ein Flackern von Weiß auf Schwarz, wenn Sie die Seite
						neu laden. (SSR)
					</li>
				</ul>
				<br />
				Diese Cookies werden nicht für Tracking- oder Marketingzwecke verwendet.
			</section>

			<section>
				<h2 class="mb-3 text-xl font-bold text-primary-800 dark:text-primary-200">
					Ihre Rechte als Betroffener
				</h2>
				<p class="mb-3">
					Sie als Betroffener haben bezüglich Ihrer Daten, welche bei uns gespeichert sind
					grundsätzlich ein Recht auf:
				</p>
				<ul class="list-disc space-y-1 pl-5 marker:text-secondary-500">
					<li>Auskunft</li>
					<li>Löschung der Daten</li>
					<li>Berichtigung der Daten</li>
					<li>Übertragbarkeit der Daten</li>
					<li>Widerruf und Widerspruch zur Datenverarbeitung</li>
					<li>Einschränkung</li>
				</ul>
				<p class="mt-4">
					Wenn Sie vermuten, dass im Zuge der Verarbeitung Ihrer Daten Verstöße gegen das
					Datenschutzrecht passiert sind, so haben Sie die Möglichkeit sich bei uns (<a
						href="mailto:somes.austria@gmail.com"
						class="text-primary-600 underline decoration-primary-300 underline-offset-2 hover:text-primary-500"
						>somes.austria@gmail.com</a
					>) oder der Datenschutzbehörde zu beschweren.
				</p>
			</section>

			<!-- Contact Box -->
			<section
				class="rounded-xl border border-primary-200 bg-primary-100/50 p-6 dark:border-primary-800/50 dark:bg-primary-900/30"
			>
				<h2 class="mb-4 text-lg font-bold text-primary-900 dark:text-primary-100">
					Sie erreichen uns unter folgenden Kontaktdaten:
				</h2>

				<div class="grid gap-4 md:grid-cols-2">
					<div>
						<span class="block text-xs font-bold tracking-wider text-primary-500 uppercase"
							>Webseitenbetreiber</span
						>
						<span class="font-medium">somes - Verein für Demokratie und politische Transparenz</span
						>
					</div>
					<div>
						<span class="block text-xs font-bold tracking-wider text-primary-500 uppercase"
							>Email</span
						>
						<a
							href="mailto:somes.austria@gmail.com"
							class="font-medium transition-colors hover:text-primary-600"
							>somes.austria@gmail.com</a
						>
					</div>
				</div>
			</section>

			<!-- Footer Credits -->
			<div
				class="mt-8 border-t border-surface-200 pt-6 text-xs text-surface-500 dark:border-surface-700 dark:text-surface-300"
			>
				<span>Rechtstext von Quelle: </span>
				<a
					href="https://rechtstexte-generator.at/dsgvo-datenschutz-generator"
					target="_blank"
					rel="noopener noreferrer"
					class="underline hover:text-primary-600">Datenschutz Generator</a
				>
				in Kooperation mit
				<b
					><a
						href="https://rechtsanwaltwien.com/baurecht"
						target="_blank"
						rel="noopener noreferrer"
						class="underline hover:text-primary-600">Anwalt Baurecht</a
					></b
				>
			</div>
		</div>
	</div>
</Container>
