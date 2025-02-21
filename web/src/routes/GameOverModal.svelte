<script lang="ts">
	import { Modal, Button, P } from 'flowbite-svelte';

	type Props = {
		check: boolean;
		currentPlayer: 'white' | 'black';
	};
	let { check, currentPlayer }: Props = $props();

	let isOpen = $state(false);
	let gameOverTitle = $derived(check ? 'Checkmate' : 'Stalemate');
	let gameOverMessage = $derived(
		check
			? `${currentPlayer === 'white' ? 'Black' : 'White'} wins by checkmate.`
			: 'The game is drawn by stalemate.',
	);

	export function open() {
		isOpen = true;
	}
</script>

<Modal title={gameOverTitle} bind:open={isOpen} autoclose outsideclose>
	<P>{gameOverMessage}</P>
	<svelte:fragment slot="footer">
		<Button class="ml-auto">Close</Button>
	</svelte:fragment>
</Modal>
