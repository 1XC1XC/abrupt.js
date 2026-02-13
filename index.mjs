import { createRequire } from "node:module"

const require = createRequire(import.meta.url)
const native = require("./bin/index.node")
const { wrapNative } = require("./index.shared.cjs")
const wrapped = wrapNative(native)

export const rand = wrapped.rand
export const crypto = wrapped.crypto
export const file = wrapped.file
export default wrapped
