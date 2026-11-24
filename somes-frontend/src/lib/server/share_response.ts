export function png_response(png: Uint8Array): Response {
	return new Response(Buffer.from(png), {
		headers: {
			'content-type': 'image/png',
			'cache-control': 'public, max-age=86400'
		}
	});
}
