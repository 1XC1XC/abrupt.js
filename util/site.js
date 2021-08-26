const { exec } = require("child_process")
const { platform : pf } = process

module.exports = (url) => exec(`${pf=="win32"?"start":pf=="darwin"?"open":"xdc-open"} ${url.startsWith("https://")||url.startsWith("http://")?url:`https://${url}`}`)
