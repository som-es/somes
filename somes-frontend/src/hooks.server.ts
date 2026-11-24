import type { Handle } from '@sveltejs/kit';
import { redirect_location } from '$lib/seo/redirects';
import { with_open_graph } from '$lib/seo/opengraph';

export const handle: Handle = async ({ event, resolve }) => {
	const host = event.request.headers.get('x-forwarded-host') ?? event.request.headers.get('host');
	const location = redirect_location(event.url, host);
	if (location) {
		return new Response(null, { status: 301, headers: { location } });
	}

	const theme = event.cookies.get('theme');
	event.locals.theme = theme as 'light' | 'dark';

	const localeCookie = event.cookies.get('locale');
	event.locals.locale = localeCookie === 'de' || localeCookie === 'en' ? localeCookie : null;
	const lang = event.locals.locale ?? (event.url.pathname.startsWith('/eu') ? 'en' : 'de');

	const response = await resolve(event, {
		transformPageChunk: ({ html }) =>
			with_open_graph(
				html.replace('class=""', `class="${theme}"`).replace('lang="en"', `lang="${lang}"`)
			)
	});

	return response;
};
