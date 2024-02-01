import { browser } from "$app/environment";
import { convertFileSrc } from "@tauri-apps/api/tauri";

const PROTOCOL = "fetch";

import { type Wasm, WASM_PATH } from "./path";
import { modifyGlobalFetch } from "$lib/utils";


export async function load() {
    
    if (!browser) {
        return;
    }

    // modifies the fetch function to work with wasm
    modifyGlobalFetch(WASM_PATH, PROTOCOL);

    // converts path to url
    const file = convertFileSrc(`${WASM_PATH}/project.js`, PROTOCOL);
    
    // dynamic import the file
    const wasm: Wasm = await import(/* @vite-ignore */ file);

    // call init(), wasm.default() is init()
    await wasm.default();

    return {wasm};
}

