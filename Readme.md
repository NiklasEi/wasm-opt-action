# wasm-opt GitHub action

This docker action allows usage of [binaryen's](https://github.com/WebAssembly/binaryen) wasm-opt to optimise Wasm files inside your GitHub workflows. Binaryen is available under its [Apache 2.0 License](https://github.com/WebAssembly/binaryen/blob/main/LICENSE).

*Currently, this action uses Binaryen version 105*

## Usage

In one of your GitHub workflow steps:
```yaml
      - name: Optimize Wasm
        uses: NiklasEi/wasm-opt-action@v2
        with:
          file: some/path/to/file.wasm
          output: some/path/to/file.wasm
```

The input parameter `file` is required. Unix shell like patterns are supported, but only the first match is optimized. The input parameter `output` is optional and defaults to the original file path. This is very useful if you pass a glob as `file`.

For example, if your wasm files get a hash attached to them, you can optimize like this:
```yaml
      - name: Optimize Wasm
        uses: NiklasEi/wasm-opt-action@v2
        with:
          file: dist/*.wasm
```

There is another optional parameter `options` with a default value of `-Os`. 

The input parameters are passed to `wasm-opt` like so: `<input> -o <output> <options>`.

Check the [wasm-opt options](https://github.com/WebAssembly/binaryen/blob/main/src/tools/optimization-options.h) for more info.
