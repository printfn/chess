<script lang="ts">
	import { Button, Label, Modal, Select, Toggle } from 'flowbite-svelte';
	import { theme, depth, enableQuiescence, Themes } from './settings';

	export let open = false;

	const themes = Object.entries(Themes).map(([k, v]) => ({ name: v, value: k }));

	function changeDepth(e: Event) {
		if (!e.currentTarget) return;
		$depth = parseInt((e.currentTarget as HTMLSelectElement).value);
	}
</script>

<Modal title="Settings" bind:open autoclose outsideclose>
	<Label>
		Theme
		<Select class="mt-2" bind:value={$theme} placeholder="" items={themes} />
	</Label>
	<Label>
		Calculation Depth
		<Select class="mt-2" value={$depth.toString()} on:change={changeDepth} placeholder="">
			<option value="1">1</option>
			<option value="2">2</option>
			<option value="3">3</option>
			<option value="4">4</option>
		</Select>
	</Label>
	<Toggle bind:checked={$enableQuiescence}>Enable Quiescence Search</Toggle>
	<svelte:fragment slot="footer">
		<Button class="ml-auto">Close</Button>
	</svelte:fragment>
</Modal>
