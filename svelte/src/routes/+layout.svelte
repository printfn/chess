<script lang="ts">
	import { theme } from '$lib/settings';
	import '../app.css';

	theme.subscribe(theme => {
		const root = document.querySelector(':root') as HTMLElement;
		if (!root) {
			return;
		}
		root.setAttribute('data-theme', theme);
	});
</script>

<slot />
