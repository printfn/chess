import { writable } from 'svelte/store';

export const Themes = ['pink', 'brown'] as const;
export type Theme = (typeof Themes)[number];

export const theme = writable<Theme>('pink');
export const depth = writable(3);
export const enableQuiescence = writable(true);
