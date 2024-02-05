// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck

// utility files to modify global this
// ts-nocheck is enabled or else ts will complain about globalThis


// exports a module "mod" to wasm
// example usage: exportWasm({myFunc, myVar})
// note that this converts camelCase to snake_case
export function exportWasm(mod: object, namespace?: string) {
    const tmp = Object.entries(mod).map(([key, value]) => {
        // convert key from camelCase to snake_case
        key = key.split("").map(ch => {
            // ch is lowercase
            if (ch.toLowerCase() === ch) {
                return ch;
            }
            return "_" + ch.toLowerCase();
        }).join("");
        return [key, value];
    });
    mod = Object.fromEntries(tmp);

    if (namespace == undefined) {
        Object.assign(globalThis, mod);
    } else {
        globalThis[namespace] = {};
        Object.assign(globalThis[namespace], mod);
    }

}
