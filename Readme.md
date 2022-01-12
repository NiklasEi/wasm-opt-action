# wasm-opt GitHub action

This docker action allows usage of [binaryen's](https://github.com/WebAssembly/binaryen) wasm-opt to optimise your WASM files.

## Usage

In one of your GitHub workflow steps:
```yaml
      - name: Optimize WASM
        uses: NiklasEi/wasm-opt@v1
        with:
          file: build/web/target/wasm_bg.wasm
          output: build/web/target/wasm_bg.wasm
```

The input parameters `file` and `output` are required. There is another optional parameter `options` with a default value of `-Os`. The input parameters are passed to `wasm-opt` like so: `<input> -o <output> <options>`. 

Check the [wasm-opt options](https://github.com/WebAssembly/binaryen/blob/main/src/tools/optimization-options.h) for more info.
