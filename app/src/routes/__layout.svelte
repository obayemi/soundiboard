<script>
	import '../app.css';
	import Navbar from '$lib/Navbar.svelte';
</script>

<svelte:head>
	<title>Sensiboard</title>
	<style>
		html,
		body {
			height: 100%;
		}
	</style>
</svelte:head>

<div class="bg-slate-800 text-slate-200 h-full overflow-auto">
	<Navbar />

	<div class="page md:container md:mx-auto px-4 p-8">
		<slot />
	</div>
</div>
