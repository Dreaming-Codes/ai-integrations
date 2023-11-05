import { purgeCss } from 'vite-plugin-tailwind-purgecss';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [
		sveltekit(),
		// eslint-disable-next-line @typescript-eslint/ban-ts-comment
		// @ts-ignore This plugin still works but is not yet updated to vite 4.5.0
		purgeCss()
	],

	// prevent vite from obscuring rust errors
	clearScreen: false,

	// tauri expects a fixed port, fail if that port is not available
	server: {
		port: 1420,
		strictPort: true
	},

	// to make use of `TAURI_DEBUG` and other env variables
	// https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
	envPrefix: ['VITE_', 'TAURI_']
});
