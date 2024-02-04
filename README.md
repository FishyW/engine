# engine

Engine pet project... Hopefully some progress can be made :(

To install, install the Tauri prerequisites from the [Tauri prerequsites page](https://tauri.app/v1/guides/getting-started/prerequisites/). Also install wasm pack using `cargo install wasm-pack` since the engine uses wasm pack to compile the Rust project scripts.

Run `yarn dev` to preview the app. Run `yarn build` to build the app.

Also, due to the sheer amount of files the project directory, I recommend getting the [Toggle Excluded Files](https://marketplace.visualstudio.com/items?itemName=amodio.toggle-excluded-files) extension to toggle/untoggle excluded files. Note you can toggle excluded files using a shortcut (`Ctrl + Shift + A`) or by clicking on the eye icon on top of the file explorer side bar. 

Then inside of your `.vscode/settings.json` file put this in,

```json
{
    // info source:
    // https://stackoverflow.com/questions/60565855/how-to-exclude-all-folders-except-one-folder-in-vscode
    // excludes node_modules and all files starting with "." except for .gitignore 
    "files.exclude": {
        "**/.[^g]*": true,
        "**/.g[^i]*": true,
        "node_modules": true
    }
}
```
