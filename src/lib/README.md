Add a path.ts file inside of this directory with the following content.

```ts
export type Wasm = typeof import(<path>);
export const WASM_PATH = <path>;
```

Note that `<path>` is the path to your wasm folder (inside of the project folder) for example,
`/home/<user>/Documents/engine/project/build/wasm`. It is recommended to specify an absolute path instead of a relative path. 

