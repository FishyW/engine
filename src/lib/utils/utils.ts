// various utilities, except checked using proper Typescript

// promise wrapper -> makes a promise returns the tuple [data, error]
// this prevents an annoying try catch block
export async function pwrap<T>(promise: Promise<T>): Promise<[T|null, null|string|Error]> {
    try {
        return [await promise, null];
    } catch (e) {
        return [null, e as string | Error];
    }
}

export function callInit(wasm: {[key: string]: unknown}) {
    const initFunctions: string[] = [];
    const initSceneFunction: string[] = [];
    const allFnNames = Object
                        .keys(wasm)
                        .filter((key) => typeof wasm[key] === "function");
    
    allFnNames.forEach((keysName) => {
        if (keysName.startsWith("init_scene_")) {
            initSceneFunction.push(keysName);
        }
        else if (keysName.startsWith("init_")) {
            initFunctions.push(keysName);
        }
    });

    initFunctions
        .concat(initSceneFunction)
        .forEach((funcName) => (wasm[funcName] as Function)());
}