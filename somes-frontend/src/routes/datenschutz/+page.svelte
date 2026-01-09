<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
</script>

<Container>
	<!-- Main Content Card -->
	<div class="bg-surface-50 dark:bg-surface-800 rounded-3xl p-6 md:p-10 border border-surface-200 dark:border-surface-700 shadow-sm mt-6">
		
		<!-- Header -->
		<div class="border-b border-surface-200 dark:border-surface-700 pb-6 mb-8">
			<h1 class="font-heading font-bold text-3xl md:text-4xl text-primary-900 dark:text-primary-100">
				Erklärung zur Informationspflicht
			</h1>
			<p class="text-secondary-600 dark:text-secondary-400 font-bold text-lg mt-2">
				Datenschutzerklärung
			</p>
		</div>

		<div class="space-y-8 text-surface-700 dark:text-surface-300 leading-relaxed">
			
			<!-- Intro -->
			<section>
				<p>
					In folgender Datenschutzerklärung informieren wir Sie über die wichtigsten Aspekte der
					Datenverarbeitung im Rahmen unserer Webseite. Wir erheben und verarbeiten personenbezogene Daten
					nur auf Grundlage der gesetzlichen Bestimmungen (Datenschutzgrundverordnung,
					Telekommunikationsgesetz 2003).
				</p>
				<p class="mt-4">
					Sobald Sie als Benutzer auf unsere Webseite zugreifen oder diese besuchen wird Ihre IP-Adresse,
					Beginn sowie Beginn und Ende der Sitzung erfasst. Dies ist technisch bedingt und stellt somit ein
					berechtigtes Interesse iSv Art 6 Abs 1 lit f DSGVO.
				</p>
			</section>

			<!-- Sections -->
			<section>
				<h2 class="text-xl font-bold text-primary-800 dark:text-primary-200 mb-3">Kontakt mit uns</h2>
				<p>
					Wenn Sie uns, entweder über unser Kontaktformular auf unserer Webseite, oder per Email
					kontaktieren, dann werden die von Ihnen an uns übermittelten Daten zwecks Bearbeitung Ihrer
					Anfrage oder für den Fall von weiteren Anschlussfragen für sechs Monate bei uns gespeichert. Es
					erfolgt, ohne Ihre Einwilligung, keine Weitergabe Ihrer übermittelten Daten.
				</p>
			</section>

			<section>
				<h2 class="text-xl font-bold text-primary-800 dark:text-primary-200 mb-3">Newsletter</h2>
				<p>
					Sie haben die Möglichkeit, über unsere Website unseren Newsletter zu abonnieren. Hierfür benötigen
					wir Ihre E-Mail-Adresse und ihre Erklärung, dass Sie mit dem Bezug des Newsletters einverstanden
					sind.
				</p>
			</section>

			<section>
				<h2 class="text-xl font-bold text-primary-800 dark:text-primary-200 mb-3">Ihre Rechte als Betroffener</h2>
				<p class="mb-3">
					Sie als Betroffener haben bezüglich Ihrer Daten, welche bei uns gespeichert sind grundsätzlich ein
					Recht auf:
				</p>
				<ul class="list-disc pl-5 space-y-1 marker:text-secondary-500">
					<li>Auskunft</li>
					<li>Löschung der Daten</li>
					<li>Berichtigung der Daten</li>
					<li>Übertragbarkeit der Daten</li>
					<li>Widerruf und Widerspruch zur Datenverarbeitung</li>
					<li>Einschränkung</li>
				</ul>
				<p class="mt-4">
					Wenn sie vermuten, dass im Zuge der Verarbeitung Ihrer Daten Verstöße gegen das Datenschutzrecht
					passiert sind, so haben Sie die Möglichkeit sich bei uns 
					(<a href="mailto:somes.austria@gmail.com" class="text-primary-600 hover:text-primary-500 underline decoration-primary-300 underline-offset-2">somes.austria@gmail.com</a>) 
					oder der Datenschutzbehörde zu beschweren.
				</p>
			</section>

			<!-- Contact Box -->
			<section class="bg-primary-100/50 dark:bg-primary-900/30 rounded-xl p-6 border border-primary-200 dark:border-primary-800/50">
				<h2 class="text-lg font-bold text-primary-900 dark:text-primary-100 mb-4">Sie erreichen uns unter folgenden Kontaktdaten:</h2>
				
				<div class="grid gap-4 md:grid-cols-2">
					<div>
						<span class="block text-xs uppercase text-primary-500 font-bold tracking-wider">Webseitenbetreiber</span>
						<span class="font-medium">somes - Verein für Demokratie und politische Transparenz</span>
					</div>
					<div>
						<span class="block text-xs uppercase text-primary-500 font-bold tracking-wider">Email</span>
						<a href="mailto:somes.austria@gmail.com" class="font-medium hover:text-primary-600 transition-colors">somes.austria@gmail.com</a>
					</div>
				</div>
			</section>

			<!-- Footer Credits -->
			<div class="text-xs text-surface-500 border-t border-surface-200 dark:border-surface-700 pt-6 mt-8">
				<span>Rechtstext von Quelle: </span>
				<a href="https://rechtstexte-generator.at/dsgvo-datenschutz-generator" target="_blank" rel="noopener noreferrer" class="hover:text-primary-600 underline">Datenschutz Generator</a>
				in Kooperation mit 
				<b><a href="https://rechtsanwaltwien.com/baurecht" target="_blank" rel="noopener noreferrer" class="hover:text-primary-600 underline">Anwalt Baurecht</a></b>
			</div>
		</div>
	</div>
</Container>