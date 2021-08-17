const { createCipheriv, createDecipheriv, randomBytes, createHash } = require("crypto")

module.exports = {
    encode: (phrase, salt, encoding = "base64") => {
        const key = Buffer.from(createHash("sha256").update(String(salt)).digest(encoding), encoding)
        const iv = randomBytes(16)
        const cipher = createCipheriv("aes-256-cbc", key, iv)
        let data = cipher.update(phrase, "utf8", encoding)
        data += cipher.final(encoding)

        return [ data, key.toString(encoding), iv.toString(encoding), encoding ]
    },
    decode: (phrase, key, iv, encoding = "base64") => {
        const decipher = createDecipheriv("aes-256-cbc", Buffer.from(key, encoding), Buffer.from(iv, encoding))
        let data = decipher.update(phrase, encoding, "utf8")
        data += decipher.final()

        return data
    }
} 