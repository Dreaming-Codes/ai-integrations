<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { emit } from '@tauri-apps/api/event';
	import { EyeIcon } from 'lucide-svelte';
	import { cssVarToRGBA } from '$lib';

	let canvas: HTMLCanvasElement;
	let context: CanvasRenderingContext2D;

	let width = window.innerWidth;
	let height = window.innerHeight;

	onMount(() => {
		context = canvas.getContext('2d')!;

		// Set color options
		context.lineWidth = 2;
		context.strokeStyle = cssVarToRGBA('--color-primary-500', 1);
		context.fillStyle = cssVarToRGBA('--color-surface-500', 0.3);
	});

	let isDrawing = false;
	let hintVisible = true;
	let start: { x: number; y: number };

	function onMouseDown({ offsetX: x, offsetY: y }: MouseEvent) {
		start = { x, y };
		isDrawing = true;
	}

	function onMouseMove({ offsetX: x, offsetY: y }: MouseEvent) {
		if (!isDrawing) return;

		context.clearRect(0, 0, canvas.width, canvas.height);
		context.beginPath();
		context.rect(start.x, start.y, x - start.x, y - start.y);
		context.stroke();
		context.fill();
	}

	async function onMouseUp({ offsetX: endX, offsetY: endY }: MouseEvent) {
		isDrawing = false;
		context.clearRect(0, 0, canvas.width, canvas.height);
		hintVisible = false;

		// Wait for the next tick to emit the selection event so that the canvas is actually cleared
		await tick();

		// For some reason, the await tick() doesn't always work, so we add a timeout as well to make sure the content has been cleared
		setTimeout(() => {
			emit('selection', {
				startX: start.x,
				startY: start.y,
				endX,
				endY
			});
		}, 50);
	}

	function onMouseLeave() {
		isDrawing = false;
		context.clearRect(0, 0, canvas.width, canvas.height);
	}

	function handleKeyDown({ key }: KeyboardEvent) {
		if (key === 'Escape') {
			window.close();
		}
	}
</script>

<svelte:window bind:innerWidth={width} bind:innerHeight={height} on:keydown={handleKeyDown} />

<canvas
	class="absolute w-screen h-screen"
	bind:this={canvas}
	on:mousedown={onMouseDown}
	on:mouseup={onMouseUp}
	on:mousemove={onMouseMove}
	on:mouseleave={onMouseLeave}
	on:contextmenu|preventDefault
	{width}
	{height}
/>
{#if hintVisible}
	<div class="absolute pointer-events-none alert variant-ghost-surface right-2 top-2">
		<EyeIcon />
		<div class="alert-message">
			<div>Drag and drop over an area to select it</div>
			<div>Press <kbd>Esc</kbd> to cancel</div>
		</div>
	</div>
{/if}
