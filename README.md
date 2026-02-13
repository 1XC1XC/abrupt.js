# abrupt

## Overview

`abrupt` is a Node.js utility library backed by Rust through N-API.

- Rust core modules: `rand`, `crypto`, `file`
- Deterministic interfaces with stable return shapes
- Boundary normalization for dynamic file inputs
- No `unsafe` in project Rust sources

## Installation

```bash
npm install abrupt
```

Build from source:

```bash
npm run build
```

## Usage

ESM:

```js
import abrupt from "abrupt"

const { rand, crypto, file } = abrupt

const token = rand.str(16, true)
const hash = crypto.sha256(token)
const packet = crypto.AES.encode(token, "secret")
const plain = crypto.AES.decode(...packet, "secret")

file.create("demo.txt", plain)
const content = file.read("demo.txt")
```

CJS:

```js
const abrupt = require("abrupt")
```

## API Reference

### `rand`

- `rand.int(min?: number, max?: number): number`
- `rand.float(min?: number, max?: number): number`
- `rand.str(lengthOrLetters?: number | boolean, letters?: boolean): string`
- `rand.bool(): boolean`
- `rand.array(values: any[]): any | null`
- `rand.object(values: object, keyOrKeys?: boolean | string[], returnKey?: boolean): any | null`

### `crypto`

Hashes:

- `crypto.md5(input: string, encoding?: "hex" | "base64"): string`
- `crypto.sha256(input: string, encoding?: "hex" | "base64"): string`
- `crypto.sha512(input: string, encoding?: "hex" | "base64"): string`

Base encoders:

- `crypto.base64.encode(input: string): string`
- `crypto.base64.decode(input: string): string`
- `crypto.base16.encode(input: string): string`
- `crypto.base16.decode(input: string): string`
- `crypto.base32.encode(input: string): string`
- `crypto.base32.decode(input: string): string`

Symmetric:

- `crypto.AES.encode(input: string, key: string, encoding?: "base64" | "hex"): [encoded: string, ivHex: string]`
- `crypto.AES.decode(encoded: string, ivHex: string, key: string, encoding?: "base64" | "hex"): string`

Asymmetric:

- `crypto.RSA.encode(input: string, encoding?: "base64" | "hex", bits?: number): { encoded: string, privateKey: string, publicKey: string, encoding: string, bits: number }`
- `crypto.RSA.decode(packet: { encoded: string, privateKey: string, publicKey: string, encoding: string, bits: number }): string`

Text transforms:

- `crypto.morse.encode(input: string): string`
- `crypto.morse.decode(input: string): string`
- `crypto.rot(input: string, shift?: number): string`

### `file`

Input boundary contract:

- `PathInput = string | string[]`
- `exists` and `remove` accept variadic path inputs: `(...inputs: PathInput[])`
- `create` and `read` accept one normalized input set plus one optional second argument
- All non-array inputs are wrapped, one array level is flattened, each path is processed in order, and single-result outputs collapse to a scalar

Functions:

- `file.create(pathInput: PathInput, contentOrPaths?: string | string[]): boolean | boolean[]`
- `file.exists(...inputs: PathInput[]): ("file" | "folder" | false) | Array<"file" | "folder" | false>`
- `file.read(pathInput: PathInput, encoding?: string): string | string[] | false | Array<string | string[] | false>`
- `file.remove(...inputs: PathInput[]): boolean | boolean[]`

## Performance

Run:

```bash
npm run benchmark
```

Observed in the project benchmark harness:

- Rust is consistently faster for `crypto.sha256` and usually faster for `crypto.base64` encode/decode.
- Rust and native JavaScript are near parity for `crypto.rot`.
- For APIs that cross the JS <-> N-API boundary with very small payloads at very high frequency, native JavaScript can still be faster.

Guidance:

- Prefer Rust-backed paths for sustained throughput and deterministic boundary behavior.
- Re-run benchmarks in your deployment runtime before selecting hot-path implementations.
