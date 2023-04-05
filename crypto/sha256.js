const { createHash } = require("crypto")
module.exports = (x, digest = "hex") => createHash("sha256").update(x).digest(digest)