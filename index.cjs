"use strict"

const native = require("./bin/index.node")
const { wrapNative } = require("./index.shared.cjs")

module.exports = wrapNative(native)
