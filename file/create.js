const combind = require("../util/combind.js")
const exists = require("./exists")
const { promises : fs } = require("fs")
// // name.match(/\..{3}$/)

module.exports = (args, content) => combind(args, (name, i) => {
    try {
        if (name.match(/[\/:*?"<>|]/)) {
            return false
        }
        let files = "./"
        for (const dir of name.split("/").slice(0,-1)) {
            files += `/${dir}`
            if (!exists(files)) {
                fs.mkdir(files)       
            }
        }
        if ((content) || (content == "")) {
            fs.writeFile(name, typeof content == "object" ? content[i] || "" : content)
        } else {
            fs.mkdir(name)
        }
        return true
    } catch(err) {
        return false
    }
})

// module.exports = (names, content) => {
//     const r = ([].concat(typeof names == "object" ? names : [names])).map((name, i) => {
//         try {
//             let files = "./"
//             for (const dir of name.split("/").slice(0,-1)) {
//                 files += `/${dir}`
//                 if (!exists(files)) {
//                     fs.mkdir(files)       
//                 }
//             }
//             if ((content) || (content == "")) {
//                 fs.writeFile(name, typeof content == "object" ? content[i] || "" : content)
//             } else {
//                 fs.mkdir(name)
//             }
//             return true
//         } catch(err) {
//             return false
//         }
//     })
//     return r.length == 1 ? r.shift() : r
// }