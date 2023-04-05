const { generateKeyPairSync, publicEncrypt, publicDecrypt, privateDecrypt } = require("crypto")
module.exports = {
    encode: (phrase, encoding = "base64", bits = 4096) => {
        var { publicKey, privateKey } = generateKeyPairSync("rsa", { // 
            modulusLength: bits, 
            publicKeyEncoding: { 
                type: "pkcs1", 
                format: "pem" 
            },
            privateKeyEncoding: {
                type: "pkcs1", 
                format: "pem" 
            }
        })
        return { encoding, publicKey: publicEncrypt(publicKey, Buffer.from(phrase)).toString(encoding), privateKey }
    },
    decode: ({encoding, privateKey, publicKey}) => {
        return privateDecrypt(privateKey, Buffer.from(publicKey, encoding)).toString()
    }
}