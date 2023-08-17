<script lang="ts">
	import { Button, Modal } from 'flowbite-svelte';
	import type { PromotionPiece } from './wasm';

	let modalOpen = false;
	let piece: PromotionPiece = 'Q';
	let onExit: ((value: PromotionPiece) => void) | undefined = undefined;

	export function promote(): Promise<PromotionPiece> {
		modalOpen = true;
		piece = 'Q';
		return new Promise<PromotionPiece>(resolve => {
			onExit = resolve;
		});
	}

	function close() {
		modalOpen = false;
		onExit?.(piece);
	}
</script>

<Modal title="Promotion" bind:open={modalOpen} permanent>
	<label for="theme-select-input">Choose which piece to promote to:</label>
	<select
		id="theme-select-input"
		class="form-select"
		aria-label="Choose a piece"
		bind:value={piece}
	>
		<option value="Q">Queen</option>
		<option value="R">Rook</option>
		<option value="B">Bishop</option>
		<option value="N">Knight</option>
	</select>
	<svelte:fragment slot="footer">
		<Button on:click={close}>Confirm</Button>
	</svelte:fragment>
</Modal>
