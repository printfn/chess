<script lang="ts">
	import { theme, type Theme } from '$lib/settings';
	import '../app.css';

	function handleThemeUpdate(theme: Theme) {
		const root = document.querySelector(':root') as HTMLElement;
		if (!root) {
			return;
		}
		root.setAttribute('data-theme', theme);
		localStorage.setItem('theme', theme);
	}

	$: handleThemeUpdate($theme);
</script>

<slot />
