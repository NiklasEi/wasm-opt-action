# wasm-opt GitHub action

This docker action allows usage of [binaryen's](https://github.com/WebAssembly/binaryen) wasm-opt to optimise Wasm files inside your GitHub workflows. Binaryen is available under its [Apache 2.0 License](https://github.com/WebAssembly/binaryen/blob/main/LICENSE).

## Usage

In one of your GitHub workflow steps:
```yaml
      - name: Optimize Wasm
        uses: NiklasEi/wasm-opt@v1
        with:
          file: some/path/to/file.wasm
          output: some/path/to/file.wasm
```

The input parameters `file` and `output` are required. There is another optional parameter `options` with a default value of `-Os`. The input parameters are passed to `wasm-opt` like so: `<input> -o <output> <options>`. 

Check the [wasm-opt options](https://github.com/WebAssembly/binaryen/blob/main/src/tools/optimization-options.h) for more info.
