<script lang="ts">
	import { onMount } from 'svelte';
	import InitModal from "$lib/components/InitModal.svelte";
	// import type * as path from '@tauri-apps/api/path';
	import { convertFileSrc } from '@tauri-apps/api/tauri';

	import {type Wasm} from "$lib/path";
	import { modifyGlobalFetch, pwrap } from '$lib/utils';

	const PROTOCOL = "fetch";

	// project/engine/wasm
	const wasmPathSuffix = ["engine", "wasm"];
	const wasmJSName = "project.js";

	// variables
	let display = "";
	let projectPath = "";
	let showInitModal = true;

	// dynamic modules
	let tauriPath: typeof import("@tauri-apps/api/path");
	let wasm: Wasm;

	let visible = false;

	// tauriPath module, needs to be dynamically imported
	// see https://github.com/tauri-apps/tauri/discussions/5271
	onMount(async () => {
		tauriPath = await import("@tauri-apps/api/path");
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
	async function initializeWasm(wasmPath: string) {

		// converts path to url
		const file = convertFileSrc(await tauriPath.join(wasmPath, wasmJSName), PROTOCOL);
	
		// dynamic import the file
		wasm = await import(/* @vite-ignore */ file);
	
		// modifies the fetch function to work with wasm
		modifyGlobalFetch(wasmPath, PROTOCOL);

		// call init(), wasm.default() is init()
		await wasm.default();		
	}

	// called after the user has selected the project path
	async function onSelectionEnd(event: CustomEvent<any>) {
		projectPath = event.detail;

		const wasmPath = await tauriPath.join(projectPath, ...wasmPathSuffix);
		const [_, err] = await pwrap(initializeWasm(wasmPath));

		if (err != null) {
			alert("Invalid project folder!");
			console.error(err);
			return;
		}

		showInitModal = false;
		display = wasm.init_script();
		
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

<svelte:window on:keypress={e => {

	if (e.ctrlKey && e.key == "r") {
		location.reload();
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