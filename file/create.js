const fs = require("fs")
const combind = require("../util/combind.js")
const exists = require("./exists")

module.exports = (args, content) => combind(args, (name, i) => {
    try {
        if (name.split("/").join("").match(/[\/:*?"<>|]/)) {
            return false
        }
        let files = "."
        for (const dir of name.split("/").slice(0,-1)) {
            files += `/${dir||""}`
            
            if (!fs.existsSync(files)) {
                fs.mkdirSync(files)       
            }
        }
        if ((content) || (content == "")) {
            fs.writeFileSync(name, typeof content == "object" ? content[i] || "" : content)
        } else {
            fs.mkdirSync(name)
        }
        return true
    } catch(err) {
        return false
    }
})