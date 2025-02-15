<script lang="ts">
	import { A, Button, Label, Modal, Select, Toggle } from 'flowbite-svelte';
	import { theme, depth, enableQuiescence, Themes, showMaterialDifference } from '../lib/settings';

	let isOpen = $state(false);

	export function open() {
		isOpen = true;
	}

	const themes = Object.entries(Themes).map(([k, v]) => ({ name: v, value: k }));

	function changeDepth(e: Event) {
		if (!e.currentTarget) return;
		$depth = parseInt((e.currentTarget as HTMLSelectElement).value);
	}
</script>

<Modal title="Settings" bind:open={isOpen} autoclose outsideclose backdropClass="fixed inset-0 z-40 dark:bg-gray-900/80 bg-gray-900/50">
	<Label>
		Theme
		<Select class="mt-2" bind:value={$theme} placeholder="" items={themes} />
	</Label>
	<Label>
		Calculation Depth
		<Select class="mt-2" value={$depth.toString()} on:change={changeDepth} placeholder="">
			<option value="0">0</option>
			<option value="1">1</option>
			<option value="2">2</option>
			<option value="3">3</option>
			<option value="4">4</option>
		</Select>
	</Label>
	<Toggle bind:checked={$enableQuiescence}>Enable Quiescence Search</Toggle>
	<Toggle bind:checked={$showMaterialDifference}>Show Material Difference</Toggle>
	<A target="_blank" rel="noreferrer noopener" href="https://github.com/printfn/chess">
		View Source Code on GitHub
	</A>
	<svelte:fragment slot="footer">
		<Button class="ml-auto">Close</Button>
	</svelte:fragment>
</Modal>
