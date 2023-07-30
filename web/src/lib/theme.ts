export const themes = ['pink', 'brown'];
export type Theme = 'pink' | 'brown';

export function initialTheme(): Theme {
	const theme = localStorage.getItem('theme');
	if (theme && themes.includes(theme)) {
		return theme as Theme;
	}
	return 'pink';
}
