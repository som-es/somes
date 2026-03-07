import type { Delegate, FullMandate, LegisPeriod, Mandate } from "$lib/types";

export function findPeriodForDate(date: Date, periods: LegisPeriod[]): string | null {
    for (let i = 0; i < periods.length; i++) {
        const periodStart = new Date(periods[i].start_date);
        const periodEnd = periods[i + 1] ? new Date(periods[i + 1].start_date) : new Date();

        if (date >= periodStart && date < periodEnd) {
            return periods[i].gp;
        }
    }
    return null;
}

export function getNrMandateSkippingDateRange(mandates: FullMandate[]) {
    let firstDate: Date | null = null;
    let lastDate: Date | null = null;

    mandates?.forEach((mandate) => {
        if (!mandate.is_nr && !mandate.is_gov_official) {
            return
        }
        if (mandate.start_date) {
            const startDate = new Date(mandate.start_date);
            if (!firstDate || startDate < firstDate) {
                firstDate = startDate;
            }
        }
        if (mandate.end_date) {
            const endDate = new Date(mandate.end_date);
            if (!lastDate || endDate > lastDate) {
                lastDate = endDate;
            }
        }
    });

    return { firstDate, lastDate };
}



export function getMandateLatestPeriod(delegate: Delegate, periods: LegisPeriod[]) {
    const latestPeriod = periods[periods.length - 1];
    const fallbackGp = latestPeriod?.gp || 'XXVIII';
    const fallbackDate = new Date();

    if (!delegate.mandates || !delegate.active_mandates) {
        return { date: fallbackDate, gp: fallbackGp };
    }

    if (delegate.active_mandates.length > 0) {
        return { date: fallbackDate, gp: fallbackGp };
    }

    const { lastDate } = getNrMandateSkippingDateRange(delegate.mandates);

    if (lastDate) {
        const foundGp = findPeriodForDate(lastDate, periods);
        return {
            date: lastDate,
            gp: foundGp || fallbackGp
        };
    }

    return {
        date: fallbackDate,
        gp: fallbackGp
    };
}

export function getMandatePeriods(delegate: Delegate | null, periods: LegisPeriod[], govMandates: boolean): string {
    if (!delegate || !delegate.mandates || !delegate.active_mandates || delegate.mandates.length === 0) {
        return 'unbekannt';
    }

    // e.g. party switches during gp or ministry changes would result in the same start and end date combination
    const combinationAlreadySeen = new Set<string>();

    // GP-skipping mandates always have a continous start and end date
    const mandatePeriods = delegate.mandates.filter(mandate => {
        return govMandates ? mandate.is_gov_official : mandate.is_nr;
    }).map(mandate => {
        const startPeriod = mandate.start_date ? findPeriodForDate(new Date(mandate.start_date), periods) : null;
        const endPeriod = mandate.end_date ? findPeriodForDate(new Date(mandate.end_date), periods) : "dato";
        if (!startPeriod && !endPeriod) {
            return null
        }

        if (!startPeriod) return `unbekannt - ${endPeriod || 'unbekannt'}`;
        return `${startPeriod} - ${endPeriod || 'unbekannt'}`;
    }).filter(val => {
        if (!val) {
            return false
        }
        if (combinationAlreadySeen.has(val)) {
            return false
        }
        combinationAlreadySeen.add(val);
        return true;
    });

    return mandatePeriods.join(", ")
}
