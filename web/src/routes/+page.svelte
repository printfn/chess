<script lang="ts">
	import Board from '$lib/board/board.svelte';
	import PromotionModal from './PromotionModal.svelte';
	import Settings from './Settings.svelte';
	import GameOverModal from './GameOverModal.svelte';
	import { applyMove, calculateMove, getGameState } from '$lib/wasm';
	import type { Config } from 'chessground/config';
	import type { Key } from 'chessground/types';
	import { Button, Heading, P } from 'flowbite-svelte';
	import { enableQuiescence, depth, showMaterialDifference } from '$lib/settings';

	const initialPosition = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1';

	let fen = $state(initialPosition);
	let perspective: 'white' | 'black' = $state('white');
	let lastMove: [Key, Key] | undefined = $state(undefined);
	let block = $state(false);
	let settingsModal: Settings;
	let promotionModal: PromotionModal;
	let gameOverModal: GameOverModal;

	let gameState = $derived(getGameState(fen));

	let config = $derived<Config>({
		fen,
		coordinates: false,
		orientation: perspective,
		check: gameState.check,
		turnColor: gameState.currentPlayer,
		lastMove,
		animation: {
			enabled: true,
		},
		movable: {
			color: gameState.currentPlayer,
			free: false,
			dests: block ? new Map() : gameState.dests,
			events: {
				after: async (from, to) => {
					lastMove = [from, to];
					let nextPos = applyMove(fen, from, to);
					if (!nextPos) {
						const promotion = await promotionModal.promote();
						nextPos = applyMove(fen, from, to, promotion);
					}
					fen = nextPos;
					block = true;
					if (getGameState(nextPos).dests.size === 0) {
						gameOverModal.open();
						return;
					}
					const result = await calculateMove(nextPos, $depth, $enableQuiescence);
					fen = result.fen;
					lastMove = [result.from, result.to];
					if (getGameState(result.fen).dests.size === 0) {
						gameOverModal.open();
						return;
					}
					block = false;
				},
			},
		},
	});

	function flip(e: MouseEvent) {
		(e.currentTarget as HTMLElement)?.blur();
		perspective = perspective === 'white' ? 'black' : 'white';
	}

	async function newGame(e: MouseEvent, color: 'white' | 'black' | 'random') {
		(e.currentTarget as HTMLElement)?.blur();
		if (color === 'random') {
			color = Math.random() > 0.5 ? 'white' : 'black';
		}
		fen = initialPosition;
		lastMove = undefined;
		perspective = color;
		block = false;
		if (color === 'black') {
			const result = await calculateMove(initialPosition, $depth, $enableQuiescence);
			fen = result.fen;
			lastMove = [result.from, result.to];
		}
	}
</script>

<div class="container mx-auto px-4">
	<Heading class="text-4xl font-semibold text-center p-2">Chess</Heading>
	<div class="max-w-[80vh] mx-auto">
		{#if $showMaterialDifference}
			<P class="text-right">Material Difference: {gameState.materialDifference}</P>
		{/if}
		<Board {config} class="aspect-square" />
		<div class="grid gap-2 my-2">
			<Button outline on:click={flip}>Flip</Button>
			<Button outline on:click={() => settingsModal.open()}>Settings</Button>
			<Button outline on:click={e => newGame(e, 'white')}>New Game (White)</Button>
			<Button outline on:click={e => newGame(e, 'black')}>New Game (Black)</Button>
			<Button outline on:click={e => newGame(e, 'random')}>New Game (Random)</Button>
		</div>
	</div>
</div>

<GameOverModal
	bind:this={gameOverModal}
	check={gameState.check}
	currentPlayer={gameState.currentPlayer} />
<PromotionModal bind:this={promotionModal} />
<Settings bind:this={settingsModal} />
