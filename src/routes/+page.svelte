<script lang="ts">
	import { onMount } from 'svelte';
	import { exportWasm } from "$lib/utils";
	

	export let data;
	let display = "";

	$: wasm = data.wasm!;
	

	function addWorld(str: string) {
		return str + " world!";
	}

	onMount(() => {
		// exports addWorld() to wasm, putting it under the "hello" namespace
		exportWasm({addWorld}, "hello");

		display = wasm.init_script();
		document.body.style.display = "revert";
	})

</script>


<h1>Welcome to SvelteKt!</h1>
<p>{display}</p>


<svelte:window on:keypress={e => {

	if (e.ctrlKey && e.key == "r") {
		location.reload();
	}
}}/>

<style>
	:global(body) {
		display: none;
	}
</style>