<script lang="ts">
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';
</script>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar>
			<svelte:fragment slot="lead">
				<strong class="text-xl uppercase">Ai Integrations</strong>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<a
					class="btn btn-sm variant-ghost-surface"
					href="https://dreaming.codes"
					target="_blank"
					rel="noreferrer"
				>
					DreamingCodes
				</a>
				<a
						class="btn btn-sm variant-ghost-surface"
						href="https://github.com/skeletonlabs/skeleton"
						target="_blank"
						rel="noreferrer"
				>
					GitHub
				</a>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<!-- Page Route Content -->
	<slot />
</AppShell>
