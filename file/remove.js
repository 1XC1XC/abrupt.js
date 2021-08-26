const combind = require("../util/combind.js")
const exists = require("./exists.js")
const fs = require("fs")

module.exports = (...args) => combind(args, name => {
    const exist = exists(name)
    if (!exist) {
        return false
    } else if (exist == "file") {
        fs.promises.unlink(name)
    } else if (exist == "folder") {
        fs.promises.rmdir(name)
    }
    return true
})