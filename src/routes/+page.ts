import { browser } from "$app/environment";
import { exportWasm } from "$lib/utils";


function addWorld(str: string) {
    return str + " world!";
}



export async function load() {
    if (!browser) {
        return;
    }
    
    // exports addWorld() to wasm, putting it under the "hello" namespace
	exportWasm({addWorld}, "hello");

}

