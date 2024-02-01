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