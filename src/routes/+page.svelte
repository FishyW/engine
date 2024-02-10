<script lang="ts">
	import { onMount } from 'svelte';
	import InitModal from "$lib/components/InitModal.svelte";

	import { pwrap } from '$lib/utils';
	import { initTauri, initializeWasm, wasm } from '$lib/init';

	// variables
	let display = "";
	let projectPath = "";
	let showInitModal = true;

	let visible = false;

	// tauriPath module, needs to be dynamically imported
	// see https://github.com/tauri-apps/tauri/discussions/5271
	onMount(async () => {
		await initTauri();
	});

	// show the document body
	function showBody() {
		const mainDiv: HTMLDivElement = document.querySelector(".main")!; 
		mainDiv.style.display = "revert";

		// when display is set to none, no transitions are played
		// -> need to wait for the next frame so display is block again 
		// https://css-tricks.com/so-youd-like-to-animate-the-display-property/
		// requestAnimationFrame calls a function in the next render frame
		requestAnimationFrame(() => {
			visible = true;
		})
	}

	// initialize the wasm module
	
	// called after the user has selected the project path
	async function onSelectionEnd(event: CustomEvent<any>) {
		projectPath = event.detail;

		const [_, err] = await pwrap(initializeWasm(projectPath));

		if (err != null) {
			alert("Invalid project folder!");
			console.error(err);
			return;
		}

		showInitModal = false;
		
		showBody();
	}


</script>

{#if showInitModal}
	<InitModal on:selection_end={onSelectionEnd} />
{/if}

<div class="main" class:visible>
	<h1>Hello World in Action!</h1>
	<p>{display}</p>
</div>

<svelte:body on:click={e => {
	if (projectPath)
		wasm.eventReceive("click");
}}/>

<svelte:window on:keypress={e => {
	

	// Ctrl + Shift + R to reload fully 
	if (e.ctrlKey && e.shiftKey && e.key == "R") {
		location.reload();
	}

	// Ctrl + R to reload wasm module
	// Also clears the console
	if (e.ctrlKey && e.key == "r" && projectPath) {
		console.clear();
		initializeWasm(projectPath);
	}
}}/>

<style>
	:global(body) {
		margin: 0;
		height: 100vh;
		width: 100vw;
		box-sizing: border-box;
		position: absolute;
	}

	h1, p {
		margin: 10px;
	}

	.main {
		opacity: 0;
		display: none;
		transition: opacity 0.5s ease-in-out;
	}

	.main.visible {
		opacity: 1;
	}

</style>