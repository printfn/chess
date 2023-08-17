<script lang="ts">
	import Board from '$lib/board/board.svelte';
	import PromotionModal from '$lib/PromotionModal.svelte';
	import { applyMove, calculateMove, possibleMoves, type PromotionPiece } from '$lib/wasm';
	import type { Config } from 'chessground/config';
	import type { Key } from 'chessground/types';
	import { Modal, Button, Heading } from 'flowbite-svelte';

	const initialPosition = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1';
	const depth = 3;
	const enableQuiescence = false;

	let fen = initialPosition;
	let perspective: 'white' | 'black' = 'white';
	let lastMove: [Key, Key] | undefined = undefined;
	let block = false;
	let gameOverModal = false;
	let promote: () => Promise<PromotionPiece>;

	let config: Config;
	$: config = {
		fen,
		coordinates: false,
		orientation: perspective,
		lastMove: lastMove,
		animation: {
			enabled: true,
		},
		movable: {
			free: false,
			dests: block ? new Map() : possibleMoves(fen),
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
					if (possibleMoves(nextPos).size === 0) {
						gameOverModal = true;
						return;
					}
					const result = await calculateMove(nextPos, depth, enableQuiescence);
					fen = result.fen;
					lastMove = [result.from, result.to];
					if (possibleMoves(result.fen).size === 0) {
						gameOverModal = true;
						return;
					}
					block = false;
				},
			},
		},
	};

	function flip() {
		perspective = perspective === 'white' ? 'black' : 'white';
	}
</script>

<div class="container mx-auto px-4">
	<Heading class="text-4xl font-semibold text-center p-2">Chess</Heading>
	<Board {config} classes="aspect-square max-w-[80vh] mx-auto" />
	<Button class="block mx-auto" on:click={flip}>Flip</Button>
</div>

<Modal title="Game Over" bind:open={gameOverModal} autoclose>
	<p class="text-base leading-relaxed text-gray-500 dark:text-gray-400">The game is over.</p>
	<svelte:fragment slot="footer">
		<Button>Close</Button>
	</svelte:fragment>
</Modal>

<PromotionModal bind:promote />
