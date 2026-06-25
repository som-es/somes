import type { FullMandate, PlenarySession } from './types';

function couldAttendPlenarySessionByTime(
	mandates: FullMandate[],
	plenarySession: PlenarySession
): boolean {
	return (
		mandates.find((mandate) => {
			if (plenarySession.council !== mandate.function) {
				return false;
			}
			const startDate = new Date(mandate.start_date!);
			const endDate = mandate.end_date ? new Date(mandate.end_date) : new Date();
			const plenaryDate = new Date(plenarySession.raw_data_created_at);
			return plenaryDate <= endDate && plenaryDate >= startDate;
		}) !== undefined
	);
}

export function activePlenarySessionsForDelegate(
	mandates: FullMandate[],
	plenarySessions: Record<string, PlenarySession[]>
): Record<string, PlenarySession[]> {
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
