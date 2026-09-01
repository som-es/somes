import type { Parliament } from '$lib/api/parliament';
import type { DelegateQuestionView, QuestionDelegate } from './types';

// TODO: remove once the delegate questions API is live (see data.ts). The data
// mirrors exactly what the API will return (`PublicDelegateQuestion` plus the
// delegate the question was addressed to). The delegate names are fictional on
// purpose so no real politician gets a made-up answer attributed to them.

const brunner: QuestionDelegate = { id: 900001, name: 'Katharina Brunner', party: 'GRÜNE' };
const ebner: QuestionDelegate = { id: 900002, name: 'Thomas Ebner', party: 'ÖVP' };
const steiner: QuestionDelegate = { id: 900003, name: 'Lisa Steiner', party: 'SPÖ' };
const moser: QuestionDelegate = { id: 900004, name: 'Andreas Moser', party: 'NEOS' };
const wagner: QuestionDelegate = { id: 900005, name: 'Elena Wagner', party: 'GRÜNE' };
const berger: QuestionDelegate = { id: 900006, name: 'Michael Berger', party: 'SPÖ' };

const atMockEntries: DelegateQuestionView[] = [
	{
		delegate: brunner,
		question: {
			delegate_id: brunner.id,
			subject:
				'Wie rechtfertigen Sie die geplante Besteuerung zuckerfreier Süßstoff-Getränke, wenn damit die angebliche gesundheitl. Lenkungswirkung entfällt u. es reine Einnahmeerzielung zu Lasten der Bürger wird?',
			body: 'Sehr geehrte Frau Abgeordnete,\n\nim aktuellen Regierungsprogramm ist eine Abgabe auf zuckerhaltige Getränke vorgesehen, die laut Entwurf aber auch Getränke mit Süßstoffen erfassen soll. Die gesundheitspolitische Begründung war bisher immer die Reduktion des Zuckerkonsums. Bei Getränken, die gar keinen Zucker enthalten, greift dieses Argument aber nicht.\n\nMich würde daher interessieren, welche Studien der Ausweitung auf Süßstoff-Getränke zugrunde liegen und ob die Einnahmen tatsächlich zweckgebunden in die Prävention fließen oder im allgemeinen Budget landen.\n\nMit freundlichen Grüßen\nSebastian L.',
			created_at: '2026-08-28T09:15:00Z',
			answers: [
				{
					body: 'Die geplante Abgabe zielt nicht nur auf den Zuckergehalt, sondern auf das gesamte Konsumverhalten bei stark verarbeiteten Getränken ab. Studien zeigen, dass auch Süßstoff-Getränke die Präferenz für süße Produkte verstärken. Die Einnahmen fließen zweckgebunden in Präventionsprogramme, insbesondere an Schulen. Von einer reinen Einnahmeerzielung kann daher keine Rede sein.',
					received_at: '2026-08-28T16:40:00Z'
				}
			]
		}
	},
	{
		delegate: ebner,
		question: {
			delegate_id: ebner.id,
			subject:
				'Warum wird der Ausbau der Kinderbetreuungsplätze im ländlichen Raum nicht schneller vorangetrieben, obwohl er seit Jahren angekündigt ist?',
			body: 'Sehr geehrter Herr Abgeordneter,\n\nich lebe mit meiner Familie in einer Gemeinde mit rund 2.000 Einwohner:innen. Für unsere zweijährige Tochter gibt es im Umkreis von 20 Kilometern keinen einzigen Betreuungsplatz mit Öffnungszeiten, die eine Vollzeitbeschäftigung beider Elternteile erlauben würden.\n\nSeit Jahren wird der Ausbau der Kinderbetreuung im ländlichen Raum angekündigt, spürbar ist davon bei uns nichts. Welche konkreten Schritte und Fristen gibt es, und woran scheitert die Umsetzung Ihrer Einschätzung nach?\n\nMit freundlichen Grüßen\nMaria H.',
			created_at: '2026-08-25T11:30:00Z',
			answers: [
				{
					body: 'Der Ausbau läuft über eine 15a-Vereinbarung mit den Ländern, die pro Jahr rund 200 Millionen Euro vorsieht. Die Umsetzung hängt stark von den Gemeinden ab, die Personal finden und Räumlichkeiten schaffen müssen. Wir arbeiten aktuell an einer Vereinfachung der Förderabwicklung, damit gerade kleine Gemeinden schneller zu Mitteln kommen.',
					received_at: '2026-08-26T08:05:00Z'
				}
			]
		}
	},
	{
		delegate: steiner,
		question: {
			delegate_id: steiner.id,
			subject:
				'Welche konkreten Maßnahmen planen Sie gegen die steigenden Mietpreise in Ballungsräumen?',
			body: 'Sehr geehrte Frau Abgeordnete,\n\nmeine Miete in Wien ist in den letzten drei Jahren um über 20 Prozent gestiegen, während mein Einkommen nur geringfügig angepasst wurde. Viele in meinem Umfeld überlegen bereits, aus der Stadt wegzuziehen.\n\nWelche konkreten Maßnahmen planen Sie gegen die steigenden Mietpreise in Ballungsräumen, und mit welchem Zeithorizont rechnen Sie, bis diese bei den Mieter:innen ankommen?\n\nMit freundlichen Grüßen\nDavid K.',
			created_at: '2026-08-24T14:00:00Z',
			answers: [
				{
					body: 'Wir fordern eine befristete Mietpreisbremse für den regulierten und den freien Markt sowie eine deutliche Ausweitung des gemeinnützigen Wohnbaus. Zusätzlich braucht es eine Leerstandsabgabe, wie sie einzelne Bundesländer bereits eingeführt haben, um spekulativen Leerstand zu mobilisieren.',
					received_at: '2026-08-27T10:20:00Z'
				}
			]
		}
	},
	{
		delegate: ebner,
		question: {
			delegate_id: ebner.id,
			subject:
				'Wie stehen Sie zur Einführung einer verpflichtenden digitalen Grundbildung ab der Volksschule?',
			body: 'Sehr geehrter Herr Abgeordneter,\n\nals Volksschullehrerin erlebe ich täglich, wie unterschiedlich die digitalen Vorkenntnisse der Kinder sind. Manche haben zu Hause Zugang zu Geräten und Unterstützung, andere gar nicht. Ohne verbindliche Grundbildung wird diese Schere im Laufe der Schulzeit größer.\n\nWie stehen Sie zur Einführung einer verpflichtenden digitalen Grundbildung ab der Volksschule, und wie soll die Ausbildung der Lehrkräfte dafür sichergestellt werden?\n\nMit freundlichen Grüßen\nJulia P.',
			created_at: '2026-08-22T08:45:00Z',
			answers: []
		}
	},
	{
		delegate: moser,
		question: {
			delegate_id: moser.id,
			subject:
				'Warum werden Einnahmen aus der CO2-Bepreisung nicht vollständig als Klimabonus an die Bevölkerung rückverteilt?',
			body: 'Sehr geehrte Frau Abgeordnete,\n\nbei der Einführung der CO2-Bepreisung wurde versprochen, dass die Einnahmen über den Klimabonus vollständig an die Bevölkerung zurückfließen. Aus dem aktuellen Budgetbericht geht allerdings hervor, dass ein wachsender Anteil im allgemeinen Haushalt verbleibt.\n\nWarum werden die Einnahmen nicht vollständig rückverteilt, und welche Verwendung ist für den einbehaltenen Teil vorgesehen?\n\nMit freundlichen Grüßen\nMarkus W.',
			created_at: '2026-08-20T15:10:00Z',
			answers: [
				{
					body: 'Eine vollständige Rückverteilung wäre aus unserer Sicht der richtige Weg, weil sie die Lenkungswirkung erhält und gleichzeitig Haushalte entlastet. Derzeit versickert ein Teil der Einnahmen im allgemeinen Budget. Wir haben dazu einen Entschließungsantrag eingebracht, der eine transparente, vollständige Rückverteilung fordert.',
					received_at: '2026-08-21T09:00:00Z'
				}
			]
		}
	},
	{
		delegate: brunner,
		question: {
			delegate_id: brunner.id,
			subject:
				'Welche Schritte unternehmen Sie, um Wartezeiten auf Facharzttermine für Kassenpatient:innen zu verkürzen?',
			body: 'Sehr geehrter Herr Abgeordneter,\n\nich warte seit vier Monaten auf einen Termin bei einem Kassen-Hautarzt. Als Wahlärztin wäre ein Termin innerhalb einer Woche möglich gewesen, das kann ich mir aber nicht leisten.\n\nWelche Schritte unternehmen Sie konkret, um die Wartezeiten auf Facharzttermine für Kassenpatient:innen zu verkürzen?\n\nMit freundlichen Grüßen\nSandra F.',
			created_at: '2026-08-18T12:25:00Z',
			answers: []
		}
	}
];

const euMockEntries: DelegateQuestionView[] = [
	{
		delegate: wagner,
		question: {
			delegate_id: wagner.id,
			subject:
				'Wie wollen Sie sicherstellen, dass der AI Act Innovationen europäischer Start-ups nicht ausbremst?',
			body: 'Sehr geehrte Frau Abgeordnete,\n\nich bin Mitgründer eines kleinen Software-Start-ups in Graz. Der AI Act bringt für uns erhebliche Dokumentations- und Prüfpflichten, die wir mit unserem Team kaum stemmen können, während große Konzerne eigene Compliance-Abteilungen dafür haben.\n\nWie wollen Sie sicherstellen, dass der AI Act Innovationen europäischer Start-ups nicht ausbremst, und sind Erleichterungen für kleine Unternehmen geplant?\n\nMit freundlichen Grüßen\nPeter R.',
			created_at: '2026-08-26T10:50:00Z',
			answers: [
				{
					body: 'Der AI Act sieht für kleine Unternehmen ausdrücklich Erleichterungen vor, etwa regulatorische Sandboxes, in denen Systeme unter Aufsicht getestet werden können. Wir setzen uns dafür ein, dass die Umsetzung in den Mitgliedstaaten einheitlich erfolgt, damit Start-ups nicht 27 verschiedene Auslegungen navigieren müssen.',
					received_at: '2026-08-27T13:35:00Z'
				}
			]
		}
	},
	{
		delegate: berger,
		question: {
			delegate_id: berger.id,
			subject:
				'Warum stimmt das EU-Parlament weiterhin über den Standortwechsel zwischen Brüssel und Straßburg ab, obwohl die Mehrheit der Abgeordneten das Pendeln ablehnt?',
			body: 'Sehr geehrter Herr Abgeordneter,\n\ndas monatliche Pendeln des EU-Parlaments zwischen Brüssel und Straßburg kostet nach Angaben des Rechnungshofs über 100 Millionen Euro pro Jahr. Eine Mehrheit der Abgeordneten hat sich bereits mehrfach für einen einzigen Sitz ausgesprochen.\n\nWarum wird darüber trotzdem weiterhin abgestimmt, und welche Möglichkeiten sieht das Parlament, hier eine dauerhafte Lösung herbeizuführen?\n\nMit freundlichen Grüßen\nAnna S.',
			created_at: '2026-08-23T09:40:00Z',
			answers: [
				{
					body: 'Der Sitz in Straßburg ist in den EU-Verträgen festgeschrieben und kann nur einstimmig von den Mitgliedstaaten geändert werden — das Parlament selbst hat hier kein Entscheidungsrecht. Eine Mehrheit der Abgeordneten hat sich wiederholt für einen einzigen Sitz ausgesprochen, zuletzt in einer Resolution, die ich mitgetragen habe.',
					received_at: '2026-08-24T11:15:00Z'
				}
			]
		}
	},
	{
		delegate: wagner,
		question: {
			delegate_id: wagner.id,
			subject:
				'Welche Position vertreten Sie beim geplanten Verbrenner-Aus 2035 angesichts der Debatte um E-Fuels?',
			body: 'Sehr geehrte Frau Abgeordnete,\n\nals Pendler im ländlichen Raum bin ich auf mein Auto angewiesen und beobachte die Diskussion um das Verbrenner-Aus 2035 mit Sorge. Gleichzeitig wird immer wieder über E-Fuels als Ausweg gesprochen, ohne dass klar ist, ob diese je in ausreichender Menge und zu leistbaren Preisen verfügbar sein werden.\n\nWelche Position vertreten Sie beim geplanten Verbrenner-Aus angesichts der Debatte um E-Fuels?\n\nMit freundlichen Grüßen\nLukas T.',
			created_at: '2026-08-21T16:05:00Z',
			answers: []
		}
	}
];

export function mockQuestionEntries(parliament: Parliament): DelegateQuestionView[] {
	return parliament === 'eu' ? euMockEntries : atMockEntries;
}
