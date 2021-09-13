const combind = require("../util/combind.js")
const fs = require("fs")

module.exports = (...args) => combind(args, name => {
    try {
        fs.accessSync(name, fs.F_OK)
        const stats = fs.statSync(name)
        
        if (stats.isFile()) {
            return "file"
        } else if (stats.isDirectory()) {
            return "folder"
        }
        return false
    } catch (err) {
        return false
    } 
})