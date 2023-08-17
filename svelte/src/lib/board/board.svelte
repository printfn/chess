<script lang="ts">
	import './board.css';
	import type { Config } from 'chessground/config';
	import type { Api } from 'chessground/api';
	import { Chessground } from 'chessground';

	export let config: Config;
	export let classes: string;

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

<div bind:this={chessboard} class={classes} />
