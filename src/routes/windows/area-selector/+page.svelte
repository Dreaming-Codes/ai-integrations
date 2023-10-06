<script lang="ts">
	import { onMount } from 'svelte';
	import { emit } from '@tauri-apps/api/event';

	let canvas: HTMLCanvasElement;
	let context: CanvasRenderingContext2D;

	let width = window.innerWidth;
	let height = window.innerHeight;

	onMount(() => {
		context = canvas.getContext('2d')!;

		// Set color options
		context.lineWidth = 2;
		context.strokeStyle = `rgb(${getComputedStyle(document.body)
			.getPropertyValue('--color-primary-500')
			.split(' ')
			.join(',')})`;
		context.fillStyle = `rgba(${getComputedStyle(document.body)
			.getPropertyValue('--color-surface-500')
			.split(' ')
			.join(',')}, 0.3)`;
	});

	let isDrawing = false;
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

	function onMouseUp({ offsetX: endX, offsetY: endY }: MouseEvent) {
		isDrawing = false;

		emit('selection', {
			startX: start.x,
			startY: start.y,
			endX,
			endY
		});
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
	class="w-screen h-screen"
	bind:this={canvas}
	on:mousedown={onMouseDown}
	on:mouseup={onMouseUp}
	on:mousemove={onMouseMove}
	on:mouseleave={onMouseLeave}
	on:contextmenu|preventDefault
	{width}
	{height}
/>

<style>
	:global(body) {
		background-color: transparent !important;
	}
</style>
