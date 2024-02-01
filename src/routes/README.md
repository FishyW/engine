Add a path.ts file inside of this directory with the following content.

```ts
export type Wasm = typeof import(<path_js>);
export const WASM_PATH = <path_wasm>;
```

`<path_js>` is the path to your wasm javascript file for example,
`~/Documents/engine_lib/web/src/lib/wasm/project`. Note that, you don't need to write the .js extension at the end. `<path_wasm>` is the path to your wasm folder, for instance, `~/Documents/engine_lib/web/src/lib/wasm`

