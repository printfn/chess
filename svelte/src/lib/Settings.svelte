<script lang="ts">
	import { Button, Label, Modal, Select, Toggle } from 'flowbite-svelte';

	export let open = false;
</script>

<Modal title="Settings" bind:open autoclose outsideclose>
	<Label>
		Theme
		<Select class="mt-2" placeholder="">
			<option value="pink">Pink</option>
			<option value="brown">Brown</option>
		</Select>
	</Label>
	<Label>
		Calculation Depth
		<Select class="mt-2" placeholder="">
			<option value="1">1</option>
			<option value="2">2</option>
			<option value="3">3</option>
			<option value="4">4</option>
		</Select>
	</Label>
	<Toggle checked={true}>Enable Quiescence Search</Toggle>
	<svelte:fragment slot="footer">
		<Button class="ml-auto">Close</Button>
	</svelte:fragment>
</Modal>
