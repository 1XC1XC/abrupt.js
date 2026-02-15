# abrupt

Fast utility toolkit for Node.js.

Rust-backed core. ESM, CJS, and TypeScript support.

## Installation

```bash
npm i abrupt
```

## Quick Start

```js
import abrupt from "abrupt"

const { rand, crypto, file } = abrupt

const token = rand.str(16, true)
const digest = crypto.sha256(token)
const packet = crypto.AES.encode("hello", "secret")
const plain = crypto.AES.decode(...packet, "secret")

file.create("demo.txt", `${token}:${digest}:${plain}`)
```

## API

- `rand`: `int`, `float`, `str`, `bool`, `array`, `object`
- `crypto`: `md5`, `sha256`, `sha512`, `base64`, `base16`, `base32`, `AES`, `RSA`, `morse`, `rot`
- `file`: `create`, `exists`, `read`, `remove`

## Benchmark

```bash
npm run benchmark
```
