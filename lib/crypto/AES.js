const { createCipheriv, createDecipheriv, randomBytes, createHash } = require("crypto")

function encodeKey(salt, encoding) {
    return Buffer.from(createHash("sha256").update(String(salt)).digest(encoding), encoding)

}
module.exports = {
    encode: (phrase, salt, encoding = "base64") => {
        const key = encodeKey(salt, encoding)
        const iv = randomBytes(16)
        const cipher = createCipheriv("aes-256-cbc", key, iv)
        let data = cipher.update(phrase, "utf8", encoding)
        data += cipher.final(encoding)

        return [ data, iv.toString(encoding), encoding ]
    },
    decode: (phrase, iv, encoding = "base64", salt) => {
        const key = encodeKey(salt, encoding)
        const decipher = createDecipheriv("aes-256-cbc", key, Buffer.from(iv, encoding))
        let data = decipher.update(phrase, encoding, "utf8")
        data += decipher.final()

        return data
    }
}  