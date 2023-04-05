module.exports = {
    encode: input => Buffer.from(input).toString("base64"),
    decode: input => Buffer.from(input, "base64").toString()
}