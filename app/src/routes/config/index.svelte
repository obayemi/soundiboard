<script context="module">
	export async function load({ session, fetch }) {
		const sounds_url = `${session.api_base_url}/sounds/`;
		const response = await fetch(sounds_url, {
			headers: {
				Authorization: `BEARER ${session.api_token}`
			}
		});
		const sounds = await response.json();
		return {
			props: {
				sounds
			}
		};
	}
</script>

<script>
	export let sounds;
</script>

<table class="border-collapse border border-slate-500 min-w-full">
	<thead>
		<tr>
			<th
				class="border border-slate-600 px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
				>Name</th
			>
			<th
				class="border border-slate-600 px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
				>File</th
			>
			<th
				class="border border-slate-600 px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
				>Volume</th
			>
		</tr>
	</thead>
	<tbody>
		{#each sounds as sound}
			<tr class="border border-slate-700 hover:bg-slate-500">
				<td class="whitespace-nowrap"
					><a class="block px-6 py-4" href="/config/{encodeURIComponent(sound.name)}"
						>{sound.name}</a
					></td
				>
				<td class="whitespace-nowrap"
					><a class="block px-6 py-4" href="/config/{encodeURIComponent(sound.name)}"
						>{sound.file_url}</a
					></td
				>
				<td class="whitespace-nowrap"
					><a class="block px-6 py-4" href="/config/{encodeURIComponent(sound.name)}"
						>{sound.volume}</a
					></td
				>
			</tr>
		{/each}
	</tbody>
</table>
