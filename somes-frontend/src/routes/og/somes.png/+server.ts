import type { RequestHandler } from './$types';
import { de } from '$lib/i18n/messages/de';
import { render_share_image, SHARE_ICONS, svgDataUrl } from '$lib/server/share_image';
import { seat_layout_svg } from '$lib/server/parliament_svg';
import { png_response } from '$lib/server/share_response';
import SiteShareCard from '$lib/components/ShareCards/SiteShareCard.svelte';

function site_image(): Promise<Uint8Array> {
	const layout = seat_layout_svg('#c3cdd9', 0.16);
	const scale = layout ? 760 / layout.width : 1;
	return render_share_image(SiteShareCard, {
		logo: SHARE_ICONS.logo,
		seats: layout
			? {
					src: svgDataUrl(layout.svg),
					width: Math.round(layout.width * scale),
					height: Math.round(layout.height * scale)
				}
			: null,
		headline: de['share.site.headline'],
		tagline: de['layout.tagline']
	});
}

let image: Promise<Uint8Array> | null = null;

export const GET: RequestHandler = async () => {
	image ??= site_image();
	return png_response(await image);
};
