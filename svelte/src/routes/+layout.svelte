<script lang="ts">
	import { theme } from '$lib/settings';
	import '../app.css';

	theme.subscribe(theme => {
		const root = document.querySelector(':root') as HTMLElement;
		if (!root) {
			return;
		}
		const name = theme === 'pink' ? 'carissma' : 'driftwood';
		const numbers = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];
		for (const n of numbers) {
			root.style.setProperty(
				`--color-primary-${n}`,
				`var(--${name}-${n})`,
			);
		}
		root.setAttribute('data-theme', theme);
	});
</script>

<slot />
