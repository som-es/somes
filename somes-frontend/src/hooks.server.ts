import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const theme = event.cookies.get('theme');
	event.locals.theme = theme as 'light' | 'dark';

	const response = await resolve(event, {
		transformPageChunk: ({ html }) => 
			html.replace('class=""', `class="${theme}"`)
	});

	return response;
};