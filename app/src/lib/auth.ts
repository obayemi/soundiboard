import { writable, get } from 'svelte/store';
import { session } from '$app/stores';

function createCount() {
	console.log('create count store');
	const { subscribe, set, update } = writable(0);

	return {
		subscribe,
		increment: () => update((n) => n + 1),
		decrement: () => update((n) => n - 1),
		reset: () => set(0)
	};
}

export const count = createCount();
