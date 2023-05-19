const combind = require("../util/combind.js")

module.exports = (...args) => combind(args, x => typeof x == "string")