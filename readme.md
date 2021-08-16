Random
```js
const { rand } = require("abrupt") 

// Number:
rand.int(100) // (30) 1 to 100
rand.int(50,100) // (76) 50 to 100
rand.int() // (3) 1 to 5

// Float:
rand.float(50, 100) // (64.44560214694141) 50 to 100
rand.float() // (4.552864077962543) 1 to 5

// String:
rand.str(10, true) // (afivfwveor) 10 characters [Letters]
rand.str(15) // (.]B7EU.^kF) 15 characters [All]
rand.str(true) // (dlhej) 5 characters [Letters]
rand.str() // (U)T%-) 5 characters [All]
 
// Boolean:
rand.bool() // random (true, false)

// Array
rand.array([1,2,3]) // random (1,2,3)

// Object (Choices random value, unless the second argument is true then it returns a key.)
rand.object({
    "abc": "efg",
    "hij": 123
}) // random value (efg, 123)

rand.object({
    "abc": "efg",
    "hij": 123
},true) // random key (abc, hij)
```

Crypto
```js
const { crypto: { base64, md5, sha256, morse, rot } } = require("abrupt")

// Base64
const base64_encoded = base64.encode("Hello World!")
base64_encoded // SGVsbG8gV29ybGQh
const base64_decoded = base64.decode(base64_encoded)
base64_decoded // Hello World!

// Hash
md5("Hello World!") // ed076287532e86365e841e92bfc50d8c
md5("Hello World!", "base64") // 7Qdih1MuhjZehB6Sv8UNjA==
sha256("Hello World!") // 7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069
sha256("Hello World!", "base64") // f4OxZX/x/FO5LcGBSKHWXfwtSx+j1ncoSt3SABJtkGk=

// Morse
const morse_encoded = morse.encode("Hello World!")
morse_encoded // .... . .-.. .-.. ---  / .-- --- .-. .-.. -.. -.-.--
const morse_decoded = morse.decode(morse_encoded)
morse_decoded // hello world!

// Rot
rot("Hello World!")// (Uryyb Jbeyq!) ROT 13 
rot("Hello World!", 10) // (Rovvy Gybvn!) ROT 10 
```

String
```js
const { string: { reverse, comma } } = require("abrupt")

// String
const Hello = "Hello World!"
reverse(Hello) // !dlroW olleH
comma(10000) // 10,000
comma(10000,"$") // $10,000
```

Misc
```js
const { site } = require("abrupt")

site("http://www.google.com/") // Load Site (Windows, Mac, Linux)
```