# Node.js Package Utility 

Require
```js
// The separate methods of initializing are either by folder or the index of objects. 

// Library
const { crypto } = require("abrupt") // Object
const crypto = require("abrupt/crypto") // Folder

// Sub-Library
const { crypto: { base64 } } = require("abrupt") // Object
const base64 = require("abrupt/crypto/base64") // Folder

// Specific Function
const { string: { reverse } } = require("abrupt") // Object
const reverse = require("abrupt/string/reverse") // Folder
```

Random
```js
const rand = require("abrupt/rand") 

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
const { base64, md5, sha256, AES, RSA, morse, rot } = require("abrupt/crypto")

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


// AES 256 CBC:
// AES: Base64 (default)
const data = AES.encode("Hello World!", "key")
const [ encoded ] = data // WtbLPEWi4eu+r4bFYQR63w==
const decoded = AES.decode(...data, "key") // Hello World!
decoded 
// AES: Hex
const data = AES.encode("Hello World!", "key", "hex")
const [ encoded ] = data // a7bda69f5eaa9d208b370df02c24e606
const decoded = AES.decode(...data, "key")
decoded // Hello World!


// RSA-PSS: 
// RSA: Base64 (default)
const data = RSA.encode("Hello World!") // Default (Encoding: Base64, Key Size: 4096)
console.log(data.privateKey, data.publicKey) // Private and Public Keys
const decoded = RSA.decode(data) // Hello World!
// RSA: Hex
const data = RSA.encode("Hello World!", "hex", 4096)
console.log(data.privateKey, data.publicKey) // Private and Public Keys
const decoded = RSA.decode(data) // Hello World!


// Morse
const morse_encoded = morse.encode("Hello World!")
morse_encoded // .... . .-.. .-.. ---  / .-- --- .-. .-.. -.. -.-.--
const morse_decoded = morse.decode(morse_encoded)
morse_decoded // hello world!

// Rot
rot("Hello World!")// (Uryyb Jbeyq!) ROT 13 
rot("Hello World!", 10) // (Rovvy Gybvn!) ROT 10 
```

File
```js
// File Library: seperating (file/directory)'s are handle in the function 
const file = require("abrupt/file")

// Create

file.create("Hello.txt", "Hello World!") // creates a text file containing "Hello World!"
file.create(["Hi.txt", "Welcome.txt"], ["Hello", "World!"]) // creates two text files with content corresponding to each array

file.create("Hello") // create a folder called hello
file.create(["Hello", "World"]) // creates two folders
// file.create(["Hello", "World"]) == file.create("Hello", "World") == file.create("Hello", ["World"])

// will create folders if they are missing  
file.create("this/is/three/folders.txt", "without this argument it would be a folder") 


// Exists

file.exists("this") // folder
file.exists("Hello.txt") // file
file.exists("not") // false
file.exists(["this", "Hello.txt", "not"]) // ["folder", "file", false]
// file.exists(["this", "Hello.txt", "not"]) == file.exists("this", "Hello.txt", "not")

// Read
// file.read(String Name, String Encoding)
file.read("Hello.txt") // Default Encoding: UTF8
file.read("this") // read's (file/directory) by (string/array)

// Remove

file.remove("this") // true
file.remove(["Hello.txt", "not"]) // [true, false] 
// file.remove(["Hello.txt", "not"]) == file.remove("Hello.txt", "not")
```

String
```js
const { reverse, comma } = require("abrupt/string")

// String
const Hello = "Hello World!"
reverse(Hello) // !dlroW olleH
comma(10000) // 10,000
comma(10000,"$") // $10,000
```
