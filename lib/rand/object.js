const randarray = require("./array.js")

randobjectkeys = x => randarray(Object.keys(x))

module.exports = (x, o) => {
    const randkey = randobjectkeys(x)
    if (o) return randkey
    return x[randkey]
}