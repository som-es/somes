import type { Parliament } from '$lib/api/parliament';
import type { SharePersonData } from '$lib/components/ShareCards/types';
import {
	delegateAge,
	delegateImageUrl,
	delegatePartyLabel,
	mandatesToDisplay
} from '$lib/delegateCard';
import type { Messages } from '$lib/i18n/messages/types';
import { partyToColor } from '$lib/partyColor';
import type { Delegate } from '$lib/types';

const MAX_PHOTO_BYTES = 3_000_000;

async function image_data_url(src: string | null, fetcher: typeof fetch): Promise<string | null> {
	if (!src) return null;
	try {
		const response = await fetcher(src, {
			headers: { 'user-agent': 'Mozilla/5.0 (compatible; somes-share-image/1.0)' },
			signal: AbortSignal.timeout(4000)
		});
		const type = response.headers.get('content-type') ?? '';
		if (!response.ok || !/^image\/(jpeg|png)/.test(type)) return null;
		const bytes = Buffer.from(await response.arrayBuffer());
		if (bytes.length > MAX_PHOTO_BYTES) return null;
		return `data:${type.split(';')[0]};base64,${bytes.toString('base64')}`;
	} catch {
		return null;
	}
}

export async function share_person(
	delegate: Delegate,
	options: {
		parliament: Parliament;
		date?: string;
		partyColors: Map<string, string>;
		messages: Messages;
		extraLines?: string[];
		fetcher: typeof fetch;
	}
): Promise<SharePersonData> {
	const photo =
		(await image_data_url(delegateImageUrl(delegate, options.parliament), options.fetcher)) ??
		(await image_data_url(delegate.image_url, options.fetcher));

	return {
		name: delegate.name,
		age: delegateAge(delegate),
		party: delegatePartyLabel(delegate, (key) => options.messages[key]),
		partyColor: partyToColor(delegate.party, options.partyColors),
		photo,
		copyright: photo ? delegate.image_copyright : null,
		lines: [
			...(mandatesToDisplay(delegate, options.date) ?? [])
				.map((mandate) => mandate.name)
				.filter((name): name is string => !!name),
			...(options.extraLines ?? [])
		]
	};
}
