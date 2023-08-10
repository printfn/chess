<script lang="ts">
	import { Chessground } from 'chessground';
	import type { Api } from 'chessground/api';
	import type { Config } from 'chessground/config';
	import { onMount } from 'svelte';
	import './board.css';

	let chessboard: HTMLDivElement;
	let api: Api;
	let config: Config;

	onMount(() => {
		api = Chessground(chessboard, config);
	});
</script>

<div class="container mx-auto px-4">
	<h1 class="text-2xl">Chess</h1>
	<div bind:this={chessboard} class="aspect-square max-w-[80vh]" />
</div>
