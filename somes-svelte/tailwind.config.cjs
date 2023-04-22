/** @type {import('tailwindcss').Config} */
module.exports = {
	mode: 'jit',
	purge: ['./src/**/*.{html,js,svelte,ts}', require('path').join(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,js,svelte,ts}')],
	darkMode: 'class',
	content: ['./src/**/*.{html,js,svelte,ts}', require('path').join(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,js,svelte,ts}')],
	theme: {
		extend: {},
	},
	plugins: [...require('@skeletonlabs/skeleton/tailwind/skeleton.cjs')()],
}
