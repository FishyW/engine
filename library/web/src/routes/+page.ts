import {browser} from "$app/environment";
import { exportWasm } from "$lib/utils";

// no wrapper or else infinite loop
import init, * as wasm from "$lib/wasm";

export async function load() {
    if (browser) {
        await init();
        exportWasm({addWorld: (str: string) => str + " world!"}, "hello");
        wasm.init_script();
        wasm.add_event();
        wasm.add_event();
        return {
            names: wasm.names()
        }
    }
    return {names: []}
}