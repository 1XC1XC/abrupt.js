const combind = require("../util/combind")

module.exports = (...args) => combind(args, x => typeof x == "function" && x.constructor.name == "AsyncFunction")