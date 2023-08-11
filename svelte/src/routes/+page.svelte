<script lang="ts">
	import Board from '$lib/board/board.svelte';

	const fen = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR';
</script>

<div class="container mx-auto px-4">
	<h1 class="text-4xl text-center p-2">Chess</h1>
	<Board {fen} classes="aspect-square max-w-[80vh] mx-auto" />
</div>
