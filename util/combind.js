module.exports = (args, cb) => {
    let r = []
    if (typeof args != "object") args = [args] 
    for (const arg of args) {
        if (typeof arg == "object") {
            r = r.concat(arg)
        } else {
            r.push(arg)
        }
    }
    
    r = r.map(cb)
    return r.length == 1 ? r.shift() : r
}