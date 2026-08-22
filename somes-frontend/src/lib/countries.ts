/**  maps countries to 3digit short nmae **/
const COUNTRY_NAMES: Record<string, string> = {
	AUT: 'Österreich',
	BEL: 'Belgien',
	BGR: 'Bulgarien',
	CYP: 'Zypern',
	CZE: 'Tschechien',
	DEU: 'Deutschland',
	DNK: 'Dänemark',
	ESP: 'Spanien',
	EST: 'Estland',
	FIN: 'Finnland',
	FRA: 'Frankreich',
	GRC: 'Griechenland',
	HRV: 'Kroatien',
	HUN: 'Ungarn',
	IRL: 'Irland',
	ITA: 'Italien',
	LTU: 'Litauen',
	LUX: 'Luxemburg',
	LVA: 'Lettland',
	MLT: 'Malta',
	NLD: 'Niederlande',
	POL: 'Polen',
	PRT: 'Portugal',
	ROU: 'Rumänien',
	SVK: 'Slowakei',
	SVN: 'Slowenien',
	SWE: 'Schweden'
};

export function countryName(alpha3Code: string): string {
	return COUNTRY_NAMES[alpha3Code] ?? alpha3Code;
}
