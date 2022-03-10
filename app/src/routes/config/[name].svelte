<script context="module">
	export async function load({ params, session, fetch }) {
		console.log(params.name);
		const sound_url = `${session.api_base_url}/sounds/${encodeURIComponent(params.name)}`;
		const response = await fetch(sound_url, {
			headers: {
				Authorization: `BEARER ${session.api_token}`
			}
		});
		const sound = await response.json();
		return {
			props: {
				sound,
				sound_url
			}
		};
	}
</script>

<script>
	import Loader from '$lib/Loader.svelte';
	import { session } from '$app/stores';
	import { goto } from '$app/navigation';

	export let sound;
	export let sound_url;
	let loading = false;
	let fileinput;
	let sound_file;
	let error = false;

	async function onFileSelected(e) {
		const file = e.target.files[0];
		const reader = new FileReader();
		reader.readAsDataURL(file);
		reader.onload = (loaded) => {
			console.log(loaded);
			sound_file = loaded.target.result;
		};
	}

	async function submit() {
		console.log(sound_file);

		loading = true;
		const response = await fetch(sound_url, {
			method: 'PUT',
			body: JSON.stringify({
				...sound,
				file_url: sound_file || sound.file_url
			}),
			headers: {
				Authorization: `BEARER ${$session.api_token}`
			}
		});
		loading = false;
		if (!response.ok) {
			error = true;
			return;
		}
		goto('/config');
	}
</script>

<form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4" on:submit|preventDefault={submit}>
	{sound.name}
	{sound.file_url}
	<audio src={sound_file || sound.file_url} controls>
		<track kind="captions" />
	</audio>
	<input type="file" accept=".mp3" on:change={(e) => onFileSelected(e)} bind:this={fileinput} />

	<button
		class="bg-slate-700 hover:bg-slate-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline inline-flex"
		type="submit"
		disabled={loading}
	>
		{#if !loading}
			Update
		{:else}
			<Loader />
		{/if}
	</button>
</form>
