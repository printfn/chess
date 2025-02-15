<script lang="ts">
	import { Button, Label, Modal, Select } from 'flowbite-svelte';
	import type { PromotionPiece } from '../lib/wasm';

	let piece: PromotionPiece = $state('Q');
	let onExit: ((value: PromotionPiece) => void) | undefined = $state(undefined);

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

<Modal title="Promotion" open={Boolean(onExit)} dismissable={false} backdropClass="fixed inset-0 z-40 dark:bg-gray-900/80 bg-gray-900/50">
	<Label>
		Choose which piece to promote to:
		<Select class="mt-2" items={pieces} placeholder="" bind:value={piece} />
	</Label>
	<svelte:fragment slot="footer">
		<Button class="ml-auto" on:click={close}>Confirm</Button>
	</svelte:fragment>
</Modal>
