<script lang="ts">
	import './board.css';
	import type { Config } from 'chessground/config';
	import type { Api } from 'chessground/api';
	import { Chessground } from 'chessground';

	export let config: Config;
	let className = '';
	export { className as class };

	let chessboard: HTMLDivElement;
	let api: Api;

	$: {
		if (api) {
			api.set(config);
		} else if (chessboard) {
			api = Chessground(chessboard, config);
		}
	}
</script>

<div bind:this={chessboard} class={className} />

<style>
	:global(cg-container) {
		left: 0;
		right: 0;
		margin-left: auto;
		margin-right: auto;
	}
</style>
