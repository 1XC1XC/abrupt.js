const fs = require("fs")
const combind = require("../util/combind.js")
const exists = require("./exists")

module.exports = (args, content) => combind(args, (name, i) => {
    try {
        if (name.split("/").join("").match(/[\/:*?"<>|]/)) {
            return false
        }
        const type = exists(name)
        
        if (type == "file") {
            return  fs.readFileSync(name, content || "utf8")
        } else if (type == "folder") {
            return fs.readdirSync(name, content)
        }

        return false  
    } catch(err) {
        return false
    }
})