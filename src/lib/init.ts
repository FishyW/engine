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

function callRustInits(wasm: {[key: string]: unknown}) {
    const initFunctions: string[] = [];
    const initSceneFunctions: string[] = [];
    const allFnNames = Object
                        .keys(wasm)
                        .filter((key) => typeof wasm[key] === "function");
    
    allFnNames.forEach((keyName) => {
        if (keyName.startsWith("init_scene")) {
            initSceneFunctions.push(keyName);
        } 
        else if (keyName == "init_start") {
            // wasm.init_start() gets called first
            (wasm[keyName] as VoidFunc)()
        }
        else if (keyName.startsWith("init_")) {
            initFunctions.push(keyName);
        }
    });

    // init scene functions get called first
    initSceneFunctions
        .concat(initFunctions)
        .forEach((funcName) => {
            (wasm[funcName] as VoidFunc)();
        });
}
