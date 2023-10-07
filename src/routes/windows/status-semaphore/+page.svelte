<script lang="ts">
	import type { PageData } from './$types';
	import { cssVarToRGBA } from '$lib';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy } from 'svelte';

	export let data: PageData;

	const alpha = 0.5;

	$: color = () => {
		switch (data.status) {
			case 'success':
				return cssVarToRGBA('--color-success-500', alpha);
			case 'loading':
				return cssVarToRGBA('--color-warning-500', alpha);
			default:
				return cssVarToRGBA('--color-error-500', alpha);
		}
	};

	const unListenStatusUpdate = listen('update-status', (event) => {
		data.status = event.payload;
	});

	// technically not needed since this window will be closed and never reused but it's good practice
	onDestroy(() => {
		unListenStatusUpdate();
	});
</script>

<svg class="w-[100vmin] h-[100vmin]">
	<circle cx="50%" cy="50%" r="50%" fill={color()} />
</svg>
