# PVM shell

A non-operational PVM shell that is compiled to WASM and can be included in the [PVM Disassembler](https://github.com/FluffyLabs/typeberry-toolkit/issues/81).

## Requirements

```
$ cargo install wasm-pack
```

## Building

```
$ wasm-pack build --target bundler
```

That will create a `pkg` folder that's a valid nodejs module.

## Running

```
$ cd www
$ npm ci
$ npm start
```
