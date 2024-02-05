
import { invalidate } from "$app/navigation";


export default function wrapper<T extends object>(wasmLib: T): T {
    const tuple = Object.entries(wasmLib).map(([key, value]) => {
        if (typeof value == "function") {
            return [key, (...args: unknown[]) => {
                const ret = value(...args);
                invalidate("data:wasm");
                return ret;
            }];
        }
        return [key, value];
    });

    return Object.fromEntries(tuple);
}

