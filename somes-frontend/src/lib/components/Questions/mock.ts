import type { Parliament } from '$lib/api/parliament';
import type { PoliticianQuestion } from './types';

// TODO: remove once the questions API exists. The delegate names are fictional
// on purpose so no real politician gets a made-up answer attributed to them.
const allMockQuestions: PoliticianQuestion[] = [
	{
		id: 1,
		parliament: 'at',
		askedBy: 'Sebastian L.',
		date: '2026-08-28',
		question:
			'Wie rechtfertigen Sie die geplante Besteuerung zuckerfreier Süßstoff-Getränke, wenn damit die angebliche gesundheitl. Lenkungswirkung entfällt u. es reine Einnahmeerzielung zu Lasten der Bürger wird?',
		text: 'Sehr geehrte Frau Abgeordnete,\n\nim aktuellen Regierungsprogramm ist eine Abgabe auf zuckerhaltige Getränke vorgesehen, die laut Entwurf aber auch Getränke mit Süßstoffen erfassen soll. Die gesundheitspolitische Begründung war bisher immer die Reduktion des Zuckerkonsums. Bei Getränken, die gar keinen Zucker enthalten, greift dieses Argument aber nicht.\n\nMich würde daher interessieren, welche Studien der Ausweitung auf Süßstoff-Getränke zugrunde liegen und ob die Einnahmen tatsächlich zweckgebunden in die Prävention fließen oder im allgemeinen Budget landen.\n\nMit freundlichen Grüßen\nSebastian L.',
		topics: ['Gesundheit', 'Zucker'],
		answer: {
			delegateName: 'Katharina Brunner',
			party: 'GRÜNE',
			text: 'Die geplante Abgabe zielt nicht nur auf den Zuckergehalt, sondern auf das gesamte Konsumverhalten bei stark verarbeiteten Getränken ab. Studien zeigen, dass auch Süßstoff-Getränke die Präferenz für süße Produkte verstärken. Die Einnahmen fließen zweckgebunden in Präventionsprogramme, insbesondere an Schulen. Von einer reinen Einnahmeerzielung kann daher keine Rede sein.',
			date: '2026-08-28'
		}
	},
	{
		id: 2,
		parliament: 'at',
		askedBy: 'Maria H.',
		date: '2026-08-25',
		question:
			'Warum wird der Ausbau der Kinderbetreuungsplätze im ländlichen Raum nicht schneller vorangetrieben, obwohl er seit Jahren angekündigt ist?',
		text: 'Sehr geehrter Herr Abgeordneter,\n\nich lebe mit meiner Familie in einer Gemeinde mit rund 2.000 Einwohner:innen. Für unsere zweijährige Tochter gibt es im Umkreis von 20 Kilometern keinen einzigen Betreuungsplatz mit Öffnungszeiten, die eine Vollzeitbeschäftigung beider Elternteile erlauben würden.\n\nSeit Jahren wird der Ausbau der Kinderbetreuung im ländlichen Raum angekündigt, spürbar ist davon bei uns nichts. Welche konkreten Schritte und Fristen gibt es, und woran scheitert die Umsetzung Ihrer Einschätzung nach?\n\nMit freundlichen Grüßen\nMaria H.',
		topics: ['Familie', 'Bildung'],
		answer: {
			delegateName: 'Thomas Ebner',
			party: 'ÖVP',
			text: 'Der Ausbau läuft über eine 15a-Vereinbarung mit den Ländern, die pro Jahr rund 200 Millionen Euro vorsieht. Die Umsetzung hängt stark von den Gemeinden ab, die Personal finden und Räumlichkeiten schaffen müssen. Wir arbeiten aktuell an einer Vereinfachung der Förderabwicklung, damit gerade kleine Gemeinden schneller zu Mitteln kommen.',
			date: '2026-08-26'
		}
	},
	{
		id: 3,
		parliament: 'at',
		askedBy: 'David K.',
		date: '2026-08-24',
		question:
			'Welche konkreten Maßnahmen planen Sie gegen die steigenden Mietpreise in Ballungsräumen?',
		text: 'Sehr geehrte Frau Abgeordnete,\n\nmeine Miete in Wien ist in den letzten drei Jahren um über 20 Prozent gestiegen, während mein Einkommen nur geringfügig angepasst wurde. Viele in meinem Umfeld überlegen bereits, aus der Stadt wegzuziehen.\n\nWelche konkreten Maßnahmen planen Sie gegen die steigenden Mietpreise in Ballungsräumen, und mit welchem Zeithorizont rechnen Sie, bis diese bei den Mieter:innen ankommen?\n\nMit freundlichen Grüßen\nDavid K.',
		topics: ['Wohnen', 'Soziales'],
		answer: {
			delegateName: 'Lisa Steiner',
			party: 'SPÖ',
			text: 'Wir fordern eine befristete Mietpreisbremse für den regulierten und den freien Markt sowie eine deutliche Ausweitung des gemeinnützigen Wohnbaus. Zusätzlich braucht es eine Leerstandsabgabe, wie sie einzelne Bundesländer bereits eingeführt haben, um spekulativen Leerstand zu mobilisieren.',
			date: '2026-08-27'
		}
	},
	{
		id: 4,
		parliament: 'at',
		askedBy: 'Julia P.',
		date: '2026-08-22',
		question:
			'Wie stehen Sie zur Einführung einer verpflichtenden digitalen Grundbildung ab der Volksschule?',
		text: 'Sehr geehrter Herr Abgeordneter,\n\nals Volksschullehrerin erlebe ich täglich, wie unterschiedlich die digitalen Vorkenntnisse der Kinder sind. Manche haben zu Hause Zugang zu Geräten und Unterstützung, andere gar nicht. Ohne verbindliche Grundbildung wird diese Schere im Laufe der Schulzeit größer.\n\nWie stehen Sie zur Einführung einer verpflichtenden digitalen Grundbildung ab der Volksschule, und wie soll die Ausbildung der Lehrkräfte dafür sichergestellt werden?\n\nMit freundlichen Grüßen\nJulia P.',
		topics: ['Bildung', 'Digitalisierung'],
		answer: null
	},
	{
		id: 5,
		parliament: 'at',
		askedBy: 'Markus W.',
		date: '2026-08-20',
		question:
			'Warum werden Einnahmen aus der CO2-Bepreisung nicht vollständig als Klimabonus an die Bevölkerung rückverteilt?',
		text: 'Sehr geehrte Frau Abgeordnete,\n\nbei der Einführung der CO2-Bepreisung wurde versprochen, dass die Einnahmen über den Klimabonus vollständig an die Bevölkerung zurückfließen. Aus dem aktuellen Budgetbericht geht allerdings hervor, dass ein wachsender Anteil im allgemeinen Haushalt verbleibt.\n\nWarum werden die Einnahmen nicht vollständig rückverteilt, und welche Verwendung ist für den einbehaltenen Teil vorgesehen?\n\nMit freundlichen Grüßen\nMarkus W.',
		topics: ['Klima', 'Budget'],
		answer: {
			delegateName: 'Andreas Moser',
			party: 'NEOS',
			text: 'Eine vollständige Rückverteilung wäre aus unserer Sicht der richtige Weg, weil sie die Lenkungswirkung erhält und gleichzeitig Haushalte entlastet. Derzeit versickert ein Teil der Einnahmen im allgemeinen Budget. Wir haben dazu einen Entschließungsantrag eingebracht, der eine transparente, vollständige Rückverteilung fordert.',
			date: '2026-08-21'
		}
	},
	{
		id: 6,
		parliament: 'at',
		askedBy: 'Sandra F.',
		date: '2026-08-18',
		question:
			'Welche Schritte unternehmen Sie, um Wartezeiten auf Facharzttermine für Kassenpatient:innen zu verkürzen?',
		text: 'Sehr geehrter Herr Abgeordneter,\n\nich warte seit vier Monaten auf einen Termin bei einem Kassen-Hautarzt. Als Wahlärztin wäre ein Termin innerhalb einer Woche möglich gewesen, das kann ich mir aber nicht leisten.\n\nWelche Schritte unternehmen Sie konkret, um die Wartezeiten auf Facharzttermine für Kassenpatient:innen zu verkürzen?\n\nMit freundlichen Grüßen\nSandra F.',
		topics: ['Gesundheit'],
		answer: null
	},
	{
		id: 7,
		parliament: 'eu',
		askedBy: 'Peter R.',
		date: '2026-08-26',
		question:
			'Wie wollen Sie sicherstellen, dass der AI Act Innovationen europäischer Start-ups nicht ausbremst?',
		text: 'Sehr geehrte Frau Abgeordnete,\n\nich bin Mitgründer eines kleinen Software-Start-ups in Graz. Der AI Act bringt für uns erhebliche Dokumentations- und Prüfpflichten, die wir mit unserem Team kaum stemmen können, während große Konzerne eigene Compliance-Abteilungen dafür haben.\n\nWie wollen Sie sicherstellen, dass der AI Act Innovationen europäischer Start-ups nicht ausbremst, und sind Erleichterungen für kleine Unternehmen geplant?\n\nMit freundlichen Grüßen\nPeter R.',
		topics: ['Digitalisierung', 'Wirtschaft'],
		answer: {
			delegateName: 'Elena Wagner',
			party: 'GRÜNE',
			text: 'Der AI Act sieht für kleine Unternehmen ausdrücklich Erleichterungen vor, etwa regulatorische Sandboxes, in denen Systeme unter Aufsicht getestet werden können. Wir setzen uns dafür ein, dass die Umsetzung in den Mitgliedstaaten einheitlich erfolgt, damit Start-ups nicht 27 verschiedene Auslegungen navigieren müssen.',
			date: '2026-08-27'
		}
	},
	{
		id: 8,
		parliament: 'eu',
		askedBy: 'Anna S.',
		date: '2026-08-23',
		question:
			'Warum stimmt das EU-Parlament weiterhin über den Standortwechsel zwischen Brüssel und Straßburg ab, obwohl die Mehrheit der Abgeordneten das Pendeln ablehnt?',
		text: 'Sehr geehrter Herr Abgeordneter,\n\ndas monatliche Pendeln des EU-Parlaments zwischen Brüssel und Straßburg kostet nach Angaben des Rechnungshofs über 100 Millionen Euro pro Jahr. Eine Mehrheit der Abgeordneten hat sich bereits mehrfach für einen einzigen Sitz ausgesprochen.\n\nWarum wird darüber trotzdem weiterhin abgestimmt, und welche Möglichkeiten sieht das Parlament, hier eine dauerhafte Lösung herbeizuführen?\n\nMit freundlichen Grüßen\nAnna S.',
		topics: ['Demokratie', 'Budget'],
		answer: {
			delegateName: 'Michael Berger',
			party: 'SPÖ',
			text: 'Der Sitz in Straßburg ist in den EU-Verträgen festgeschrieben und kann nur einstimmig von den Mitgliedstaaten geändert werden — das Parlament selbst hat hier kein Entscheidungsrecht. Eine Mehrheit der Abgeordneten hat sich wiederholt für einen einzigen Sitz ausgesprochen, zuletzt in einer Resolution, die ich mitgetragen habe.',
			date: '2026-08-24'
		}
	},
	{
		id: 9,
		parliament: 'eu',
		askedBy: 'Lukas T.',
		date: '2026-08-21',
		question:
			'Welche Position vertreten Sie beim geplanten Verbrenner-Aus 2035 angesichts der Debatte um E-Fuels?',
		text: 'Sehr geehrte Frau Abgeordnete,\n\nals Pendler im ländlichen Raum bin ich auf mein Auto angewiesen und beobachte die Diskussion um das Verbrenner-Aus 2035 mit Sorge. Gleichzeitig wird immer wieder über E-Fuels als Ausweg gesprochen, ohne dass klar ist, ob diese je in ausreichender Menge und zu leistbaren Preisen verfügbar sein werden.\n\nWelche Position vertreten Sie beim geplanten Verbrenner-Aus angesichts der Debatte um E-Fuels?\n\nMit freundlichen Grüßen\nLukas T.',
		topics: ['Klima', 'Verkehr'],
		answer: null
	}
];

export function mockQuestions(parliament: Parliament): PoliticianQuestion[] {
	return allMockQuestions.filter((question) => question.parliament === parliament);
}
