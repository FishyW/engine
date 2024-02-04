# engine

Engine pet project... Hopefully some progress can be made :(

## Prerequisites 

To install, install the Tauri prerequisites from the [Tauri prerequsites page](https://tauri.app/v1/guides/getting-started/prerequisites/). Also install wasm pack using `cargo install wasm-pack` since the engine uses wasm pack to compile the Rust project scripts.

This project uses `go-task` as a task runner to build the project files. Think of `go-task` as the modern version of `make`. Install `go-task` [here](https://taskfile.dev/installation/). Then, if you want alias `task` with `go-task` using the linux `alias` command, `alias task="go-task"`.

Run `task dev` to preview the app. Run `task build` to build the app. To preview the library, run `task lib:dev`. Run `task lib:build` to build the Rust project files (and prepare it for library preview). To watch the project Rust files (and library Rust files) run `task lib:watch`. To watch both Rust files and Svelte files in `library/web`, run `task lib:watch-all`.

## Project Navigation
The engine library files (engine-lib) are stored inside of the library directory. A sample project folder used for testing purposes is available inside of the project directory. To test a game without the use of the game engine (effectively you're only viewing the scene view), running `task lib:dev` will host a preview of the game inside of localhost. The relevant files used to preview this game is under `library/web`. You can also think of the files in `library/web` as a simple template project that will eventually be used by the builder. 

Finally, the `src` directory contains Sveltekit (frontend) files used by Tauri and the `src-tauri` directory contains the Tauri Rust (backend) files. The tauri config file can be found in `src-tauri/tauri.conf.json`.

## Sidenote

Also, due to the sheer amount of files the project directory, I recommend getting the [Toggle Excluded Files](https://marketplace.visualstudio.com/items?itemName=amodio.toggle-excluded-files) extension to toggle/untoggle excluded files. Note you can toggle excluded files using a shortcut (`Ctrl + Shift + A`) or by clicking on the eye icon on top of the file explorer side bar. 

Then inside of your `.vscode/settings.json` file put this in,

```json
{
    "files.exclude": {
        "**/.[^g]*": true,
        "**/.g[^i]*": true,
        "node_modules": true
    }
}
```
