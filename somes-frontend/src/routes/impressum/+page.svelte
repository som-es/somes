<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
</script>

<svelte:head>
	<title>Impressum - somes.at</title>
	<meta
		name="description"
		content="Impressum und rechtliche Informationen der somes.at Plattform."
	/>
</svelte:head>

<Container>
	<!-- Main Content Card -->
	<div
		class="mt-6 mb-12 rounded-3xl border border-surface-200 bg-surface-50 p-6 shadow-sm md:p-10 dark:border-surface-700 dark:bg-surface-800"
	>
		<!-- Header -->
		<div class="mb-8 border-b border-surface-200 pb-6 dark:border-surface-700">
			<h1
				class="font-heading text-3xl font-bold text-primary-900 md:text-4xl dark:text-primary-100"
			>
				Impressum
			</h1>
			<p class="mt-2 text-sm text-surface-600 md:text-base dark:text-surface-400">
				Angaben nach § 5 E-Commerce Gesetz und Offenlegung nach § 25 Mediengesetz.
			</p>
		</div>

		<!-- Section: Media Owner (Highlighted) -->
		<section
			class="mb-10 rounded-xl border border-primary-200 bg-primary-100/50 p-6 dark:border-primary-800/50 dark:bg-primary-900/30"
		>
			<h2 class="mb-5 text-xl font-bold text-primary-900 dark:text-primary-100">
				Medieninhaber & Herausgeber
			</h2>

			<div class="space-y-4">
				<p class="text-lg font-bold text-primary-800 dark:text-primary-200">
					somes - Verein für Demokratie und politische Transparenz
				</p>

				<div class="grid gap-x-8 gap-y-4 text-sm sm:grid-cols-2">
					<div>
						<span class="mb-1 block text-xs font-bold tracking-wider text-primary-500 uppercase"
							>ZVR-Zahl</span
						>
						<span
							class="rounded bg-white/50 px-2 py-1 font-mono text-surface-700 dark:bg-black/20 dark:text-surface-300"
							>1443766724</span
						>
					</div>
					<div>
						<span class="mb-1 block text-xs font-bold tracking-wider text-primary-500 uppercase"
							>Vereinssitz</span
						>
						<span class="text-surface-700 dark:text-surface-300"
							>3470 Kirchberg am Wagram, Sonnengasse 1</span
						>
					</div>

					<!-- Kontakt -->
					<div>
						<span class="mb-1 block text-xs font-bold tracking-wider text-primary-500 uppercase"
							>E-Mail</span
						>
						<a
							href="mailto:somes.austria@gmail.com"
							class="font-medium underline decoration-primary-300 underline-offset-2 transition-colors hover:text-primary-600"
						>
							somes.austria@gmail.com
						</a>
					</div>

					<!-- Behörde & Gesetz -->
					<div
						class="mt-2 border-t border-primary-200 pt-2 sm:col-span-2 dark:border-primary-800/50"
					>
						<div class="grid gap-4 sm:grid-cols-2">
							<div>
								<span class="mb-1 block text-xs font-bold tracking-wider text-primary-500 uppercase"
									>Zuständige Behörde</span
								>
								<span class="text-surface-700 dark:text-surface-300"
									>Bezirkshauptmannschaft Tulln an der Donau</span
								>
							</div>
							<div>
								<span class="mb-1 block text-xs font-bold tracking-wider text-primary-500 uppercase"
									>Anwendbare Vorschriften</span
								>
								<a
									href="https://www.ris.bka.gv.at"
									target="_blank"
									rel="noopener noreferrer"
									class="text-surface-700 underline hover:text-primary-600 dark:text-surface-300"
									>Vereinsgesetz 2002</a
								>
							</div>
						</div>
					</div>
				</div>
			</div>
		</section>

		<!-- Section: Board Members -->
		<section class="mb-10">
			<h2
				class="mb-4 border-l-4 border-secondary-500 pl-3 text-xl font-bold text-primary-800 dark:text-primary-200"
			>
				Vorstand
			</h2>
			<p class="mb-4 text-sm text-surface-500 italic dark:text-surface-300">
				Vertretungsbefugte Organe des Vereins
			</p>
			<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
				<div
					class="rounded-lg border border-surface-200 bg-white p-4 shadow-sm dark:border-surface-600 dark:bg-surface-700/50"
				>
					<span class="block font-bold text-surface-900 dark:text-surface-100"
						>Daniel Atzwanger</span
					>
					<span
						class="text-xs font-bold tracking-wide text-secondary-600 uppercase dark:text-secondary-400"
						>Obperson</span
					>
				</div>
				<div
					class="rounded-lg border border-surface-200 bg-white p-4 shadow-sm dark:border-surface-600 dark:bg-surface-700/50"
				>
					<span class="block font-bold text-surface-900 dark:text-surface-100">Clemens Bauer</span>
					<span
						class="text-xs font-bold tracking-wide text-secondary-600 uppercase dark:text-secondary-400"
						>Obperson</span
					>
				</div>
				<div
					class="rounded-lg border border-surface-200 bg-white p-4 shadow-sm dark:border-surface-600 dark:bg-surface-700/50"
				>
					<span class="block font-bold text-surface-900 dark:text-surface-100">Florian Nagy</span>
					<span
						class="text-xs font-bold tracking-wide text-secondary-600 uppercase dark:text-secondary-400"
						>Obperson</span
					>
				</div>
				<div
					class="rounded-lg border border-surface-200 bg-white p-4 shadow-sm dark:border-surface-600 dark:bg-surface-700/50"
				>
					<span class="block font-bold text-surface-900 dark:text-surface-100">Martin Zambal</span>
					<span
						class="text-xs font-bold tracking-wide text-primary-600 uppercase dark:text-primary-400"
						>Kassier</span
					>
				</div>
			</div>
		</section>

		<!-- Section: Purpose & Means -->
		<section class="space-y-8">
			<!-- Blattlinie -->
			<div>
				<h2
					class="mb-4 border-l-4 border-tertiary-500 pl-3 text-xl font-bold text-primary-800 dark:text-primary-200"
				>
					Grundlegende Richtung & Vereinszweck
				</h2>
				<div
					class="mb-6 rounded-lg border-l-2 border-surface-300 bg-surface-100 p-4 dark:border-surface-600 dark:bg-surface-900/50"
				>
					<span
						class="mb-1 block text-xs font-bold tracking-wider text-surface-500 uppercase dark:text-surface-300"
						>Blattlinie</span
					>
					<p class="text-sm text-surface-700 dark:text-surface-300">
						Information über die Aktivitäten des Vereins sowie Förderung der politischen Bildung,
						Demokratie und Transparenz entsprechend dem nachstehenden Vereinszweck.
					</p>
				</div>

				<ol
					class="list-decimal space-y-4 pl-5 leading-relaxed text-surface-700 marker:font-bold marker:text-primary-600 dark:text-surface-300"
				>
					<li>
						Der Zweck des Vereins ist die Förderung einer gerechten, demokratischen und
						transparenten Gesellschaft im Einklang mit den Grundwerten und Prinzipien des Vereins.
						Somes setzt sich ein für die Wahrung von Frieden, Menschenrechten und Demokratie, die
						Förderung von Transparenz und Partizipation auf allen Ebenen sowie die Schaffung eines
						freien Zugangs zu Informationen und die Überwindung gesellschaftlicher Polarisierung.
						Der Verein fördert politische Bildung, internationalen Austausch und den Aufbau eines
						globalen Netzwerks, um soziale und kulturelle Barrieren zu überwinden.
					</li>
					<li>Die Tätigkeit des Vereins ist nicht auf Gewinn gerichtet.</li>
					<li>
						Der Verein verfolgt nach seinen Statuten ausschließlich und unmittelbar gemeinnützige
						und mildtätige Zwecke im Sinne der geltenden abgabenrechtlichen Bestimmungen (§§ 34 bis
						47 der Bundesabgabenordnung – BAO). Allfällige nicht im Sinne der §§ 34 ff BAO
						begünstigte Zwecke sind den begünstigten Zwecken völlig untergeordnet und werden
						höchstens im Ausmaß von 10% der Gesamtressourcen verfolgt.
					</li>
				</ol>
			</div>

			<!-- Section: Means (Mittel) -->
			<div class="border-t border-surface-200 pt-6 dark:border-surface-700">
				<h3 class="mb-4 text-lg font-bold text-primary-900 dark:text-primary-100">
					Ideelle und materielle Mittel zur Erreichung des Zwecks
				</h3>
				<p class="mb-4 text-sm text-surface-600 dark:text-surface-300">
					Der Zweck des Vereins soll durch folgende Tätigkeiten erreicht werden:
				</p>

				<ul class="grid gap-4 text-surface-700 dark:text-surface-300">
					<li class="flex gap-3">
						<span class="w-6 shrink-0 font-bold text-secondary-600 dark:text-secondary-400">a.</span
						>
						<span>
							<strong class="text-surface-900 dark:text-surface-100"
								>Politische Bildung und Aufklärung:</strong
							>
							Der Verein fördert die politische Bildung und das kritische Denken. Dazu werden Workshops,
							Seminare und Veranstaltungen zu Themen wie Demokratie, Menschenrechten und gesellschaftlicher
							Teilhabe durchgeführt. Zudem erfolgt die Verbreitung von Wissen durch Publikationen, Studien
							und Analysen.
						</span>
					</li>
					<li class="flex gap-3">
						<span class="w-6 shrink-0 font-bold text-secondary-600 dark:text-secondary-400">b.</span
						>
						<span>
							<strong class="text-surface-900 dark:text-surface-100"
								>Förderung von Transparenz und Partizipation:</strong
							>
							Der Verein setzt Initiativen zur Förderung der politischen Transparenz und Partizipation
							auf lokaler, nationaler und internationaler Ebene um. Dabei werden Plattformen geschaffen,
							die Bürgerinnen und Bürgern den Zugang zu Informationen und Mitbestimmung erleichtern.
						</span>
					</li>
					<li class="flex gap-3">
						<span class="w-6 shrink-0 font-bold text-secondary-600 dark:text-secondary-400">c.</span
						>
						<span>
							<strong class="text-surface-900 dark:text-surface-100"
								>Aufbau und Betrieb der Online-Plattform „somes“:</strong
							>
							Der Verein entwickelt und pflegt eine Online-Plattform zur Förderung von politischer Transparenz,
							demokratischer Partizipation und des offenen Dialogs. Dabei werden Tools und Ressourcen
							bereitgestellt, die den Austausch von Ideen, die Organisation von Initiativen und die weltweite
							Vernetzung von Menschen erleichtern. Zudem wird die Plattform unter besonderer Berücksichtigung
							des Datenschutzes und der Barrierefreiheit für alle Nutzer gestaltet.
						</span>
					</li>
					<li class="flex gap-3">
						<span class="w-6 shrink-0 font-bold text-secondary-600 dark:text-secondary-400">d.</span
						>
						<span>
							<strong class="text-surface-900 dark:text-surface-100"
								>Überwindung gesellschaftlicher Spaltung:</strong
							>
							Der Verein führt Dialogforen durch, um soziale Blasen und gesellschaftliche Polarisierung
							zu verringern. Dabei wird der Austausch unterschiedlicher Perspektiven gefördert, um Verständnis
							und gesellschaftlichen Zusammenhalt zu stärken.
						</span>
					</li>
					<li class="flex gap-3">
						<span class="w-6 shrink-0 font-bold text-secondary-600 dark:text-secondary-400">e.</span
						>
						<span>
							<strong class="text-surface-900 dark:text-surface-100"
								>Internationale Zusammenarbeit und Netzwerkarbeit:</strong
							>
							Der Verein baut ein globales Netzwerk zur Förderung von Frieden und internationalem Zusammenhalt
							auf und pflegt dieses aktiv. Dabei werden Projekte unterstützt, die den interkulturellen
							Dialog und die Zusammenarbeit zwischen Nationen fördern.
						</span>
					</li>
					<li class="flex gap-3">
						<span class="w-6 shrink-0 font-bold text-secondary-600 dark:text-secondary-400">f.</span
						>
						<span>
							<strong class="text-surface-900 dark:text-surface-100"
								>Einsatz für ein freies Internet:</strong
							>
							Der Verein fördert den freien Zugang zu unabhängigen und qualitativ hochwertigen Informationen.
							Zudem setzt er sich für Open-Source, Datenschutz, Medienkompetenz und die kritische Auseinandersetzung
							mit Desinformationen ein.
						</span>
					</li>
					<li class="flex gap-3">
						<span class="w-6 shrink-0 font-bold text-secondary-600 dark:text-secondary-400">g.</span
						>
						<span>
							<strong class="text-surface-900 dark:text-surface-100"
								>Investigative Aufklärungsarbeit:</strong
							>
							Der Verein führt investigative Recherchen durch, um gesellschaftliche Missstände aufzudecken.
							Dabei werden Analysen und Daten veröffentlicht, die zur Förderung fundierter öffentlicher
							Diskussionen beitragen.
						</span>
					</li>
					<li class="flex gap-3">
						<span class="w-6 shrink-0 font-bold text-secondary-600 dark:text-secondary-400">h.</span
						>
						<span>
							<strong class="text-surface-900 dark:text-surface-100"
								>Beratung und Unterstützung:</strong
							>
							Der Verein berät staatliche Institutionen, Unternehmen und andere Organisationen in Übereinstimmung
							mit seinen Grundwerten. Zudem unterstützt er Projekte und Initiativen, die den Zielen des
							Vereins dienen.
						</span>
					</li>
					<li class="flex gap-3">
						<span class="w-6 shrink-0 font-bold text-secondary-600 dark:text-secondary-400">i.</span
						>
						<span>
							<strong class="text-surface-900 dark:text-surface-100"
								>Organisation von und Beteiligung an gesellschaftlichen Initiativen:</strong
							>
							Der Verein plant und unterstützt Volksbegehren, Demonstrationen und Veranstaltungen im Einklang
							mit seinen Werten. Darüber hinaus fördert er den Austausch zwischen Mitgliedern und Partnerorganisationen.
						</span>
					</li>
					<li class="flex gap-3 rounded-lg bg-surface-100 p-3 text-sm dark:bg-surface-900/50">
						<span class="w-6 shrink-0 font-bold text-secondary-600 dark:text-secondary-400">j.</span
						>
						<span class="text-surface-600 italic dark:text-surface-400">
							Sofern dies dem Vereinszweck dient, ist der Verein weiters berechtigt, sich an
							Kapitalgesellschaften zu beteiligen, sich Erfüllungsgehilfen gemäß § 40 Abs. 1 BAO zu
							bedienen oder selbst als Erfüllungsgehilfe tätig zu werden, Geldmittel gemäß § 40a Z 1
							BAO weiterzuleiten, entgeltliche Lieferungen gemäß § 40a Z 2 BAO ohne
							Gewinnerzielungsabsicht zu erbringen sowie Geldmittel gemäß § 40b BAO für Preise und
							Stipendien zur Verfügung zu stellen.
						</span>
					</li>
				</ul>
			</div>
		</section>

		<!-- Section: Haftungsausschluss (UPDATED) -->
		<section class="mt-10 border-t border-surface-200 pt-8 dark:border-surface-700">
			<h2
				class="mb-6 border-l-4 border-secondary-500 pl-3 text-xl font-bold text-primary-800 dark:text-primary-200"
			>
				Haftungsausschluss (Disclaimer)
			</h2>

			<div class="space-y-8">
				<!-- 1. Unabhängigkeit -->
				<div>
					<h3 class="mb-3 text-base font-bold text-surface-900 dark:text-surface-100">
						1. Unabhängigkeit der Plattform
					</h3>
					<div
						class="rounded-lg border-l-2 border-surface-300 bg-surface-100 p-4 dark:border-surface-600 dark:bg-surface-900/50"
					>
						<p class="text-sm text-surface-700 dark:text-surface-300">
							<strong>somes</strong> ist ein unabhängiges, zivilgesellschaftliches Projekt
							(entstanden aus einer HTL-Diplomarbeit) und steht in
							<strong>keiner offiziellen Verbindung</strong> zur Republik Österreich, der Parlamentsdirektion,
							Regierungsorganen oder politischen Parteien. Weder die Betreiber und Entwickler dieser Website,
							noch die Mitglieder des Vereinsvorstands, sind Mitglied bei einer politischen Partei oder
							stehen einer nahe.
						</p>
					</div>
				</div>

				<!-- 2. Herkunft der Daten -->
				<div>
					<h3 class="mb-3 text-base font-bold text-surface-900 dark:text-surface-100">
						2. Herkunft und Richtigkeit der Daten
					</h3>
					<div class="space-y-3 text-sm leading-relaxed text-surface-700 dark:text-surface-300">
						<p>
							Die auf dieser Plattform dargestellten Informationen basieren auf öffentlich
							zugänglichen Daten (Open Data), primär bereitgestellt durch das österreichische
							Parlament (parlament.gv.at) und dem Rechtsinformationssytem des Bundes
							(www.ris.bka.gv.at). Die Erfassung, Aggregation und Aufbereitung dieser Daten erfolgt
							vollständig automatisiert und wird freiwillig, kostenlos, ohne Werbung und ohne
							Tracking zur Verfügung gestellt.
						</p>
						<p>
							Wir bemühen uns um höchste Sorgfalt, übernehmen jedoch <strong
								>keinerlei Gewähr und Haftung</strong
							> (auch nicht bei leichter Fahrlässigkeit):
						</p>
						<ul class="list-disc space-y-1 pl-5 marker:text-primary-500">
							<li>
								für die Richtigkeit, Vollständigkeit, Aktualität und technische Verfügbarkeit der
								bereitgestellten Daten;
							</li>
							<li>
								für Fehler, die durch automatisierte Verarbeitungsprozesse (z.B. Parsing-Fehler)
								oder durch Fehlinterpretationen der Originaldaten entstehen;
							</li>
							<li>
								für Handlungen oder Entscheidungen, die auf Basis der auf <em>somes</em> bereitgestellten
								Informationen getroffen werden.
							</li>
						</ul>
						<p class="text-surface-600 italic dark:text-surface-400">
							Im Zweifelsfall gelten immer die offiziellen Dokumente und Publikationen der
							Parlamentsdirektion. Die Nutzung dieser Webseite erfolgt auf eigene Gefahr.
						</p>
					</div>
				</div>

				<!-- 3. Links -->
				<div>
					<h3 class="mb-3 text-base font-bold text-surface-900 dark:text-surface-100">
						3. Haftung für Links
					</h3>
					<p class="text-sm leading-relaxed text-surface-700 dark:text-surface-300">
						Unsere Plattform enthält Links zu externen Webseiten Dritter, auf deren Inhalte wir
						keinen Einfluss haben. Für die Inhalte der verlinkten Seiten ist stets der jeweilige
						Anbieter oder Betreiber verantwortlich. Zum Zeitpunkt der Verlinkung wurden die Seiten
						auf mögliche Rechtsverletzungen geprüft; rechtswidrige Inhalte waren nicht erkennbar.
						Eine permanente inhaltliche Kontrolle der verlinkten Seiten ist jedoch ohne konkrete
						Anhaltspunkte einer Rechtsverletzung nicht zumutbar. Bei Bekanntwerden von
						Rechtsverletzungen werden wir derartige Links umgehend entfernen.
					</p>
				</div>
			</div>
		</section>

		<!-- ODR & Footer Info -->
		<div
			class="mt-10 border-t border-surface-200 pt-6 text-xs text-surface-500 dark:border-surface-700 dark:text-surface-400"
		>
			Verbraucher haben die Möglichkeit, Beschwerden an die Online-Streitbeilegungsplattform der EU
			zu richten:
			<a
				href="http://ec.europa.eu/odr"
				target="_blank"
				rel="noopener noreferrer"
				class="underline hover:text-primary-600">http://ec.europa.eu/odr</a
			>. Sie können allfällige Beschwerden auch an die oben angegebene E-Mail-Adresse richten.
		</div>
	</div>
</Container>
