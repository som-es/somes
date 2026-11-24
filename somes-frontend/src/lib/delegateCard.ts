import { url } from '$lib/api/api';
import type { Parliament } from '$lib/api/parliament';
import type { Messages } from '$lib/i18n/messages/types';
import type { Delegate, FullMandate } from '$lib/types';

export function dateDiffInDays(a: Date, b: Date): number {
	const _MS_PER_DAY = 1000 * 60 * 60 * 24;
	const utc1 = Date.UTC(a.getFullYear(), a.getMonth(), a.getDate());
	const utc2 = Date.UTC(b.getFullYear(), b.getMonth(), b.getDate());

	return Math.floor((utc2 - utc1) / _MS_PER_DAY);
}

export function delegateAge(delegate: Delegate): number | null {
	if (!delegate.is_active || !delegate.birthdate) return null;
	return Math.floor(dateDiffInDays(new Date(delegate.birthdate), new Date()) / 365);
}

export function mandatesToDisplay(
	delegate: Delegate,
	date?: string
): FullMandate[] | null | undefined {
	if (date) {
		const cmpDate = new Date(date);
		return delegate.mandates?.filter((mandate) => {
			if (!mandate.start_date) {
				return false;
			}
			let startDate = new Date(mandate.start_date);
			let endDate = mandate.end_date ? new Date(mandate.end_date) : new Date();
			return cmpDate >= startDate && cmpDate <= endDate;
		});
	} else {
		return delegate.mandates_at_time;
	}
}

export function delegateImageUrl(delegate: Delegate, parliament: Parliament): string | null {
	return parliament == 'at' ? `${url}assets/${delegate.id}.jpg` : delegate.image_url;
}

export function delegatePartyLabel(
	delegate: Delegate,
	translate: (key: keyof Messages) => string
): string {
	return delegate.party == null || delegate.party == 'OK'
		? translate('delegate.withoutParty')
		: delegate.party;
}
