import { writable } from 'svelte/store';

export const themes = ['pink', 'brown'];
export type Theme = 'pink' | 'brown';

function initialTheme(): Theme {
	const theme = localStorage.getItem('theme');
	if (theme && themes.includes(theme)) {
		return theme as Theme;
	}
	return 'pink';
}

export const theme = writable<Theme>(initialTheme());
theme.subscribe(theme => {
	localStorage.setItem('theme', theme);
});
export const depth = writable(3);
export const enableQuiescence = writable(true);
