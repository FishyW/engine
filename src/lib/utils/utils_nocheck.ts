// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck

// utility files to modify global this
// ts-nocheck is enabled or else ts will complain about globalThis



import { convertFileSrc } from "@tauri-apps/api/tauri";
import {snakelize} from "./utils";

function isURL(url: RequestInfo | URL): url is URL {
    return url.href !== undefined;
}

// exports a module "mod" to wasm
// example usage: exportWasm({myFunc, myVar})
// note that this converts camelCase to snake_case
export function exportWasm(mod: object, namespace?: string) {
    mod = snakelize(mod);
    if (namespace == undefined) {
        Object.assign(globalThis, mod);
    } else {
        globalThis[namespace] = {};
        Object.assign(globalThis[namespace], mod);
    }

}



// modifies the fetch api to fetch the wasm file in the correct directory
// takes in the path to the wasm folder
export function modifyGlobalFetch(wasmPath: string, protocol: string) {
    // means this function has already been called
    if (globalThis.webFetch == undefined) {
        globalThis.webFetch = fetch;
    }

    globalThis.fetch = (url, ...args) => {
        if (isURL(url)) {
            // if wasm file is loaded
            if (url.pathname.endsWith(".wasm")) {
                const path = `${wasmPath}/project_bg.wasm`; 
                url = new URL(convertFileSrc(path, protocol));
            }
        }
        
        return globalThis.webFetch(url, ...args);
    }
}
