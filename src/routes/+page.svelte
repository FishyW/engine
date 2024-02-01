<script lang="ts">
	import { onMount } from 'svelte';
	import InitModal from "$lib/components/InitModal.svelte";
	// import type * as path from '@tauri-apps/api/path';
	import { convertFileSrc } from '@tauri-apps/api/tauri';

	import {type Wasm} from "$lib/path";
	import { modifyGlobalFetch } from '$lib/utils';

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

	// tauriPath module, needs to be dynamically imported
	// see https://github.com/tauri-apps/tauri/discussions/5271
	onMount(async () => {
		tauriPath = await import("@tauri-apps/api/path");
	});

	// promise wrapper -> makes a promise returns the tuple [data, error]
	// this prevents an annoying try catch block
	async function pwrap<T>(promise: Promise<T>): Promise<[T|null, null|string|Error]> {
		try {
			return [await promise, null];
		} catch (e) {
			return [null, e as string | Error];
		}
	}

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

	async function onSelectionEnd(event: CustomEvent<any>) {
		const projectPath: string = event.detail;

		const wasmPath = await tauriPath.join(projectPath, ...wasmPathSuffix);
		const [_, err] = await pwrap(initializeWasm(wasmPath));

		if (err != null) {
			alert("Invalid project folder!");
			console.error(err);
			return;
		}

		showInitModal = false;
		display = wasm.init_script();
	}


</script>

{#if showInitModal}
<InitModal on:selection_end={onSelectionEnd} />
{/if}

{#if display}
<h1>Hello World in Action!</h1>
<p>{display}</p>
{/if}

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
	}

	h1, p {
		margin: 10px;
	}




</style>