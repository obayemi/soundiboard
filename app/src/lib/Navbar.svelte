<script>
	import { session } from '$app/stores';
	import { logout } from '$lib/auth';

	import { clickOutside } from '$lib/clickOutside';
	$: loggedUser = $session.user;
	let menu_opened = false;
</script>

<nav class="bg-slate-700 sticky top-0">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="flex items-center justify-between h-16">
			<div class="flex items-center">
				<div class="block">
					<div class="ml-10 flex items-baseline space-x-4">
						<a
							href="/"
							class="bg-slate-900 text-white px-3 py-2 rounded-md text-sm font-medium"
							aria-current="page">Sensiboard</a
						>

						{#if loggedUser}
							<a
								href="/config"
								class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium"
								>Configure</a
							>
						{/if}
					</div>
				</div>
			</div>
			<div class="block">
				<div class="ml-4 flex items-center md:ml-6">
					{#if loggedUser}
						<!-- Profile dropdown -->
						<div class="ml-3 relative">
							<div>
								<button
									type="button"
									class="max-w-xs bg-gray-800 rounded-full flex items-center text-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white"
									on:click={() => {
										menu_opened = !menu_opened;
									}}
									aria-expanded={!menu_opened}
									aria-haspopup="true"
									aria-controls="menu_opened"
								>
									<span class="inline-block h-8 rounded-full px-3 py-1 align-middle"
										>{loggedUser.username}</span
									>
								</button>
							</div>

							<div
								id="menu_opened"
								class="origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg py-1 bg-white ring-1 ring-black ring-opacity-5 focus:outline-none"
								tabindex="-1"
								hidden={!menu_opened}
								use:clickOutside
								on:click_outside={() => {
									menu_opened = false;
								}}
							>
								<button
									on:click={logout}
									class="block px-4 py-2 text-sm text-gray-700"
									role="menuitem"
									tabindex="-1"
									id="user-menu-item-2"
								>
									Sign out
								</button>
							</div>
						</div>
					{:else}
						<div class="ml-3 relative">
							<div>
								<a
									href="/login"
									type="button"
									class="max-w-xs bg-gray-800 rounded-full flex items-center text-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white"
									on:click={() => {
										menu_opened = !menu_opened;
									}}
									aria-expanded={!menu_opened}
									aria-haspopup="true"
									aria-controls="menu_opened"
								>
									<span class="inline-block h-8 rounded-full px-3 py-1 align-middle">Sign in</span>
								</a>
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>
</nav>
