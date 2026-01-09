<script lang="ts">
	import Container from "$lib/components/Layout/Container.svelte";
</script>

<svelte:head>
    <title>Impressum - somes.at</title>
    <meta name="description" content="Impressum und rechtliche Informationen der somes.at Plattform." />
</svelte:head>

<Container>
	<!-- Main Content Card -->
	<div class="bg-surface-50 dark:bg-surface-800 rounded-3xl p-6 md:p-10 border border-surface-200 dark:border-surface-700 shadow-sm mt-6 mb-12">
		
		<!-- Header -->
		<div class="border-b border-surface-200 dark:border-surface-700 pb-6 mb-8">
			<h1 class="font-heading font-bold text-3xl md:text-4xl text-primary-900 dark:text-primary-100">
				Impressum
			</h1>
			<p class="text-surface-600 dark:text-surface-400 mt-2 text-sm md:text-base">
				Angaben nach § 5 E-Commerce Gesetz und Offenlegung nach § 25 Mediengesetz.
			</p>
		</div>

		<!-- Section: Media Owner (Highlighted) -->
		<section class="bg-primary-100/50 dark:bg-primary-900/30 rounded-xl p-6 border border-primary-200 dark:border-primary-800/50 mb-10">
			<h2 class="text-xl font-bold text-primary-900 dark:text-primary-100 mb-5">Medieninhaber & Herausgeber</h2>
			
			<div class="space-y-4">
				<p class="text-lg font-bold text-primary-800 dark:text-primary-200">
					somes - Verein für Demokratie und politische Transparenz
				</p>
				
				<div class="grid sm:grid-cols-2 gap-y-4 gap-x-8 text-sm">
					<div>
						<span class="block text-xs uppercase text-primary-500 font-bold tracking-wider mb-1">ZVR-Zahl</span>
						<span class="font-mono text-surface-700 dark:text-surface-300 bg-white/50 dark:bg-black/20 px-2 py-1 rounded">1443766724</span>
					</div>
					<div>
						<span class="block text-xs uppercase text-primary-500 font-bold tracking-wider mb-1">Vereinssitz</span>
						<span class="text-surface-700 dark:text-surface-300">3470 Kirchberg am Wagram, Sonnengasse 1</span>
					</div>
                    
                    <!-- Kontakt -->
					<div>
						<span class="block text-xs uppercase text-primary-500 font-bold tracking-wider mb-1">E-Mail</span>
						<a href="mailto:somes.austria@gmail.com" class="font-medium hover:text-primary-600 underline decoration-primary-300 underline-offset-2 transition-colors">
							somes.austria@gmail.com
						</a>
					</div>

                    <!-- Behörde & Gesetz -->
                    <div class="sm:col-span-2 pt-2 border-t border-primary-200 dark:border-primary-800/50 mt-2">
                        <div class="grid sm:grid-cols-2 gap-4">
                            <div>
                                <span class="block text-xs uppercase text-primary-500 font-bold tracking-wider mb-1">Zuständige Behörde</span>
                                <span class="text-surface-700 dark:text-surface-300">Bezirkshauptmannschaft Tulln</span>
                            </div>
                            <div>
                                <span class="block text-xs uppercase text-primary-500 font-bold tracking-wider mb-1">Anwendbare Vorschriften</span>
                                <a href="https://www.ris.bka.gv.at" target="_blank" rel="noopener noreferrer" class="text-surface-700 dark:text-surface-300 hover:text-primary-600 underline">Vereinsgesetz 2002</a>
                            </div>
                        </div>
                    </div>
				</div>
			</div>
		</section>

		<!-- Section: Board Members -->
		<section class="mb-10">
			<h2 class="text-xl font-bold text-primary-800 dark:text-primary-200 mb-4 border-l-4 border-secondary-500 pl-3">
				Vorstand
			</h2>
            <p class="text-sm text-surface-500 mb-4 italic">Vertretungsbefugte Organe des Vereins</p>
			<div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
				<div class="p-4 bg-white dark:bg-surface-700/50 rounded-lg border border-surface-200 dark:border-surface-600 shadow-sm">
					<span class="block font-bold text-surface-900 dark:text-surface-100">Daniel Atzwanger</span>
					<span class="text-xs text-secondary-600 dark:text-secondary-400 uppercase tracking-wide font-bold">Obperson</span>
				</div>
				<div class="p-4 bg-white dark:bg-surface-700/50 rounded-lg border border-surface-200 dark:border-surface-600 shadow-sm">
					<span class="block font-bold text-surface-900 dark:text-surface-100">Clemens Bauer</span>
					<span class="text-xs text-secondary-600 dark:text-secondary-400 uppercase tracking-wide font-bold">Obperson</span>
				</div>
				<div class="p-4 bg-white dark:bg-surface-700/50 rounded-lg border border-surface-200 dark:border-surface-600 shadow-sm">
					<span class="block font-bold text-surface-900 dark:text-surface-100">Florian Nagy</span>
					<span class="text-xs text-secondary-600 dark:text-secondary-400 uppercase tracking-wide font-bold">Obperson</span>
				</div>
				<div class="p-4 bg-white dark:bg-surface-700/50 rounded-lg border border-surface-200 dark:border-surface-600 shadow-sm">
					<span class="block font-bold text-surface-900 dark:text-surface-100">Martin Zambal</span>
					<span class="text-xs text-primary-600 dark:text-primary-400 uppercase tracking-wide font-bold">Kassier</span>
				</div>
			</div>
		</section>

		<!-- Section: Purpose & Means -->
		<section class="space-y-8">
			
            <!-- Blattlinie -->
            <div>
                <h2 class="text-xl font-bold text-primary-800 dark:text-primary-200 mb-4 border-l-4 border-tertiary-500 pl-3">
                    Grundlegende Richtung & Vereinszweck
                </h2>
                <div class="bg-surface-100 dark:bg-surface-900/50 p-4 rounded-lg border-l-2 border-surface-300 dark:border-surface-600 mb-6">
                    <span class="block text-xs uppercase text-surface-500 font-bold tracking-wider mb-1">Blattlinie</span>
                    <p class="text-surface-700 dark:text-surface-300 text-sm">
                        Information über die Aktivitäten des Vereins sowie Förderung der politischen Bildung, Demokratie und Transparenz entsprechend dem nachstehenden Vereinszweck.
                    </p>
                </div>

                <ol class="list-decimal pl-5 space-y-4 text-surface-700 dark:text-surface-300 leading-relaxed marker:text-primary-600 marker:font-bold">
                    <li>
                        Der Zweck des Vereins ist die Förderung einer gerechten, demokratischen und transparenten
                        Gesellschaft im Einklang mit den Grundwerten und Prinzipien des Vereins. Somes setzt sich
                        ein für die Wahrung von Frieden, Menschenrechten und Demokratie, die Förderung von
                        Transparenz und Partizipation auf allen Ebenen sowie die Schaffung eines freien Zugangs zu
                        Informationen und die Überwindung gesellschaftlicher Polarisierung. Der Verein fördert
                        politische Bildung, internationalen Austausch und den Aufbau eines globalen Netzwerks, um
                        soziale und kulturelle Barrieren zu überwinden.
                    </li>
                    <li>
                        Die Tätigkeit des Vereins ist nicht auf Gewinn gerichtet.
                    </li>
                    <li>
                        Der Verein verfolgt nach seinen Statuten ausschließlich und unmittelbar gemeinnützige und
                        mildtätige Zwecke im Sinne der geltenden abgabenrechtlichen Bestimmungen (§§ 34 bis 47
                        der Bundesabgabenordnung – BAO). Allfällige nicht im Sinne der §§ 34 ff BAO begünstigte
                        Zwecke sind den begünstigten Zwecken völlig untergeordnet und werden höchstens im
                        Ausmaß von 10% der Gesamtressourcen verfolgt.
                    </li>
                </ol>
            </div>

            <!-- New Section: Means (Mittel) -->
            <div class="pt-6 border-t border-surface-200 dark:border-surface-700">
                <h3 class="text-lg font-bold text-primary-900 dark:text-primary-100 mb-4">
                    Ideelle und materielle Mittel zur Erreichung des Zwecks
                </h3>
                <p class="text-surface-600 dark:text-surface-400 mb-4 text-sm">
                    Der Zweck des Vereins soll durch folgende Tätigkeiten erreicht werden:
                </p>
                
                <ul class="grid gap-4 text-surface-700 dark:text-surface-300">
                    <li class="flex gap-3">
                        <span class="font-bold text-secondary-600 dark:text-secondary-400 shrink-0 w-6">a.</span>
                        <span>
                            <strong class="text-surface-900 dark:text-surface-100">Politische Bildung und Aufklärung:</strong> 
                            Der Verein fördert die politische Bildung und das kritische Denken. Dazu werden Workshops, Seminare und Veranstaltungen zu Themen wie Demokratie, Menschenrechten und gesellschaftlicher Teilhabe durchgeführt. Zudem erfolgt die Verbreitung von Wissen durch Publikationen, Studien und Analysen.
                        </span>
                    </li>
                    <li class="flex gap-3">
                        <span class="font-bold text-secondary-600 dark:text-secondary-400 shrink-0 w-6">b.</span>
                        <span>
                            <strong class="text-surface-900 dark:text-surface-100">Förderung von Transparenz und Partizipation:</strong>
                            Der Verein setzt Initiativen zur Förderung der politischen Transparenz und Partizipation auf lokaler, nationaler und internationaler Ebene um. Dabei werden Plattformen geschaffen, die Bürgerinnen und Bürgern den Zugang zu Informationen und Mitbestimmung erleichtern.
                        </span>
                    </li>
                    <li class="flex gap-3">
                        <span class="font-bold text-secondary-600 dark:text-secondary-400 shrink-0 w-6">c.</span>
                        <span>
                            <strong class="text-surface-900 dark:text-surface-100">Aufbau und Betrieb der Online-Plattform „somes“:</strong>
                            Der Verein entwickelt und pflegt eine Online-Plattform zur Förderung von politischer Transparenz, demokratischer Partizipation und des offenen Dialogs. Dabei werden Tools und Ressourcen bereitgestellt, die den Austausch von Ideen, die Organisation von Initiativen und die weltweite Vernetzung von Menschen erleichtern. Zudem wird die Plattform unter besonderer Berücksichtigung des Datenschutzes und der Barrierefreiheit für alle Nutzer gestaltet.
                        </span>
                    </li>
                    <li class="flex gap-3">
                        <span class="font-bold text-secondary-600 dark:text-secondary-400 shrink-0 w-6">d.</span>
                        <span>
                            <strong class="text-surface-900 dark:text-surface-100">Überwindung gesellschaftlicher Spaltung:</strong>
                            Der Verein führt Dialogforen durch, um soziale Blasen und gesellschaftliche Polarisierung zu verringern. Dabei wird der Austausch unterschiedlicher Perspektiven gefördert, um Verständnis und gesellschaftlichen Zusammenhalt zu stärken.
                        </span>
                    </li>
                    <li class="flex gap-3">
                        <span class="font-bold text-secondary-600 dark:text-secondary-400 shrink-0 w-6">e.</span>
                        <span>
                            <strong class="text-surface-900 dark:text-surface-100">Internationale Zusammenarbeit und Netzwerkarbeit:</strong>
                            Der Verein baut ein globales Netzwerk zur Förderung von Frieden und internationalem Zusammenhalt auf und pflegt dieses aktiv. Dabei werden Projekte unterstützt, die den interkulturellen Dialog und die Zusammenarbeit zwischen Nationen fördern.
                        </span>
                    </li>
                    <li class="flex gap-3">
                        <span class="font-bold text-secondary-600 dark:text-secondary-400 shrink-0 w-6">f.</span>
                        <span>
                            <strong class="text-surface-900 dark:text-surface-100">Einsatz für ein freies Internet:</strong>
                            Der Verein fördert den freien Zugang zu unabhängigen und qualitativ hochwertigen Informationen. Zudem setzt er sich für Open-Source, Datenschutz, Medienkompetenz und die kritische Auseinandersetzung mit Desinformationen ein.
                        </span>
                    </li>
                    <li class="flex gap-3">
                        <span class="font-bold text-secondary-600 dark:text-secondary-400 shrink-0 w-6">g.</span>
                        <span>
                            <strong class="text-surface-900 dark:text-surface-100">Investigative Aufklärungsarbeit:</strong>
                            Der Verein führt investigative Recherchen durch, um gesellschaftliche Missstände aufzudecken. Dabei werden Analysen und Daten veröffentlicht, die zur Förderung fundierter öffentlicher Diskussionen beitragen.
                        </span>
                    </li>
                    <li class="flex gap-3">
                        <span class="font-bold text-secondary-600 dark:text-secondary-400 shrink-0 w-6">h.</span>
                        <span>
                            <strong class="text-surface-900 dark:text-surface-100">Beratung und Unterstützung:</strong>
                            Der Verein berät staatliche Institutionen, Unternehmen und andere Organisationen in Übereinstimmung mit seinen Grundwerten. Zudem unterstützt er Projekte und Initiativen, die den Zielen des Vereins dienen.
                        </span>
                    </li>
                    <li class="flex gap-3">
                        <span class="font-bold text-secondary-600 dark:text-secondary-400 shrink-0 w-6">i.</span>
                        <span>
                            <strong class="text-surface-900 dark:text-surface-100">Organisation von und Beteiligung an gesellschaftlichen Initiativen:</strong>
                            Der Verein plant und unterstützt Volksbegehren, Demonstrationen und Veranstaltungen im Einklang mit seinen Werten. Darüber hinaus fördert er den Austausch zwischen Mitgliedern und Partnerorganisationen.
                        </span>
                    </li>
                    <li class="flex gap-3 bg-surface-100 dark:bg-surface-900/50 p-3 rounded-lg text-sm">
                        <span class="font-bold text-secondary-600 dark:text-secondary-400 shrink-0 w-6">j.</span>
                        <span class="italic text-surface-600 dark:text-surface-400">
                            Sofern dies dem Vereinszweck dient, ist der Verein weiters berechtigt, sich an Kapitalgesellschaften zu beteiligen, sich Erfüllungsgehilfen gemäß § 40 Abs. 1 BAO zu bedienen oder selbst als Erfüllungsgehilfe tätig zu werden, Geldmittel gemäß § 40a Z 1 BAO weiterzuleiten, entgeltliche Lieferungen gemäß § 40a Z 2 BAO ohne Gewinnerzielungsabsicht zu erbringen sowie Geldmittel gemäß § 40b BAO für Preise und Stipendien zur Verfügung zu stellen.
                        </span>
                    </li>
                </ul>
            </div>
		</section>

        <!-- ODR & Footer Info -->
        <div class="text-xs text-surface-500 border-t border-surface-200 dark:border-surface-700 pt-6 mt-8">
            Verbraucher haben die Möglichkeit, Beschwerden an die Online-Streitbeilegungsplattform der EU zu richten: 
            <a href="http://ec.europa.eu/odr" target="_blank" rel="noopener noreferrer" class="hover:text-primary-600 underline">http://ec.europa.eu/odr</a>. 
            Sie können allfällige Beschwerde auch an die oben angegebene E-Mail-Adresse richten.
        </div>
	</div>
</Container>