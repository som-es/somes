import type { FullMandate, PlenarySession } from './types';

const toViennaDateString = (date) => {
	return new Intl.DateTimeFormat('en-CA', {
		// 'en-CA' outputs YYYY-MM-DD
		timeZone: 'Europe/Vienna',
		year: 'numeric',
		month: '2-digit',
		day: '2-digit'
	}).format(date);
};

function couldAttendPlenarySessionByTime(
	mandates: FullMandate[],
	plenarySession: PlenarySession
): boolean {
	return (
		mandates.find((mandate) => {
			if (plenarySession.council !== mandate.function && mandate.function !== "MEP") {
				return false;
			}

			const startDate = new Date(mandate.start_date!);
			const endDate = mandate.end_date ? new Date(mandate.end_date) : new Date();
			const plenaryDate = new Date(
				toViennaDateString(new Date(plenarySession.raw_data_created_at))
			);
			return plenaryDate <= endDate && plenaryDate >= startDate;
		}) !== undefined
	);
}

export function activePlenarySessionsForDelegate(
	mandates: FullMandate[],
	plenarySessions: Record<string, PlenarySession[]>
): Record<string, PlenarySession[]> {
  console.log(mandates);
	const filteredActivePlenarySessions: Record<string, PlenarySession[]> = {};

	for (const [key, values] of Object.entries(plenarySessions)) {
		const activeSessions = values.filter((plenarySession) =>
			couldAttendPlenarySessionByTime(mandates, plenarySession)
		);
		if (activeSessions.length > 0) {
			filteredActivePlenarySessions[key] = activeSessions;
		}
	}

	return filteredActivePlenarySessions;
}
