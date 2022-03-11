<script>
	import Loader from '$lib/Loader.svelte';
	import { session } from '$app/stores';
	import { goto } from '$app/navigation';

	let name = null;
	let loading = false;
	let newFileName = null;
	let fileinput;
	let sound_file;
	let error = false;

	async function onFileSelected(e) {
		const file = e.target.files[0];
		newFileName = file.name;
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
		const response = await fetch(`${$session.api_base_url}/sounds`, {
			method: 'POST',
			body: JSON.stringify({
				name: name,
				file_url: sound_file,
				volume: 75
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

	$: console.log(fileinput);
</script>

<div class="grid place-content-center w-full max-w-s">
	<form
		class="px-8 pt-6 pb-8 mb-4 bg-white shadow-md rounded w-full"
		on:submit|preventDefault={submit}
	>
		<div class="mb-4">
			<label class="block text-slate-700 text-sm font-bold mb-2" for="name"> Name </label>
			<input
				class="shadow appearance-none border rounded w-full py-2 px-3 text-slate-700 leading-tight focus:outline-none focus:shadow-outline"
				type="text"
				placeholder="name"
				bind:value={name}
			/>
		</div>

		<div class="mb-4">
			<label class="block text-slate-700 text-sm font-bold mb-2" for="name"> Sound </label>
			<div class="flex">
				<svg
					class="w-5 h-5 cursor-pointer"
					xmlns="http://www.w3.org/2000/svg"
					on:click={() => {
						fileinput.click();
					}}
					viewBox="0 0 512 512"
					><!-- Font Awesome Pro 5.15.4 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) --><path
						d="M296 384h-80c-13.3 0-24-10.7-24-24V192h-87.7c-17.8 0-26.7-21.5-14.1-34.1L242.3 5.7c7.5-7.5 19.8-7.5 27.3 0l152.2 152.2c12.6 12.6 3.7 34.1-14.1 34.1H320v168c0 13.3-10.7 24-24 24zm216-8v112c0 13.3-10.7 24-24 24H24c-13.3 0-24-10.7-24-24V376c0-13.3 10.7-24 24-24h136v8c0 30.9 25.1 56 56 56h80c30.9 0 56-25.1 56-56v-8h136c13.3 0 24 10.7 24 24zm-124 88c0-11-9-20-20-20s-20 9-20 20 9 20 20 20 20-9 20-20zm64 0c0-11-9-20-20-20s-20 9-20 20 9 20 20 20 20-9 20-20z"
					/></svg
				>
				{#if newFileName}
					{newFileName}
				{:else}
					[upload new]
				{/if}

				<input
					class="hidden"
					type="file"
					accept=".mp3"
					on:change={(e) => onFileSelected(e)}
					bind:this={fileinput}
					required
				/>
				<audio src={sound_file} controls />
			</div>
		</div>

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
</div>
