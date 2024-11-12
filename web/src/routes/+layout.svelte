<script lang="ts">
	import { theme, type Theme } from '$lib/settings';
	import '../app.css';

	let { children } = $props();

	function handleThemeUpdate(theme: Theme) {
		const root = document.querySelector(':root') as HTMLElement;
		if (!root) {
			return;
		}
		root.setAttribute('data-theme', theme);
		localStorage.setItem('theme', theme);
	}

	$effect(() => handleThemeUpdate($theme));
</script>

{@render children?.()}
