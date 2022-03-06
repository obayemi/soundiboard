<script>
	import '../app.css';
</script>

<svelte:head>
	<title>Sensiboard</title>
	<style>
		html,
		body {
			height: 100%;
		}
		/*body {*/
		/*background-color: #191919;*/
		/*}*/
	</style>
</svelte:head>

<div class="bg-slate-800 h-full">
	<slot />
</div>
