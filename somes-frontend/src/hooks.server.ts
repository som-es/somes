import type { Handle } from '@sveltejs/kit';
import { redirect_location } from '$lib/seo/redirects';

export const handle: Handle = async ({ event, resolve }) => {
	const host = event.request.headers.get('x-forwarded-host') ?? event.request.headers.get('host');
	const location = redirect_location(event.url, host);
	if (location) {
		return new Response(null, { status: 301, headers: { location } });
	}

	const theme = event.cookies.get('theme');
	event.locals.theme = theme as 'light' | 'dark';

	const response = await resolve(event, {
		transformPageChunk: ({ html }) => html.replace('class=""', `class="${theme}"`)
	});

	return response;
};
