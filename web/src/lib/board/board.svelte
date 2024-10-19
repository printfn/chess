<script lang="ts">
	import './board.css';
	import type { Config } from 'chessground/config';
	import type { Api } from 'chessground/api';
	import { Chessground } from 'chessground';

	interface Props {
		config: Config;
		class?: string;
	}

	let { config, class: className = '' }: Props = $props();

	let chessboard: HTMLDivElement;
	let api: Api;

	$effect(() => {
		if (api) {
			console.log('api.set(config)', config);
			api.set(config);
		} else if (chessboard) {
			console.log('api = Chessground');
			api = Chessground(chessboard, config);
		}
	});
</script>

<div bind:this={chessboard} class={className}></div>

<style>
	:global(cg-container) {
		left: 0;
		right: 0;
		margin-left: auto;
		margin-right: auto;
	}
</style>
