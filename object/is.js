const combind = require("../util/combind.js")

module.exports = (...args) => combind(args, x => (!Array.isArray(x)) && typeof x == "object")