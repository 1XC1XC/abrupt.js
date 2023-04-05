const { createHash } = require("crypto")
module.exports = (x, digest = "hex") => createHash("sha512").update(x).digest(digest)