export const topicColors: Map<string, string> = new Map([
	['Sport', '#006400'], // Dunkelgrün
	['Budget und Finanzen', '#FFD700'], // Gold
	['Information und Medien', '#4B0082'], // Indigo
	['Arbeit', '#4682B4'], // Stahlblau
	['Europäische Union', '#4169E1'], // Königsblau
	['Familie und Generationen', '#B76E79'], // Roségold
	['Gesundheit und Ernährung', '#191970'], // Mitternachtsblau
	['Klima, Umwelt und Energie', '#008000'], // Smaragdgrün
	['Frauen und Gleichbehandlung', '#DCB0D0'], // Malvenfarben
	['Verkehr und Infrastruktur', '#FF8C00'], // Dunkelorange
	['Inneres und Recht', '#800000'], // Granatrot
	['Innovation, Technologie und Forschung', '#008080'], // Teal
	['Bildung', '#0F52BA'], // Saphirblau
	['Wirtschaft', '#8B008B'], // Dunkles Magenta
	['Landesverteidigung', '#556B2F'], // Olivgrün
	['Parlament und Demokratie', '#FF6F61'], // Korallenrot
	['Außenpolitik', '#800020'], // Burgunderrot
	['Land- und Forstwirtschaft', '#A0522D'], // Siena
	['Soziales', '#FFE5B4'], // Pfirsich
	['Kultur', '#8A2BE2'], // Violett

	// id_as_hash variants (i64 as exact decimal string, matching the `id_as_hash::text`
	// the API returns); same colors as the topic_name entries above
	['-1616215530511931679', '#FF6F61'], // id_as_hash: Parlament und Demokratie
	['6986394555873399515', '#FF8C00'], // id_as_hash: Verkehr und Infrastruktur
	['3689247817911674857', '#B76E79'], // id_as_hash: Familie und Generationen
	['-1304874518460135721', '#A0522D'], // id_as_hash: Land- und Forstwirtschaft
	['-8291460274263416759', '#008000'], // id_as_hash: Klima, Umwelt und Energie
	['4389424592831942243', '#008080'], // id_as_hash: Innovation, Technologie und Forschung
	['-2354184341686450276', '#8A2BE2'], // id_as_hash: Kultur
	['7146239111052806644', '#4682B4'], // id_as_hash: Arbeit
	['7034393618900042138', '#8B008B'], // id_as_hash: Wirtschaft
	['4836563141530063945', '#FFD700'], // id_as_hash: Budget und Finanzen
	['-1856946024521834068', '#FFE5B4'], // id_as_hash: Soziales
	['-7075531979603628241', '#4169E1'], // id_as_hash: Europäische Union
	['-890368908971066027', '#0F52BA'], // id_as_hash: Bildung
	['527789797658294746', '#006400'], // id_as_hash: Sport
	['1871509628119247022', '#DCB0D0'], // id_as_hash: Frauen und Gleichbehandlung
	['4542432137549833441', '#4B0082'], // id_as_hash: Information und Medien
	['-7165695291448025285', '#800020'], // id_as_hash: Außenpolitik
	['1728667647777507741', '#800000'], // id_as_hash: Inneres und Recht
	['5726904618553562468', '#556B2F'], // id_as_hash: Landesverteidigung
	['212413626333465818', '#191970'] // id_as_hash: Gesundheit und Ernährung
]);

export const TOPIC_MAP: Record<string, string> = {
	Arbeitsrecht: 'Arbeit',
	'Beschäftigung und Arbeitsbedingungen': 'Arbeit',
	Arbeitslosenversicherung: 'Arbeit',
	Gewerbeaufsicht: 'Arbeit',
	Unternehmensarten: 'Arbeit',
	'Verwaltung und Entlohnung des Personals': 'Arbeit',
	'internationales Abkommen': 'Außenpolitik',
	'Internationale Beziehungen': 'Außenpolitik',
	'diplomatische Beziehungen': 'Außenpolitik',
	'Vereinte Nationen': 'Außenpolitik',
	Völkerrecht: 'Außenpolitik',
	'völkerrechtliche Verantwortlichkeit': 'Außenpolitik',
	'Politik der Zusammenarbeit': 'Außenpolitik',
	Europarat: 'Außenpolitik',
	'Interparlamentarische Union': 'Außenpolitik',
	Hochschulausbildung: 'Bildung',
	'Organisation des Unterrichtswesens': 'Bildung',
	Bildung: 'Bildung',
	Sekundarstufe: 'Bildung',
	Primarstufe: 'Bildung',
	'vorschulische Erziehung': 'Bildung',
	Erwachsenenbildung: 'Bildung',
	Ausbildungsbeihilfe: 'Bildung',
	'Öffentliche Finanzen und Haushaltspolitik': 'Budget und Finanzen',
	Finanzwesen: 'Budget und Finanzen',
	Steuerwesen: 'Budget und Finanzen',
	Haushaltsplan: 'Budget und Finanzen',
	Haushaltskontrolle: 'Budget und Finanzen',
	Finanzausgleich: 'Budget und Finanzen',
	Nachtragshaushaltsplan: 'Budget und Finanzen',
	Einkommen: 'Budget und Finanzen',
	Buchprüfung: 'Budget und Finanzen',
	Währungsbeziehungen: 'Budget und Finanzen',
	'Europäische Union': 'Europäische Union',
	Familie: 'Familie und Generationen',
	Familienleistungsausgleich: 'Familie und Generationen',
	'junger Mensch': 'Familie und Generationen',
	Frau: 'Frauen und Gleichbehandlung',
	Gleichbehandlung: 'Frauen und Gleichbehandlung',
	Gesundheit: 'Gesundheit und Ernährung',
	Krankenpflege: 'Gesundheit und Ernährung',
	'Betreuung von Pflegebedürftigen': 'Gesundheit und Ernährung',
	Apotheke: 'Gesundheit und Ernährung',
	Altersversorgungssystem: 'Gesundheit und Ernährung',
	'soziale Sicherheit': 'Gesundheit und Ernährung',
	Tabak: 'Gesundheit und Ernährung',
	Wein: 'Gesundheit und Ernährung',
	Weinbau: 'Gesundheit und Ernährung',
	Branntwein: 'Gesundheit und Ernährung',
	Salz: 'Gesundheit und Ernährung',
	'Agrarerzeugnisse und Lebensmittel': 'Gesundheit und Ernährung',
	Fernsehen: 'Information und Medien',
	Hörfunk: 'Information und Medien',
	Presse: 'Information und Medien',
	'Post- und Fernmeldewesen': 'Information und Medien',
	Internet: 'Information und Medien',
	Telekommunikation: 'Information und Medien',
	'Information und Informationsverarbeitung': 'Information und Medien',
	Opferhilfe: 'Inneres und Recht',
	Strafrecht: 'Inneres und Recht',
	Verfassung: 'Inneres und Recht',
	Verwaltungsrecht: 'Inneres und Recht',
	'öffentliche Verwaltung': 'Inneres und Recht',
	'öffentlicher Dienst': 'Inneres und Recht',
	Exekutive: 'Inneres und Recht',
	Misstrauensantrag: 'Inneres und Recht',
	Staatsangehöriger: 'Inneres und Recht',
	'Bürgerliches Recht': 'Inneres und Recht',
	Menschenrechte: 'Inneres und Recht',
	Gerichtswesen: 'Inneres und Recht',
	Notar: 'Inneres und Recht',
	Rechtsanwalt: 'Inneres und Recht',
	'öffentliches Eigentum': 'Inneres und Recht',
	Verfassungsgerichtsbarkeit: 'Inneres und Recht',
	Verwaltungsgerichtsbarkeit: 'Inneres und Recht',
	Verwaltungsorganisation: 'Inneres und Recht',
	Verwaltungsreform: 'Inneres und Recht',
	Verwaltungskontrolle: 'Inneres und Recht',
	'Vereinfachung der Rechtsvorschriften': 'Inneres und Recht',
	Religion: 'Inneres und Recht',
	Versammlungsfreiheit: 'Inneres und Recht',
	Flüchtling: 'Inneres und Recht',
	Staatschef: 'Inneres und Recht',
	Ausweis: 'Inneres und Recht',
	Staatswald: 'Inneres und Recht',
	Inkompatibilität: 'Inneres und Recht',
	'Veröffentlichung des Gesetzes': 'Inneres und Recht',
	Neutralität: 'Inneres und Recht',
	'gesetzlicher Feiertag': 'Inneres und Recht',
	Staatssymbol: 'Inneres und Recht',
	Personenstand: 'Inneres und Recht',
	Zivilschutz: 'Inneres und Recht',
	'öffentliche Sicherheit': 'Inneres und Recht',
	Informatik: 'Innovation, Technologie und Forschung',
	'Forschung und geistiges Eigentum': 'Innovation, Technologie und Forschung',
	Wissenschaften: 'Innovation, Technologie und Forschung',
	Statistik: 'Innovation, Technologie und Forschung',
	Geodäsie: 'Innovation, Technologie und Forschung',
	'Maße und Gewichte': 'Innovation, Technologie und Forschung',
	Umwelt: 'Klima, Umwelt und Energie',
	Abfall: 'Klima, Umwelt und Energie',
	Abfallwirtschaft: 'Klima, Umwelt und Energie',
	Energie: 'Klima, Umwelt und Energie',
	Elektrizitätsindustrie: 'Klima, Umwelt und Energie',
	'Elektrizitäts- und Kernkraftindustrie': 'Klima, Umwelt und Energie',
	Wasser: 'Klima, Umwelt und Energie',
	Wasserbewirtschaftung: 'Klima, Umwelt und Energie',
	Wasserbau: 'Klima, Umwelt und Energie',
	Kulturpolitik: 'Kultur',
	Kunst: 'Kultur',
	Museum: 'Kultur',
	Bibliothek: 'Kultur',
	Filmindustrie: 'Kultur',
	'Darstellende Künste': 'Kultur',
	Vereinsleben: 'Kultur',
	'ehrende Auszeichnung': 'Kultur',
	Preis: 'Kultur',
	Verteidigung: 'Landesverteidigung',
	Zivildienst: 'Landesverteidigung',
	'Land- und Forstwirtschaft, Fischerei': 'Land- und Forstwirtschaft',
	Fischerei: 'Land- und Forstwirtschaft',
	Jagd: 'Land- und Forstwirtschaft',
	Bodenreform: 'Land- und Forstwirtschaft',
	'Geschäftsordnung des Parlaments': 'Parlament und Demokratie',
	Ausschussbericht: 'Parlament und Demokratie',
	Parlamentarier: 'Parlament und Demokratie',
	Parlamentsdebatte: 'Parlament und Demokratie',
	'parlamentarischer Ausschuss': 'Parlament und Demokratie',
	'direkt gewählte Kammer': 'Parlament und Demokratie',
	'Zweite Kammer': 'Parlament und Demokratie',
	Zweikammersystem: 'Parlament und Demokratie',
	Gesetzgebungsverfahren: 'Parlament und Demokratie',
	'partizipative Demokratie': 'Parlament und Demokratie',
	'namentliche Abstimmung': 'Parlament und Demokratie',
	Regierungserklärung: 'Parlament und Demokratie',
	'öffentliche Anhörung': 'Parlament und Demokratie',
	'parlamentarische Immunität': 'Parlament und Demokratie',
	'Zusammensetzung des Parlaments': 'Parlament und Demokratie',
	Parlamentssitzung: 'Parlament und Demokratie',
	Föderalismus: 'Parlament und Demokratie',
	Gliedstaat: 'Parlament und Demokratie',
	'Politische Partei': 'Parlament und Demokratie',
	'Mensch mit Behinderung': 'Soziales',
	Sozialpolitik: 'Soziales',
	Sport: 'Sport',
	Schienentransport: 'Verkehr und Infrastruktur',
	Wohnungspolitik: 'Verkehr und Infrastruktur',
	Straßenverkehr: 'Verkehr und Infrastruktur',
	Luftverkehr: 'Verkehr und Infrastruktur',
	'See- und Binnenschiffsverkehr': 'Verkehr und Infrastruktur',
	'Straßen- und Brückenbau': 'Verkehr und Infrastruktur',
	'Bauindustrie und öffentliches Bauwesen': 'Verkehr und Infrastruktur',
	Baupolitik: 'Verkehr und Infrastruktur',
	Verkehr: 'Verkehr und Infrastruktur',
	Raumordnung: 'Verkehr und Infrastruktur',
	Grenze: 'Verkehr und Infrastruktur',
	Tourismus: 'Verkehr und Infrastruktur',
	Handel: 'Wirtschaft',
	Industrie: 'Wirtschaft',
	Wirtschaft: 'Wirtschaft',
	Zolltarifpolitik: 'Wirtschaft',
	'Unternehmen und Wettbewerb': 'Wirtschaft',
	Versicherungswesen: 'Wirtschaft',
	Glücksspiel: 'Wirtschaft',
	Berufsverband: 'Wirtschaft',
	'Öffentlicher Sektor': 'Sonstige',
	Personalvertretung: 'Sonstige',
	Archiv: 'Sonstige',
	'Trentino-Südtirol': 'Sonstige',
	'ethnische Gruppe': 'Sonstige',
	Volkszählung: 'Sonstige',

	// id_as_hash variants of the detailed topics (i64 as exact decimal string,
	// matching the `id_as_hash::text` the API returns); values follow the name
	// mappings above; names without a mapping got a best-fit topic
	'-2416859643503345465': 'Parlament und Demokratie', // id_as_hash: Ausschussbericht
	'-8177274867228732151': 'Wirtschaft', // id_as_hash: Berufsverband
	'-7075531979603628241': 'Europäische Union', // id_as_hash: Europäische Union
	'-3776135436165669373': 'Verkehr und Infrastruktur', // id_as_hash: Bauindustrie und öffentliches Bauwesen
	'2422866852148064875': 'Budget und Finanzen', // id_as_hash: Buchprüfung
	'-5245210603404575744': 'Verkehr und Infrastruktur', // id_as_hash: Baupolitik
	'320318018696362364': 'Verkehr und Infrastruktur', // id_as_hash: Straßenverkehr
	'8152057460987354572': 'Bildung', // id_as_hash: Organisation des Unterrichtswesens
	'-7343528145087637971': 'Parlament und Demokratie', // id_as_hash: Gesetzgebungsverfahren
	'-7653391034165075892': 'Inneres und Recht', // id_as_hash: Staatsangehöriger
	'7865596608347696343': 'Kultur', // id_as_hash: Bibliothek
	'742005966613500088': 'Information und Medien', // id_as_hash: Internet
	'2744959793450980441': 'Budget und Finanzen', // id_as_hash: Öffentliche Finanzen und Haushaltspolitik
	'-1677622390783902321': 'Inneres und Recht', // id_as_hash: Zivilschutz
	'4372277134046265720': 'Land- und Forstwirtschaft', // id_as_hash: Fischerei
	'-3670086680519181276': 'Familie und Generationen', // id_as_hash: junger Mensch
	'6982542943847991285': 'Innovation, Technologie und Forschung', // id_as_hash: Wissenschaften
	'6431319086751981909': 'Parlament und Demokratie', // id_as_hash: Sitzungsperiode des Parlaments
	'-3055013730732350301': 'Sonstige', // id_as_hash: Archiv
	'-6883391205162386675': 'Inneres und Recht', // id_as_hash: Vereinfachung der Rechtsvorschriften
	'-3817959487911257704': 'Parlament und Demokratie', // id_as_hash: Regierung
	'-116124816074025154': 'Gesundheit und Ernährung', // id_as_hash: Weinbau
	'-1078985174006133461': 'Außenpolitik', // id_as_hash: internationale Beziehungen (via 'Internationale Beziehungen')
	'-1994809125483493132': 'Außenpolitik', // id_as_hash: Europarat
	'-7664496842319301745': 'Wirtschaft', // id_as_hash: Glücksspiel
	'-8569276042626501781': 'Außenpolitik', // id_as_hash: Völkerrecht
	'1006303494205308724': 'Frauen und Gleichbehandlung', // id_as_hash: Gleichbehandlung
	'-2622345082478316876': 'Klima, Umwelt und Energie', // id_as_hash: Tierschutz
	'-2704555241845934763': 'Parlament und Demokratie', // id_as_hash: Präsident des Parlaments
	'3659730917742764573': 'Gesundheit und Ernährung', // id_as_hash: soziale Sicherheit
	'-7218707742172247': 'Land- und Forstwirtschaft', // id_as_hash: Bodenreform
	'5313272245955072963': 'Gesundheit und Ernährung', // id_as_hash: Agrarerzeugnisse und Lebensmittel
	'-5936006186629512878': 'Sonstige', // id_as_hash: ethnische Gruppe
	'8427234282244277040': 'Parlament und Demokratie', // id_as_hash: parlamentarische Abstimmung
	'-2864171327909878016': 'Inneres und Recht', // id_as_hash: Gerichtswesen
	'1681720472120096337': 'Kultur', // id_as_hash: Filmindustrie
	'1222868868058750711': 'Gesundheit und Ernährung', // id_as_hash: Salz
	'2899645597729319296': 'Inneres und Recht', // id_as_hash: Veröffentlichung des Gesetzes
	'-595356618543464039': 'Inneres und Recht', // id_as_hash: Neutralität
	'8091910644820318117': 'Verkehr und Infrastruktur', // id_as_hash: Grenze
	'-7701117062108413171': 'Verkehr und Infrastruktur', // id_as_hash: Luftverkehr
	'-5900483617969481548': 'Budget und Finanzen', // id_as_hash: Finanzausgleich
	'5194004583412536708': 'Verkehr und Infrastruktur', // id_as_hash: Tourismus
	'-3213963740441117738': 'Inneres und Recht', // id_as_hash: öffentliches Eigentum
	'1947374352788577760': 'Wirtschaft', // id_as_hash: Handel
	'4850488530291683437': 'Budget und Finanzen', // id_as_hash: Währungsbeziehungen
	'7184818072075398698': 'Kultur', // id_as_hash: Kulturpolitik
	'62790260327637111': 'Kultur', // id_as_hash: Darstellende Künste
	'-5710950432640110849': 'Klima, Umwelt und Energie', // id_as_hash: Elektrizitäts- und Kernkraftindustrie
	'1313703166207930010': 'Klima, Umwelt und Energie', // id_as_hash: Elektrizitätsindustrie
	'7034393618900042138': 'Wirtschaft', // id_as_hash: Wirtschaft
	'-9086573566221205324': 'Inneres und Recht', // id_as_hash: Exekutive
	'-7620113146160683841': 'Information und Medien', // id_as_hash: Post- und Fernmeldewesen
	'-4583961258420025871': 'Inneres und Recht', // id_as_hash: Bürgerliches Recht
	'-5182849164224476980': 'Verkehr und Infrastruktur', // id_as_hash: Schienentransport
	'-6373196079974094102': 'Inneres und Recht', // id_as_hash: Menschenrechte
	'7161344963723352545': 'Parlament und Demokratie', // id_as_hash: direkt gewählte Kammer
	'-8326661781223873238': 'Inneres und Recht', // id_as_hash: Inkompatibilität
	'-8416116987315339137': 'Inneres und Recht', // id_as_hash: Misstrauensantrag
	'-1981976715547176594': 'Klima, Umwelt und Energie', // id_as_hash: Wasser
	'5834088506749885934': 'Parlament und Demokratie', // id_as_hash: Gliedstaat
	'-9061751640186802684': 'Arbeit', // id_as_hash: Arbeitslosenversicherung
	'-3363953592676656750': 'Innovation, Technologie und Forschung', // id_as_hash: Maße und Gewichte
	'-8032588695458514156': 'Gesundheit und Ernährung', // id_as_hash: Apotheke
	'7363569939516580463': 'Kultur', // id_as_hash: ehrende Auszeichnung
	'7225135931184066489': 'Parlament und Demokratie', // id_as_hash: Parlamentssitzung
	'-3989291420005008751': 'Innovation, Technologie und Forschung', // id_as_hash: Forschung und geistiges Eigentum
	'8168520211894781882': 'Inneres und Recht', // id_as_hash: Rechtsanwalt
	'8965469848375981534': 'Kultur', // id_as_hash: Preis
	'4975437568713295339': 'Verkehr und Infrastruktur', // id_as_hash: See- und Binnenschiffsverkehr
	'1426499152624211049': 'Verkehr und Infrastruktur', // id_as_hash: Verkehr
	'2937662328754414275': 'Gesundheit und Ernährung', // id_as_hash: Tiermedizin
	'-5937704826975998642': 'Budget und Finanzen', // id_as_hash: Geld- und Kreditwesen
	'-6274823893128344577': 'Klima, Umwelt und Energie', // id_as_hash: Abfallwirtschaft
	'-5202541122163749724': 'Arbeit', // id_as_hash: Beschäftigung und Arbeitsbedingungen
	'-1974366246263773758': 'Inneres und Recht', // id_as_hash: Staatssymbol
	'-455929572263538449': 'Inneres und Recht', // id_as_hash: Opferhilfe
	'-2414944111775596189': 'Innovation, Technologie und Forschung', // id_as_hash: Statistik
	'6983233823520313738': 'Arbeit', // id_as_hash: Arbeitsrecht
	'-7455782182013283271': 'Parlament und Demokratie', // id_as_hash: Zweikammersystem
	'6184450518414462050': 'Innovation, Technologie und Forschung', // id_as_hash: Geodäsie
	'6774125867518735394': 'Inneres und Recht', // id_as_hash: Versammlungsfreiheit
	'-6426876607954080711': 'Inneres und Recht', // id_as_hash: Notar
	'-2771275842606020551': 'Information und Medien', // id_as_hash: Presse
	'-2735796747774276722': 'Familie und Generationen', // id_as_hash: Familienleistungsausgleich
	'-183526430877240779': 'Wirtschaft', // id_as_hash: Bergbau
	'-4295632185823109921': 'Wirtschaft', // id_as_hash: Industrie
	'7688408294365284429': 'Inneres und Recht', // id_as_hash: Staatswald
	'-6979859215591671927': 'Bildung', // id_as_hash: vorschulische Erziehung
	'-4025656386112317217': 'Inneres und Recht', // id_as_hash: Verfassung
	'4375162434286597551': 'Verkehr und Infrastruktur', // id_as_hash: Straßen- und Brückenbau
	'-5278688163404529754': 'Außenpolitik', // id_as_hash: Politik der Zusammenarbeit
	'1944341025458209190': 'Bildung', // id_as_hash: Primarstufe
	'-7651660619271344345': 'Budget und Finanzen', // id_as_hash: Einkommen
	'6976104934999714976': 'Soziales', // id_as_hash: Sozialpolitik
	'3964922080801773262': 'Inneres und Recht', // id_as_hash: öffentliche Verwaltung
	'-6090712770891158986': 'Klima, Umwelt und Energie', // id_as_hash: Energie
	'4481012622188300445': 'Familie und Generationen', // id_as_hash: Familie
	'3453000362151218163': 'Arbeit', // id_as_hash: internationales Arbeitsrecht
	'-3293010323441518190': 'Gesundheit und Ernährung', // id_as_hash: Wein
	'-3719330512430139221': 'Klima, Umwelt und Energie', // id_as_hash: Wasserbewirtschaftung
	'527789797658294746': 'Sport', // id_as_hash: Sport
	'-6149516028667222463': 'Wirtschaft', // id_as_hash: Zolltarifpolitik
	'2242621420767961475': 'Parlament und Demokratie', // id_as_hash: parlamentarischer Ausschuss
	'254348809747893415': 'Wirtschaft', // id_as_hash: Versicherungswesen
	'4009034585095812735': 'Inneres und Recht', // id_as_hash: Staatschef
	'-3886592482787699028': 'Parlament und Demokratie', // id_as_hash: Zweite Kammer
	'-3317863863359586575': 'Budget und Finanzen', // id_as_hash: Haushaltskontrolle
	'-1305805635105607048': 'Landesverteidigung', // id_as_hash: Verteidigung
	'2141648060898698623': 'Klima, Umwelt und Energie', // id_as_hash: Umwelt
	'3545442996399934699': 'Landesverteidigung', // id_as_hash: Zivildienst
	'6838196640260527284': 'Klima, Umwelt und Energie', // id_as_hash: Abfall
	'69116138432549588': 'Kultur', // id_as_hash: Museum
	'-1863692512052724378': 'Inneres und Recht', // id_as_hash: Verwaltungskontrolle
	'-7114247078293199078': 'Inneres und Recht', // id_as_hash: öffentlicher Dienst
	'-890368908971066027': 'Bildung', // id_as_hash: Bildung
	'7155699464014693298': 'Bildung', // id_as_hash: Hochschulausbildung
	'3424458251116917432': 'Außenpolitik', // id_as_hash: Interparlamentarische Union
	'7429785989783386291': 'Außenpolitik', // id_as_hash: Vereinte Nationen
	'5770646648013719315': 'Parlament und Demokratie', // id_as_hash: namentliche Abstimmung
	'8927046160473311446': 'Bildung', // id_as_hash: Ausbildungsbeihilfe
	'8902882027586439350': 'Inneres und Recht', // id_as_hash: Verwaltungsorganisation
	'-1565943284140194530': 'Information und Medien', // id_as_hash: Fernsehen
	'-6621792963585673385': 'Inneres und Recht', // id_as_hash: Verwaltungsrecht
	'4024251996086516778': 'Inneres und Recht', // id_as_hash: Strafrecht
	'6331919302994094140': 'Parlament und Demokratie', // id_as_hash: Föderalismus
	'-2543237106388231628': 'Inneres und Recht', // id_as_hash: Verwaltungsgerichtsbarkeit
	'-3328476602196531182': 'Außenpolitik', // id_as_hash: völkerrechtliche Verantwortlichkeit
	'5018258273786059224': 'Information und Medien', // id_as_hash: Information und Informationsverarbeitung
	'-9216073347674499817': 'Arbeit', // id_as_hash: Gewerbeaufsicht
	'-7115595120885569941': 'Budget und Finanzen', // id_as_hash: Haushaltsplan
	'7710840363453707798': 'Außenpolitik', // id_as_hash: internationales Abkommen
	'-1977202168032730236': 'Information und Medien', // id_as_hash: Hörfunk
	'-8772079197011205591': 'Arbeit', // id_as_hash: Verwaltung und Entlohnung des Personals
	'-2444633175659864198': 'Klima, Umwelt und Energie', // id_as_hash: Wasserbau
	'8532541971031656114': 'Sonstige', // id_as_hash: Volkszählung
	'648728568305055856': 'Budget und Finanzen', // id_as_hash: Nachtragshaushaltsplan
	'5460797139956906874': 'Inneres und Recht', // id_as_hash: Flüchtling
	'4711462390894029880': 'Parlament und Demokratie', // id_as_hash: Regierungserklärung
	'3999076644398115805': 'Innovation, Technologie und Forschung', // id_as_hash: Informatik
	'7959560264881830005': 'Gesundheit und Ernährung', // id_as_hash: Betreuung von Pflegebedürftigen
	'7278843018244654149': 'Bildung', // id_as_hash: Sekundarstufe
	'111016712374050048': 'Parlament und Demokratie', // id_as_hash: Parlamentsdebatte
	'-4288842651174487062': 'Gesundheit und Ernährung', // id_as_hash: Gesundheit
	'-7602984788717469093': 'Parlament und Demokratie', // id_as_hash: Zusammensetzung des Parlaments
	'-6949522808114763026': 'Verkehr und Infrastruktur', // id_as_hash: Raumordnung
	'7600111061299103260': 'Inneres und Recht', // id_as_hash: Verwaltungsreform
	'-555435688416101487': 'Außenpolitik', // id_as_hash: diplomatische Beziehungen
	'6027935914417308387': 'Land- und Forstwirtschaft', // id_as_hash: Land- und Forstwirtschaft, Fischerei
	'-5705135093312096793': 'Parlament und Demokratie', // id_as_hash: öffentliche Anhörung
	'-4617695394434095903': 'Gesundheit und Ernährung', // id_as_hash: Branntwein
	'6132222316441031369': 'Inneres und Recht', // id_as_hash: Religion
	'4621414729942456846': 'Gesundheit und Ernährung', // id_as_hash: Altersversorgungssystem
	'-7786899788644249111': 'Inneres und Recht', // id_as_hash: Ausweis
	'-6852355963475205621': 'Parlament und Demokratie', // id_as_hash: partizipative Demokratie
	'4241797552780000656': 'Parlament und Demokratie', // id_as_hash: Parlamentarier
	'-5990572700364036550': 'Parlament und Demokratie', // id_as_hash: parlamentarische Immunität
	'6422361109257218465': 'Inneres und Recht', // id_as_hash: Personenstand
	'7160514807738546123': 'Wirtschaft', // id_as_hash: Unternehmen und Wettbewerb
	'-8695317860929342101': 'Sonstige', // id_as_hash: Öffentlicher Sektor
	'2500629409989177181': 'Inneres und Recht', // id_as_hash: öffentliche Sicherheit
	'-6222992519720322350': 'Sonstige', // id_as_hash: Personalvertretung
	'7029010661597543710': 'Budget und Finanzen', // id_as_hash: Finanzwesen
	'-266236295296057220': 'Inneres und Recht', // id_as_hash: gesetzlicher Feiertag
	'-2085224649643040460': 'Verkehr und Infrastruktur', // id_as_hash: Wohnungspolitik
	'-8873618668744327483': 'Gesundheit und Ernährung', // id_as_hash: Tabak
	'-6777097561206669794': 'Parlament und Demokratie', // id_as_hash: Politische Partei
	'6331449645262880303': 'Budget und Finanzen', // id_as_hash: Steuerwesen
	'-3599954878603622863': 'Land- und Forstwirtschaft', // id_as_hash: Jagd
	'2744312002006138239': 'Budget und Finanzen', // id_as_hash: Geldwirtschaft
	'507220098538370992': 'Gesundheit und Ernährung', // id_as_hash: Care-Ökonomie
	'-7232608210204097488': 'Parlament und Demokratie', // id_as_hash: Wahl
	'-24840039544047750': 'Kultur', // id_as_hash: Kunst
	'-7045682888451267541': 'Inneres und Recht', // id_as_hash: Verfassungsgerichtsbarkeit
	'3772341195404358187': 'Sonstige', // id_as_hash: Trentino-Südtirol
	'-1832200415242258459': 'Inneres und Recht', // id_as_hash: ausländischer Staatsangehöriger
	'-4179037506815471353': 'Soziales', // id_as_hash: Mensch mit Behinderung
	'-3398453471298443115': 'Frauen und Gleichbehandlung', // id_as_hash: Frau
	'4288636036476167810': 'Gesundheit und Ernährung', // id_as_hash: Krankenpflege
	'503182838938588854': 'Bildung', // id_as_hash: Erwachsenenbildung
	'-692722167548940723': 'Parlament und Demokratie', // id_as_hash: Geschäftsordnung des Parlaments
	'-7508965499761858562': 'Information und Medien', // id_as_hash: Telekommunikation
	'-5737465716010798577': 'Arbeit', // id_as_hash: Unternehmensarten
	'5270190025130252715': 'Kultur' // id_as_hash: Vereinsleben
};

export function translateTopicToParent(complexTopic: string): string {
	return TOPIC_MAP[complexTopic] ?? 'Sonstige';
}
