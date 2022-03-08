<script>
	import { goto } from '$app/navigation';
	import { session } from '$app/stores';
	import Loader from '$lib/Loader.svelte';

	const user = {
		username: undefined,
		password: undefined
	};
	let error = false;
	let loading = false;

	async function submit(a) {
		loading = true;
		const response = await fetch(`${$session.api_base_url}/signup`, {
			method: 'POST',
			body: JSON.stringify(user)
		});
		loading = false;
		if (!response.ok) {
			error = true;
			return;
		}
		goto('login');
	}
</script>

<div class="grid place-content-center">
	<div class="w-full max-w-xs">
		<form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4" on:submit|preventDefault={submit}>
			<div class="mb-4">
				<label class="block text-slate-700 text-sm font-bold mb-2" for="username"> Username </label>
				<input
					class="shadow appearance-none border rounded w-full py-2 px-3 text-slate-700 leading-tight focus:outline-none focus:shadow-outline"
					id="username"
					type="text"
					placeholder="Username"
					bind:value={user.username}
				/>
			</div>
			<div class="mb-6">
				<label class="block text-slate-700 text-sm font-bold mb-2" for="password"> Password </label>
				<input
					class="shadow appearance-none border rounded w-full py-2 px-3 text-slate-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
					id="password"
					type="password"
					placeholder="******************"
					bind:value={user.password}
				/>
			</div>

			{#if error}
				<div class="mb-6">
					<p class="text-red-500 text-xs italic">Something went wrong</p>
				</div>
			{/if}

			<div class="flex items-center justify-between">
				<button
					class="bg-slate-700 hover:bg-slate-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline inline-flex"
					type="submit"
				>
					{#if !loading}
						Sign up
					{:else}
						<Loader />
					{/if}
				</button>
			</div>
		</form>
		<p class="text-center text-gray-500 text-xs">&copy;2020 Acme Corp. All rights reserved.</p>
	</div>
</div>
