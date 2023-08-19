<script lang="ts">
	import Board from '$lib/board/board.svelte';
	import PromotionModal from './PromotionModal.svelte';
	import Settings from './Settings.svelte';
	import { applyMove, calculateMove, getGameState, type PromotionPiece } from '$lib/wasm';
	import type { Config } from 'chessground/config';
	import type { Key } from 'chessground/types';
	import { Modal, Button, Heading, P } from 'flowbite-svelte';
	import { enableQuiescence, depth } from '$lib/settings';

	const initialPosition = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1';

	let fen = initialPosition;
	let perspective: 'white' | 'black' = 'white';
	let lastMove: [Key, Key] | undefined = undefined;
	let block = false;
	let gameOverModal = false;
	let settingsModal = false;
	let promote: () => Promise<PromotionPiece>;

	function getGameOverMessage(fen: string) {
		const gameState = getGameState(fen);
		if (gameState.dests.size > 0) {
			return;
		}
		if (gameState.check) {
			const winner = gameState.currentPlayer === 'white' ? 'Black' : 'White';
			return { title: 'Checkmate', message: `${winner} wins by checkmate.` };
		}
		return { title: 'Stalemate', message: 'The game is drawn by stalemate.' };
	}

	$: gameOverTitle = getGameOverMessage(fen)?.title || '';
	$: gameOverMessage = getGameOverMessage(fen)?.message || '';

	let config: Config;
	$: {
		const gameState = getGameState(fen);
		config = {
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
				free: false,
				dests: block ? new Map() : gameState.dests,
				events: {
					after: async (from, to) => {
						lastMove = [from, to];
						let nextPos = applyMove(fen, from, to);
						if (!nextPos) {
							const promotion = await promote();
							nextPos = applyMove(fen, from, to, promotion);
						}
						fen = nextPos;
						block = true;
						if (getGameState(nextPos).dests.size === 0) {
							gameOverModal = true;
							return;
						}
						const result = await calculateMove(nextPos, $depth, $enableQuiescence);
						fen = result.fen;
						lastMove = [result.from, result.to];
						if (getGameState(result.fen).dests.size === 0) {
							gameOverModal = true;
							return;
						}
						block = false;
					},
				},
			},
		};
	}

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
		<Board {config} class="aspect-square" />
		<div class="grid gap-2 my-2">
			<Button outline on:click={flip}>Flip</Button>
			<Button outline on:click={() => (settingsModal = true)}>Settings</Button>
			<Button outline on:click={e => newGame(e, 'white')}>New Game (White)</Button>
			<Button outline on:click={e => newGame(e, 'black')}>New Game (Black)</Button>
			<Button outline on:click={e => newGame(e, 'random')}>New Game (Random)</Button>
		</div>
	</div>
</div>

<Modal title={gameOverTitle} bind:open={gameOverModal} autoclose outsideclose>
	<P>{gameOverMessage}</P>
	<svelte:fragment slot="footer">
		<Button class="ml-auto">Close</Button>
	</svelte:fragment>
</Modal>

<PromotionModal bind:promote />
<Settings bind:open={settingsModal} />
