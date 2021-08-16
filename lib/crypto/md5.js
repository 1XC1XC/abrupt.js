const { createHash } = require("crypto")
module.exports = (input, digest = "hex") => createHash("md5").update(input).digest(digest)