import { browser } from "$app/environment";
import { exportWasm } from "$lib/utils";
import { initTauri } from '$lib/init';


function addWorld(str: string) {
    return str + " world!";
}



export async function load() {
    if (!browser) {
        return;
    }
    
    // tauriPath module, needs to be dynamically imported
	// see https://github.com/tauri-apps/tauri/discussions/5271
    await initTauri();

    // exports addWorld() to wasm, putting it under the "hello" namespace
	exportWasm({addWorld}, "hello");

}

