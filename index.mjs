import { createRequire } from "node:module"

const require = createRequire(import.meta.url)
const native = require("./bin/index.node")
const { wrapNative } = require("./index.shared.cjs")
const wrapped = wrapNative(native)

const { rand, crypto, file } = wrapped

export { rand, crypto, file }
export default wrapped
