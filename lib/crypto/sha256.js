const { createHash } = require("crypto")
module.exports = (input, digest = "hex") => createHash("sha256").update(input).digest(digest)