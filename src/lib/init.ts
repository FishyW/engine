// functions to init the project

import { convertFileSrc } from '@tauri-apps/api/tauri';
import { type Wasm } from "$lib/path";
import {  modifyGlobalFetch, type CamelizeKeys, camelize } from './utils';

const PROTOCOL = "fetch";
const WASM_JS_FILENAME = "project.js";

// project/engine/wasm
const WASM_PATH_SUFFIX = ["build", "wasm"];

// dynamic modules
export let wasm: CamelizeKeys<Wasm>;
export let tauri: {
    path: typeof import("@tauri-apps/api/path")
};



// initialize the wasm project
export async function initializeWasm(projectPath: string) {
    const wasmPath = await tauri.path.join(projectPath, ...WASM_PATH_SUFFIX);

    // converts path to url
    const file = convertFileSrc(await tauri.path.join(wasmPath, WASM_JS_FILENAME), PROTOCOL);

    // dynamic import the file
    // add a Date.now() because of browser cache
    wasm = await import(/* @vite-ignore */ file + "?" + Date.now());

    // modifies the fetch function to work with wasm
    modifyGlobalFetch(wasmPath, PROTOCOL);

    // call init(), wasm.default() is init()
    await wasm.default();

    callRustInits(wasm);
    wasm = camelize(wasm);
}

// initialize the tauri module
export async function initTauri() {
    const path = await import("@tauri-apps/api/path");
    tauri = {
        path
    }
}

type VoidFunc = () => void;
const initOrder = ["start", "component", "scene", "receiver", "@"];

function callRustInits(wasm: {[key: string]: unknown}) {
    // order of registration is component first, then scene, the receiver
    const allFnNames = Object
                        .keys(wasm)
                        .filter((key) => typeof wasm[key] === "function")
                        // remove the init() function, all init function names 
                        // must start with `init_`
                        .filter(key => key.startsWith("__init_"));
    

    const initMap: Map<string, string[]> = new Map();
    allFnNames.forEach((keyName) => {
        const keyTmp = keyName.replaceAll("_", " ")
            .trimStart().replaceAll(" ", "_");

        const [_, tmp, ..._rest] = keyTmp.split("_");
        const key = initOrder.includes(tmp) ? tmp: "@";

        let arr = initMap.get(key);
        if (arr == undefined) {
            arr = [];
        }

        arr.push(keyName);
        
        initMap.set(key, arr);
    });
    

    initOrder.forEach(val => {
        const funcNames = initMap.get(val);

        if (funcNames == undefined) {
            return;
        }
        funcNames.forEach(funcName => {
            (wasm[funcName] as VoidFunc)();
        })
    })
}
