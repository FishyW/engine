// various utilities, except checked using proper Typescript

// From medium article
// https://medium.com/@fullstack-shepherd/typescript-transforming-types-with-snake-case-keys-to-camelcase-keys-or-how-to-keep-busy-in-9d5f074d9bfa
type Camelize<T extends string> = T extends `${infer A}_${infer B}` ? `${A}${Camelize<Capitalize<B>>}` : T

export type CamelizeKeys<T extends object> = {
  [key in keyof T as key extends string ? Camelize<key> : key]: T[key]
}


// promise wrapper -> makes a promise returns the tuple [data, error]
// this prevents an annoying try catch block
export async function pwrap<T>(promise: Promise<T>): Promise<[T|null, null|string|Error]> {
    try {
        return [await promise, null];
    } catch (e) {
        return [null, e as string | Error];
    }
}


export function snakelize(mod: object): object {
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
    return Object.fromEntries(tmp);
}

function capitalize(string: string) {
    return string.charAt(0).toUpperCase() + string.slice(1);
}

// converts snake_case Rust module to camelCase JavaScript
export function camelize<T extends object>(object: T): CamelizeKeys<T> {
   const tmp = Object.entries(object).map(([key, value]) => {
        let splitted = key.split("_");
        const first = splitted[0];
        splitted = splitted.slice(1);
        
        key = first + splitted.map(word => capitalize(word))
            .join("");
        return [key, value];
   });
   return Object.fromEntries(tmp);

}