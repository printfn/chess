<script lang="ts">
	import { Button, Label, Modal, Select } from 'flowbite-svelte';
	import type { PromotionPiece } from './wasm';

	let piece: PromotionPiece = 'Q';
	let onExit: ((value: PromotionPiece) => void) | undefined = undefined;

	const pieces = [
		{ value: 'Q', name: 'Queen' },
		{ value: 'R', name: 'Rook' },
		{ value: 'B', name: 'Bishop' },
		{ value: 'N', name: 'Knight' },
	];

	export function promote(): Promise<PromotionPiece> {
		return new Promise<PromotionPiece>(resolve => {
			piece = 'Q';
			onExit = resolve;
		});
	}

	function close() {
		onExit?.(piece);
		onExit = undefined;
	}
</script>

<Modal title="Promotion" open={Boolean(onExit)} permanent>
	<Label>
		Choose which piece to promote to:
		<Select class="mt-2" items={pieces} placeholder="" bind:value={piece} />
	</Label>
	<svelte:fragment slot="footer">
		<Button class="ml-auto" on:click={close}>Confirm</Button>
	</svelte:fragment>
</Modal>
