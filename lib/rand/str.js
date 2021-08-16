const randint = require("./int.js")
const randobject = require("./object.js")
let alpha = {}

for (let i = 97; i <= 122; i++) alpha[i-97] = String.fromCharCode(i)

module.exports = (len, alphabet) => {
    const isBool = ((typeof len == "boolean") && (len))
    if ((isBool) || (!len)) len = 5
    let s = ""
    for (let i = 1; i <= len; i++) {
        s += ((alphabet) || (isBool)) ? randobject(alpha) : String.fromCharCode(randint(33,126))
    }
    return s
}